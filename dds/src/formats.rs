use super::{TextureFormat, SpecialUncompressedFormat, CompressedFormat, BlockCompressionType, SignedCompressionType, BC6HCompressionType};
use super::TextureFormat::{Uncompressed, Compressed};
use super::UncompressedFormat::*;
use super::ChannelFormat::*;

// DXGI Formats
const R32G32B32A32_TYPELESS: TextureFormat = Uncompressed(Typeless(RGBA(32, 32, 32, 32)));
const R32G32B32A32_FLOAT: TextureFormat = Uncompressed(Float(RGBA(32, 32, 32, 32)));
const R32G32B32A32_UINT: TextureFormat = Uncompressed(UnsignedInt(RGBA(32, 32, 32, 32)));
const R32G32B32A32_SINT: TextureFormat = Uncompressed(SignedInt(RGBA(32, 32, 32, 32)));

const R32G32B32_TYPELESS: TextureFormat = Uncompressed(Typeless(RGB(32, 32, 32)));
const R32G32B32_FLOAT: TextureFormat = Uncompressed(Float(RGB(32, 32, 32)));
const R32G32B32_UINT: TextureFormat = Uncompressed(UnsignedInt(RGB(32, 32, 32)));
const R32G32B32_SINT: TextureFormat = Uncompressed(SignedInt(RGB(32, 32, 32)));

const R16G16B16A16_TYPELESS: TextureFormat = Uncompressed(Typeless(RGBA(16, 16, 16, 16)));
const R16G16B16A16_FLOAT: TextureFormat = Uncompressed(Float(RGBA(16, 16, 16, 16)));
const R16G16B16A16_UNORM: TextureFormat = Uncompressed(UnsignedNormalized(RGBA(16, 16, 16, 16)));
const R16G16B16A16_SNORM: TextureFormat = Uncompressed(SignedNormalized(RGBA(16, 16, 16, 16)));
const R16G16B16A16_UINT: TextureFormat = Uncompressed(UnsignedInt(RGBA(16, 16, 16, 16)));
const R16G16B16A16_SINT: TextureFormat = Uncompressed(SignedInt(RGBA(16, 16, 16, 16)));

const R32G32_TYPELESS: TextureFormat = Uncompressed(Typeless(RG(32, 32)));
const R32G32_FLOAT: TextureFormat = Uncompressed(Float(RG(32, 32)));
const R32G32_UINT: TextureFormat = Uncompressed(UnsignedInt(RG(32, 32)));
const R32G32_SINT: TextureFormat = Uncompressed(SignedInt(RG(32, 32)));

const R32G8X24_TYPELESS: TextureFormat = Uncompressed(Other(SpecialUncompressedFormat::R32G8X24Typeless));
const D32_FLOAT_S8X24_UINT: TextureFormat = Uncompressed(Other(SpecialUncompressedFormat::D32FloatS8X24UnsignedInt));
const R32_FLOAT_X8X24_TYPELESS: TextureFormat = Uncompressed(Other(SpecialUncompressedFormat::R32FloatX8X24Typeless));
const X32_TYPELESS_G8X24_UINT: TextureFormat = Uncompressed(Other(SpecialUncompressedFormat::X32TypelessG8X24UnsignedInt));

const R10G10B10A2_TYPELESS: TextureFormat = Uncompressed(Typeless(RGBA(10, 10, 10, 2)));
const R10G10B10A2_UNORM: TextureFormat = Uncompressed(UnsignedNormalized(RGBA(10, 10, 10, 2)));
const R10G10B10A2_UINT: TextureFormat = Uncompressed(UnsignedInt(RGBA(10, 10, 10, 2)));

const R11G11B10_FLOAT: TextureFormat = Uncompressed(Float(RGB(11, 11, 10)));

const R8G8B8A8_TYPELESS: TextureFormat = Uncompressed(Typeless(RGBA(8, 8, 8, 8)));
const R8G8B8A8_UNORM: TextureFormat = Uncompressed(UnsignedNormalized(RGBA(8, 8, 8, 8)));
const R8G8B8A8_UNORM_SRGB: TextureFormat = Uncompressed(UnsignedNormalizedSrgb(RGBA(8, 8, 8, 8)));
const R8G8B8A8_UINT: TextureFormat = Uncompressed(UnsignedInt(RGBA(8, 8, 8, 8)));
const R8G8B8A8_SNORM: TextureFormat = Uncompressed(SignedNormalized(RGBA(8, 8, 8, 8)));
const R8G8B8A8_SINT: TextureFormat = Uncompressed(SignedInt(RGBA(8, 8, 8, 8)));

