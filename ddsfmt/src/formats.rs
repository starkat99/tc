use super::{
    BC6HCompressionType, BlockCompressionType,
    ChannelFormat::*,
    CompressedFormat, SignedCompressionType, SpecialUncompressedFormat,
    TextureFormat::{self, *},
    UncompressedFormat::*,
};

// DXGI Formats
pub const R32G32B32A32_TYPELESS: TextureFormat = Uncompressed(Typeless(RGBA(32, 32, 32, 32)));
pub const R32G32B32A32_FLOAT: TextureFormat = Uncompressed(Float(RGBA(32, 32, 32, 32)));
pub const R32G32B32A32_UINT: TextureFormat = Uncompressed(UnsignedInt(RGBA(32, 32, 32, 32)));
pub const R32G32B32A32_SINT: TextureFormat = Uncompressed(SignedInt(RGBA(32, 32, 32, 32)));

pub const R32G32B32_TYPELESS: TextureFormat = Uncompressed(Typeless(RGB(32, 32, 32)));
pub const R32G32B32_FLOAT: TextureFormat = Uncompressed(Float(RGB(32, 32, 32)));
pub const R32G32B32_UINT: TextureFormat = Uncompressed(UnsignedInt(RGB(32, 32, 32)));
pub const R32G32B32_SINT: TextureFormat = Uncompressed(SignedInt(RGB(32, 32, 32)));

pub const R16G16B16A16_TYPELESS: TextureFormat = Uncompressed(Typeless(RGBA(16, 16, 16, 16)));
pub const R16G16B16A16_FLOAT: TextureFormat = Uncompressed(Float(RGBA(16, 16, 16, 16)));
pub const R16G16B16A16_UNORM: TextureFormat =
    Uncompressed(UnsignedNormalized(RGBA(16, 16, 16, 16)));
pub const R16G16B16A16_SNORM: TextureFormat = Uncompressed(SignedNormalized(RGBA(16, 16, 16, 16)));
pub const R16G16B16A16_UINT: TextureFormat = Uncompressed(UnsignedInt(RGBA(16, 16, 16, 16)));
pub const R16G16B16A16_SINT: TextureFormat = Uncompressed(SignedInt(RGBA(16, 16, 16, 16)));

pub const R32G32_TYPELESS: TextureFormat = Uncompressed(Typeless(RG(32, 32)));
pub const R32G32_FLOAT: TextureFormat = Uncompressed(Float(RG(32, 32)));
pub const R32G32_UINT: TextureFormat = Uncompressed(UnsignedInt(RG(32, 32)));
pub const R32G32_SINT: TextureFormat = Uncompressed(SignedInt(RG(32, 32)));

pub const R32G8X24_TYPELESS: TextureFormat =
    Uncompressed(Other(SpecialUncompressedFormat::R32G8X24Typeless));
pub const D32_FLOAT_S8X24_UINT: TextureFormat =
    Uncompressed(Other(SpecialUncompressedFormat::D32FloatS8X24UnsignedInt));
pub const R32_FLOAT_X8X24_TYPELESS: TextureFormat =
    Uncompressed(Other(SpecialUncompressedFormat::R32FloatX8X24Typeless));
pub const X32_TYPELESS_G8X24_UINT: TextureFormat = Uncompressed(Other(
    SpecialUncompressedFormat::X32TypelessG8X24UnsignedInt,
));

pub const R10G10B10A2_TYPELESS: TextureFormat = Uncompressed(Typeless(RGBA(10, 10, 10, 2)));
pub const R10G10B10A2_UNORM: TextureFormat = Uncompressed(UnsignedNormalized(RGBA(10, 10, 10, 2)));
pub const R10G10B10A2_UINT: TextureFormat = Uncompressed(UnsignedInt(RGBA(10, 10, 10, 2)));

pub const R11G11B10_FLOAT: TextureFormat = Uncompressed(Float(RGB(11, 11, 10)));

