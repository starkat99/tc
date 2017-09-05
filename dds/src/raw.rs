use std::mem;
use std::io::{Read, Write, Cursor};
use num_traits::{FromPrimitive, ToPrimitive};
use bytes::{LittleEndian, BigEndian, ByteOrder, IntoBuf, Buf, BufMut};

error_chain! {
    foreign_links {
        Io(::std::io::Error);
    }

    errors {
        InvalidHeader {
            description("data is not a valid DDS")
        }
    }
}

mod header_flags {
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
            const TEXTURE = CAPS.bits
                          | HEIGHT.bits
                          | WIDTH.bits
                          | PIXEL_FORMAT.bits;
        }
    }
}

mod caps {
    bitflags! {
        pub struct CapsFlags: u32 {
            const COMPLEX = 0x8;
            const MIPMAP = 0x400000;
            const TEXTURE = 0x1000;
        }
    }
}

mod caps2 {
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
            const CUBEMAP_ALLFACES = CUBEMAP.bits
                                   | CUBEMAP_POSITIVE_X.bits
                                   | CUBEMAP_NEGATIVE_X.bits
                                   | CUBEMAP_POSITIVE_Y.bits
                                   | CUBEMAP_NEGATIVE_Y.bits
                                   | CUBEMAP_POSITIVE_Z.bits
                                   | CUBEMAP_NEGATIVE_Z.bits;
        }
    }
}

mod pixel_format {
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
}

mod misc {
    bitflags! {
        pub struct MiscFlags: u32 {
            const TEXTURE_CUBE = 0x4;
        }
    }
}

mod misc2 {
    bitflags! {
        pub struct Misc2Flags: u32 {
            const ALPHA_MODE_UNKNOWN = 0x0;
            const ALPHA_MODE_STRAIGHT = 0x1;
            const ALPHA_MODE_PREMULTIPLIED = 0x2;
            const ALPHA_MODE_OPAQUE = 0x3;
            const ALPHA_MODE_CUSTOM = 0x4;
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Primitive)]
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

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Primitive)]
#[repr(u32)]
enum ResourceDimension {
    Texture1D = 2,
    Texture2D = 3,
    Texture3D = 4,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Endianness {
    Little,
    Big,
}

#[derive(Debug, Clone)]
pub struct DdsHeader {
    flags: header_flags::HeaderFlags,
    height: u32,
    width: u32,
    pitch_or_linear_size: u32,
    depth: u32,
    mipmap_count: u32,
    pixel_format: pixel_format::PixelFormatFlags,
    four_cc: [u8; 4],
    rgb_bit_counts: u32,
    r_bit_mask: u32,
    g_bit_mask: u32,
    b_bit_mask: u32,
    a_bit_mask: u32,
    caps: caps::CapsFlags,
    caps2: caps2::Caps2Flags,
    format: Option<Format>,
    dimension: Option<ResourceDimension>,
    misc: Option<misc::MiscFlags>,
    array_size: Option<u32>,
    misc2: Option<misc2::Misc2Flags>,
}

impl DdsHeader {
    pub const BYTE_SIZE: usize = 128;
    const PIXEL_FORMAT_BYTE_SIZE: u32 = 32;
    const EXTENDED_BYTE_SIZE: usize = 20;

    pub fn from_bytes<E: ByteOrder, T: IntoBuf>(bytes: T) -> Result<DdsHeader> {
        let mut buf = bytes.into_buf();
        ensure!(
            buf.remaining() >= DdsHeader::BYTE_SIZE,
            ErrorKind::InvalidHeader
        );
        ensure!(&buf.bytes()[..4] == b"DDS ", ErrorKind::InvalidHeader);
        buf.advance(4);
        let size = buf.get_u32::<E>();
        ensure!(
            size as usize == DdsHeader::BYTE_SIZE - 4,
            ErrorKind::InvalidHeader
        );
        let flags = header_flags::HeaderFlags::from_bits_truncate(buf.get_u32::<E>());
        let height = buf.get_u32::<E>();
        let width = buf.get_u32::<E>();
        let pitch_or_linear_size = buf.get_u32::<E>();
        let depth = buf.get_u32::<E>();
        let mipmap_count = buf.get_u32::<E>();
        buf.advance(11 * 4); // Reserved bytes

        let size = buf.get_u32::<E>();
        ensure!(
            size == DdsHeader::PIXEL_FORMAT_BYTE_SIZE,
            ErrorKind::InvalidHeader
        );
        let pixel_format = pixel_format::PixelFormatFlags::from_bits_truncate(buf.get_u32::<E>());

        let mut four_cc: [u8; 4] = [0, 0, 0, 0];
        four_cc.copy_from_slice(&buf.bytes()[..4]);
        let is_extended_header = &four_cc == b"DX10";

        let rgb_bit_counts = buf.get_u32::<E>();
        let r_bit_mask = buf.get_u32::<E>();
        let g_bit_mask = buf.get_u32::<E>();
        let b_bit_mask = buf.get_u32::<E>();
        let a_bit_mask = buf.get_u32::<E>();

        let caps = caps::CapsFlags::from_bits_truncate(buf.get_u32::<E>());
        let caps2 = caps2::Caps2Flags::from_bits_truncate(buf.get_u32::<E>());
        buf.advance(3 * 4); // Unused/reserved bytes

        ensure!(
            !is_extended_header || buf.remaining() >= DdsHeader::EXTENDED_BYTE_SIZE,
            ErrorKind::InvalidHeader
        );
        let format: Option<Format> = if is_extended_header {
            Format::from_u32(buf.get_u32::<E>())
        } else {
            None
        };
        let dimension: Option<ResourceDimension> = if is_extended_header {
             ResourceDimension::from_u32(buf.get_u32::<E>())
        } else {
            None
        };
        let misc: Option<misc::MiscFlags> = if is_extended_header {
            Some(misc::MiscFlags::from_bits_truncate(buf.get_u32::<E>()))
        } else {
            None
        };
        let array_size: Option<u32> = if is_extended_header {
            Some(buf.get_u32::<E>())
        } else {
            None
        };
        let misc2: Option<misc2::Misc2Flags> = if is_extended_header {
            Some(misc2::Misc2Flags::from_bits_truncate(buf.get_u32::<E>()))
        } else {
            None
        };

        Ok(DdsHeader {
            flags,
            height,
            width,
            pitch_or_linear_size,
            depth,
            mipmap_count,
            pixel_format,
            four_cc,
            rgb_bit_counts,
            r_bit_mask,
            g_bit_mask,
            b_bit_mask,
            a_bit_mask,
            caps,
            caps2,
            format,
            dimension,
            misc,
            array_size,
            misc2,
        })
    }

