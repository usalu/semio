//! 🖼️ DWG AC1024 viewer — Main window: a real 2D `ImageWindowKit` render of the typed drawing geometry (read-only).
//! This file imports nothing from the sibling editor module.
Objects without a typed geometry stay in the logical model untouched and are not drawn.

use semio_s_artifact_stdio_dwg_snapshot::standards::v_ac1024::subsets::any::schema::snapshot::DwgSnapshot;
use semio_s_artifact_stdio_dwg_snapshot_drawing::standards::v_ac1024::subsets::any::schema::snapshot::drawing::preview::{preview_svg, DWG_PREVIEW_CANVAS};
use semio_plugin::app::{ImageView, ImageWindowKit};
use semio_plugin::{BuiltNode, WindowKindDefinition, WindowKit};

pub const WINDOW_KIND_ID: &str = ImageWindowKit::KIND_ID;
pub const BODY_KEY: &str = ImageWindowKit::KIND_ID;

/// 🧱️ Stitched into the viewer manifest by the surface root's `create_*_viewer`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn definition() -> WindowKindDefinition {
    ImageWindowKit::window_kind()
}

/// 🖼️ Pure `DwgSnapshot -> BuiltNode` read.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn render(document: &DwgSnapshot) -> semio_plugin::UiAssemblyResult<BuiltNode> {
    let (width, height) = DWG_PREVIEW_CANVAS;
    ImageWindowKit::render(&ImageView { width, height, mime: "image/svg+xml".into(), base64: semio_s_artifact_stdio_contract::base64_standard(preview_svg(&document.drawing).as_bytes()) })
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
