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

use crate::standards::v1::subsets::animation::schema::snapshot::SemioAnimationSnapshot;
use semio_framework_plugin::{ArtifactSerializer, Dialect, StandardId, SubsetId};
use semio_s_artifact_stdio_mp4::standards::isobmff::subsets::any::schema::snapshot::{Mp4Codec, Mp4Ftyp, Mp4Sample, Mp4Track};
use semio_s_artifact_stdio_mp4::Mp4Snapshot;

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
        let keyframes = from.timelines.first().and_then(|t| t.channels.first()).map_or(&[][..], |c| c.keyframes.as_slice());
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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests
