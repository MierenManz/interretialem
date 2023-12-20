use super::error::DecodingError;
use super::util::decode_resulttype;
use crate::structures::types::FuncType;
use crate::structures::DecodedModule;
use leb128::read;

impl DecodedModule {
    fn decode_type_section<R: std::io::Read>(
        &mut self,
        reader: &mut R,
    ) -> Result<(), DecodingError> {
        let elem_count = read::unsigned(reader)? as u32;

        for _ in 0..elem_count {
            let params = decode_resulttype(reader)?;
            let result = decode_resulttype(reader)?;
            self.type_section.push(FuncType { params, result });
        }

        Ok(())
    }

    pub fn from_reader<R: std::io::Read>(reader: &mut R) -> Result<Self, DecodingError> {
        let mut module: DecodedModule = Default::default();
        reader.read_exact(&mut module.magic_cookie)?;
        reader.read_exact(&mut module.version_bytes)?;

        let mut last_section = 0;
        for i in 0..11 {
            let mut section_byte_buff: [u8; 1] = [0; 1];
            reader.read_exact(&mut section_byte_buff)?;
            let section_id = section_byte_buff[0];

            if last_section >= section_id {
                return Err(DecodingError::DuplicateSection);
            }

            let slice_len = leb128::read::unsigned(reader)?;
            let mut section_buff = Vec::with_capacity(slice_len as usize);

            reader.read_exact(&mut section_buff)?;

            last_section = i;
        }

        Ok(module)
    }
}
