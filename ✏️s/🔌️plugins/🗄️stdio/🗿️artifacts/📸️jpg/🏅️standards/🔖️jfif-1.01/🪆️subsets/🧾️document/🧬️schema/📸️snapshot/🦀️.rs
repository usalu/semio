//! 🧬️ JpgSnapshot schema — complete JFIF 1.01 semantic model, real codecs. Ticket
//! 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION: replaces the old
//! `RasterImage`-shaped stub (`image: RasterImage{width,height,rgba}` only, no JFIF/SOF/DQT/DHT
//! typing at all) with a typed JFIF APP0 header, typed SOF (frame) + id-keyed DQT/DHT tables,
//! `DRI` restart interval, verbatim-retained other APPn/COM segments, and decoded pixels —
//! `## Snapshot completeness spec`'s jpg row. `RasterImage` itself dies per the ticket's explicit
//! kill directive (W0: "shared verbatim across jpg/png/tiff, png already killed its own copy") —
//! `width`/`height`/`pixels` are first-class fields here, no shared wrapper type.

use crate::artifacts::jpg::STDIO_JPG_DOCUMENT_SCHEMA;
use schema::ArtifactSchema;

//#region Jfif
/// 📏️ JFIF APP0 `units` byte (ITU-T T.871 / JFIF 1.02 §). `Aspect` means `x_density`/
/// `y_density` are merely a pixel aspect ratio, not an absolute resolution.
#[derive(Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, Default)]
#[value(rename_all = "camelCase")]
pub enum JfifDensityUnits {
    #[default]
    Aspect,
    PixelsPerInch,
    PixelsPerCm,
}

impl JfifDensityUnits {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn from_u8(v: u8) -> Result<Self, String> {
        match v {
            0 => Ok(JfifDensityUnits::Aspect),
            1 => Ok(JfifDensityUnits::PixelsPerInch),
            2 => Ok(JfifDensityUnits::PixelsPerCm),
            _ => Err(format!("jfif: unsupported density unit {v}")),
        }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn to_u8(self) -> u8 {
        match self {
            JfifDensityUnits::Aspect => 0,
            JfifDensityUnits::PixelsPerInch => 1,
            JfifDensityUnits::PixelsPerCm => 2,
        }
    }
}

/// 🖼️ JFIF APP0's optional embedded thumbnail — uncompressed 24-bit RGB, `width * height * 3`
/// bytes, row-major. A weak value (whole-value replaced in diffs, never sub-diffed).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, Default)]
#[value(rename_all = "camelCase")]
pub struct JfifThumbnail {
    pub width: u8,
    pub height: u8,
    #[value(default)]
    pub rgb_data: Vec<u8>,
}
//#endregion Jfif

//#region FrameScanModel
/// 🧩 One SOF0 frame component descriptor: id, H/V sampling factors, and which of the (up to 4)
/// DQT tables it dequantizes against. Id-keyed within `JpgFrameHeader.components`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct JpgFrameComponent {
    pub id: u8,
    pub h_sampling: u8,
    pub v_sampling: u8,
    pub quant_table_id: u8,
}

/// 🖼️ Baseline (SOF0) frame header — sample precision, dimensions, and the per-component
/// sampling/quant-table layout the entropy-coded scan follows.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct JpgFrameHeader {
    pub precision: u8,
    pub width: u16,
    pub height: u16,
    pub components: Vec<JpgFrameComponent>,
}

/// 🎯 One SOS scan component: which DC/AC Huffman table (of up to 4 each) it decodes with.
/// Transient decode/encode state — not persisted on `JpgSnapshot` (the persisted per-component
/// table binding is `JpgFrameComponent.quant_table_id` plus `JpgSnapshot.huffman_tables`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct JpgScanComponent {
    pub id: u8,
    pub dc_table_id: u8,
    pub ac_table_id: u8,
}
//#endregion FrameScanModel

