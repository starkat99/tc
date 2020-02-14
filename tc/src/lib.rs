use thiserror::Error;

mod bc1;

#[derive(Error, Debug)]
pub enum DecoderError {}

pub type DecoderResult<T> = std::result::Result<T, DecoderError>;

pub struct DecodedBlock {}

pub struct Blocks {}

pub trait Decoder {
    fn decode<T: AsRef<[u8]>>(&self, bytes: T) -> DecoderResult<Vec<u8>>;

    fn decode_to<T: AsRef<[u8]>>(&self, bytes: T, buffer: &mut Vec<u8>) -> DecoderResult<()>;

    fn decode_at_pixel<T: AsRef<[u8]>>(
        &self,
        bytes: T,
        coords: (usize, usize),
    ) -> DecoderResult<DecodedBlock>;

    fn blocks<T: AsRef<[u8]>>(&self, bytes: T) -> Blocks;
}
