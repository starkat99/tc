#[macro_use]
extern crate bitflags;
extern crate byteorder;

mod raw {
    use byteorder::{LE, ByteOrder};

    bitflags! {
        pub struct HeaderFlags: u32 {
            const Caps = 0x1;
            const Height = 0x2;
            const Width = 0x4;
            const Pitch = 0x8;
            const PixelFormat = 0x1000;
            const MipMapCount = 0x20000;
            const LinearSize = 0x80000;
            const Depth = 0x800000;
            const Texture = Caps.bits | Height.bits | Width.bits | PixelFormat.bits;
        }
    }

    struct Header<'a> {
        buffer: &'a [u8]
    }

    impl<'a> Header<'a> {
        pub fn new(buf: &'a [u8]) -> Header<'a> {
            Header { buffer: buf }
        }

        pub fn has_magic_number(&self) -> bool {
            &self.buffer[..8] == b"DDS \0\0\0\x7c" // Magic number plus header size
        }

        pub fn size(&self) -> (u32, u32) {
            (LE::read_u32(&self.buffer[8..]), LE::read_u32(&self.buffer[12..]))
        }

        fn raw_flags(&self) -> HeaderFlags {
            HeaderFlags::from_bits_truncate(LE::read_u32(&self.buffer[16..]))
        }

        fn raw_pitch(&self) -> Option<u32> {
            if self.raw_flags().contains(Pitch) && !self.raw_flags().contains(LinearSize) {
                Some(LE::read_u32(&self.buffer[20..]))
            }
            else {
                None
            }
        }
    }
}

pub struct Dds {
    size: (u32, u32),
    depth: Option<u32>,
    dimension: Dimension,
    srgb: bool,
    alpha: Option<AlphaMode>,
    format: Option<DataFormat>,
}

pub enum Dimension {
    Texture1D,
    Texture2D,
    Texture3D,
    Cube,
}

pub enum CompressedFormat {
    /// Block Compression 1, a.k.a. DXT1. 4 bits per pixel.
    ///
    /// Compression resolution: 5 bits red, 6 bits green, 5 bits blue, 0 or 1-bit alpha
    /// Legacy compression for simple RGB and no alpha.
    BC1(BlockCompressionType),
    /// Block Compression 2, a.k.a. DXT2 & DXT3. 8 bits per pixel.
    ///
    /// Compression resolution: 5 bits red, 6 bits green, 5 bits blue, 4 bits alpha
    /// Legacy compression for RGBA values not using alpha blending.
    BC2(BlockCompressionType),
    /// Block compression 3, a.k.a. DXT4 & DXT5. 8 bits per pixel.
    ///
    /// Compression resolution: 5 bits red, 6 bits green, 5 bits blue, 8 bits alpha
    /// Legacy compression for RGBA values with smoother alpha blending.
    BC3(BlockCompressionType),
    /// YCrCb color, 16 bits per pixel, 8 bits per sample.
    ///
    /// YUV 4:2:2 sampling: First pixel has U and Y samples, second has V and Y, etc., for a total
    /// of 2 pixels in one block.
    UYVY,
    /// 16-bit packed RGB, 8 bits per channel, analogous to `UYVY`.
    ///
    /// First pixel has R & G channels, second has B & G channels, etc. for a total of 2 pixels in
    /// one block.
    R8G8B8G8,
    /// YCrCb color, 16 bits per pixel, 8 bits per sample.
    ///
    /// YUV 4:2:2 sampling: First pixel has Y and U samples, second has Y and V, etc., for a total
    /// of 2 pixels in one block. Reverse component ordering of `UYVY`.
    YUY2,
    /// 16-bit packed RGB, 8 bits per channel, analogous to `UYVY`.
    ///
    /// First pixel has G & R channels, second has G & B channels, etc. for a total of 2 pixels in
    /// one block. Reverse component ordering of `R8G8B8G8`.
    G8R8G8B8,
    // Multi2Argb8: Uncompressed MultiElement texture, whatever that is
}

pub enum BlockCompressionType {
    Unspecified,
    Unsigned,
    Signed,
}

pub enum AlphaMode {
    /// Alpha channel should be treated as fully opaque.
    Opaque,
    /// Treat alpha channel data as alpha transparency, i.e. 'straight' alpha.
    Transparent,
    /// Alpha channel data is premultiplied with other channels.
    Premultiplied,
    /// Alpha channel data is used as a 4th component channel only, and is not to be used for
    /// transparency.
    Data,
}

pub enum ComponentType {
    Unspecified(usize),
    UnsignedInt(usize),
    SignedInt(usize),
    Float(usize),
    UnsignedNormalized(usize),
    SignedNormalized(usize),
    FixedPoint(usize, usize),
}

pub enum ChannelSemantic {
    Unspecified(ComponentType),
    Red(ComponentType),
    Green(ComponentType),
    Blue(ComponentType),
    Alpha(ComponentType),
    Luminance(ComponentType),
    Chrominance(ComponentType),
    Depth(ComponentType),
    Stencil(ComponentType),
    PaletteIndex(ComponentType),
    Exponent(ComponentType),
}

pub enum UncompressedFormat {
    OneComponent(ChannelSemantic),
    TwoComponents(ChannelSemantic, ChannelSemantic),
    ThreeComponents(ChannelSemantic, ChannelSemantic, ChannelSemantic),
    FourComponents(ChannelSemantic, ChannelSemantic, ChannelSemantic, ChannelSemantic),
}

pub enum DataFormat {
    Uncompressed(UncompressedFormat),
    Compressed(CompressedFormat),
    Unknown(u32),
}