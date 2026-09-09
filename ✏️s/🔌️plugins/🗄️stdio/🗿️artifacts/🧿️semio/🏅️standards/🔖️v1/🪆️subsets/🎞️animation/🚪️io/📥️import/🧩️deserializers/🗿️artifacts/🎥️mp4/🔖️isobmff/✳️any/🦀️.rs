//! 📥️ Deserialize `s.stdio.mp4` (isobmff/✳️any) into `s.stdio.semio` (v1/animation) — MINIMAL,
//! frame-sequence-only mapping per the master plan ("if mp4 has no keyframe/timeline concept map
//! minimally and document"): ISO-BMFF has no animation/transform-channel concept at all, only
//! sample TIMING (`stts` duration, `stss` sync flag). This bridge exposes that real timing as a
//! `Step`-interpolated `Custom`-property channel carrying the sample's own INDEX as its value --
//! honest (never invents a fake translation/rotation curve from opaque video bytes), but
//! deliberately thin: it is NOT a general "extract animation from video" feature.
//!
//! One `AnimTimeline` per `Mp4Track` (name `"track-<id>"`), one channel per track targeting node
//! `"track-<id>"` with `property: Custom{name:"mp4SampleIndex"}`. `t` is the same real
//! `dts + cts_offset` pts derivation `video↔mp4` uses, converted to SECONDS via `timescale`
//! (`pts_ticks / timescale`) so timelines from different tracks/timescales are directly comparable.

use crate::standards::v1::subsets::animation::schema::snapshot::{AnimChannel, AnimInterpolation, AnimKeyframe, AnimTarget, AnimTargetProperty, AnimTimeline, AnimValue, SemioAnimationSnapshot, STDIO_SEMIOANIMATION_DOCUMENT_SCHEMA};
use semio_framework_plugin::{ArtifactDeserializer, Dialect, StandardId, SubsetId};
use semio_s_artifact_stdio_mp4::Mp4Snapshot;

const FROM_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.mp4", standard: StandardId("isobmff"), subset: SubsetId("*") };
const INTO_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("animation") };

pub struct SemioAnimationFromMp4;

impl ArtifactDeserializer for SemioAnimationFromMp4 {
    type From = Mp4Snapshot;
    type Into = SemioAnimationSnapshot;
    const FROM: Dialect = FROM_DIALECT;
    const INTO: Dialect = INTO_DIALECT;

    async fn deserialize(from: &Self::From) -> Result<Self::Into, store::PackError> {
        let timelines = from
            .tracks
            .iter()
            .map(|track| {
                let timescale = track.timescale.max(1) as f64;
                let node = format!("track-{}", track.track_id);
                let mut dts: i64 = 0;
                let keyframes = track
                    .samples
                    .iter()
                    .enumerate()
                    .map(|(i, sample)| {
                        let pts = dts + sample.cts_offset as i64;
                        dts += sample.duration as i64;
                        AnimKeyframe { t: pts.max(0) as f64 / timescale, value: AnimValue::Scalar { value: i as f64 } }
                    })
                    .collect();
                AnimTimeline { name: Some(node.clone()), channels: vec![AnimChannel { target: AnimTarget { node, property: AnimTargetProperty::Custom { name: "mp4SampleIndex".into() } }, interpolation: AnimInterpolation::Step, keyframes }] }
            })
            .collect();
        Ok(SemioAnimationSnapshot { schema: STDIO_SEMIOANIMATION_DOCUMENT_SCHEMA.into(), timelines })
    }
}

//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests
