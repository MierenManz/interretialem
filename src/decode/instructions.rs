use crate::decoded_ir::{instructions::Block, Instruction};

use super::Decode;

impl Decode for Block {
    fn decode<R: std::io::Read>(reader: &mut R) -> Result<Self, crate::error::DecodingError> {
        let mut buf = [0 ;1];
        reader.read_exact(&mut buf)?;
        if buf[0] == 0x40 {
            return Ok(Self::Void);
        }

        if let Ok(v) = buf[0].try_into() {
            return Ok(Self::Value(v));
        }
        
        leb128::read::signed(reader)?.try_into()
        .map(|x: u32| Self::Type(x.into()))
        .or(Err(crate::error::DecodingError::VarintOverflow))
    }
}

impl Decode for Instruction {
    fn decode<R: std::io::Read>(reader: &mut R) -> Result<Self, crate::error::DecodingError> {
        let mut byte = [0; 1];
        reader.read_exact(&mut byte)?;

        let res = match byte[0] {
            0x00 => Self::Unreachable,
            0x01 => Self::Nop,
            0x02 => Self::Block(Decode::decode(reader)?),
            0x03 => Self::Loop(Decode::decode(reader)?),
            0x04 => Self::If(Decode::decode(reader)?),
            0x05 => Self::Else,
            0x0B => Self::End,
            0x0C => Self::Br(Decode::decode(reader)?),
            0x0D => Self::BrIf(Decode::decode(reader)?),
            0x0E => {
                let mut v = Vec::decode(reader)?;
                v.push(Decode::decode(reader)?);
                Self::BrTable(v.into())
            },
            0x0F => Self::Return,
            0x10 => Self::Call(Decode::decode(reader)?),
            0x11 => Self::CallIndirect(Decode::decode(reader)?, Decode::decode(reader)?),
            0xD0 => Self::IsNull
        };

        Ok(res)
    }
}