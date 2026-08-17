//! ✏️ `png` edit (any) — Main window: real `ImageWindowKit`
//! render of the current document (editable variant).

use base64::Engine as _;
use crate::artifacts::png::standards::v1_2::subsets::any::io::encode_png;
use crate::artifacts::png::standards::v1_2::subsets::any::schema::snapshot::PngSnapshot;
use semio_framework_plugin::app::{ImageView, ImageWindowKit};
use semio_framework_plugin::{UiNode, WindowKindDefinition, WindowKit};

pub const WINDOW_KIND_ID: &str = ImageWindowKit::KIND_ID;
pub const BODY_KEY: &str = ImageWindowKit::KIND_ID;

pub fn definition() -> WindowKindDefinition {
    ImageWindowKit::editable_window_kind()
}

pub fn render(snapshot: &PngSnapshot) -> UiNode {
    ImageWindowKit::render(&image_view(snapshot))
}

fn image_view(snapshot: &PngSnapshot) -> ImageView {
    let bytes = encode_png(snapshot).ok().unwrap_or_default();
    ImageView { width: snapshot.width, height: snapshot.height, mime: "image/png".into(), base64: base64::engine::general_purpose::STANDARD.encode(bytes) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn definition_uses_the_frozen_window_kit_kind_id() {
        let def = definition();
        assert_eq!(def.id, WINDOW_KIND_ID);
    }

    #[test]
    fn render_produces_a_scene_node_for_the_default_document() {
        let document = PngSnapshot::default();
        let _node = render(&document);
    }
}
