//! 🎯️ Edit-mode window option — the selection group: which entity kinds
//! (objects/vortices/attractions) a pick may even reach. 🕹️ ticket
//! 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the marquee method (rectangle/lasso) and
//! default merge mode toggles moved into the framework's `vortex` interaction domain
//! (`interactionSelect`'s `method`/`merge` args, `setSelectionMode`) — no longer app config, and no
//! longer renderable here (`ArtifactApp::window_measures` never gained an `InteractionView`
//! parameter; see `panels::inspection::render`'s doc comment for the same framework-level gap).

use crate::editor::puzzle3d::config::Puzzle3dRuntime;
use crate::editor::puzzle3d::terminology::Puzzle3dLabels;
use crate::editor::puzzle3d::{puzzle3d_action, PUZZLE3D_PLAY_CONTROLLER_ID};
use semio_framework_plugin::WindowMeasure;
use dsl::json;

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
