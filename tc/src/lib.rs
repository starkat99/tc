#[macro_use]
extern crate error_chain;
extern crate byteorder;

mod bc1;

mod errors {
    error_chain! {
        errors {

        }
    }
}

pub use errors::{Error, ErrorKind, Result, ResultExt};

pub struct DecodedBlock {}

pub struct Blocks {}

pub trait Decoder {
    fn decode<T: AsRef<[u8]>>(&self, bytes: T) -> Result<Vec<u8>>;

    fn decode_to<T: AsRef<[u8]>>(&self, bytes: T, buffer: &mut Vec<u8>) -> Result<()>;

    fn decode_at_pixel<T: AsRef<[u8]>>(
        &self,
        bytes: T,
        coords: (usize, usize),
    ) -> Result<DecodedBlock>;

    fn blocks<T: AsRef<[u8]>>(&self, bytes: T) -> Blocks;
}
