//! 👁️ Puzzle 2d viewer — the `view` mode: a single full-pane Board window, the read-only counterpart
//! of the editor's `edit` mode (whose quad selection/overview/detail panes all mutate). Ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §1 only requires "one real view mode
//! with ≥1 real window" for a viewer packet — read-only twins of the editor's selection/detail panes
//! are a follow-up, not a purity or completeness requirement.

use crate::viewer::puzzle2d::modes::view::windows::board;
use semio_framework_plugin::{LocalizedLabel, ModeDefinition, WindowLayout, WindowLayoutRoot, WindowLayoutStackNode, WindowLayoutWindowNode};

pub const PUZZLE2D_VIEW_MODE_VIEW: &str = "view";

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::viewer::puzzle2d::create_puzzle2d_viewer`.
pub async fn definition() -> ModeDefinition {
    ModeDefinition { id: PUZZLE2D_VIEW_MODE_VIEW.into(), label: LocalizedLabel::native("View", "Ansicht"), icon_id: "eye".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

/// 🪟️ Single full-pane Board window — the read-only viewer has no quadrant layout to allocate.
pub async fn layout() -> WindowLayout {
    WindowLayout {
        root: WindowLayoutRoot::Stack(WindowLayoutStackNode {
            kind: "stack".into(),
            size: None,
            active_window_kind_id: None,
            children: vec![WindowLayoutWindowNode { kind: "window".into(), window_kind_id: board::WINDOW_KIND_ID.into(), title: Some("Board".into()), instance_id: None, template_id: None, corner: None }],
        }),
    }
}
//#endregion 🔖️Definition
