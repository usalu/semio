//! 🔄️ 3D-window utility — Transform: the world gumball, puzzle 3d's `🪛️utilities/🔄️transform` twin. Its
//! Utility Options are the Move/Rotate flags that compose which handles the gumball draws (scale handles
//! are deliberately absent — a part's scale comes from its kind catalog, not from a free drag). Bound
//! only by the 3D world window — the 2D board window drags parts natively.

use crate::editor::puzzle5d::config::Puzzle5dRuntime;
use crate::editor::puzzle5d::terminology::Puzzle5dLabels;
use crate::editor::puzzle5d::{puzzle5d_action, PUZZLE5D_PLAY_CONTROLLER_ID};
use dsl::json;
use semio_framework_plugin::{LocalizedLabel, UtilityDefinition, WindowMeasure};

pub const UTILITY_ID: &str = "transform";

/// 🧱️ Stitched into the app manifest by `crate::editor::puzzle5d::create_puzzle5d_app`.
pub fn definition() -> UtilityDefinition {
    UtilityDefinition::new(UTILITY_ID, LocalizedLabel::native("Transform", "Transformieren"), "transform-3d")
}

/// 🎛️ Utility Options for the Transform utility — the Move and Rotate flags (`setTransformGumballFlag`);
/// with both off `puzzle5d_gumball_active` refuses to render a gumball nobody could grab. Tagged with this
/// utility's id as a routing envelope only; `partition_window_measures` unwraps the children so they render
/// flat under the Transform toggle (the toggle already owns that row, hence the empty group label).
pub fn options(runtime: &Puzzle5dRuntime, labels: &Puzzle5dLabels) -> WindowMeasure {
    WindowMeasure::Group {
        id: format!("{PUZZLE5D_PLAY_CONTROLLER_ID}-utility-options-{UTILITY_ID}"),
        label: String::new(),
        default_open: Some(true),
        active_utility_id: Some(UTILITY_ID.into()),
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
                id: "puzzle5d-transform-move".into(),
                icon_id: "move-3d".into(),
                label: Some(labels.move_handle.into()),
                pressed: runtime.transform_move,
                text: None,
                on_change: puzzle5d_action("setTransformGumballFlag", Some(json!({ "flag": "move" }))),
            },
            WindowMeasure::Toggle {
                id: "puzzle5d-transform-rotate".into(),
                icon_id: "rotate-cw".into(),
                label: Some(labels.rotate_handle.into()),
                pressed: runtime.transform_rotate,
                text: None,
                on_change: puzzle5d_action("setTransformGumballFlag", Some(json!({ "flag": "rotate" }))),
            },
        ],
    }
}
