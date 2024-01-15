use crate::decoded_ir::indices::*;
use super::Decode;
use super::util::decode_varint;
use crate::error::DecodingError;

macro_rules! impl_decode {
    ($name: ty) => {
        impl Decode for $name {
            fn decode<R: std::io::Read>(reader: &mut R) -> Result<$name, DecodingError> {
                decode_varint(reader).map(|x| x.into())
            }
        }
    };
}

impl_decode!(TypeIndex);
impl_decode!(FuncIndex);
impl_decode!(TableIndex);
impl_decode!(MemoryIndex);
impl_decode!(GlobalIndex);
impl_decode!(ElemIndex);
impl_decode!(DataIndex);
impl_decode!(LocalIndex);
impl_decode!(LabelIndex);
