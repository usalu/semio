//! 👁️ `svg` view (tiny) — Main window: real `ImageWindowKit`
//! render of the current document (read-only).

use crate::standards::v1_1::subsets::tiny::schema::snapshot::write_svg_xml;
use crate::standards::v1_1::subsets::tiny::schema::snapshot::SvgSnapshot;
use semio_framework_plugin::app::{ImageView, ImageWindowKit};
use semio_framework_plugin::{BuiltNode, WindowKindDefinition, WindowKit};

pub const WINDOW_KIND_ID: &str = ImageWindowKit::KIND_ID;
pub const BODY_KEY: &str = ImageWindowKit::KIND_ID;

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn definition() -> WindowKindDefinition {
    ImageWindowKit::window_kind()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn render(snapshot: &SvgSnapshot) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    ImageWindowKit::render(&image_view(snapshot))
}

/// 🖼️ SVG has no pixel buffer — the "image" IS its own XML source, base64-wrapped as an
/// `image/svg+xml` data URI so `ImageWindowKit::render` displays it like any other raster.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn image_view(snapshot: &SvgSnapshot) -> ImageView {
    let xml = write_svg_xml(&snapshot.doc);
    ImageView { width: 300, height: 150, mime: "image/svg+xml".into(), base64: crate::base64_standard(xml.as_bytes()) }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
