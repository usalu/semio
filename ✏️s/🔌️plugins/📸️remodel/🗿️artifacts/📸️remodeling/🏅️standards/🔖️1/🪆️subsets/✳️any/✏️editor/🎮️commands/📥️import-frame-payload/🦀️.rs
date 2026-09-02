//! 📥️ 📥️ Remodeling play app commands command — `import-frame-payload`.

use crate::artifacts::remodeling::mutations::{add_stream_frame, create_asset, create_stream};
use crate::artifacts::remodeling::op::RemodelingMutation;
use crate::artifacts::remodeling::schema::next_remodeling_id;
use crate::artifacts::remodeling::{FrameRef, ImageAsset, MediaKind, MediaStream, RemodelingSnapshot};
use crate::editor::remodeling::commands::import_video_bytes_payload;
use crate::editor::remodeling::config::{RemodelingConfig, RemodelingConfigMutation};
#[cfg(test)]
use crate::editor::remodeling::engine::images as remodeling_image;
use crate::editor::remodeling::{decode_still_image, payload_from_data_url};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️VideoImportScratch
/// 🆔️ The stream a batch tick lands on: `index == 0` starts a new stream, `index > 0` appends to
/// `scene.streams.last()` — the stream THIS batch's `index == 0` call just created (each call sees the
/// prior call's already-committed mutations, since dispatches within one batch are sequential).
fn batch_stream_id(scene: &RemodelingSnapshot, index: u32) -> String {
    if index == 0 {
        next_remodeling_id("stream")
    } else {
        scene.streams.last().map_or_else(|| next_remodeling_id("stream"), |stream| stream.id.clone())
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
pub(crate) async fn testkit_import_checker_stream(app: &mut crate::editor::remodeling::testkit::RemodelingApp, n: u32) {
    use crate::editor::remodeling::testkit::dispatch;
    use crate::editor::remodeling::RemodelingCommand;
    for index in 0..n {
        dispatch(app, RemodelingCommand::ImportFramePayload(ImportFramePayload { payload: checker_data_url(24, 24, 3), name: format!("frame-{index}.png"), index }));
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
#[cfg(test)]
pub(crate) async fn checker_video_data_url(n: u32, w: u32, h: u32, cell: u32) -> String {
    let jpeg = remodeling_image::encode_jpeg(&checker_image(w, h, cell), 90);
    let frames: Vec<Vec<u8>> = (0..n).map(|_| jpeg.clone()).collect();
    format!("data:video/mp4;base64,{}", base64_codec::base64_standard_encode(remodeling_video::write_mp4_mjpeg(&frames, 10.0)))
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
//#endregion 🧪️Testkit

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "import-frame-payload")]
pub struct ImportFramePayload {
    pub payload: String,
    pub name: String,
    pub index: u32,
}

/// 📥️ A still-image drop-zone/file-picker payload; a `video/*` mime is re-routed to the in-process
/// video-bytes decoder.
pub async fn handle(payload: &ImportFramePayload, doc: &ArtifactView<'_, RemodelingSnapshot>, cfg: &ConfigView<'_, RemodelingConfig>) -> Result<Emit<RemodelingMutation, RemodelingConfigMutation>, Fault> {
    let Some((mime, bytes)) = payload_from_data_url(&payload.payload) else { return Ok(Emit::default()) };
    if mime.starts_with("video/") {
        return import_video_bytes_payload::handle(&import_video_bytes_payload::ImportVideoBytesPayload { payload: payload.payload.clone(), name: payload.name.clone() }, doc, cfg);
    }
    let scene = doc.snapshot;
    let stream_id = batch_stream_id(scene, payload.index);

    let (width, height) = decode_still_image(&mime, &bytes).map_or((0, 0), |image| (image.width, image.height));
    let asset_key = format!("{stream_id}-frame-{}", payload.index);
    let asset = ImageAsset { mime, data: base64_codec::base64_standard_encode(&bytes), width, height };

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
    Ok(Emit::amend(mutations, format!("remodeling-import:{stream_id}")))
}
