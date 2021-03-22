pub use self::SetResult::{FromSet, NotFromSet};

use crate::util::smallcharset::SmallCharSet;

use std::collections::VecDeque;

use tendril::StrTendril;

#[derive(PartialEq, Eq, Debug)]
pub enum SetResult {
  FromSet(char),
  NotFromSet(StrTendril),
}

pub struct BufferQueue {
  buffers: VecDeque<StrTendril>,
}

impl BufferQueue {
  pub fn new() -> BufferQueue {
    BufferQueue {
      buffers: VecDeque::with_capacity(16),
    }
  }

  pub fn push_front(&mut self, buf: StrTendril) {
    if buf.len32() == 0 {
      return;
    }

    self.buffers.push_front(buf);
  }

  pub fn push_back(&mut self, buf: StrTendril) {
    if buf.len32() == 0 {
      return;
    }

    self.buffers.push_back(buf);
  }

  pub fn peek(&mut self) -> Option<char> {
    self.buffers.front().map(|b| b.chars().next().unwrap())
  }

  pub fn next(&mut self) -> Option<char> {
    let (result, now_empty) = match self.buffers.front_mut() {
      None => (None, false),
      Some(buf) => {
        let c = buf.pop_front_char().expect("empty buffer in queue");
        (Some(c), buf.is_empty())
      }
    };

    if now_empty {
      self.buffers.pop_front();
    }

    result
  }

  pub fn pop_except_from(&mut self, set: SmallCharSet) -> Option<SetResult> {
    let (result, now_empty) = match self.buffers.front_mut() {
      None => (None, false),
      Some(buf) => {
        let n = set.nonmember_prefix_len(&buf);
        if n > 0 {
          let out;
          unsafe {
            out = buf.unsafe_subtendril(0, n);
            buf.unsafe_pop_front(n);
          }
          (Some(NotFromSet(out)), buf.is_empty())
        } else {
          let c = buf.pop_front_char().expect("empty buffer in queue");
          (Some(FromSet(c)), buf.is_empty())
        }
      }
    };

    if now_empty {
      self.buffers.pop_front();
    }

    result
  }

  pub fn eat(&mut self, pat: &str) -> Option<bool> {
    let mut buffers_exhausted = 0;
    let mut consumed_from_last = 0;
    if self.buffers.front().is_none() {
      return None;
    }

    for pattern_byte in pat.bytes() {
      if buffers_exhausted >= self.buffers.len() {
        return None;
      }
      let ref buf = self.buffers[buffers_exhausted];

      if !buf.as_bytes()[consumed_from_last].eq_ignore_ascii_case(&pattern_byte)
      {
        return Some(false);
      }

      consumed_from_last += 1;
      if consumed_from_last >= buf.len() {
        buffers_exhausted += 1;
        consumed_from_last = 0;
      }
    }

    for _ in 0..buffers_exhausted {
      self.buffers.pop_front();
    }

    match self.buffers.front_mut() {
      None => assert_eq!(consumed_from_last, 0),
      Some(ref mut buf) => buf.pop_front(consumed_from_last as u32),
    }

    Some(true)
  }
}
