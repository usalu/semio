//! 🧬️ TiffSnapshot schema — complete TIFF 6.0 semantic model. Ticket
//! 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION: replaces the old
//! shared `RasterImage{width,height,rgba}` stub with TIFF's REAL generic tag/type/value model:
//! `byte_order` + index-keyed `ifds: Vec<TiffIfd>`, each holding tag-id-keyed `TiffTag{tag,
//! kind, values}` entries. `TiffFieldType`/`TiffValues` cover all 12 TIFF 6.0 field types —
//! "unknown tags" are simply tags the codec doesn't specially interpret, but whose typed
//! VALUE is still stored losslessly via this same triple (the tag/type/value model IS the
//! raw-retention mechanism; no separate unknown-tag fallback is needed). Every IFD owns its exact
//! strip or tile chunks in `storage`; decoded RGBA pixels are bounded, ephemeral projections and
//! never become a second authored authority.

use crate::STDIO_TIFF_DOCUMENT_SCHEMA;
use framework_schema::ArtifactSchema;

#[path = "🪶️sqlite/🦀️.rs"]
pub mod sqlite;

#[cfg(test)]
#[path = "🧪️tests/🪶️sqlite/🦀️.rs"]
mod sqlite_tests;

//#region ByteOrder
/// 🧭️ TIFF6 §2 byte-order mark (`II` little-endian / `MM` big-endian) — governs every
/// multi-byte field in the file, including every IFD entry's `count`/inline value bytes.
#[derive(Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, Default)]
#[value(rename_all = "camelCase")]
pub enum TiffByteOrder {
    #[default]
    LittleEndian,
    BigEndian,
}
//#endregion ByteOrder

//#region FieldType
/// 🏷️ TIFF6 §2 Table 2 — the 12 real IFD entry field types. Type code 13 (`IFD`) is a later
/// extension outside the 6.0 core table and is deliberately NOT modeled (decode errors
/// honestly rather than fabricating a 13th variant this standard doesn't claim).
#[derive(Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub enum TiffFieldType {
    Byte,
    Ascii,
    Short,
    Long,
    Rational,
    SByte,
    Undefined,
    SShort,
    SLong,
    SRational,
    Float,
    Double,
}

impl TiffFieldType {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn from_u16(v: u16) -> Result<Self, String> {
        match v {
            1 => Ok(Self::Byte),
            2 => Ok(Self::Ascii),
            3 => Ok(Self::Short),
            4 => Ok(Self::Long),
            5 => Ok(Self::Rational),
            6 => Ok(Self::SByte),
            7 => Ok(Self::Undefined),
            8 => Ok(Self::SShort),
            9 => Ok(Self::SLong),
            10 => Ok(Self::SRational),
            11 => Ok(Self::Float),
            12 => Ok(Self::Double),
            other => Err(format!("tiff: unrecognized field type code {other} (TIFF 6.0 core table is 1-12)")),
        }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn to_u16(self) -> u16 {
        match self {
            Self::Byte => 1,
            Self::Ascii => 2,
            Self::Short => 3,
            Self::Long => 4,
            Self::Rational => 5,
            Self::SByte => 6,
            Self::Undefined => 7,
            Self::SShort => 8,
            Self::SLong => 9,
            Self::SRational => 10,
            Self::Float => 11,
            Self::Double => 12,
        }
    }
    /// 📏️ Byte size of ONE value of this type (TIFF6 §2 Table 2) — drives the inline-vs-offset
    /// rule (`element_size * count <= 4` stays inline in the entry's value field).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn element_size(self) -> usize {
        match self {
            Self::Byte | Self::Ascii | Self::SByte | Self::Undefined => 1,
            Self::Short | Self::SShort => 2,
            Self::Long | Self::SLong | Self::Float => 4,
            Self::Rational | Self::SRational | Self::Double => 8,
        }
    }
}
//#endregion FieldType

