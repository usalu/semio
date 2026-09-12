//! 🧬️ Mp4Snapshot — ISO-BMFF: `ftyp` typed, decoded per-track sample tables (`stts`/`ctts`/
//! `stsc`/`stsz`/`stco`/`stss` flattened into per-sample records), AVC codec config typed
//! (`avcC` SPS/PPS) and logical sample-to-chunk grouping. Native bytes are materialized only by
//! the ordinary ISO-BMFF writer.

use framework_schema::ArtifactSchema;

//#region 🔖️Ids
pub const STDIO_MP4_DOCUMENT_SCHEMA: &str = "stdio.mp4";
//#endregion 🔖️Ids

//#region 🔖️Ftyp
/// 🏷️ File-type box: brand + compatible-brand list. <https://www.iso.org/standard/74428.html> §4.3
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct Mp4Ftyp {
    pub major_brand: String,
    pub minor_version: u32,
    #[value(default)]
    pub compatible_brands: Vec<String>,
}
//#endregion 🔖️Ftyp

//#region 🔖️Codec
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct Mp4AvcExtension {
    pub chroma_format: u8,
    pub bit_depth_luma_minus8: u8,
    pub bit_depth_chroma_minus8: u8,
    #[value(default)]
    pub sps_ext: Vec<Vec<u8>>,
}

/// 🎥️ A track's typed AVC sample description. Unsupported codecs are rejected on import.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct Mp4Codec {
    #[value(default)]
    pub sps: Vec<Vec<u8>>,
    #[value(default)]
    pub pps: Vec<Vec<u8>>,
    pub nal_length_size: u8,
    #[value(default)]
    pub extension: Option<Mp4AvcExtension>,
}

impl Default for Mp4Codec {
    fn default() -> Self {
        Self { sps: Vec::new(), pps: Vec::new(), nal_length_size: 4, extension: None }
    }
}
//#endregion 🔖️Codec

//#region 🔖️Sample
/// 🎞️ One decoded sample: exact payload bytes (AVCC/length-prefixed as the container held them —
/// payload-opaque, matching the master plan's "video is container-typed, payload-opaque" call),
/// its `stts` duration in the track's timescale, its `ctts` composition-time offset, and whether
/// `stss` marks it a sync (random-access) sample (absent `stss` ⇒ every sample is sync, per spec).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct Mp4Sample {
    #[value(default)]
    #[dsl(base64)]
    pub data: Vec<u8>,
    pub duration: u32,
    #[value(default)]
    pub cts_offset: i32,
    pub sync: bool,
}
//#endregion 🔖️Sample

//#region 🎬️Movie
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct Mp4Movie {
    pub creation_time: u64,
    pub modification_time: u64,
    pub timescale: u32,
    pub duration: u64,
    pub rate: i32,
    pub volume: i16,
    pub matrix: [i32; 9],
    pub next_track_id: u32,
    #[value(default)]
    pub title: Option<String>,
    #[value(default)]
    pub encoder: Option<String>,
}

