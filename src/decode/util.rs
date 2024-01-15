use leb128::read;
use std::io::Read;
use crate::error::DecodingError;
use super::Decode;

pub(crate) fn decode_varint<R: Read>(reader: &mut R) -> Result<u32, DecodingError> {
    read::unsigned(reader)?
        .try_into()
        .or(Err(DecodingError::VarintOverflow))
}

impl<T> Decode for Vec<T>
where T: Decode {
    fn decode<R: std::io::Read>(reader: &mut R) -> Result<Self, DecodingError> {
        let cap = decode_varint(reader)?;
        let mut v = Vec::with_capacity(cap as usize);

        for _ in 0..cap {
            v.push(T::decode(reader)?);
        }

        Ok(v)
    }
}

impl<T> Decode for Box<[T]>
where T: Decode {
    fn decode<R: std::io::Read>(reader: &mut R) -> Result<Self, DecodingError> {
        Vec::decode(reader).map(|x| x.into())
    }
}