//! 📥️ Remodeling play app commands — `import-frame-payload`: one picked file of an import. A single picked file is ONE
//! plain commit. A multi-file pick is ONE streamed tool transaction (design §15) whose tool state lives in the picking
//! window's transient (`✏️editor/🫧️transient`): file `0` starts it, the first decodable file mints its stream, every file
//! but the last streams its frames into the open edit and the last (`index + 1 == total`) commits it as one edit and one
//! history row; `import-abort` (or the window closing) reverts it with zero trace, and a file arriving after that is
//! dropped. A still is one frame; a video file contributes its in-process sampled frames to the same stream.

use crate::editor::remodeling::commands::import_video_bytes_payload;
use crate::editor::remodeling::commands::import_video_frame_payload::import_transaction;
use crate::editor::remodeling::transient::{RemodelingImport, RemodelingWindowTransient};
use semio_framework_plugin::{NoConfig, NoConfigMutation};
#[cfg(test)]
use crate::editor::remodeling::engine::images as remodeling_image;
use crate::editor::remodeling::{decode_still_image, payload_from_data_url};
use crate::mutations::{add_stream_frame, create_asset, create_stream};
use crate::op::RemodelingMutation;
use crate::schema::mint_remodeling_id;
use crate::{FrameRef, ImageAsset, MediaKind, MediaStream, RemodelingSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Effect, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

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
        dispatch(app, RemodelingCommand::ImportFramePayload(ImportFramePayload { payload: checker_data_url(24, 24, 3).await, name: format!("frame-{index}.png"), index, total: n })).await;
    }
}

