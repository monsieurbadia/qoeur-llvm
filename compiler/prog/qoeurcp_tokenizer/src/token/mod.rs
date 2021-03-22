mod interface;
mod token_printer;
mod token_queue;

pub use self::interface::{
  BinaryKind::{self, *},
  LiteralKind::{self, *},
  NumberBase::{self, *},
  PrecedenceKind::{self, *},
  TokenKind::{self, *},
  TokenSink,
  UnaryKind::{self, *},
};

pub use self::token_printer::TokenPrinter;
pub use self::token_queue::TokenQueue;

use qoeurcp_span::Span;

use std::fmt;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Token {
  pub kind: TokenKind,
  pub span: Span,
}

impl fmt::Display for Token {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.text())
  }
}

impl Token {
  pub fn new(kind: TokenKind, span: Span) -> Token {
    Self { kind, span }
  }

  pub fn kind(&self) -> TokenKind {
    self.kind.to_owned()
  }

  pub fn text(&self) -> String {
    format!("{}", self.kind.text())
  }
}
