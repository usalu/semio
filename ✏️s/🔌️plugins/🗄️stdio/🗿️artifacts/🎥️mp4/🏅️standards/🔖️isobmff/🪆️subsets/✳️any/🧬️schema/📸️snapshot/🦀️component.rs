//! 🧬️ Mp4Snapshot — ISO-BMFF: `ftyp` typed, decoded per-track sample tables (`stts`/`ctts`/
//! `stsc`/`stsz`/`stco`/`stss` flattened into per-sample records), AVC codec config typed
//! (`avcC` SPS/PPS) and logical sample-to-chunk grouping. Native bytes are materialized only by
//! the ordinary ISO-BMFF writer.

use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Ids
pub const STDIO_MP4_DOCUMENT_SCHEMA: &str = "stdio.mp4";
//#endregion 🔖️Ids

//#region 🔖️Ftyp
/// 🏷️ File-type box: brand + compatible-brand list. <https://www.iso.org/standard/74428.html> §4.3
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Mp4Ftyp {
    pub major_brand: String,
    pub minor_version: u32,
    #[serde(default)]
    pub compatible_brands: Vec<String>,
}
//#endregion 🔖️Ftyp

//#region 🔖️Codec
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Mp4AvcExtension {
    pub chroma_format: u8,
    pub bit_depth_luma_minus8: u8,
    pub bit_depth_chroma_minus8: u8,
    #[serde(default)]
    pub sps_ext: Vec<Vec<u8>>,
}

/// 🎥️ A track's typed AVC sample description. Unsupported codecs are rejected on import.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Mp4Codec {
    #[serde(default)]
    pub sps: Vec<Vec<u8>>,
    #[serde(default)]
    pub pps: Vec<Vec<u8>>,
    pub nal_length_size: u8,
    #[serde(default)]
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
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Mp4Sample {
    #[serde(default)]
    #[dsl(base64)]
    pub data: Vec<u8>,
    pub duration: u32,
    #[serde(default)]
    pub cts_offset: i32,
    pub sync: bool,
}
//#endregion 🔖️Sample

//#region 🎬️Movie
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Mp4Movie {
    pub creation_time: u64,
    pub modification_time: u64,
    pub timescale: u32,
    pub duration: u64,
    pub rate: i32,
    pub volume: i16,
    pub matrix: [i32; 9],
    pub next_track_id: u32,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub encoder: Option<String>,
}

