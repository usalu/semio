//! 📤️ Serialize `s.stdio.semio` (v1/video) into `s.stdio.mp4` (isobmff/✳️any) — mirror image of
//! this pair's deserializer: one `Mp4Track` per `SemioVideoStream`, `timescale = rate.num`, each
//! sample's `duration` derived from the delta to the NEXT keyframe's `pts` (the last sample falls
//! back to `rate.den`, the subset's own nominal per-sample duration) — `cts_offset` is always `0`
//! on export since `SemioVideoSample` carries a single `pts`, not a decode/presentation pair (a
//! real, honest, documented simplification: this loses any true B-frame reordering a real mp4
//! encode would need, matching the master plan's "direct reshape of typed metadata" framing, not a
//! byte-identical codec round trip).
//!
//! The bridge emits the supported logical AVC variant and never retains an opaque sample-entry box.

use crate::standards::v1::subsets::video::schema::snapshot::SemioVideoSnapshot;
use semio_framework_plugin::{ArtifactSerializer, Dialect, StandardId, SubsetId};
use semio_s_artifact_stdio_mp4::standards::isobmff::subsets::any::schema::snapshot::{Mp4Codec, Mp4Ftyp, Mp4Sample, Mp4Track};
use semio_s_artifact_stdio_mp4::Mp4Snapshot;

const FROM_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("video") };
const INTO_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.mp4", standard: StandardId("isobmff"), subset: SubsetId("*") };

pub struct SemioVideoToMp4;

impl ArtifactSerializer for SemioVideoToMp4 {
    type From = SemioVideoSnapshot;
    type Into = Mp4Snapshot;
    const FROM: Dialect = FROM_DIALECT;
    const INTO: Dialect = INTO_DIALECT;

    async fn serialize(from: &Self::From) -> Result<Self::Into, store::PackError> {
        let tracks = from
            .streams
            .iter()
            .enumerate()
            .map(|(i, stream)| {
                let codec = Mp4Codec::default();
                let fallback_duration = stream.rate.den.max(1) as u32;
                let samples: Vec<Mp4Sample> = stream
                    .samples
                    .iter()
                    .enumerate()
                    .map(|(j, sample)| {
                        let duration = match stream.samples.get(j + 1) {
                            Some(next) => next.pts.saturating_sub(sample.pts).max(1) as u32,
                            None => fallback_duration,
                        };
                        Mp4Sample { data: sample.data.clone(), duration, cts_offset: 0, sync: sample.key }
                    })
                    .collect();
                Mp4Track { track_id: (i + 1) as u32, timescale: stream.rate.num.max(1) as u32, codec, width: stream.width, height: stream.height, metadata: Default::default(), chunk_sample_counts: vec![samples.len() as u32], samples }
            })
            .collect();
        Ok(Mp4Snapshot { schema: "stdio.mp4".into(), ftyp: Mp4Ftyp { major_brand: "isom".into(), minor_version: 0, compatible_brands: Vec::new() }, movie: Default::default(), tracks })
    }
}

//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests
