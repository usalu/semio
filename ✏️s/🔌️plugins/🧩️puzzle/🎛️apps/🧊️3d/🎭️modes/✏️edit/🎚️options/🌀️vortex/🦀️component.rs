//! 🌀️ Edit-mode window options — how vortex markers are surfaced in the viewport: WHEN they are
//! emitted (always, or only for hovered/selected objects) and HOW their direction arrows point
//! (outwards from the vortex point, or inwards onto it). Both are per-window-instance chrome.

use crate::apps::puzzle3d::config::Puzzle3dRuntime;
use crate::apps::puzzle3d::terminology::Puzzle3dLabels;
use crate::apps::puzzle3d::{puzzle3d_action, PUZZLE3D_PLAY_CONTROLLER_ID, PUZZLE3D_VORTEX_DIRECTION_INWARDS, PUZZLE3D_VORTEX_DIRECTION_OUTWARDS, PUZZLE3D_VORTEX_SHOW_ALWAYS, PUZZLE3D_VORTEX_SHOW_SELECTED};
use semio_framework_plugin::{MeasureSelectItem, WindowMeasure};

/// 🌀️ Window option for when vortex markers are emitted — Always (every object) or Selected (hovered/selected only).
pub fn show_measure(runtime: &Puzzle3dRuntime, labels: &Puzzle3dLabels) -> WindowMeasure {
    WindowMeasure::Select {
        id: format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-vortex-show"),
        label: Some(labels.vortex_show.into()),
        value: runtime.vortex_show.clone(),
        items: vec![
            MeasureSelectItem { id: PUZZLE3D_VORTEX_SHOW_ALWAYS.into(), value: PUZZLE3D_VORTEX_SHOW_ALWAYS.into(), label: labels.always.into() },
            MeasureSelectItem { id: PUZZLE3D_VORTEX_SHOW_SELECTED.into(), value: PUZZLE3D_VORTEX_SHOW_SELECTED.into(), label: labels.selected.into() },
        ],
        on_change: puzzle3d_action("setVortexShow", None),
    }
}

/// 🧭️ Window option for how vortex direction arrows are drawn — Outwards (tip away from point) or Inwards (tip on point).
pub fn direction_measure(runtime: &Puzzle3dRuntime, labels: &Puzzle3dLabels) -> WindowMeasure {
    WindowMeasure::Select {
        id: format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-vortex-direction"),
        label: Some(labels.vortex_direction.into()),
        value: runtime.vortex_direction.clone(),
        items: vec![
            MeasureSelectItem { id: PUZZLE3D_VORTEX_DIRECTION_OUTWARDS.into(), value: PUZZLE3D_VORTEX_DIRECTION_OUTWARDS.into(), label: labels.outwards.into() },
            MeasureSelectItem { id: PUZZLE3D_VORTEX_DIRECTION_INWARDS.into(), value: PUZZLE3D_VORTEX_DIRECTION_INWARDS.into(), label: labels.inwards.into() },
        ],
        on_change: puzzle3d_action("setVortexDirection", None),
    }
}
