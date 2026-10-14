//! 🧰️ Remodeling editor kit — the plumbing every command, mode, window and panel of the remodeling editor shares:
//! app-addressed actions, fixed UI values and labels, the reset-document effect and payload decoding.

use semio_s_artifact_remodel_remodeling_images::editor::remodeling::engine::images as remodeling_image;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::REMODELING_DOCUMENT_SCHEMA;
use pack::snapshot::ArtifactPack;

//#region 🔖️Actions
/// 🪪️ The remodeling editor's app id — the controller every action of this editor addresses.
pub const REMODELING_PLAY_APP_ID: &str = "remodeling-play";
/// 🎯️ An `ActionDescriptor` addressed at this app — the single factory every taxonomy node's chrome
/// (`📌️panels/*`, `☑️options/*`) builds its `on_change`/item actions with.
pub fn remodeling_action(action: &str, args: Option<semio_framework_plugin::UiValue>) -> semio_framework_plugin::UiAssemblyResult<(semio_framework_plugin::ActionId, Option<semio_framework_plugin::UiValue>)> {
    semio_framework_plugin::ActionFactory::new(REMODELING_PLAY_APP_ID).action(action, args)
}

/// 🧱️ Admits one fixed UI text action value without JSON staging.
pub fn ui_value_text(value: impl AsRef<str>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiValue> {
    semio_framework_plugin::UiText::try_from_str(value.as_ref()).map(semio_framework_plugin::UiValue::Text).ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI text admission failed"))
}

/// 🔘️ Admits one boolean UI action value.
pub fn ui_value_bool(value: bool) -> semio_framework_plugin::UiValue {
    semio_framework_plugin::UiValue::Bool(value)
}

/// 🔢️ Admits one numeric UI action value.
pub fn ui_value_number(value: impl Into<f64>) -> semio_framework_plugin::UiValue {
    semio_framework_plugin::UiValue::Number(value.into())
}

/// 📚️ Admits one fixed UI list action value without dynamic staging.
pub fn ui_value_list(values: impl IntoIterator<Item = semio_framework_plugin::UiValue>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiValue> {
    let mut builder = semio_framework_plugin::UiListBuilder::try_new().ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI list admission failed"))?;
    for value in values {
        builder.push(value).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI list item admission failed"))?;
    }
    Ok(semio_framework_plugin::UiValue::List(builder.finish()))
}

/// 🗺️ Admits one ordered fixed UI map action value without JSON staging.
pub fn ui_value_map(values: impl IntoIterator<Item = (&'static str, semio_framework_plugin::UiValue)>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiValue> {
    let mut builder = semio_framework_plugin::UiMapBuilder::try_new().ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI map admission failed"))?;
    for (key, value) in values {
        builder.push(key.to_owned(), value).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI map entry admission failed"))?;
    }
    Ok(semio_framework_plugin::UiValue::Map(builder.finish()))
}

/// ☑️ A window-measure/`ActionDescriptor` action — `WindowMeasure` is the RETAINED (`ui_wgpu`) vocabulary
/// and takes a plain `ActionDescriptor { controller_id, action, args }`, not the fallible
/// `(ActionId, Option<UiValue>)` pair `remodeling_action` builds for semantic-contract chrome.
pub fn remodeling_window_action(action: &str, args: Option<semio_framework_value::DslValue>) -> semio_framework_plugin::ActionDescriptor {
    semio_framework_plugin::ActionDescriptor { controller_id: REMODELING_PLAY_APP_ID.into(), action: action.into(), args }
}

/// 🏷️ Admits one dynamic string as a semantic-contract `Label` — the SECOND `Label` type in scope
/// (`plugin_app_close_prelude::Label`, what `PanelTreeBuilder::section`/`tree_item` take), never the
/// retained `ui_wgpu` `Label` the SDK root's glob also re-exports and this file imports unqualified.
pub fn ui_label(value: impl AsRef<str>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_ui_contract::Label> {
    semio_framework_ui_contract::Label::try_from(value.as_ref()).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI label admission failed"))
}
//#endregion 🔖️Actions

//#region 🔖️ResetDocument
/// 🌱️ Builds an `Effect::LoadDocument` that swaps the live document to `document` OUTSIDE undo history — the sanctioned
/// path for an example switch, a JSON load or an import: the whole document is load content, never mutation rows and never
/// a before/after diff. The spr is a fresh, edit-free op-log for `document` (`store::os_store::text_format::empty_document_spr`).
pub fn reset_document_effect(document: &RemodelingSnapshot) -> semio_framework_plugin::Effect {
    let pack = <RemodelingSnapshot as ArtifactPack>::encode_pack(document);
    let spr = ::semio_framework_async::poll::resolve_ready(store::os_store::text_format::empty_document_spr("remodeling", REMODELING_DOCUMENT_SCHEMA));
    semio_framework_plugin::Effect::LoadDocument { pack, spr }
}
//#endregion 🔖️ResetDocument

//#region 🔖️Payloads
/// 📦️ Decodes a `requestFileOpen(readAs: "dataUrl")`/`RequestMediaFrames` payload into `(mime, bytes)`.
/// Relocated from the artifact's `⚙️engine/🦀️.rs` (#2553): its only three consumers are all
/// app-side (`🎮️commands/🖼️import-frame-payload`, `🎮️commands/🏗️run-reconstruction`, this app's own `import_media`).
pub fn payload_from_data_url(data_url: &str) -> Option<(String, Vec<u8>)> {
    let (header, encoded) = data_url.split_once(',')?;
    let mime = header.strip_prefix("data:")?.split(';').next().unwrap_or("application/octet-stream").to_string();
    let bytes = base64_codec::base64_standard_decode(encoded).ok()?;
    Some((mime, bytes))
}

/// 🖼️ Decodes a still-image payload by mime — three consumers (`🎮️commands/🖼️import-frame-payload`,
/// `🎮️commands/🏗️run-reconstruction`, this app's own `import_media`), and it takes no artifact-schema
/// type, so it stays app-side (relocated from `⚙️engine/🦀️.rs`, #2553).
pub fn decode_still_image(mime: &str, bytes: &[u8]) -> Result<remodeling_image::ImageRgba8, remodeling_image::ImageError> {
    if mime.contains("jpeg") || mime.contains("jpg") {
        remodeling_image::decode_jpeg(bytes)
    } else {
        remodeling_image::decode_png(bytes)
    }
}
//#endregion 🔖️Payloads

//#region 🏁️Fixtures
/// 🏁️ High-contrast `cell`-pixel checkerboard, PNG-encoded and base64-wrapped as a `requestFileOpen`
/// `dataUrl` payload — so the real decode path is exercised, not a stub.
pub async fn checker_data_url(w: u32, h: u32, cell: u32) -> String {
    format!("data:image/png;base64,{}", base64_codec::base64_standard_encode(remodeling_image::encode_png(&checker_image(w, h, cell)).expect("encode checker png")))
}

/// 🏁️ The same checkerboard, real-JPEG-encoded — mirrors what a `RequestMediaFrames` host actually
/// dispatches to `frame_action` (`payload: dataUrl(image/jpeg)`).
pub async fn checker_data_url_jpeg(w: u32, h: u32, cell: u32) -> String {
    format!("data:image/jpeg;base64,{}", base64_codec::base64_standard_encode(remodeling_image::encode_jpeg(&checker_image(w, h, cell), 90)))
}

/// 🎞️ A tiny synthesized MJPEG-in-MP4 video (n frames of the same checker pattern) as a
/// `RequestMediaFrames`-fallback-style raw base64 data URL payload.
pub async fn checker_video_data_url(n: u32, w: u32, h: u32, cell: u32) -> String {
    let jpeg = remodeling_image::encode_jpeg(&checker_image(w, h, cell), 90);
    let frames: Vec<Vec<u8>> = (0..n).map(|_| jpeg.clone()).collect();
    format!("data:video/mp4;base64,{}", base64_codec::base64_standard_encode(semio_s_artifact_remodel_remodeling_video::editor::remodeling::engine::video::container_providers::mp4::write_mjpeg(&frames, 10.0)))
}

pub fn checker_image(w: u32, h: u32, cell: u32) -> remodeling_image::ImageRgba8 {
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
//#endregion 🏁️Fixtures
