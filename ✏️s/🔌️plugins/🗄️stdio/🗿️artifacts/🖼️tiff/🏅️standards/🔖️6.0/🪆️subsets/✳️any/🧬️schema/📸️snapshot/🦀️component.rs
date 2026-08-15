//! 🧬️ TiffSnapshot schema — complete TIFF 6.0 semantic model. Ticket
//! 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION: replaces the old
//! shared `RasterImage{width,height,rgba}` stub with TIFF's REAL generic tag/type/value model:
//! `byte_order` + index-keyed `ifds: Vec<TiffIfd>`, each holding tag-id-keyed `TiffTag{tag,
//! kind, values}` entries. `TiffFieldType`/`TiffValues` cover all 12 TIFF 6.0 field types —
//! "unknown tags" are simply tags the codec doesn't specially interpret, but whose typed
//! VALUE is still stored losslessly via this same triple (the tag/type/value model IS the
//! raw-retention mechanism; no separate raw-bytes fallback needed, unlike PNG's
//! `unknown_chunks`). Decoded pixels stay a flat `pixels: Vec<u8>` (documented normalization:
//! canonical 8-bit RGBA, row-major, decoded from IFD 0 only — see `⚙️engine`'s
//! `EncodeScopeNote`/decode doc for the full completeness accounting).

use crate::artifacts::tiff::STDIO_TIFF_DOCUMENT_SCHEMA;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region ByteOrder
/// 🧭️ TIFF6 §2 byte-order mark (`II` little-endian / `MM` big-endian) — governs every
/// multi-byte field in the file, including every IFD entry's `count`/inline value bytes.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "value", rename_all = "camelCase")]
pub enum TiffValues {
    Byte(Vec<u8>),
    Ascii(String),
    Short(Vec<u16>),
    Long(Vec<u32>),
    Rational(Vec<(u32, u32)>),
    SByte(Vec<i8>),
    Undefined(Vec<u8>),
    SShort(Vec<i16>),
    SLong(Vec<i32>),
    SRational(Vec<(i32, i32)>),
    Float(Vec<f32>),
    Double(Vec<f64>),
}

impl TiffValues {
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
    pub fn count(&self) -> u32 {
        match self {
            Self::Byte(v) => v.len() as u32,
            Self::Ascii(s) => s.len() as u32 + 1,
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TiffTag {
    pub tag: u16,
    pub kind: TiffFieldType,
    pub values: TiffValues,
}
//#endregion Tag

//#region Ifd
/// 🗂️ One Image File Directory — tag-id-keyed `entries` (TIFF requires ascending-tag-order
/// within an IFD; codecs/mutations both maintain that invariant, see `⚙️engine`/`🔺️diff`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TiffIfd {
    #[serde(default)]
    pub entries: Vec<TiffTag>,
}
//#endregion Ifd

//#region WellKnownTags
/// 📌️ TIFF6 §8/§19 baseline tag ids `⚙️engine`/`🧐️analyzer` (✳️any + ✳️baseline) share by name.
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
/// diffed. `pixels` is a legitimate `Vec<u8>` exception (decoded raster payload, canonical
/// 8-bit RGBA — see `⚙️engine` doc); everything else the format defines lives in `ifds`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.tiff")]
pub struct TiffSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub byte_order: TiffByteOrder,
    #[state(artifact)]
    #[serde(default)]
    pub ifds: Vec<TiffIfd>,
    #[state(artifact)]
    #[serde(default)]
    pub pixels: Vec<u8>,
}

impl Default for TiffSnapshot {
    fn default() -> Self {
        Self { schema: STDIO_TIFF_DOCUMENT_SCHEMA.into(), byte_order: TiffByteOrder::LittleEndian, ifds: Vec::new(), pixels: Vec::new() }
    }
}

impl TiffSnapshot {
    /// 🔍️ Looks up a tag by id in IFD 0 (the primary image) — `None` if there is no IFD 0 or
    /// the tag isn't present in it.
    pub fn tag(&self, tag: u16) -> Option<&TiffTag> {
        self.ifds.first()?.entries.iter().find(|t| t.tag == tag)
    }
    /// 📐️ `ImageWidth` (256) from IFD 0, widened to `u32`.
    pub fn width(&self) -> Option<u32> {
        self.tag(TAG_IMAGE_WIDTH).and_then(|t| t.values.first_u32())
    }
    /// 📐️ `ImageLength` (257) from IFD 0, widened to `u32`.
    pub fn height(&self) -> Option<u32> {
        self.tag(TAG_IMAGE_LENGTH).and_then(|t| t.values.first_u32())
    }
}
//#endregion Snapshot

//#region HandcraftedArtifactCodecs
impl store::ArtifactDsl for TiffSnapshot {
    const EXTENSION: &'static str = "tiff";
    fn envelope_id() -> &'static str {
        "stdio.tiff"
    }

    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let hex: String = body.chars().filter(|c| !c.is_whitespace()).collect();
        if hex.len() % 2 != 0 {
            return Err(store::TextError::new("odd hex length", dsl::TextSpan::at(1, 1)));
        }
        let mut bytes = Vec::with_capacity(hex.len() / 2);
        let mut i = 0usize;
        while i < hex.len() {
            let byte = u8::from_str_radix(&hex[i..i + 2], 16).map_err(|e| store::TextError::new(format!("invalid hex: {e}"), dsl::TextSpan::at(1, 1)))?;
            bytes.push(byte);
            i += 2;
        }
        crate::artifacts::tiff::engine::decode_tiff(&bytes).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }

    fn print_dsl(&self) -> String {
        let bytes = crate::artifacts::tiff::engine::encode_tiff(self).unwrap_or_default();
        let body: String = bytes.iter().map(|b| format!("{b:02x}")).collect();
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for TiffSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = crate::artifacts::tiff::engine::encode_tiff(self).map_err(|e| store::PackError::Schema(e))?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }

    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.envelope_id())));
        }
        let _ = options;
        crate::artifacts::tiff::engine::decode_tiff(&inner).map_err(|e| store::PackError::Schema(e))
    }
}
//#endregion HandcraftedArtifactCodecs