impl Default for Mp4Movie {
    fn default() -> Self {
        Self { creation_time: 0, modification_time: 0, timescale: 1000, duration: 0, rate: 0x0001_0000, volume: 0x0100, matrix: [0x0001_0000, 0, 0, 0, 0x0001_0000, 0, 0, 0, 0x4000_0000], next_track_id: 1, title: None, encoder: None }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Mp4Edit {
    pub segment_duration: u64,
    pub media_time: i64,
    pub media_rate_integer: i16,
    pub media_rate_fraction: i16,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
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

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Mp4Color {
    pub color_type: String,
    pub primaries: u16,
    pub transfer: u16,
    pub matrix: u16,
    #[serde(default)]
    pub full_range: Option<bool>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Mp4PixelAspectRatio {
    pub horizontal_spacing: u32,
    pub vertical_spacing: u32,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Mp4Bitrate {
    pub buffer_size: u32,
    pub maximum: u32,
    pub average: u32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
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
    #[serde(default)]
    pub edits: Vec<Mp4Edit>,
    pub visual: Mp4VisualSampleEntry,
    #[serde(default)]
    pub color: Option<Mp4Color>,
    #[serde(default)]
    pub pixel_aspect_ratio: Option<Mp4PixelAspectRatio>,
    #[serde(default)]
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
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Mp4Track {
    pub track_id: u32,
    pub timescale: u32,
    pub codec: Mp4Codec,
    pub width: u32,
    pub height: u32,
    pub metadata: Mp4TrackMetadata,
    /// 🧱️ Logical sample grouping per media chunk, in `stco`/`co64` order.
    #[serde(default)]
    pub chunk_sample_counts: Vec<u32>,
    #[serde(default)]
    pub samples: Vec<Mp4Sample>,
}
//#endregion 🔖️Track

//#region 🔖️Snapshot
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.mp4")]
pub struct Mp4Snapshot {
    #[state(artifact)]
    #[serde(default = "default_schema")]
    pub schema: String,
    #[state(artifact)]
    pub ftyp: Mp4Ftyp,
    #[state(artifact)]
    pub movie: Mp4Movie,
    #[state(artifact)]
    #[serde(default)]
    pub tracks: Vec<Mp4Track>,
}

async fn default_schema() -> String {
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
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.envelope_id())));
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
mod tests {
    use super::*;

    async fn sample_snapshot() -> Mp4Snapshot {
        Mp4Snapshot {
            schema: STDIO_MP4_DOCUMENT_SCHEMA.into(),
            ftyp: Mp4Ftyp { major_brand: "isom".into(), minor_version: 512, compatible_brands: vec!["isom".into(), "iso2".into(), "avc1".into(), "mp41".into()] },
            movie: Mp4Movie::default(),
            tracks: vec![Mp4Track {
                track_id: 1,
                timescale: 1000,
                codec: Mp4Codec { sps: vec![vec![0x67, 0x42, 0x00, 0x1E, 0x8C, 0x8D, 0x40]], pps: vec![vec![0x68, 0xCE, 0x3C, 0x80]], nal_length_size: 4, extension: None },
                width: 64,
                height: 64,
                metadata: Mp4TrackMetadata::default(),
                chunk_sample_counts: vec![2],
                samples: vec![Mp4Sample { data: vec![0, 0, 0, 4, 0x65, 1, 2, 3], duration: 33, cts_offset: 0, sync: true }, Mp4Sample { data: vec![0, 0, 0, 3, 0x61, 4, 5], duration: 33, cts_offset: 33, sync: false }],
            }],
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn json_pack_round_trips_via_real_mp4_bytes() {
        let snap = sample_snapshot();
        let bytes = <Mp4Snapshot as store::ArtifactPack>::encode_pack(&snap);
        let back = <Mp4Snapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(snap, back);
    }

    #[semio_framework_async_macros::async_test]
    async fn dsl_text_round_trips_via_real_mp4_bytes() {
        let snap = sample_snapshot();
        let text = <Mp4Snapshot as store::ArtifactDsl>::print_dsl(&snap);
        let back = <Mp4Snapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
        assert_eq!(snap, back);
    }

    #[semio_framework_async_macros::async_test]
    async fn default_snapshot_round_trips_through_real_codec() {
        let snap = Mp4Snapshot::default();
        let bytes = <Mp4Snapshot as store::ArtifactPack>::encode_pack(&snap);
        let back = <Mp4Snapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(snap, back);
    }

    #[semio_framework_async_macros::async_test]
    async fn logical_snapshot_and_facets_have_no_shadow_state() {
        let model = format!("{:?}", sample_snapshot());
        for forbidden in ["unknownBoxes", "physical", "sourceBytes", "nativeArchive", "\"raw\""] {
            assert!(!model.contains(forbidden), "snapshot contains forbidden shadow field {forbidden}");
        }
        for facet in [
            include_str!("🟦️component.ts"),
            include_str!("🔗️component.graphql"),
            include_str!("🔣️component.json"),
            include_str!("🛰️component.proto"),
            include_str!("../🔺️diff/🦀️component.rs"),
            include_str!("../🔺️diff/🟦️component.ts"),
            include_str!("../🔺️diff/🔗️component.graphql"),
            include_str!("../🔺️diff/🔣️component.json"),
            include_str!("../🔺️diff/🛰️component.proto"),
            include_str!("../🧬️mutations/🦀️component.rs"),
            include_str!("../🧬️mutations/🟦️component.ts"),
            include_str!("../🧬️mutations/🔗️component.graphql"),
            include_str!("../🧬️mutations/🔣️component.json"),
            include_str!("../🧬️mutations/🛰️component.proto"),
            include_str!("💾️binary/🔠️component.abnf"),
            include_str!("💾️binary/📡️component.protocol.semio"),
            include_str!("../🧬️mutations/📝️text/📖️component.grammar.semio"),
            include_str!("../🧬️mutations/📝️text/🔤️component.ebnf"),
            include_str!("../🧬️mutations/📝️text/🅰️component.g4"),
            include_str!("../🧬️mutations/💾️binary/📡️component.protocol.semio"),
            include_str!("../🧬️mutations/💾️binary/🌶️component.spicy"),
        ] {
            for forbidden in ["unknownBoxes", "unknown_boxes", "Mp4CodecOther", "ADD_UNKNOWN_BOX", "addUnknownBox", "nativeArchive", "sourceBytes", "serde_json::", "json_line", "jsonLine", "json_utf8", "JSON bytes", "iso-bmff-box-stream"] {
                assert!(!facet.contains(forbidden), "facet contains forbidden shadow concept {forbidden}");
            }
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn exact_fixture_survives_pack_and_dsl_codecs() {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../temp/bauen-mit-bestand.mp4");
        let bytes = std::fs::read(path).expect("read exact MP4 fixture");
        let snapshot = crate::artifacts::mp4::standards::isobmff::subsets::any::io::decode_mp4(&bytes).expect("decode exact MP4 fixture");

        let pack = <Mp4Snapshot as store::ArtifactPack>::encode_pack(&snapshot);
        let from_pack = <Mp4Snapshot as store::ArtifactPack>::decode_pack(&pack).expect("decode pack");
        assert_eq!(crate::artifacts::mp4::standards::isobmff::subsets::any::io::encode_mp4(&from_pack), bytes);

        let dsl = <Mp4Snapshot as store::ArtifactDsl>::print_dsl(&snapshot);
        let from_dsl = <Mp4Snapshot as store::ArtifactDsl>::parse_dsl(&dsl).expect("parse dsl");
        assert_eq!(crate::artifacts::mp4::standards::isobmff::subsets::any::io::encode_mp4(&from_dsl), bytes);
    }
}
//#endregion 🔖️Tests
