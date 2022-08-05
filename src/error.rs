#[derive(Debug)]
pub enum SrtErrorKind {
   MalformedTimestamp,
   MissingTimestamp,
   UnexpectedToken,
}

macro_rules! methods {
   ($($id:ident),*$(,)+) => {
      $(
         #[allow(non_snake_case)]
         pub fn $id(line_number: impl Into<Option<usize>>) -> Self {
            Self::new(line_number, SrtErrorKind::$id)
         }
      )*
   };
}

#[derive(Debug)]
pub struct Error {
   pub line_number: Option<usize>,
   pub kind: SrtErrorKind,
}

impl Error {
   pub fn new(location: impl Into<Option<usize>>, kind: SrtErrorKind) -> Self {
      Self {
         line_number: location.into(),
         kind,
      }
   }
   methods! {
         MalformedTimestamp,
         MissingTimestamp,
         UnexpectedToken,
   }
}
