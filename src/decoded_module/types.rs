use binrw::binread;
use super::values::parse_varuint32;

#[binread]
#[derive(Clone, Copy)]
#[repr(u8)]
#[br(repr = u8)]
pub(crate) enum NumType {
    I32 = 0x7F,
    I64 = 0x7E,
    F32 = 0x7D,
    F64 = 0x7C,
}

#[binread]
#[derive(Clone, Copy)]
#[repr(u8)]
#[br(repr = u8)]
pub(crate) enum VecType {
    V128 = 0x7B,
}

#[binread]
#[derive(Clone, Copy)]
#[repr(u8)]
#[br(repr = u8)]
pub(crate) enum RefType {
    Funcref = 0x70,
    Externref = 0x6F,
}

#[binread]
#[derive(Clone, Copy)]
pub(crate) enum ValType {
    Num(NumType),
    Vec(VecType),
    Ref(RefType)
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