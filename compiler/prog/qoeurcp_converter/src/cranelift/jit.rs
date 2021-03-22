use qoeurcp_tokenizer::ast::*;

use cranelift::prelude::*;
use cranelift_jit::{JITBuilder, JITModule};
use cranelift_module::{DataContext, Linkage, Module};

use std::collections::HashMap;
use std::fs::File;
use std::slice;

pub struct Jit {
  builder_context: FunctionBuilderContext,
  ctx: codegen::Context,
  data_ctx: DataContext,
  module: JITModule,
}

impl Jit {
  pub fn new() -> Jit {
    let builder = JITBuilder::new(cranelift_module::default_libcall_names());
    let module = JITModule::new(builder);

    Self {
      builder_context: FunctionBuilderContext::new(),
      ctx: module.make_context(),
      data_ctx: DataContext::new(),
      module,
    }
  }

  pub fn compile(
    &mut self,
    stmts: Vec<Box<Stmt>>,
  ) -> Result<*const u8, String> {
    let name = "basics";

    self.translate(vec![], String::new(), stmts)?;

    let id = self
      .module
      .declare_function(name, Linkage::Export, &self.ctx.func.signature)
      .map_err(|e| e.to_string())?;

    self
      .module
      .define_function(
        id,
        &mut self.ctx,
        &mut codegen::binemit::NullTrapSink {},
      )
      .map_err(|e| e.to_string())?;

    self.module.clear_context(&mut self.ctx);
    self.module.finalize_definitions();

    let code = self.module.get_finalized_function(id);

    Ok(code)
  }

  pub fn create_data(
    &mut self,
    name: &str,
    contents: Vec<u8>,
  ) -> Result<&[u8], String> {
    self.data_ctx.define(contents.into_boxed_slice());

    let id = self
      .module
      .declare_data(name, Linkage::Export, true, false)
      .map_err(|e| e.to_string())?;

    self
      .module
      .define_data(id, &self.data_ctx)
      .map_err(|e| e.to_string())?;

    self.data_ctx.clear();
    self.module.finalize_definitions();

    let buffer = self.module.get_finalized_data(id);

    Ok(unsafe { slice::from_raw_parts(buffer.0, buffer.1) })
  }

  fn translate(
    &mut self,
    params: Vec<String>,
    the_return: String,
    stmts: Vec<Box<Stmt>>,
  ) -> Result<(), String> {
    let int = self.module.target_config().pointer_type();

    params.iter().for_each(|_| {
      self.ctx.func.signature.params.push(AbiParam::new(int));
    });

    self.ctx.func.signature.returns.push(AbiParam::new(int));

    let mut builder =
      FunctionBuilder::new(&mut self.ctx.func, &mut self.builder_context);
    let entry_block = builder.create_block();

    builder.append_block_params_for_function_params(entry_block);
    builder.switch_to_block(entry_block);
    builder.seal_block(entry_block);

    let variables = declare_variables(
      int,
      &mut builder,
      &params,
      &the_return,
      &stmts,
      entry_block,
    );

    let mut trans = FunctionTranslator {
      int,
      builder,
      variables,
      module: &mut self.module,
    };

    stmts.iter().for_each(|stmt| {
      trans.translate_stmt(stmt);
    });

    let return_variable = trans.variables.get(&the_return).unwrap();
    let return_value = trans.builder.use_var(*return_variable);

    trans.builder.ins().return_(&[return_value]);
    trans.builder.finalize();

    Ok(())
  }

  // pub fn finish(self) {
  //   let product = self.module.finish();
  //   let file = File::create(product.name()).expect("error opening file");

  //   product.write(file).expect("error writing to file");
  // }
}

struct FunctionTranslator<'a> {
  int: types::Type,
  builder: FunctionBuilder<'a>,
  variables: HashMap<String, Variable>,
  module: &'a mut JITModule,
}

impl<'a> FunctionTranslator<'a> {
  fn translate_binop_expr(
    &mut self,
    lhs: &Box<Expr>,
    op: &BinOpKind,
    rhs: &Box<Expr>,
  ) -> Value {
    match op {
      BinOpKind::Add => {
        let lhs_expr = self.translate_expr_stmt(lhs);
        let rhs_expr = self.translate_expr_stmt(rhs);

        make_translate_binop_add_expr(&mut self.builder, lhs_expr, rhs_expr)
      }
      _ => unimplemented!(),
    }
  }

  fn translate_expr_stmt(&mut self, expr: &Box<Expr>) -> Value {
    match expr.kind() {
      ExprKind::BinOp {
        ref lhs,
        ref op,
        ref rhs,
        ..
      } => self.translate_binop_expr(lhs, op, rhs),
      ExprKind::Lit(ref lit) => self.translate_lit_expr(lit),
      _ => unreachable!(),
    }
  }

  fn translate_lit_expr(&mut self, kind: &LitKind) -> Value {
    match kind {
      // LitKind::Real(ref value) => make_codegen_lit_real_expr(&mut self.builder, value),
      LitKind::Int(ref value) => {
        make_codegen_lit_int_expr(&mut self.builder, value)
      }
      _ => unreachable!(),
    }
  }

  fn translate_stmt(&mut self, stmt: &Box<Stmt>) -> Value {
    match stmt.kind() {
      StmtKind::Expr(ref expr) => self.translate_expr_stmt(expr),
      _ => unreachable!(),
    }
  }
}

fn declare_variable(
  int: types::Type,
  builder: &mut FunctionBuilder,
  variables: &mut HashMap<String, Variable>,
  index: &mut usize,
  name: &str,
) -> Variable {
  let var = Variable::new(*index);

  if !variables.contains_key(name) {
    variables.insert(name.into(), var);
    builder.declare_var(var, int);
    *index += 1;
  }

  var
}

fn declare_variables(
  int: types::Type,
  builder: &mut FunctionBuilder,
  params: &[String],
  the_return: &str,
  stmts: &[Box<Stmt>],
  entry_block: cranelift::prelude::Block,
) -> HashMap<String, Variable> {
  let mut variables = HashMap::new();
  let mut index = 0;

  params.iter().enumerate().for_each(|(i, name)| {
    let val = builder.block_params(entry_block)[i];
    let var = declare_variable(int, builder, &mut variables, &mut index, name);

    builder.def_var(var, val);
  });

  let zero = builder.ins().iconst(int, 0);

  let return_variable =
    declare_variable(int, builder, &mut variables, &mut index, the_return);

  builder.def_var(return_variable, zero);

  stmts.iter().for_each(|stmt| {
    declare_variables_in_stmt(int, builder, &mut variables, &mut index, stmt);
  });

  variables
}

fn declare_variables_in_stmt(
  int: types::Type,
  builder: &mut FunctionBuilder,
  variables: &mut HashMap<String, Variable>,
  index: &mut usize,
  expr: &Box<Stmt>,
) {
  match expr.kind {
    _ => (),
  }
}

pub fn make_translate_binop_add_expr<'a>(
  builder: &mut FunctionBuilder<'a>,
  lhs: Value,
  rhs: Value,
) -> Value {
  builder.ins().iadd(lhs, rhs)
}

pub fn make_codegen_lit_int_expr<'a>(
  builder: &mut FunctionBuilder<'a>,
  int: &i64,
) -> Value {
  builder.ins().iconst(types::I64, *int)
}