//#region QuantHuffmanTables
/// 📊️ One `DQT` table (id-keyed within `JpgSnapshot.quant_tables`). `values` is retained in the
/// EXACT zigzag scan order the DQT segment stores on disk (T.81 Annex B §B.2.4.1) — never
/// reindexed to natural/row-major order, so a decoded table round-trips byte-for-byte.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct JpgQuantTable {
    pub id: u8,
    /// 🔢️ DQT `Pq` nibble: `0` = 8-bit values, `1` = 16-bit values.
    pub precision: u8,
    pub values: [u16; 64],
}

/// 🌳️ `DHT` table class — DC (differential prediction) or AC (run-length coefficients).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, value_derive::ToValue, value_derive::FromValue, Default)]
#[value(rename_all = "camelCase")]
pub enum JpgHuffmanClass {
    #[default]
    Dc,
    Ac,
}

impl JpgHuffmanClass {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn from_u8(v: u8) -> Result<Self, String> {
        match v {
            0 => Ok(JpgHuffmanClass::Dc),
            1 => Ok(JpgHuffmanClass::Ac),
            _ => Err(format!("jpg: unsupported huffman class {v}")),
        }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn to_u8(self) -> u8 {
        match self {
            JpgHuffmanClass::Dc => 0,
            JpgHuffmanClass::Ac => 1,
        }
    }
}

/// 🌳️ One `DHT` table, keyed by `(class, id)` within `JpgSnapshot.huffman_tables` (DC id=0 and
/// AC id=0 are DIFFERENT tables — the compound key is load-bearing). `bits`/`values` are the raw
/// canonical-code counts-per-length and symbol-value bytes exactly as the DHT segment stores them.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct JpgHuffmanTable {
    pub id: u8,
    pub class: JpgHuffmanClass,
    pub bits: [u8; 16],
    #[value(default)]
    pub values: Vec<u8>,
}
//#endregion QuantHuffmanTables

//#region OtherSegments
/// 🗃️ An APPn (other than a recognized JFIF APP0)/COM segment the codec doesn't specifically
/// model, retained VERBATIM (typed raw-retention — "nothing real on disk silently dropped").
/// Index-keyed (not marker-keyed): duplicate COM/APPn markers are legal, so position is the only
/// safe stable identity within one decode (mirrors png's `PngTextChunk` reasoning).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct JpgSegment {
    pub marker: u8,
    #[value(default)]
    pub data: Vec<u8>,
}
//#endregion OtherSegments

//#region Snapshot
/// 🧬️ Complete `stdio.jpg` jfif-1.01 semantic snapshot. `schema` is an identity field, never
/// diffed. `frame`/`sof_marker`/`arithmetic` are populated by `engine::decode_jpg` at successful
/// decode (`None`/`0`/`false` only for a snapshot that has never round-tripped through a real
/// JPEG byte stream) — retained under those exact names/shapes because
/// `🧱️baseline::analyzer::check_baseline_conformance` depends on them (ticket
/// 26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.jpg")]
pub struct JpgSnapshot {
    #[state(artifact)]
    pub schema: String,

    // Decoded raster payload — legitimate `Vec<u8>` exception (the format's payload IS pixels):
    // canonical 8-bit-per-channel RGBA, `width * height * 4` bytes, row-major, top-to-bottom.
    // `width`/`height` here are the CANONICAL raster dimensions a caller wants encoded — distinct
    // from `frame.width`/`frame.height` (u16 on-disk SOF values, only present after a real
    // decode/encode); the two agree for any engine-produced snapshot but a freshly hand-authored
    // one (via `SetPixels`) has no `frame` yet and still needs its own dimensions.
    #[state(artifact)]
    #[value(default)]
    pub width: u32,
    #[state(artifact)]
    #[value(default)]
    pub height: u32,
    #[state(artifact)]
    #[value(default)]
    pub pixels: Vec<u8>,
    /// 🎚️ Quality parameter `engine::encode_jpg` scales the Annex K quantization tables by
    /// (IJG convention, `1..=100`). `None` = the engine's own default (90).
    #[state(artifact)]
    #[value(default)]
    pub re_encode_quality: Option<u8>,