pub const R8G8B8A8_TYPELESS: TextureFormat = Uncompressed(Typeless(RGBA(8, 8, 8, 8)));
pub const R8G8B8A8_UNORM: TextureFormat = Uncompressed(UnsignedNormalized(RGBA(8, 8, 8, 8)));
pub const R8G8B8A8_UNORM_SRGB: TextureFormat =
    Uncompressed(UnsignedNormalizedSrgb(RGBA(8, 8, 8, 8)));
pub const R8G8B8A8_UINT: TextureFormat = Uncompressed(UnsignedInt(RGBA(8, 8, 8, 8)));
pub const R8G8B8A8_SNORM: TextureFormat = Uncompressed(SignedNormalized(RGBA(8, 8, 8, 8)));
pub const R8G8B8A8_SINT: TextureFormat = Uncompressed(SignedInt(RGBA(8, 8, 8, 8)));

pub const R16G16_TYPELESS: TextureFormat = Uncompressed(Typeless(RG(16, 16)));
pub const R16G16_FLOAT: TextureFormat = Uncompressed(Float(RG(16, 16)));
pub const R16G16_UNORM: TextureFormat = Uncompressed(UnsignedNormalized(RG(16, 16)));
pub const R16G16_UINT: TextureFormat = Uncompressed(UnsignedInt(RG(16, 16)));
pub const R16G16_SNORM: TextureFormat = Uncompressed(SignedNormalized(RG(16, 16)));
pub const R16G16_SINT: TextureFormat = Uncompressed(SignedInt(RG(16, 16)));

pub const R32_TYPELESS: TextureFormat = Uncompressed(Typeless(R(32)));
pub const R32_FLOAT: TextureFormat = Uncompressed(Float(R(32)));
pub const R32_UINT: TextureFormat = Uncompressed(UnsignedInt(R(32)));
pub const R32_SINT: TextureFormat = Uncompressed(SignedInt(R(32)));

pub const R24G8_TYPELESS: TextureFormat = Uncompressed(Typeless(RG(24, 8)));

pub const D32_FLOAT: TextureFormat = Uncompressed(Other(SpecialUncompressedFormat::D32Float));
pub const D24_UNORM_S8_UINT: TextureFormat = Uncompressed(Other(
    SpecialUncompressedFormat::D24UnsignedNormalizedS8UnsignedInt,
));
pub const R24_UNORM_X8_TYPELESS: TextureFormat = Uncompressed(Other(
    SpecialUncompressedFormat::R24UnsignedNormalizedX8Typeless,
));
pub const X24_TYPELESS_G8_UINT: TextureFormat =
    Uncompressed(Other(SpecialUncompressedFormat::X24TypelessG8UnsignedInt));
pub const D16_UNORM: TextureFormat =
    Uncompressed(Other(SpecialUncompressedFormat::D16UnsignedNormalized));

pub const R8G8_TYPELESS: TextureFormat = Uncompressed(Typeless(RG(8, 8)));
pub const R8G8_UNORM: TextureFormat = Uncompressed(UnsignedNormalized(RG(8, 8)));
pub const R8G8_UINT: TextureFormat = Uncompressed(UnsignedInt(RG(8, 8)));
pub const R8G8_SNORM: TextureFormat = Uncompressed(SignedNormalized(RG(8, 8)));
pub const R8G8_SINT: TextureFormat = Uncompressed(SignedInt(RG(8, 8)));

pub const R16_TYPELESS: TextureFormat = Uncompressed(Typeless(R(16)));
pub const R16_FLOAT: TextureFormat = Uncompressed(Float(R(16)));
pub const R16_UNORM: TextureFormat = Uncompressed(UnsignedNormalized(R(16)));
pub const R16_UINT: TextureFormat = Uncompressed(UnsignedInt(R(16)));
pub const R16_SNORM: TextureFormat = Uncompressed(SignedNormalized(R(16)));
pub const R16_SINT: TextureFormat = Uncompressed(SignedNormalized(R(16)));

