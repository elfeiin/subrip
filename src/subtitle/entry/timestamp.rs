use crate::error::Error;
use core::fmt::Display;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Timestamp {
   pub hours: u8,
   pub minutes: u8,
   pub seconds: u8,
   pub milliseconds: u16,
}

impl Timestamp {
   /// # Errors
   /// If any numbers are invalid. May fail on trying to parse numbers other than ASCII.
   pub fn from_str_slice(s: &str) -> Result<Self, Error> {
      let mut hours = None;
      let mut minutes = None;
      let mut seconds = None;
      let mut milliseconds = None;
      let mut num_buf = String::new();
      for (i, ch) in s.char_indices() {
         if ch.is_numeric() {
            num_buf.push(ch);
         } else {
            match (&hours, &minutes, &seconds, &milliseconds) {
               (None, ..) => {
                  hours = Some(if let Ok(n) = num_buf.parse() {
                     n
                  } else {
                     return Err(Error::MalformedTimestamp(i));
                  });
               }
               (_, None, ..) => {
                  minutes = Some(if let Ok(n) = num_buf.parse() {
                     n
                  } else {
                     return Err(Error::MalformedTimestamp(i));
                  });
               }
               (_, _, None, ..) => {
                  seconds = Some(if let Ok(n) = num_buf.parse() {
                     n
                  } else {
                     return Err(Error::MalformedTimestamp(i));
                  });
               }
               (.., None) => {
                  milliseconds = Some(if let Ok(n) = num_buf.parse() {
                     n
                  } else {
                     return Err(Error::MalformedTimestamp(i));
                  });
               }
               _ => break,
            }
            num_buf.clear();
         }
      }
      milliseconds = Some(if let Ok(n) = num_buf.parse() {
         n
      } else {
         return Err(Error::MalformedTimestamp(s.len() - num_buf.len()));
      });

      if let (Some(hours), Some(minutes), Some(seconds), Some(milliseconds)) =
         (hours, minutes, seconds, milliseconds)
      {
         Ok(Self {
            hours,
            minutes,
            seconds,
            milliseconds,
         })
      } else {
         Err(Error::MalformedTimestamp(0))
      }
   }
}

impl Display for Timestamp {
   fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
      write![
         f,
         "{}:{}:{},{}",
         self.hours, self.minutes, self.seconds, self.milliseconds
      ]
   }
}

impl PartialOrd for Timestamp {
   fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
      Some(
         (self.hours, self.minutes, self.seconds, self.milliseconds).cmp(&(
            other.hours,
            other.minutes,
            other.seconds,
            other.milliseconds,
         )),
      )
   }
}

impl Ord for Timestamp {
   fn cmp(&self, other: &Self) -> std::cmp::Ordering {
      (self.hours, self.minutes, self.seconds, self.milliseconds).cmp(&(
         other.hours,
         other.minutes,
         other.seconds,
         other.milliseconds,
      ))
   }
}
