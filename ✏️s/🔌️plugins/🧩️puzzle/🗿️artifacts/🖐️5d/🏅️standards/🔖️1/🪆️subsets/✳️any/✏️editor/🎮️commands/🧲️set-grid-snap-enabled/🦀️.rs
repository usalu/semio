//! 🧲️ `set-grid-snap-enabled` command.

use crate::editor::puzzle5d::Puzzle5dActionCtx;
use dsl::os_pack::json::Value;

/// 🧲️ A `WindowMeasure::Toggle` dispatches its next state as `pressed`; an argument-less invocation
/// (keybinding, context menu) flips the current state. Reading an absent argument as `false` — what
/// this arm used to do — turned every toggle press into an unconditional "off".
pub fn set_grid_snap_enabled(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    ctx.scene.runtime.grid_snap_enabled = args.and_then(|value| value.get("pressed")).and_then(|value| value.as_bool()).unwrap_or(!ctx.scene.runtime.grid_snap_enabled);
}
