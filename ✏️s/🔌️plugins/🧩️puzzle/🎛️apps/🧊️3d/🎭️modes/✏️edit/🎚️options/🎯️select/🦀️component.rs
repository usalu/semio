//! 🎯️ Edit-mode window option — the selection group: the marquee method (rectangle/lasso), the
//! default merge mode (selective/additive/subtractive/invertive) and which entity kinds
//! (objects/vortices/attractions) a pick may even reach.

use crate::apps::puzzle3d::config::Puzzle3dRuntime;
use crate::apps::puzzle3d::terminology::Puzzle3dLabels;
use crate::apps::puzzle3d::{puzzle3d_action, PUZZLE3D_PLAY_CONTROLLER_ID};
use semio_framework_plugin::WindowMeasure;
use serde_json::json;

pub fn measure(runtime: &Puzzle3dRuntime, labels: &Puzzle3dLabels) -> WindowMeasure {
    WindowMeasure::Group {
        id: format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-select"),
        label: labels.select.into(),
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
                id: format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-select-rectangle"),
                icon_id: "rectangle-tool".into(),
                label: Some(labels.rectangle.into()),
                pressed: runtime.selection_method == "rectangle",
                text: None,
                on_change: puzzle3d_action("setSelectionMethod", Some(json!({ "method": "rectangle" }))),
            },
            WindowMeasure::Toggle {
                id: format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-select-lasso"),
                icon_id: "lasso".into(),
                label: Some(labels.lasso.into()),
                pressed: runtime.selection_method == "lasso",
                text: None,
                on_change: puzzle3d_action("setSelectionMethod", Some(json!({ "method": "lasso" }))),
            },
            WindowMeasure::Toggle {
                id: format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-select-mode-default"),
                icon_id: "mouse-pointer".into(),
                label: Some(labels.selective.into()),
                pressed: runtime.selection_mode_default == "default",
                text: None,
                on_change: puzzle3d_action("setSelectionModeDefault", Some(json!({ "mode": "default" }))),
            },
            WindowMeasure::Toggle {
                id: format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-select-mode-additive"),
                icon_id: "plus".into(),
                label: Some(labels.additive.into()),
                pressed: runtime.selection_mode_default == "additive",
                text: None,
                on_change: puzzle3d_action("setSelectionModeDefault", Some(json!({ "mode": "additive" }))),
            },
            WindowMeasure::Toggle {
                id: format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-select-mode-subtractive"),
                icon_id: "minus".into(),
                label: Some(labels.subtractive.into()),
                pressed: runtime.selection_mode_default == "subtractive",
                text: None,
                on_change: puzzle3d_action("setSelectionModeDefault", Some(json!({ "mode": "subtractive" }))),
            },
            WindowMeasure::Toggle {
                id: format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-select-mode-invertive"),
                icon_id: "rotate-ccw".into(),
                label: Some(labels.invertive.into()),
                pressed: runtime.selection_mode_default == "invertive",
                text: None,
                on_change: puzzle3d_action("setSelectionModeDefault", Some(json!({ "mode": "invertive" }))),
            },
            WindowMeasure::Toggle {
                id: format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-select-objects"),
                icon_id: "box".into(),
                label: Some(labels.objects.into()),
                pressed: runtime.selectable_kinds.objects,
                text: None,
                on_change: puzzle3d_action("setSelectableKind", Some(json!({ "kind": "objects" }))),
            },
            WindowMeasure::Toggle {
                id: format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-select-vortices"),
                icon_id: "circle-dot".into(),
                label: Some(labels.vortices.into()),
                pressed: runtime.selectable_kinds.vortices,
                text: None,
                on_change: puzzle3d_action("setSelectableKind", Some(json!({ "kind": "vortices" }))),
            },
            WindowMeasure::Toggle {
                id: format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-select-attractions"),
                icon_id: "link".into(),
                label: Some(labels.attractions.into()),
                pressed: runtime.selectable_kinds.attractions,
                text: None,
                on_change: puzzle3d_action("setSelectableKind", Some(json!({ "kind": "attractions" }))),
            },
        ],
    }
}
