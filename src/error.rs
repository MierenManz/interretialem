use std::error::Error;
use std::io::ErrorKind;

#[derive(Debug)]
pub(crate) enum ValidationError {
    TooManyElements,
    MultipleMemories,
}
impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}
impl Error for ValidationError {}

#[derive(Debug)]
pub(crate) enum DecodingError {
    IoError(ErrorKind),
    UnknownValueType,
    ExpectedNonZero,
    MinBiggerThanMax,
    VarintOverflow,
    InvalidFunctionSignature
}

impl From<std::io::Error> for DecodingError {
    fn from(value: std::io::Error) -> Self {
        Self::IoError(value.kind())
    }
}

impl From<leb128::read::Error> for DecodingError {
    fn from(value: leb128::read::Error) -> Self {
        match value {
            leb128::read::Error::IoError(e) => e.into(),
            leb128::read::Error::Overflow => Self::VarintOverflow,
        }
    }
}

impl std::fmt::Display for DecodingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}
impl Error for DecodingError {}
