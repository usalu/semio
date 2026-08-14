//! 🧬️ Mp4Snapshot — ISO-BMFF: `ftyp` typed, decoded per-track sample tables (`stts`/`ctts`/
//! `stsc`/`stsz`/`stco`/`stss` flattened into per-sample records), AVC codec config typed
//! (`avcC` SPS/PPS), everything else typed-raw retained (`unknown_boxes`) — never a fabricated
//! decode. Real binary codec: `ArtifactPack`/`ArtifactDsl` below wrap the REAL ISO-BMFF bytes
//! produced/consumed by `⚙️engine::{decode_mp4,encode_mp4}` (moved from remodel's video engine,
//! see that module's doc comment), the same pattern `stdio.png`'s snapshot uses — NOT a
//! JSON-pack passthrough.

use crate::artifacts::mp4::standards::isobmff::subsets::any::io as engine;
use crate::ArtifactSource;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Ids
pub const STDIO_MP4_DOCUMENT_SCHEMA: &str = "stdio.mp4";
//#endregion 🔖️Ids

//#region 🔖️Ftyp
/// 🏷️ File-type box: brand + compatible-brand list. <https://www.iso.org/standard/74428.html> §4.3
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mp4Ftyp {
    pub major_brand: String,
    pub minor_version: u32,
    #[serde(default)]
    pub compatible_brands: Vec<String>,
}
//#endregion 🔖️Ftyp

//#region 🔖️Codec
/// 🎥️ A track's sample-description codec: AVC typed (SPS/PPS NAL lists + AVCC length-field
/// width), anything else typed-raw (the full first sample-description entry box, verbatim —
/// honest boundary, never a fabricated decode of a codec this engine doesn't understand).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "codec", rename_all = "camelCase")]
pub enum Mp4Codec {
    Avc {
        #[serde(default)]
        sps: Vec<Vec<u8>>,
        #[serde(default)]
        pps: Vec<Vec<u8>>,
        nal_length_size: u8,
    },
    Other {
        fourcc: String,
        #[serde(default)]
        raw: Vec<u8>,
    },
}

impl Default for Mp4Codec {
    fn default() -> Self { Self::Other { fourcc: String::new(), raw: Vec::new() } }
}
//#endregion 🔖️Codec

//#region 🔖️Sample
/// 🎞️ One decoded sample: exact payload bytes (AVCC/length-prefixed as the container held them —
/// payload-opaque, matching the master plan's "video is container-typed, payload-opaque" call),
/// its `stts` duration in the track's timescale, its `ctts` composition-time offset, and whether
/// `stss` marks it a sync (random-access) sample (absent `stss` ⇒ every sample is sync, per spec).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mp4Sample {
    #[serde(default)]
    pub data: Vec<u8>,
    pub duration: u32,
    #[serde(default)]
    pub cts_offset: i32,
    pub sync: bool,
}
//#endregion 🔖️Sample

//#region 🔖️Track
/// 🛤️ One `trak` (this codec decodes video-handler tracks only — a non-`vide` `trak` is retained
/// whole in `unknown_boxes` under fourcc `"trak"`, never silently dropped).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mp4Track {
    pub track_id: u32,
    pub timescale: u32,
    pub codec: Mp4Codec,
    pub width: u32,
    pub height: u32,
    #[serde(default)]
    pub samples: Vec<Mp4Sample>,
}
//#endregion 🔖️Track

//#region 🔖️RawBox
/// 📦️ Typed-raw retention for any top-level box this codec doesn't otherwise type (`free`,
/// `uuid`, `skip`, a non-video `trak`, …) — verbatim fourcc + payload bytes, replayed at the same
/// relative position (right after `ftyp`, before `mdat`) on encode.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mp4Box {
    pub fourcc: String,
    #[serde(default)]
    pub data: Vec<u8>,
}
//#endregion 🔖️RawBox

//#region 🔖️Snapshot
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.mp4")]
pub struct Mp4Snapshot {
    #[state(artifact)]
    #[serde(default = "default_schema")]
    pub schema: String,
    #[state(artifact)]
    pub ftyp: Mp4Ftyp,
    #[state(artifact)]
    #[serde(default)]
    pub tracks: Vec<Mp4Track>,
    #[state(artifact)]
    #[serde(default)]
    pub unknown_boxes: Vec<Mp4Box>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<ArtifactSource>,
}

fn default_schema() -> String { STDIO_MP4_DOCUMENT_SCHEMA.into() }

impl Default for Mp4Snapshot {
    /// 🌱️ A minimal but real, 4-byte-brand `ftyp` — `major_brand` MUST be exactly 4 ASCII bytes
    /// for a genuinely valid box (unlike an empty string, which `⚙️engine::encode_mp4` would have
    /// to pad, breaking the empty-snapshot round trip below).
    fn default() -> Self {
        Self { schema: STDIO_MP4_DOCUMENT_SCHEMA.into(), ftyp: Mp4Ftyp { major_brand: "isom".into(), minor_version: 0, compatible_brands: Vec::new() }, tracks: Vec::new(), unknown_boxes: Vec::new(), source: None }
    }
}

