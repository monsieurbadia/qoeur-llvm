use super::{Token, TokenSink};

pub struct TokenPrinter {}

impl TokenPrinter {
  pub fn new() -> TokenPrinter {
    Self {}
  }
}

impl TokenSink for TokenPrinter {
  fn end(&mut self) {}

  fn print(&self, _level: usize) {}

  fn process_token(&mut self, token: Token) {
    println!("token_printer: {}", token);
    match token.kind {
      _ => {}
    }
  }
}
