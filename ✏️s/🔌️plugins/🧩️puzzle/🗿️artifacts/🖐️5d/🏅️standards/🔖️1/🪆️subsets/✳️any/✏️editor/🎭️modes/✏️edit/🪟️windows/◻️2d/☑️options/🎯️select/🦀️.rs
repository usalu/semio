//! 🎯️ 2D-window option — the board pane's selection group: which entity kinds (parts/grips/fasteners)
//! a pick may even reach. Renders the shared three-toggle body declared once under the world pane's
//! `🧊️3d/☑️options/🎯️select` over THIS pane's own `selectable_kinds` record.

use crate::editor::puzzle5d::config::Puzzle5dRuntime;
use crate::editor::puzzle5d::modes::edit::windows::world3d::options::select as world_select;
use crate::editor::puzzle5d::terminology::Puzzle5dLabels;
use crate::editor::puzzle5d::PUZZLE5D_PLAY_CONTROLLER_ID;
use semio_framework_plugin::WindowMeasure;

/// 🎯️ `puzzle5d-play-board-select` — parts / grips / fasteners for the board pane.
pub fn measure(runtime: &Puzzle5dRuntime, labels: &Puzzle5dLabels) -> WindowMeasure {
    world_select::selectable_kind_group(runtime, labels, &format!("{PUZZLE5D_PLAY_CONTROLLER_ID}-board-select"))
}
