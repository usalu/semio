//! 🧬️ WavSnapshot — a typed `fmt ` chunk + typed `data` samples + any other RIFF chunk
//! verbatim. Real byte-accurate RIFF/WAVE codec (see `⚙️engine`), not a container placeholder.

/// 📦️ Owned by `wav`: the `fmt ` chunk's fields, typed. `ext` carries the extensible/non-PCM
/// tail (`cbSize` bytes) verbatim when present — `None` for the plain 16-byte PCM form. NO type
/// sharing with `avi` (both are RIFF-based but deliberately distinct vocabularies per the master
/// plan).
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WavFmt {
    pub audio_format: u16,
    pub channels: u16,
    pub sample_rate: u32,
    pub byte_rate: u32,
    pub block_align: u16,
    pub bits_per_sample: u16,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ext: Option<Vec<u8>>,
}

impl Default for WavFmt {
    fn default() -> Self {
        Self { audio_format: 1, channels: 1, sample_rate: 44_100, byte_rate: 88_200, block_align: 2, bits_per_sample: 16, ext: None }
    }
}

/// 📦️ Owned by `wav`: the `data` chunk's samples, typed per `WavFmt`'s
/// `(audio_format, bits_per_sample)` — `Raw` is the honest fallback for anything this codec
/// doesn't interpret sample-by-sample (24-bit PCM, ADPCM, WAVE_FORMAT_EXTENSIBLE payloads, …).
/// 🏷️ Adjacently tagged (`tag`+`content`), not purely internally tagged — serde cannot serialize
/// an internally-tagged newtype variant wrapping a non-map type (`Vec<T>` here), the same
/// constraint already on record for `HtmlNode`/`JsonValue` elsewhere in this codebase.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "kind", content = "value", rename_all = "camelCase")]
pub enum WavData {
    Pcm16(Vec<i16>),
    Pcm8(Vec<u8>),
    Float32(Vec<f32>),
    Raw(Vec<u8>),
}

impl Default for WavData {
    fn default() -> Self {
        WavData::Raw(Vec::new())
    }
}

/// 📦️ Owned by `wav`: any RIFF chunk other than `fmt `/`data`, retained byte-for-byte
/// (`LIST`/`INFO`/`fact`/`cue `/…).
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RiffChunk {
    pub fourcc: String,
    #[serde(default)]
    pub data: Vec<u8>,
}

use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Ids
pub const STDIO_WAV_DOCUMENT_SCHEMA: &str = "stdio.wav";
//#endregion 🔖️Ids

//#region 🔖️Snapshot
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.wav")]
pub struct WavSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub fmt: WavFmt,
    #[state(artifact)]
    pub data: WavData,
    #[state(artifact)]
    #[serde(default)]
    pub other_chunks: Vec<RiffChunk>,
}

impl Default for WavSnapshot {
    fn default() -> Self {
        Self { schema: STDIO_WAV_DOCUMENT_SCHEMA.into(), fmt: WavFmt::default(), data: WavData::default(), other_chunks: Default::default() }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️HandcraftedArtifactCodecs
/// 🎧️ `ArtifactDsl`/`ArtifactPack` route through the REAL RIFF/WAVE codec
/// (`⚙️engine::encode_wav`/`decode_wav`) — the envelope wraps genuine on-disk WAV bytes, the same
/// convention `BmpSnapshot`'s handcrafted codecs use (real format bytes inside the
/// `store::semio_format` envelope, not a JSON re-serialization of the Rust type).
impl store::ArtifactDsl for WavSnapshot {
    const EXTENSION: &'static str = "wav";
    fn envelope_id() -> &'static str {
        STDIO_WAV_DOCUMENT_SCHEMA
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
        crate::artifacts::wav::standards::riff_pcm::subsets::any::io::decode_wav(&bytes).map_err(|e| store::TextError::new(format!("wav decode: {e}"), dsl::TextSpan::at(1, 1)))
    }

    fn print_dsl(&self) -> String {
        let bytes = crate::artifacts::wav::standards::riff_pcm::subsets::any::io::encode_wav(self);
        let body: String = bytes.iter().map(|b| format!("{b:02x}")).collect();
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for WavSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = crate::artifacts::wav::standards::riff_pcm::subsets::any::io::encode_wav(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }

    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.envelope_id())));
        }
        let _ = options;
        crate::artifacts::wav::standards::riff_pcm::subsets::any::io::decode_wav(&inner).map_err(store::PackError::Schema)
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn sample_snapshot() -> WavSnapshot {
        WavSnapshot { data: WavData::Pcm16(vec![1, -1, 100, -100]), ..WavSnapshot::default() }
    }

    #[semio_framework_async_macros::async_test]
    async fn json_pack_round_trips() {
        let snap = sample_snapshot();
        let bytes = <WavSnapshot as store::ArtifactPack>::encode_pack(&snap);
        let back = <WavSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(snap, back);
    }

    #[semio_framework_async_macros::async_test]
    async fn dsl_text_round_trips() {
        let snap = sample_snapshot();
        let text = <WavSnapshot as store::ArtifactDsl>::print_dsl(&snap);
        let back = <WavSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
        assert_eq!(snap, back);
    }
}
//#endregion 🔖️Tests
