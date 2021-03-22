mod interface;

pub use self::interface::{TreeBuilderPrinter, TreePrinter, TreeSink};

use crate::ast::*;
use crate::token::*;

use qoeurcp_span::Span;

use std::collections::VecDeque;
use std::mem;

pub struct TreeBuilder<Handle, Sink> {
  handle: Handle,
  nodes: Vec<Handle>,
  tokens: VecDeque<Token>,
  sink: Sink,
  errors: Vec<String>,
  stmts: Vec<Box<Stmt>>,
  token: Box<Token>,
  first: Box<Token>,
}

impl<Handle, Sink> TreeBuilder<Handle, Sink>
where
  Handle: Clone,
  Sink: TreeSink<Handle = Handle>,
{
  pub fn new(mut sink: Sink) -> TreeBuilder<Handle, Sink> {
    let handle = sink.get_stmts();

    Self {
      handle: handle,
      sink: sink,
      stmts: vec![],
      nodes: vec![],
      errors: vec![],
      tokens: VecDeque::new(),
      token: box Token::new(TokenKind::EOF, Span::zero()),
      first: box Token::new(TokenKind::EOF, Span::zero()),
    }
  }

  pub fn stmts(&self) -> Vec<Box<Stmt>> {
    self.stmts.to_vec()
  }

  pub fn unwrap(self) -> Sink {
    self.sink
  }

  pub fn sink<'a>(&'a self) -> &'a Sink {
    &self.sink
  }

  pub fn sink_mut<'a>(&'a mut self) -> &'a mut Sink {
    &mut self.sink
  }

  pub fn next_token<'a>(&mut self) {
    self.token = mem::replace(
      &mut self.first,
      box self
        .tokens
        .pop_front()
        .unwrap_or(Token::new(TokenKind::EOF, Span::zero()))
        .to_owned(),
    );
  }

  fn current_precedence(&self) -> PrecedenceKind {
    TokenKind::precedence(&self.token.kind())
  }

  fn emit(&mut self, ast: Box<Ast>) {
    println!("{:#?}", ast);
    self.sink.ast(ast);
  }

  fn expect_first(&mut self, kind: &TokenKind) -> Result<(), String> {
    if self.first_is(kind) {
      return Ok(self.next_token());
    }

    Err(format!(
      "token {:?} expected, but the current token is {:?}!",
      &kind,
      &self.first.kind()
    ))
  }

  fn first_is(&self, kind: &TokenKind) -> bool {
    self.first.kind() == *kind
  }

  fn parse_array_expr(&mut self) -> Result<Box<Expr>, String> {
    let data = self.parse_until(&CloseBracket)?;
    Ok(make_array_expr(data))
  }

  fn parse_binop_expr(&mut self, lhs: Box<Expr>) -> Result<Box<Expr>, String> {
    let precedence = self.current_precedence();
    let op = BinOpKind::from(&self.token);

    self.next_token();

    let rhs = self.parse_expr_by_precedence(&precedence)?;

    Ok(make_binop_expr(lhs, op, rhs))
  }

  fn parse_binop_expr_by_lhs(
    &mut self,
    lhs: Box<Expr>,
  ) -> Result<Box<Expr>, String> {
    match self.token.kind() {
      TokenKind::OpenBracket => self.parse_index_expr(lhs),
      TokenKind::OpenParen => self.parse_call_expr(lhs),
      _ => self.parse_binop_expr(lhs),
    }
  }

  fn parse_block(&mut self) -> Result<Box<Block>, String> {
    let mut stmts = vec![];

    self.next_token();

    while !self.token_is(&CloseBrace) {
      match self.parse_stmt() {
        Err(_) => break,
        Ok(stmt) => stmts.push(stmt),
      };

      self.next_token();
    }

    Ok(make_block_expr(stmts))
  }

  fn parse_bool_expr(&mut self) -> Result<Box<Expr>, String> {
    let expr = self.token_is(&True);
    Ok(make_bool_expr(expr))
  }

  fn parse_call_expr(
    &mut self,
    callee: Box<Expr>,
  ) -> Result<Box<Expr>, String> {
    let args = self.parse_until(&CloseParen)?;

    Ok(make_call_expr(callee, args))
  }

  // TODO: implements Comment for ast
  // fn parse_comment_expr(&mut self) -> Result<Box<Expr>, String> {
  //   match self.token.kind {
  //     TokenKind::Comment(Line) => {
  //       let expr = self.token.text();
  //       Ok(make_comment_line_expr(expr))
  //     }
  //     _ => Err(format!(
  //       "parser:fn:parse_comment_expr:error: {}",
  //       &self.token.text()
  //     )),
  //   }
  // }

  fn parse_expr(&mut self) -> Result<Box<Expr>, String> {
    match self.token.kind() {
      TokenKind::OpenBrace => self.parse_hash_expr(),
      TokenKind::OpenBracket => self.parse_array_expr(),
      TokenKind::OpenParen => self.parse_group_expr(),
      TokenKind::Ident(_) => self.parse_ident_expr(),
      TokenKind::False | TokenKind::True => self.parse_bool_expr(),
      TokenKind::For => self.parse_loop_for_expr(),
      // TokenKind::If => self.parse_if_else_expr(),
      TokenKind::Loop => self.parse_loop_loop_expr(),
      TokenKind::While => self.parse_loop_while_expr(),
      TokenKind::Literal(RealNumber(_)) => self.parse_lit_real_expr(),
      TokenKind::Literal(IntNumber(_)) => self.parse_lit_int_expr(),
      TokenKind::Literal(StrBuffer(_)) => self.parse_lit_str_expr(),
      TokenKind::Binary(BinaryKind::Sub) | TokenKind::Unary(UnaryKind::Not) => {
        self.parse_unop_expr()
      }
      _ => Err(format!(
        "parser:fn:parse_expr_stmt:error: {}",
        self.token.text()
      )),
    }
  }

  fn parse_expr_by_precedence(
    &mut self,
    precedence: &PrecedenceKind,
  ) -> Result<Box<Expr>, String> {
    let mut node = self.parse_expr()?;

    while !self.first_is(&Semicolon)
      && self.should_precedence_has_priority(precedence)
    {
      self.next_token();

      node = self.parse_binop_expr_by_lhs(node)?;
    }

    Ok(node)
  }

  fn parse_expr_stmt(&mut self) -> Result<Box<Stmt>, String> {
    let expr = self.parse_expr_by_precedence(&Lowest)?;

    if self.first_is(&Semicolon) {
      self.next_token();
    }

    Ok(make_expr_stmt(expr))
  }

  fn parse_fun_stmt(&mut self) -> Result<Box<Stmt>, String> {
    self.expect_first(&TokenKind::Ident(self.first.text()))?;

    let name = self.parse_ident_expr()?;

    let ty;

    if self.first_is(&&Semicolon) {
      self.next_token();

      ty = self.parse_ident_expr()?;
    } else {
      // TODO: void type
      ty = self.parse_ident_expr()?;
    }

    self.expect_first(&TokenKind::AssignOp(BinaryKind::Eq))?;

    self.expect_first(&OpenParen)?;

    let args = self.parse_fun_arg_exprs()?;

    self.expect_first(&OpenBrace)?;

    let block = self.parse_block()?;

    Ok(make_fun_stmt(name, args, ty, block))
  }

  fn parse_fun_arg_expr(&mut self) -> Result<Box<FunArg>, String> {
    self.next_token();

    let expr = self.parse_ident_expr()?;

    self.expect_first(&Colon)?;
    self.next_token();

    let ty = self.parse_ident_expr()?;

    Ok(make_fun_arg(expr, ty))
  }

  fn parse_fun_arg_exprs(&mut self) -> Result<Vec<Box<FunArg>>, String> {
    let mut args = vec![];

    if self.first_is(&CloseParen) {
      self.next_token();
      return Ok(args);
    }

    args.push(self.parse_fun_arg_expr()?);

    while self.first_is(&Comma) {
      self.next_token();
      args.push(self.parse_fun_arg_expr()?);
    }

    self.expect_first(&CloseParen)?;

    Ok(args)
  }

  fn parse_group_expr(&mut self) -> Result<Box<Expr>, String> {
    self.next_token();

    let expr = self.parse_expr_by_precedence(&Lowest)?;

    self.expect_first(&CloseParen)?;

    Ok(expr)
  }

  fn parse_ident_expr(&mut self) -> Result<Box<Expr>, String> {
    match self.token.kind() {
      TokenKind::Ident(ref ident) => Ok(make_ident_expr(ident)),
      _ => Err(format!(
        "parser::fn::parse_ident_expr: {}",
        &self.token.text()
      )),
    }
  }

  // TODO: implements IfElse for ast
  // fn parse_if_else_expr(&mut self) -> Result<Box<Expr>, String> {
  //   self.next_token();

  //   let condition = self.parse_expr_by_precedence(&Lowest)?;

  //   self.expect_first(&OpenBrace)?;

  //   let consequence = self.parse_block()?;

  //   let alternative = if self.first_is(&Else) {
  //     self.next_token();
  //     self.expect_first(&OpenBrace)?;

  //     Some(self.parse_block()?)
  //   } else {
  //     None
  //   };

  //   Ok(make_if_else_expr(condition, consequence, alternative))
  // }

  pub fn parse_index_expr(
    &mut self,
    lhs: Box<Expr>,
  ) -> Result<Box<Expr>, String> {
    self.next_token();

    let rhs = self.parse_expr_by_precedence(&Lowest)?;

    self.expect_first(&CloseBracket)?;

    Ok(make_index_expr(rhs, lhs))
  }

  fn parse_lit_int_expr(&mut self) -> Result<Box<Expr>, String> {
    match self.token.text().replace("_", "").parse() {
      Err(_) => Err(format!("parser:parse_lit_int_expr:error")),
      Ok(expr) => Ok(make_lit_int_expr(expr)),
    }
  }

  fn parse_lit_real_expr(&mut self) -> Result<Box<Expr>, String> {
    match self.token.text().parse() {
      Err(_) => Err(format!("parser:parse_lit_real_expr:error")),
      Ok(expr) => Ok(make_lit_real_expr(expr)),
    }
  }

  fn parse_lit_str_expr(&mut self) -> Result<Box<Expr>, String> {
    let expr = self.token.text();
    Ok(make_lit_str_expr(expr))
  }

  fn parse_local_stmt(&mut self) -> Result<Box<Stmt>, String> {
    let from_kw = self.token.kind();

    self.expect_first(&TokenKind::Ident(self.first.text()))?;

    let name = self.parse_ident_expr()?;

    self.expect_first(&Colon)?;
    self.next_token();

    let ty = self.parse_ident_expr()?;

    self.expect_first(&TokenKind::AssignOp(BinaryKind::Eq))?;
    self.next_token();

    let value = self.parse_expr_by_precedence(&Lowest)?;
    self.next_token();

    Ok(match from_kw {
      TokenKind::Mut => make_mut_stmt(name, ty, value),
      TokenKind::Val => make_val_stmt(name, ty, value),
      _ => unreachable!(),
    })
  }

  fn parse_loop_for_expr(&mut self) -> Result<Box<Expr>, String> {
    let iterable;

    if self.first_is(&OpenBracket) {
      iterable = self.parse_array_expr()?;
    } else {
      iterable = self.parse_ident_expr()?;
    }

    self.next_token();
    self.expect_first(&OpenParen)?;
    self.expect_first(&TokenKind::Ident(self.first.text()))?;

    let iterator = self.parse_ident_expr()?;

    self.expect_first(&CloseParen)?;
    self.next_token();
    self.expect_first(&OpenBrace)?;

    let block = self.parse_block()?;

    Ok(make_loop_for_expr(iterable, iterator, block))
  }

  fn parse_loop_loop_expr(&mut self) -> Result<Box<Expr>, String> {
    self.expect_first(&OpenBrace)?;

    let block = self.parse_block()?;

    Ok(make_loop_loop_expr(block))
  }

  fn parse_loop_while_expr(&mut self) -> Result<Box<Expr>, String> {
    self.next_token();

    let condition = self.parse_expr_by_precedence(&Lowest)?;

    self.expect_first(&OpenBrace)?;

    let block = self.parse_block()?;

    Ok(make_loop_while_expr(condition, block))
  }

  fn parse_hash_expr(&mut self) -> Result<Box<Expr>, String> {
    let mut data = vec![];

    while !self.first_is(&CloseBrace) {
      self.next_token();

      let key = self.parse_expr_by_precedence(&Lowest)?;

      self.expect_first(&Colon)?;
      self.next_token();

      let value = self.parse_expr_by_precedence(&Lowest)?;

      data.push(make_hash_data_expr(key, value));

      if !self.first_is(&CloseBrace) {
        self.expect_first(&Comma)?;
      }
    }

    self.expect_first(&CloseBrace)?;

    Ok(make_hash_expr(data))
  }

  fn parse_nodes_ast(&mut self) -> Result<Box<Ast>, String> {
    let mut ast = Ast::new(vec![]);

    while self.token.kind() != EOF {
      match self.token.kind() {
        TokenKind::EOF => break,
        TokenKind::Indent(_) => {
          self.next_token();
          continue;
        }
        _ => match self.parse_stmt() {
          Err(error) => self.errors.push(error),
          Ok(stmt) => ast.add(stmt),
        },
      }

      self.next_token();
    }

    Ok(box ast)
  }

  fn parse_ret_stmt(&mut self) -> Result<Box<Stmt>, String> {
    self.next_token();

    let expr = self.parse_expr_by_precedence(&Lowest)?;

    while self.token.kind() != Semicolon && self.token.kind() != EOL {
      self.next_token();
    }

    Ok(make_ret_stmt(expr))
  }

  fn parse_stmt(&mut self) -> Result<Box<Stmt>, String> {
    match self.token.kind() {
      // TokenKind::Use => self.parse_use_stmt(),
      TokenKind::Fun => self.parse_fun_stmt(),
      TokenKind::Mut | TokenKind::Val => self.parse_local_stmt(),
      TokenKind::Ret => self.parse_ret_stmt(),
      _ => self.parse_expr_stmt(),
    }
  }

  fn parse_use_stmt(&mut self) -> Result<Box<Stmt>, String> {
    let name = self.parse_use_path_stmt()?;

    self.next_token();
    self.parse_until(&Semicolon)?;

    let nodes = self.parse_nodes_ast()?;

    Ok(make_use_stmt(&name, nodes))
  }

  fn parse_use_path_stmt(&mut self) -> Result<String, String> {
    let mut use_path = vec![];
    let mut use_path_items = vec![];

    self.expect_first(&TokenKind::At)?;
    self.next_token();

    let parent_path = self.token.text();
    use_path.push(parent_path);

    while self.first_is(&ColonColon) {
      self.parse_until(&ColonColon)?;
      self.next_token();

      let path = self.token.text();

      use_path.push(path);
    }

    if self.token.kind() == TokenKind::OpenParen {
      while self.first_is(&TokenKind::Ident(self.first.text())) {
        let use_name = self.token.text();

        use_path_items.push(use_name);
        self.next_token();
      }
    }

    Ok(use_path.join("::"))
  }

  fn parse_unop_expr(&mut self) -> Result<Box<Expr>, String> {
    let operand = self.token.text();

    self.next_token();

    let rhs = self.parse_expr_by_precedence(&Unary)?;

    Ok(make_unop_expr(&operand, rhs))
  }

  fn parse_until(
    &mut self,
    kind: &TokenKind,
  ) -> Result<Vec<Box<Expr>>, String> {
    let mut exprs: Vec<Box<Expr>> = vec![];

    if self.first_is(&kind) {
      self.next_token();

      return Ok(exprs);
    }

    self.next_token();

    exprs.push(self.parse_expr_by_precedence(&Lowest)?);

    while self.first_is(&Comma) {
      self.next_token();
      self.next_token();
      exprs.push(self.parse_expr_by_precedence(&Lowest)?);
    }

    self.expect_first(&kind)?;

    Ok(exprs)
  }

  fn process_to_completion(&mut self, token: Token) {
    self.tokens.push_back(token);
  }

  fn should_precedence_has_priority(&self, kind: &PrecedenceKind) -> bool {
    kind < &TokenKind::precedence(&self.first.kind())
  }

  fn token_is(&self, kind: &TokenKind) -> bool {
    self.token.kind() == *kind
  }
}

impl<Handle, Sink> TreeBuilderPrinter for TreeBuilder<Handle, Sink>
where
  Handle: Clone,
  Sink: TreeSink<Handle = Handle>,
{
  fn print(&mut self, _stmt: Box<Stmt>) {}
}

impl<Handle, Sink> TokenSink for TreeBuilder<Handle, Sink>
where
  Handle: Clone,
  Sink: TreeSink<Handle = Handle>,
{
  // runs the parsing after the scanning end
  fn end(&mut self) {
    self.next_token();
    self.next_token();

    let ast = self.parse_nodes_ast().unwrap();

    self.emit(ast);
  }

  fn print(&self, _level: usize) {}

  fn process_token(&mut self, token: Token) {
    self.process_to_completion(token);
  }
}
