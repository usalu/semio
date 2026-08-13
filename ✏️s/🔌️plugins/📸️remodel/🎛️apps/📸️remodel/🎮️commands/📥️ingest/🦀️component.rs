//! 📥️ Remodel play app commands — media ingestion: still-image drop-zone payloads, the host-decoded
//! video frame/done tick pair, the in-process video-bytes fallback, and manual stream bookkeeping.

use crate::apps::remodel::config::{RemodelConfig, RemodelConfigMutation};
use crate::apps::remodel::engine::{describe_video_probe, images as remodel_image, video as remodel_video, video_codec_to_artifact};
use crate::apps::remodel::{decode_still_image, payload_from_data_url};
use crate::artifacts::remodel::mutations::{add_stream_frame, change_stream_sync, create_asset, create_stream, delete_stream, replace_stream_source};
use crate::artifacts::remodel::schema::{next_remodel_id, video_codec_from_label};
use crate::artifacts::remodel::op::RemodelMutation;
use crate::artifacts::remodel::{FrameRef, ImageAsset, MediaKind, MediaStream, RemodelSnapshot, VideoSource};
use base64::Engine as _;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault, HostEffect};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

//#region 🔖️VideoImportScratch
/// 📥️ Rolling blur-gate scratch for one in-progress `importVideoFramePayload`/`importVideoBytesPayload`
/// batch — mirrors the reconstruction engine's own relative-sharpness gate (not reusable directly: that
/// gate lives inside a whole `FrameSource`, this one only needs the rolling-median scratch itself).
#[derive(Clone, Debug, Default, PartialEq)]
struct VideoImportScratch {
    rolling_scores: VecDeque<f32>,
}

const BLUR_GATE_ROLLING_WINDOW: usize = 15;
const BLUR_GATE_MIN_SAMPLES: usize = 3;

/// 🧭️ Gradient-energy sharpness proxy — a local mirror of the reconstruction engine's private
/// `sharpness_score` (not exported by that topic file), reused here so import-time frame gating uses
/// the identical signal.
fn local_sharpness_score(image: &remodel_image::ImageRgba8) -> f32 {
    let gray = remodel_image::ImageGray::from_rgba8_luma(image);
    let grad = remodel_image::scharr_gradients(&gray);
    if grad.gx.is_empty() {
        return 0.0;
    }
    let sum_sq: f32 = grad.gx.iter().zip(grad.gy.iter()).map(|(&gx, &gy)| gx * gx + gy * gy).sum();
    sum_sq / grad.gx.len() as f32
}

fn local_rolling_median(scores: &VecDeque<f32>) -> f32 {
    let mut v: Vec<f32> = scores.iter().copied().collect();
    v.sort_by(f32::total_cmp);
    v[v.len() / 2]
}

/// 🚦️ Whether the sample should be rejected by the relative blur gate, given `scratch`'s rolling window
/// and `min_sharpness` (a fraction of the rolling median); also records the sample if accepted.
fn blur_gate_reject(scratch: &mut VideoImportScratch, score: f32, min_sharpness: f32) -> bool {
    if scratch.rolling_scores.len() >= BLUR_GATE_MIN_SAMPLES {
        let median = local_rolling_median(&scratch.rolling_scores);
        if score < min_sharpness * median {
            return true;
        }
    }
    if scratch.rolling_scores.len() >= BLUR_GATE_ROLLING_WINDOW {
        scratch.rolling_scores.pop_front();
    }
    scratch.rolling_scores.push_back(score);
    false
}

/// 🧩️ Pure reconstruction of the blur-gate rolling window from `stream_id`'s already-persisted frames
/// (most recent `BLUR_GATE_ROLLING_WINDOW` first, then scored oldest-to-newest so the window fills in
/// the same order the original per-tick `RefCell` scratch would have) — the pure-trait replacement
/// for carrying `VideoImportScratch` as hidden interior-mutable state across `ImportVideoFramePayload`
/// ticks.
fn rebuild_video_import_scratch(scene: &RemodelSnapshot, stream_id: &str) -> VideoImportScratch {
    let mut scratch = VideoImportScratch::default();
    let Some(stream) = scene.streams.iter().find(|stream| stream.id == stream_id) else { return scratch };
    let mut recent: Vec<&FrameRef> = stream.frames.iter().rev().take(BLUR_GATE_ROLLING_WINDOW).collect();
    recent.reverse();
    for frame in recent {
        let Some(asset) = crate::artifacts::remodel::remodel_asset(&scene.assets, &frame.asset_id) else { continue };
        let Ok(bytes) = base64::engine::general_purpose::STANDARD.decode(&asset.data) else { continue };
        let Ok(image) = decode_still_image(&asset.mime, &bytes) else { continue };
        scratch.rolling_scores.push_back(local_sharpness_score(&image));
    }
    scratch
}

