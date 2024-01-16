use binrw::{binread, BinRead};

use super::indices::TypeIndex;
use super::parsers::parse_varuint32;
use super::types::*;

#[binread]
#[br(little)]
#[repr(u8)]
pub(crate) enum ImportDescriptor {
    Func(TypeIndex) = 0x00,
    Table(TableType) = 0x01,
    Mem(Limits) = 0x02,
    Global(GlobalType) = 0x03,
}

#[binread]
pub(crate) struct Import {
    #[br(temp, parse_with = parse_varuint32)]
    mod_len: u32,
    #[br(count = mod_len, try_map = |x: Vec<u8>| String::from_utf8(x))]
    module: String,
    #[br(temp, parse_with = parse_varuint32)]
    name_len: u32,
    #[br(count = name_len, try_map = |x: Vec<u8>| String::from_utf8(x))]
    name: String,
    descriptor: ImportDescriptor,
}

#[binread]
pub(crate) struct DecodedModule {
    #[br(temp, parse_with = parse_varuint32)]
    type_len: u32,
    #[br(count = type_len)]
    type_section: Vec<FuncType>,

    #[br(temp, parse_with = parse_varuint32)]
    import_len: u32,
    #[br(count = import_len)]
    import_section: Vec<Import>,

    #[br(temp, parse_with = parse_varuint32)]
    func_len: u32,
    #[br(count = func_len)]
    func_section: Vec<u32>,
}

impl DecodedModule {
    pub(crate) fn build_module(self) -> Module {}
}
