//! 👁️ `avi` view (any) — Main window: real `MediaWindowKit`
//! render of the current document (read-only).

use crate::standards::v1_0::subsets::any::schema::snapshot::AviSnapshot;
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
pub fn render(_snapshot: &AviSnapshot) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    MediaWindowKit::render(&MediaView { duration_ms: 0, position_ms: 0, kind: MediaKind::Video })
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
