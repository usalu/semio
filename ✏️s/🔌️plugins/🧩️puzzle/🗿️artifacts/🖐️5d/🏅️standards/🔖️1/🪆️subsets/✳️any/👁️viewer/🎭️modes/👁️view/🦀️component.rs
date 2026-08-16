//! 👁️ Puzzle 5D viewer — the `view` mode: a single full-pane World3d window, the read-only
//! counterpart of the editor's paired-window `edit` mode. Ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §1 only requires "at least one real
//! window" for a viewer packet — a read-only Board2d twin is a follow-up, not a purity or
//! completeness requirement.

use crate::viewer::puzzle5d::modes::view::windows::world3d;
use semio_framework_plugin::{LocalizedLabel, ModeDefinition, WindowLayout, WindowLayoutRoot, WindowLayoutStackNode, WindowLayoutWindowNode};

pub const PUZZLE5D_VIEW_MODE_VIEW: &str = "view";

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::viewer::puzzle5d::create_puzzle5d_viewer`.
pub fn definition() -> ModeDefinition {
    ModeDefinition { id: PUZZLE5D_VIEW_MODE_VIEW.into(), label: LocalizedLabel::native("View", "Ansicht"), icon_id: "eye".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

/// 🪟️ Single full-pane World3d window — the read-only viewer has no quadrant layout to allocate.
pub fn layout() -> WindowLayout {
    WindowLayout {
        root: WindowLayoutRoot::Stack(WindowLayoutStackNode {
            kind: "stack".into(),
            size: None,
            active_window_kind_id: None,
            children: vec![WindowLayoutWindowNode { kind: "window".into(), window_kind_id: world3d::WINDOW_KIND_ID.into(), title: Some("3D".into()), instance_id: None, template_id: None }],
        }),
    }
}
//#endregion 🔖️Definition
