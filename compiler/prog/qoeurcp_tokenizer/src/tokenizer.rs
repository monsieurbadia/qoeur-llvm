use super::buffer_queue::{BufferQueue, FromSet, NotFromSet, SetResult};
use super::state::TokenizerState;

use super::token::{
  LiteralKind, NumberBase, Token, TokenKind, TokenPrinter, TokenQueue,
  TokenSink,
};

use super::util::ascii::*;
use super::util::smallcharset::{small_char_set, SmallCharSet};

use qoeurcp_span::{
  ColumnIndex, ColumnOffset, LineIndex, LineOffset, Loc, Span,
};

use std::borrow::Cow;
use std::mem;
use std::path::Path;

use tendril::StrTendril;

macro get_char( $me:expr ) {
  ::mac::unwrap_or_return!($me.get_char(), false)
}

macro pop_except_from( $me:expr, $set:expr ) {
  ::mac::unwrap_or_return!($me.pop_except_from($set), false)
}

static INDENT_LEVEL_NEWLINE: usize = 0;
static INDENT_LEVEL_WHITESPACE: usize = 1;
static INDENT_LEVEL_TAB: usize = 4;

pub fn tokenize_program_to<Sink, It>(
  sink: Sink,
  input: It,
  opts: TokenizerOpts,
) -> Sink
where
  Sink: TokenSink,
  It: IntoIterator<Item = tendril::StrTendril>,
{
  let mut tokenizer = Tokenizer::new(sink, opts);

  input.into_iter().for_each(|s| tokenizer.feed(s));
  tokenizer.end();
  tokenizer.unwrap()
}

pub fn tokenize(source: &str) -> Result<TokenQueue, String> {
  let path = Path::new(source);

  let f = match std::fs::read_to_string(&path) {
    Err(_) => None,
    Ok(file) => Some(file),
  };

  match f {
    None => Err(format!("")),
    Some(file) => {
      println!("\n{}", file);

      let opts = TokenizerOpts {
        profile: true,
        exact_errors: true,
        ..Default::default()
      };

      let buffer = StrTendril::from(file);
      let sink = TokenPrinter::new();
      let mut tokenizer = Tokenizer::new(sink, opts);
      let _ = tokenizer.feed(buffer.try_reinterpret().unwrap());
      let _ = tokenizer.end();

      Ok(tokenizer.token_queue)
    }
  }
}

#[derive(Copy, Clone)]
pub struct TokenizerOpts {
  pub exact_errors: bool,
  pub initial_state: Option<TokenizerState>,
  pub profile: bool,
  pub safe_mod: bool,
}

impl Default for TokenizerOpts {
  fn default() -> TokenizerOpts {
    Self {
      exact_errors: false,
      initial_state: None,
      profile: false,
      safe_mod: true,
    }
  }
}

pub struct Tokenizer<Sink> {
  pub token_queue: TokenQueue,
  at_eof: bool,
  data: String,
  escape_code: bool,
  current_char: char,
  current_base_number: NumberBase,
  current_token: TokenKind,
  ignore_lf: bool,
  indent_level: usize,
  input_buffers: BufferQueue,
  loc: Loc,
  opts: TokenizerOpts,
  reconsume: bool,
  sink: Sink,
  state: TokenizerState,
  token_start_loc: Loc,
}

impl<Sink: TokenSink> Tokenizer<Sink> {
  pub fn new(sink: Sink, opts: TokenizerOpts) -> Tokenizer<Sink> {
    let state = *opts
      .initial_state
      .as_ref()
      .unwrap_or(&TokenizerState::Start);

    Self {
      at_eof: false,
      current_base_number: NumberBase::Int,
      current_char: '\0',
      current_token: TokenKind::EOF,
      data: String::new(),
      escape_code: false,
      ignore_lf: false,
      indent_level: 0,
      input_buffers: BufferQueue::new(),
      loc: Loc::new(LineIndex(1), ColumnIndex(1)),
      opts: opts,
      reconsume: false,
      sink: sink,
      state: state,
      token_queue: TokenQueue::new(),
      token_start_loc: Loc::new(LineIndex(1), ColumnIndex(1)),
    }
  }

