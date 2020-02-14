use bitflags::bitflags;
use byteorder::{ByteOrder, LittleEndian, ReadBytesExt, WriteBytesExt};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::{
    convert::TryFrom,
    io::{Read, Write},
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DdsError {
    #[error("invalid DDS header")]
    InvalidHeader,
    #[error("header format is not supported")]
    UnsupportedFormat,
    #[error("DDS io error: {0}")]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, DdsError>;

bitflags! {
    pub struct HeaderFlags: u32 {
        const CAPS = 0x1;
        const HEIGHT = 0x2;
        const WIDTH = 0x4;
        const PITCH = 0x8;
        const PIXEL_FORMAT = 0x1000;
        const MIPMAP_COUNT = 0x20000;
        const LINEAR_SIZE = 0x80000;
        const DEPTH = 0x800000;
        const TEXTURE = Self::CAPS.bits
                      | Self::HEIGHT.bits
                      | Self::WIDTH.bits
                      | Self::PIXEL_FORMAT.bits;
    }
}

bitflags! {
    pub struct CapsFlags: u32 {
        const COMPLEX = 0x8;
        const MIPMAP = 0x400000;
        const TEXTURE = 0x1000;
    }
}

bitflags! {
    pub struct Caps2Flags: u32 {
        const CUBEMAP = 0x200;
        const CUBEMAP_POSITIVE_X = 0x400;
        const CUBEMAP_NEGATIVE_X = 0x800;
        const CUBEMAP_POSITIVE_Y = 0x1000;
        const CUBEMAP_NEGATIVE_Y = 0x2000;
        const CUBEMAP_POSITIVE_Z = 0x4000;
        const CUBEMAP_NEGATIVE_Z = 0x8000;
        const VOLUME = 0x200000;
        const CUBEMAP_ALLFACES = Self::CUBEMAP.bits
                               | Self::CUBEMAP_POSITIVE_X.bits
                               | Self::CUBEMAP_NEGATIVE_X.bits
                               | Self::CUBEMAP_POSITIVE_Y.bits
                               | Self::CUBEMAP_NEGATIVE_Y.bits
                               | Self::CUBEMAP_POSITIVE_Z.bits
                               | Self::CUBEMAP_NEGATIVE_Z.bits;
    }
}

bitflags! {
    pub struct PixelFormatFlags: u32 {
        const ALPHA_PIXELS = 0x1;
        const ALPHA = 0x2;
        const FOURCC = 0x4;
        const RGB = 0x40;
        const YUV = 0x200;
        const LUMINANCE = 0x20000;
    }
}

bitflags! {
    pub struct MiscFlags: u32 {
        const TEXTURE_CUBE = 0x4;
    }
}

bitflags! {
    pub struct Misc2Flags: u32 {
        const ALPHA_MODE_UNKNOWN = 0x0;
        const ALPHA_MODE_STRAIGHT = 0x1;
        const ALPHA_MODE_PREMULTIPLIED = 0x2;
        const ALPHA_MODE_OPAQUE = 0x3;
        const ALPHA_MODE_CUSTOM = 0x4;
    }
}

#[allow(non_camel_case_types)]
#[derive(
    Debug, Copy, Clone, Eq, PartialEq, Hash, PartialOrd, Ord, IntoPrimitive, TryFromPrimitive,
)]
#[repr(u32)]
enum Format {
    Unknown = 0,
    R32G32B32A32Typeless = 1,
    R32G32B32A32Float = 2,
    R32G32B32A32UnsignedInt = 3,
    R32G32B32A32SignedInt = 4,
    R32G32B32Typeless = 5,
    R32G32B32Float = 6,
    R32G32B32UnsignedInt = 7,
    R32G32B32SignedInt = 8,
    R16G16B16A16Typeless = 9,
    R16G16B16A16Float = 10,
    R16G16B16A16UnsignedNormalized = 11,
    R16G16B16A16UnsignedInt = 12,
    R16G16B16A16SignedNormalized = 13,
    R16G16B16A16SignedInt = 14,
    R32G32Typeless = 15,
    R32G32Float = 16,
    R32G32UnsignedInt = 17,
    R32G32SignedInt = 18,
    R32G8X24Typeless = 19,
    D32FloatS8X24UnsignedInt = 20,
    R32FloatX8X24Typeless = 21,
    X32TypelessG8X24UnsignedInt = 22,
    R10G10B10A2Typeless = 23,
    R10G10B10A2UnsignedNormalized = 24,
    R10G10B10A2UnsignedInt = 25,
    R11G11B10Float = 26,
    R8G8B8A8Typeless = 27,
    R8G8B8A8UnsignedNormalized = 28,
    R8G8B8A8UnsignedNormalizedSrgb = 29,
    R8G8B8A8UnsignedInt = 30,
    R8G8B8A8SignedNormalized = 31,
    R8G8B8A8SignedInt = 32,
    R16G16Typeless = 33,
    R16G16Float = 34,
    R16G16UnsignedNormalized = 35,
    R16G16UnsignedInt = 36,
    R16G16SignedNormalized = 37,
    R16G16SignedInt = 38,
    R32Typeless = 39,
    D32Float = 40,
    R32Float = 41,
    R32UnsignedInt = 42,
    R32SignedInt = 43,
    R24G8Typeless = 44,
    D24UnsignedNormalizedS8UnsignedInt = 45,
    R24UnsignedNormalizedX8Typeless = 46,
    X24TypelessG8UnsignedInt = 47,
    R8G8Typeless = 48,
    R8G8UnsignedNormalized = 49,
    R8G8UnsignedInt = 50,
    R8G8SignedNormalized = 51,
    R8G8SignedInt = 52,
    R16Typeless = 53,
    R16Float = 54,
    D16UnsignedNormalized = 55,
    R16UnsignedNormalized = 56,
    R16UnsignedInt = 57,
    R16SignedNormalized = 58,
    R16SignedInt = 59,
    R8Typeless = 60,
    R8UnsignedNormalized = 61,
    R8UnsignedInt = 62,
    R8SignedNormalized = 63,
    R8SignedInt = 64,
    A8UnsignedNormalized = 65,
    R1UnsignedNormalized = 66,
    R9G9B9E5SharedExp = 67,
    R8G8_B8G8UnsignedNormalized = 68,
    G8R8_G8B8UnsignedNormalized = 69,
    BC1Typeless = 70,
    BC1UnsignedNormalized = 71,
    BC1UnsignedNormalizedSrgb = 72,
    BC2Typeless = 73,
    BC2UnsignedNormalized = 74,
    BC2UnsignedNormalizedSrgb = 75,
    BC3Typeless = 76,
    BC3UnsignedNormalized = 77,
    BC3UnsignedNormalizedSrgb = 78,
    BC4Typeless = 79,
    BC4UnsignedNormalized = 80,
    BC4SignedNormalized = 81,
    BC5Typeless = 82,
    BC5UnsignedNormalized = 83,
    BC5SignedNormalized = 84,
    B5G6R5UnsignedNormalized = 85,
    B5G5R5A1UnsignedNormalized = 86,
    B8G8R8A8UnsignedNormalized = 87,
    B8G8R8X8UnsignedNormalized = 88,
    R10G10B10XrBiasA2UnsignedNormalized = 89,
    B8G8R8A8Typeless = 90,
    B8G8R8A8UnsignedNormalizedSrgb = 91,
    B8G8R8X8Typeless = 92,
    B8G8R8X8UnsignedNormalizedSrgb = 93,
    BC6HTypeless = 94,
    BC6HUnsignedFloat16 = 95,
    BC6HSignedFloat16 = 96,
    BC7Typeless = 97,
    BC7UnsignedNormalized = 98,
    BC7UnsignedNormalizedSrgb = 99,
    AYUV = 100,
    Y410 = 101,
    Y416 = 102,
    NV12 = 103,
    P010 = 104,
    P016 = 105,
    Opaque420 = 106,
    YUY2 = 107,
    Y210 = 108,
    Y216 = 109,
    NV11 = 110,
    AI44 = 111,
    IA44 = 112,
    P8 = 113,
    A8P8 = 114,
    B4G4R4A4UnsignedNormalized = 115,
    P208 = 130,
    V208 = 131,
    V408 = 132,
}

