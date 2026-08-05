//! 🔄️ Main-window utility — Transform: the world gumball. Its Utility Options are the Move/Rotate
//! flags that compose which handles the gumball draws (scale handles are deliberately absent — a
//! puzzle-3d object's scale comes from its kind catalog, not from a free drag).

use crate::apps::puzzle3d::config::Puzzle3dRuntime;
use crate::apps::puzzle3d::terminology::Puzzle3dLabels;
use crate::apps::puzzle3d::{puzzle3d_action, PUZZLE3D_PLAY_CONTROLLER_ID};
use semio_framework_plugin::{LocalizedLabel, UtilityDefinition, WindowMeasure};
use serde_json::json;

pub const UTILITY_ID: &str = "transform";

/// 🧱️ Stitched into the app manifest by `crate::apps::puzzle3d::create_puzzle3d_app`.
pub fn definition() -> UtilityDefinition {
    UtilityDefinition::new(UTILITY_ID, LocalizedLabel::native("Transform", "Transformieren"), "transform-3d")
}

/// 🎛️ Utility Options for the Transform utility — Move and Rotate flags. Tagged with this utility's
/// id as a routing envelope only; `partition_window_measures` unwraps the children so they render
/// flat under the Transform toggle (the toggle already owns that row, hence the empty group label).
pub fn options(runtime: &Puzzle3dRuntime, labels: &Puzzle3dLabels) -> WindowMeasure {
    WindowMeasure::Group {
        id: format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-utility-options-transform"),
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
                id: "puzzle3d-transform-move".into(),
                icon_id: "move-3d".into(),
                label: Some(labels.move_flag.into()),
                pressed: runtime.transform_move,
                text: None,
                on_change: puzzle3d_action("setTransformGumballFlag", Some(json!({ "flag": "move" }))),
            },
            WindowMeasure::Toggle {
                id: "puzzle3d-transform-rotate".into(),
                icon_id: "rotate-cw".into(),
                label: Some(labels.rotate_flag.into()),
                pressed: runtime.transform_rotate,
                text: None,
                on_change: puzzle3d_action("setTransformGumballFlag", Some(json!({ "flag": "rotate" }))),
            },
        ],
    }
}
