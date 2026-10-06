//! 🪦️ `delete-target-region` command.

use crate::editor::puzzle2d::{delete_target_regions_from_snapshot, Puzzle2dActionCtx};
use serde_json::Value;

/// 🪦️ Removes the one addressed target region. A region owns no dependents, so nothing cascades.
pub fn delete_target_region(ctx: &mut Puzzle2dActionCtx<'_>, args: Option<&Value>) {
    let Some(id) = args.and_then(|value| value.get("id")).and_then(Value::as_str) else {
        return;
    };
    delete_target_regions_from_snapshot(&mut ctx.scene.board_snapshot, &[id.to_string()]);
}
