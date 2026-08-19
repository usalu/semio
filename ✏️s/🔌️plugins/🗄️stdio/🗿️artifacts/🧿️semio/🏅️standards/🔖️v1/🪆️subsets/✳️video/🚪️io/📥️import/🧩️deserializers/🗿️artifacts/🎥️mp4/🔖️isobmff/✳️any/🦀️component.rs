//! 📥️ Deserialize `s.stdio.mp4` (isobmff/✳️any) into `s.stdio.semio` (v1/video) — a direct
//! reshape of ISO-BMFF track/sample metadata onto video's container-typed, payload-opaque stream
//! model (master plan: "close to a direct reshape of typed metadata, not a pixel/codec-level
//! operation"). Every `Mp4Track` decodes to one `SemioVideoStream` of `kind: Video` (this codec
//! only ever types video-handler tracks — see `Mp4Track`'s own doc comment — audio/other tracks
//! are rejected by the native MP4 deserializer before this reshape).
//!
//! Honest, documented lossy fields (real, unavoidable — never fabricated):
//! - `Mp4Codec{sps,pps,nal_length_size}` collapses to the plain codec name string `"avc1"` —
//!   `SemioVideoStream.codec` has no slot for structured codec-config bytes.
//! - `Mp4Sample.cts_offset` (composition-time offset) is folded into the derived `pts` (`pts = dts
//!   + cts_offset`, `dts` = running sum of prior `duration`s) rather than kept as its own field —
//!   `SemioVideoSample` has no separate decode/presentation timestamp pair.
//! - `Mp4Snapshot.ftyp` has no video-subset counterpart and is dropped.

use crate::artifacts::mp4::Mp4Snapshot;
use crate::artifacts::semio::standards::v1::subsets::video::schema::snapshot::{SemioRational, SemioVideoSample, SemioVideoSnapshot, SemioVideoStream, SemioVideoStreamKind, STDIO_SEMIOVIDEO_DOCUMENT_SCHEMA};
use semio_framework_plugin::{ArtifactDeserializer, Dialect, StandardId, SubsetId};

const FROM_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.mp4", standard: StandardId("isobmff"), subset: SubsetId("*") };
const INTO_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("video") };

pub struct SemioVideoFromMp4;

impl ArtifactDeserializer for SemioVideoFromMp4 {
    type From = Mp4Snapshot;
    type Into = SemioVideoSnapshot;
    const FROM: Dialect = FROM_DIALECT;
    const INTO: Dialect = INTO_DIALECT;

    async fn deserialize(from: &Self::From) -> Result<Self::Into, store::PackError> {
        let streams = from
            .tracks
            .iter()
            .map(|track| {
                let codec = "avc1".to_string();
                let mut dts: i64 = 0;
                let samples = track
                    .samples
                    .iter()
                    .map(|sample| {
                        let pts = (dts + sample.cts_offset as i64).max(0) as u64;
                        dts += sample.duration as i64;
                        SemioVideoSample { pts, key: sample.sync, data: sample.data.clone() }
                    })
                    .collect();
                let den = track.samples.first().map(|s| s.duration as i64).unwrap_or(1).max(1);
                SemioVideoStream { kind: SemioVideoStreamKind::Video, codec, width: track.width, height: track.height, rate: SemioRational { num: track.timescale as i64, den }, samples }
            })
            .collect();
        Ok(SemioVideoSnapshot { schema: STDIO_SEMIOVIDEO_DOCUMENT_SCHEMA.into(), streams })
    }
}

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::mp4::standards::isobmff::subsets::any::schema::snapshot::{Mp4Codec, Mp4Ftyp, Mp4Sample, Mp4Track};

    async fn real_world_mp4() -> Mp4Snapshot {
        Mp4Snapshot {
            schema: "stdio.mp4".into(),
            ftyp: Mp4Ftyp { major_brand: "isom".into(), minor_version: 0, compatible_brands: vec!["isom".into(), "mp41".into()] },
            movie: Default::default(),
            tracks: vec![Mp4Track {
                track_id: 1,
                timescale: 30,
                codec: Mp4Codec::default(),
                width: 1920,
                height: 1080,
                metadata: Default::default(),
                chunk_sample_counts: vec![3],
                samples: vec![
                    Mp4Sample { data: vec![0xAA, 0xBB], duration: 1, cts_offset: 0, sync: true },
                    Mp4Sample { data: vec![0xCC], duration: 1, cts_offset: 1, sync: false },
                    Mp4Sample { data: vec![0xDD, 0xEE, 0xFF], duration: 1, cts_offset: 0, sync: false },
                ],
            }],
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn deserialize_maps_real_track_metadata_and_derives_pts_from_duration_plus_cts_offset() {
        let video = semio_framework_plugin::resolve_ready(SemioVideoFromMp4::deserialize(&real_world_mp4())).expect("deserialize");
        assert_eq!(video.streams.len(), 1);
        let stream = &video.streams[0];
        assert_eq!(stream.kind, SemioVideoStreamKind::Video);
        assert_eq!(stream.codec, "avc1");
        assert_eq!(stream.width, 1920);
        assert_eq!(stream.height, 1080);
        assert_eq!(stream.rate, SemioRational { num: 30, den: 1 });
        assert_eq!(stream.samples.len(), 3);
        // dts: 0, 1, 2 -- pts = dts + cts_offset
        assert_eq!(stream.samples[0].pts, 0);
        assert_eq!(stream.samples[1].pts, 2); // dts=1, cts_offset=1
        assert_eq!(stream.samples[2].pts, 2); // dts=2, cts_offset=0
        assert!(stream.samples[0].key);
        assert!(!stream.samples[1].key);
        assert_eq!(stream.samples[2].data, vec![0xDD, 0xEE, 0xFF]);
    }

    #[semio_framework_async_macros::async_test]
    async fn deserialize_of_track_with_no_samples_falls_back_to_unit_rate_denominator() {
        let mut mp4 = real_world_mp4();
        mp4.tracks[0].samples.clear();
        let video = semio_framework_plugin::resolve_ready(SemioVideoFromMp4::deserialize(&mp4)).expect("deserialize");
        assert_eq!(video.streams[0].rate, SemioRational { num: 30, den: 1 });
        assert!(video.streams[0].samples.is_empty());
    }
}
//#endregion 🔖️Tests
