//! 👁️ `wav` view (any) — Main window: real `MediaWindowKit`
//! render of the current document (read-only).

use crate::artifacts::wav::standards::riff_pcm::subsets::any::schema::snapshot::WavSnapshot;
use semio_framework_plugin::app::{MediaKind, MediaView, MediaWindowKit};
use semio_framework_plugin::{BuiltNode, WindowKindDefinition, WindowKit};

pub const WINDOW_KIND_ID: &str = MediaWindowKit::KIND_ID;
pub const BODY_KEY: &str = MediaWindowKit::KIND_ID;

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn definition() -> WindowKindDefinition {
    MediaWindowKit::window_kind()
}

/// 🎬️ Duration/position stay at the kit's zero defaults — this format's decoded snapshot does not
/// model a playable transport position yet (thin v1: the kit's own transport chrome is real, the
/// per-document duration/position feed is a documented follow-up, not invented here).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn render(_snapshot: &WavSnapshot) -> BuiltNode {
    MediaWindowKit::render(&MediaView { duration_ms: 0, position_ms: 0, kind: MediaKind::Audio })
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
        let document = WavSnapshot::default();
        let _node = render(&document);
    }
}
