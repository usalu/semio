//! ✏️ `avi` edit (any) — Main window: real `MediaWindowKit`
//! render of the current document (editable variant).

use crate::artifacts::avi::standards::v1_0::subsets::any::schema::snapshot::AviSnapshot;
use semio_framework_plugin::app::{MediaKind, MediaView, MediaWindowKit};
use semio_framework_plugin::{UiNode, WindowKindDefinition, WindowKit};

pub const WINDOW_KIND_ID: &str = MediaWindowKit::KIND_ID;
pub const BODY_KEY: &str = MediaWindowKit::KIND_ID;

pub async fn definition() -> WindowKindDefinition {
    MediaWindowKit::editable_window_kind().await
}

/// 🎬️ Duration/position stay at the kit's zero defaults — this format's decoded snapshot does not
/// model a playable transport position yet (thin v1: the kit's own transport chrome is real, the
/// per-document duration/position feed is a documented follow-up, not invented here).
pub async fn render(_snapshot: &AviSnapshot) -> UiNode {
    MediaWindowKit::render(&MediaView { duration_ms: 0, position_ms: 0, kind: MediaKind::Video }).await
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
        let document = AviSnapshot::default();
        let _node = render(&document);
    }
}
