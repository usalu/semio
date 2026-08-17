//! 🖼️ Raster viewer — the Composite window: a read-only render of the full composited document,
//! built from the same artifact-level `🚪️io` SVG/PNG bridge the editor's own `raster_composite_media`
//! uses — this file imports nothing from the sibling editor surface (`policyViewerPurityBreaches`
//! forbids it outright). No selection, no brush/eraser chrome, no utilities: a viewer has no actions
//! that edit and emits no mutations by construction (`ViewEmit`). Uses the frozen `ImageWindowKit`
//! (contract §2.6) as raster's right base — this artifact IS a pixel image.

use base64::Engine;
use crate::artifacts::raster::RasterSnapshot;
use semio_framework_plugin::app::{ImageView, ImageWindowKit, WindowKit};
use semio_framework_plugin::UiNode;

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
/// 👁️ Pure `RasterSnapshot -> UiNode` read: composites the real layer stack to a canonical PNG through
/// the same artifact-level `🚪️io` bridge the editor's `raster_composite_media` uses
/// (`raster_document_json_to_svg` → `rasterize_svg_to_png_base64` → `canonicalize_png_bytes`), then
/// hands the pixels to `ImageWindowKit::render` — never a bespoke renderer, never a call through the
/// sibling editor module.
pub fn render(document: &RasterSnapshot) -> UiNode {
    ImageWindowKit::render(&composited_image_view(document))
}

/// 🧭️ `pub(super)` — the sibling `🧭️navigator` window reuses this exact composited view-model (same
/// real pixels, not different content) rather than re-deriving it.
pub fn composited_image_view(document: &RasterSnapshot) -> ImageView {
    composite_document_to_png(document).unwrap_or_else(|| ImageView { width: 1, height: 1, mime: "image/png".into(), base64: RASTER_VIEW_FALLBACK_PNG_BASE64.into() })
}

/// 🌉️ The read-only composite primitive — real pixels, never a placeholder title card, matching the
/// editor's own `raster_composite_media` fidelity exactly (same three-step bridge), just returning the
/// framework's `ImageView` view-model instead of a `Media` payload.
fn composite_document_to_png(document: &RasterSnapshot) -> Option<ImageView> {
    let value = serde_json::to_value(document).ok()?;
    let (svg, width, height) = crate::artifacts::raster::io::raster_document_json_to_svg(&value).ok()?;
    let rendered_base64 = semio_framework_os::rasterize_svg_to_png_base64(&svg, width, height).ok()?;
    let raw_bytes = base64::engine::general_purpose::STANDARD.decode(rendered_base64.as_bytes()).ok()?;
    let canonical = crate::artifacts::raster::io::canonicalize_png_bytes(&raw_bytes).ok()?;
    let base64_string = base64::engine::general_purpose::STANDARD.encode(canonical);
    Some(ImageView { width, height, mime: "image/png".into(), base64: base64_string })
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn definition_declares_the_frozen_image_window_kind() {
        let def = definition();
        assert_eq!(def.id, ImageWindowKit::KIND_ID);
        assert_eq!(def.body_key, ImageWindowKit::KIND_ID);
    }

    #[test]
    fn render_produces_a_scene_node_for_the_default_document() {
        let document = crate::artifacts::raster::schema::empty_raster_document();
        let _node = render(&document);
    }
}
//#endregion 🧪️Tests
