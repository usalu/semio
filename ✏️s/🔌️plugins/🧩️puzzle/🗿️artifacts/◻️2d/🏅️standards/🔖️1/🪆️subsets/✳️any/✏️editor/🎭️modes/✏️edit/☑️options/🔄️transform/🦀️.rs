//! 🔄️ Edit-mode window option — the Select utility's transform gumball flags: Move (the board's
//! native node drag) and Rotate (the ring around the selection centroid). A scale handle is
//! deliberately absent, the same product decision puzzle3d's gumball states: a node's size comes
//! from its kind catalog, never from a free drag.

use crate::editor::puzzle2d::config::Puzzle2dPlayRuntime;
use crate::editor::puzzle2d::modes::edit::windows::overview::utilities::select as select_utility;
use crate::editor::puzzle2d::terminology::Puzzle2dLabels;
use crate::editor::puzzle2d::{puzzle2d_action, PUZZLE2D_PLAY_CONTROLLER_ID};
use semio_framework_plugin::WindowMeasure;
use serde_json::json;

/// 🎛️ Tagged with the select utility's id as a routing envelope only; `partition_window_measures`
/// unwraps the children so both toggles render flat under the Select toggle that already owns the row.
pub fn measure(runtime: &Puzzle2dPlayRuntime, labels: &Puzzle2dLabels) -> WindowMeasure {
    WindowMeasure::Group {
        id: format!("{PUZZLE2D_PLAY_CONTROLLER_ID}-utility-options-transform"),
        label: String::new(),
        default_open: Some(true),
        active_utility_id: Some(select_utility::UTILITY_ID.into()),
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
                id: format!("{PUZZLE2D_PLAY_CONTROLLER_ID}-transform-move"),
                icon_id: "move".into(),
                label: Some(labels.move_flag.into()),
                pressed: runtime.transform_move,
                text: None,
                on_change: puzzle2d_action("setTransformGumballFlag", Some(json!({ "flag": "move" }))),
            },
            WindowMeasure::Toggle {
                id: format!("{PUZZLE2D_PLAY_CONTROLLER_ID}-transform-rotate"),
                icon_id: "rotate-cw".into(),
                label: Some(labels.rotate_flag.into()),
                pressed: runtime.transform_rotate,
                text: None,
                on_change: puzzle2d_action("setTransformGumballFlag", Some(json!({ "flag": "rotate" }))),
            },
        ],
    }
}
