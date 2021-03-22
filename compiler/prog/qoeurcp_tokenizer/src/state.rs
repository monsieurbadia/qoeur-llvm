pub use self::TokenizerState::*;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub enum TokenizerState {
  Char,
  Comment,
  Ident,
  Number,
  Op,
  Quiescent,
  Start,
  Str,
}
