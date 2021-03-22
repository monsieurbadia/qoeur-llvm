use super::interface::{BinaryKind, TokenKind};
use super::Token;

use qoeurcp_span::{ColumnIndex, LineIndex, Loc, Span};

use std::collections::VecDeque;

pub type CompileResult<T> = Result<T, String>;

#[derive(Clone, Debug)]
pub struct TokenQueue {
  pub tokens: VecDeque<Token>,
  pub last_loc: Loc,
}

impl Iterator for TokenQueue {
  type Item = Token;

  fn next(&mut self) -> Option<Self::Item> {
    self.tokens.pop_front()
  }
}

impl TokenQueue {
  pub fn new() -> TokenQueue {
    Self {
      tokens: VecDeque::new(),
      last_loc: Loc::new(LineIndex(1), ColumnIndex(1)),
    }
  }

  pub fn push_back(&mut self, token: Token) {
    self.tokens.push_back(token);
  }

  pub fn loc(&self) -> Loc {
    self.last_loc
  }

  pub fn dump(&self) {
    self.tokens.iter().for_each(|t| println!("{:?}", t));
  }

  pub fn push_front(&mut self, token: Token) {
    self.tokens.push_front(token)
  }

  pub fn pop(&mut self) -> CompileResult<Token> {
    if let Some(token) = self.tokens.pop_front() {
      self.last_loc = token.span.end;
      return Ok(token);
    }

    Err(format!(""))
  }

  pub fn peek(&self) -> Option<&Token> {
    self.tokens.front()
  }

  pub fn peek_at(&self, index: usize) -> Option<&Token> {
    self.tokens.get(index)
  }

  pub fn expect(&mut self, kind: &TokenKind) -> CompileResult<Token> {
    self.pop().and_then(|token| {
      if token.kind == *kind {
        return Ok(token);
      }

      Err(format!(""))
    })
  }

  pub fn expect_int(&mut self) -> CompileResult<(i64, Span)> {
    let _token = self.pop()?;

    // if let TokenKind::Number(ref number) = token.kind {
    //   let n = number.parse::<i64>().map_err(|_| {
    //     Err(format!(""))
    //   })?;

    //   return Ok((n, token.span));
    // }

    Err(format!(""))
  }

  pub fn expect_identifier(&mut self) -> CompileResult<(String, Span)> {
    let token = self.pop()?;

    if let TokenKind::Ident(s) = token.kind {
      return Ok((s, token.span));
    }

    Err(format!(""))
  }

  pub fn expect_binary_operator(&mut self) -> CompileResult<BinaryKind> {
    let token = self.pop()?;

    if let TokenKind::Binary(op) = token.kind {
      return Ok(op);
    }

    Err(format!(""))
  }

  pub fn is_next(&self, kind: &TokenKind) -> bool {
    match self.tokens.front() {
      Some(ref token) => token.kind == *kind,
      None => false,
    }
  }

  pub fn is_next_at(&self, idx: usize, kind: &TokenKind) -> bool {
    match self.peek_at(idx) {
      Some(ref token) => token.kind == *kind,
      None => false,
    }
  }

  pub fn is_next_binary_operator(&self) -> bool {
    match self.tokens.front() {
      Some(token) => {
        if let TokenKind::Binary(_) = token.kind {
          return true;
        }

        false
      }
      None => false,
    }
  }

  pub fn is_next_assign_operator(&self) -> Option<BinaryKind> {
    match self.tokens.front() {
      Some(token) => {
        if let TokenKind::AssignOp(op) = &token.kind {
          return Some(op.to_owned());
        }

        None
      }
      _ => None,
    }
  }

  pub fn is_next_identifier(&self, value: &str) -> bool {
    match self.tokens.front() {
      Some(token) => {
        if let TokenKind::Ident(ref v) = token.kind {
          return *v == *value;
        }

        false
      }
      None => false,
    }
  }

  pub fn is_in_same_block(&self, indent_level: usize) -> bool {
    match self.tokens.front() {
      Some(token) => {
        if let TokenKind::Indent(level) = token.kind {
          return level >= indent_level;
        }

        token.kind != TokenKind::EOF
      }
      None => false,
    }
  }

  pub fn pop_indent(&mut self) -> CompileResult<Option<(usize, Span)>> {
    let level = if let Some(token) = self.tokens.front() {
      if let TokenKind::Indent(level) = token.kind {
        level
      } else {
        return Ok(None);
      }
    } else {
      return Ok(None);
    };

    let token = self.pop()?;

    Ok(Some((level, token.span)))
  }
}
