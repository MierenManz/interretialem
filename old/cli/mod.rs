use super::parser::DecodedModule;
const COOKIE: &'static str = "\0asm";
const VERSION: [u8; 4] = 1u32.to_le_bytes();
fn main() {
} 
//   DecodedModule::from_slice([]).unwrap()
// }