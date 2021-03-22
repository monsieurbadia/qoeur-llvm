pub use self::BinOpKind::*;
pub use self::ExprKind::*;
pub use self::LitKind::*;
pub use self::StmtKind::*;
pub use self::TyKind::*;

use crate::token::*;
use crate::tree_builder::TreePrinter;

use qoeurcp_span::Span;

use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub struct Ast {
  pub nodes: Vec<Box<Stmt>>,
}

impl Default for Ast {
  fn default() -> Ast {
    Self::new(vec![])
  }
}

impl TreePrinter for Ast {
  fn print(&self, _level: usize) {
    println!("{} (len: {})", self.text(), self.nodes.len());
  }
}

impl Ast {
  pub fn new(nodes: Vec<Box<Stmt>>) -> Ast {
    Self { nodes }
  }

  pub fn add(&mut self, node: Box<Stmt>) {
    self.nodes.push(node);
  }

  pub fn text(&self) -> String {
    let nodes = self
      .nodes
      .iter()
      .map(|node| node.text())
      .collect::<Vec<String>>()
      .join("\n");

    format!("{}", nodes)
  }
}

#[derive(Clone, Debug, PartialEq)]
pub enum BinOpKind {
  Add,
  Sub,
  Mul,
  Div,
  Mod,
  Lt,
  Gt,
  Le,
  Ge,
  Ne,
  Eq,
}

impl BinOpKind {
  pub fn from(token: &Token) -> BinOpKind {
    match token.kind() {
      TokenKind::Binary(BinaryKind::Add) => BinOpKind::Add,
      TokenKind::Binary(BinaryKind::Sub) => BinOpKind::Sub,
      TokenKind::Binary(BinaryKind::Mul) => BinOpKind::Mul,
      TokenKind::Binary(BinaryKind::Div) => BinOpKind::Div,
      TokenKind::Binary(BinaryKind::Mod) => BinOpKind::Mod,
      TokenKind::Binary(BinaryKind::Lt) => BinOpKind::Lt,
      TokenKind::Binary(BinaryKind::Le) => BinOpKind::Le,
      TokenKind::Binary(BinaryKind::Gt) => BinOpKind::Gt,
      TokenKind::Binary(BinaryKind::Ge) => BinOpKind::Ge,
      TokenKind::Binary(BinaryKind::Eq) => BinOpKind::Eq,
      TokenKind::Binary(BinaryKind::Ne) => BinOpKind::Ne,
      tkn => unimplemented!("{} is not a operator", tkn),
    }
  }

  pub fn text(&self) -> &'static str {
    match self {
      Self::Add => "+",
      Self::Sub => "-",
      Self::Mul => "*",
      Self::Div => "/",
      Self::Mod => "%",
      Self::Lt => "<",
      Self::Le => "<=",
      Self::Gt => ">",
      Self::Ge => ">=",
      Self::Eq => "==",
      Self::Ne => "!=",
    }
  }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Block {
  pub stmts: Vec<Box<Stmt>>,
  pub span: Span,
}

impl fmt::Display for Block {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.text())
  }
}

impl From<Vec<Box<Stmt>>> for Block {
  fn from(stmts: Vec<Box<Stmt>>) -> Block {
    Self {
      stmts,
      span: Span::zero(),
    }
  }
}

impl TreePrinter for Block {
  fn print(&self, _level: usize) {
    println!("{{ {} }} (span: {})", self.text(), self.span);
  }
}

impl Block {
  pub fn new() -> Block {
    Self::from(vec![])
  }

  pub fn add(&mut self, stmt: Stmt) {
    self.stmts.push(box stmt);
  }

  fn text(&self) -> String {
    let stmts = &self
      .stmts
      .iter()
      .map(|stmt| stmt.text())
      .collect::<Vec<String>>()
      .join("\n");

    format!("{}", stmts)
  }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Capsule {
  pub span: Span,
  pub stmt: Option<Stmt>,
  pub with_traits: Vec<String>,
  pub name: String,
  pub args: Vec<FunArg>,
  pub members: Vec<TraitMember>,
  pub visibility: bool,
}

impl TreePrinter for Capsule {
  fn print(&self, _level: usize) {
    println!("{} (span: {})", self.text(), self.span);
  }
}

impl Capsule {
  pub fn new(
    span: Span,
    stmt: Option<Stmt>,
    with_traits: Vec<String>,
    name: &str,
    args: Vec<FunArg>,
    members: Vec<TraitMember>,
  ) -> Capsule {
    Self {
      span,
      stmt,
      with_traits,
      name: name.into(),
      args,
      members,
      visibility: false,
    }
  }

