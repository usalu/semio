//! 🎯️ Edit-mode window option — the selection group: which `vortex` granularity (node / handle /
//! edge) a pick may even reach. Per window instance (`Puzzle2dWindowConfig`), mirroring puzzle3d's
//! `☑️options/🎯️select` over objects/vortices/attractions. 🕹️ The marquee method and the default merge
//! mode are framework-owned (`interactionSelect`'s `method`/`merge` args) and deliberately absent here.

use crate::editor::puzzle2d::config::Puzzle2dPlayRuntime;
use crate::editor::puzzle2d::terminology::Puzzle2dLabels;
use crate::editor::puzzle2d::{puzzle2d_action, PUZZLE2D_GRANULARITY_EDGE, PUZZLE2D_GRANULARITY_HANDLE, PUZZLE2D_GRANULARITY_NODE, PUZZLE2D_PLAY_CONTROLLER_ID};
use semio_framework_plugin::WindowMeasure;
use serde_json::json;

pub fn measure(runtime: &Puzzle2dPlayRuntime, labels: &Puzzle2dLabels) -> WindowMeasure {
    let kinds = runtime.selectable_kinds;
    let toggle = |suffix: &str, icon: &str, label: semio_framework_plugin::LabelText, granularity: &str, pressed: bool| WindowMeasure::Toggle {
        id: format!("{PUZZLE2D_PLAY_CONTROLLER_ID}-select-{suffix}"),
        icon_id: icon.into(),
        label: Some(label.into()),
        pressed,
        text: None,
        on_change: puzzle2d_action("setSelectableKind", Some(json!({ "kind": granularity }))),
    };
    WindowMeasure::Group {
        id: format!("{PUZZLE2D_PLAY_CONTROLLER_ID}-select"),
        label: labels.select.into(),
        default_open: Some(false),
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
            toggle("nodes", "circle-dot", labels.nodes, PUZZLE2D_GRANULARITY_NODE, kinds.nodes),
            toggle("handles", "target", labels.handles, PUZZLE2D_GRANULARITY_HANDLE, kinds.handles),
            toggle("edges", "link", labels.edges, PUZZLE2D_GRANULARITY_EDGE, kinds.edges),
        ],
    }
}