  pub fn emit_token(&mut self, kind: TokenKind, span: Span) {
    self.process_token(Token::new(kind, span));
  }

  pub fn end(&mut self) {
    self.at_eof = true;
    self.run();

    while self.eof_step() { /* loop */ }

    self.sink.end();
  }

  pub fn feed(&mut self, input: StrTendril) {
    if input.len() == 0 {
      return;
    }

    self.input_buffers.push_back(input);
    self.run();
  }

  pub fn unwrap(self) -> Sink {
    self.sink
  }

  fn add(&mut self, kind: TokenKind, span: Span) {
    self
      .token_queue
      .push_back(Token::new(kind.clone(), span.clone()));

    self.sink.process_token(Token::new(kind, span));
  }

  fn current_span(&self) -> Span {
    Span::new(
      self.token_start_loc,
      Loc::new(self.loc.line, ColumnIndex(u32::from(self.loc.column) - 1)),
    )
  }

  fn current_single_span(&self) -> Span {
    Span::from_start(self.loc)
  }

  fn eat(&mut self, pattern: &str) -> Option<bool> {
    match self.input_buffers.eat(pattern) {
      None if self.at_eof => Some(false),
      r => r,
    }
  }

  fn emit_error(&mut self, error: Cow<'static, str>) {
    let span = self.current_span();
    let error = TokenKind::ParseError(error);

    self.process_token(Token::new(error, span));
  }

  fn emit_eof(&mut self) {
    let span = self.current_span();
    self.add(TokenKind::EOF, span);
  }

  fn eof_step(&mut self) -> bool {
    self.emit_eof(); // TODO: tmp
    match self.state {
      TokenizerState::Quiescent => {
        self.emit_eof();
        return false;
      }
      _ => {
        return false;
      }
    }
  }

  fn get_char(&mut self) -> Option<char> {
    if self.reconsume {
      self.reconsume = false;
      Some(self.current_char)
    } else {
      self
        .input_buffers
        .next()
        .and_then(|c| self.get_preprocessed_char(c))
    }
  }

  fn get_preprocessed_char(&mut self, mut c: char) -> Option<char> {
    if self.ignore_lf {
      self.ignore_lf = false;

      if c == '\n' {
        c = mac::unwrap_or_return!(self.input_buffers.next(), None);
      }
    }

    if c == '\r' {
      self.ignore_lf = true;
      c = '\n';
    }

    if c == '\x00' {
      c = '\u{FFFD}'
    }

    self.loc.column += ColumnOffset(1);
    self.current_char = c;

    Some(c)
  }

  fn process_escape_sequence(&mut self, c: char, end: char) {
    if self.escape_code {
      self.escape_code = false;
      match c {
        'r' => self.data.push('\r'),
        'n' => self.data.push('\n'),
        't' => self.data.push('\t'),
        _ => self.data.push(c),
      }
    } else if c == '\\' {
      self.escape_code = true;
    } else if c != end {
      self.data.push(c);
    }
  }

  fn peek(&mut self) -> Option<char> {
    if self.reconsume {
      Some(self.current_char)
    } else {
      self.input_buffers.peek()
    }
  }

  fn pop_except_from(&mut self, set: SmallCharSet) -> Option<SetResult> {
    if self.opts.exact_errors || self.reconsume || self.ignore_lf {
      return self.get_char().map(|x| FromSet(x));
    }

    let d = self.input_buffers.pop_except_from(set);
    match d {
      Some(FromSet(c)) => self.get_preprocessed_char(c).map(|x| FromSet(x)),
      _ => d,
    }
  }

  fn process_token(&mut self, token: Token) {
    self.sink.process_token(token);
  }

  fn run(&mut self) {
    while self.step() { /* loop */ }
  }

