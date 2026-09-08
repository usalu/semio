
use super::*;
use semio_s_artifact_stdio_avi::standards::v1_0::subsets::any::schema::snapshot::{AviChunk, AviMainHeader, AviStream, AviStreamHeader};

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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
                rc_frame_width: 16,
                strh_extra: Vec::new(),
            },
            strf: AviStreamFormat::BitmapInfo { size: 40, width: 16, height: 16, planes: 1, bit_count: 24, compression: "MJPG".into(), size_image: 140, x_pels_per_meter: 0, y_pels_per_meter: 0, colors_used: 0, colors_important: 0 },
            chunks: vec![AviChunk { fourcc: "00dc".into(), data: vec![1, 2, 3, 4], keyframe: true }, AviChunk { fourcc: "00dc".into(), data: vec![5, 6, 7, 8], keyframe: false }],
            strl_extra: Vec::new(),
        }],
        idx1_present: true,
        unknown_chunks: vec![],
        hdrl_extra: Vec::new(),
    }
}

#[semio_framework_async_macros::async_test]
async fn deserialize_maps_vids_stream_and_synthesizes_pts_from_scale() {
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

#[semio_framework_async_macros::async_test]
async fn non_vids_non_auds_stream_kind_is_honestly_dropped_not_fabricated() {
    let mut avi = real_world_avi();
    avi.streams[0].strh.fcc_type = "txts".into();
    let video = semio_framework_plugin::resolve_ready(SemioVideoFromAvi::deserialize(&avi)).expect("deserialize");
    assert!(video.streams.is_empty());
}
