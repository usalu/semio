//! 👁️ DXF viewer — the Main window: a read-only 2D `ImageWindowKit` render of the drawing, projected by the shared pure DXF preview.
//! This file imports nothing from the sibling editor module.

use crate::standards::v_r12::subsets::any::io::text::snapshot::preview::{preview_svg, DXF_PREVIEW_CANVAS};
use crate::standards::v_r12::subsets::any::schema::snapshot::DxfSnapshot;
use semio_framework_plugin::app::{ImageView, ImageWindowKit};
use semio_framework_plugin::{BuiltNode, WindowKindDefinition, WindowKit};

pub const WINDOW_KIND_ID: &str = ImageWindowKit::KIND_ID;
pub const BODY_KEY: &str = ImageWindowKit::KIND_ID;

/// 🧱️ Stitched into the viewer manifest by the surface root's `create_*_viewer`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn definition() -> WindowKindDefinition {
    ImageWindowKit::window_kind()
}

/// 👁️ Pure `DxfSnapshot -> BuiltNode` read.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn render(document: &DxfSnapshot) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let (width, height) = DXF_PREVIEW_CANVAS;
    ImageWindowKit::render(&ImageView { width, height, mime: "image/svg+xml".into(), base64: semio_s_artifact_stdio_contract::base64_standard(preview_svg(document).as_bytes()) })
}
