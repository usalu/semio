//! 🔊️ SemioAudioSnapshot — complete per the master plan's `audio` row: `sample_rate` +
//! `format` (typed sample-format enum, describing the ORIGINAL encoding the samples were decoded
//! from) + `channels` (ordered, index-keyed — index 0 = left/mono, 1 = right, … matching wav's
//! interleaved-channel-order convention) + `tags` (ordered key/value metadata pairs, ID3/RIFF
//! `LIST INFO`-shaped — duplicate keys are legal on disk, hence a `Vec`, never a `BTreeMap`).
//! Per the ticket's honest-boundary note: audio is schema-complete for ITS OWN shape and stores
//! REAL decoded `f32` samples (unlike `video`, which is deliberately payload-opaque) — decoding a
//! compressed container's samples into this shape is a W3/W4 codec concern, not this subset's.
//! Owned types (see `w1b-type-ownership.md`): `SemioAudioSnapshot`, `SemioAudioChannel`. New this
//! wave: `SemioAudioFormat`, `SemioAudioTag` (the `tags` field was W1b-reserved, not yet defined).

use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Ids
pub const STDIO_SEMIOAUDIO_DOCUMENT_SCHEMA: &str = "stdio.semio.audio";
//#endregion 🔖️Ids

//#region 🔖️Format
/// 🎚️ The sample format the audio was originally encoded in — metadata describing provenance,
/// independent of this snapshot's own always-`f32` sample storage (see module doc comment).
/// `wav`-shaped: mirrors PCM8/16/24/32 + IEEE float, the `fmt ` chunk's `wBitsPerSample`/
/// `wFormatTag` space, without depending on wav's own (future, W3) types — own type, per the
/// repo-wide "own types, not merged into a sibling format" convention (tsv-vs-csv, docx-vs-xlsx).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum SemioAudioFormat {
    Pcm8,
    #[default]
    Pcm16,
    Pcm24,
    Pcm32,
    Float32,
    Float64,
}
//#endregion 🔖️Format

//#region 🔖️Channel
/// 🔊️ Owned by the `audio` subset (per `w1b-type-ownership.md`). One channel's full, decoded
/// sample sequence — a strong, per-field-diffable entity (today one field, `samples`, but kept as
/// its own struct + collection triple rather than `Vec<Vec<f32>>` so a future field, e.g. a
/// per-channel gain/pan, slots in without reshaping the collection).
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SemioAudioChannel {
    #[serde(default)]
    pub samples: Vec<f32>,
}
//#endregion 🔖️Channel

//#region 🔖️Tag
/// 🏷️ One metadata key/value pair (ID3/RIFF `LIST INFO`-shaped: `title`, `artist`, `comment`, …).
/// A weak/value entity per the recipe (its "diff" is the whole new pair, never sub-diffed).
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SemioAudioTag {
    pub key: String,
    pub value: String,
}
//#endregion 🔖️Tag

//#region 🔖️Snapshot
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.audio")]
pub struct SemioAudioSnapshot {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    pub sample_rate: u32,
    #[state(persistent)]
    #[serde(default)]
    pub format: SemioAudioFormat,
    #[state(persistent)]
    #[serde(default)]
    pub channels: Vec<SemioAudioChannel>,
    #[state(persistent)]
    #[serde(default)]
    pub tags: Vec<SemioAudioTag>,
}

impl Default for SemioAudioSnapshot {
    fn default() -> Self {
        Self {
            schema: STDIO_SEMIOAUDIO_DOCUMENT_SCHEMA.into(),
            sample_rate: 0,
            format: SemioAudioFormat::default(),
            channels: Vec::new(),
            tags: Vec::new(),
        }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️HandcraftedArtifactCodecs
/// 🧩️ JSON-pack round trip — honest and genuinely working (not a per-format binary codec, since
/// this subset's snapshot is a NEUTRAL semio type, not an on-disk file format; matches every other
/// semio subset's own `ArtifactDsl`/`ArtifactPack` convention). Wrapped in the same
/// `store::semio_format` envelope every stdio artifact uses.
impl store::ArtifactDsl for SemioAudioSnapshot {
    const EXTENSION: &'static str = "semio";
    fn envelope_id() -> &'static str { STDIO_SEMIOAUDIO_DOCUMENT_SCHEMA }

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

impl store::ArtifactPack for SemioAudioSnapshot {
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

    fn sample_snapshot() -> SemioAudioSnapshot {
        SemioAudioSnapshot {
            sample_rate: 44_100,
            format: SemioAudioFormat::Float32,
            channels: vec![
                SemioAudioChannel { samples: vec![0.0, 0.5, -0.5, 1.0] },
                SemioAudioChannel { samples: vec![0.0, -0.5, 0.5, -1.0] },
            ],
            tags: vec![SemioAudioTag { key: "title".into(), value: "test tone".into() }],
            ..SemioAudioSnapshot::default()
        }
    }

    #[test]
    fn json_pack_round_trips() {
        let snap = sample_snapshot();
        let bytes = <SemioAudioSnapshot as store::ArtifactPack>::encode_pack(&snap);
        let back = <SemioAudioSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(snap, back);
    }

    #[test]
    fn dsl_text_round_trips() {
        let snap = sample_snapshot();
        let text = <SemioAudioSnapshot as store::ArtifactDsl>::print_dsl(&snap);
        let back = <SemioAudioSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
        assert_eq!(snap, back);
    }

    #[test]
    fn default_snapshot_has_no_channels_or_tags() {
        let snap = SemioAudioSnapshot::default();
        assert!(snap.channels.is_empty());
        assert!(snap.tags.is_empty());
        assert_eq!(snap.format, SemioAudioFormat::Pcm16);
    }
}
//#endregion 🔖️Tests
