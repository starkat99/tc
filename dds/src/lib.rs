#![allow(dead_code, unknown_lints)]

#[macro_use]
extern crate bitflags;
extern crate bytes;
#[macro_use]
extern crate error_chain;
extern crate num_traits;
#[macro_use]
extern crate enum_primitive_derive;

pub mod raw;

error_chain!{}

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