/// 🆔️ The stream a batch tick lands on: `index == 0` starts a new stream, `index > 0` appends to
/// `scene.streams.last()` — the stream THIS batch's `index == 0` call just created (each call sees the
/// prior call's already-committed mutations, since dispatches within one batch are sequential).
fn batch_stream_id(scene: &RemodelSnapshot, index: u32) -> String {
    if index == 0 {
        next_remodel_id("stream")
    } else {
        scene.streams.last().map_or_else(|| next_remodel_id("stream"), |stream| stream.id.clone())
    }
}
//#endregion 🔖️VideoImportScratch

//#region 🔖️ImportFramePayload
pub mod import_frame_payload {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "import-frame-payload")]
    pub struct ImportFramePayload {
        pub payload: String,
        pub name: String,
        pub index: u32,
    }

    /// 📥️ A still-image drop-zone/file-picker payload; a `video/*` mime is re-routed to the in-process
    /// video-bytes decoder.
    pub fn handle(payload: &ImportFramePayload, doc: &ArtifactView<'_, RemodelSnapshot>, cfg: &ConfigView<'_, RemodelConfig>) -> Result<Emit<RemodelMutation, RemodelConfigMutation>, Fault> {
        let Some((mime, bytes)) = payload_from_data_url(&payload.payload) else { return Ok(Emit::default()) };
        if mime.starts_with("video/") {
            return import_video_bytes_payload::handle(&import_video_bytes_payload::ImportVideoBytesPayload { payload: payload.payload.clone(), name: payload.name.clone() }, doc, cfg);
        }
        let scene = doc.snapshot;
        let stream_id = batch_stream_id(scene, payload.index);

        let (width, height) = decode_still_image(&mime, &bytes).map_or((0, 0), |image| (image.width, image.height));
        let asset_key = format!("{stream_id}-frame-{}", payload.index);
        let asset = ImageAsset { mime, data: base64::engine::general_purpose::STANDARD.encode(&bytes), width, height };

        let mut mutations = vec![create_asset(asset_key.clone(), asset)];
        match scene.streams.iter().find(|stream| stream.id == stream_id) {
            Some(stream) => {
                let frame_index = stream.frames.len() as u32;
                mutations.push(add_stream_frame(stream_id.clone(), FrameRef { index: frame_index, timestamp_ms: f64::from(frame_index) * 1000.0 / 30.0, asset_id: asset_key.clone() }, MediaKind::ImageSequence));
            }
            None => {
                mutations.push(create_stream(MediaStream {
                    id: stream_id.clone(),
                    name: payload.name.clone(),
                    kind: MediaKind::ImageSequence,
                    camera_id: None,
                    sync_offset_ms: 0.0,
                    fps_hint: 30.0,
                    frames: vec![FrameRef { index: 0, timestamp_ms: 0.0, asset_id: asset_key.clone() }],
                    source: None,
                }));
            }
        }
        Ok(Emit::amend(mutations, format!("remodel-import:{stream_id}")))
    }
}
//#endregion 🔖️ImportFramePayload