const R16G16_TYPELESS: TextureFormat = Uncompressed(Typeless(RG(16, 16)));
const R16G16_FLOAT: TextureFormat = Uncompressed(Float(RG(16, 16)));
const R16G16_UNORM: TextureFormat = Uncompressed(UnsignedNormalized(RG(16, 16)));
const R16G16_UINT: TextureFormat = Uncompressed(UnsignedInt(RG(16, 16)));
const R16G16_SNORM: TextureFormat = Uncompressed(SignedNormalized(RG(16, 16)));
const R16G16_SINT: TextureFormat = Uncompressed(SignedInt(RG(16, 16)));

const R32_TYPELESS: TextureFormat = Uncompressed(Typeless(R(32)));
const R32_FLOAT: TextureFormat = Uncompressed(Float(R(32)));
const R32_UINT: TextureFormat = Uncompressed(UnsignedInt(R(32)));
const R32_SINT: TextureFormat = Uncompressed(SignedInt(R(32)));

const R24G8_TYPELESS: TextureFormat = Uncompressed(Typeless(RG(24, 8)));

const D32_FLOAT: TextureFormat = Uncompressed(Other(SpecialUncompressedFormat::D32Float));
const D24_UNORM_S8_UINT: TextureFormat = Uncompressed(Other(SpecialUncompressedFormat::D24UnsignedNormalizedS8UnsignedInt));
const R24_UNORM_X8_TYPELESS: TextureFormat = Uncompressed(Other(SpecialUncompressedFormat::R24UnsignedNormalizedX8Typeless));
const X24_TYPELESS_G8_UINT: TextureFormat = Uncompressed(Other(SpecialUncompressedFormat::X24TypelessG8UnsignedInt));
const D16_UNORM: TextureFormat = Uncompressed(Other(SpecialUncompressedFormat::D16UnsignedNormalized));

const R8G8_TYPELESS: TextureFormat = Uncompressed(Typeless(RG(8, 8)));
const R8G8_UNORM: TextureFormat = Uncompressed(UnsignedNormalized(RG(8, 8)));
const R8G8_UINT: TextureFormat = Uncompressed(UnsignedInt(RG(8, 8)));
const R8G8_SNORM: TextureFormat = Uncompressed(SignedNormalized(RG(8, 8)));
const R8G8_SINT: TextureFormat = Uncompressed(SignedInt(RG(8, 8)));

const R16_TYPELESS: TextureFormat = Uncompressed(Typeless(R(16)));
const R16_FLOAT: TextureFormat = Uncompressed(Float(R(16)));
const R16_UNORM: TextureFormat = Uncompressed(UnsignedNormalized(R(16)));
const R16_UINT: TextureFormat = Uncompressed(UnsignedInt(R(16)));
const R16_SNORM: TextureFormat = Uncompressed(SignedNormalized(R(16)));
const R16_SINT: TextureFormat = Uncompressed(SignedNormalized(R(16)));

const R8_TYPELESS: TextureFormat = Uncompressed(Typeless(R(8)));
const R8_FLOAT: TextureFormat = Uncompressed(Float(R(8)));
const R8_UNORM: TextureFormat = Uncompressed(UnsignedNormalized(R(8)));
const R8_UINT: TextureFormat = Uncompressed(UnsignedInt(R(8)));
const R8_SNORM: TextureFormat = Uncompressed(SignedNormalized(R(8)));
const R8_SINT: TextureFormat = Uncompressed(SignedNormalized(R(8)));

const A8_UNORM: TextureFormat = Uncompressed(UnsignedNormalized(A(8)));
const R1_UNORM: TextureFormat = Uncompressed(UnsignedNormalized(R(1)));

const R9G9B9E5_SHAREDEXP: TextureFormat = Uncompressed(Other(SpecialUncompressedFormat::R9G9B9E5SharedExponent));

const R8G8_B8G8_UNORM: TextureFormat = Compressed(CompressedFormat::R8G8B8G8);
const G8R8_G8B8_UNORM: TextureFormat = Compressed(CompressedFormat::G8R8G8B8);

const BC1_TYPELESS: TextureFormat = Compressed(CompressedFormat::BC1(BlockCompressionType::Typeless));
const BC1_UNORM: TextureFormat = Compressed(CompressedFormat::BC1(BlockCompressionType::UnsignedNormalized));
const BC1_UNORM_SRGB: TextureFormat = Compressed(CompressedFormat::BC1(BlockCompressionType::UnsignedNormalizedSrgb));

const BC2_TYPELESS: TextureFormat = Compressed(CompressedFormat::BC2(BlockCompressionType::Typeless));
const BC2_UNORM: TextureFormat = Compressed(CompressedFormat::BC2(BlockCompressionType::UnsignedNormalized));
const BC2_UNORM_SRGB: TextureFormat = Compressed(CompressedFormat::BC2(BlockCompressionType::UnsignedNormalizedSrgb));

