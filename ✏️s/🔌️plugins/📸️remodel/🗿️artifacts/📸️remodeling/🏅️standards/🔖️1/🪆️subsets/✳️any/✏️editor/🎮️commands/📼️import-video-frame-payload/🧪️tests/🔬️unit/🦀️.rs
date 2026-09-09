use super::*;
use crate::editor::remodeling::commands::{add_stream, import_video_bytes_payload, import_video_done, remove_stream, set_stream_sync};
use crate::editor::remodeling::testkit::{app, dispatch};
use crate::editor::remodeling::RemodelingCommand;

#[semio_framework_async_macros::async_test]
async fn import_frame_payload_creates_a_stream_and_asset() {
    let mut app = app().await;
    testkit_import_checker_stream(&mut app, 3).await;
    let scene = app.snapshot().expect("projection");
    assert_eq!(scene.streams.len(), 1, "one importFrames batch creates exactly one stream");
    assert_eq!(scene.streams[0].frames.len(), 3);
    assert_eq!(scene.assets.len(), 3);
}

/// 🎞️ In-process video import (the `ImportVideoBytesPayload` fallback path): a tiny synthesized
/// MJPEG mp4 must decode into a new video stream whose frame count matches what was muxed in.
#[semio_framework_async_macros::async_test]
async fn import_video_bytes_payload_extracts_frames_in_process() {
    let mut app = app().await;
    // 🎯️ `IngestParams::default().frame_sample_stride == 5`; force stride 1 so all 5 synthesized
    // frames are kept (a stride-sampling test belongs to the video engine topic file, not here).
    dispatch(&mut app, RemodelingCommand::SetIngestParams(crate::editor::remodeling::commands::set_ingest_params::SetIngestParams { frame_sample_stride: 1, max_frames: 200, downscale_long_edge_px: 1600, min_sharpness: 0.3 })).await;
    dispatch(&mut app, RemodelingCommand::ImportVideoBytesPayload(import_video_bytes_payload::ImportVideoBytesPayload { payload: checker_video_data_url(5, 32, 32, 4).await, name: "clip.mp4".into() })).await;
    let scene = app.snapshot().expect("projection");
    assert_eq!(scene.streams.len(), 1);
    assert_eq!(scene.streams[0].kind, MediaKind::Video);
    assert_eq!(scene.streams[0].frames.len(), 5);
    assert_eq!(scene.assets.len(), 5);
}

/// 🎞️ Host-decoded video import path: `ImportVideoFramePayload` ticks followed by `ImportVideoDone`
/// must accumulate into one stream and write `VideoSource` provenance, all under one coalesce key.
#[semio_framework_async_macros::async_test]
async fn import_video_frame_payload_then_done_writes_one_stream_with_video_source() {
    let mut app = app().await;
    for index in 0..4u32 {
        dispatch(&mut app, RemodelingCommand::ImportVideoFramePayload(ImportVideoFramePayload { payload: checker_data_url_jpeg(24, 24, 3).await, name: "clip.mp4".into(), index, frame_index: index, timestamp_ms: f64::from(index) * 100.0 })).await;
    }
    dispatch(&mut app, RemodelingCommand::ImportVideoDone(import_video_done::ImportVideoDone { name: "clip.mp4".into(), duration_ms: 400.0, frame_count: 4, width: 24, height: 24, codec: "mjpeg".into() })).await;
    let scene = app.snapshot().expect("projection");
    assert_eq!(scene.streams.len(), 1);
    assert_eq!(scene.streams[0].kind, MediaKind::Video);
    assert!(scene.streams[0].source.is_some());
    assert_eq!(scene.streams[0].source.as_ref().expect("video source").frame_count, 4);
}

#[semio_framework_async_macros::async_test]
async fn add_remove_and_sync_streams_edit_the_stream_list() {
    let mut app = app().await;
    dispatch(&mut app, RemodelingCommand::AddStream(add_stream::AddStream { name: "Front".into(), kind: "video".into(), camera_id: "cam-0".into() })).await;
    let stream_id = app.snapshot().expect("projection").streams[0].id.clone();
    dispatch(&mut app, RemodelingCommand::SetStreamSync(set_stream_sync::SetStreamSync { stream_id: stream_id.clone(), sync_offset_ms: 12.5 })).await;
    assert_eq!(app.snapshot().expect("projection").streams[0].sync_offset_ms, 12.5);
    dispatch(&mut app, RemodelingCommand::RemoveStream(remove_stream::RemoveStream { stream_id })).await;
    assert!(app.snapshot().expect("projection").streams.is_empty());
}
