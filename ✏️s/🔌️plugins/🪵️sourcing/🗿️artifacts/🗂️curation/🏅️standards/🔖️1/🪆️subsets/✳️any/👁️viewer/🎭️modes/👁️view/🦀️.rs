//! 👁️ Sourcing viewer — the `view` mode: a single read-only pool table window. MUST NOT import
//! anything from the sibling `editor` module (`policyViewerPurityBreaches`).

use crate::viewer::sourcing::modes::view::windows::pool;
use semio_framework_plugin::{LocalizedLabel, ModeDefinition, WindowLayout, WindowLayoutAxisNode, WindowLayoutChild, WindowLayoutRoot, WindowLayoutStackNode, WindowLayoutWindowNode};

pub const SOURCING_VIEW_MODE_VIEW: &str = "view";

//#region 🔖️Definition
pub fn definition() -> ModeDefinition {
    ModeDefinition { id: SOURCING_VIEW_MODE_VIEW.into(), label: LocalizedLabel::native("View", "Ansicht"), icon_id: "eye".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

/// 🪟️ Single full-width stack holding the pool table.
pub fn layout() -> WindowLayout {
    WindowLayout {
        root: WindowLayoutRoot::Axis(WindowLayoutAxisNode {
            kind: "row".into(),
            size: None,
            children: vec![WindowLayoutChild::Stack(WindowLayoutStackNode {
                kind: "stack".into(),
                size: None,
                active_window_kind_id: None,
                children: vec![WindowLayoutWindowNode { kind: "window".into(), window_kind_id: pool::WINDOW_KIND_ID.into(), title: Some("Pool".into()), instance_id: None, template_id: None, corner: None }],
            })],
        }),
    }
}
//#endregion 🔖️Definition

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
