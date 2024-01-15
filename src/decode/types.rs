use super::{Decode, util::decode_varint};
use crate::{decoded_ir::types::*, error::DecodingError};

impl Decode for NumType {
    fn decode<R: std::io::Read>(reader: &mut R) -> Result<Self, crate::error::DecodingError> {
        let mut b = [0; 1];
        reader.read_exact(&mut b)?;

        b[0].try_into()
    }
}

impl Decode for ValType {
    fn decode<R: std::io::Read>(reader: &mut R) -> Result<Self, crate::error::DecodingError> {
        let mut b = [0; 1];
        reader.read_exact(&mut b)?;

        b[0].try_into()
    }
}

impl Decode for ResultType {
    fn decode<R: std::io::Read>(reader: &mut R) -> Result<Self, crate::error::DecodingError> {
        Vec::decode(reader).map(|x| x.into())
    }
}

impl Decode for FuncType {
    fn decode<R: std::io::Read>(reader: &mut R) -> Result<Self, crate::error::DecodingError> {
        let mut b = [0; 1];
        reader.read_exact(&mut b)?;
        if b[0] != 0x60 {
            return Err(DecodingError::InvalidFunctionSignature);
        }
        
        Ok((Decode::decode(reader)?, Decode::decode(reader)?).into())
    }
}

impl Decode for Limits {
    fn decode<R: std::io::Read>(reader: &mut R) -> Result<Self, crate::error::DecodingError> {
        let mut b = [0; 1];
        reader.read_exact(&mut b)?;

        let min = decode_varint(reader)?;
        
        let mut limits = Limits::try_from(min)?;
        if b[0] == 1 {
            limits.set_max(
                decode_varint(reader)?
                    .try_into()
                    .or(Err(DecodingError::ExpectedNonZero))?
            );
        }

        Ok(limits)
    }
}