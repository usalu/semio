//! 📥️ 📥️ Remodeling play app commands command — `import-video-bytes-payload`.

use crate::editor::remodeling::commands::import_video_frame_payload::import_transaction;
use semio_framework_plugin::{NoConfig, NoConfigMutation};
use crate::editor::remodeling::engine::{describe_video_probe, images as remodeling_image, video as remodeling_video, video_codec_to_artifact};
use crate::editor::remodeling::payload_from_data_url;
use crate::mutations::{create_asset, create_stream};
use crate::op::RemodelingMutation;
use crate::schema::mint_remodeling_id;
use crate::{FrameRef, ImageAsset, MediaKind, MediaStream, RemodelingSnapshot, VideoSource};
use semio_framework_plugin::{ArtifactView, ConfigView, Effect, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};
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
fn local_sharpness_score(image: &remodeling_image::ImageRgba8) -> f32 {
    let gray = remodeling_image::ImageGray::from_rgba8_luma(image);
    let grad = remodeling_image::scharr_gradients(&gray);
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

/// 🎞️ One in-process sampled video frame, JPEG-encoded, with its sample index and timestamp.
pub struct SampledFrame {
    pub asset: ImageAsset,
    pub index: u32,
    pub timestamp_ms: f64,
}

/// 🎞️ Probes, demuxes, decodes and blur-gates a video container fully in-process under the scene's ingest parameters:
/// the sampled frames and the probed provenance, or the notice naming why the container could not be read.
pub fn sample_video(bytes: &[u8], scene: &RemodelingSnapshot) -> Result<(Vec<SampledFrame>, VideoSource), String> {
    let probe = remodeling_video::probe(bytes).map_err(|error| format!("Could not probe video: {error}"))?;
    let (codec, width, height, duration_ms, container) = describe_video_probe(&probe);
    let ingest = &scene.params.ingest;
    let opts = remodeling_video::VideoIngestOptions { stride: ingest.frame_sample_stride.max(1), max_frames: ingest.max_frames, max_long_edge_px: ingest.downscale_long_edge_px };
    let iter = remodeling_video::extract_frames(bytes, &opts).map_err(|error| format!("Unsupported video codec ({codec:?}): {error} - probed {container} {width}x{height}"))?;
    let mut scratch = VideoImportScratch::default();
    let mut frames = Vec::new();
    for extracted in iter {
        let Ok(extracted) = extracted else { continue };
        if blur_gate_reject(&mut scratch, local_sharpness_score(&extracted.image), ingest.min_sharpness) {
            continue;
        }
        let jpeg = remodeling_image::encode_jpeg(&extracted.image, 90);
        frames.push(SampledFrame { asset: ImageAsset { mime: "image/jpeg".into(), data: base64_codec::base64_standard_encode(&jpeg), width: extracted.image.width, height: extracted.image.height }, index: extracted.index, timestamp_ms: extracted.timestamp_ms });
    }
    Ok((frames, VideoSource { name: String::new(), container: container.into(), codec: video_codec_to_artifact(codec), duration_ms, frame_count: 0, width, height }))
}

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "import-video-bytes-payload")]
pub struct ImportVideoBytesPayload {
    pub payload: String,
    pub name: String,
}

/// 🎞️ Tier-3 fallback (or a single picked video in `ImportFramePayload`): the host couldn't decode the video, so it hands
/// over the raw container bytes and [`sample_video`] extracts the frames fully in-process. The whole video lands in this
/// ONE call as one new video stream and commits as ONE import transaction (one edit, one history row, one undo step). An
/// unreadable container surfaces as a `Notify` naming it.
pub fn handle(payload: &ImportVideoBytesPayload, doc: &ArtifactView<'_, RemodelingSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<RemodelingMutation, NoConfigMutation>, Fault> {
    let Some((_mime, bytes)) = payload_from_data_url(&payload.payload) else { return Ok(Emit::default()) };
    let (sampled, source) = match sample_video(&bytes, doc.snapshot) {
        Ok(sampled) => sampled,
        Err(message) => return Ok(Emit::effect(Effect::Notify { message })),
    };
    let stream_id = mint_remodeling_id(doc.operation_optional(), "stream");
    let mut mutations = Vec::with_capacity(sampled.len() + 1);
    let mut frames = Vec::with_capacity(sampled.len());
    for frame in sampled {
        let asset_key = format!("{stream_id}-frame-{}", frame.index);
        mutations.push(create_asset(asset_key.clone(), frame.asset));
        frames.push(FrameRef { index: frame.index, timestamp_ms: frame.timestamp_ms, asset_id: asset_key });
    }
    let transaction = import_transaction(&stream_id);
    mutations.push(create_stream(MediaStream { id: stream_id, name: payload.name.clone(), kind: MediaKind::Video, camera_id: None, sync_offset_ms: 0.0, fps_hint: 0.0, frames, source: Some(source) }));
    Ok(Emit::commit_transaction(transaction, mutations))
}
