#![allow(dead_code)]

pub use self::BinaryKind::*;
pub use self::LiteralKind::*;
pub use self::NumberBase::*;
pub use self::PrecedenceKind::*;
pub use self::TokenKind::*;
pub use self::UnaryKind::*;

use super::Token;

use std::borrow::Cow;
use std::fmt;

macro symbols {
  { $type:tt { $($kind:ident,)* } } => {
    impl std::fmt::Display for $type {
      fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
          $($kind(ref value) => write!(f, "{}", *value),)*
        }
      }
    }
  },
  { $type:tt { $($kind:ident: $value:expr,)* } } => {
    impl std::fmt::Display for $type {
      fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
          $($kind => write!(f, "{}", $value),)*
        }
      }
    }
  },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BinaryKind {
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
  EqEq,
  And,
  AndAnd,
  Or,
  OrOr,
  Dot,
  DotDot,
}

symbols! {
  BinaryKind {
    Add: "+",
    Sub: "-",
    Mul: "*",
    Div: "/",
    Mod: "%",
    Lt: "<",
    Gt: ">",
    Le: "<=",
    Ge: ">=",
    Ne: "!=",
    Eq: "=",
    EqEq: "==",
    And: "&",
    AndAnd: "&&",
    Or: "|",
    OrOr: "||",
    Dot: ".",
    DotDot: "..",
  }
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum PrecedenceKind {
  Lowest,
  Assignement,
  Conditional,
  Sum,
  Exponent,
  Unary,
  Calling,
  Index,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum NumberBase {
  Int,
  Bin,
  Dec,
  Hex,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LiteralKind {
  RealNumber(String),
  IntNumber(String),
  StrBuffer(String),
  CharAscii(char),
}

symbols! {
  LiteralKind {
    RealNumber,
    IntNumber,
    StrBuffer,
    CharAscii,
  }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TokenKind {
  EOF,
  EOL,
  Ident(String),
  Literal(LiteralKind),
  Binary(BinaryKind),
  Unary(UnaryKind),
  AssignOp(BinaryKind),
  Indent(usize),
  OpenBrace,
  CloseBrace,
  OpenBracket,
  CloseBracket,
  OpenParen,
  CloseParen,
  Arrow,
  ArrowFat,
  At,
  Attr,
  BackSlash,
  Colon,
  ColonColon,
  Comma,
  Dollar,
  DollarDotDot,
  QuestionMark,
  Shebang,
  Semicolon,
  As,
  Async,
  Await,
  Break,
  Capsule,
  Continue,
  Else,
  Enum,
  Exp,
  Ext,
  False,
  For,
  Fun,
  If,
  Load,
  Loop,
  Match,
  Module,
  Mut,
  Pub,
  Ref,
  Ret,
  SelfLower,
  SelfUpper,
  Set,
  Static,
  Struct,
  Super,
  True,
  Type,
  Typeof,
  Underscore,
  Unsafe,
  Use,
  Val,
  Void,
  While,
  Unknown,
  ParseError(Cow<'static, str>),
}

impl fmt::Display for TokenKind {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.text())
  }
}

impl TokenKind {
  pub fn keyword(name: &str) -> TokenKind {
    match name {
      "fun" => Self::Fun,
      "mut" => Self::Mut,
      "use" => Self::Use,
      "val" => Self::Val,
      _ => Self::Ident(name.into()),
    }
  }

  pub fn glue(symbol: &str) -> TokenKind {
    match symbol {
      "+" => Self::Binary(Add),
      "-" => Self::Binary(Sub),
      "*" => Self::Binary(Mul),
      "/" => Self::Binary(Div),
      "%" => Self::Binary(Mod),
      "<" => Self::Binary(Lt),
      ">" => Self::Binary(Gt),
      "<=" => Self::Binary(Le),
      ">=" => Self::Binary(Ge),
      "=" => Self::AssignOp(Eq),
      "+=" => Self::AssignOp(Eq),
      "-=" => Self::AssignOp(Sub),
      "*=" => Self::AssignOp(Mul),
      "/=" => Self::AssignOp(Div),
      "==" => Self::Binary(EqEq),
      "!" => Self::Unary(UnaryKind::Not),
      "!=" => Self::Binary(Ne),
      "&&" => Self::Binary(And),
      "||" => Self::Binary(Or),
      "->" => Self::Arrow,
      "=>" => Self::ArrowFat,
      ":" => Self::Colon,
      "::" => Self::ColonColon,
      "|" => Self::Binary(Or),
      "." => Self::Binary(Dot),
      _ => Self::Unknown,
    }
  }

  pub fn precedence(kind: &TokenKind) -> PrecedenceKind {
    match kind {
      Self::Binary(Mul) | Self::Binary(Div) => PrecedenceKind::Exponent,
      Self::Binary(Add) | Self::Binary(Sub) => PrecedenceKind::Sum,
      Self::Binary(Lt)
      | Self::Binary(Le)
      | Self::Binary(Gt)
      | Self::Binary(Ge) => PrecedenceKind::Conditional,
      Self::Binary(Eq) | Self::Binary(Ne) => PrecedenceKind::Assignement,
      Self::OpenParen => PrecedenceKind::Calling,
      Self::OpenBracket => PrecedenceKind::Index,
      _ => PrecedenceKind::Lowest,
    }
  }

  pub fn text(&self) -> String {
    match *self {
      Self::EOF => format!("EOF"),
      Self::EOL => format!("EOL"),
      Self::OpenBrace => format!("{{"),
      Self::CloseBrace => format!("}}"),
      Self::OpenBracket => format!("["),
      Self::CloseBracket => format!("]"),
      Self::OpenParen => format!("("),
      Self::CloseParen => format!(")"),
      Self::Arrow => format!("->"),
      Self::ArrowFat => format!("=>"),
      Self::At => format!("@"),
      Self::Attr => format!("|>"),
      Self::BackSlash => format!("\\"),
      Self::Colon => format!(":"),
      Self::ColonColon => format!("::"),
      Self::Comma => format!(","),
      Self::Dollar => format!("$"),
      Self::DollarDotDot => format!("$.."),
      Self::QuestionMark => format!("?"),
      Self::Shebang => format!("#!"),
      Self::Semicolon => format!(";"),
      Self::As => format!("as"),
      Self::Async => format!("async"),
      Self::Await => format!("await"),
      Self::Break => format!("break"),
      Self::Capsule => format!("capsule"),
      Self::Continue => format!("continue"),
      Self::Else => format!("else"),
      Self::Enum => format!("enum"),
      Self::Exp => format!("exp"),
      Self::Ext => format!("ext"),
      Self::False => format!("false"),
      Self::For => format!("for"),
      Self::Fun => format!("fun"),
      Self::If => format!("if"),
      Self::Load => format!("load"),
      Self::Loop => format!("loop"),
      Self::Match => format!("match"),
      Self::Module => format!("mod"),
      Self::Mut => format!("mut"),
      Self::Pub => format!("pub"),
      Self::Ref => format!("ref"),
      Self::Ret => format!("ret"),
      Self::SelfLower => format!("self"),
      Self::SelfUpper => format!("Self"),
      Self::Set => format!("set"),
      Self::Static => format!("static"),
      Self::Struct => format!("struct"),
      Self::Super => format!("super"),
      Self::True => format!("true"),
      Self::Type => format!("type"),
      Self::Typeof => format!("typeof"),
      Self::Underscore => format!("_"),
      Self::Unsafe => format!("unsafe"),
      Self::Use => format!("use"),
      Self::Val => format!("val"),
      Self::Void => format!("void"),
      Self::While => format!("while"),
      Self::Unknown => format!("UNKNOWN"),
      Self::AssignOp(ref kind) => format!("{}", kind),
      Self::Binary(ref kind) => format!("{}", kind),
      Self::Ident(ref ident) => format!("{}", ident),
      Self::Indent(ref indent) => format!("{}", indent),
      Self::Literal(ref lit) => format!("{}", lit),
      Self::Unary(ref unop) => format!("{}", unop),
      Self::ParseError(ref error) => format!("{}", error),
    }
  }
}

pub trait TokenSink {
  fn end(&mut self);
  fn print(&self, level: usize);
  fn process_token(&mut self, token: Token);
}

pub trait TokenizeResult {
  type Sink: TokenSink + Default;
  fn get_result(sink: Self::Sink) -> Self;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UnaryKind {
  Not,
  Neg,
}

symbols! {
  UnaryKind {
    Not: "!",
    Neg: "-",
  }
}
