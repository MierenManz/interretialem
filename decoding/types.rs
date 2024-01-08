use super::error::DecodingError;

pub(crate) enum NumType {
    I32 = 0x7F,
    I64 = 0x7E,
    F32 = 0x7D,
    F64 = 0x7C
}
impl TryFrom<u8> for NumType {
    type Error = DecodingError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let res = match value {
            0x7F => Self::I32,
            0x7E => Self::I64,
            0x7D => Self::F32,
            0x7C => Self::F64,
            _ => return Err(DecodingError::UnknownType)
        };

        Ok(res)
    }
}
impl TryFrom<&[u8]> for NumType {
    type Error = DecodingError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        value[0].try_into()
    }
}

pub(crate) enum VecType {
    V128 = 0x7B
}
impl TryFrom<u8> for VecType {
    type Error = DecodingError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value == 0x7B { Ok(Self::V128) } else { Err(DecodingError::UnknownType) }
    }
}
impl TryFrom<&[u8]> for VecType {
    type Error = DecodingError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        value[0].try_into()
    }
}
pub(crate) enum RefType {
    Funcref = 0x70,
    Externref = 0x6F,
}
impl TryFrom<u8> for RefType {
    type Error = DecodingError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let res = match value {
            0x70 => Self::Funcref,
            0x6F => Self::Externref,
            _ => return Err(DecodingError::UnknownType)
        };

        Ok(res)
    }
}
impl TryFrom<&[u8]> for RefType {
    type Error = DecodingError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        value[0].try_into()
    }
}

pub(crate) enum ValType {
    Num(NumType),
    Vec(VecType),
    Ref(RefType),
}
impl TryFrom<u8> for ValType {
    type Error = DecodingError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if let Ok(v) = TryFrom::try_from(value) {
            return Ok(Self::Num(v));
        }

        if let Ok(v) = TryFrom::try_from(value) {
            return Ok(Self::Vec(v));
        }

        if let Ok(v) = TryFrom::try_from(value) {
            return Ok(Self::Ref(v));
        }

        Err(DecodingError::UnknownType)
    }
}
impl TryFrom<&[u8]> for ValType {
    type Error = DecodingError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        value[0].try_into()
    }
}

pub(crate) struct ResultType {
    inner: Vec<ValType>,
}
impl ResultType {
    pub(crate) fn new(value: Vec<ValType>) -> Self {
        Self { inner: value }
    }

    pub(crate) fn empty() -> Self {
        Self { inner: vec![] }
    }

    pub(crate) fn as_slice(&self) -> &[ValType] {
        &self.inner
    }
}

impl From<Vec<ValType>> for ResultType {
    fn from(value: Vec<ValType>) -> Self {
        Self { inner: value }
    }
}
impl From<ResultType> for Vec<ValType> {
    fn from(value: ResultType) -> Self {
        value.inner
    }
}

pub(crate) struct FuncType {
    params: ResultType,
    result: ResultType,
}
impl FuncType {
    pub(crate) fn new(params: ResultType, result: ResultType) -> Self {
        Self { params, result }
    }

    pub(crate) fn params(&self) -> &[ValType] {
        self.params.as_slice()
    }

    pub(crate) fn result(&self) -> &[ValType] {
        self.params.as_slice()
    }
}