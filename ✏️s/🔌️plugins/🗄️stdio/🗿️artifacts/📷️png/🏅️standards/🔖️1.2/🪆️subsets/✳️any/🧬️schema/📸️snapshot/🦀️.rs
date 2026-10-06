//! 🧬️ Exact PNG byte authority and checked projection value types.

use crate::STDIO_PNG_DOCUMENT_SCHEMA;
use framework_schema::ArtifactSchema;

#[derive(Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, Default, semio_framework_dsl_record_derive::DslScalar)]
#[value(rename_all = "camelCase")]
pub enum PngColorType {
    Grayscale,
    Rgb,
    Palette,
    GrayscaleAlpha,
    #[default]
    Rgba,
}

impl PngColorType {
    pub fn from_u8(value: u8) -> Result<Self, String> {
        match value {
            0 => Ok(Self::Grayscale),
            2 => Ok(Self::Rgb),
            3 => Ok(Self::Palette),
            4 => Ok(Self::GrayscaleAlpha),
            6 => Ok(Self::Rgba),
            _ => Err(format!("png: unsupported color type {value}")),
        }
    }

    pub const fn to_u8(self) -> u8 {
        match self {
            Self::Grayscale => 0,
            Self::Rgb => 2,
            Self::Palette => 3,
            Self::GrayscaleAlpha => 4,
            Self::Rgba => 6,
        }
    }

    pub const fn samples_per_pixel(self) -> usize {
        match self {
            Self::Grayscale | Self::Palette => 1,
            Self::Rgb => 3,
            Self::GrayscaleAlpha => 2,
            Self::Rgba => 4,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, Default)]
#[value(rename_all = "camelCase")]
pub struct PngRgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "colorType", rename_all = "camelCase")]
pub enum PngTransparency {
    Indexed { alpha: Vec<u8> },
    Grayscale { gray: u16 },
    Rgb { r: u16, g: u16, b: u16 },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, Default)]
#[value(rename_all = "camelCase")]
pub struct PngChromaticities {
    pub white_x: u32,
    pub white_y: u32,
    pub red_x: u32,
    pub red_y: u32,
    pub green_x: u32,
    pub green_y: u32,
    pub blue_x: u32,
    pub blue_y: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, Default)]
#[value(rename_all = "camelCase")]
pub enum PngSrgbIntent {
    #[default]
    Perceptual,
    RelativeColorimetric,
    Saturation,
    AbsoluteColorimetric,
}

impl PngSrgbIntent {
    pub fn from_u8(value: u8) -> Result<Self, String> {
        match value {
            0 => Ok(Self::Perceptual),
            1 => Ok(Self::RelativeColorimetric),
            2 => Ok(Self::Saturation),
            3 => Ok(Self::AbsoluteColorimetric),
            _ => Err(format!("png sRGB: unsupported rendering intent {value}")),
        }
    }

    pub const fn to_u8(self) -> u8 {
        match self {
            Self::Perceptual => 0,
            Self::RelativeColorimetric => 1,
            Self::Saturation => 2,
            Self::AbsoluteColorimetric => 3,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, Default)]
#[value(rename_all = "camelCase")]
pub struct PngPhysicalDims {
    pub ppu_x: u32,
    pub ppu_y: u32,
    pub unit_is_meter: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, Default)]
#[value(rename_all = "camelCase")]
pub struct PngTimestamp {
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "colorType", rename_all = "camelCase")]
pub enum PngBackground {
    Grayscale { gray: u16 },
    Rgb { r: u16, g: u16, b: u16 },
    Indexed { index: u8 },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, Default)]
#[value(rename_all = "camelCase")]
pub enum PngTextKind {
    #[default]
    Text,
    ZText,
    IText,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, Default)]
#[value(rename_all = "camelCase")]
pub struct PngTextChunk {
    pub keyword: String,
    pub value: String,
    #[value(default)]
    pub compressed: bool,
    #[value(default)]
    pub kind: PngTextKind,
    #[value(default)]
    pub language_tag: String,
    #[value(default)]
    pub translated_keyword: String,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PngChunk {
    pub kind: [u8; 4],
    pub data: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "chunk", rename_all = "camelCase")]
pub enum PngChunkMarker {
    Ihdr,
    Plte,
    Trns,
    Gama,
    Chrm,
    Srgb,
    Phys,
    Time,
    Bkgd,
    Idat,
    Iend,
    Text { index: usize },
    Unknown { index: usize },
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.png")]
pub struct PngSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[dsl(base64)]
    pub bytes: Vec<u8>,
}

impl Default for PngSnapshot {
    fn default() -> Self {
        Self { schema: STDIO_PNG_DOCUMENT_SCHEMA.into(), bytes: crate::standards::v1_2::subsets::any::io::empty_png_bytes() }
    }
}






#[cfg(test)]
#[path="🧪️tests/🪆️owner/🦀️.rs"]
mod logical_owner_tests;
