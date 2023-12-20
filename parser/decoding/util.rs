use std::num::NonZeroU32;

use crate::structures::ValType;
use crate::structures::ResultType;
use leb128::read;
use super::error::DecodingError;

#[inline]
pub(crate) fn decode_varint<R: std::io::Read>(reader: &mut R) -> Result<u32, DecodingError> {
    let n = read::unsigned(reader)?;
    n.try_into().or(Err(DecodingError::VarintOverflow))
}

pub(crate) fn decode_resulttype<R: std::io::Read>(reader: &mut R) -> Result<ResultType, DecodingError> {
    let elem_count = decode_varint(reader)?;
    let mut vec = Vec::with_capacity(elem_count as usize);

    for _ in 0..elem_count {
        let mut byte_buff: [u8; 1] = [0; 1];
        reader.read_exact(&mut byte_buff)?;
        let val_type = byte_buff[0]
            .try_into()
            .or(Err(DecodingError::UnknownValType))?;
        vec.push(val_type);
    }

    Ok(vec)
}

pub(crate) fn decode_locals<R: std::io::Read>(reader: &mut R) -> Result<Vec<(NonZeroU32, ValType)>, DecodingError> {
    let elem_count = decode_varint(reader)?;
    let mut vec = Vec::with_capacity(elem_count as usize);


    Ok(vec)
}