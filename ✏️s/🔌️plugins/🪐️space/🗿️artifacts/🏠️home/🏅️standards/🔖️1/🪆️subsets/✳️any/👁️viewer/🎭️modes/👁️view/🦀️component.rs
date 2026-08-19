//! 👁️ S Home viewer — the `view` mode: a single full-pane read-only studio-catalog window, the
//! read-only counterpart of the editor's `explore` mode. Ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §1 only requires a viewer packet to
//! carry at least one real window.

use crate::viewer::home::modes::view::windows::main;
use semio_framework_plugin::{LocalizedLabel, ModeDefinition, WindowLayout, WindowLayoutRoot, WindowLayoutStackNode, WindowLayoutWindowNode};

pub const S_HOME_VIEW_MODE: &str = "view";

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::viewer::home::create_home_viewer`.
pub async fn definition() -> ModeDefinition {
    ModeDefinition { id: S_HOME_VIEW_MODE.into(), label: LocalizedLabel::native("View", "Ansicht"), icon_id: "eye".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

/// 🪟️ Single full-pane VFS window — the read-only viewer has no split layout to allocate.
pub async fn layout() -> WindowLayout {
    WindowLayout {
        root: WindowLayoutRoot::Stack(WindowLayoutStackNode {
            kind: "stack".into(),
            size: None,
            active_window_kind_id: None,
            children: vec![WindowLayoutWindowNode { kind: "window".into(), window_kind_id: main::S_HOME_VIEW_WINDOW.into(), title: Some("Studios".into()), instance_id: None, template_id: None, corner: None }],
        }),
    }
}
//#endregion 🔖️Definition

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    async fn the_view_layout_lists_the_main_window() {
        let json = serde_json::to_string(&layout()).expect("layout json");
        assert!(json.contains(main::S_HOME_VIEW_WINDOW), "layout must reference the main window kind: {json}");
    }
}
//#endregion 🧪️Tests
