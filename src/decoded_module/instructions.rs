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
impl BrTable {
    pub(crate) fn branch(&self, index: u32) -> Option<LabelIndex> {
        if index < (self.branches.len() - 1) as u32 {
            Some(self.branches[index as usize])
        } else {
            None
        }
    }

    pub(crate) fn default(&self) -> LabelIndex {
        self.branch((self.branches.len() - 1) as u32).unwrap()
    }
}

#[binread]
pub(crate) struct MemArg {
    #[br(parse_with = parse_varuint32)]
    align: u32,
    #[br(parse_with = parse_varuint32)]
    offset: u32,
}

// #[binread]
// #[br(parse_with = )]
// pub(crate) enum Instruction {
//     OneByteInstr(),
//     BulkMemInstr(),
//     SIMDInstr(),
// }
