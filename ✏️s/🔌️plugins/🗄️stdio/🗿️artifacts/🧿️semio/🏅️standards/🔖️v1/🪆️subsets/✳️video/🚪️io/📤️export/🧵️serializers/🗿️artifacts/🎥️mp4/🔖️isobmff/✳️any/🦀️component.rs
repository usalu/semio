//! 📤️ Serialize `s.stdio.semio` (v1/video) into `s.stdio.mp4` (isobmff/✳️any) — mirror image of
//! this pair's deserializer: one `Mp4Track` per `SemioVideoStream`, `timescale = rate.num`, each
//! sample's `duration` derived from the delta to the NEXT keyframe's `pts` (the last sample falls
//! back to `rate.den`, the subset's own nominal per-sample duration) — `cts_offset` is always `0`
//! on export since `SemioVideoSample` carries a single `pts`, not a decode/presentation pair (a
//! real, honest, documented simplification: this loses any true B-frame reordering a real mp4
//! encode would need, matching the master plan's "direct reshape of typed metadata" framing, not a
//! byte-identical codec round trip).
//!
//! `SemioVideoStream.codec` is always written back as `Mp4Codec::Other{fourcc, raw: vec![]}` —
//! this bridge never fabricates AVC `sps`/`pps`/`nal_length_size` structured codec config (that
//! data was never captured on decode; see the deserializer's own doc comment), so an AVC track
//! round-tripped through `video` loses its codec-config box contents, keeping only its fourcc name.

use semio_framework_plugin::{ArtifactSerializer, Dialect, StandardId, SubsetId};
use crate::artifacts::mp4::Mp4Snapshot;
use crate::artifacts::mp4::standards::isobmff::subsets::any::schema::snapshot::{Mp4Codec, Mp4Ftyp, Mp4Sample, Mp4Track};
use crate::artifacts::semio::standards::v1::subsets::video::schema::snapshot::SemioVideoSnapshot;

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
                let codec = Mp4Codec::Other { fourcc: normalize_fourcc(&stream.codec), raw: Vec::new() };
                let fallback_duration = stream.rate.den.max(1) as u32;
                let samples = stream
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
                Mp4Track { track_id: (i + 1) as u32, timescale: stream.rate.num.max(1) as u32, codec, width: stream.width, height: stream.height, samples }
            })
            .collect();
        Ok(Mp4Snapshot {
            schema: "stdio.mp4".into(),
            ftyp: Mp4Ftyp { major_brand: "isom".into(), minor_version: 0, compatible_brands: Vec::new() },
            tracks,
            unknown_boxes: Vec::new(),
        })
    }
}

/// 🔤️ `Mp4Box`/`Mp4Codec::Other.fourcc` is meant to hold a 4-byte ISO-BMFF box type; a
/// video-subset codec name shorter/longer than 4 ASCII bytes is padded with spaces / truncated so
/// the written snapshot stays a structurally plausible fourcc (never silently dropped -- the full
/// original string, if longer, is impossible to preserve exactly here, a real, documented
/// truncation boundary of this specific typed field, not of the bridge as a whole).
fn normalize_fourcc(codec: &str) -> String {
    let mut out: String = codec.chars().take(4).collect();
    while out.len() < 4 {
        out.push(' ');
    }
    out
}

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::semio::standards::v1::subsets::video::schema::snapshot::{
        SemioRational, SemioVideoSample, SemioVideoStream, SemioVideoStreamKind, STDIO_SEMIOVIDEO_DOCUMENT_SCHEMA,
    };
    use crate::artifacts::semio::standards::v1::subsets::video::io::mp4_deserializer::SemioVideoFromMp4;
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
                samples: vec![
                    SemioVideoSample { pts: 0, key: true, data: vec![1, 2, 3] },
                    SemioVideoSample { pts: 1, key: false, data: vec![4, 5] },
                    SemioVideoSample { pts: 2, key: false, data: vec![6] },
                ],
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
        match &mp4.tracks[0].codec {
            Mp4Codec::Other { fourcc, .. } => assert_eq!(fourcc, "hevc"),
            other => panic!("expected Other codec, got {other:?}"),
        }
    }

    #[test]
    fn empty_stream_list_serializes_to_zero_tracks() {
        let snap = SemioVideoSnapshot { schema: STDIO_SEMIOVIDEO_DOCUMENT_SCHEMA.into(), streams: Vec::new() };
        let mp4 = SemioVideoToMp4::serialize(&snap).expect("serialize");
        assert!(mp4.tracks.is_empty());
    }
}
//#endregion 🔖️Tests
