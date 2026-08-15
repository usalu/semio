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

use crate::artifacts::mp4::standards::isobmff::subsets::any::schema::snapshot::{Mp4Codec, Mp4Ftyp, Mp4Sample, Mp4Track};
use crate::artifacts::mp4::Mp4Snapshot;
use crate::artifacts::semio::standards::v1::subsets::video::schema::snapshot::SemioVideoSnapshot;
use semio_framework_plugin::{ArtifactSerializer, Dialect, StandardId, SubsetId};

const FROM_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("video") };
const INTO_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.mp4", standard: StandardId("isobmff"), subset: SubsetId("*") };

pub struct SemioVideoToMp4;

impl ArtifactSerializer for SemioVideoToMp4 {
    type From = SemioVideoSnapshot;
    type Into = Mp4Snapshot;
    const FROM: Dialect = FROM_DIALECT;
    const INTO: Dialect = INTO_DIALECT;

    fn serialize(from: &Self::From) -> Result<Self::Into, store::PackError> {
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
mod tests {
    use super::*;
    use crate::artifacts::semio::standards::v1::subsets::video::io::mp4_deserializer::SemioVideoFromMp4;
    use crate::artifacts::semio::standards::v1::subsets::video::schema::snapshot::{SemioRational, SemioVideoSample, SemioVideoStream, SemioVideoStreamKind, STDIO_SEMIOVIDEO_DOCUMENT_SCHEMA};
    use semio_framework_plugin::ArtifactDeserializer;

    fn real_world_video() -> SemioVideoSnapshot {
        SemioVideoSnapshot {
            schema: STDIO_SEMIOVIDEO_DOCUMENT_SCHEMA.into(),
            streams: vec![SemioVideoStream {
                kind: SemioVideoStreamKind::Video,
                codec: "avc1".into(),
                width: 640,
                height: 480,
                rate: SemioRational { num: 30, den: 1 },
                samples: vec![SemioVideoSample { pts: 0, key: true, data: vec![1, 2, 3] }, SemioVideoSample { pts: 1, key: false, data: vec![4, 5] }, SemioVideoSample { pts: 2, key: false, data: vec![6] }],
            }],
        }
    }

    /// 🧪️ codec_retention_law-style round trip FROM the semio side: video -> mp4 -> video must be
    /// a clean fixpoint (everything `video` can represent survives), even though mp4 -> video ->
    /// mp4 is documented-lossy (sps/pps/cts_offset) and therefore not the direction under test.
    #[test]
    fn video_to_mp4_to_video_round_trips_everything_the_video_subset_can_represent() {
        let original = real_world_video();
        let mp4 = SemioVideoToMp4::serialize(&original).expect("serialize");
        assert_eq!(mp4.tracks.len(), 1);
        assert_eq!(mp4.tracks[0].timescale, 30);
        assert_eq!(mp4.tracks[0].width, 640);
        assert_eq!(mp4.tracks[0].height, 480);
        let back = SemioVideoFromMp4::deserialize(&mp4).expect("deserialize");
        assert_eq!(back, original);
    }

    #[test]
    fn codec_name_longer_than_four_bytes_is_truncated_not_panicking() {
        let mut snap = real_world_video();
        snap.streams[0].codec = "hevc-main10".into();
        let mp4 = SemioVideoToMp4::serialize(&snap).expect("serialize");
        assert_eq!(mp4.tracks[0].codec.nal_length_size, 4);
    }

    #[test]
    fn empty_stream_list_serializes_to_zero_tracks() {
        let snap = SemioVideoSnapshot { schema: STDIO_SEMIOVIDEO_DOCUMENT_SCHEMA.into(), streams: Vec::new() };
        let mp4 = SemioVideoToMp4::serialize(&snap).expect("serialize");
        assert!(mp4.tracks.is_empty());
    }
}
//#endregion 🔖️Tests
