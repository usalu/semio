//! 🤏️ 3D-window options — how grip markers are surfaced in the world pane: WHEN they are emitted
//! (always, or only for the hovered/selected parts the live `vortex` interaction names) and HOW their
//! direction arrows point (outwards from the grip point, or inwards onto it). Both are per-window
//! chrome, never document state.

use crate::editor::puzzle5d::config::Puzzle5dRuntime;
use crate::editor::puzzle5d::terminology::Puzzle5dLabels;
use crate::editor::puzzle5d::{puzzle5d_action, PUZZLE5D_GRIP_DIRECTION_INWARDS, PUZZLE5D_GRIP_DIRECTION_OUTWARDS, PUZZLE5D_GRIP_SHOW_ALWAYS, PUZZLE5D_GRIP_SHOW_SELECTED, PUZZLE5D_PLAY_CONTROLLER_ID};
use semio_framework_plugin::{MeasureSelectItem, WindowMeasure};

/// 🤏️ `puzzle5d-play-world-grip-show` — Always (every part) or Selected (hovered/selected only).
pub fn show_measure(runtime: &Puzzle5dRuntime, labels: &Puzzle5dLabels) -> WindowMeasure {
    WindowMeasure::Select {
        id: format!("{PUZZLE5D_PLAY_CONTROLLER_ID}-world-grip-show"),
        label: Some(labels.grip_show.into()),
        value: runtime.grip_show.clone(),
        items: vec![
            MeasureSelectItem { id: PUZZLE5D_GRIP_SHOW_ALWAYS.into(), value: PUZZLE5D_GRIP_SHOW_ALWAYS.into(), label: labels.always.into() },
            MeasureSelectItem { id: PUZZLE5D_GRIP_SHOW_SELECTED.into(), value: PUZZLE5D_GRIP_SHOW_SELECTED.into(), label: labels.selected.into() },
        ],
        on_change: puzzle5d_action("setGripShow", None),
    }
}

/// 🧭️ `puzzle5d-play-world-grip-direction` — Outwards (tip away from the point) or Inwards (tip on it).
pub fn direction_measure(runtime: &Puzzle5dRuntime, labels: &Puzzle5dLabels) -> WindowMeasure {
    WindowMeasure::Select {
        id: format!("{PUZZLE5D_PLAY_CONTROLLER_ID}-world-grip-direction"),
        label: Some(labels.grip_direction.into()),
        value: runtime.grip_direction.clone(),
        items: vec![
            MeasureSelectItem { id: PUZZLE5D_GRIP_DIRECTION_OUTWARDS.into(), value: PUZZLE5D_GRIP_DIRECTION_OUTWARDS.into(), label: labels.outwards.into() },
            MeasureSelectItem { id: PUZZLE5D_GRIP_DIRECTION_INWARDS.into(), value: PUZZLE5D_GRIP_DIRECTION_INWARDS.into(), label: labels.inwards.into() },
        ],
        on_change: puzzle5d_action("setGripDirection", None),
    }
}
