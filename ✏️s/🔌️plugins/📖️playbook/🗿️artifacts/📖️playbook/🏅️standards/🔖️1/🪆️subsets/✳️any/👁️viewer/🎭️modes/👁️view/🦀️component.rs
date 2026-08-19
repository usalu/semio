//! 👁️ Playbook viewer — the `view` mode: a single full-pane read-only Steps tree, the read-only
//! counterpart of the editor's `builder` mode.

use crate::viewer::playbook::modes::view::windows::steps;
use semio_framework_plugin::{LocalizedLabel, ModeDefinition, WindowLayout, WindowLayoutRoot, WindowLayoutStackNode, WindowLayoutWindowNode};

pub const PLAYBOOK_VIEW_MODE_VIEW: &str = "view";

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::viewer::playbook::create_playbook_viewer`.
pub async fn definition() -> ModeDefinition {
    ModeDefinition { id: PLAYBOOK_VIEW_MODE_VIEW.into(), label: LocalizedLabel::native("View", "Ansicht"), icon_id: "eye".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

/// 🪟️ Single full-pane Steps window — the read-only viewer has no quadrant layout to allocate.
pub async fn layout() -> WindowLayout {
    WindowLayout {
        root: WindowLayoutRoot::Stack(WindowLayoutStackNode {
            kind: "stack".into(),
            size: None,
            active_window_kind_id: None,
            children: vec![WindowLayoutWindowNode { kind: "window".into(), window_kind_id: steps::PLAYBOOK_VIEW_WINDOW_STEPS.into(), title: Some("Steps".into()), instance_id: None, template_id: None, corner: None }],
        }),
    }
}
//#endregion 🔖️Definition
