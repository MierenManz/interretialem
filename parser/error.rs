use leb128::read;
use std::fmt::Debug;

pub enum ValidationError {
    BadCookie,
    UnsupportedWasmVersion,
}

#[derive(Debug)]
pub enum DecodingError {
    UnknownValType,
    UnknownSection,
    Unknown,
    DuplicateSection,
    IoError(std::io::ErrorKind),
    VarintOverflow,
}

impl std::fmt::Display for DecodingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self, f)
    }
}

impl std::error::Error for DecodingError {}

impl From<std::io::Error> for DecodingError {
    fn from(value: std::io::Error) -> Self {
        Self::IoError(value.kind())
    }
}

impl From<read::Error> for DecodingError {
    fn from(value: read::Error) -> Self {
        match value {
            read::Error::IoError(e) => e.into(),
            read::Error::Overflow => Self::VarintOverflow,
        }
    }
}