/// 🏁️ High-contrast `cell`-pixel checkerboard, PNG-encoded and base64-wrapped as a `requestFileOpen`
/// `dataUrl` payload — so the real decode path is exercised, not a stub.
#[cfg(test)]
pub(crate) async fn checker_data_url(w: u32, h: u32, cell: u32) -> String {
    format!("data:image/png;base64,{}", base64_codec::base64_standard_encode(remodeling_image::encode_png(&checker_image(w, h, cell)).expect("encode checker png")))
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
#[dsl(keyword = "import-frame-payload")]
pub struct ImportFramePayload {
    pub payload: String,
    pub name: String,
    pub index: u32,
    pub total: u32,
}

/// 🖼️ Appends `assets` to the image-sequence stream `stream_id` in order, as frames `n, n + 1, …` after its `n` frames —
/// creating the stream (named `name`) with the first when `scene` has none.
fn append_frames(scene: &RemodelingSnapshot, stream_id: &str, name: &str, assets: Vec<ImageAsset>) -> Vec<RemodelingMutation> {
    let existing = scene.streams.iter().find(|stream| stream.id == stream_id).map(|stream| stream.frames.len());
    let mut mutations = Vec::with_capacity(assets.len() * 2);
    for (offset, asset) in assets.into_iter().enumerate() {
        let index = (existing.unwrap_or(0) + offset) as u32;
        let asset_key = format!("{stream_id}-frame-{index}");
        let frame = FrameRef { index, timestamp_ms: f64::from(index) * 1000.0 / 30.0, asset_id: asset_key.clone() };
        mutations.push(create_asset(asset_key, asset));
        mutations.push(match (existing, offset) {
            (None, 0) => create_stream(MediaStream { id: stream_id.to_string(), name: name.to_string(), kind: MediaKind::ImageSequence, camera_id: None, sync_offset_ms: 0.0, fps_hint: 30.0, frames: vec![frame], source: None }),
            _ => add_stream_frame(stream_id.to_string(), frame, MediaKind::ImageSequence),
        });
    }
    mutations
}

/// 🖼️ The frames one picked file contributes — a still is one, a video its in-process sampled ones — and the notices of
/// a video that could not be read; an undecodable file contributes none.
fn file_frames(payload: &ImportFramePayload, scene: &RemodelingSnapshot) -> (Vec<ImageAsset>, Vec<Effect>) {
    match payload_from_data_url(&payload.payload) {
        Some((mime, bytes)) if mime.starts_with("video/") => match import_video_bytes_payload::sample_video(&bytes, scene) {
            Ok((sampled, _)) => (sampled.into_iter().map(|frame| frame.asset).collect(), Vec::new()),
            Err(message) => (Vec::new(), vec![Effect::Notify { message }]),
        },
        Some((mime, bytes)) => {
            let (width, height) = decode_still_image(&mime, &bytes).map_or((0, 0), |image| (image.width, image.height));
            (vec![ImageAsset { mime, data: base64_codec::base64_standard_encode(&bytes), width, height }], Vec::new())
        }
        None => (Vec::new(), Vec::new()),
    }
}

/// 📥️ One picked file through the picking window's tool state `window`: the emission and the window's next partition. A
/// single picked video is the in-process video import (its own video stream, provenance included).
pub fn handle_in_window(payload: &ImportFramePayload, doc: &ArtifactView<'_, RemodelingSnapshot>, cfg: &ConfigView<'_, NoConfig>, window: &RemodelingWindowTransient) -> Result<(Emit<RemodelingMutation, NoConfigMutation>, RemodelingWindowTransient), Fault> {
    let scene = doc.snapshot;
    let total = payload.total.max(1);
    if total == 1 && payload_from_data_url(&payload.payload).is_some_and(|(mime, _)| mime.starts_with("video/")) {
        let emit = import_video_bytes_payload::handle(&import_video_bytes_payload::ImportVideoBytesPayload { payload: payload.payload.clone(), name: payload.name.clone() }, doc, cfg)?;
        return Ok((emit, window.clone()));
    }
    let started = match (payload.index, total) {
        (_, 1) => Some(RemodelingImport { stream_id: None, done: 0, total }),
        (0, _) => Some(RemodelingImport { stream_id: None, done: 0, total }),
        _ => window.import.clone(),
    };
    let Some(mut import) = started else { return Ok((Emit::default(), window.clone())) };
    if import.stream_id.as_ref().is_some_and(|stream_id| !scene.streams.iter().any(|stream| stream.id == *stream_id)) {
        return Ok((Emit::default(), RemodelingWindowTransient::default()));
    }
    import.done = import.done.saturating_add(1);
    let last = payload.index.saturating_add(1) >= total;
    let (assets, effects) = file_frames(payload, scene);
    if !assets.is_empty() && import.stream_id.is_none() {
        import.stream_id = Some(mint_remodeling_id(doc.operation_optional(), "stream"));
    }
    let mutations = match &import.stream_id {
        Some(stream_id) if !assets.is_empty() => append_frames(scene, stream_id, &payload.name, assets),
        _ => Vec::new(),
    };
    let mut emit = match (&import.stream_id, last) {
        (Some(stream_id), true) => Emit::commit_transaction(import_transaction(stream_id), mutations),
        (Some(stream_id), false) if !mutations.is_empty() => Emit::stream_transaction(import_transaction(stream_id), mutations),
        _ => Emit::default(),
    };
    emit.effects.extend(effects);
    let next = match (total, last) {
        (1, _) => window.clone(),
        (_, true) => RemodelingWindowTransient::default(),
        (_, false) => RemodelingWindowTransient { import: Some(import) },
    };
    Ok((emit, next))
}

/// 📥️ One picked file dispatched without a window: a single file imports, a multi-file pick is refused at its first file
/// (its tool state needs the picking window) and a later file is dropped.
pub fn handle(payload: &ImportFramePayload, doc: &ArtifactView<'_, RemodelingSnapshot>, cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<RemodelingMutation, NoConfigMutation>, Fault> {
    let resting = RemodelingWindowTransient::default();
    let (emit, next) = handle_in_window(payload, doc, cfg, &resting)?;
    match next == resting {
        true => Ok(emit),
        false => Err(Fault::from("remodeling-import-window-required")),
    }
}