impl Default for Mp4Movie {
    fn default() -> Self {
        Self { creation_time: 0, modification_time: 0, timescale: 1000, duration: 0, rate: 0x0001_0000, volume: 0x0100, matrix: [0x0001_0000, 0, 0, 0, 0x0001_0000, 0, 0, 0, 0x4000_0000], next_track_id: 1, title: None, encoder: None }
    }
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct Mp4Edit {
    pub segment_duration: u64,
    pub media_time: i64,
    pub media_rate_integer: i16,
    pub media_rate_fraction: i16,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct Mp4VisualSampleEntry {
    pub data_reference_index: u16,
    pub version: u16,
    pub revision_level: u16,
    pub vendor: u32,
    pub temporal_quality: u32,
    pub spatial_quality: u32,
    pub horizontal_resolution: u32,
    pub vertical_resolution: u32,
    pub frame_count: u16,
    pub compressor_name: String,
    pub depth: u16,
    pub color_table_id: i16,
}

impl Default for Mp4VisualSampleEntry {
    fn default() -> Self {
        Self {
            data_reference_index: 1,
            version: 0,
            revision_level: 0,
            vendor: 0,
            temporal_quality: 0,
            spatial_quality: 0,
            horizontal_resolution: 0x0048_0000,
            vertical_resolution: 0x0048_0000,
            frame_count: 1,
            compressor_name: String::new(),
            depth: 24,
            color_table_id: -1,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct Mp4Color {
    pub color_type: String,
    pub primaries: u16,
    pub transfer: u16,
    pub matrix: u16,
    #[value(default)]
    pub full_range: Option<bool>,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct Mp4PixelAspectRatio {
    pub horizontal_spacing: u32,
    pub vertical_spacing: u32,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct Mp4Bitrate {
    pub buffer_size: u32,
    pub maximum: u32,
    pub average: u32,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct Mp4TrackMetadata {
    pub creation_time: u64,
    pub modification_time: u64,
    pub flags: u32,
    pub duration: u64,
    pub layer: i16,
    pub alternate_group: i16,
    pub volume: i16,
    pub matrix: [i32; 9],
    pub media_duration: u64,
    pub media_creation_time: u64,
    pub media_modification_time: u64,
    pub language: String,
    pub quality: u16,
    pub handler_name: String,
    #[value(default)]
    pub edits: Vec<Mp4Edit>,
    pub visual: Mp4VisualSampleEntry,
    #[value(default)]
    pub color: Option<Mp4Color>,
    #[value(default)]
    pub pixel_aspect_ratio: Option<Mp4PixelAspectRatio>,
    #[value(default)]
    pub bitrate: Option<Mp4Bitrate>,
}

impl Default for Mp4TrackMetadata {
    fn default() -> Self {
        Self {
            creation_time: 0,
            modification_time: 0,
            flags: 3,
            duration: 0,
            layer: 0,
            alternate_group: 0,
            volume: 0,
            matrix: [0x0001_0000, 0, 0, 0, 0x0001_0000, 0, 0, 0, 0x4000_0000],
            media_duration: 0,
            media_creation_time: 0,
            media_modification_time: 0,
            language: "und".into(),
            quality: 0,
            handler_name: String::new(),
            edits: Vec::new(),
            visual: Mp4VisualSampleEntry::default(),
            color: None,
            pixel_aspect_ratio: None,
            bitrate: None,
        }
    }
}
//#endregion 🎬️Movie

//#region 🔖️Track
/// 🛤️ One typed video `trak`; unsupported handler types are rejected on import.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct Mp4Track {
    pub track_id: u32,
    pub timescale: u32,
    pub codec: Mp4Codec,
    pub width: u32,
    pub height: u32,
    pub metadata: Mp4TrackMetadata,
    /// 🧱️ Logical sample grouping per media chunk, in `stco`/`co64` order.
    #[value(default)]
    pub chunk_sample_counts: Vec<u32>,
    #[value(default)]
    pub samples: Vec<Mp4Sample>,
}
//#endregion 🔖️Track

//#region 🔖️Snapshot
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.mp4")]
pub struct Mp4Snapshot {
    #[state(artifact)]
    #[value(default = "default_schema")]
    pub schema: String,
    #[state(artifact)]
    pub ftyp: Mp4Ftyp,
    #[state(artifact)]
    pub movie: Mp4Movie,
    #[state(artifact)]
    #[value(default)]
    pub tracks: Vec<Mp4Track>,
}

fn default_schema() -> String {
    STDIO_MP4_DOCUMENT_SCHEMA.into()
}

impl Default for Mp4Snapshot {
    /// 🌱️ A minimal but real, 4-byte-brand `ftyp` — `major_brand` MUST be exactly 4 ASCII bytes
    /// for a genuinely valid box (unlike an empty string, which `⚙️engine::encode_mp4` would have
    /// to pad, breaking the empty-snapshot round trip below).
    fn default() -> Self {
        Self { schema: STDIO_MP4_DOCUMENT_SCHEMA.into(), ftyp: Mp4Ftyp { major_brand: "isom".into(), minor_version: 0, compatible_brands: Vec::new() }, movie: Mp4Movie::default(), tracks: Vec::new() }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️HandcraftedArtifactCodecs
/// 🎙️ Snapshot-model codecs serialize only the logical ISO-BMFF model.
impl store::ArtifactDsl for Mp4Snapshot {
    const EXTENSION: &'static str = "semio";
    fn envelope_id() -> &'static str {
        STDIO_MP4_DOCUMENT_SCHEMA
    }

    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = dsl::parse(body, &Self::__dsl_spec(), &dsl::ParseOptions { limits: dsl::Limits { max_bytes: 32 * 1024 * 1024, ..dsl::Limits::default() }, mode: dsl::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }

    fn print_dsl(&self) -> String {
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for Mp4Snapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let raw = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }

    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.binary_token())));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }

    fn record_spec() -> Option<dsl::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs

//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests
