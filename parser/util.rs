use super::error::DecodingError;
use crate::structures::types::ResultType;
use leb128::read;

pub fn decode_resulttype<R: std::io::Read>(reader: &mut R) -> Result<ResultType, DecodingError> {
    let elem_count = read::unsigned(reader)? as u32;
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
