//! 👁️ Puzzle 3d viewer — the `view` mode: a single full-pane Mesh window, the read-only counterpart
//! of the editor's single `🧊️main` split-pane window. Ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §1 only requires one real window per
//! viewer packet.

use crate::viewer::puzzle3d::modes::view::windows::main;
use semio_framework_plugin::{LocalizedLabel, ModeDefinition, WindowLayout, WindowLayoutRoot, WindowLayoutStackNode, WindowLayoutWindowNode};

pub const PUZZLE3D_VIEW_MODE_VIEW: &str = "view";

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::viewer::puzzle3d::create_puzzle3d_viewer`.
pub fn definition() -> ModeDefinition {
    ModeDefinition { id: PUZZLE3D_VIEW_MODE_VIEW.into(), label: LocalizedLabel::native("View", "Ansicht"), icon_id: "eye".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

/// 🪟️ Single full-pane Mesh window — the read-only viewer has no split top/perspective layout to allocate.
pub fn layout() -> WindowLayout {
    WindowLayout {
        root: WindowLayoutRoot::Stack(WindowLayoutStackNode {
            kind: "stack".into(),
            size: None,
            active_window_kind_id: None,
            children: vec![WindowLayoutWindowNode { kind: "window".into(), window_kind_id: main::WINDOW_KIND_ID.into(), title: Some("Puzzle 3D".into()), instance_id: None, template_id: None }],
        }),
    }
}
//#endregion 🔖️Definition