#[derive(
    Debug, Copy, Clone, Eq, PartialEq, Hash, PartialOrd, Ord, IntoPrimitive, TryFromPrimitive,
)]
#[repr(u32)]
enum ResourceDimension {
    Texture1D = 2,
    Texture2D = 3,
    Texture3D = 4,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum SizeField {
    Pitch(u32),
    Linear(u32),
}

#[derive(Debug, Clone)]
pub struct DdsHeader {
    height: u32,
    width: u32,
    size: Option<SizeField>,
    depth: Option<u32>,
    mipmap_count: Option<u32>,
    pixel_format: PixelFormatFlags,
    four_cc_bytes: Option<[u8; 4]>,
    rgb_bit_counts: u32,
    r_bit_mask: u32,
    g_bit_mask: u32,
    b_bit_mask: u32,
    a_bit_mask: u32,
    caps2: Caps2Flags,
    format: Option<Format>,
    dimension: Option<ResourceDimension>,
    array_size: Option<u32>,
    misc2: Option<Misc2Flags>,
}

impl DdsHeader {
    const BYTE_SIZE: usize = 128;
    const PIXEL_FORMAT_BYTE_SIZE: u32 = 32;
    const EXTENDED_BYTE_SIZE: usize = 20;

    fn calculate_pitch_or_linear_size(&self) -> SizeField {
        unimplemented!()
    }

