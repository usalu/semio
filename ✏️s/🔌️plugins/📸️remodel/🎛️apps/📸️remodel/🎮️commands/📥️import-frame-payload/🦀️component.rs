//! 📥️ 📥️ Remodel play app commands command — `import-frame-payload`.

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
//#endregion 🔖️ImportFramePayload

//#region 🔖️ImportVideoFramePayload
//#endregion 🔖️ImportVideoFramePayload

//#region 🔖️ImportVideoDone
//#endregion 🔖️ImportVideoDone

//#region 🔖️ImportVideoBytesPayload
//#endregion 🔖️ImportVideoBytesPayload

//#region 🔖️AddStream
//#endregion 🔖️AddStream

//#region 🔖️RemoveStream
//#endregion 🔖️RemoveStream

//#region 🔖️SetStreamSync
//#endregion 🔖️SetStreamSync

//#region 🧪️Testkit
/// 📥️ Imports `n` checker frames as one new image-sequence stream via `ImportFramePayload`, mirroring
/// exactly what a real `importFrames` → `RequestFileOpen.multiple` re-dispatch loop sends. Shared with
/// `🎮️commands/🚀️run-reconstruction`'s own tests, which need real decodable frames to run a pipeline on.
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
