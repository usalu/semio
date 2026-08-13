//! 🕸️ `patch-inspector` command.

use crate::apps::puzzle2d::{add_node_to_fixture, patch_inspector_nodes, Puzzle2dActionCtx};
use serde_json::Value;

pub fn patch_inspector(ctx: &mut Puzzle2dActionCtx<'_>, args: Option<&Value>) {
    let ids: Vec<String> = args.and_then(|value| value.get("ids")).and_then(|value| serde_json::from_value(value.clone()).ok()).unwrap_or_else(|| ctx.scene.runtime.selected_ids.clone());
    let field = args.and_then(|value| value.get("field")).and_then(|value| value.as_str()).unwrap_or("");
    let value = args.and_then(|value| value.get("value"));
    let delta = args.and_then(|value| value.get("delta"));
    if !field.is_empty() {
        patch_inspector_nodes(&mut ctx.scene.fixture, &ids, field, value, delta);
    }
}
