use binrw::binread;
use super::parsers::{
    parse_string,
    parse_varuint32,
};
use super::indices::*;
use super::types::{
    Limits,
    TableType,
    GlobalType,
    FuncType
};

type Expr = ();

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
    #[br(parse_with = parse_string)]
    module: String,
    #[br(parse_with = parse_string)]
    name: String,
    descriptor: ImportDescriptor,
}

#[binread]
#[br(little)]
#[repr(u8)]
pub(crate) enum ExportDescriptor {
    Func(FuncIndex) = 0x00,
    Table(TableIndex) = 0x01,
    Mem(MemIndex) = 0x02,
    Global(GlobalIndex) = 0x03,
}

#[binread]
pub(crate) struct Export {
    #[br(parse_with = parse_string)]
    name: String,
    descriptor: ExportDescriptor,
}

#[binread]
#[br(magic = 1u8)]
pub(crate) struct TypeSection {
    #[br(temp, parse_with = parse_varuint32)]
    size: u32,
    #[br(temp, parse_with = parse_varuint32)]
    len: u32,
    #[br(count = len)]
    inner: Vec<FuncType>,
}

#[binread]
#[br(magic = 2u8)]
pub(crate) struct ImportSection {
    #[br(temp, parse_with = parse_varuint32)]
    size: u32,
    #[br(temp, parse_with = parse_varuint32)]
    len: u32,
    #[br(count = len)]
    inner: Vec<FuncType>,
}

#[binread]
#[br(magic = 3u8)]
pub(crate) struct FuncSection {
    #[br(temp, parse_with = parse_varuint32)]
    size: u32,
    #[br(temp, parse_with = parse_varuint32)]
    len: u32,
    #[br(count = len)]
    inner: Vec<TypeIndex>,
}

#[binread]
#[br(magic = 4u8)]
pub(crate) struct TableSection {
    #[br(temp, parse_with = parse_varuint32)]
    size: u32,
    #[br(temp, parse_with = parse_varuint32)]
    len: u32,
    #[br(count = len)]
    inner: Vec<TableType>,
}

#[binread]
#[br(magic = 5u8)]
pub(crate) struct MemorySection {
    #[br(temp, parse_with = parse_varuint32)]
    size: u32,
    #[br(temp, parse_with = parse_varuint32)]
    len: u32,
    #[br(count = len)]
    inner: Vec<Limits>,
}

#[binread]
#[br(magic = 6u8)]
pub(crate) struct GlobalSection {
    #[br(temp, parse_with = parse_varuint32)]
    size: u32,
    #[br(temp, parse_with = parse_varuint32)]
    len: u32,
    #[br(count = len)]
    inner: Vec<(GlobalType, Expr)>,
}

#[binread]
#[br(magic = 7u8)]
pub(crate) struct ExportSection {
    #[br(temp, parse_with = parse_varuint32)]
    size: u32,
    #[br(temp, parse_with = parse_varuint32)]
    len: u32,
    #[br(count = len)]
    inner: Vec<Export>,
}

#[binread]
#[br(magic = 8u8)]
pub(crate) struct StartSection {
    #[br(temp, parse_with = parse_varuint32)]
    size: u32,
    index: FuncIndex,
}