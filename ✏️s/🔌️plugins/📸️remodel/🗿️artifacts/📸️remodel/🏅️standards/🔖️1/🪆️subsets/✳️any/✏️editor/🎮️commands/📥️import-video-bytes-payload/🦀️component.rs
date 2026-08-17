//! 📥️ 📥️ Remodel play app commands command — `import-video-bytes-payload`.

use crate::editor::remodel::config::{RemodelConfig, RemodelConfigMutation};
use crate::editor::remodel::engine::{describe_video_probe, images as remodel_image, video as remodel_video, video_codec_to_artifact};
use crate::editor::remodel::payload_from_data_url;
use crate::artifacts::remodel::mutations::{create_asset, create_stream};
use crate::artifacts::remodel::schema::next_remodel_id;
use crate::artifacts::remodel::op::RemodelMutation;
use crate::artifacts::remodel::{FrameRef, ImageAsset, MediaKind, MediaStream, RemodelSnapshot, VideoSource};
use base64::Engine as _;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault, Effect};
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
pub(crate) fn testkit_import_checker_stream(app: &mut crate::editor::remodel::testkit::RemodelApp, n: u32) {
    use crate::editor::remodel::testkit::dispatch;
    use crate::editor::remodel::RemodelCommand;
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
        Err(error) => return Ok(Emit::effect(Effect::Notify { message: format!("Could not probe video: {error}") })),
    };
    let (codec, width, height, duration_ms, container) = describe_video_probe(&probe);
    let scene = doc.snapshot;
    let ingest = &scene.params.ingest;
    let opts = remodel_video::VideoIngestOptions { stride: ingest.frame_sample_stride.max(1), max_frames: ingest.max_frames, max_long_edge_px: ingest.downscale_long_edge_px };
    let iter = match remodel_video::extract_frames(&bytes, &opts) {
        Ok(iter) => iter,
        Err(error) => return Ok(Emit::effect(Effect::Notify { message: format!("Unsupported video codec ({codec:?}): {error} - probed {container} {width}x{height}") })),
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