impl Mp4Snapshot {
    /// 🪞️ Clones the semantic projection without its native source image.
    pub fn projection(&self) -> Self {
        let mut projection = self.clone();
        projection.source = None;
        projection
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️HandcraftedArtifactCodecs
/// 🎙️ Real binary codec: the `.semio` envelope wrapping the REAL ISO-BMFF bytes this engine
/// decodes/encodes (moved from remodel's video engine — see `⚙️engine`'s doc comment). Text form
/// is a whitespace-tolerant ASCII hex dump of those same real bytes (mirrors `stdio.png`'s
/// pattern exactly — PNG likewise has no textual syntax of its own).
impl store::ArtifactDsl for Mp4Snapshot {
    const EXTENSION: &'static str = "semio";
    fn envelope_id() -> &'static str { STDIO_MP4_DOCUMENT_SCHEMA }

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
            let byte = u8::from_str_radix(&hex[i..i + 2], 16)
                .map_err(|e| store::TextError::new(format!("invalid hex: {e}"), dsl::TextSpan::at(1, 1)))?;
            bytes.push(byte);
            i += 2;
        }
        engine::decode_mp4(&bytes).map_err(|e| store::TextError::new(format!("mp4 decode: {e}"), dsl::TextSpan::at(1, 1)))
    }

    fn print_dsl(&self) -> String {
        let bytes = engine::encode_mp4(self);
        let body: String = bytes.iter().map(|b| format!("{b:02x}")).collect();
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        ).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for Mp4Snapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = engine::encode_mp4(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Pack,
            1,
        ).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }

    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!(
                "pack envelope mismatch: expected {}, got {}",
                <Self as store::ArtifactDsl>::envelope_id(),
                envelope.envelope_id()
            )));
        }
        let _ = options;
        engine::decode_mp4(&inner).map_err(store::PackError::Schema)
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn sample_snapshot() -> Mp4Snapshot {
        Mp4Snapshot {
            schema: STDIO_MP4_DOCUMENT_SCHEMA.into(),
            ftyp: Mp4Ftyp { major_brand: "isom".into(), minor_version: 512, compatible_brands: vec!["isom".into(), "iso2".into(), "avc1".into(), "mp41".into()] },
            tracks: vec![Mp4Track {
                track_id: 1,
                timescale: 1000,
                codec: Mp4Codec::Avc { sps: vec![vec![0x67, 0x42, 0x00, 0x1E, 0x8C, 0x8D, 0x40]], pps: vec![vec![0x68, 0xCE, 0x3C, 0x80]], nal_length_size: 4 },
                width: 64,
                height: 64,
                samples: vec![
                    Mp4Sample { data: vec![0, 0, 0, 4, 0x65, 1, 2, 3], duration: 33, cts_offset: 0, sync: true },
                    Mp4Sample { data: vec![0, 0, 0, 3, 0x61, 4, 5], duration: 33, cts_offset: 33, sync: false },
                ],
            }],
            unknown_boxes: vec![Mp4Box { fourcc: "free".into(), data: vec![0, 0, 0, 0] }],
            source: None,
        }
    }

    #[test]
    fn json_pack_round_trips_via_real_mp4_bytes() {
        let snap = sample_snapshot();
        let bytes = <Mp4Snapshot as store::ArtifactPack>::encode_pack(&snap);
        let back = <Mp4Snapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(snap, back.projection());
    }

    #[test]
    fn dsl_text_round_trips_via_real_mp4_bytes() {
        let snap = sample_snapshot();
        let text = <Mp4Snapshot as store::ArtifactDsl>::print_dsl(&snap);
        let back = <Mp4Snapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
        assert_eq!(snap, back.projection());
    }

    #[test]
    fn default_snapshot_round_trips_through_real_codec() {
        let snap = Mp4Snapshot::default();
        let bytes = <Mp4Snapshot as store::ArtifactPack>::encode_pack(&snap);
        let back = <Mp4Snapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(snap, back.projection());
    }

    #[test]
    fn exact_fixture_survives_pack_and_dsl_codecs() {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../temp/bauen-mit-bestand.mp4");
        let bytes = std::fs::read(path).expect("read exact MP4 fixture");
        let snapshot = engine::decode_mp4(&bytes).expect("decode exact MP4 fixture");

        let pack = <Mp4Snapshot as store::ArtifactPack>::encode_pack(&snapshot);
        let from_pack = <Mp4Snapshot as store::ArtifactPack>::decode_pack(&pack).expect("decode pack");
        assert_eq!(engine::encode_mp4(&from_pack), bytes);

        let dsl = <Mp4Snapshot as store::ArtifactDsl>::print_dsl(&snapshot);
        let from_dsl = <Mp4Snapshot as store::ArtifactDsl>::parse_dsl(&dsl).expect("parse dsl");
        assert_eq!(engine::encode_mp4(&from_dsl), bytes);
    }
}
//#endregion 🔖️Tests