//#region 🔖️ImportVideoFramePayload
pub mod import_video_frame_payload {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "import-video-frame-payload")]
    pub struct ImportVideoFramePayload {
        pub payload: String,
        pub name: String,
        pub index: u32,
        pub frame_index: u32,
        pub timestamp_ms: f64,
    }

    /// 🎞️ Host-decoded video frame tick (Tier 1/2 `RequestMediaFrames` frame dispatch): decodes the
    /// sampled JPEG, runs it through the relative blur gate (rebuilt from persisted frames each tick —
    /// see `rebuild_video_import_scratch`), and amends it into the active stream.
    pub fn handle(payload: &ImportVideoFramePayload, doc: &ArtifactView<'_, RemodelSnapshot>, _cfg: &ConfigView<'_, RemodelConfig>) -> Result<Emit<RemodelMutation, RemodelConfigMutation>, Fault> {
        let Some((_mime, bytes)) = payload_from_data_url(&payload.payload) else { return Ok(Emit::default()) };
        let Ok(image) = remodel_image::decode_jpeg(&bytes) else { return Ok(Emit::default()) };
        let scene = doc.snapshot;
        let stream_id = batch_stream_id(scene, payload.index);

        let score = local_sharpness_score(&image);
        let min_sharpness = scene.params.ingest.min_sharpness;
        let mut scratch = rebuild_video_import_scratch(scene, &stream_id);
        if blur_gate_reject(&mut scratch, score, min_sharpness) {
            return Ok(Emit::default());
        }

        let asset_key = format!("{stream_id}-frame-{}", payload.frame_index);
        let asset = ImageAsset { mime: "image/jpeg".into(), data: base64::engine::general_purpose::STANDARD.encode(&bytes), width: image.width, height: image.height };
        let mut mutations = vec![create_asset(asset_key.clone(), asset)];
        match scene.streams.iter().any(|stream| stream.id == stream_id) {
            true => mutations.push(add_stream_frame(stream_id.clone(), FrameRef { index: payload.frame_index, timestamp_ms: payload.timestamp_ms, asset_id: asset_key.clone() }, MediaKind::Video)),
            false => mutations.push(create_stream(MediaStream {
                id: stream_id.clone(),
                name: payload.name.clone(),
                kind: MediaKind::Video,
                camera_id: None,
                sync_offset_ms: 0.0,
                fps_hint: 0.0,
                frames: vec![FrameRef { index: payload.frame_index, timestamp_ms: payload.timestamp_ms, asset_id: asset_key.clone() }],
                source: None,
            })),
        }
        Ok(Emit::amend(mutations, format!("remodel-import:{stream_id}")))
    }
}
//#endregion 🔖️ImportVideoFramePayload

//#region 🔖️ImportVideoDone
pub mod import_video_done {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "import-video-done")]
    pub struct ImportVideoDone {
        pub name: String,
        pub duration_ms: f64,
        pub frame_count: u32,
        pub width: u32,
        pub height: u32,
        pub codec: String,
    }

    /// ✅️ Host-decoded video import finished: writes `VideoSource` provenance on the just-imported
    /// stream (`scene.streams.last()` — the stream this batch's ticks just built). Uses the SAME
    /// coalesce key as every preceding `ImportVideoFramePayload` tick, so the whole import (every
    /// accepted frame plus this final metadata write) collapses into one undo step.
    pub fn handle(payload: &ImportVideoDone, doc: &ArtifactView<'_, RemodelSnapshot>, _cfg: &ConfigView<'_, RemodelConfig>) -> Result<Emit<RemodelMutation, RemodelConfigMutation>, Fault> {
        let scene = doc.snapshot;
        let Some(stream_id) = scene.streams.last().map(|stream| stream.id.clone()) else { return Ok(Emit::default()) };
        let codec_value = video_codec_from_label(&payload.codec);
        let source = VideoSource { name: payload.name.clone(), container: "unknown".into(), codec: codec_value, duration_ms: payload.duration_ms, frame_count: payload.frame_count, width: payload.width, height: payload.height };
        Ok(Emit::amend(vec![replace_stream_source(stream_id.clone(), Some(source))], format!("remodel-import:{stream_id}")))
    }
}
//#endregion 🔖️ImportVideoDone