    fn is_extended_header(&self) -> bool {
        &self.four_cc_bytes.unwrap_or_default() == b"DX10"
    }

    pub fn read_from_byte_order<E: ByteOrder, R: Read>(mut reader: R) -> Result<DdsHeader> {
        let mut buf: [u8; 4] = [0; 4];
        reader.read_exact(&mut buf)?;
        if &buf != b"DDS " {
            return Err(DdsError::InvalidHeader);
        }
        let byte_size = reader.read_u32::<E>()?;
        if byte_size as usize != DdsHeader::BYTE_SIZE {
            return Err(DdsError::InvalidHeader);
        }
        let flags =
            HeaderFlags::from_bits(reader.read_u32::<E>()?).ok_or(DdsError::UnsupportedFormat)?;
        let height = reader.read_u32::<E>()?;
        let width = reader.read_u32::<E>()?;
        let pitch_or_linear_size = reader.read_u32::<E>()?;
        let size = if flags.contains(HeaderFlags::LINEAR_SIZE) {
            Some(SizeField::Linear(pitch_or_linear_size))
        } else if flags.contains(HeaderFlags::PITCH) {
            Some(SizeField::Pitch(pitch_or_linear_size))
        } else {
            None
        };
        let depth = reader.read_u32::<E>()?;
        let depth = if flags.contains(HeaderFlags::DEPTH) {
            Some(depth)
        } else {
            None
        };
        let mipmap_count = reader.read_u32::<E>()?;
        let mipmap_count = if flags.contains(HeaderFlags::MIPMAP_COUNT) {
            Some(mipmap_count)
        } else {
            None
        };
        let mut buf: [u8; 11 * 4] = [0; 11 * 4];
        reader.read_exact(&mut buf)?; // Reserved bytes

        let byte_size = reader.read_u32::<E>()?;
        if byte_size != DdsHeader::PIXEL_FORMAT_BYTE_SIZE {
            return Err(DdsError::InvalidHeader);
        }
        let pixel_format = PixelFormatFlags::from_bits(reader.read_u32::<E>()?)
            .ok_or(DdsError::UnsupportedFormat)?;

        let mut four_cc_bytes: [u8; 4] = [0; 4];
        reader.read_exact(&mut four_cc_bytes)?;
        let is_extended_header = &four_cc_bytes == b"DX10";
        let four_cc_bytes = if pixel_format.contains(PixelFormatFlags::FOURCC) {
            Some(four_cc_bytes)
        } else {
            None
        };

        let rgb_bit_counts = reader.read_u32::<E>()?;
        let r_bit_mask = reader.read_u32::<E>()?;
        let g_bit_mask = reader.read_u32::<E>()?;
        let b_bit_mask = reader.read_u32::<E>()?;
        let a_bit_mask = reader.read_u32::<E>()?;

        reader.read_u32::<E>()?; // Caps, don't need this info, will generate ourselves if needed
        let caps2 =
            Caps2Flags::from_bits(reader.read_u32::<E>()?).ok_or(DdsError::UnsupportedFormat)?;
        let mut buf: [u8; 3 * 4] = [0; 3 * 4];
        reader.read_exact(&mut buf)?; // Unused/reserved bytes

        let format: Option<Format> = if is_extended_header {
            Some(Format::try_from(reader.read_u32::<E>()?).or(Err(DdsError::UnsupportedFormat))?)
        } else {
            None
        };
        let dimension: Option<ResourceDimension> = if is_extended_header {
            Some(
                ResourceDimension::try_from(reader.read_u32::<E>()?)
                    .or(Err(DdsError::UnsupportedFormat))?,
            )
        } else {
            None
        };
        let caps2 = if is_extended_header {
            let misc =
                MiscFlags::from_bits(reader.read_u32::<E>()?).ok_or(DdsError::UnsupportedFormat)?;
            if misc.contains(MiscFlags::TEXTURE_CUBE) {
                caps2 | Caps2Flags::CUBEMAP
            } else {
                caps2
            }
        } else {
            caps2
        };
        let array_size: Option<u32> = if is_extended_header {
            Some(reader.read_u32::<E>()?)
        } else {
            None
        };
        let misc2: Option<Misc2Flags> = if is_extended_header {
            Some(
                Misc2Flags::from_bits(reader.read_u32::<E>()?)
                    .ok_or(DdsError::UnsupportedFormat)?,
            )
        } else {
            None
        };

        Ok(DdsHeader {
            height,
            width,
            size,
            depth,
            mipmap_count,
            pixel_format,
            four_cc_bytes,
            rgb_bit_counts,
            r_bit_mask,
            g_bit_mask,
            b_bit_mask,
            a_bit_mask,
            caps2,
            format,
            dimension,
            array_size,
            misc2,
        })
    }

