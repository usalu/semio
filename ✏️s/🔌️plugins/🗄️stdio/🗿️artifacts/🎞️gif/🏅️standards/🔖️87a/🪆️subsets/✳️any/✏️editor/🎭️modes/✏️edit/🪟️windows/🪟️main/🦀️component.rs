//! ✏️ `gif` edit (any) — Main window: real `ImageWindowKit`
//! render of the current document (editable variant).

use base64::Engine as _;
use crate::artifacts::gif::standards::v87a::subsets::any::io::encode_gif;
use crate::artifacts::gif::standards::v87a::subsets::any::schema::snapshot::GifSnapshot;
use semio_framework_plugin::app::{ImageView, ImageWindowKit};
use semio_framework_plugin::{UiNode, WindowKindDefinition, WindowKit};

pub const WINDOW_KIND_ID: &str = ImageWindowKit::KIND_ID;
pub const BODY_KEY: &str = ImageWindowKit::KIND_ID;

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn definition() -> WindowKindDefinition {
    ImageWindowKit::editable_window_kind()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn render(snapshot: &GifSnapshot) -> UiNode {
    ImageWindowKit::render(&image_view(snapshot))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn image_view(snapshot: &GifSnapshot) -> ImageView {
    let bytes = encode_gif(snapshot).ok().unwrap_or_default();
    ImageView { width: snapshot.width, height: snapshot.height, mime: "image/gif".into(), base64: base64::engine::general_purpose::STANDARD.encode(bytes) }
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
        let document = GifSnapshot::default();
        let _node = render(&document);
    }
}