const BC3_TYPELESS: TextureFormat = Compressed(CompressedFormat::BC3(BlockCompressionType::Typeless));
const BC3_UNORM: TextureFormat = Compressed(CompressedFormat::BC3(BlockCompressionType::UnsignedNormalized));
const BC3_UNORM_SRGB: TextureFormat = Compressed(CompressedFormat::BC3(BlockCompressionType::UnsignedNormalizedSrgb));

const BC4_TYPELESS: TextureFormat = Compressed(CompressedFormat::BC4(SignedCompressionType::Typeless));
const BC4_UNORM: TextureFormat = Compressed(CompressedFormat::BC4(SignedCompressionType::UnsignedNormalized));
const BC4_SNORM: TextureFormat = Compressed(CompressedFormat::BC4(SignedCompressionType::SignedNormalized));

const BC5_TYPELESS: TextureFormat = Compressed(CompressedFormat::BC5(SignedCompressionType::Typeless));
const BC5_UNORM: TextureFormat = Compressed(CompressedFormat::BC5(SignedCompressionType::UnsignedNormalized));
const BC5_SNORM: TextureFormat = Compressed(CompressedFormat::BC5(SignedCompressionType::SignedNormalized));

const B5G6R5_UNORM: TextureFormat = Uncompressed(UnsignedNormalized(BGR(5, 6, 5)));
const B5G6R5A1_UNORM: TextureFormat = Uncompressed(UnsignedNormalized(BGRA(5, 6, 5, 1)));
const B5G6R5A8_UNORM: TextureFormat = Uncompressed(UnsignedNormalized(BGRA(5, 6, 5, 8)));
const B5G6R5X8_UNORM: TextureFormat = Uncompressed(UnsignedNormalized(BGRUnused(5, 6, 5, 8)));

const R10G10B10_XR_BIAS_A2_UNORM: TextureFormat = Uncompressed(Other(SpecialUncompressedFormat::R10G10B10FixedPointBiasA2UnsignedNormalized));

const B8G8R8A8_TYPELESS: TextureFormat = Uncompressed(Typeless(BGRA(8, 8, 8, 8)));
const B8G8R8A8_UNORM_SRGB: TextureFormat = Uncompressed(UnsignedNormalizedSrgb(BGRA(8, 8, 8, 8)));
const B8G8R8X8_TYPELESS: TextureFormat = Uncompressed(Typeless(BGRUnused(8, 8, 8, 8)));
const B8G8R8X8_UNORM_SRGB: TextureFormat = Uncompressed(UnsignedNormalizedSrgb(BGRUnused(8, 8, 8, 8)));

const BC6H_TYPELESS: TextureFormat = Compressed(CompressedFormat::BC6H(BC6HCompressionType::Typeless));
const BC6H_UF16: TextureFormat = Compressed(CompressedFormat::BC6H(BC6HCompressionType::UnsignedFloat16));
const BC6H_SF16: TextureFormat = Compressed(CompressedFormat::BC6H(BC6HCompressionType::SignedFloat16));

const BC7_TYPELESS: TextureFormat = Compressed(CompressedFormat::BC7(BlockCompressionType::Typeless));
const BC7_UNORM: TextureFormat = Compressed(CompressedFormat::BC7(BlockCompressionType::UnsignedNormalized));
const BC7_UNORM_SRGB: TextureFormat = Compressed(CompressedFormat::BC7(BlockCompressionType::UnsignedNormalizedSrgb));

const AYUV: TextureFormat = Compressed(CompressedFormat::AYUV);
const Y410: TextureFormat = Compressed(CompressedFormat::Y410);
const Y416: TextureFormat = Compressed(CompressedFormat::Y416);
const NV12: TextureFormat = Compressed(CompressedFormat::NV12);
const P010: TextureFormat = Compressed(CompressedFormat::P010);
const P016: TextureFormat = Compressed(CompressedFormat::P016);
const OPAQUE_420: TextureFormat = Compressed(CompressedFormat::Opaque420);
const YUY2: TextureFormat = Compressed(CompressedFormat::YUY2);
const Y210: TextureFormat = Compressed(CompressedFormat::Y210);
const Y216: TextureFormat = Compressed(CompressedFormat::Y216);
const NV11: TextureFormat = Compressed(CompressedFormat::NV11);
const AI44: TextureFormat = Compressed(CompressedFormat::AI44);
const IA44: TextureFormat = Compressed(CompressedFormat::IA44);

const P8: TextureFormat = Uncompressed(Other(SpecialUncompressedFormat::P8));
const A8P8: TextureFormat = Uncompressed(Other(SpecialUncompressedFormat::A8P8));

const B4G4R4A4_UNORM: TextureFormat = Uncompressed(UnsignedNormalized(BGRA(4, 4, 4, 4)));

const P208: TextureFormat = Compressed(CompressedFormat::P208);
const V208: TextureFormat = Compressed(CompressedFormat::V208);
const V408: TextureFormat = Compressed(CompressedFormat::V408);
