//! 🧬️ Precise native PNG samples and owned image metadata.

use crate::STDIO_PNG_DOCUMENT_SCHEMA;
use framework_schema::ArtifactSchema;

#[derive(semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, Default, semio_framework_dsl_record_derive::DslScalar)]
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

#[derive(semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, Default, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct PngRgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslEnum)]
#[value(tag = "colorType", rename_all = "camelCase")]
pub enum PngTransparency {
    Indexed { alpha: Vec<u8> },
    Grayscale { gray: u16 },
    Rgb { r: u16, g: u16, b: u16 },
}

#[derive(semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, Default, semio_framework_dsl_record_derive::DslRecord)]
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

#[derive(semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, Default, semio_framework_dsl_record_derive::DslScalar)]
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

#[derive(semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, Default, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct PngPhysicalDims {
    pub ppu_x: u32,
    pub ppu_y: u32,
    pub unit_is_meter: bool,
}

#[derive(semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, Default, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct PngTimestamp {
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
}

#[derive(semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslEnum)]
#[value(tag = "colorType", rename_all = "camelCase")]
pub enum PngBackground {
    Grayscale { gray: u16 },
    Rgb { r: u16, g: u16, b: u16 },
    Indexed { index: u8 },
}

#[derive(semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, Default, semio_framework_dsl_record_derive::DslScalar)]
#[value(rename_all = "camelCase")]
pub enum PngTextKind {
    #[default]
    Text,
    ZText,
    IText,
}

#[derive(semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, Default, semio_framework_dsl_record_derive::DslRecord)]
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

#[derive(semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct PngRegion {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

#[derive(semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslScalar)]
#[value(rename_all = "kebab-case")]
pub enum PngNativeProfile {
    Indexed,
    Grayscale,
    GrayscaleAlpha,
    Rgb,
    Rgba,
}

#[derive(semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct PngNativePaint {
    pub profile: PngNativeProfile,
    pub first: u16,
    pub second: u16,
    pub third: u16,
    pub fourth: u16,
}

impl PngNativePaint {
    pub const fn indexed(index: u16) -> Self { Self { profile: PngNativeProfile::Indexed, first: index, second: 0, third: 0, fourth: 0 } }
    pub const fn grayscale(gray: u16) -> Self { Self { profile: PngNativeProfile::Grayscale, first: gray, second: 0, third: 0, fourth: 0 } }
    pub const fn grayscale_alpha(gray: u16, alpha: u16) -> Self { Self { profile: PngNativeProfile::GrayscaleAlpha, first: gray, second: alpha, third: 0, fourth: 0 } }
    pub const fn rgb(red: u16, green: u16, blue: u16) -> Self { Self { profile: PngNativeProfile::Rgb, first: red, second: green, third: blue, fourth: 0 } }
    pub const fn rgba(red: u16, green: u16, blue: u16, alpha: u16) -> Self { Self { profile: PngNativeProfile::Rgba, first: red, second: green, third: blue, fourth: alpha } }

    pub(crate) fn samples(self) -> [u16; 4] { [self.first, self.second, self.third, self.fourth] }
}


#[derive(semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct PngAncillaryChunk { pub kind: [u8; 4], pub data: Vec<u8>, pub after_raster: bool }

/// 🌗️ The gamma chunk value a diff sets: `None` removes the chunk.
#[derive(semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct PngGammaValue {
    pub gama: Option<u32>,
}

/// 🧩 One rectangle of native samples, row-major, `samples_per_pixel` values per pixel: the sparse row a paint writes and its undo restores.
#[derive(semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct PngSampleRect {
    pub region: PngRegion,
    pub samples: Vec<u16>,
}

#[derive(semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct PngImage {
    pub width: u32,
    pub height: u32,
    pub bit_depth: u8,
    pub color_type: PngColorType,
    pub interlace: bool,
    pub samples: Vec<u16>,
    pub palette: Option<Vec<PngRgb>>,
    #[dsl(statements, block)]
    pub transparency: Option<PngTransparency>,
    pub gamma: Option<u32>,
    pub chromaticities: Option<PngChromaticities>,
    pub srgb: Option<PngSrgbIntent>,
    pub physical_dims: Option<PngPhysicalDims>,
    pub timestamp: Option<PngTimestamp>,
    #[dsl(statements, block)]
    pub background: Option<PngBackground>,
    pub text_chunks: Vec<PngTextChunk>,
    pub ancillary_chunks: Vec<PngAncillaryChunk>,
}

impl Default for PngImage {
    fn default() -> Self { Self { width: 1, height: 1, bit_depth: 8, color_type: PngColorType::Rgba, interlace: false, samples: vec![255; 4], palette: None, transparency: None, gamma: None, chromaticities: None, srgb: None, physical_dims: None, timestamp: None, background: None, text_chunks: Vec::new(), ancillary_chunks: Vec::new() } }
}

