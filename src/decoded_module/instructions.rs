use super::indices::*;
use super::parsers::parse_varuint32;
use super::types::{RefType, ValType};
use binrw::binread;

#[binread]
#[repr(u8)]
pub(crate) enum BlockType {
    #[br(magic = 0x40u8)]
    Void,
    Value(ValType),
    Type(TypeIndex),
}

#[binread]
pub(crate) struct BrTable {
    #[br(temp, parse_with = parse_varuint32)]
    v_len: u32,
    #[br(count = v_len as usize, map = |x: Vec<LabelIndex>| x.into() )]
    branches: Box<[LabelIndex]>,
}

#[binread]
pub(crate) struct MemArg {
    #[br(parse_with = parse_varuint32)]
    align: u32,
    #[br(parse_with = parse_varuint32)]
    offset: u32,
}