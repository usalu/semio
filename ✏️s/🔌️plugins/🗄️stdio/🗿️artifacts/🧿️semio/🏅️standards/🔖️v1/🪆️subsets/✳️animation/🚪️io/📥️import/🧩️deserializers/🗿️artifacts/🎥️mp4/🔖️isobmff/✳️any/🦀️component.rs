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

use semio_framework_plugin::{ArtifactDeserializer, Dialect, StandardId, SubsetId};
use crate::artifacts::mp4::Mp4Snapshot;
use crate::artifacts::semio::standards::v1::subsets::animation::schema::snapshot::{
    AnimChannel, AnimInterpolation, AnimKeyframe, AnimTarget, AnimTargetProperty, AnimTimeline, AnimValue, SemioAnimationSnapshot, STDIO_SEMIOANIMATION_DOCUMENT_SCHEMA,
};

const FROM_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.mp4", standard: StandardId("isobmff"), subset: SubsetId("*") };
const INTO_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("animation") };

pub struct SemioAnimationFromMp4;

impl ArtifactDeserializer for SemioAnimationFromMp4 {
    type From = Mp4Snapshot;
    type Into = SemioAnimationSnapshot;
    const FROM: Dialect = FROM_DIALECT;
    const INTO: Dialect = INTO_DIALECT;

    fn deserialize(from: &Self::From) -> Result<Self::Into, store::PackError> {
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
                AnimTimeline {
                    name: Some(node.clone()),
                    channels: vec![AnimChannel { target: AnimTarget { node, property: AnimTargetProperty::Custom { name: "mp4SampleIndex".into() } }, interpolation: AnimInterpolation::Step, keyframes }],
                }
            })
            .collect();
        Ok(SemioAnimationSnapshot { schema: STDIO_SEMIOANIMATION_DOCUMENT_SCHEMA.into(), timelines })
    }
}

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::mp4::standards::isobmff::subsets::any::schema::snapshot::{Mp4Codec, Mp4Ftyp, Mp4Sample, Mp4Track};

    fn real_world_mp4() -> Mp4Snapshot {
        Mp4Snapshot {
            schema: "stdio.mp4".into(),
            ftyp: Mp4Ftyp { major_brand: "isom".into(), minor_version: 0, compatible_brands: vec![] },
            movie: Default::default(),
            tracks: vec![Mp4Track {
                track_id: 1,
                timescale: 30,
                codec: Mp4Codec::Other { fourcc: "avc1".into(), raw: vec![] },
                width: 640,
                height: 480,
                metadata: Default::default(),
                chunk_sample_counts: vec![2],
                samples: vec![
                    Mp4Sample { data: vec![1], duration: 30, cts_offset: 0, sync: true },
                    Mp4Sample { data: vec![2], duration: 30, cts_offset: 0, sync: false },
                ],
            }],
            unknown_boxes: vec![],
        }
    }

    #[test]
    fn deserialize_exposes_real_sample_timing_as_a_step_scalar_channel() {
        let anim = SemioAnimationFromMp4::deserialize(&real_world_mp4()).expect("deserialize");
        assert_eq!(anim.timelines.len(), 1);
        assert_eq!(anim.timelines[0].name.as_deref(), Some("track-1"));
        let ch = &anim.timelines[0].channels[0];
        assert_eq!(ch.target.node, "track-1");
        assert_eq!(ch.interpolation, AnimInterpolation::Step);
        assert_eq!(ch.keyframes.len(), 2);
        assert_eq!(ch.keyframes[0].t, 0.0); // dts=0 / timescale 30
        assert_eq!(ch.keyframes[1].t, 1.0); // dts=30 / timescale 30 = 1s
        assert_eq!(ch.keyframes[0].value, AnimValue::Scalar { value: 0.0 });
        assert_eq!(ch.keyframes[1].value, AnimValue::Scalar { value: 1.0 });
    }
}
//#endregion 🔖️Tests