impl PngImage {
    /// 🧭 Start offset of `region`'s first sample row, or `None` when the region is empty or leaves the image.
    pub fn region_start(&self, region: PngRegion) -> Option<usize> {
        let inside = region.width != 0 && region.height != 0 && region.x.checked_add(region.width).is_some_and(|end| end <= self.width) && region.y.checked_add(region.height).is_some_and(|end| end <= self.height);
        inside.then(|| (region.y as usize * self.width as usize + region.x as usize) * self.color_type.samples_per_pixel())
    }

    /// 📋 The samples `region` currently holds, row-major (`None` when the region is outside the image).
    pub fn region_samples(&self, region: PngRegion) -> Option<Vec<u16>> {
        let start = self.region_start(region)?;
        let spp = self.color_type.samples_per_pixel();
        let row = region.width as usize * spp;
        let stride = self.width as usize * spp;
        Some((0..region.height as usize).flat_map(|line| self.samples.get(start + line * stride..start + line * stride + row).unwrap_or_default().iter().copied()).collect())
    }

    /// ✍️ Writes `rect` into the image rows it addresses and nothing else.
    pub fn write_rect(&mut self, rect: &PngSampleRect) -> Result<(), String> {
        let start = self.region_start(rect.region).ok_or("png: sample rectangle exceeds the owned image or is empty")?;
        let spp = self.color_type.samples_per_pixel();
        let row = rect.region.width as usize * spp;
        let stride = self.width as usize * spp;
        if rect.samples.len() != row * rect.region.height as usize {
            return Err("png: sample rectangle cardinality differs from its region".into());
        }
        rect.samples.chunks_exact(row).enumerate().for_each(|(line, values)| self.samples[start + line * stride..start + line * stride + row].copy_from_slice(values));
        Ok(())
    }

    /// 🔎 Refuses a rectangle that is out of bounds, mis-sized, over the sample precision or past the palette.
    pub fn validate_rect(&self, rect: &PngSampleRect) -> Result<(), String> {
        self.validate_region_samples(rect.region, &rect.samples)
    }

    /// 🔎 [`PngImage::validate_rect`] over borrowed samples.
    pub fn validate_region_samples(&self, region: PngRegion, samples: &[u16]) -> Result<(), String> {
        self.validate_header()?;
        self.region_start(region).ok_or("png: sample rectangle exceeds the owned image or is empty")?;
        if samples.len() != region.width as usize * region.height as usize * self.color_type.samples_per_pixel() {
            return Err("png: sample rectangle cardinality differs from its region".into());
        }
        let maximum = if self.bit_depth == 16 { u16::MAX } else { (1u16 << self.bit_depth) - 1 };
        let palette = self.palette.as_ref().map_or(0, Vec::len);
        if samples.iter().any(|sample| *sample > maximum || self.color_type == PngColorType::Palette && usize::from(*sample) >= palette) {
            return Err("png: sample rectangle exceeds the sample precision or palette".into());
        }
        Ok(())
    }

    pub fn same_metadata(&self,other:&Self)->bool {self.width==other.width&&self.height==other.height&&self.bit_depth==other.bit_depth&&self.color_type==other.color_type&&self.interlace==other.interlace&&self.palette==other.palette&&self.transparency==other.transparency&&self.gamma==other.gamma&&self.chromaticities==other.chromaticities&&self.srgb==other.srgb&&self.physical_dims==other.physical_dims&&self.timestamp==other.timestamp&&self.background==other.background&&self.text_chunks==other.text_chunks&&self.ancillary_chunks==other.ancillary_chunks}

