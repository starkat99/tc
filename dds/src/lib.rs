#![allow(dead_code, unknown_lints)]

#[macro_use]
extern crate bitflags;
extern crate bytes;
#[macro_use]
extern crate error_chain;
extern crate num_traits;
#[macro_use]
extern crate enum_primitive_derive;

mod header;
pub mod formats;

pub enum TextureDimension {
    Texture1D,
    Texture2D,
    Texture3D,
    TextureCube,
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
    BC4(SignedCompressionType),
    BC5(SignedCompressionType),
    BC6H(BC6HCompressionType),
    BC7(BlockCompressionType),
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
    AYUV,
    Y410,
    Y416,
    NV12,
    P010,
    P016,
    Opaque420,
    Y210,
    Y216,
    NV11,
    AI44,
    IA44,
    P208,
    V208,
    V408,
}

pub enum BlockCompressionType {
    Typeless,
    UnsignedNormalized,
    UnsignedNormalizedSrgb,
}

pub enum SignedCompressionType {
    Typeless,
    UnsignedNormalized,
    SignedNormalized,
}

pub enum BC6HCompressionType {
    Typeless,
    UnsignedFloat16,
    SignedFloat16,
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

pub enum ChannelFormat {
    RGBA(usize, usize, usize, usize),
    BGRA(usize, usize, usize, usize),
    BGRUnused(usize, usize, usize, usize),
    RGB(usize, usize, usize),
    BGR(usize, usize, usize),
    RG(usize, usize),
    R(usize),
    A(usize),
    BitMask(usize, u32, u32, u32, u32),
}

pub enum UncompressedFormat {
    Typeless(ChannelFormat),
    Float(ChannelFormat),
    UnsignedNormalized(ChannelFormat),
    UnsignedNormalizedSrgb(ChannelFormat),
    SignedNormalized(ChannelFormat),
    UnsignedInt(ChannelFormat),
    SignedInt(ChannelFormat),
    Other(SpecialUncompressedFormat),
}

pub enum SpecialUncompressedFormat {
    R32G8X24Typeless,
    D32FloatS8X24UnsignedInt,
    R32FloatX8X24Typeless,
    X32TypelessG8X24UnsignedInt,
    D32Float,
    D24UnsignedNormalizedS8UnsignedInt,
    R24UnsignedNormalizedX8Typeless,
    X24TypelessG8UnsignedInt,
    D16UnsignedNormalized,
    R9G9B9E5SharedExponent,
    R10G10B10FixedPointBiasA2UnsignedNormalized,
    P8,
    A8P8,
}

pub enum TextureFormat {
    Uncompressed(UncompressedFormat),
    Compressed(CompressedFormat),
    Unknown(u32),
}
