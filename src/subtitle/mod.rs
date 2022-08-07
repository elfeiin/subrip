pub mod entry;

use crate::error::Error;
use core::fmt::Display;
use entry::Entry;

enum ParseState {
   Empty,
   Timing,
   Lines,
}

#[derive(Debug)]
pub struct Subtitles<'src_buf> {
   pub entries: Vec<Entry<'src_buf>>,
}

impl<'src_buf> Subtitles<'src_buf> {
   #[allow(clippy::should_implement_trait)]
   pub fn from_str(input: &'src_buf str) -> Result<Self, Error> {
      let mut entries: Vec<Entry> = vec![];
      let mut state = ParseState::Empty;
      let mut curr_entry = Entry::builder();
      let mut lines = input.lines().enumerate();
      loop {
         match lines.next() {
            Some((line_number, line)) => match state {
               ParseState::Empty => {
                  curr_entry.line_number = line_number;
                  let trimmed = line.trim();
                  if trimmed.is_empty() {
                     continue;
                  }
                  if trimmed.parse::<usize>().is_err() {
                     return Err(Error::UnexpectedToken(line_number));
                  }
                  state = ParseState::Timing;
               }
               ParseState::Timing => {
                  let split = line.split_whitespace().collect::<Vec<&str>>();
                  match split.len().cmp(&3) {
                     std::cmp::Ordering::Less => {
                        return Err(Error::MissingTimestamp(line_number));
                     }

                     std::cmp::Ordering::Equal => {
                        if split[1] != "-->" {
                           return Err(Error::UnexpectedToken(line_number));
                        }
                        curr_entry.start = Some(split[0]);
                        curr_entry.end = Some(split[2]);
                     }
                     std::cmp::Ordering::Greater => {
                        return Err(Error::UnexpectedToken(line_number));
                     }
                  }
                  state = ParseState::Lines;
               }
               ParseState::Lines => {
                  if line.trim().is_empty() {
                     entries.push(curr_entry.build()?);
                     curr_entry = Entry::builder();
                     state = ParseState::Empty;
                  } else {
                     curr_entry.lines.push(line);
                  }
               }
            },
            None => {
               if !curr_entry.is_empty() {
                  entries.push(curr_entry.build()?);
               }
               break;
            }
         }
      }
      Ok(Self { entries })
   }
}

impl<'src_buf> Display for Subtitles<'src_buf> {
   fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
      write![
         f,
         "{}",
         self
            .entries
            .iter()
            .enumerate()
            .map(|(i, entry)| format!["{}\n{entry}", i + 1])
            .collect::<Vec<String>>()
            .join("\n\n")
      ]
   }
}
