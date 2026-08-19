//! ✏️ `bmp` edit (any) — Main window: real `ImageWindowKit`
//! render of the current document (editable variant).

use base64::Engine as _;
use crate::artifacts::bmp::standards::v_v3::subsets::any::io::encode_bmp;
use crate::artifacts::bmp::standards::v_v3::subsets::any::schema::snapshot::BmpSnapshot;
use semio_framework_plugin::app::{ImageView, ImageWindowKit};
use semio_framework_plugin::{UiNode, WindowKindDefinition, WindowKit};

pub const WINDOW_KIND_ID: &str = ImageWindowKit::KIND_ID;
pub const BODY_KEY: &str = ImageWindowKit::KIND_ID;

pub async fn definition() -> WindowKindDefinition {
    ImageWindowKit::editable_window_kind()
}

pub async fn render(snapshot: &BmpSnapshot) -> UiNode {
    ImageWindowKit::render(&image_view(snapshot))
}

async fn image_view(snapshot: &BmpSnapshot) -> ImageView {
    let bytes = encode_bmp(snapshot).ok().unwrap_or_default();
    ImageView { width: snapshot.width, height: snapshot.height, mime: "image/bmp".into(), base64: base64::engine::general_purpose::STANDARD.encode(bytes) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    async fn definition_uses_the_frozen_window_kit_kind_id() {
        let def = definition();
        assert_eq!(def.id, WINDOW_KIND_ID);
    }

    #[test]
    async fn render_produces_a_scene_node_for_the_default_document() {
        let document = BmpSnapshot::default();
        let _node = render(&document);
    }
}
