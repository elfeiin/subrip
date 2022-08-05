pub mod timestamp;

use crate::error::Error;
use core::fmt::Display;
use timestamp::Timestamp;

pub struct Builder<'src_buf> {
   pub(crate) line_number: usize,
   pub(crate) start: Option<&'src_buf str>,
   pub(crate) end: Option<&'src_buf str>,
   pub(crate) lines: Vec<&'src_buf str>,
}

impl<'src_buf> Builder<'src_buf> {
   /// Builds an `Entry`.
   /// # Errors
   /// If any `Timestamp`s are invalid.
   pub fn build(self) -> Result<Entry<'src_buf>, Error> {
      let start = if let Some(s) = self.start {
         Timestamp::from_str_slice(s)?
      } else {
         return Err(Error::MissingTimestamp(self.line_number + 1));
      };
      let end = if let Some(s) = self.end {
         Timestamp::from_str_slice(s)?
      } else {
         return Err(Error::MissingTimestamp(self.line_number + 1));
      };
      let lines = self.lines;
      Ok(Entry { start, end, lines })
   }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Entry<'src_buf> {
   pub start: Timestamp,
   pub end: Timestamp,
   pub lines: Vec<&'src_buf str>,
}

impl<'src_buf> Entry<'src_buf> {
   #[must_use]
   pub const fn builder() -> Builder<'src_buf> {
      Builder {
         line_number: 0,
         start: None,
         end: None,
         lines: vec![],
      }
   }
}

impl<'src_buf> Display for Entry<'src_buf> {
   fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
      write![
         f,
         "{} ---> {}\n{}",
         self.start,
         self.end,
         self.lines.join("\n")
      ]
   }
}

impl<'src_buf> PartialOrd for Entry<'src_buf> {
   fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
      Some(self.start.cmp(&other.start))
   }
}

impl<'src_buf> Ord for Entry<'src_buf> {
   fn cmp(&self, other: &Self) -> std::cmp::Ordering {
      self.start.cmp(&other.start)
   }
}
