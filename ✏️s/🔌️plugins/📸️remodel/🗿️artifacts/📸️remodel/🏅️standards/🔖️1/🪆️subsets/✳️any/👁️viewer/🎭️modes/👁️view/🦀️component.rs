//! 👁️ Remodel viewer — the `view` mode: a single full-pane Model window, the read-only counterpart
//! of the editor's `model`/`capture`/`analyze` triad. Ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §1 only requires "at least one real
//! window" for a viewer packet — read-only Frames/Report windows are a follow-up, not a purity or
//! completeness requirement.

use crate::viewer::remodel::modes::view::windows::model;
use semio_framework_plugin::{LocalizedLabel, ModeDefinition, WindowLayout, WindowLayoutRoot, WindowLayoutStackNode, WindowLayoutWindowNode};

pub const REMODEL_VIEW_MODE_VIEW: &str = "view";

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::viewer::remodel::create_remodel_viewer`.
pub fn definition() -> ModeDefinition {
    ModeDefinition { id: REMODEL_VIEW_MODE_VIEW.into(), label: LocalizedLabel::native("View", "Ansicht"), icon_id: "eye".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

/// 🪟️ Single full-pane Model window — the read-only viewer has no capture/analyze layout to allocate.
pub fn layout() -> WindowLayout {
    WindowLayout {
        root: WindowLayoutRoot::Stack(WindowLayoutStackNode {
            kind: "stack".into(),
            size: None,
            active_window_kind_id: None,
            children: vec![WindowLayoutWindowNode { kind: "window".into(), window_kind_id: model::WINDOW_KIND_ID.into(), title: Some("Model".into()), instance_id: None, template_id: None, corner: None }],
        }),
    }
}
//#endregion 🔖️Definition
