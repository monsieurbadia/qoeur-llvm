use super::interface::*;
use super::util::cstring;

use qoeurcp_tokenizer::ast::*;

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::ffi::CString;
use std::os::raw::c_uint;
use std::ptr;

use llvm_sys::core::*;
use llvm_sys::prelude::*;
use llvm_sys::target::*;
use llvm_sys::target_machine::*;
use llvm_sys::transforms::pass_manager_builder::*;
use llvm_sys::*;

pub struct Jit {
  pub context: LLVMContextRef,
  pub module: LLVMModuleRef,
  pub builder: LLVMBuilderRef,
  pub target: RefCell<LLVMTargetRef>,
  pub target_machine: RefCell<LLVMTargetMachineRef>,
  pub target_data: RefCell<LLVMTargetDataRef>,
}

impl Drop for Jit {
  fn drop(&mut self) {
    dispose_builder(self.builder);
    dispose_module(self.module);
    dispose_context(self.context);
  }
}

impl Jit {
  pub fn new() -> Jit {
    unsafe {
      let context = LLVMContextCreate();
      let module = LLVMModuleCreateWithName(cstring!(""));
      let builder = LLVMCreateBuilderInContext(context);

      Self {
        context,
        builder,
        module,
        target: RefCell::new(ptr::null_mut()),
        target_machine: RefCell::new(ptr::null_mut()),
        target_data: RefCell::new(ptr::null_mut()),
      }
    }
  }

  pub fn codegen(&mut self, stmts: Vec<Box<Stmt>>) {
    unsafe {
      let context = LLVMContextCreate();
      let module = LLVMModuleCreateWithName(cstring!("basics"));
      let builder = LLVMCreateBuilderInContext(context);

      let int_ty = LLVMInt64TypeInContext(context);
      let fun_ty = LLVMFunctionType(int_ty, ptr::null_mut(), 0, 0);
      let fun = LLVMAddFunction(module, cstring!("main"), fun_ty);

      let entry_name = cstring!("entry");

      let bb = LLVMAppendBasicBlockInContext(context, fun, entry_name);

      LLVMPositionBuilderAtEnd(builder, bb);

      let mut names = HashMap::new();
      insert_allocations(context, builder, &mut names, &stmts);

      let int_ty = LLVMInt64TypeInContext(context);
      let zero = LLVMConstInt(int_ty, 0, 0);

      let mut ret_value = zero; // return value on empty program

      stmts.iter().for_each(|stmt| {
        ret_value = self.codegen_stmt(context, builder, fun, &mut names, stmt);
      });

      LLVMBuildRet(builder, ret_value);

      let out_file = cstring!("out/test.ll");
      LLVMPrintModuleToFile(module, out_file, ptr::null_mut());

      LLVMDisposeBuilder(builder);
      LLVMDisposeModule(module);
      LLVMContextDispose(context);
    }
  }

  fn codegen_binop_expr(
    &mut self,
    lhs: &Box<Expr>,
    op: &BinOpKind,
    rhs: &Box<Expr>,
  ) -> LLVMValueRef {
    match op {
      BinOpKind::Add => {
        let lhs_expr = self.codegen_expr_stmt(lhs);
        let rhs_expr = self.codegen_expr_stmt(rhs);

        make_build_binop_add_value(self.builder, lhs_expr, rhs_expr)
      }
      _ => unimplemented!(),
    }
  }

  fn codegen_expr_stmt(&mut self, expr: &Box<Expr>) -> LLVMValueRef {
    match expr.kind() {
      ExprKind::BinOp {
        ref lhs,
        ref op,
        ref rhs,
        ..
      } => self.codegen_binop_expr(lhs, op, rhs),
      ExprKind::Lit(ref lit) => self.codegen_lit_expr(lit),
      _ => unimplemented!(),
    }
  }

  fn codegen_lit_expr(&mut self, kind: &LitKind) -> LLVMValueRef {
    match kind {
      // LitKind::Bool(ref value) => make_codegen_lit_bool_expr(value),
      // LitKind::Char(ref value) => make_codegen_lit_char_expr(value),
      LitKind::Real(ref value) => make_const_real_value(self.context, value),
      LitKind::Int(ref value) => make_const_int_value(self.context, value),
      // LitKind::Str(ref value) => make_codegen_lit_str_expr(value),
      _ => unreachable!(),
    }
  }

  fn codegen_stmt(
    &mut self,
    context: LLVMContextRef,
    builder: LLVMBuilderRef,
    fun: LLVMValueRef,
    names: &mut HashMap<String, LLVMValueRef>,
    stmt: &Box<Stmt>,
  ) -> LLVMValueRef {
    match stmt.kind() {
      StmtKind::Expr(ref expr) => self.codegen_expr_stmt(expr),
      _ => unreachable!(),
    }
  }
}

fn insert_allocations(
  context: LLVMContextRef,
  builder: LLVMBuilderRef,
  names: &mut HashMap<String, LLVMValueRef>,
  stmts: &[Box<Stmt>],
) {
  // let mut variable_names = HashSet::new();

  // for stmt in stmts {
  //   match stmt.kind {
  //     StmtKind::Val
  //     _ => unreachable!(),
  //   }
  // }

  // for variable_name in variable_names {
  // unsafe {
  // let int_ty = LLVMInt64TypeInContext(context);
  // let name = CString::new(variable_name.as_bytes()).unwrap();
  // let pointer = LLVMBuildAlloca(builder, int_ty, name.as_ptr());

  // names.insert(variable_name.to_owned(), pointer);
  // }
  // }
}
