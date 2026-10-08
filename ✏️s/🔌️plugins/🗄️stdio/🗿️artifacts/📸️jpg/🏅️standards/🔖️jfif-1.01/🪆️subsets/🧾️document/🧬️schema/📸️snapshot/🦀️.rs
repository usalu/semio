//! 🧬️ Owned JPEG image content, density, thumbnail, and opaque metadata.

use crate::STDIO_JPG_DOCUMENT_SCHEMA;
use framework_schema::ArtifactSchema;


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
    #[value(default, with = "pack::value::bytes", serialize_controlled_with = "pack::value::bytes::to_value_controlled")]
    pub rgb_data: Vec<u8>,
}
//#endregion Jfif

//#region OtherSegments
/// 🗃️ An APPn (other than a recognized JFIF APP0)/COM segment the codec doesn't specifically
/// model, retained VERBATIM (typed raw-retention — "nothing real on disk silently dropped").
/// Index-keyed (not marker-keyed): duplicate COM/APPn markers are legal, so position is the only
/// safe stable identity within one decode (mirrors png's `PngTextChunk` reasoning).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct JpgSegment {
    pub marker: u8,
    #[value(default, with = "pack::value::bytes", serialize_controlled_with = "pack::value::bytes::to_value_controlled")]
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
    #[value(default, with = "pack::value::bytes", serialize_controlled_with = "pack::value::bytes::to_value_controlled")]
    pub pixels: Vec<u8>,

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
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub jfif_thumbnail: Option<JfifThumbnail>,

    // SOF (T.81 §B.2.2) — see the struct doc for why `frame`/`sof_marker`/`arithmetic` keep
    // their pre-existing shapes/names.

    // DQT (T.81 §B.2.4.1) / DHT (T.81 §B.2.4.2) — id-keyed (DHT compound-keyed by class+id).

    // DRI (T.81 §B.2.4.4) — `None` = no restart interval segment was present.

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
            jfif_version: (1, 1),
            jfif_density_units: JfifDensityUnits::Aspect,
            jfif_x_density: 1,
            jfif_y_density: 1,
            jfif_thumbnail: None,
            other_segments: Vec::new(),
        }
    }
}
//#endregion Snapshot

//#region HandcraftedArtifactCodecs



//#endregion HandcraftedArtifactCodecs


