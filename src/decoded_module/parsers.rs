use leb128::read;
use binrw::{BinResult, BinRead};

use super::{error::DecodingError, types::{ValType, NumType}};

#[binrw::parser(reader)]
pub(crate) fn parse_varint32() -> BinResult<i32> {
    read::signed(reader)
        .map_err(|e| DecodingError::from(e))?
        .try_into()
        .or(Err(DecodingError::VarintOverflow.into()))
}

#[binrw::parser(reader)]
pub(crate) fn parse_varuint32() -> BinResult<u32> {
    read::unsigned(reader)
    .map_err(|e| DecodingError::from(e))?
    .try_into()
    .or(Err(DecodingError::VarintOverflow.into()))
}

// #[binrw::parser(reader)]
// pub(crate) fn parse_valtype() -> BinResult<ValType> {
//     let mut buf = [0; 1];
//     reader.read_exact(&mut buf)?;
// }

pub(crate) fn parse_varint64() {}