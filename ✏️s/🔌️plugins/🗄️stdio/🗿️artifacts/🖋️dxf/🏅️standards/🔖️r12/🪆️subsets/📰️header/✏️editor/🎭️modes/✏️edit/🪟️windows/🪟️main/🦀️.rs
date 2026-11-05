//! 🖼️ DXF editor — Main window: a real 2D `ImageWindowKit` render of the drawing (read-only canvas); typed geometry and layer edits live in Details.

pub use crate::standards::v_r12::subsets::any::io::text::snapshot::preview::preview_svg;
use crate::standards::v_r12::subsets::any::io::text::snapshot::preview::DXF_PREVIEW_CANVAS;
use crate::standards::v_r12::subsets::any::schema::snapshot::DxfSnapshot;
use semio_framework_plugin::app::{ImageView, ImageWindowKit};
use semio_framework_plugin::{BuiltNode, WindowKindDefinition, WindowKit};

pub const WINDOW_KIND_ID: &str = ImageWindowKit::KIND_ID;
pub const BODY_KEY: &str = ImageWindowKit::KIND_ID;

/// 🧱️ Stitched into the editor manifest by the surface root's `create_*_editor`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn definition() -> WindowKindDefinition {
    ImageWindowKit::window_kind()
}

/// ✏️ Real read of the current document.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn render(document: &DxfSnapshot) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let (width, height) = DXF_PREVIEW_CANVAS;
    ImageWindowKit::render(&ImageView { width, height, mime: "image/svg+xml".into(), base64: semio_s_artifact_stdio_contract::base64_standard(preview_svg(document).as_bytes()) })
}
