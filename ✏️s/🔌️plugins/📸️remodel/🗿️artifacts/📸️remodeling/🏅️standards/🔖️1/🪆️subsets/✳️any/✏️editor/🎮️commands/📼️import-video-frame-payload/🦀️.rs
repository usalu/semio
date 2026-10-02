//! 📥️ Remodeling play app commands — `import-video-frame-payload`: one tick of a host-decoded video import. Every import
//! is ONE streamed tool transaction (design §15) whose tool state lives in the importing window's transient
//! (`✏️editor/🫧️transient`): tick `0` starts it, the first accepted frame mints its stream and opens the edit (the
//! document shows the frames at once), `import-video-done` commits it as one edit and one history row, `import-abort` (or
//! the window closing) reverts it with zero trace, and a tick arriving after that is dropped. The transaction is minted
//! from the stream the import builds.

#[cfg(test)]
use crate::editor::remodeling::commands::import_frame_payload;
use semio_framework_plugin::{NoConfig, NoConfigMutation};
use crate::editor::remodeling::engine::images as remodeling_image;
#[cfg(test)]
use crate::editor::remodeling::engine::video as remodeling_video;
use crate::editor::remodeling::payload_from_data_url;
use crate::editor::remodeling::transient::{RemodelingImport, RemodelingWindowTransient};
use crate::mutations::{add_stream_frame, create_asset, create_stream};
use crate::op::RemodelingMutation;
use crate::schema::mint_remodeling_id;
use crate::{FrameRef, ImageAsset, MediaKind, MediaStream, RemodelingSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};
use std::collections::VecDeque;

//#region 🧾️ImportTransaction
/// 🪪️ The tool every import transaction is stamped with: `<appId>#import`.
pub const REMODELING_IMPORT_TOOL_ID: &str = "s.remodel.remodeling@1/*#editor#import";

/// 🧾️ The ONE transaction an import building `stream_id` streams into, minted from that stream: every tick of the import,
/// its commit and its abort name the same ref.
pub fn import_transaction(stream_id: &str) -> protocol::TransactionRef {
    protocol::TransactionRef::mint(&protocol::ActorId(stream_id.to_string()), &protocol::HybridLogicalTimestamp { actor: 0, physical_ms: 0, logical: 0 }, REMODELING_IMPORT_TOOL_ID)
}
//#endregion 🧾️ImportTransaction

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

/// 🧩️ Pure reconstruction of the blur-gate rolling window from `stream_id`'s already-persisted frames
/// (most recent `BLUR_GATE_ROLLING_WINDOW` first, then scored oldest-to-newest so the window fills in
/// the same order the original per-tick `RefCell` scratch would have) — the pure-trait replacement
/// for carrying `VideoImportScratch` as hidden interior-mutable state across `ImportVideoFramePayload`
/// ticks.
fn rebuild_video_import_scratch(scene: &RemodelingSnapshot, stream_id: &str) -> VideoImportScratch {
    let mut scratch = VideoImportScratch::default();
    let Some(stream) = scene.streams.iter().find(|stream| stream.id == stream_id) else { return scratch };
    let mut recent: Vec<&FrameRef> = stream.frames.iter().rev().take(BLUR_GATE_ROLLING_WINDOW).collect();
    recent.reverse();
    for frame in recent {
        let Some(source) = crate::remodeling_asset_chunk_source(scene, &frame.asset_id) else { continue };
        let Ok(rope) = remodeling_image::CompressedChunkRope::from_leaves(source.leaves, 1_114_112) else { continue };
        let mut decoder = remodeling_image::BoundedStillDecoder::new(&source.mime, rope);
        let image = loop {
            match decoder.advance() {
                remodeling_image::BoundedDecodeProgress::Working => {}
                remodeling_image::BoundedDecodeProgress::Complete(image) => break Some(image),
                remodeling_image::BoundedDecodeProgress::Failed(_) => break None,
            }
        };
        let Some(image) = image else { continue };
        scratch.rolling_scores.push_back(local_sharpness_score(&image));
    }
    scratch
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

//#region 🧪️UnitTests
/// 📥️ Imports `n` checker frames as one new image-sequence stream via `ImportFramePayload`, mirroring
/// exactly what a real `importFrames` → `RequestFileOpen.multiple` re-dispatch loop sends. Shared with
/// `🎮️commands/🏗️run-reconstruction`'s own tests, which need real decodable frames to run a pipeline on.
#[cfg(test)]
pub(crate) async fn verify_import_checker_stream(app: &mut crate::editor::remodeling::unit_tests::context::RemodelingApp, n: u32) {
    use crate::editor::remodeling::unit_tests::context::dispatch;
    use crate::editor::remodeling::RemodelingCommand;
    for index in 0..n {
        dispatch(app, RemodelingCommand::ImportFramePayload(import_frame_payload::ImportFramePayload { payload: checker_data_url(24, 24, 3).await, name: format!("frame-{index}.png"), index, total: n })).await;
    }
}

/// 🏁️ High-contrast `cell`-pixel checkerboard, PNG-encoded and base64-wrapped as a `requestFileOpen`
/// `dataUrl` payload — so the real decode path is exercised, not a stub.
#[cfg(test)]
pub(crate) async fn checker_data_url(w: u32, h: u32, cell: u32) -> String {
    format!("data:image/png;base64,{}", base64_codec::base64_standard_encode(remodeling_image::encode_png(&checker_image(w, h, cell)).expect("encode checker png")))
}

/// 🏁️ The same checkerboard, real-JPEG-encoded — mirrors what a `RequestMediaFrames` host actually
/// dispatches to `frame_action` (`payload: dataUrl(image/jpeg)`).
#[cfg(test)]
pub(crate) async fn checker_data_url_jpeg(w: u32, h: u32, cell: u32) -> String {
    format!("data:image/jpeg;base64,{}", base64_codec::base64_standard_encode(remodeling_image::encode_jpeg(&checker_image(w, h, cell), 90)))
}

/// 🎞️ A tiny synthesized MJPEG-in-MP4 video (n frames of the same checker pattern) as a
/// `RequestMediaFrames`-fallback-style raw base64 data URL payload.
#[cfg(all(test, feature = "video-mp4"))]
pub(crate) async fn checker_video_data_url(n: u32, w: u32, h: u32, cell: u32) -> String {
    let jpeg = remodeling_image::encode_jpeg(&checker_image(w, h, cell), 90);
    let frames: Vec<Vec<u8>> = (0..n).map(|_| jpeg.clone()).collect();
    format!("data:video/mp4;base64,{}", base64_codec::base64_standard_encode(remodeling_video::container_providers::mp4::write_mjpeg(&frames, 10.0)))
}

#[cfg(test)]
fn checker_image(w: u32, h: u32, cell: u32) -> remodeling_image::ImageRgba8 {
    let mut image = remodeling_image::ImageRgba8::new(w, h);
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
//#endregion 🧪️UnitTests

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "import-video-frame-payload")]
pub struct ImportVideoFramePayload {
    pub payload: String,
    pub name: String,
    pub index: u32,
    pub frame_index: u32,
    pub timestamp_ms: f64,
}

/// 🎞️ Host-decoded video frame tick through the importing window's tool state `window`: decodes the sampled JPEG, runs it
/// through the relative blur gate (rebuilt from the stream's frames each tick — see `rebuild_video_import_scratch`) and
/// streams it into the import's open transaction; the emission and the window's next partition.
pub fn handle_in_window(payload: &ImportVideoFramePayload, doc: &ArtifactView<'_, RemodelingSnapshot>, window: &RemodelingWindowTransient) -> Result<(Emit<RemodelingMutation, NoConfigMutation>, RemodelingWindowTransient), Fault> {
    let scene = doc.snapshot;
    let started = match payload.index {
        0 => Some(RemodelingImport::default()),
        _ => window.import.clone(),
    };
    let Some(mut import) = started else { return Ok((Emit::default(), window.clone())) };
    if import.stream_id.as_ref().is_some_and(|stream_id| !scene.streams.iter().any(|stream| stream.id == *stream_id)) {
        return Ok((Emit::default(), RemodelingWindowTransient::default()));
    }
    import.done = import.done.saturating_add(1);
    let accepted = payload_from_data_url(&payload.payload).and_then(|(_mime, bytes)| remodeling_image::decode_jpeg(&bytes).ok().map(|image| (bytes, image)));
    let Some((bytes, image)) = accepted else { return Ok((Emit::default(), RemodelingWindowTransient { import: Some(import) })) };
    let stream_id = import.stream_id.clone().unwrap_or_else(|| mint_remodeling_id(doc.operation_optional(), "stream"));
    let mut scratch = rebuild_video_import_scratch(scene, &stream_id);
    if blur_gate_reject(&mut scratch, local_sharpness_score(&image), scene.params.ingest.min_sharpness) {
        return Ok((Emit::default(), RemodelingWindowTransient { import: Some(import) }));
    }
    let asset_key = format!("{stream_id}-frame-{}", payload.frame_index);
    let asset = ImageAsset { mime: "image/jpeg".into(), data: base64_codec::base64_standard_encode(&bytes), width: image.width, height: image.height };
    let frame = FrameRef { index: payload.frame_index, timestamp_ms: payload.timestamp_ms, asset_id: asset_key.clone() };
    let mut mutations = vec![create_asset(asset_key, asset)];
    match scene.streams.iter().any(|stream| stream.id == stream_id) {
        true => mutations.push(add_stream_frame(stream_id.clone(), frame, MediaKind::Video)),
        false => mutations.push(create_stream(MediaStream { id: stream_id.clone(), name: payload.name.clone(), kind: MediaKind::Video, camera_id: None, sync_offset_ms: 0.0, fps_hint: 0.0, frames: vec![frame], source: None })),
    }
    import.stream_id = Some(stream_id.clone());
    Ok((Emit::stream_transaction(import_transaction(&stream_id), mutations), RemodelingWindowTransient { import: Some(import) }))
}

/// 🎞️ A video frame tick dispatched without a window: refused at the first tick (an import's tool state needs the
/// importing window), a later tick is dropped.
pub fn handle(payload: &ImportVideoFramePayload, doc: &ArtifactView<'_, RemodelingSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<RemodelingMutation, NoConfigMutation>, Fault> {
    let resting = RemodelingWindowTransient::default();
    let (emit, next) = handle_in_window(payload, doc, &resting)?;
    match next == resting {
        true => Ok(emit),
        false => Err(Fault::from("remodeling-import-window-required")),
    }
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
