use super::parsers::parse_varuint32;
use binrw::binread;

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
    Ref(RefType),
}

#[binread]
#[derive(Debug, PartialEq)]
#[br(little)]
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

    fn len(&self) -> u32 {
        self.inner.len() as u32
    }
}

#[binread]
#[derive(Debug, PartialEq)]
#[br(magic = 0x60u8, little)]
pub(crate) struct FuncType {
    #[br(temp)]
    t_params: ResultType,
    #[br(temp)]
    t_results: ResultType,

    #[br(calc = t_params.len() as u32)]
    param_count: u32,
    #[br(calc = [t_params.as_slice(), t_results.as_slice()].concat().into_boxed_slice())]
    inner: Box<[ValType]>,
}
impl FuncType {
    pub(crate) fn params(&self) -> &[ValType] {
        &self.inner[..self.param_count as usize]
    }

    pub(crate) fn results(&self) -> &[ValType] {
        &self.inner[self.param_count as usize..]
    }

    pub(crate) const fn param_count(&self) -> u32 {
        self.param_count
    }
}

#[binread]
#[derive(Debug, PartialEq)]
#[br(little)]
pub(crate) struct Limits {
    #[br(temp)]
    t: u8,

    #[br(parse_with = parse_varuint32)]
    min: u32,
    #[br(if(t == 1), parse_with = |x,y,z| Ok(Some(parse_varuint32(x,y,z)?)))]
    max: Option<u32>,
}

#[binread]
#[br(little)]
pub(crate) struct TableType {
    limits: Limits,
    kind: RefType,
}

#[binread]
#[br(little)]
pub(crate) struct GlobalType {
    #[br(map = |x: u8| x == 1)]
    is_mut: bool,
    kind: ValType,
}

#[allow(unused_imports)]
mod test {
    use crate::decoded_module::types::{FuncType, NumType, RefType, ResultType, ValType, VecType};
    use binrw::BinRead;
    use binrw::BinReaderExt;
    use std::io::Cursor;

    use super::Limits;
    #[test]
    fn numtype() {
        let buff: [u8; 1] = [0x7F];
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
        let buff: [u8; 1] = [0x7B];
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
        let buff: [u8; 1] = [0x70];
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
        assert_eq!(
            ValType::read(&mut cursor).unwrap(),
            ValType::Num(NumType::I32)
        );
    }
    #[test]
    fn valtype_vec() {
        let buf: [u8; 1] = [0x7B];
        let mut cursor = Cursor::new(buf);
        assert_eq!(
            ValType::read(&mut cursor).unwrap(),
            ValType::Vec(VecType::V128)
        );
    }
    #[test]
    fn valtype_ref() {
        let buf: [u8; 1] = [0x70];
        let mut cursor = Cursor::new(buf);
        assert_eq!(
            ValType::read(&mut cursor).unwrap(),
            ValType::Ref(RefType::Funcref)
        );
    }

    #[test]
    fn result_type() {
        let buff: [u8; 4] = [0x03, 0x7F, 0x7F, 0x7F];
        let mut cursor = Cursor::new(buff);
        let result_type = ResultType {
            inner: vec![
                ValType::Num(NumType::I32),
                ValType::Num(NumType::I32),
                ValType::Num(NumType::I32),
            ],
        };
        assert_eq!(ResultType::read(&mut cursor).unwrap(), result_type);
    }

    #[test]
    fn empty_resulttype() {
        let buff: [u8; 1] = [0];
        let mut cursor = Cursor::new(buff);
        let result_type = ResultType { inner: vec![] };
        assert_eq!(ResultType::read(&mut cursor).unwrap(), result_type);
    }

    #[test]
    fn func_type() {
        let buff: [u8; 4] = [0x60, 0x01, 0x7F, 0x0];
        let mut cursor = Cursor::new(buff);

        let fn_type = FuncType {
            param_count: 1,
            inner: vec![ValType::Num(NumType::I32)].into(),
        };
        assert_eq!(FuncType::read(&mut cursor).unwrap(), fn_type);
    }

    #[test]
    fn limits_only_min() {
        let buf: [u8; 2] = [0x00, 0x01];
        let mut cursor = Cursor::new(buf);

        let limits = Limits { min: 1, max: None };

        assert_eq!(Limits::read(&mut cursor).unwrap(), limits);
    }

    #[test]
    fn limits_min_max() {
        let buf: [u8; 3] = [0x01, 0x01, 0x02];
        let mut cursor = Cursor::new(buf);

        let limits = Limits {
            min: 1,
            max: Some(2),
        };

        assert_eq!(Limits::read(&mut cursor).unwrap(), limits);
    }
}