//#region Values
/// 📦️ Typed union over every TIFF 6.0 field type's decoded value — the tag/type/value TRIPLE
/// (with [`TiffTag::tag`]/[`TiffTag::kind`]) is what makes an "unknown" tag still losslessly
/// retained: the codec doesn't need to specially interpret a tag id to store its real typed
/// value honestly. Adjacently tagged (`kind`/`value`) rather than internally tagged so these
/// newtype variants (all of which wrap arrays/strings, not structs) serialize cleanly — same
/// pattern as `ply`'s `PlyValue`.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "kind", content = "value", rename_all = "camelCase")]
pub enum TiffValues {
    Byte(Vec<u8>),
    Ascii(Vec<u8>),
    Short(Vec<u16>),
    Long(Vec<u32>),
    Rational(Vec<(u32, u32)>),
    SByte(Vec<i8>),
    Undefined(Vec<u8>),
    SShort(Vec<i16>),
    SLong(Vec<i32>),
    SRational(Vec<(i32, i32)>),
    Float(Vec<TiffBinary32>),
    Double(Vec<TiffBinary64>),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct TiffBinary32 {
    pub bits: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct TiffBinary64 {
    pub bits: u64,
}

impl TiffValues {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn kind(&self) -> TiffFieldType {
        match self {
            Self::Byte(_) => TiffFieldType::Byte,
            Self::Ascii(_) => TiffFieldType::Ascii,
            Self::Short(_) => TiffFieldType::Short,
            Self::Long(_) => TiffFieldType::Long,
            Self::Rational(_) => TiffFieldType::Rational,
            Self::SByte(_) => TiffFieldType::SByte,
            Self::Undefined(_) => TiffFieldType::Undefined,
            Self::SShort(_) => TiffFieldType::SShort,
            Self::SLong(_) => TiffFieldType::SLong,
            Self::SRational(_) => TiffFieldType::SRational,
            Self::Float(_) => TiffFieldType::Float,
            Self::Double(_) => TiffFieldType::Double,
        }
    }
    /// 🔢️ IFD entry `Count` for this value — number of elements of `kind()`, EXCEPT `Ascii`
    /// which counts BYTES including the terminating NUL (TIFF6 §2's own special case).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn count(&self) -> u32 {
        match self {
            Self::Byte(v) => v.len() as u32,
            Self::Ascii(s) => s.len() as u32,
            Self::Short(v) => v.len() as u32,
            Self::Long(v) => v.len() as u32,
            Self::Rational(v) => v.len() as u32,
            Self::SByte(v) => v.len() as u32,
            Self::Undefined(v) => v.len() as u32,
            Self::SShort(v) => v.len() as u32,
            Self::SLong(v) => v.len() as u32,
            Self::SRational(v) => v.len() as u32,
            Self::Float(v) => v.len() as u32,
            Self::Double(v) => v.len() as u32,
        }
    }
    /// 🔍️ First value widened to `u32`, for integer-typed variants only — convenience used by
    /// well-known-tag accessors ([`TiffSnapshot::width`] etc.) and the baseline subset's
    /// conformance checks. `None` for non-integer/empty variants.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn first_u32(&self) -> Option<u32> {
        match self {
            Self::Byte(v) => v.first().map(|&x| x as u32),
            Self::Short(v) => v.first().map(|&x| x as u32),
            Self::Long(v) => v.first().copied(),
            Self::SByte(v) => v.first().map(|&x| x as u32),
            Self::SShort(v) => v.first().map(|&x| x as u32),
            Self::SLong(v) => v.first().map(|&x| x as u32),
            _ => None,
        }
    }
}
//#endregion Values

//#region Tag
/// 🏷️ One IFD entry — a weak value (whole-value replaced in diffs: `kind`/`values` move
/// together atomically, never sub-diffed).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct TiffTag {
    pub tag: u16,
    pub values: TiffValues,
}
//#endregion Tag

//#region Ifd
#[derive(Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, Default)]
#[value(rename_all = "camelCase")]
pub enum TiffStorageKind {
    #[default]
    None,
    Strips,
    Tiles,
}

#[derive(Clone, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct TiffStorage {
    pub kind: TiffStorageKind,
    pub offsets_kind: TiffFieldType,
    pub byte_counts_kind: TiffFieldType,
    #[value(default)]
    pub chunks: Vec<Vec<u8>>,
}

impl Default for TiffStorage {
    fn default() -> Self {
        Self { kind: TiffStorageKind::None, offsets_kind: TiffFieldType::Long, byte_counts_kind: TiffFieldType::Long, chunks: Vec::new() }
    }
}

impl TiffStorage {
    pub fn is_empty(&self) -> bool {
        self.chunks.is_empty()
    }
}

/// 🗂️ One Image File Directory with tag-id-keyed `entries` and its own canonical authored
/// strip or tile chunks. TIFF requires ascending tag order within an IFD; codecs and mutations
/// maintain that invariant. Storage bytes keep their declared compression, precision, sample
/// layout, and chunk boundaries so every page can round-trip even when the preview projector does
/// not support that representation.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, Default)]
#[value(rename_all = "camelCase")]
pub struct TiffIfd {
    #[value(default)]
    pub entries: Vec<TiffTag>,
    #[value(default)]
    pub storage: TiffStorage,
}
//#endregion Ifd

