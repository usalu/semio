//! 🖼️ Raster viewer — the Composite window: a read-only render of the full composited document,
//! built from the same artifact-level `🚪️io` SVG/PNG bridge the editor's own `raster_composite_media`
//! uses — this file imports nothing from the sibling editor surface (`policyViewerPurityBreaches`
//! forbids it outright). No selection, no brush/eraser chrome, no utilities: a viewer has no actions
//! that edit and emits no mutations by construction (`ViewEmit`). Uses the frozen `ImageWindowKit`
//! (contract §2.6) as raster's right base — this artifact IS a pixel image.

use crate::RasterSnapshot;
use semio_framework_plugin::app::{ImageView, ImageWindowKit, WindowKit};
use semio_framework_plugin::{BuiltNode, UiAssemblyResult};

//#region 🔖️Constants
pub const RASTER_VIEW_WINDOW_COMPOSITE: &str = ImageWindowKit::KIND_ID;
pub const RASTER_VIEW_BODY_COMPOSITE: &str = ImageWindowKit::KIND_ID;
/// 🖼️ A well-known 1x1 transparent PNG — the fail-soft fallback `render` returns when the document
/// fails to composite (mirrors the fail-soft cache-miss pattern every other exemplar in this ticket
/// documents rather than papers over; never a panic).
const RASTER_VIEW_FALLBACK_PNG_BASE64: &str = "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR42mNk+A8AAQUBAScY42YAAAAASUVORK5CYII=";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::viewer::raster::create_raster_viewer` — the read-only
/// `ImageWindowKit::window_kind()` variant verbatim (never `editable_window_kind()`, which declares the
/// mutating `set-pixel-region` command a viewer must never carry).
pub fn definition() -> semio_framework_plugin::WindowKindDefinition {
    ImageWindowKit::window_kind()
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 👁️ Displays the canonical layer composite through the shared image window.
pub fn render(document: &RasterSnapshot) -> UiAssemblyResult<BuiltNode> {
    ImageWindowKit::render(&composited_image_view(document))
}

/// 🧭️ `pub(super)` — the sibling `🧭️navigator` window reuses this exact composited view-model (same
/// real pixels, not different content) rather than re-deriving it.
pub fn composited_image_view(document: &RasterSnapshot) -> ImageView {
    composite_document_to_png(document).unwrap_or_else(|| ImageView { width: 1, height: 1, mime: "image/png".into(), base64: RASTER_VIEW_FALLBACK_PNG_BASE64.into() })
}

/// 🌉️ Uses the same compositor and PNG encoder as pixel export and image ports.
fn composite_document_to_png(document: &RasterSnapshot) -> Option<ImageView> {
    let image=crate::standards::v1::subsets::any::io::raster_composite_image(document).ok()?;
    let bytes=crate::standards::v1::subsets::any::io::png_bytes_from_semio_image(&image).ok()?;
    Some(ImageView { width:image.width, height:image.height, mime:"image/png".into(), base64:base64_codec::base64_standard_encode(bytes) })
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