  pub fn text(&self) -> String {
    let members = self
      .members
      .iter()
      .map(|member| member.text())
      .collect::<Vec<String>>()
      .join("\n");

    format!(
      "{} capsule {} {{ {} }} (span: {})",
      self.visibility, self.name, members, self.span,
    )
  }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ExprKind {
  Empty,
  Closure(Box<Fun>),
  Ident(String),
  Lit(LitKind),
  Loop(LoopKind),
  Array {
    data: Vec<Box<Expr>>,
    span: Span,
  },
  BinOp {
    lhs: Box<Expr>,
    op: BinOpKind,
    rhs: Box<Expr>,
    span: Span,
  },
  Call {
    callee: Box<Expr>,
    args: Vec<Box<Expr>>,
    span: Span,
  },
  Hash {
    data: Vec<(Box<HashKind>, Box<Expr>)>,
  },
  IfElse {
    conditions: Vec<Box<Expr>>,
    alternative: Option<Box<Block>>,
  },
  Index {
    index: Box<Expr>,
    data: Box<Expr>,
  },
  MemberAccess {
    from: Box<Expr>,
    access: String,
  },
  UnOp {
    operand: UnOpKind,
    rhs: Box<Expr>,
  },
}

impl TreePrinter for ExprKind {
  fn print(&self, _level: usize) {
    match *self {
      Self::BinOp {
        ref lhs,
        ref op,
        ref rhs,
        ref span,
      } => println!(
        "{} {} {} (span: {})",
        lhs.text(),
        op.text(),
        rhs.text(),
        span
      ),
      _ => println!(""),
    }
  }
}

impl ExprKind {
  pub fn as_bool(&self) -> bool {
    match self {
      Self::Lit(LitKind::Bool(false)) => false,
      _ => true,
    }
  }

  pub fn text(&self) -> String {
    match *self {
      Self::Ident(ref ident) => format!("{}", ident),
      _ => format!(""),
    }
  }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Expr {
  pub kind: ExprKind,
  pub span: Span,
}

impl TreePrinter for Expr {
  fn print(&self, _level: usize) {
    println!("{} (span: {})", self.text(), self.span);
  }
}

impl Expr {
  pub fn new(kind: ExprKind, span: Span) -> Expr {
    Self { kind, span }
  }

  pub fn kind(&self) -> ExprKind {
    self.kind.to_owned()
  }

  pub fn text(&self) -> String {
    format!("{}", self.kind.text())
  }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Field {
  pub name: String,
  pub ty: Ty,
  pub expr: Option<Expr>,
  pub span: Span,
}

impl TreePrinter for Field {
  fn print(&self, _level: usize) {
    println!("{} (span: {})", self.text(), self.span);
  }
}

impl Field {
  pub fn new(name: &str, ty: Ty, expr: Option<Expr>, span: Span) -> Field {
    Self {
      name: name.into(),
      expr,
      ty,
      span,
    }
  }

  pub fn text(&self) -> String {
    format!("{} {}", self.name, self.ty.text())
  }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Fun {
  pub span: Span,
  pub name: String,
  pub args: Vec<Box<FunArg>>,
  pub ret_ty: Ty,
  pub block: Option<Block>,
}

impl TreePrinter for Fun {
  fn print(&self, _level: usize) {
    println!("{} (span: {})", self.text(), self.span);
  }
}

impl Fun {
  pub fn new(
    span: Span,
    name: &str,
    args: Vec<Box<FunArg>>,
    ret_ty: Ty,
    block: Box<Block>,
  ) -> Fun {
    Self {
      span,
      name: name.into(),
      args,
      ret_ty,
      block: Some(*block),
    }
  }

