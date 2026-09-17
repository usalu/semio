//! 🧬️ Mp4Snapshot — ISO-BMFF: `ftyp` typed, decoded per-track sample tables (`stts`/`ctts`/
//! `stsc`/`stsz`/`stco`/`stss` flattened into per-sample records), codec config typed
//! per sample entry (`avcC`, `hvcC`, Motion-JPEG) and logical sample-to-chunk grouping. Native bytes are materialized only by
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

/// 🏷️ The visual sample entry (`stsd` child box type) a track's samples are coded with.
/// AVC per ISO/IEC 14496-15 §5.4 (`avc1`/`avc3`), HEVC per §8.4 (`hvc1`/`hev1`), and Motion-JPEG
/// per the QuickTime File Format sample description table (`jpeg` Photo-JPEG, `mjpa` Motion-JPEG
/// format A) — each sample of a JPEG track is one complete JFIF/JPEG interchange image.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, dsl::DslScalar)]
#[value(rename_all = "camelCase")]
pub enum Mp4CodecFormat {
    #[default]
    Avc1,
    Avc3,
    Hvc1,
    Hev1,
    Jpeg,
    Mjpa,
}

impl Mp4CodecFormat {
    /// 🔤️ The sample entry's four-character box type.
    pub fn fourcc(self) -> &'static str {
        match self {
            Self::Avc1 => "avc1",
            Self::Avc3 => "avc3",
            Self::Hvc1 => "hvc1",
            Self::Hev1 => "hev1",
            Self::Jpeg => "jpeg",
            Self::Mjpa => "mjpa",
        }
    }

    /// 🔍️ The format whose sample entry box type is `fourcc`, if this artifact models it.
    pub fn from_fourcc(fourcc: &[u8]) -> Option<Self> {
        Some(match fourcc {
            b"avc1" => Self::Avc1,
            b"avc3" => Self::Avc3,
            b"hvc1" => Self::Hvc1,
            b"hev1" => Self::Hev1,
            b"jpeg" => Self::Jpeg,
            b"mjpa" => Self::Mjpa,
            _ => return None,
        })
    }

    pub fn is_avc(self) -> bool {
        matches!(self, Self::Avc1 | Self::Avc3)
    }

    pub fn is_hevc(self) -> bool {
        matches!(self, Self::Hvc1 | Self::Hev1)
    }

    pub fn is_jpeg(self) -> bool {
        matches!(self, Self::Jpeg | Self::Mjpa)
    }

    fn is_default(&self) -> bool {
        *self == Self::Avc1
    }
}

/// 🧱️ One `hvcC` NAL unit array (ISO/IEC 14496-15 §8.3.3.1): VPS/SPS/PPS/SEI units of one type.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct Mp4HevcNalArray {
    pub array_completeness: bool,
    pub nal_unit_type: u8,
    #[value(default)]
    pub nal_units: Vec<Vec<u8>>,
}

/// 🎥️ `hvcC` — HEVCDecoderConfigurationRecord (ISO/IEC 14496-15 §8.3.3.1), every field typed.
/// `nal_length_size` lives on [`Mp4Codec`] and is shared with AVC.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct Mp4HevcConfig {
    pub general_profile_space: u8,
    pub general_tier_flag: bool,
    pub general_profile_idc: u8,
    pub general_profile_compatibility_flags: u32,
    /// 🔢️ 48-bit `general_constraint_indicator_flags`.
    pub general_constraint_indicator_flags: u64,
    pub general_level_idc: u8,
    pub min_spatial_segmentation_idc: u16,
    pub parallelism_type: u8,
    pub chroma_format_idc: u8,
    pub bit_depth_luma_minus8: u8,
    pub bit_depth_chroma_minus8: u8,
    pub avg_frame_rate: u16,
    pub constant_frame_rate: u8,
    pub num_temporal_layers: u8,
    pub temporal_id_nested: bool,
    #[value(default)]
    pub arrays: Vec<Mp4HevcNalArray>,
}

impl Default for Mp4HevcConfig {
    /// 🌱️ Main profile, level 3.1, 4:2:0 8-bit, no parameter sets.
    fn default() -> Self {
        Self {
            general_profile_space: 0,
            general_tier_flag: false,
            general_profile_idc: 1,
            general_profile_compatibility_flags: 0x6000_0000,
            general_constraint_indicator_flags: 0,
            general_level_idc: 93,
            min_spatial_segmentation_idc: 0,
            parallelism_type: 0,
            chroma_format_idc: 1,
            bit_depth_luma_minus8: 0,
            bit_depth_chroma_minus8: 0,
            avg_frame_rate: 0,
            constant_frame_rate: 0,
            num_temporal_layers: 1,
            temporal_id_nested: true,
            arrays: Vec::new(),
        }
    }
}

/// 🎥️ A track's typed sample description: the sample entry `format` plus that format's typed
/// decoder configuration — `avcC` (`sps`/`pps`/`nal_length_size`/`extension`) for AVC, `hvcC`
/// (`hevc` + `nal_length_size`) for HEVC, none for JPEG. Unsupported sample entries are rejected on
/// import. The encoder writes only the configuration record `format` names.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct Mp4Codec {
    #[value(default, skip_serializing_if = "Mp4CodecFormat::is_default")]
    pub format: Mp4CodecFormat,
    #[value(default)]
    pub sps: Vec<Vec<u8>>,
    #[value(default)]
    pub pps: Vec<Vec<u8>>,
    pub nal_length_size: u8,
    #[value(default)]
    pub extension: Option<Mp4AvcExtension>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub hevc: Option<Mp4HevcConfig>,
}

impl Mp4Codec {
    /// 🎥️ An AVC (`avc1`) description from its parameter sets.
    pub fn avc(sps: Vec<Vec<u8>>, pps: Vec<Vec<u8>>, nal_length_size: u8, extension: Option<Mp4AvcExtension>) -> Self {
        Self { format: Mp4CodecFormat::Avc1, sps, pps, nal_length_size, extension, hevc: None }
    }

    /// 🎥️ An HEVC description (`hvc1` or `hev1`) from its `hvcC` record.
    pub fn hevc(format: Mp4CodecFormat, config: Mp4HevcConfig, nal_length_size: u8) -> Self {
        Self { format, sps: Vec::new(), pps: Vec::new(), nal_length_size, extension: None, hevc: Some(config) }
    }

    /// 🖼️ A Motion-JPEG description (`jpeg` or `mjpa`); JPEG carries no configuration record.
    pub fn jpeg(format: Mp4CodecFormat) -> Self {
        Self { format, sps: Vec::new(), pps: Vec::new(), nal_length_size: 4, extension: None, hevc: None }
    }
}

impl Default for Mp4Codec {
    fn default() -> Self {
        Self::avc(Vec::new(), Vec::new(), 4, None)
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
