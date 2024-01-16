use super::error::DecodingError;
use binrw::BinResult;
use leb128::read;

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

pub(crate) fn parse_varint64() {}
