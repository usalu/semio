//! 🔗️ `delete-fastener` command.

use crate::apps::puzzle5d::{find_part_by_grip_full_id, next_fastener_id, puzzle5d_grip_full_id, puzzle5d_resolve_number_edit, world_grip_position, Puzzle5dActionCtx, Puzzle5dDocument, Puzzle5dFastener, PUZZLE5D_PROXIMITY_RADIUS};
use serde_json::Value;

fn arg_str<'a>(args: Option<&'a Value>, key: &str) -> Option<&'a str> {
    args.and_then(|value| value.get(key)).and_then(Value::as_str).filter(|text| !text.is_empty())
}

/// 🗑️ Deletes one fastener by id.
pub fn delete_fastener(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    let Some(id) = arg_str(args, "id").or_else(|| arg_str(args, "fastenerId")) else {
        return;
    };
    ctx.scene.document.fasteners.retain(|fastener| fastener.id != id);
}
