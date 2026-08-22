//! ✏️ `svg` edit (basic) — Main window: real `ImageWindowKit`
//! render of the current document (editable variant).

use crate::artifacts::svg::standards::v1_1::subsets::basic::schema::snapshot::write_svg_xml;
use crate::artifacts::svg::standards::v1_1::subsets::basic::schema::snapshot::SvgSnapshot;
use semio_framework_plugin::app::{ImageView, ImageWindowKit};
use semio_framework_plugin::{BuiltNode, WindowKindDefinition, WindowKit};

pub const WINDOW_KIND_ID: &str = ImageWindowKit::KIND_ID;
pub const BODY_KEY: &str = ImageWindowKit::KIND_ID;

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn definition() -> WindowKindDefinition {
    ImageWindowKit::editable_window_kind()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn render(snapshot: &SvgSnapshot) -> BuiltNode {
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
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn definition_uses_the_frozen_window_kit_kind_id() {
        let def = definition();
        assert_eq!(def.id, WINDOW_KIND_ID);
    }

    #[semio_framework_async_macros::async_test]
    async fn render_produces_a_scene_node_for_the_default_document() {
        let document = SvgSnapshot::default();
        let _node = render(&document);
    }
}
