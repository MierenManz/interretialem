use std::cell::Ref;

use binrw::binread;
use binrw::BinRead;
use super::parsers::parse_varuint32;

#[binread]
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
#[br(little, repr = u8)]
pub(crate) enum NumType {
    I32 = 0x7F,
    I64 = 0x7E,
    F32 = 0x7D,
    F64 = 0x7C,
}

#[binread]
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
#[br(little, repr = u8)]
pub(crate) enum VecType {
    V128 = 0x7B,
}

#[binread]
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
#[br(little, repr = u8)]
pub(crate) enum RefType {
    Funcref = 0x70,
    Externref = 0x6F,
}

#[binread]
#[derive(Debug, Clone, Copy, PartialEq)]
#[br(little)]
pub(crate) enum ValType {
    Num(NumType),
    Vec(VecType),
    Ref(RefType)
}

impl From<NumType> for ValType {
    fn from(value: NumType) -> Self {
        Self::Num(value)
    }
}

impl From<VecType> for ValType {
    fn from(value: VecType) -> Self {
        Self::Vec(value)
    }
}
impl From<RefType> for ValType {
    fn from(value: RefType) -> Self {
        Self::Ref(value)
    }
}


#[binread]
pub(crate) struct ResultType {
    #[br(temp, parse_with = parse_varuint32)]
    count: u32,
    #[br(count = count as usize)]
    inner: Vec<ValType>,
}
impl ResultType {
    pub(crate) fn as_slice(&self) -> &[ValType] {
        &self.inner
    }
}

#[binread]
#[br(magic = 0x60u8)]
pub(crate) struct FuncType {
    #[br(temp)]
    t_params: ResultType,
    #[br(temp)]
    t_results: ResultType,

    #[br(calc = t_params.as_slice().len() as u32)]
    param_count: u32,
    #[br(calc = [t_params.as_slice(), t_results.as_slice()].concat().into_boxed_slice())]
    values: Box<[ValType]>,
}
impl FuncType {
    pub(crate) fn params(&self) -> &[ValType] {
        &self.values[..self.param_count as usize]   
    }

    pub(crate) fn results(&self) -> &[ValType] {
        &self.values[self.param_count as usize..]
    }
}

#[binread]
pub(crate) struct Limits {
    #[br(temp)]
    has_max: u8,

    #[br(parse_with = parse_varuint32)]
    min: u32,
    #[br(if(has_max == 1, None), parse_with = |x,y,z| Ok(Some(parse_varuint32(x,y,z)?)))]
    max: Option<u32>
}

#[binread]
pub(crate) struct TableType {
    limits: Limits,
    kind: RefType,
}

#[binread]
pub(crate) struct GlobalType {
    #[br(temp)]
    t: u8,
    #[br(calc = t == 1)]
    is_mut: bool,
    kind: ValType,
}

mod test {
    use binrw::BinReaderExt;
    use std::io::Cursor;
    use binrw::BinRead;
    use crate::decoded_module::types::{NumType, VecType, RefType, ValType};
    #[test]
    fn numtype() {
        let buff: [u8; 1]= [0x7F];
        let mut cursor = Cursor::new(buff);
        assert_eq!(NumType::read(&mut cursor).unwrap(), NumType::I32);
    }

    #[test]
    fn bad_numtype() {
        let buff: [u8; 1] = [0x10];
        let mut cursor = Cursor::new(buff);
        assert!(NumType::read(&mut cursor).is_err());
    }

    #[test]
    fn vectype() {
        let buff: [u8; 1]= [0x7B];
        let mut cursor = Cursor::new(buff);
        assert_eq!(VecType::read(&mut cursor).unwrap(), VecType::V128);
    }

    #[test]
    fn bad_vectype() {
        let buff: [u8; 1] = [0x10];
        let mut cursor = Cursor::new(buff);
        assert!(NumType::read(&mut cursor).is_err());
    }

    #[test]
    fn reftype() {
        let buff: [u8; 1]= [0x70];
        let mut cursor = Cursor::new(buff);
        assert_eq!(RefType::read(&mut cursor).unwrap(), RefType::Funcref);
    }

    #[test]
    fn bad_reftype() {
        let buff: [u8; 1] = [0x10];
        let mut cursor = Cursor::new(buff);
        assert!(RefType::read(&mut cursor).is_err());
    }

    #[test]
    fn valtype_num() {
        let buf: [u8; 1] = [0x7F];
        let mut cursor = Cursor::new(buf);
        assert_eq!(ValType::read(&mut cursor).unwrap(), ValType::Num(NumType::I32));
    }
    #[test]
    fn valtype_vec() {
        let buf: [u8; 1] = [0x7B];
        let mut cursor = Cursor::new(buf);
        assert_eq!(ValType::read(&mut cursor).unwrap(), ValType::Vec(VecType::V128));
    }
    #[test]
    fn valtype_ref() {
        let buf: [u8; 1] = [0x70];
        let mut cursor = Cursor::new(buf);
        assert_eq!(ValType::read(&mut cursor).unwrap(), ValType::Ref(RefType::Funcref));
    }
}