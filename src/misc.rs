#[derive(Debug)]
pub struct Error(pub String);


macro_rules! impl_from {
    ($t:path) => {
        impl From<$t> for Error
        {
            fn from(err: $t) -> Self
            {
                Self(format!("{}", err))
            }
        }
    };
}

impl_from!(rusqlite::Error);


use std::fmt::Debug;
use lalrpop_util::ParseError;
impl<A: Debug, B: Debug, C: Debug> From<ParseError<A, B, C>> for Error    
{
    fn from(error: ParseError<A, B, C>) -> Self {
        Error(format!("{:?}", error))
    }
}