    #[inline]
    pub fn read_from<R: Read>(reader: R) -> Result<DdsHeader> {
        DdsHeader::read_from_byte_order::<LittleEndian, _>(reader)
    }

    pub fn write_to_byte_order<E: ByteOrder, W: Write>(&self, mut writer: W) -> Result<()> {
        let is_extended_header = self.is_extended_header();
        let mut flags = HeaderFlags::TEXTURE; // Required flags
        let pitch_or_linear_size: u32;
        let mut depth = self.depth.unwrap_or_default();
        let mut caps = CapsFlags::TEXTURE; // Required
        let mut caps2 = self.caps2;
        let mut misc = MiscFlags::empty();
        let mut dimension = self.dimension.unwrap_or(ResourceDimension::Texture2D);
        let mut pixel_format = self.pixel_format;

        // Make sure pitch/linear size is correct
        match self
            .size
            .unwrap_or_else(|| self.calculate_pitch_or_linear_size())
        {
            SizeField::Pitch(x) => {
                flags |= HeaderFlags::PITCH;
                pitch_or_linear_size = x;
            }
            SizeField::Linear(x) => {
                flags |= HeaderFlags::LINEAR_SIZE;
                pitch_or_linear_size = x;
            }
        }
        // Check for 3D Texture
        if self.depth.is_some()
            || self.caps2.contains(Caps2Flags::VOLUME)
            || dimension == ResourceDimension::Texture3D
        {
            flags |= HeaderFlags::DEPTH;
            caps2 |= Caps2Flags::VOLUME;
            dimension = ResourceDimension::Texture3D;
            depth = std::cmp::max(depth, 1);
        }
        // Check for mips
        if self.mipmap_count.is_some() {
            flags |= HeaderFlags::MIPMAP_COUNT;
            caps |= CapsFlags::MIPMAP | CapsFlags::COMPLEX;
        }
        if self.caps2.contains(Caps2Flags::CUBEMAP) {
            caps |= CapsFlags::COMPLEX;
            if is_extended_header {
                misc |= MiscFlags::TEXTURE_CUBE;
            }
        }
        if self.four_cc_bytes.is_some() {
            pixel_format |= PixelFormatFlags::FOURCC;
        }

        writer.write_all(b"DDS ")?;
        writer.write_u32::<E>(DdsHeader::BYTE_SIZE as u32)?;

        writer.write_u32::<E>(flags.bits())?;
        writer.write_u32::<E>(self.height)?;
        writer.write_u32::<E>(self.width)?;
        writer.write_u32::<E>(pitch_or_linear_size)?;
        writer.write_u32::<E>(depth)?;
        writer.write_u32::<E>(self.mipmap_count.unwrap_or_default())?;
        let reserved: [u8; 11 * 4] = [0; 11 * 4];
        writer.write_all(&reserved)?;

        writer.write_u32::<E>(DdsHeader::PIXEL_FORMAT_BYTE_SIZE)?;
        writer.write_u32::<E>(pixel_format.bits())?;
        writer.write_u32::<E>(self.rgb_bit_counts)?;
        writer.write_all(&self.four_cc_bytes.unwrap_or_default())?;
        writer.write_u32::<E>(self.rgb_bit_counts)?;
        writer.write_u32::<E>(self.r_bit_mask)?;
        writer.write_u32::<E>(self.g_bit_mask)?;
        writer.write_u32::<E>(self.b_bit_mask)?;
        writer.write_u32::<E>(self.a_bit_mask)?;

        writer.write_u32::<E>(caps.bits())?;
        writer.write_u32::<E>(caps2.bits())?;
        let reserved: [u8; 3 * 4] = [0; 3 * 4];
        writer.write_all(&reserved)?;

        if is_extended_header {
            writer.write_u32::<E>(self.format.map(|x| x as u32).unwrap_or_default())?;
            writer.write_u32::<E>(dimension as u32)?;
            writer.write_u32::<E>(misc.bits())?;
            writer.write_u32::<E>(self.array_size.unwrap_or_default())?;
            writer.write_u32::<E>(self.misc2.map(|x| x.bits()).unwrap_or_default())?;
        }

        Ok(())
    }

    #[inline]
    pub fn write_to<W: Write>(&self, writer: W) -> Result<()> {
        self.write_to_byte_order::<LittleEndian, _>(writer)
    }
}

impl Format {}
