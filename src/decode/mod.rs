mod indices;
mod util;
mod types;
mod instructions;
use crate::error::DecodingError;

trait Decode
where Self: Sized {
    fn decode<R: std::io::Read>(reader: &mut R) -> Result<Self, DecodingError>;
}