pub const R8_TYPELESS: TextureFormat = Uncompressed(Typeless(R(8)));
pub const R8_FLOAT: TextureFormat = Uncompressed(Float(R(8)));
pub const R8_UNORM: TextureFormat = Uncompressed(UnsignedNormalized(R(8)));
pub const R8_UINT: TextureFormat = Uncompressed(UnsignedInt(R(8)));
pub const R8_SNORM: TextureFormat = Uncompressed(SignedNormalized(R(8)));
pub const R8_SINT: TextureFormat = Uncompressed(SignedNormalized(R(8)));

pub const A8_UNORM: TextureFormat = Uncompressed(UnsignedNormalized(A(8)));
pub const R1_UNORM: TextureFormat = Uncompressed(UnsignedNormalized(R(1)));

pub const R9G9B9E5_SHAREDEXP: TextureFormat =
    Uncompressed(Other(SpecialUncompressedFormat::R9G9B9E5SharedExponent));

pub const R8G8_B8G8_UNORM: TextureFormat = Compressed(CompressedFormat::R8G8B8G8);
pub const G8R8_G8B8_UNORM: TextureFormat = Compressed(CompressedFormat::G8R8G8B8);

pub const BC1_TYPELESS: TextureFormat =
    Compressed(CompressedFormat::BC1(BlockCompressionType::Typeless));
pub const BC1_UNORM: TextureFormat = Compressed(CompressedFormat::BC1(
    BlockCompressionType::UnsignedNormalized,
));
pub const BC1_UNORM_SRGB: TextureFormat = Compressed(CompressedFormat::BC1(
    BlockCompressionType::UnsignedNormalizedSrgb,
));

pub const BC2_TYPELESS: TextureFormat =
    Compressed(CompressedFormat::BC2(BlockCompressionType::Typeless));
pub const BC2_UNORM: TextureFormat = Compressed(CompressedFormat::BC2(
    BlockCompressionType::UnsignedNormalized,
));
pub const BC2_UNORM_SRGB: TextureFormat = Compressed(CompressedFormat::BC2(
    BlockCompressionType::UnsignedNormalizedSrgb,
));

pub const BC3_TYPELESS: TextureFormat =
    Compressed(CompressedFormat::BC3(BlockCompressionType::Typeless));
pub const BC3_UNORM: TextureFormat = Compressed(CompressedFormat::BC3(
    BlockCompressionType::UnsignedNormalized,
));
pub const BC3_UNORM_SRGB: TextureFormat = Compressed(CompressedFormat::BC3(
    BlockCompressionType::UnsignedNormalizedSrgb,
));

pub const BC4_TYPELESS: TextureFormat =
    Compressed(CompressedFormat::BC4(SignedCompressionType::Typeless));
pub const BC4_UNORM: TextureFormat = Compressed(CompressedFormat::BC4(
    SignedCompressionType::UnsignedNormalized,
));
pub const BC4_SNORM: TextureFormat = Compressed(CompressedFormat::BC4(
    SignedCompressionType::SignedNormalized,
));

pub const BC5_TYPELESS: TextureFormat =
    Compressed(CompressedFormat::BC5(SignedCompressionType::Typeless));
pub const BC5_UNORM: TextureFormat = Compressed(CompressedFormat::BC5(
    SignedCompressionType::UnsignedNormalized,
));
pub const BC5_SNORM: TextureFormat = Compressed(CompressedFormat::BC5(
    SignedCompressionType::SignedNormalized,
));

pub const B5G6R5_UNORM: TextureFormat = Uncompressed(UnsignedNormalized(BGR(5, 6, 5)));
pub const B5G5R5A1_UNORM: TextureFormat = Uncompressed(UnsignedNormalized(BGRA(5, 5, 5, 1)));
pub const B5G6R5A8_UNORM: TextureFormat = Uncompressed(UnsignedNormalized(BGRA(5, 6, 5, 8)));
pub const B5G6R5X8_UNORM: TextureFormat = Uncompressed(UnsignedNormalized(BGRUnused(5, 6, 5, 8)));

