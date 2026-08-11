//! 🧬️ SemioVideoSnapshot — streams{kind, codec, width, height, rate:Rational, samples{pts, key,
//! opaque data}} — container-typed, payload-opaque (honest boundary per the master plan: real,
//! complete metadata for this subset's own shape; the compressed sample bytes themselves are
//! never decoded here — that is W3/W4's container-format job, mp4/avi).

use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️VideoModel
/// 🎞️ Owned by the `video` subset (per `w1b-type-ownership.md`): `SemioVideoStream`,
/// `SemioVideoSample`, plus this subset's own `SemioVideoStreamKind`/`SemioRational` (not shared
/// engine types — `Rational` is video-specific, unlike `SemioPoint3`/`SemioTransform` etc).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SemioVideoStreamKind {
    #[default]
    Video,
    Audio,
    Subtitle,
}

/// 🎚️ A frame/sample rate as an exact fraction — named struct, never a bare tuple (f6-final-summary.md
/// §4.3: `dsl` has no blanket `DslField` impl for tuples of any arity).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SemioRational {
    pub num: i64,
    pub den: i64,
}

impl Default for SemioRational {
    /// 🎯️ `1/1`, not `0/0` — a rational's denominator must never default to zero.
    fn default() -> Self {
        Self { num: 1, den: 1 }
    }
}

/// 🎯️ One decoded/encoded unit within a stream. `data` is the format's opaque compressed payload
/// (honest boundary — never decoded by this subset).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SemioVideoSample {
    pub pts: u64,
    #[serde(default)]
    pub key: bool,
    #[serde(default)]
    pub data: Vec<u8>,
}

/// 🎞️ One elementary stream (video/audio/subtitle track) inside the container.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SemioVideoStream {
    #[serde(default)]
    pub kind: SemioVideoStreamKind,
    #[serde(default)]
    pub codec: String,
    #[serde(default)]
    pub width: u32,
    #[serde(default)]
    pub height: u32,
    #[serde(default)]
    pub rate: SemioRational,
    #[serde(default)]
    pub samples: Vec<SemioVideoSample>,
}
//#endregion 🔖️VideoModel

//#region 🔖️Ids
pub const STDIO_SEMIOVIDEO_DOCUMENT_SCHEMA: &str = "stdio.semio.video";
//#endregion 🔖️Ids

//#region 🔖️Snapshot
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.video")]
pub struct SemioVideoSnapshot {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    #[serde(default)]
    pub streams: Vec<SemioVideoStream>,
}

impl Default for SemioVideoSnapshot {
    fn default() -> Self {
        Self {
            schema: STDIO_SEMIOVIDEO_DOCUMENT_SCHEMA.into(),
            streams: Default::default(),
        }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️HandcraftedArtifactCodecs
/// 🧬️ JSON-pack round trip wrapped in the repo-wide `store::semio_format` envelope (the same
/// convention every neutral semio-subset snapshot uses — this subset's snapshot is not itself an
/// on-disk file format, so there is no bespoke binary layout to hand-roll here; the honest
/// per-field structure lives in the `SemioVideoDiff`/`SemioVideoMutation` grammars instead, which
/// ARE hand-rolled below).
impl store::ArtifactDsl for SemioVideoSnapshot {
    const EXTENSION: &'static str = "semio";
    fn envelope_id() -> &'static str { STDIO_SEMIOVIDEO_DOCUMENT_SCHEMA }

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
        serde_json::from_slice(&bytes).map_err(|e| store::TextError::new(format!("json decode: {e}"), dsl::TextSpan::at(1, 1)))
    }

    fn print_dsl(&self) -> String {
        let bytes = serde_json::to_vec(self).unwrap_or_default();
        let body: String = bytes.iter().map(|b| format!("{b:02x}")).collect();
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        ).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for SemioVideoSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = serde_json::to_vec(self).map_err(|e| store::PackError::Schema(e.to_string()))?;
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
        serde_json::from_slice(&inner).map_err(|e| store::PackError::Schema(e.to_string()))
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn sample_snapshot() -> SemioVideoSnapshot {
        SemioVideoSnapshot {
            schema: STDIO_SEMIOVIDEO_DOCUMENT_SCHEMA.into(),
            streams: vec![
                SemioVideoStream {
                    kind: SemioVideoStreamKind::Video,
                    codec: "h264".into(),
                    width: 1920,
                    height: 1080,
                    rate: SemioRational { num: 30, den: 1 },
                    samples: vec![SemioVideoSample { pts: 0, key: true, data: vec![1, 2, 3] }],
                },
                SemioVideoStream {
                    kind: SemioVideoStreamKind::Audio,
                    codec: "aac".into(),
                    width: 0,
                    height: 0,
                    rate: SemioRational { num: 48_000, den: 1_000 },
                    samples: Vec::new(),
                },
            ],
        }
    }

    #[test]
    fn json_pack_round_trips() {
        let snap = sample_snapshot();
        let bytes = <SemioVideoSnapshot as store::ArtifactPack>::encode_pack(&snap);
        let back = <SemioVideoSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(snap, back);
    }

    #[test]
    fn dsl_text_round_trips() {
        let snap = sample_snapshot();
        let text = <SemioVideoSnapshot as store::ArtifactDsl>::print_dsl(&snap);
        let back = <SemioVideoSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
        assert_eq!(snap, back);
    }

    #[test]
    fn stream_kind_defaults_to_video_and_rational_defaults_to_one_over_one() {
        assert_eq!(SemioVideoStreamKind::default(), SemioVideoStreamKind::Video);
        assert_eq!(SemioRational::default(), SemioRational { num: 1, den: 1 });
    }
}
//#endregion 🔖️Tests