//#region 🔖️ImportVideoBytesPayload
pub mod import_video_bytes_payload {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "import-video-bytes-payload")]
    pub struct ImportVideoBytesPayload {
        pub payload: String,
        pub name: String,
    }

    /// 🎞️ Tier-3 fallback (or `ImportFramePayload`'s own video-mime branch): the host couldn't decode
    /// the video, so it hands over the raw container bytes and the `video` engine topic file's
    /// demux/MJPEG/baseline-AVC decoder extracts frames fully in-process. The whole batch materializes
    /// inside this ONE pure call, so it needs no coalesce key (already exactly one `Emit`, hence one
    /// undo step). An undecodable codec surfaces as a `Notify` naming it, with provenance from the probe.
    pub fn handle(payload: &ImportVideoBytesPayload, doc: &ArtifactView<'_, RemodelSnapshot>, _cfg: &ConfigView<'_, RemodelConfig>) -> Result<Emit<RemodelMutation, RemodelConfigMutation>, Fault> {
        let Some((_mime, bytes)) = payload_from_data_url(&payload.payload) else { return Ok(Emit::default()) };
        let probe = match remodel_video::probe(&bytes) {
            Ok(probe) => probe,
            Err(error) => return Ok(Emit::effect(HostEffect::Notify { message: format!("Could not probe video: {error}") })),
        };
        let (codec, width, height, duration_ms, container) = describe_video_probe(&probe);
        let scene = doc.snapshot;
        let ingest = &scene.params.ingest;
        let opts = remodel_video::VideoIngestOptions { stride: ingest.frame_sample_stride.max(1), max_frames: ingest.max_frames, max_long_edge_px: ingest.downscale_long_edge_px };
        let iter = match remodel_video::extract_frames(&bytes, &opts) {
            Ok(iter) => iter,
            Err(error) => return Ok(Emit::effect(HostEffect::Notify { message: format!("Unsupported video codec ({codec:?}): {error} - probed {container} {width}x{height}") })),
        };

        let stream_id = next_remodel_id("stream");
        let min_sharpness = ingest.min_sharpness;
        let mut scratch = VideoImportScratch::default();
        let mut frames = Vec::new();
        let mut mutations = Vec::new();
        for extracted in iter {
            let Ok(extracted) = extracted else { continue };
            let score = local_sharpness_score(&extracted.image);
            if blur_gate_reject(&mut scratch, score, min_sharpness) {
                continue;
            }
            let jpeg = remodel_image::encode_jpeg(&extracted.image, 90);
            let asset_key = format!("{stream_id}-frame-{}", extracted.index);
            mutations.push(create_asset(asset_key.clone(), ImageAsset { mime: "image/jpeg".into(), data: base64::engine::general_purpose::STANDARD.encode(&jpeg), width: extracted.image.width, height: extracted.image.height }));
            frames.push(FrameRef { index: extracted.index, timestamp_ms: extracted.timestamp_ms, asset_id: asset_key });
        }
        mutations.push(create_stream(MediaStream {
            id: stream_id,
            name: payload.name.clone(),
            kind: MediaKind::Video,
            camera_id: None,
            sync_offset_ms: 0.0,
            fps_hint: 0.0,
            frames,
            source: Some(VideoSource { name: String::new(), container: container.into(), codec: video_codec_to_artifact(codec), duration_ms, frame_count: 0, width, height }),
        }));
        Ok(Emit::mutations(mutations))
    }
}
//#endregion 🔖️ImportVideoBytesPayload

//#region 🔖️AddStream
pub mod add_stream {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "add-stream")]
    pub struct AddStream {
        pub name: String,
        pub kind: String,
        pub camera_id: String,
    }

    pub fn handle(payload: &AddStream, _doc: &ArtifactView<'_, RemodelSnapshot>, _cfg: &ConfigView<'_, RemodelConfig>) -> Result<Emit<RemodelMutation, RemodelConfigMutation>, Fault> {
        let kind = if payload.kind == "video" { MediaKind::Video } else { MediaKind::ImageSequence };
        let camera_id = if payload.camera_id.is_empty() { None } else { Some(payload.camera_id.clone()) };
        let id = next_remodel_id("stream");
        let stream = MediaStream { id, name: payload.name.clone(), kind, camera_id, sync_offset_ms: 0.0, fps_hint: 30.0, frames: Vec::new(), source: None };
        Ok(Emit::mutations(vec![create_stream(stream)]))
    }
}
//#endregion 🔖️AddStream