pub const R10G10B10_XR_BIAS_A2_UNORM: TextureFormat = Uncompressed(Other(
    SpecialUncompressedFormat::R10G10B10FixedPointBiasA2UnsignedNormalized,
));

pub const B8G8R8A8_TYPELESS: TextureFormat = Uncompressed(Typeless(BGRA(8, 8, 8, 8)));
pub const B8G8R8A8_UNORM: TextureFormat = Uncompressed(UnsignedNormalized(BGRA(8, 8, 8, 8)));
pub const B8G8R8A8_UNORM_SRGB: TextureFormat =
    Uncompressed(UnsignedNormalizedSrgb(BGRA(8, 8, 8, 8)));
pub const B8G8R8X8_TYPELESS: TextureFormat = Uncompressed(Typeless(BGRUnused(8, 8, 8, 8)));
pub const B8G8R8X8_UNORM: TextureFormat = Uncompressed(UnsignedNormalized(BGRUnused(8, 8, 8, 8)));
pub const B8G8R8X8_UNORM_SRGB: TextureFormat =
    Uncompressed(UnsignedNormalizedSrgb(BGRUnused(8, 8, 8, 8)));

pub const BC6H_TYPELESS: TextureFormat =
    Compressed(CompressedFormat::BC6H(BC6HCompressionType::Typeless));
pub const BC6H_UF16: TextureFormat =
    Compressed(CompressedFormat::BC6H(BC6HCompressionType::UnsignedFloat16));
pub const BC6H_SF16: TextureFormat =
    Compressed(CompressedFormat::BC6H(BC6HCompressionType::SignedFloat16));

pub const BC7_TYPELESS: TextureFormat =
    Compressed(CompressedFormat::BC7(BlockCompressionType::Typeless));
pub const BC7_UNORM: TextureFormat = Compressed(CompressedFormat::BC7(
    BlockCompressionType::UnsignedNormalized,
));
pub const BC7_UNORM_SRGB: TextureFormat = Compressed(CompressedFormat::BC7(
    BlockCompressionType::UnsignedNormalizedSrgb,
));

pub const AYUV: TextureFormat = Compressed(CompressedFormat::AYUV);
pub const Y410: TextureFormat = Compressed(CompressedFormat::Y410);
pub const Y416: TextureFormat = Compressed(CompressedFormat::Y416);
pub const NV12: TextureFormat = Compressed(CompressedFormat::NV12);
pub const P010: TextureFormat = Compressed(CompressedFormat::P010);
pub const P016: TextureFormat = Compressed(CompressedFormat::P016);
pub const OPAQUE_420: TextureFormat = Compressed(CompressedFormat::Opaque420);
pub const YUY2: TextureFormat = Compressed(CompressedFormat::YUY2);
pub const Y210: TextureFormat = Compressed(CompressedFormat::Y210);
pub const Y216: TextureFormat = Compressed(CompressedFormat::Y216);
pub const NV11: TextureFormat = Compressed(CompressedFormat::NV11);
pub const AI44: TextureFormat = Compressed(CompressedFormat::AI44);
pub const IA44: TextureFormat = Compressed(CompressedFormat::IA44);

pub const P8: TextureFormat = Uncompressed(Other(SpecialUncompressedFormat::P8));
pub const A8P8: TextureFormat = Uncompressed(Other(SpecialUncompressedFormat::A8P8));

pub const B4G4R4A4_UNORM: TextureFormat = Uncompressed(UnsignedNormalized(BGRA(4, 4, 4, 4)));

pub const P208: TextureFormat = Compressed(CompressedFormat::P208);
pub const V208: TextureFormat = Compressed(CompressedFormat::V208);
pub const V408: TextureFormat = Compressed(CompressedFormat::V408);
