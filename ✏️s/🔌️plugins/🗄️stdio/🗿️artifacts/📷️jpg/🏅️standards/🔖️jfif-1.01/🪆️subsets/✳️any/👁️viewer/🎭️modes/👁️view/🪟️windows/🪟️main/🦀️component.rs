//! 👁️ `jpg` view (any) — Main window: real `ImageWindowKit`
//! render of the current document (read-only).

use base64::Engine as _;
use crate::artifacts::jpg::standards::v_jfif_1_01::subsets::any::io::encode_jpg;
use crate::artifacts::jpg::standards::v_jfif_1_01::subsets::any::schema::snapshot::JpgSnapshot;
use semio_framework_plugin::app::{ImageView, ImageWindowKit};
use semio_framework_plugin::{UiNode, WindowKindDefinition, WindowKit};

pub const WINDOW_KIND_ID: &str = ImageWindowKit::KIND_ID;
pub const BODY_KEY: &str = ImageWindowKit::KIND_ID;

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn definition() -> WindowKindDefinition {
    ImageWindowKit::window_kind()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn render(snapshot: &JpgSnapshot) -> UiNode {
    ImageWindowKit::render(&image_view(snapshot))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn image_view(snapshot: &JpgSnapshot) -> ImageView {
    let bytes = encode_jpg(snapshot).ok().unwrap_or_default();
    ImageView { width: snapshot.width, height: snapshot.height, mime: "image/jpeg".into(), base64: base64::engine::general_purpose::STANDARD.encode(bytes) }
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
        let document = JpgSnapshot::default();
        let _node = render(&document);
    }
}