//#region 🔖️RemoveStream
pub mod remove_stream {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "remove-stream")]
    pub struct RemoveStream {
        pub stream_id: String,
    }

    pub fn handle(payload: &RemoveStream, _doc: &ArtifactView<'_, RemodelSnapshot>, _cfg: &ConfigView<'_, RemodelConfig>) -> Result<Emit<RemodelMutation, RemodelConfigMutation>, Fault> {
        Ok(Emit::mutations(vec![delete_stream(payload.stream_id.clone())]))
    }
}
//#endregion 🔖️RemoveStream

//#region 🔖️SetStreamSync
pub mod set_stream_sync {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "stream-sync")]
    pub struct SetStreamSync {
        pub stream_id: String,
        pub sync_offset_ms: f64,
    }

    pub fn handle(payload: &SetStreamSync, doc: &ArtifactView<'_, RemodelSnapshot>, _cfg: &ConfigView<'_, RemodelConfig>) -> Result<Emit<RemodelMutation, RemodelConfigMutation>, Fault> {
        if !doc.snapshot.streams.iter().any(|stream| stream.id == payload.stream_id) {
            return Ok(Emit::default());
        }
        Ok(Emit::mutations(vec![change_stream_sync(payload.stream_id.clone(), payload.sync_offset_ms)]))
    }
}
//#endregion 🔖️SetStreamSync

//#region 🧪️Testkit
/// 📥️ Imports `n` checker frames as one new image-sequence stream via `ImportFramePayload`, mirroring
/// exactly what a real `importFrames` → `RequestFileOpen.multiple` re-dispatch loop sends. Shared with
/// `🎮️commands/🚀️reconstruction`'s own tests, which need real decodable frames to run a pipeline on.
#[cfg(test)]
pub(crate) fn testkit_import_checker_stream(app: &mut crate::apps::remodel::testkit::RemodelApp, n: u32) {
    use crate::apps::remodel::testkit::dispatch;
    use crate::apps::remodel::RemodelCommand;
    for index in 0..n {
        dispatch(app, RemodelCommand::ImportFramePayload(import_frame_payload::ImportFramePayload { payload: checker_data_url(24, 24, 3), name: format!("frame-{index}.png"), index }));
    }
}

/// 🏁️ High-contrast `cell`-pixel checkerboard, PNG-encoded and base64-wrapped as a `requestFileOpen`
/// `dataUrl` payload — so the real decode path is exercised, not a stub.
#[cfg(test)]
pub(crate) fn checker_data_url(w: u32, h: u32, cell: u32) -> String {
    format!("data:image/png;base64,{}", base64::engine::general_purpose::STANDARD.encode(remodel_image::encode_png(&checker_image(w, h, cell)).expect("encode checker png")))
}

/// 🏁️ The same checkerboard, real-JPEG-encoded — mirrors what a `RequestMediaFrames` host actually
/// dispatches to `frame_action` (`payload: dataUrl(image/jpeg)`).
#[cfg(test)]
pub(crate) fn checker_data_url_jpeg(w: u32, h: u32, cell: u32) -> String {
    format!("data:image/jpeg;base64,{}", base64::engine::general_purpose::STANDARD.encode(remodel_image::encode_jpeg(&checker_image(w, h, cell), 90)))
}

/// 🎞️ A tiny synthesized MJPEG-in-MP4 video (n frames of the same checker pattern) as a
/// `RequestMediaFrames`-fallback-style raw base64 data URL payload.
#[cfg(test)]
pub(crate) fn checker_video_data_url(n: u32, w: u32, h: u32, cell: u32) -> String {
    let jpeg = remodel_image::encode_jpeg(&checker_image(w, h, cell), 90);
    let frames: Vec<Vec<u8>> = (0..n).map(|_| jpeg.clone()).collect();
    format!("data:video/mp4;base64,{}", base64::engine::general_purpose::STANDARD.encode(remodel_video::write_mp4_mjpeg(&frames, 10.0)))
}