  pub fn name(&self) -> String {
    self.name.to_owned()
  }

  pub fn text(&self) -> String {
    let args = self
      .args
      .iter()
      .map(|arg| arg.text())
      .collect::<Vec<String>>()
      .join("\n");

    let block = &self.block.as_ref().unwrap().text();

    format!("fun {}: {} = ({}) {{ {} }}", self.name, "", args, block)
  }
}

#[derive(Clone, Debug, PartialEq)]
pub struct FunArg {
  pub name: Option<String>,
  pub immutable: bool,
  pub expr: Box<Expr>,
  pub span: Span,
  pub ty: Ty,
}

impl TreePrinter for FunArg {
  fn print(&self, _level: usize) {
    println!("{} (span: {})", self.text(), self.span);
  }
}

impl FunArg {
  pub fn new(name: Option<String>, expr: Box<Expr>) -> FunArg {
    Self {
      name,
      immutable: false,
      expr,
      span: Span::zero(),
      ty: Ty::unknown(),
    }
  }

  pub fn text(&self) -> String {
    let name = self.name.as_ref().unwrap();
    let ty = self.ty.text();

    format!("{}: {}", name, ty)
  }
}

#[derive(Clone, Debug, PartialEq)]
pub struct FunSig {
  pub name: String,
  pub args: Vec<FunArg>,
  pub ret_ty: Ty,
  pub span: Span,
  pub ty: Ty,
}

impl TreePrinter for FunSig {
  fn print(&self, _level: usize) {
    println!("{} (span: {})", self.text(), self.span)
  }
}

impl FunSig {
  pub fn text(&self) -> String {
    let args = self
      .args
      .iter()
      .map(|arg| arg.text())
      .collect::<Vec<String>>()
      .join("\n");

    format!("fun {}: {} = ({})", self.name, self.ty.text(), args)
  }
}

#[derive(Clone, Debug, PartialEq)]
pub enum HashKind {
  Bool(bool),
  Int(i64),
  Str(String),
}

impl fmt::Display for HashKind {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.text())
  }
}

impl From<Box<Expr>> for HashKind {
  fn from(expr: Box<Expr>) -> HashKind {
    match expr.kind() {
      Lit(LitKind::Bool(value)) => Self::Bool(value),
      Lit(LitKind::Int(value)) => Self::Int(value),
      Lit(LitKind::Str(value)) => Self::Str(value),
      _ => unreachable!(), // TODO: handle an error instead
    }
  }
}

impl TreePrinter for HashKind {
  fn print(&self, _level: usize) {
    println!("{}", self.text())
  }
}

impl HashKind {
  pub fn text(&self) -> String {
    match self {
      Self::Bool(value) => format!("{}", value),
      Self::Int(value) => format!("{}", value),
      Self::Str(value) => format!("{}", value),
    }
  }
}

#[derive(Clone, Debug, PartialEq)]
pub enum LitKind {
  Bool(bool),
  Char(char),
  Int(i64),
  Real(f64),
  Str(String),
}

impl LitKind {
  pub fn text(&self) -> String {
    match *self {
      Self::Bool(ref b) => format!("{}", b),
      Self::Char(ref c) => format!("'{}'", c),
      Self::Int(ref int) => format!("{}", int),
      Self::Real(ref real) => format!("{}", real),
      Self::Str(ref string) => format!("\"{}\"", string),
    }
  }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Local {
  pub name: String,
  pub immutable: bool,
  pub ty: Ty,
  pub value: Box<Expr>,
  pub span: Span,
}

impl TreePrinter for Local {
  fn print(&self, _level: usize) {
    println!("{} (span: {})", self.text(), self.span);
  }
}

impl Local {
  pub fn name(&self) -> String {
    self.name.to_owned()
  }

