//! 📤️ Serialize `s.stdio.semio` (v1/animation) into `s.stdio.mp4` (isobmff/✳️any) — the MINIMAL,
//! documented reverse of this pair's deserializer (master plan: "map minimally and document").
//! `animation` is a multi-timeline, multi-channel model; mp4 is fundamentally single-track-timing
//! per elementary stream, a genuine cardinality mismatch. This bridge uses ONLY the FIRST
//! timeline's FIRST channel's `keyframes` (in order) as the master sample timing for ONE synthetic
//! track -- every other timeline/channel is honestly dropped (documented here, not silently).
//!
//! The produced track can NEVER carry real decodable video: `animation` has no pixel/codec payload
//! at all, so every `Mp4Sample.data` is empty, `width`/`height` are `0`, and `codec` is a fixed
//! `Other{fourcc:"anim "}` marker -- this is a structurally-valid container capturing ONLY real
//! timing, never a fabricated playable video, matching the ticket's "honest boundary" rule.

use crate::artifacts::mp4::standards::isobmff::subsets::any::schema::snapshot::{Mp4Codec, Mp4Ftyp, Mp4Sample, Mp4Track};
use crate::artifacts::mp4::Mp4Snapshot;
use crate::artifacts::semio::standards::v1::subsets::animation::schema::snapshot::SemioAnimationSnapshot;
use semio_framework_plugin::{ArtifactSerializer, Dialect, StandardId, SubsetId};

const FROM_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("animation") };
const INTO_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.mp4", standard: StandardId("isobmff"), subset: SubsetId("*") };

/// ⏱️ Fixed millisecond timescale for the synthetic track -- `animation`'s `t` is float seconds
/// with no implied rate of its own, so a base must be chosen; 1000 (ms) keeps sub-frame timing
/// exact for any realistic keyframe spacing without needing per-channel rate metadata this subset
/// doesn't carry.
const SYNTHETIC_TIMESCALE: u32 = 1000;

pub struct SemioAnimationToMp4;

impl ArtifactSerializer for SemioAnimationToMp4 {
    type From = SemioAnimationSnapshot;
    type Into = Mp4Snapshot;
    const FROM: Dialect = FROM_DIALECT;
    const INTO: Dialect = INTO_DIALECT;

    async fn serialize(from: &Self::From) -> Result<Self::Into, store::PackError> {
        let keyframes = from.timelines.first().and_then(|t| t.channels.first()).map(|c| c.keyframes.as_slice()).unwrap_or(&[]);
        let samples: Vec<Mp4Sample> = keyframes
            .iter()
            .enumerate()
            .map(|(i, k)| {
                let duration = match keyframes.get(i + 1) {
                    Some(next) => ((next.t - k.t) * SYNTHETIC_TIMESCALE as f64).round().max(1.0) as u32,
                    None => SYNTHETIC_TIMESCALE / 10,
                };
                Mp4Sample { data: Vec::new(), duration, cts_offset: 0, sync: true }
            })
            .collect();
        let tracks = if samples.is_empty() {
            Vec::new()
        } else {
            vec![Mp4Track { track_id: 1, timescale: SYNTHETIC_TIMESCALE, codec: Mp4Codec::default(), width: 0, height: 0, metadata: Default::default(), chunk_sample_counts: vec![samples.len() as u32], samples }]
        };
        Ok(Mp4Snapshot { schema: "stdio.mp4".into(), ftyp: Mp4Ftyp { major_brand: "isom".into(), minor_version: 0, compatible_brands: Vec::new() }, movie: Default::default(), tracks })
    }
}

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::semio::standards::v1::subsets::animation::schema::snapshot::{AnimChannel, AnimInterpolation, AnimKeyframe, AnimTarget, AnimTargetProperty, AnimTimeline, AnimValue, STDIO_SEMIOANIMATION_DOCUMENT_SCHEMA};

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn real_world_animation() -> SemioAnimationSnapshot {
        SemioAnimationSnapshot {
            schema: STDIO_SEMIOANIMATION_DOCUMENT_SCHEMA.into(),
            timelines: vec![AnimTimeline {
                name: Some("clip".into()),
                channels: vec![AnimChannel {
                    target: AnimTarget { node: "n".into(), property: AnimTargetProperty::Translation },
                    interpolation: AnimInterpolation::Linear,
                    keyframes: vec![AnimKeyframe { t: 0.0, value: AnimValue::Scalar { value: 0.0 } }, AnimKeyframe { t: 0.5, value: AnimValue::Scalar { value: 1.0 } }, AnimKeyframe { t: 1.0, value: AnimValue::Scalar { value: 2.0 } }],
                }],
            }],
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn serialize_builds_one_synthetic_track_with_real_derived_durations() {
        let mp4 = semio_framework_plugin::resolve_ready(SemioAnimationToMp4::serialize(&real_world_animation())).expect("serialize");
        assert_eq!(mp4.tracks.len(), 1);
        assert_eq!(mp4.tracks[0].timescale, SYNTHETIC_TIMESCALE);
        assert_eq!(mp4.tracks[0].samples.len(), 3);
        assert_eq!(mp4.tracks[0].samples[0].duration, 500); // 0.5s * 1000
        assert_eq!(mp4.tracks[0].samples[1].duration, 500);
        assert!(mp4.tracks[0].samples.iter().all(|s| s.data.is_empty()), "no fabricated frame bytes");
    }

    #[semio_framework_async_macros::async_test]
    async fn empty_animation_serializes_to_zero_tracks() {
        let snap = SemioAnimationSnapshot { schema: STDIO_SEMIOANIMATION_DOCUMENT_SCHEMA.into(), timelines: Vec::new() };
        let mp4 = semio_framework_plugin::resolve_ready(SemioAnimationToMp4::serialize(&snap)).expect("serialize");
        assert!(mp4.tracks.is_empty());
    }
}
//#endregion 🔖️Tests
