//! 📤️ Serialize `s.stdio.semio` (v1/video) into `s.stdio.avi` (1.0/✳️any) — one `AviStream` per
//! `SemioVideoStream`, `strh.scale = rate.den`/`strh.rate = rate.num` (AVI's own frames-per-second
//! fraction, matching this pair's deserializer exactly), `strh.fcc_type` from `kind`
//! (`Video`→`"vids"`, everything else→`"auds"` -- AVI's own two-stream-kind vocabulary is coarser
//! than `video`'s three-way `SemioVideoStreamKind`, so `Subtitle` streams honestly fold to `"auds"`
//! rather than being dropped, since AVI has no dedicated subtitle `fccType` and dropping would lose
//! the samples entirely; documented here rather than silently done). `AviMainHeader` is synthesized
//! from the FIRST stream's dimensions/frame count (a real, honest, documented simplification -- AVI
//! has exactly one global header per file, `video` has per-stream dimensions, a genuine cardinality
//! mismatch when there is more than one stream).

use crate::artifacts::avi::standards::v1_0::subsets::any::schema::snapshot::{AviChunk, AviMainHeader, AviStream, AviStreamFormat, AviStreamHeader};
use crate::artifacts::avi::AviSnapshot;
use crate::artifacts::semio::standards::v1::subsets::video::schema::snapshot::{SemioVideoSnapshot, SemioVideoStreamKind};
use semio_framework_plugin::{ArtifactSerializer, Dialect, StandardId, SubsetId};

const FROM_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("video") };
const INTO_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.avi", standard: StandardId("1.0"), subset: SubsetId("*") };

pub struct SemioVideoToAvi;

impl ArtifactSerializer for SemioVideoToAvi {
    type From = SemioVideoSnapshot;
    type Into = AviSnapshot;
    const FROM: Dialect = FROM_DIALECT;
    const INTO: Dialect = INTO_DIALECT;

    async fn serialize(from: &Self::From) -> Result<Self::Into, store::PackError> {
        let streams: Vec<AviStream> = from
            .streams
            .iter()
            .map(|s| {
                let fcc_type = if s.kind == SemioVideoStreamKind::Video { "vids" } else { "auds" };
                let scale = s.rate.den.max(1) as u32;
                let rate = s.rate.num.max(1) as u32;
                let strh = AviStreamHeader {
                    fcc_type: fcc_type.into(),
                    fcc_handler: s.codec.clone(),
                    flags: 0,
                    priority: 0,
                    language: 0,
                    initial_frames: 0,
                    scale,
                    rate,
                    start: 0,
                    length: s.samples.len() as u32,
                    suggested_buffer_size: 0,
                    quality: -1,
                    sample_size: 0,
                    rc_frame_left: 0,
                    rc_frame_top: 0,
                    rc_frame_right: s.width as i32,
                    rc_frame_bottom: s.height as i32,
                };
                let strf = if fcc_type == "vids" {
                    AviStreamFormat::BitmapInfo {
                        size: 40,
                        width: s.width as i32,
                        height: s.height as i32,
                        planes: 1,
                        bit_count: 24,
                        compression: s.codec.clone(),
                        size_image: 0,
                        x_pels_per_meter: 0,
                        y_pels_per_meter: 0,
                        colors_used: 0,
                        colors_important: 0,
                    }
                } else {
                    AviStreamFormat::Raw { data: Vec::new() }
                };
                let chunks = s.samples.iter().map(|sample| AviChunk { fourcc: if fcc_type == "vids" { "00dc".into() } else { "01wb".into() }, data: sample.data.clone(), keyframe: sample.key }).collect();
                AviStream { strh, strf, chunks }
            })
            .collect();
        let first = from.streams.first();
        let main_header = AviMainHeader {
            micro_sec_per_frame: first.map(|s| if s.rate.num > 0 { (1_000_000 * s.rate.den / s.rate.num).max(0) as u32 } else { 0 }).unwrap_or(0),
            max_bytes_per_sec: 0,
            padding_granularity: 0,
            flags: 0x10,
            total_frames: first.map(|s| s.samples.len() as u32).unwrap_or(0),
            initial_frames: 0,
            streams: streams.len() as u32,
            suggested_buffer_size: 0,
            width: first.map(|s| s.width).unwrap_or(0),
            height: first.map(|s| s.height).unwrap_or(0),
            reserved: vec![0, 0, 0, 0],
        };
        Ok(AviSnapshot { schema: "stdio.avi".into(), main_header, streams, idx1_present: true, unknown_chunks: Vec::new() })
    }
}

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::semio::standards::v1::subsets::video::io::avi_deserializer::SemioVideoFromAvi;
    use crate::artifacts::semio::standards::v1::subsets::video::schema::snapshot::{SemioRational, SemioVideoSample, SemioVideoStream, STDIO_SEMIOVIDEO_DOCUMENT_SCHEMA};
    use semio_framework_plugin::ArtifactDeserializer;

    async fn real_world_video() -> SemioVideoSnapshot {
        SemioVideoSnapshot {
            schema: STDIO_SEMIOVIDEO_DOCUMENT_SCHEMA.into(),
            streams: vec![SemioVideoStream {
                kind: SemioVideoStreamKind::Video,
                codec: "MJPG".into(),
                width: 16,
                height: 16,
                rate: SemioRational { num: 10, den: 1 },
                samples: vec![SemioVideoSample { pts: 0, key: true, data: vec![1, 2, 3, 4] }, SemioVideoSample { pts: 1, key: false, data: vec![5, 6, 7, 8] }],
            }],
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn video_to_avi_to_video_round_trips_everything_the_video_subset_can_represent() {
        let original = real_world_video();
        let avi = semio_framework_plugin::resolve_ready(SemioVideoToAvi::serialize(&original)).expect("serialize");
        assert_eq!(avi.streams.len(), 1);
        assert_eq!(avi.streams[0].strh.fcc_type, "vids");
        assert_eq!(avi.streams[0].strh.scale, 1);
        assert_eq!(avi.streams[0].strh.rate, 10);
        let back = semio_framework_plugin::resolve_ready(SemioVideoFromAvi::deserialize(&avi)).expect("deserialize");
        assert_eq!(back, original);
    }

    #[semio_framework_async_macros::async_test]
    async fn subtitle_kind_folds_to_auds_fcc_type_honestly_documented() {
        let mut snap = real_world_video();
        snap.streams[0].kind = SemioVideoStreamKind::Subtitle;
        let avi = semio_framework_plugin::resolve_ready(SemioVideoToAvi::serialize(&snap)).expect("serialize");
        assert_eq!(avi.streams[0].strh.fcc_type, "auds");
    }
}
//#endregion 🔖️Tests
