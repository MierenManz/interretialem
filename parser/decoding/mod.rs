mod instruction;
mod error;
mod util;

use util::*;
use error::DecodingError;
use crate::structures::*;
use std::io::Read;

impl DecodedModule {
    fn decode_type_section<R: Read>(&mut self, reader: &mut R) -> Result<(), DecodingError> {
        let elem_count = decode_varint(reader)?;

        for _ in 0..elem_count {
            let params = decode_resulttype(reader)?;
            let result = decode_resulttype(reader)?;
            self.type_section.push(FuncType { params, result });
        }

        Ok(())
    }

    fn decode_import_section<R: Read>(&mut self, reader: &mut R) -> Result<(), DecodingError> {
        unimplemented!();
    }

    fn decode_fn_section<R: Read>(&mut self, reader: &mut R) -> Result<(), DecodingError> {
        let elem_count = decode_varint(reader)?;

        for _ in 0..elem_count {
            self.fn_section.push(decode_varint(reader)?);
        }

        Ok(())
    }

    fn decode_table_section<R: Read>(&mut self, reader: &mut R) -> Result<(), DecodingError> {
        unimplemented!();
    }

    fn decode_memory_section<R: Read>(&mut self, reader: &mut R) -> Result<(), DecodingError> {
        unimplemented!();
    }

    fn decode_global_section<R: Read>(&mut self, reader: &mut R) -> Result<(), DecodingError> {
        unimplemented!();
    }

    fn decode_export_section<R: Read>(&mut self, reader: &mut R) -> Result<(), DecodingError> {
        unimplemented!();
    }

    fn decode_start_section<R: Read>(&mut self, reader: &mut R) -> Result<(), DecodingError> {
        self.start_section = Some(decode_varint(reader)?);
        Ok(())
    }

    fn decode_element_section<R: Read>(&mut self, reader: &mut R) -> Result<(), DecodingError> {
        unimplemented!();
    }

    fn decode_code_section<R: Read>(&mut self, reader: &mut R) -> Result<(), DecodingError> {
        let elem_count = decode_varint(reader)?;
        for _ in 0..elem_count {
            // read `size`
            let size = decode_varint(reader)?;

            let function = Function {
                locals: decode_locals(reader)?,
            };

            let code_body = CodeBody {
                size,
                function
            };

            self.code_section.push(code_body);
        }

        Ok(())
    }

    fn decode_data_section<R: Read>(&mut self, reader: &mut R) -> Result<(), DecodingError> {
        unimplemented!();
    }

    fn decode_datacount_section<R: Read>(&mut self, reader: &mut R) -> Result<(), DecodingError> {
        unimplemented!();
    }

    pub fn from_slice(mut slice: &[u8]) -> Result<Self, DecodingError> {
        DecodedModule::from_reader(&mut slice)
    }

    pub fn from_reader<R: Read>(reader: &mut R) -> Result<Self, DecodingError> {
        let mut module: DecodedModule = Default::default();
        reader.read_exact(&mut module.magic_cookie)?;
        reader.read_exact(&mut module.version_bytes)?;

        let mut last_section = 0;
        for i in 0..13 {
            let mut section_byte_buff: [u8; 1] = [0; 1];
            reader.read_exact(&mut section_byte_buff)?;
            let section_id = section_byte_buff[0];

            if last_section >= section_id {
                return Err(DecodingError::DuplicateSection);
            }

            let slice_len = leb128::read::unsigned(reader)?;
            let mut section_buff = Vec::with_capacity(slice_len as usize);
            reader.read_exact(&mut section_buff)?;

            match section_id {
                1 => module.decode_type_section(&mut section_buff.as_slice())?,
                2 => module.decode_import_section(&mut section_buff.as_slice())?,
                3 => module.decode_fn_section(&mut section_buff.as_slice())?,
                4 => module.decode_table_section(&mut section_buff.as_slice())?,
                5 => module.decode_memory_section(&mut section_buff.as_slice())?,
                6 => module.decode_global_section(&mut section_buff.as_slice())?,
                7 => module.decode_export_section(&mut section_buff.as_slice())?,
                8 => module.decode_start_section(&mut section_buff.as_slice())?,
                9 => module.decode_element_section(&mut section_buff.as_slice())?,
                10 => module.decode_code_section(&mut section_buff.as_slice())?,
                11 => module.decode_data_section(&mut section_buff.as_slice())?,
                12 => module.decode_datacount_section(&mut section_buff.as_slice())?,
                _ => return Err(DecodingError::UnknownSection),
            }

            last_section = i;
        }

        Ok(module)
    }
}
