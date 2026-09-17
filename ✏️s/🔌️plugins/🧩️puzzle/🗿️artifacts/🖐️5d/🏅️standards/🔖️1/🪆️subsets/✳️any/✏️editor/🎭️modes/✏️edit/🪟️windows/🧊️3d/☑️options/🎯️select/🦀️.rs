//! 🎯️ 3D-window option — the selection group: which entity kinds (parts/grips/fasteners) a pick in
//! the world pane may even reach. The marquee method and merge mode live in the framework's own
//! `vortex` interaction domain (`interactionSelect`'s `method`/`merge`), never here.
//!
//! 🔁️ Both panes render the identical three toggles over their own per-window `selectable_kinds`
//! record, so the body is declared once here as [`selectable_kind_group`] and the board pane's
//! `◻️2d/☑️options/🎯️select` calls it with its own group id.

use crate::editor::puzzle5d::config::Puzzle5dRuntime;
use crate::editor::puzzle5d::terminology::Puzzle5dLabels;
use crate::editor::puzzle5d::{puzzle5d_action, PUZZLE5D_PLAY_CONTROLLER_ID};
use dsl::json;
use semio_framework_plugin::WindowMeasure;

/// 🎯️ `puzzle5d-play-world-select` — parts / grips / fasteners for the world pane.
pub fn measure(runtime: &Puzzle5dRuntime, labels: &Puzzle5dLabels) -> WindowMeasure {
    selectable_kind_group(runtime, labels, &format!("{PUZZLE5D_PLAY_CONTROLLER_ID}-world-select"))
}

/// 🎯️ The three-toggle body both panes' selection groups render — one `setSelectableKind` verb, one
/// per-window `selectable_kinds` record, two group ids.
pub fn selectable_kind_group(runtime: &Puzzle5dRuntime, labels: &Puzzle5dLabels, group_id: &str) -> WindowMeasure {
    WindowMeasure::Group {
        id: group_id.to_string(),
        label: labels.selection.into(),
        default_open: Some(true),
        active_utility_id: None,
        value: None,
        min: None,
        max: None,
        step: None,
        ready: None,
        loading: None,
        waiting: None,
        on_change: None,
        children: vec![
            WindowMeasure::Toggle {
                id: format!("{group_id}-parts"),
                icon_id: "box".into(),
                label: Some(labels.parts.into()),
                pressed: runtime.selectable_kinds.parts,
                text: None,
                on_change: puzzle5d_action("setSelectableKind", Some(json!({ "kind": "parts" }))),
            },
            WindowMeasure::Toggle {
                id: format!("{group_id}-grips"),
                icon_id: "circle-dot".into(),
                label: Some(labels.grips.into()),
                pressed: runtime.selectable_kinds.grips,
                text: None,
                on_change: puzzle5d_action("setSelectableKind", Some(json!({ "kind": "grips" }))),
            },
            WindowMeasure::Toggle {
                id: format!("{group_id}-fasteners"),
                icon_id: "link".into(),
                label: Some(labels.fasteners.into()),
                pressed: runtime.selectable_kinds.fasteners,
                text: None,
                on_change: puzzle5d_action("setSelectableKind", Some(json!({ "kind": "fasteners" }))),
            },
        ],
    }
}
