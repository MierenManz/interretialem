use std::fmt::Debug;
use leb128::read;

#[derive(Clone, Copy, Debug)]
pub enum ValidationError {
    BadCookie,
    UnknownIndex,
    MaxBiggerThanMin,
    MoreBodiesThanFunctions,
    DuplicateExport,
    DataWithoutMemory,
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self, f)
    }
}

impl std::error::Error for ValidationError {}



#[derive(Clone, Copy, Debug)]
pub enum ParsingError {
    IoError(std::io::ErrorKind),
    ValidationError(ValidationError),
    UnsupportedWasmVersion,
    DuplicateSection,
    UnknownSection,
    VarintOverflow,
}

impl From<std::io::Error> for ParsingError {
    fn from(value: std::io::Error) -> Self {
        return Self::IoError(value.kind())
    }
}

impl From<ValidationError> for ParsingError {
    fn from(value: ValidationError) -> Self {
        Self::ValidationError(value)
    }
}

impl From<read::Error> for ParsingError {
fn from(value: read::Error) -> Self {
    match value {
        read::Error::IoError(e) => e.into(),
        read::Error::Overflow => Self::VarintOverflow,
    }
}
}

impl std::fmt::Display for ParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self, f)
    }
}

impl std::error::Error for ParsingError {}