    // JFIF APP0 (ITU-T T.871 / JFIF 1.02). Always first-class (non-optional): every JFIF file
    // carries exactly one of these; a never-decoded snapshot keeps the spec's own defaults
    // (version 1.01, aspect-ratio units, 1x1 density, no thumbnail) — `engine::encode_jpg`
    // writes them out unconditionally, matching every real JFIF encoder.
    #[state(artifact)]
    #[value(default)]
    pub jfif_version: (u8, u8),
    #[state(artifact)]
    #[value(default)]
    pub jfif_density_units: JfifDensityUnits,
    #[state(artifact)]
    #[value(default)]
    pub jfif_x_density: u16,
    #[state(artifact)]
    #[value(default)]
    pub jfif_y_density: u16,
    #[state(artifact)]
    #[value(default)]
    pub jfif_thumbnail: Option<JfifThumbnail>,

    // SOF (T.81 §B.2.2) — see the struct doc for why `frame`/`sof_marker`/`arithmetic` keep
    // their pre-existing shapes/names.
    #[state(artifact)]
    #[value(default)]
    pub frame: Option<JpgFrameHeader>,
    #[state(artifact)]
    #[value(default)]
    pub sof_marker: u8,
    #[state(artifact)]
    #[value(default)]
    pub arithmetic: bool,

    // DQT (T.81 §B.2.4.1) / DHT (T.81 §B.2.4.2) — id-keyed (DHT compound-keyed by class+id).
    #[state(artifact)]
    #[value(default)]
    pub quant_tables: Vec<JpgQuantTable>,
    #[state(artifact)]
    #[value(default)]
    pub huffman_tables: Vec<JpgHuffmanTable>,

    // DRI (T.81 §B.2.4.4) — `None` = no restart interval segment was present.
    #[state(artifact)]
    #[value(default)]
    pub restart_interval: Option<u16>,

    // Verbatim-retained other APPn/COM segments, in encounter order (§`JpgSegment` doc).
    #[state(artifact)]
    #[value(default)]
    pub other_segments: Vec<JpgSegment>,
}

impl Default for JpgSnapshot {
    fn default() -> Self {
        Self {
            schema: STDIO_JPG_DOCUMENT_SCHEMA.into(),
            width: 0,
            height: 0,
            pixels: Vec::new(),
            re_encode_quality: None,
            jfif_version: (1, 1),
            jfif_density_units: JfifDensityUnits::Aspect,
            jfif_x_density: 1,
            jfif_y_density: 1,
            jfif_thumbnail: None,
            frame: None,
            sof_marker: 0,
            arithmetic: false,
            quant_tables: Vec::new(),
            huffman_tables: Vec::new(),
            restart_interval: None,
            other_segments: Vec::new(),
        }
    }
}
//#endregion Snapshot

//#region HandcraftedArtifactCodecs
impl store::ArtifactDsl for JpgSnapshot {
    const EXTENSION: &'static str = "jpg";
    fn envelope_id() -> &'static str {
        "stdio.jpg"
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
        crate::artifacts::jpg::engine::decode_jpg(&bytes).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))
    }

    fn print_dsl(&self) -> String {
        let bytes = crate::artifacts::jpg::engine::encode_jpg(self).unwrap_or_default();
        let body: String = bytes.iter().map(|b| format!("{b:02x}")).collect();
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for JpgSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = crate::artifacts::jpg::engine::encode_jpg(self).map_err(|e| store::PackError::Schema(e.to_string()))?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }

    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.envelope_id())));
        }
        let _ = options;
        crate::artifacts::jpg::engine::decode_jpg(&inner).map_err(|e| store::PackError::Schema(e.to_string()))
    }
}
//#endregion HandcraftedArtifactCodecs
