use std::fmt::Debug;
use binrw::Error;

#[derive(Debug)]
pub(crate) enum DecodingError {
    VarintOverflow,
    IoError(std::io::ErrorKind),
    NonUTF8String
}

impl std::fmt::Display for DecodingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self, f)
    }
}

impl std::error::Error for DecodingError {}

impl From<leb128::read::Error> for DecodingError {
    fn from(value: leb128::read::Error) -> Self {
        match value {
            leb128::read::Error::IoError(v) => Self::IoError(v.kind()),
            leb128::read::Error::Overflow => Self::VarintOverflow,
        }
    }
}

impl From<DecodingError> for Error {
    fn from(value: DecodingError) -> Self {
        Self::Custom {
            pos: 0,
            err: Box::new(value),
        }
    }
}
