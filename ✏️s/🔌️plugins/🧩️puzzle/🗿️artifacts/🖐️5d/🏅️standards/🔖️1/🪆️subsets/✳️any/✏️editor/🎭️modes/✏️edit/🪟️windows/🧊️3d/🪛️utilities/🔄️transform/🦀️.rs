//! 🔄️ 3D-window utility — the transform gumball, exposed as the three grouped handles
//! `move`/`rotate`/`scale`. One node rather than three, because they are one concept with one
//! `group: "transform"` collapse: the utility bar renders them as a single dropdown, and
//! `puzzle5d_transform_handle` maps whichever is active onto the gumball mode the world engine draws.
//! Bound only by the 3D world window — the 2D board window leads with `select` instead.

use crate::editor::puzzle5d::config::Puzzle5dRuntime;
use crate::editor::puzzle5d::terminology::Puzzle5dLabels;
use crate::editor::puzzle5d::{puzzle5d_action, PUZZLE5D_PLAY_CONTROLLER_ID};
use dsl::json;
use semio_framework_plugin::{LocalizedLabel, UtilityDefinition, WindowMeasure};

pub const MOVE_UTILITY_ID: &str = "move";
pub const ROTATE_UTILITY_ID: &str = "rotate";
pub const SCALE_UTILITY_ID: &str = "scale";
/// 🔗️ The utility-bar group all three handles collapse into.
const TRANSFORM_GROUP: &str = "transform";

/// 🧱️ Stitched into the app manifest by `crate::editor::puzzle5d::create_puzzle5d_app`.
pub fn move_definition() -> UtilityDefinition {
    UtilityDefinition { group: Some(TRANSFORM_GROUP.into()), ..UtilityDefinition::new(MOVE_UTILITY_ID, LocalizedLabel::native("Move", "Verschieben"), "move") }
}

/// 🧱️ Stitched into the app manifest by `crate::editor::puzzle5d::create_puzzle5d_app`.
pub fn rotate_definition() -> UtilityDefinition {
    UtilityDefinition { group: Some(TRANSFORM_GROUP.into()), ..UtilityDefinition::new(ROTATE_UTILITY_ID, LocalizedLabel::native("Rotate", "Drehen"), "rotate-cw") }
}

/// 🧱️ Stitched into the app manifest by `crate::editor::puzzle5d::create_puzzle5d_app`.
pub fn scale_definition() -> UtilityDefinition {
    UtilityDefinition { group: Some(TRANSFORM_GROUP.into()), ..UtilityDefinition::new(SCALE_UTILITY_ID, LocalizedLabel::native("Scale", "Skalieren"), "maximize-2") }
}

/// 🎛️ Utility Options for the gumball — the Move and Rotate handle flags
/// (`setTransformGumballFlag`) that compose which handles it draws; with both off
/// `puzzle5d_gumball_active` refuses to render a gumball nobody could grab. One group per handle
/// utility, because whichever of `move`/`rotate`/`scale` is active is the one whose options row the
/// shell unwraps — the flags gate the same two families in all three.
pub fn options(runtime: &Puzzle5dRuntime, labels: &Puzzle5dLabels) -> Vec<WindowMeasure> {
    [MOVE_UTILITY_ID, ROTATE_UTILITY_ID, SCALE_UTILITY_ID].into_iter().map(|utility_id| flags_group(runtime, labels, utility_id)).collect()
}

fn flags_group(runtime: &Puzzle5dRuntime, labels: &Puzzle5dLabels, utility_id: &str) -> WindowMeasure {
    let prefix = format!("{PUZZLE5D_PLAY_CONTROLLER_ID}-utility-options-{utility_id}");
    WindowMeasure::Group {
        id: prefix.clone(),
        label: String::new(),
        default_open: Some(true),
        active_utility_id: Some(utility_id.into()),
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
                id: format!("{prefix}-move"),
                icon_id: "move-3d".into(),
                label: Some(labels.move_handle.into()),
                pressed: runtime.transform_move,
                text: None,
                on_change: puzzle5d_action("setTransformGumballFlag", Some(json!({ "flag": "move" }))),
            },
            WindowMeasure::Toggle {
                id: format!("{prefix}-rotate"),
                icon_id: "rotate-cw".into(),
                label: Some(labels.rotate_handle.into()),
                pressed: runtime.transform_rotate,
                text: None,
                on_change: puzzle5d_action("setTransformGumballFlag", Some(json!({ "flag": "rotate" }))),
            },
        ],
    }
}