  pub fn text(&self) -> String {
    let ty = self.ty.text();
    let value = self.value.text();

    if self.immutable {
      return format!("mut {}: {} = {}", self.name, ty, value);
    }

    format!("val {}: {} = {}", self.name, ty, value)
  }
}

#[derive(Clone, Debug, PartialEq)]
pub enum LoopKind {
  LoopFor {
    iterable: Box<Expr>,
    iterator: Box<Expr>,
    block: Box<Block>,
  },
  LoopLoop {
    block: Box<Block>,
  },
  LoopWhile {
    condition: Box<Expr>,
    block: Box<Block>,
  },
}

#[derive(Clone, Debug, PartialEq)]
pub enum StmtKind {
  Empty,
  Capsule(Box<Capsule>),
  Expr(Box<Expr>),
  Fun(Box<Fun>),
  Mut(Box<Local>),
  Ret(Option<Box<Expr>>),
  Struct(Box<Struct>),
  Use(Box<Use>),
  Val(Box<Local>),
  IfBlock {
    conditions: Vec<(Box<Expr>, Box<Block>)>,
    alternative: Option<Box<Block>>,
  },
}

impl StmtKind {
  pub fn text(&self) -> String {
    match *self {
      Self::Empty => format!("Empty"),
      _ => format!(""),
    }
  }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Stmt {
  pub kind: StmtKind,
  pub span: Span,
}

impl TreePrinter for Stmt {
  fn print(&self, _level: usize) {
    println!("{} (span: {})", self.text(), self.span);
  }
}

impl Stmt {
  pub fn new(kind: StmtKind, span: Span) -> Stmt {
    Self { kind, span }
  }

  pub fn kind(&self) -> StmtKind {
    self.kind.to_owned()
  }

  pub fn text(&self) -> String {
    format!("{}", self.kind.text())
  }
}

#[derive(Clone, Debug, PartialEq)]
pub enum StructMember {
  Field(Field),
  Method(Fun),
  StaticMethod(Fun),
}

impl StructMember {
  pub fn text(&self) -> String {
    match *self {
      Self::Field(ref field) => format!("{}", field.text()),
      Self::Method(ref method) => format!("{}", method.text()),
      Self::StaticMethod(ref static_method) => {
        format!("{}", static_method.text())
      }
    }
  }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Struct {
  pub span: Span,
  pub node: Option<Stmt>,
  pub parents: Vec<String>,
  pub name: String,
  pub param_tys: Vec<FunArg>,
  pub members: Vec<StructMember>,
}

impl TreePrinter for Struct {
  fn print(&self, _level: usize) {
    println!("{} (span: {})", self.text(), self.span);
  }
}

impl Struct {
  pub fn new(
    span: Span,
    node: Option<Stmt>,
    parents: Vec<String>,
    name: &str,
    param_tys: Vec<FunArg>,
    members: Vec<StructMember>,
  ) -> Struct {
    Self {
      span,
      node,
      parents,
      name: name.into(),
      param_tys,
      members,
    }
  }

  pub fn text(&self) -> String {
    let members = self
      .members
      .iter()
      .map(|member| member.text())
      .collect::<Vec<String>>()
      .join("\n");

    format!("struct {} {{ {} }}", self.name, members)
  }
}

#[derive(Clone, Debug, PartialEq)]
pub enum TraitMember {
  Field(Field),
  Method(Fun),
}

impl TreePrinter for TraitMember {
  fn print(&self, _level: usize) {
    match *self {
      Self::Field(ref field) => {
        println!("{} (span: {})", field.text(), field.span)
      }
      Self::Method(ref method) => {
        println!("{} (span: {})", method.text(), method.span)
      }
    }
  }
}

impl TraitMember {
  pub fn text(&self) -> String {
    match *self {
      Self::Field(ref field) => format!("{}", field.text()),
      Self::Method(ref method) => format!("{}", method.text()),
    }
  }
}

#[derive(Clone, Debug, PartialEq)]
pub enum UnOpKind {
  Deref,
  Not,
  Neg,
}

impl From<&str> for UnOpKind {
  fn from(operand: &str) -> UnOpKind {
    match operand {
      "*" => Self::Deref,
      "!" => Self::Not,
      "-" => Self::Neg,
      _ => unreachable!(),
    }
  }
}

impl UnOpKind {
  pub fn is_by_value(unop: UnOpKind) -> bool {
    match unop {
      Self::Neg | Self::Not => true,
      _ => false,
    }
  }

