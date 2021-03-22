#![feature(box_patterns)]
#![feature(box_syntax)]
#![feature(decl_macro)]
#![recursion_limit = "256"]

#[macro_use]
extern crate serde_derive;

mod interface;
mod loc;
mod span;

#[cfg(test)]
mod test;

pub use self::interface::{
  ColumnIndex, ColumnOffset, LineIndex, LineOffset, RawIndex, RawOffset,
};

pub use self::loc::Loc;
pub use self::span::Span;
