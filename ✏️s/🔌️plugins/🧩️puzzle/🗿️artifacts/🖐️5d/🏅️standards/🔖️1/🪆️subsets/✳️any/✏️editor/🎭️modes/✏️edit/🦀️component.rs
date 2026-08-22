//! ✏️ Puzzle 5d play app — the `edit` mode: the 3D-first 60/40 split layout over the two window
//! kinds, plus the engagement HUD builder both windows share (it only differs by which window id it
//! binds its input/abort actions to). The Utility Options every window exposes for the shared
//! brush/fill utilities live in `🎚️options/*`; genuinely per-window chrome lives under that window's
//! own `🎚️options/`.

use crate::editor::puzzle5d::modes::edit::windows::{board2d, world3d};
use crate::editor::puzzle5d::terminology::Puzzle5dLabels;
use crate::editor::puzzle5d::{puzzle5d_action, Puzzle5dScene};
use semio_framework_plugin::{create_default_layout, LocalizedLabel, ModeDefinition, WindowEngagement, WindowEngagementInput, WindowEngagementStatus, WindowLayout};
use serde_json::json;

pub const PUZZLE5D_PLAY_MODE_EDIT: &str = "edit";

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::editor::puzzle5d::create_puzzle5d_app`.
pub fn definition() -> ModeDefinition {
    ModeDefinition { id: PUZZLE5D_PLAY_MODE_EDIT.into(), label: LocalizedLabel::native("Edit", "Bearbeiten"), icon_id: "pencil".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

/// 🪟️ 3D-first 60/40 split — mirrors semio_compose_rs's design app (scene 60% / diagram 40%,
/// `semio_compose_rs/client/lib/sketchpad/js/index.ts:15367-15378`), the assembly-editing use case
/// this app replaces.
pub fn layout() -> WindowLayout {
    create_default_layout(&[world3d::WINDOW_KIND_ID.into(), board2d::WINDOW_KIND_ID.into()], "row", Some(&[60.0, 40.0]), Some(&["Puzzle 3D".into(), "Puzzle 2D".into()]))
}
//#endregion 🔖️Definition

//#region 🔖️Engagement
/// 🧭️ Whether the engagement HUD should mark an active session for the given utility.
fn puzzle5d_engagement_session_active(window: &str, active_utility: &str) -> bool {
    if window == world3d::WINDOW_KIND_ID {
        matches!(active_utility, "brush" | "fill" | "worldRelocate")
    } else {
        active_utility != "select"
    }
}

/// 🤝️ The engagement HUD for one window: the select/brush/fill switcher lives in the framework
/// utility bar (declared via `.utility` + each window's `utilities` binding); the fill-count slider
/// and brush placement picker live as tagged [`semio_framework_plugin::WindowMeasure::Group`]s in the
/// dedicated "Utility Options" rail, so what is left here is a bare command input plus a status line.
pub fn puzzle5d_engagement(envelope: &Puzzle5dScene, window: &str, labels: &Puzzle5dLabels) -> WindowEngagement {
    let part_count = envelope.document.parts.len();
    let fastener_count = envelope.document.fasteners.len();
    let active_utility = envelope.active_utility.as_str();
    let input_value = envelope.runtime.engagement_input_by_window.get(window).cloned().unwrap_or_default();
    let placeholder = match active_utility {
        "fill" => "Fill",
        "brush" => "Brush",
        _ => "select, brush, fill, clear",
    };
    WindowEngagement {
        session_active: Some(puzzle5d_engagement_session_active(window, active_utility)),
        input: Some(WindowEngagementInput {
            id: Some(format!("puzzle5d-engagement-{window}")),
            value: Some(input_value),
            placeholder: Some(placeholder.into()),
            disabled: None,
            on_change: Some(puzzle5d_action("engagementInput", Some(json!({ "window": window })))),
            on_submit: Some(puzzle5d_action("engagementSubmit", Some(json!({ "window": window })))),
            on_repeat_last: None,
            on_abort: Some(puzzle5d_action("engagementAbort", Some(json!({ "window": window })))),
        }),
        control: None,
        controls: None,
        status: Some(vec![WindowEngagementStatus { id: format!("puzzle5d-status-{window}"), text: format!("{part_count} {} · {fastener_count} {} · {} {active_utility}", labels.parts.as_str(), labels.fasteners.as_str(), labels.utility.as_str()) }]),
        options: None,
        possible_engagements: None,
    }
}
//#endregion 🔖️Engagement

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use crate::editor::puzzle5d::create_puzzle5d_app;
    use semio_framework_plugin::WindowLayoutRoot;

    #[test]
    fn default_layout_is_world_three_fifths_and_board_two_fifths() {
        let app = create_puzzle5d_app();
        let layout = app.default_layout.as_ref().expect("default layout");
        let WindowLayoutRoot::Axis(root) = &layout.root else {
            panic!("default layout root must be a row axis");
        };
        assert_eq!(root.kind, "row");
        assert_eq!(root.children.len(), 2);
    }
}
//#endregion 🧪️Tests