  pub fn text(op: UnOpKind) -> &'static str {
    match op {
      Self::Deref => "*",
      Self::Not => "!",
      Self::Neg => "-",
    }
  }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Use {
  pub name: String,
  pub stmts: Vec<Box<Stmt>>,
  pub span: Span,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TyKind {
  Unknown,
  NameRefTy(String),
  GenericTy {
    name: String,
    param_tys: Vec<TyKind>,
  },
}

impl TyKind {
  pub fn text(&self) -> String {
    match *self {
      Self::NameRefTy(ref name) => format!("{}", name),
      Self::GenericTy { ref name, .. } => format!("{}", name),
      Self::Unknown => format!("Unknown"),
    }
  }

  pub fn generics(&self) -> Vec<TyKind> {
    match *self {
      Self::NameRefTy(_) | Self::Unknown => vec![],
      Self::GenericTy { ref param_tys, .. } => param_tys.to_vec(),
    }
  }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Ty {
  kind: Box<TyKind>,
  span: Span,
}

impl From<Box<Expr>> for Ty {
  fn from(ident: Box<Expr>) -> Ty {
    match ident.text() {
      _ => *make_unknown_ty(TyKind::Unknown),
    }
  }
}

impl Ty {
  pub fn new(kind: TyKind, span: Span) -> Ty {
    Self {
      kind: box kind,
      span,
    }
  }

  pub fn text(&self) -> String {
    format!("{}", self.kind.text())
  }

  pub fn unknown() -> Ty {
    Self {
      kind: box TyKind::Unknown,
      span: Span::zero(),
    }
  }
}

pub fn make_array_expr(data: Vec<Box<Expr>>) -> Box<Expr> {
  box Expr::new(
    ExprKind::Array {
      data,
      span: Span::zero(),
    },
    Span::zero(),
  )
}

pub fn make_binop_expr(
  lhs: Box<Expr>,
  op: BinOpKind,
  rhs: Box<Expr>,
) -> Box<Expr> {
  box Expr::new(
    BinOp {
      lhs,
      op,
      rhs,
      span: Span::zero(),
    },
    Span::zero(),
  )
}

pub fn make_block_expr(stmts: Vec<Box<Stmt>>) -> Box<Block> {
  box Block::from(stmts)
}

pub fn make_bool_expr(expr: bool) -> Box<Expr> {
  box Expr::new(ExprKind::Lit(LitKind::Bool(expr)), Span::zero())
}

pub fn make_call_expr(callee: Box<Expr>, args: Vec<Box<Expr>>) -> Box<Expr> {
  box Expr::new(
    Call {
      callee,
      args,
      span: Span::zero(),
    },
    Span::zero(),
  )
}

pub fn make_hash_expr(data: Vec<(Box<HashKind>, Box<Expr>)>) -> Box<Expr> {
  box Expr::new(ExprKind::Hash { data }, Span::zero())
}

pub fn make_hash_data_expr(
  key: Box<Expr>,
  value: Box<Expr>,
) -> (Box<HashKind>, Box<Expr>) {
  (Box::new(HashKind::from(key)), value)
}

pub fn make_ident_expr(id: &str) -> Box<Expr> {
  box Expr::new(ExprKind::Ident(id.into()), Span::zero())
}

// pub fn make_if_else_expr(
//   condition: Box<Expr>,
//   consequence: Box<Expr>,
//   alternative: Box<Expr>,
// ) -> Box<Expr> {
//   box Expr::new(
//     ExprKind::IfElse {
//       condition,
//       consequence,
//       alternative,
//     },
//     Span::zero(),
//   )
// }

pub fn make_index_expr(data: Box<Expr>, index: Box<Expr>) -> Box<Expr> {
  box Expr::new(ExprKind::Index { data, index }, Span::zero())
}

pub fn make_lit_int_expr(int: i64) -> Box<Expr> {
  box Expr::new(ExprKind::Lit(LitKind::Int(int)), Span::zero())
}

pub fn make_lit_real_expr(real: f64) -> Box<Expr> {
  box Expr::new(ExprKind::Lit(LitKind::Real(real)), Span::zero())
}

pub fn make_lit_str_expr(expr: String) -> Box<Expr> {
  box Expr::new(ExprKind::Lit(LitKind::Str(expr)), Span::zero())
}

pub fn make_loop_for_expr(
  iterable: Box<Expr>,
  iterator: Box<Expr>,
  block: Box<Block>,
) -> Box<Expr> {
  box Expr::new(
    ExprKind::Loop(LoopKind::LoopFor {
      iterable,
      iterator,
      block,
    }),
    Span::zero(),
  )
}

pub fn make_loop_loop_expr(block: Box<Block>) -> Box<Expr> {
  box Expr::new(ExprKind::Loop(LoopKind::LoopLoop { block }), Span::zero())
}

pub fn make_loop_while_expr(
  condition: Box<Expr>,
  block: Box<Block>,
) -> Box<Expr> {
  box Expr::new(
    ExprKind::Loop(LoopKind::LoopWhile { condition, block }),
    Span::zero(),
  )
}

pub fn make_expr(kind: ExprKind) -> Box<Expr> {
  box Expr::new(kind, Span::zero())
}

pub fn make_member_access_expr(from: Expr, access: &str) -> Box<Expr> {
  box Expr::new(
    MemberAccess {
      from: box from,
      access: access.into(),
    },
    Span::zero(),
  )
}

pub fn make_unop_expr(operand: &str, rhs: Box<Expr>) -> Box<Expr> {
  box Expr::new(
    ExprKind::UnOp {
      operand: UnOpKind::from(operand),
      rhs,
    },
    Span::zero(),
  )
}

pub fn make_expr_stmt(expr: Box<Expr>) -> Box<Stmt> {
  box Stmt::new(StmtKind::Expr(expr), Span::zero())
}

pub fn make_fun_stmt(
  name: Box<Expr>,
  args: Vec<Box<FunArg>>,
  ret_ty: Box<Expr>,
  block: Box<Block>,
) -> Box<Stmt> {
  box Stmt::new(
    StmtKind::Fun(box Fun {
      name: name.text(),
      args,
      block: Some(*block),
      ret_ty: Ty::from(ret_ty),
      span: Span::zero(),
    }),
    Span::zero(),
  )
}

pub fn make_fun_arg(expr: Box<Expr>, ty: Box<Expr>) -> Box<FunArg> {
  box FunArg {
    immutable: true,
    expr: expr.to_owned(),
    name: Some(expr.text()),
    ty: Ty::from(ty),
    span: Span::zero(),
  }
}

pub fn make_mut_stmt(
  name: Box<Expr>,
  ty: Box<Expr>,
  value: Box<Expr>,
) -> Box<Stmt> {
  box Stmt::new(
    StmtKind::Mut(box Local {
      immutable: false,
      name: name.text(),
      value,
      ty: Ty::from(ty),
      span: Span::zero(),
    }),
    Span::zero(),
  )
}

pub fn make_ret_stmt(expr: Box<Expr>) -> Box<Stmt> {
  box Stmt::new(StmtKind::Ret(Some(expr)), Span::zero())
}

pub fn make_val_stmt(
  name: Box<Expr>,
  ty: Box<Expr>,
  value: Box<Expr>,
) -> Box<Stmt> {
  box Stmt::new(
    StmtKind::Val(box Local {
      immutable: true,
      name: name.text(),
      value,
      ty: Ty::from(ty),
      span: Span::zero(),
    }),
    Span::zero(),
  )
}

pub fn make_use_stmt(name: &str, ast: Box<Ast>) -> Box<Stmt> {
  box Stmt::new(
    StmtKind::Use(box self::Use {
      name: name.into(),
      stmts: ast.nodes,
      span: Span::zero(),
    }),
    Span::zero(),
  )
}

pub fn make_name_ref_ty(name: &str) -> Box<Ty> {
  box Ty::new(NameRefTy(name.into()), Span::zero())
}

pub fn make_generics_ty() -> Box<Ty> {
  box Ty::new(
    GenericTy {
      name: "".into(),
      param_tys: vec![],
    },
    Span::zero(),
  )
}

pub fn make_unknown_ty(kind: TyKind) -> Box<Ty> {
  box Ty::new(kind, Span::zero())
}
