#[repr(u8)]
pub enum NumType {
    I32 = 0x7F,
    I64 = 0x7E,
    F32 = 0x7D,
    F64 = 0x7C,
}

impl TryFrom<u8> for NumType {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let v = match value {
            0x7F => Self::I32,
            0x7E => Self::I64,
            0x7D => Self::F32,
            0x7C => Self::I64,
            _ => return Err(()),
        };

        Ok(v)
    }
}

#[repr(u8)]
pub enum VecType {
    V128 = 0x7B,
}

impl TryFrom<u8> for VecType {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if Self::V128 as u8 == value {
            return Ok(Self::V128);
        }

        Err(())
    }
}

#[repr(u8)]
pub enum RefType {
    FuncRef = 0x70,
    ExternRef = 0x6F,
}

impl TryFrom<u8> for RefType {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let v = match value {
            0x70 => Self::FuncRef,
            0x6F => Self::ExternRef,
            _ => return Err(()),
        };

        Ok(v)
    }
}

pub enum ValType {
    Num(NumType),
    Vec(VecType),
    Ref(RefType),
}

impl TryFrom<u8> for ValType {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if let Ok(v) = NumType::try_from(value) {
            return Ok(Self::Num(v));
        }

        if let Ok(v) = VecType::try_from(value) {
            return Ok(Self::Vec(v));
        }

        if let Ok(v) = RefType::try_from(value) {
            return Ok(Self::Ref(v));
        }

        Err(())
    }
}

pub type ResultType = Vec<ValType>;

pub struct FuncType {
    pub params: ResultType,
    pub result: ResultType,
}

pub struct Limits {
    pub min: u32,
    pub max: Option<u32>,
}

pub struct MemoryType {
    pub min: u16,
    pub max: Option<u16>,
}

pub struct TableType {
    pub kind: RefType,
    pub limits: Limits,
}

pub struct GlobalType {
    pub kind: ValType,
    pub is_mut: bool,
}

pub enum BlockType {
    Void,
    Value(ValType),
    TypeSignature(u32),
}
