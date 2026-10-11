//! 🧬️ Exact owned TIFF sample words and typed directory metadata.

use crate::STDIO_TIFF_DOCUMENT_SCHEMA;
use framework_schema::ArtifactSchema;

#[derive(Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub enum TiffFieldType { Byte, Ascii, Short, Long, Rational, SByte, Undefined, SShort, SLong, SRational, Float, Double }

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct TiffBinary32 { pub bits: u32 }

/// 🔢️ Exact intrinsic 64-bit identity shared by samples and IEEE metadata.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[retained_clone(bitwise)]
pub struct TiffWord64 { pub lo: u32, pub hi: u32 }
impl TiffWord64 {
    pub const fn from_word(word: u64) -> Self { Self { lo: word as u32, hi: (word >> 32) as u32 } }
    pub const fn word(self) -> u64 { self.lo as u64 | (self.hi as u64) << 32 }
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(tag = "kind", content = "value", rename_all = "camelCase", deny_unknown_fields)]
pub enum TiffValues {
    Byte(Vec<u8>), Ascii(Vec<String>), Short(Vec<u16>), Long(Vec<u32>), Rational(Vec<(u32, u32)>),
    SByte(Vec<i8>), Undefined(Vec<u8>), SShort(Vec<i16>), SLong(Vec<i32>), SRational(Vec<(i32, i32)>),
    Float(Vec<TiffBinary32>), Double(Vec<TiffWord64>),
}
impl TiffValues {
    pub fn kind(&self) -> TiffFieldType {
        match self { Self::Byte(_) => TiffFieldType::Byte, Self::Ascii(_) => TiffFieldType::Ascii, Self::Short(_) => TiffFieldType::Short, Self::Long(_) => TiffFieldType::Long, Self::Rational(_) => TiffFieldType::Rational, Self::SByte(_) => TiffFieldType::SByte, Self::Undefined(_) => TiffFieldType::Undefined, Self::SShort(_) => TiffFieldType::SShort, Self::SLong(_) => TiffFieldType::SLong, Self::SRational(_) => TiffFieldType::SRational, Self::Float(_) => TiffFieldType::Float, Self::Double(_) => TiffFieldType::Double }
    }
    pub fn count(&self) -> u32 {
        (match self { Self::Byte(v) => v.len(), Self::Ascii(v) => v.len(), Self::Short(v) => v.len(), Self::Long(v) => v.len(), Self::Rational(v) => v.len(), Self::SByte(v) => v.len(), Self::Undefined(v) => v.len(), Self::SShort(v) => v.len(), Self::SLong(v) => v.len(), Self::SRational(v) => v.len(), Self::Float(v) => v.len(), Self::Double(v) => v.len() }) as u32
    }
    pub fn first_u32(&self) -> Option<u32> {
        match self { Self::Byte(v) => v.first().map(|n| u32::from(*n)), Self::Short(v) => v.first().map(|n| u32::from(*n)), Self::Long(v) => v.first().copied(), Self::SByte(v) => v.first().map(|n| *n as u32), Self::SShort(v) => v.first().map(|n| *n as u32), Self::SLong(v) => v.first().map(|n| *n as u32), _ => None }
    }
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct TiffTag { pub tag: u16, pub values: TiffValues }

/// 🧱️ Logical image rectangle with pixel-major channel samples independent of native packing.
#[derive(Clone, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct TiffSampleBlock {
    pub x: u32, pub y: u32, pub width: u32, pub height: u32, pub channels: u16,
    pub samples: Vec<TiffWord64>,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct TiffIfd {
    pub entries: Vec<TiffTag>,
    pub blocks: Vec<TiffSampleBlock>,
}

pub const TAG_IMAGE_WIDTH: u16 = 256;
pub const TAG_IMAGE_LENGTH: u16 = 257;
pub const TAG_BITS_PER_SAMPLE: u16 = 258;
pub const TAG_PHOTOMETRIC: u16 = 262;
pub const TAG_SAMPLES_PER_PIXEL: u16 = 277;

impl TiffIfd {
    pub fn tag(&self, tag: u16) -> Option<&TiffValues> { self.entries.iter().find(|entry| entry.tag == tag).map(|entry| &entry.values) }
    pub fn integer(&self, tag: u16) -> Option<u32> { self.tag(tag).and_then(TiffValues::first_u32) }
    pub fn integers(&self, tag: u16) -> Vec<u32> { match self.tag(tag) { Some(TiffValues::Byte(v)) => v.iter().map(|n| u32::from(*n)).collect(), Some(TiffValues::Short(v)) => v.iter().map(|n| u32::from(*n)).collect(), Some(TiffValues::Long(v)) => v.clone(), _ => Vec::new() } }
    pub fn validate(&self) -> Result<(), String> {
        if self.entries.windows(2).any(|pair| pair[0].tag >= pair[1].tag) { return Err("tiff: tags must have unique ascending semantic identities".into()); }
        for entry in &self.entries {
            if matches!(entry.tag, 259 | 266 | 273 | 278 | 279 | 284 | 317 | 322 | 323 | 324 | 325 | 292 | 293 | 347 | 512 | 513 | 514 | 515 | 517 | 518 | 519 | 520 | 521 | 530) { return Err("tiff: native storage policy cannot be a semantic tag".into()); }
            if let TiffValues::Ascii(texts) = &entry.values { if texts.iter().any(|text| !text.is_ascii() || text.contains('\0')) { return Err("tiff: ASCII metadata needs owned text without native terminators".into()); } }
        }
        if self.blocks.is_empty() { return Ok(()); }
        if self.blocks.len() != 1 { return Err("tiff: one logical image block owns each page independently of native strips and tiles".into()); }
        let width = self.integer(TAG_IMAGE_WIDTH).ok_or("tiff: sample blocks require image width")?;
        let height = self.integer(TAG_IMAGE_LENGTH).ok_or("tiff: sample blocks require image height")?;
        let channels = self.integer(TAG_SAMPLES_PER_PIXEL).unwrap_or(1);
        let mut bits = self.integers(TAG_BITS_PER_SAMPLE);
        if bits.is_empty() { bits.push(1); }
        if bits.len() == 1 { bits.resize(channels as usize, bits[0]); }
        let mut formats = self.integers(339);
        if formats.is_empty() { formats.push(1); }
        if formats.len() == 1 { formats.resize(channels as usize, formats[0]); }
        if width == 0 || height == 0 || channels == 0 || channels > u16::MAX as u32 || bits.len() != channels as usize || bits.iter().any(|depth| *depth == 0 || *depth > 64) || formats.len() != channels as usize || formats.iter().zip(&bits).any(|(format, bits)| !matches!(format, 1 | 2 | 3 | 4) || *format == 3 && !matches!(bits, 16 | 32 | 64)) { return Err("tiff: invalid precise sample interpretation".into()); }
        let expected = u64::from(width).checked_mul(u64::from(height)).and_then(|n| n.checked_mul(u64::from(channels))).ok_or("tiff: semantic extent overflow")?;
        let mut actual = 0u64;
        for (index, block) in self.blocks.iter().enumerate() {
            if block.x != 0 || block.y != 0 || block.width != width || block.height != height || block.width == 0 || block.height == 0 || u32::from(block.channels) != channels || block.x.checked_add(block.width).is_none_or(|end| end > width) || block.y.checked_add(block.height).is_none_or(|end| end > height) { return Err("tiff: sample block lies outside its image".into()); }
            let count = u64::from(block.width).checked_mul(u64::from(block.height)).and_then(|n| n.checked_mul(u64::from(channels))).ok_or("tiff: block extent overflow")?;
            if count != block.samples.len() as u64 { return Err("tiff: sample block cardinality differs from its logical extent".into()); }
            actual = actual.checked_add(count).ok_or("tiff: sample cardinality overflow")?;
            for (lane, sample) in block.samples.iter().enumerate() { let depth = bits[lane % channels as usize]; if depth < 64 && sample.word() >> depth != 0 { return Err("tiff: sample exceeds declared native precision".into()); } }
            for other in &self.blocks[..index] { if block.x < other.x + other.width && other.x < block.x + block.width && block.y < other.y + other.height && other.y < block.y + block.height { return Err("tiff: logical sample blocks overlap".into()); } }
        }
        if actual != expected { return Err("tiff: sample blocks do not cover every owned pixel".into()); }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[artifact_schema(id = "s.stdio.tiff")]
pub struct TiffSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub ifds: Vec<TiffIfd>,
}
impl Default for TiffSnapshot { fn default() -> Self { Self { schema: STDIO_TIFF_DOCUMENT_SCHEMA.into(), ifds: Vec::new() } } }
impl TiffSnapshot {
    pub fn tag(&self, tag: u16) -> Option<&TiffTag> { self.ifds.first()?.entries.iter().find(|entry| entry.tag == tag) }
    pub fn width(&self) -> Option<u32> { self.tag(TAG_IMAGE_WIDTH).and_then(|tag| tag.values.first_u32()) }
    pub fn height(&self) -> Option<u32> { self.tag(TAG_IMAGE_LENGTH).and_then(|tag| tag.values.first_u32()) }
    pub fn validate(&self) -> Result<(), String> { if self.schema != STDIO_TIFF_DOCUMENT_SCHEMA { return Err("tiff: undeclared semantic schema".into()); } for ifd in &self.ifds { ifd.validate()?; } Ok(()) }
}
