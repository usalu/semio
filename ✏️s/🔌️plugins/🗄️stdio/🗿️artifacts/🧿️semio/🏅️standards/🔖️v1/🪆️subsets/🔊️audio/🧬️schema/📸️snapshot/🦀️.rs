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
//!
//! ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION's audio wave replaces the old
//! hex-of-`serde_json` envelope passthrough with real hand-rolled text/binary codecs (this is a
//! NEUTRAL semio type, not itself an on-disk file format — real per-format bytes for wav/mp3 are
//! produced by the semio↔format `🚪️io` leaves).



use framework_schema::ArtifactSchema;

//#region 🔖️Ids
pub const STDIO_SEMIOAUDIO_DOCUMENT_SCHEMA: &str = "stdio.semio.audio";
//#endregion 🔖️Ids

//#region 🔖️Format
/// 🎚️ The sample format the audio was originally encoded in — metadata describing provenance,
/// independent of this snapshot's own always-`f32` sample storage (see module doc comment).
/// `wav`-shaped: mirrors PCM8/16/24/32 + IEEE float, the `fmt ` chunk's `wBitsPerSample`/
/// `wFormatTag` space, without depending on wav's own (future, W3) types — own type, per the
/// repo-wide "own types, not merged into a sibling format" convention (tsv-vs-csv, docx-vs-xlsx).
#[derive(Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, Default)]
#[value(rename_all = "camelCase")]
pub enum SemioAudioFormat {
    Pcm8,
    #[default]
    Pcm16,
    Pcm24,
    Pcm32,
    #[value(rename = "f32")]
    Float32,
    #[value(rename = "f64")]
    Float64,
}
//#endregion 🔖️Format

//#region 🔖️Channel
/// 🔊️ Owned by the `audio` subset (per `w1b-type-ownership.md`). One channel's full, decoded
/// sample sequence — a strong, per-field-diffable entity (today one field, `samples`, but kept as
/// its own struct + collection triple rather than `Vec<Vec<f32>>` so a future field, e.g. a
/// per-channel gain/pan, slots in without reshaping the collection).
#[derive(Clone, Debug, PartialEq, Default, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct SemioAudioChannel {
    #[value(default)]
    pub samples: Vec<f32>,
}
//#endregion 🔖️Channel

//#region 🔖️Tag
/// 🏷️ One metadata key/value pair (ID3/RIFF `LIST INFO`-shaped: `title`, `artist`, `comment`, …).
/// A weak/value entity per the recipe (its "diff" is the whole new pair, never sub-diffed).
#[derive(Clone, Debug, PartialEq, Default, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct SemioAudioTag {
    pub key: String,
    pub value: String,
}
//#endregion 🔖️Tag

//#region 🔖️Snapshot
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.audio")]
pub struct SemioAudioSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub sample_rate: u32,
    #[state(artifact)]
    #[value(default)]
    pub format: SemioAudioFormat,
    #[state(artifact)]
    #[value(default)]
    pub channels: Vec<SemioAudioChannel>,
    #[state(artifact)]
    #[value(default)]
    pub tags: Vec<SemioAudioTag>,
}

impl Default for SemioAudioSnapshot {
    fn default() -> Self {
        Self { schema: STDIO_SEMIOAUDIO_DOCUMENT_SCHEMA.into(), sample_rate: 0, format: SemioAudioFormat::default(), channels: Vec::new(), tags: Vec::new() }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️TextPrimitives




















//#endregion 🔖️TextPrimitives

//#region 🔖️BinaryPrimitives











//#endregion 🔖️BinaryPrimitives

//#region 🔖️HandcraftedArtifactCodecs


//#region 🔖️DslFreeFunctions



//#endregion 🔖️DslFreeFunctions


//#endregion 🔖️HandcraftedArtifactCodecs

//#region 🔖️Demo
/// 🌱 The demo `stdio.semio.audio` document — two channels (a short sweep each), a non-default
/// sample format, and one metadata tag — exercising every leaf/collection shape at least once.
/// Single source of truth for `📚️examples/…/🖼️assets/🗣️.dsl.semio`/`🎒️.pack.semio`
/// and for the conformance-law tests in `🎹️composer/🦀️.rs`.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_audio_snapshot() -> SemioAudioSnapshot {
    SemioAudioSnapshot {
        schema: STDIO_SEMIOAUDIO_DOCUMENT_SCHEMA.into(),
        sample_rate: 44_100,
        format: SemioAudioFormat::Float32,
        channels: vec![SemioAudioChannel { samples: vec![0.0, 0.5, -0.5, 1.0] }, SemioAudioChannel { samples: vec![0.0, -0.5, 0.5, -1.0] }],
        tags: vec![SemioAudioTag { key: "title".into(), value: "test tone".into() }],
    }
}
//#endregion 🔖️Demo

//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests






