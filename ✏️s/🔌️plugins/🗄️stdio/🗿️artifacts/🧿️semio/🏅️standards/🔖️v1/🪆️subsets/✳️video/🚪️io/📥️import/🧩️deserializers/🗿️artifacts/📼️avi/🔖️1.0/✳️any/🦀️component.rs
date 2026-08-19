//! 📥️ Deserialize `s.stdio.avi` (1.0/✳️any) into `s.stdio.semio` (v1/video) — direct reshape of
//! RIFF/AVI stream metadata. Only `vids` (`fccType == "vids"`) streams become `SemioVideoStream`s
//! (an honest boundary matching `video`'s own kind vocabulary of Video/Audio/Subtitle -- `auds`
//! streams map to `Audio`, everything else is dropped with a doc-noted reason below, never
//! fabricated). AVI has no per-sample timestamp -- `pts` is synthesized as the running index times
//! `strh.scale` (AVI's own per-frame duration unit, `rate`/`scale` gives frames/sec), matching the
//! spec's own constant-frame-duration assumption for `dwLength`.
//!
//! Honest, documented lossy fields:
//! - `AviMainHeader` (global flags/buffer sizes) has no video-subset counterpart; dropped.
//! - `AviStreamFormat::BitmapInfo`/`WaveFormat` structured fields collapse to a codec name string
//!   (`compression` for `vids`, `"pcm"`/`format_tag` for `auds`) -- the rest is dropped.
//! - `AviChunk.fourcc` (e.g. `"00dc"`) is dropped -- `SemioVideoSample` has no per-sample tag slot.

use crate::artifacts::avi::standards::v1_0::subsets::any::schema::snapshot::AviStreamFormat;
use crate::artifacts::avi::AviSnapshot;
use crate::artifacts::semio::standards::v1::subsets::video::schema::snapshot::{SemioRational, SemioVideoSample, SemioVideoSnapshot, SemioVideoStream, SemioVideoStreamKind, STDIO_SEMIOVIDEO_DOCUMENT_SCHEMA};
use semio_framework_plugin::{ArtifactDeserializer, Dialect, StandardId, SubsetId};

const FROM_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.avi", standard: StandardId("1.0"), subset: SubsetId("*") };
const INTO_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("video") };

pub struct SemioVideoFromAvi;

impl ArtifactDeserializer for SemioVideoFromAvi {
    type From = AviSnapshot;
    type Into = SemioVideoSnapshot;
    const FROM: Dialect = FROM_DIALECT;
    const INTO: Dialect = INTO_DIALECT;

    async fn deserialize(from: &Self::From) -> Result<Self::Into, store::PackError> {
        let streams = from
            .streams
            .iter()
            .filter_map(|s| {
                let kind = match s.strh.fcc_type.as_str() {
                    "vids" => SemioVideoStreamKind::Video,
                    "auds" => SemioVideoStreamKind::Audio,
                    _ => return None, // 📦️ AVI also permits "txts"/"mids" streams -- out of `video`'s own Video/Audio/Subtitle+opaque-sample vocabulary honestly dropped, never fabricated as one of the three.
                };
                let (codec, width, height) = match &s.strf {
                    AviStreamFormat::BitmapInfo { compression, width, height, .. } => (compression.clone(), (*width).max(0) as u32, (*height).unsigned_abs()),
                    AviStreamFormat::WaveFormat { format_tag, .. } => (format!("wav-tag-{format_tag}"), 0, 0),
                    AviStreamFormat::Raw { .. } => (s.strh.fcc_handler.clone(), 0, 0),
                };
                let scale = s.strh.scale.max(1) as i64;
                let rate_num = s.strh.rate.max(1) as i64;
                let samples = s.chunks.iter().enumerate().map(|(i, chunk)| SemioVideoSample { pts: (i as i64 * scale).max(0) as u64, key: chunk.keyframe, data: chunk.data.clone() }).collect();
                Some(SemioVideoStream { kind, codec, width, height, rate: SemioRational { num: rate_num, den: scale }, samples })
            })
            .collect();
        Ok(SemioVideoSnapshot { schema: STDIO_SEMIOVIDEO_DOCUMENT_SCHEMA.into(), streams })
    }
}

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::avi::standards::v1_0::subsets::any::schema::snapshot::{AviChunk, AviMainHeader, AviStream, AviStreamHeader};

    fn real_world_avi() -> AviSnapshot {
        AviSnapshot {
            schema: "stdio.avi".into(),
            main_header: AviMainHeader {
                micro_sec_per_frame: 100_000,
                max_bytes_per_sec: 1400,
                padding_granularity: 0,
                flags: 0x10,
                total_frames: 2,
                initial_frames: 0,
                streams: 1,
                suggested_buffer_size: 140,
                width: 16,
                height: 16,
                reserved: vec![0, 0, 0, 0],
            },
            streams: vec![AviStream {
                strh: AviStreamHeader {
                    fcc_type: "vids".into(),
                    fcc_handler: "MJPG".into(),
                    flags: 0,
                    priority: 0,
                    language: 0,
                    initial_frames: 0,
                    scale: 1,
                    rate: 10,
                    start: 0,
                    length: 2,
                    suggested_buffer_size: 140,
                    quality: -1,
                    sample_size: 0,
                    rc_frame_left: 0,
                    rc_frame_top: 0,
                    rc_frame_right: 16,
                    rc_frame_bottom: 16,
                },
                strf: AviStreamFormat::BitmapInfo { size: 40, width: 16, height: 16, planes: 1, bit_count: 24, compression: "MJPG".into(), size_image: 140, x_pels_per_meter: 0, y_pels_per_meter: 0, colors_used: 0, colors_important: 0 },
                chunks: vec![AviChunk { fourcc: "00dc".into(), data: vec![1, 2, 3, 4], keyframe: true }, AviChunk { fourcc: "00dc".into(), data: vec![5, 6, 7, 8], keyframe: false }],
            }],
            idx1_present: true,
            unknown_chunks: vec![],
        }
    }

    #[test]
    fn deserialize_maps_vids_stream_and_synthesizes_pts_from_scale() {
        let video = semio_framework_plugin::resolve_ready(SemioVideoFromAvi::deserialize(&real_world_avi())).expect("deserialize");
        assert_eq!(video.streams.len(), 1);
        let stream = &video.streams[0];
        assert_eq!(stream.kind, SemioVideoStreamKind::Video);
        assert_eq!(stream.codec, "MJPG");
        assert_eq!(stream.width, 16);
        assert_eq!(stream.height, 16);
        assert_eq!(stream.rate, SemioRational { num: 10, den: 1 });
        assert_eq!(stream.samples.len(), 2);
        assert_eq!(stream.samples[0].pts, 0);
        assert_eq!(stream.samples[1].pts, 1);
        assert!(stream.samples[0].key);
        assert!(!stream.samples[1].key);
    }

    #[test]
    fn non_vids_non_auds_stream_kind_is_honestly_dropped_not_fabricated() {
        let mut avi = real_world_avi();
        avi.streams[0].strh.fcc_type = "txts".into();
        let video = semio_framework_plugin::resolve_ready(SemioVideoFromAvi::deserialize(&avi)).expect("deserialize");
        assert!(video.streams.is_empty());
    }
}
//#endregion 🔖️Tests
