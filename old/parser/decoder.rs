use super::module::DecodedModule;
use std::io::Read;
use super::error::ParsingError;
use super::error::ValidationError;
use leb128::read;

const WASM_COOKIE: &'static str = "\0asm";
const WASM_VERSION: [u8; 4] = 1u32.to_le_bytes();

impl Decoder {
    pub fn new() -> Self {
        Self
    }

    pub fn decode_reader<R: Read>(&mut self, reader: &mut R, module: &mut DecodedModule) -> Result<(), ParsingError> {
        let mut cookie: [u8; 8] = [0; 8];
        reader.read_exact(&mut cookie)?;
        if &cookie[0..4] != WASM_COOKIE.as_bytes() { 
            return Err(ValidationError::BadCookie.into())
        }

        if &cookie[5..8] != WASM_VERSION {
            return Err(ParsingError::UnsupportedWasmVersion);
        }

        let mut last_section = 0;
        for i in 0..11 {
            let mut section_byte_buff: [u8; 1] = [0; 1];
            reader.read_exact(&mut section_byte_buff)?;
            let section_byte = section_byte_buff[0];

            if last_section >= section_byte {
                return Err(ParsingError::DuplicateSection);
            }
            last_section = i;
            let slice_len = read::unsigned(reader)? as u32;
            
        }

        Ok(())
    }
}

mod test {
    #[test]
    fn x() {
        for i in 1..13 {
            println!("{i}");
        }
    }
}