#[cfg(test)]
fn checker_image(w: u32, h: u32, cell: u32) -> remodel_image::ImageRgba8 {
    let mut image = remodel_image::ImageRgba8::new(w, h);
    for y in 0..h {
        for x in 0..w {
            let on = ((x / cell.max(1)) + (y / cell.max(1))).is_multiple_of(2);
            let v = if on { 235u8 } else { 20u8 };
            let idx = ((y * w + x) * 4) as usize;
            image.data[idx] = v;
            image.data[idx + 1] = v;
            image.data[idx + 2] = v;
            image.data[idx + 3] = 255;
        }
    }
    image
}
//#endregion 🧪️Testkit

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::remodel::testkit::{app, dispatch};
    use crate::apps::remodel::RemodelCommand;

    #[test]
    fn import_frame_payload_creates_a_stream_and_asset() {
        let mut app = app();
        testkit_import_checker_stream(&mut app, 3);
        let scene = app.snapshot().expect("projection");
        assert_eq!(scene.streams.len(), 1, "one importFrames batch creates exactly one stream");
        assert_eq!(scene.streams[0].frames.len(), 3);
        assert_eq!(scene.assets.len(), 3);
    }

    /// 🎞️ In-process video import (the `ImportVideoBytesPayload` fallback path): a tiny synthesized
    /// MJPEG mp4 must decode into a new video stream whose frame count matches what was muxed in.
    #[test]
    fn import_video_bytes_payload_extracts_frames_in_process() {
        let mut app = app();
        // 🎯️ `IngestParams::default().frame_sample_stride == 5`; force stride 1 so all 5 synthesized
        // frames are kept (a stride-sampling test belongs to the video engine topic file, not here).
        dispatch(&mut app, RemodelCommand::SetIngestParams(crate::apps::remodel::commands::params::set_ingest_params::SetIngestParams { frame_sample_stride: 1, max_frames: 200, downscale_long_edge_px: 1600, min_sharpness: 0.3 }));
        dispatch(&mut app, RemodelCommand::ImportVideoBytesPayload(import_video_bytes_payload::ImportVideoBytesPayload { payload: checker_video_data_url(5, 32, 32, 4), name: "clip.mp4".into() }));
        let scene = app.snapshot().expect("projection");
        assert_eq!(scene.streams.len(), 1);
        assert_eq!(scene.streams[0].kind, MediaKind::Video);
        assert_eq!(scene.streams[0].frames.len(), 5);
        assert_eq!(scene.assets.len(), 5);
    }

    /// 🎞️ Host-decoded video import path: `ImportVideoFramePayload` ticks followed by `ImportVideoDone`
    /// must accumulate into one stream and write `VideoSource` provenance, all under one coalesce key.
    #[test]
    fn import_video_frame_payload_then_done_writes_one_stream_with_video_source() {
        let mut app = app();
        for index in 0..4u32 {
            dispatch(
                &mut app,
                RemodelCommand::ImportVideoFramePayload(import_video_frame_payload::ImportVideoFramePayload { payload: checker_data_url_jpeg(24, 24, 3), name: "clip.mp4".into(), index, frame_index: index, timestamp_ms: f64::from(index) * 100.0 }),
            );
        }
        dispatch(&mut app, RemodelCommand::ImportVideoDone(import_video_done::ImportVideoDone { name: "clip.mp4".into(), duration_ms: 400.0, frame_count: 4, width: 24, height: 24, codec: "mjpeg".into() }));
        let scene = app.snapshot().expect("projection");
        assert_eq!(scene.streams.len(), 1);
        assert_eq!(scene.streams[0].kind, MediaKind::Video);
        assert!(scene.streams[0].source.is_some());
        assert_eq!(scene.streams[0].source.as_ref().expect("video source").frame_count, 4);
    }

    #[test]
    fn add_remove_and_sync_streams_edit_the_stream_list() {
        let mut app = app();
        dispatch(&mut app, RemodelCommand::AddStream(add_stream::AddStream { name: "Front".into(), kind: "video".into(), camera_id: "cam-0".into() }));
        let stream_id = app.snapshot().expect("projection").streams[0].id.clone();
        dispatch(&mut app, RemodelCommand::SetStreamSync(set_stream_sync::SetStreamSync { stream_id: stream_id.clone(), sync_offset_ms: 12.5 }));
        assert_eq!(app.snapshot().expect("projection").streams[0].sync_offset_ms, 12.5);
        dispatch(&mut app, RemodelCommand::RemoveStream(remove_stream::RemoveStream { stream_id }));
        assert!(app.snapshot().expect("projection").streams.is_empty());
    }
}
//#endregion 🧪️Tests