//#region WellKnownTags
/// 📌️ TIFF6 §8/§19 baseline tag ids `⚙️engine`/`🧐️analyzer` (✳️any + 🧱️baseline) share by name.
pub const TAG_IMAGE_WIDTH: u16 = 256;
pub const TAG_IMAGE_LENGTH: u16 = 257;
pub const TAG_BITS_PER_SAMPLE: u16 = 258;
pub const TAG_COMPRESSION: u16 = 259;
pub const TAG_PHOTOMETRIC: u16 = 262;
pub const TAG_STRIP_OFFSETS: u16 = 273;
pub const TAG_SAMPLES_PER_PIXEL: u16 = 277;
pub const TAG_ROWS_PER_STRIP: u16 = 278;
pub const TAG_STRIP_BYTE_COUNTS: u16 = 279;
pub const TAG_TILE_WIDTH: u16 = 322;
pub const TAG_TILE_LENGTH: u16 = 323;
pub const TAG_TILE_OFFSETS: u16 = 324;
pub const TAG_TILE_BYTE_COUNTS: u16 = 325;
//#endregion WellKnownTags

//#region Snapshot
/// 🧬️ Complete `stdio.tiff` 6.0 semantic snapshot. `schema` is an identity field, never
/// diffed. Every IFD owns its authored strip or tile chunks; RGBA display pixels are ephemeral
/// projections and never compete with those canonical bytes.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.tiff")]
pub struct TiffSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub byte_order: TiffByteOrder,
    #[state(artifact)]
    #[value(default)]
    pub ifds: Vec<TiffIfd>,
}

impl Default for TiffSnapshot {
    fn default() -> Self {
        Self { schema: STDIO_TIFF_DOCUMENT_SCHEMA.into(), byte_order: TiffByteOrder::LittleEndian, ifds: Vec::new() }
    }
}

impl TiffSnapshot {
    /// 🔍️ Looks up a tag by id in IFD 0 (the primary image) — `None` if there is no IFD 0 or
    /// the tag isn't present in it.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn tag(&self, tag: u16) -> Option<&TiffTag> {
        self.ifds.first()?.entries.iter().find(|t| t.tag == tag)
    }
    /// 📐️ `ImageWidth` (256) from IFD 0, widened to `u32`.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn width(&self) -> Option<u32> {
        self.tag(TAG_IMAGE_WIDTH).and_then(|t| t.values.first_u32())
    }
    /// 📐️ `ImageLength` (257) from IFD 0, widened to `u32`.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn height(&self) -> Option<u32> {
        self.tag(TAG_IMAGE_LENGTH).and_then(|t| t.values.first_u32())
    }
}
//#endregion Snapshot

//#region HandcraftedArtifactCodecs
impl store::ArtifactDsl for TiffSnapshot {
 const EXTENSION:&'static str="tiff";
 fn envelope_id()->&'static str{"stdio.tiff"}
 fn parse_dsl(text:&str)->Result<Self,semio_framework_diagnostic::TextError>{
  let(envelope,body)=store::semio_format::split_text_preamble(text).map_err(|error|semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,error.to_string(),semio_framework_diagnostic::TextSpan::at(1,1)))?;
  if !envelope.matches_identity(Self::envelope_id(),store::semio_format::Component::Dsl,1){return Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,"TIFF owned Text envelope mismatch",semio_framework_diagnostic::TextSpan::at(1,1)))}
  super::text::from_record(semio_framework_dsl_record::parse_exact(body,&super::text::spec(),&semio_framework_dsl_record::ParseOptions::default())?)
 }
 fn print_dsl(&self)->String{
  let body=semio_framework_dsl_record::print(&super::text::to_record(self),&super::text::spec(),semio_framework_dsl_record::JoinMode::Document);
  let envelope=store::semio_format::SemioEnvelope::from_envelope_id(Self::envelope_id(),store::semio_format::Component::Dsl,1).expect("declared TIFF envelope");store::semio_format::wrap_text(&envelope,&body)
 }
}
impl store::ArtifactPack for TiffSnapshot {
 fn record_spec()->Option<semio_framework_dsl_record::RecordSpec>{Some(super::text::spec())}
 fn sqlite_snapshot_codec()->Option<store::ArtifactSqliteSnapshotCodec>{Some(<Self as store::ArtifactSqliteSnapshot>::sqlite_codec())}
 fn encode_pack_with(&self,options:&store::PackEncodeOptions)->Result<Vec<u8>,store::PackError>{
  let body=store::pack_rt::encode_document(&super::text::spec(),&super::text::to_record(self),options)?;
  let envelope=store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(),store::semio_format::Component::Pack,1).map_err(|error|store::PackError::from(error.into_value_error()))?;Ok(store::semio_format::wrap_binary(&envelope,&body))
 }
 fn decode_pack_with(bytes:&[u8],options:&store::PackDecodeOptions)->Result<Self,store::PackError>{
  let(envelope,body)=store::semio_format::unwrap_binary(bytes).map_err(|error|store::PackError::from(error.into_value_error()))?;
  if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(),store::semio_format::Component::Pack,1){return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "TIFF owned Pack envelope mismatch")))}
  super::text::from_record(store::pack_rt::decode_document(&body,&super::text::spec(),options)?.0).map_err(|error|store::PackError::from(error))
 }
}
//#endregion HandcraftedArtifactCodecs