  fn start(&mut self, c: char, state: TokenizerState) {
    self.token_start_loc = self.loc;
    self.state = state;

    self.data.clear();

    if self.state != TokenizerState::Str && self.state != TokenizerState::Char {
      self.data.push(c);
    }
  }

  fn start_newline(&mut self) {
    if self.at_eof {
      self.state = TokenizerState::Quiescent;
    } else {
      self.state = TokenizerState::Start;
      self.loc.line += LineOffset(1);
    }

    self.indent_level = 0;
  }

  fn step(&mut self) -> bool {
    match self.state {
      TokenizerState::Start => loop {
        match pop_except_from!(self, small_char_set!(' ' '\n' '\t')) {
          FromSet(' ') => {
            self.indent_level = INDENT_LEVEL_WHITESPACE;
            return true;
          }
          FromSet('\n') => {
            self.indent_level += INDENT_LEVEL_NEWLINE;

            return true;
          }
          FromSet('\t') => {
            self.indent_level += INDENT_LEVEL_TAB;

            return true;
          }
          FromSet(c) => {
            let level = self.indent_level;
            let span = Span::new(self.token_start_loc, self.loc);
            self.reconsume = true;

            self.add(TokenKind::Indent(level), span);
            self.start(c, TokenizerState::Quiescent);
            return true;
          }
          NotFromSet(..) => {
            return false;
          }
        }
      },
      TokenizerState::Quiescent => loop {
        match get_char!(self) {
          '\n' => {
            self.start_newline();
            return true;
          }
          ' ' | '\t' => {
            return true;
          }
          '#' => {
            self.state = TokenizerState::Quiescent;
            let span = self.current_span();

            self.add(TokenKind::Comma, span);
            return true;
          }
          ',' => {
            self.state = TokenizerState::Quiescent;
            let span = self.current_span();

            self.add(TokenKind::Comma, span);
            return true;
          }
          '(' => {
            self.state = TokenizerState::Quiescent;
            let span = self.current_span();

            self.add(TokenKind::OpenParen, span);
            return true;
          }
          ')' => {
            self.state = TokenizerState::Quiescent;
            let span = self.current_span();

            self.add(TokenKind::CloseParen, span);
            return true;
          }
          '{' => {
            self.state = TokenizerState::Quiescent;
            let span = self.current_span();

            self.add(TokenKind::OpenBrace, span);
            return true;
          }
          '}' => {
            self.state = TokenizerState::Quiescent;
            let span = self.current_span();

            self.add(TokenKind::CloseBrace, span);
            return true;
          }
          '[' => {
            self.state = TokenizerState::Quiescent;
            let span = self.current_span();

            self.add(TokenKind::OpenBracket, span);
            return true;
          }
          ']' => {
            self.state = TokenizerState::Quiescent;
            let span = self.current_span();

            self.add(TokenKind::CloseBracket, span);
            return true;
          }
          '$' => {
            self.state = TokenizerState::Quiescent;
            let span = self.current_span();

            self.add(TokenKind::Dollar, span);
            return true;
          }
          ':' => {
            if self.peek() == Some(':') {
              self.state = TokenizerState::Quiescent;
              let span = self.current_span();
              let kind = TokenKind::glue("::");

              self.input_buffers.next();
              self.add(kind, span);
              return true;
            } else {
              self.state = TokenizerState::Quiescent;
              let span = self.current_span();

              self.add(TokenKind::Colon, span);
              return true;
            }
          }
          ';' => {
            self.state = TokenizerState::Quiescent;
            let span = self.current_span();

            self.add(TokenKind::Semicolon, span);
            return true;
          }
          '?' => {
            self.state = TokenizerState::Quiescent;
            let span = self.current_span();

            self.add(TokenKind::QuestionMark, span);
            return true;
          }
          '@' => {
            self.state = TokenizerState::Quiescent;
            let span = self.current_span();

            self.add(TokenKind::At, span);
            return true;
          }
          c if is_double_quote(c) => {
            self.start(self.current_char, TokenizerState::Str);
            return true;
          }
          c if is_single_quote(c) => {
            self.start(self.current_char, TokenizerState::Char);
            return true;
          }
          c if is_id_start(c) => {
            self.start(c, TokenizerState::Ident);
            return true;
          }
          c if is_operator(c) => {
            self.start(c, TokenizerState::Op);
            return true;
          }
          c if is_number(c) => {
            self.start(c, TokenizerState::Number);
            return true;
          }
          _ => {
            self.state = TokenizerState::Quiescent;
            let span = self.current_span();

            self.add(TokenKind::Unknown, span);
            return false;
          }
        }
      },
      TokenizerState::Comment => loop {
        match get_char!(self) {
          c if c == '\n' => {
            self.start_newline();
            return true;
          }
          _ => {}
        }
      },
      TokenizerState::Char => loop {
        self.process_escape_sequence(self.current_char, '\'');

        match get_char!(self) {
          c if c == '\'' => {
            let mut span = self.current_span();
            span.end.column += ColumnOffset(1);

            if self.data.len() != 1 {
              return false; // FIXME: handle an error instead
            }

            let ch = self.data.chars().nth(0).expect("Invalid char literal");

            self.data.clear();
            self.add(TokenKind::Literal(LiteralKind::CharAscii(ch)), span);

            self.escape_code = false;
            self.state = TokenizerState::Quiescent;
          }
          _ => {}
        }
      },
      TokenizerState::Ident => loop {
        match get_char!(self) {
          c if is_id_continue(c) => {
            self.state = TokenizerState::Ident;
            self.data.push(c);
            return true;
          }
          _ => {
            self.state = TokenizerState::Quiescent;
            self.reconsume = true;
            let span = self.current_span();
            let kind = TokenKind::keyword(&self.data[..]);

            self.add(kind, span);
            return true;
          }
        }
      },
      TokenizerState::Number => loop {
        match get_char!(self) {
          c if is_number(c) => {
            self.current_base_number = NumberBase::Int;
            self.state = TokenizerState::Number;

            self.data.push(c);
            return true;
          }
          c if c == '.' => {
            self.current_base_number = NumberBase::Dec;
            self.state = TokenizerState::Number;

            self.data.push(c);
            return true;
          }
          c if c == 'e' => {
            self.current_base_number = NumberBase::Hex;
            self.state = TokenizerState::Number;

            self.data.push(c);
            return true;
          }
          _ => {
            self.state = TokenizerState::Quiescent;
            let span = self.current_span();
            let num = mem::replace(&mut self.data, String::new());

            let kind = match num.trim().parse::<u64>().is_ok() {
              true => TokenKind::Literal(LiteralKind::IntNumber(num)),
              false => TokenKind::Literal(LiteralKind::RealNumber(num)),
            };

            self.add(kind, span);
            return true;
          }
        }
      },
      TokenizerState::Op => loop {
        match get_char!(self) {
          c if c.is_whitespace()
            || c.is_alphanumeric()
            || c == '{'
            || c == '('
            || c == '['
            || c == '}'
            || c == ')'
            || c == ']'
            || c == '$'
            || c == ','
            || c == '_' =>
          {
            self.state = TokenizerState::Quiescent;
            let span = self.current_span();
            let kind = TokenKind::glue(&self.data[..]);

            self.add(kind, span);
            return true;
          }
          c => {
            self.data.push(c);
            return true;
          }
        }
      },
      TokenizerState::Str => loop {
        self.process_escape_sequence(self.current_char, '"');

        match get_char!(self) {
          c if c == '"' => {
            self.escape_code = false;
            self.state = TokenizerState::Quiescent;
            let s = mem::replace(&mut self.data, String::new());
            let mut span = self.current_span();
            span.end.column += ColumnOffset(1);

            self.add(TokenKind::Literal(LiteralKind::StrBuffer(s)), span);
            return true;
          }
          _ => {}
        }
      },
    }
  }
}