    pub fn from_bytes_with_order<T: IntoBuf>(
        bytes: T,
        byte_order: Endianness,
    ) -> Result<DdsHeader> {
        match byte_order {
            Endianness::Little => DdsHeader::from_bytes::<LittleEndian, T>(bytes),
            Endianness::Big => DdsHeader::from_bytes::<BigEndian, T>(bytes),
        }
    }

    #[inline]
    pub fn from_bytes_default_order<T: IntoBuf>(bytes: T) -> Result<DdsHeader> {
        DdsHeader::from_bytes::<LittleEndian, _>(bytes)
    }

    pub fn read_from<E: ByteOrder, T: Read>(input: &mut T) -> Result<DdsHeader> {
        let mut buf: [u8; DdsHeader::BYTE_SIZE] = unsafe { mem::uninitialized() };
        input.read_exact(&mut buf)?;
        DdsHeader::from_bytes::<E, _>(&buf[..])
    }

    pub fn read_from_with_order<T: Read>(
        input: &mut T,
        byte_order: Endianness,
    ) -> Result<DdsHeader> {
        match byte_order {
            Endianness::Little => DdsHeader::read_from::<LittleEndian, T>(input),
            Endianness::Big => DdsHeader::read_from::<BigEndian, T>(input),
        }
    }

    #[inline]
    pub fn read_from_default_order<T: Read>(input: &mut T) -> Result<DdsHeader> {
        DdsHeader::read_from::<LittleEndian, _>(input)
    }

    pub fn write_to<E: ByteOrder, T: Write>(&self, output: &mut T) -> Result<()> {
        let mut bytes: [u8; DdsHeader::BYTE_SIZE + DdsHeader::EXTENDED_BYTE_SIZE] = unsafe { mem::uninitialized() };
        let len = if &self.four_cc == b"DX10" {bytes.len()} else { DdsHeader::BYTE_SIZE };
        {
            let mut buf = Cursor::new(&mut bytes[..len]);
            self.put_buf::<E, _>(&mut buf);
        }
        Ok(output.write_all(&bytes[..len])?)
    }

    fn put_buf<E: ByteOrder, T: BufMut>(&self, buf: &mut T) {
        buf.put_slice(b"DDS ");
        buf.put_u32::<E>(DdsHeader::BYTE_SIZE as u32);
        buf.put_u32::<E>(self.flags.bits());
        buf.put_u32::<E>(self.height);
        buf.put_u32::<E>(self.width);
        buf.put_u32::<E>(self.pitch_or_linear_size);
        buf.put_u32::<E>(self.depth);
        buf.put_u32::<E>(self.mipmap_count);
        let reserved: [u8; 11 * 4] = unsafe { mem::zeroed() };
        buf.put_slice(&reserved);

        buf.put_u32::<E>(DdsHeader::PIXEL_FORMAT_BYTE_SIZE);
        buf.put_u32::<E>(self.pixel_format.bits());
        buf.put_u32::<E>(self.rgb_bit_counts);
        buf.put_slice(&self.four_cc);
        buf.put_u32::<E>(self.rgb_bit_counts);
        buf.put_u32::<E>(self.r_bit_mask);
        buf.put_u32::<E>(self.g_bit_mask);
        buf.put_u32::<E>(self.b_bit_mask);
        buf.put_u32::<E>(self.a_bit_mask);

        buf.put_u32::<E>(self.caps.bits());
        buf.put_u32::<E>(self.caps2.bits());
        let reserved: [u8; 3 * 4] = unsafe { mem::zeroed() };
        buf.put_slice(&reserved);

        if &self.four_cc == b"DX10" {
            buf.put_u32::<E>(self.format.and_then(|x| x.to_u32()).unwrap_or_default());
            buf.put_u32::<E>(self.dimension.and_then(|x| x.to_u32()).unwrap_or_default());
            buf.put_u32::<E>(self.misc.map(|x| x.bits()).unwrap_or_default());
            buf.put_u32::<E>(self.array_size.unwrap_or_default());
            buf.put_u32::<E>(self.misc2.map(|x| x.bits()).unwrap_or_default());
        }
    }
}
