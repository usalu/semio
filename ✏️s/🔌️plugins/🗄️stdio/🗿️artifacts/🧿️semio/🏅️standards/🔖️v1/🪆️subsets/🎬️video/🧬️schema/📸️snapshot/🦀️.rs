//! 🧬️ SemioVideoSnapshot — streams{kind, codec, width, height, rate:Rational, samples{pts, key,
//! opaque data}} — container-typed, payload-opaque (honest boundary per the master plan: real,
//! complete metadata for this subset's own shape; the compressed sample bytes themselves are
//! never decoded here — that is W3/W4's container-format job, mp4/avi).
//!
//! 🧩️ `#[derive(dsl::DslArtifact)]` was tried first per this ticket's brief. Blocked the same way
//! image's own bare-`Option<Vec<u8>>`-on-the-snapshot gap generalizes: `SemioVideoStream.samples:
//! Vec<SemioVideoSample>` nests a `Vec<u8>` buffer field (`data`) inside a `Vec<T>`-of-struct field
//! (`streams`) — the derive's `#[dsl(table)]`/`Vec<Record>` support (confirmed by reading the
//! framework's `SceneDocument`/`TableDocument` worked examples) covers one level of id-keyed
//! `Vec<Record>`; it has no tested path for a nested buffer-bearing leaf record two collections
//! deep, the SAME `derive-nested-multi-buffer-record` wall mesh's own report first named. Hand-rolled
//! instead — see this wave's report `mechanism_gaps`.



use framework_schema::ArtifactSchema;

//#region 🔖️VideoModel
/// 🎞️ Owned by the `video` subset (per `w1b-type-ownership.md`): `SemioVideoStream`,
/// `SemioVideoSample`, plus this subset's own `SemioVideoStreamKind`/`SemioRational` (not shared
/// engine types — `Rational` is video-specific, unlike `SemioPoint3`/`SemioTransform` etc).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub enum SemioVideoStreamKind {
    #[default]
    Video,
    Audio,
    Subtitle,
}

/// 🎚️ A frame/sample rate as an exact fraction — named struct, never a bare tuple (f6-final-summary.md
/// §4.3: `dsl` has no blanket `DslField` impl for tuples of any arity).
#[derive(Clone, Copy, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
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
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct SemioVideoSample {
    pub pts: u64,
    #[value(default)]
    pub key: bool,
    #[value(default)]
    pub data: Vec<u8>,
}

/// 🎞️ One elementary stream (video/audio/subtitle track) inside the container.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct SemioVideoStream {
    #[value(default)]
    pub kind: SemioVideoStreamKind,
    #[value(default)]
    pub codec: String,
    #[value(default)]
    pub width: u32,
    #[value(default)]
    pub height: u32,
    #[value(default)]
    pub rate: SemioRational,
    #[value(default)]
    pub samples: Vec<SemioVideoSample>,
}
//#endregion 🔖️VideoModel

//#region 🔖️Ids
pub const STDIO_SEMIOVIDEO_DOCUMENT_SCHEMA: &str = "stdio.semio.video";
//#endregion 🔖️Ids

//#region 🔖️Snapshot
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.video")]
pub struct SemioVideoSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[value(default)]
    pub streams: Vec<SemioVideoStream>,
}

impl Default for SemioVideoSnapshot {
    fn default() -> Self {
        Self { schema: STDIO_SEMIOVIDEO_DOCUMENT_SCHEMA.into(), streams: Default::default() }
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
/// 🌱 The demo `s.stdio.semio.video` document — 2 streams (one video w/ 2 samples incl. a key
/// frame, one audio w/ no samples), exercising every leaf shape at least once. Single source of
/// truth for `📚️examples/…/🖼️assets/🗣️.dsl.semio`/`🎒️.pack.semio` and for the
/// conformance-law tests in `🎹️composer/🦀️.rs`.
#[cfg(all(test, feature = "conversion-video"))]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_video_snapshot() -> SemioVideoSnapshot {
    SemioVideoSnapshot {
        schema: STDIO_SEMIOVIDEO_DOCUMENT_SCHEMA.into(),
        streams: vec![
            SemioVideoStream {
                kind: SemioVideoStreamKind::Video,
                codec: "h264".into(),
                width: 1920,
                height: 1080,
                rate: SemioRational { num: 30, den: 1 },
                samples: vec![SemioVideoSample { pts: 0, key: true, data: vec![0x00, 0x01, 0x02, 0x03] }, SemioVideoSample { pts: 33, key: false, data: vec![0x04, 0x05] }],
            },
            SemioVideoStream { kind: SemioVideoStreamKind::Audio, codec: "aac".into(), width: 0, height: 0, rate: SemioRational { num: 48_000, den: 1_000 }, samples: Vec::new() },
        ],
    }
}
//#endregion 🔖️Demo

//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests






