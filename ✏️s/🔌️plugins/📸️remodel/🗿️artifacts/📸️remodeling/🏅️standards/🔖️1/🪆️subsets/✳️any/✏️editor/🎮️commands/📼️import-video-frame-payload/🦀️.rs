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
use crate::editor::remodeling::engine::reconstruction::{blur_gate_admits, sharpness_score, BLUR_GATE_ROLLING_WINDOW};
#[cfg(test)]
use crate::editor::remodeling::engine::video as remodeling_video;
use crate::editor::remodeling::payload_from_data_url;
use crate::editor::remodeling::transient::{RemodelingImport, RemodelingWindowTransient};
use crate::mutations::{add_stream_frame, create_asset, create_stream};
use crate::standards::v1::subsets::any::schema::mutations::RemodelingMutation;
use crate::schema::mint_remodeling_id;
use crate::{FrameRef, ImageAsset, MediaKind, MediaStream, RemodelingSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, FaultCode, FaultOrigin};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🧾️ImportTransaction
/// 🪪️ The tool every import transaction is stamped with: `<appId>#import`.
pub const REMODELING_IMPORT_TOOL_ID: &str = "s.remodel.remodeling@1/*#editor#import";

/// 🧾️ The ONE transaction an import building `stream_id` streams into, minted from that stream: every tick of the import,
/// its commit and its abort name the same ref.
pub fn import_transaction(stream_id: &str) -> protocol::TransactionRef {
    protocol::TransactionRef::mint(&protocol::ActorId(stream_id.into()), &protocol::HybridLogicalTimestamp { actor: 0, physical_ms: 0, logical: 0 }, REMODELING_IMPORT_TOOL_ID)
}
//#endregion 🧾️ImportTransaction

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

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "import-video-frame-payload")]
pub struct ImportVideoFramePayload {
    pub payload: String,
    pub name: String,
    pub index: u32,
    pub frame_index: u32,
    pub timestamp_ms: f64,
}

/// 🎞️ Host-decoded video frame tick through the importing window's tool state `window`: decodes the sampled JPEG, runs it
/// through the engine's one relative blur gate over the import's own rolling scores (kept in `window`, so a tick costs one decode) and
/// streams it into the import's open transaction; the emission and the window's next partition. Tick `0` while this
/// window's earlier import still streams is refused (`remodeling.import.open`): one window imports one stream at a time.
pub fn handle_in_window(payload: &ImportVideoFramePayload, doc: &ArtifactView<'_, RemodelingSnapshot>, window: &RemodelingWindowTransient) -> Result<(Emit<RemodelingMutation, NoConfigMutation>, RemodelingWindowTransient), Fault> {
    let scene = doc.snapshot;
    let started = match payload.index {
        0 => {
            refuse_while_streaming(scene, window)?;
            Some(RemodelingImport::default())
        }
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
    if !blur_gate_admits(&mut import.rolling_scores, BLUR_GATE_ROLLING_WINDOW, sharpness_score(&image), scene.params.ingest.min_sharpness) {
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
        false => Err(import_window_required()),
    }
}

/// 🪟️ The refusal of a streamed import dispatched without a window: its tool state lives in the importing window.
pub(crate) fn import_window_required() -> Fault {
    Fault::new(FaultOrigin::App, FaultCode::new("remodeling.import.window-required"), "A streamed import keeps its progress in the window that starts it; dispatch it from a window.")
}

/// 🚧️ Refuses a new import in a window whose earlier import still streams into an open transaction (its stream is in the
/// document): that import commits or aborts first.
pub(crate) fn refuse_while_streaming(scene: &RemodelingSnapshot, window: &RemodelingWindowTransient) -> Result<(), Fault> {
    match window.import.as_ref().and_then(|import| import.stream_id.as_ref()).is_some_and(|stream_id| scene.streams.iter().any(|stream| stream.id == *stream_id)) {
        true => Err(Fault::new(FaultOrigin::App, FaultCode::new("remodeling.import.open"), "This window is still importing; finish or cancel that import first.")),
        false => Ok(()),
    }
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
