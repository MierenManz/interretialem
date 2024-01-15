use crate::error::DecodingError;
use std::num::NonZeroU32;

#[derive(Debug, Clone, Copy)]
pub(crate) enum NumType {
    I32 = 0x7F,
    I64 = 0x7E,
    F32 = 0x7D,
    F64 = 0x7C,
}

impl TryFrom<u8> for NumType {
    type Error = DecodingError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let result = match value {
            0x7F => Self::I32,
            0x7E => Self::I64,
            0x7D => Self::F32,
            0x7C => Self::F64,
            _ => return Err(DecodingError::UnknownValueType),
        };

        Ok(result)
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum ValType {
    Num(NumType),
}

impl TryFrom<u8> for ValType {
    type Error = DecodingError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(Self::Num(value.try_into()?))
    }
}

#[derive(Clone)]
pub(crate) struct ResultType(Vec<ValType>);
impl ResultType {
    pub(crate) fn arg_count(&self) -> u32 {
        self.0.len() as u32
    }
}
impl From<Vec<ValType>> for ResultType {
    fn from(value: Vec<ValType>) -> Self {
        Self(value)
    }
}
pub(crate) struct FuncType(ResultType, ResultType);
impl From<(ResultType, ResultType)> for FuncType {
    fn from(value: (ResultType, ResultType)) -> Self {
        Self(value.0, value.1)
    }
}

pub(crate) struct Limits(NonZeroU32, Option<NonZeroU32>);
impl Limits {
    pub(crate) fn set_max(&mut self, max: NonZeroU32) {
        self.1 = Some(max);
    }

    pub(crate) fn min_bigger(&self) -> bool {
        self.1.is_some_and(|x| self.0 > x)
    }
}
impl From<NonZeroU32> for Limits {
    fn from(value: NonZeroU32) -> Self {
        Self(value, None)
    }
}

impl TryFrom<u32> for Limits {
    type Error = DecodingError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match NonZeroU32::try_from(value) {
            Ok(v) => Ok(v.into()),
            Err(_) => Err(DecodingError::ExpectedNonZero),
        }
    }
}