    pub fn validate_header(&self) -> Result<(), String> {
        if self.width == 0 || self.height == 0 { return Err("png: owned dimensions must be positive".into()); }
        let valid_depth = match self.color_type { PngColorType::Grayscale => matches!(self.bit_depth,1|2|4|8|16), PngColorType::Palette => matches!(self.bit_depth,1|2|4|8), _ => matches!(self.bit_depth,8|16) };
        if !valid_depth { return Err("png: owned color profile has an invalid sample depth".into()); }
        let count = (self.width as usize).checked_mul(self.height as usize).and_then(|n|n.checked_mul(self.color_type.samples_per_pixel())).ok_or("png: owned sample cardinality overflow")?;
        let maximum = if self.bit_depth == 16 { u16::MAX } else { (1u16 << self.bit_depth) - 1 };
        if self.samples.len() != count { return Err("png: native sample precision or cardinality differs from the owned profile".into()); }
        if let Some(palette) = &self.palette { if palette.is_empty() || palette.len() > 256 || matches!(self.color_type,PngColorType::Grayscale|PngColorType::GrayscaleAlpha) { return Err("png: invalid owned palette".into()); } }
        if self.color_type == PngColorType::Palette { let palette = self.palette.as_ref().ok_or("png: indexed samples require an owned palette")?; if palette.len() > usize::from(maximum) + 1 { return Err("png: indexed sample refers to an absent palette entry".into()); } }
        if let Some(transparency) = &self.transparency { match transparency {
            PngTransparency::Indexed { alpha } if self.color_type == PngColorType::Palette && !alpha.is_empty() && alpha.len() <= self.palette.as_ref().map_or(0,Vec::len) => {},
            PngTransparency::Grayscale { gray } if self.color_type == PngColorType::Grayscale && *gray <= maximum => {},
            PngTransparency::Rgb { r,g,b } if self.color_type == PngColorType::Rgb && [r,g,b].into_iter().all(|v|*v <= maximum) => {},
            _ => return Err("png: transparency differs from the owned color profile".into()),
        } }
        if let Some(background) = &self.background { match background {
            PngBackground::Indexed { index } if self.color_type == PngColorType::Palette && (*index as usize) < self.palette.as_ref().map_or(0,Vec::len) => {},
            PngBackground::Grayscale { gray } if matches!(self.color_type,PngColorType::Grayscale|PngColorType::GrayscaleAlpha) && *gray <= maximum => {},
            PngBackground::Rgb { r,g,b } if matches!(self.color_type,PngColorType::Rgb|PngColorType::Rgba) && [r,g,b].into_iter().all(|v|*v <= maximum) => {},
            _ => return Err("png: background differs from the owned color profile".into()),
        } }
        if self.gamma == Some(0) { return Err("png: gamma must be positive".into()); }
        if self.timestamp.is_some_and(|time| !(1..=12).contains(&time.month) || !(1..=31).contains(&time.day) || time.hour > 23 || time.minute > 59 || time.second > 60) { return Err("png: timestamp is outside the owned field ranges".into()); }
        Ok(())
    }
    pub fn validate(&self)->Result<(),String> {
        self.validate_header()?;
        let maximum=if self.bit_depth==16 {u16::MAX}else{(1u16<<self.bit_depth)-1};
        if self.samples.iter().any(|sample|*sample>maximum||self.color_type==PngColorType::Palette&&usize::from(*sample)>=self.palette.as_ref().map_or(0,Vec::len)) {return Err("png: native sample precision or palette identity differs from the owned profile".into());}
        for text in &self.text_chunks { if text.keyword.is_empty() || text.keyword.chars().count() > 79 || text.keyword.chars().any(|c|c == '\0' || c as u32 > 255) || text.kind == PngTextKind::Text && text.compressed || text.kind == PngTextKind::ZText && !text.compressed { return Err("png: text metadata differs from its native text profile".into()); } if text.kind != PngTextKind::IText && (text.value.chars().any(|c|c as u32 > 255) || !text.language_tag.is_empty() || !text.translated_keyword.is_empty()) { return Err("png: Latin-1 text metadata contains an international-only field".into()); } if !text.language_tag.is_ascii() { return Err("png: international text language tag must be ASCII".into()); } }
        let mut after = false;
        for chunk in &self.ancillary_chunks { if !chunk.kind.iter().all(u8::is_ascii_alphabetic) || !chunk.kind[0].is_ascii_lowercase() || !chunk.kind[2].is_ascii_uppercase() || [*b"tRNS",*b"gAMA",*b"cHRM",*b"sRGB",*b"pHYs",*b"tIME",*b"bKGD",*b"tEXt",*b"zTXt",*b"iTXt"].contains(&chunk.kind) || after && !chunk.after_raster { return Err("png: opaque ancillary metadata overlaps a typed field or has invalid placement".into()); } after |= chunk.after_raster; }
        Ok(())
    }
}

#[derive(semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[artifact_schema(id = "s.stdio.png")]
pub struct PngSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[dsl(block)]
    pub image: PngImage,
}

impl Default for PngSnapshot { fn default() -> Self { Self { schema: STDIO_PNG_DOCUMENT_SCHEMA.into(), image: PngImage::default() } } }
impl PngSnapshot { pub fn validate(&self) -> Result<(), String> { if self.schema != STDIO_PNG_DOCUMENT_SCHEMA { return Err("png: undeclared semantic schema".into()); } self.image.validate() } }

#[cfg(test)]
#[path="🧪️tests/🪆️owner/🦀️.rs"]
mod logical_owner_tests;

#[cfg(test)]
#[path="🧪️tests/🧬️owned-native-contract/🦀️.rs"]
mod owned_native_contract_tests;
