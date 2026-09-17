//! 💔️ `delete-edge` command.

use crate::editor::puzzle2d::Puzzle2dActionCtx;
use serde_json::Value;

/// 💔️ Drops one edge by id — the inverse of `createEdge`, and the programmatic twin of the board
/// engine's own `edgeDelete` event.
pub fn delete_edge(ctx: &mut Puzzle2dActionCtx<'_>, args: Option<&Value>) {
    let Some(id) = args.and_then(|value| value.get("id")).and_then(Value::as_str).filter(|text| !text.is_empty()) else {
        return;
    };
    if let Some(edges) = ctx.scene.fixture.get_mut("edges").and_then(Value::as_array_mut) {
        edges.retain(|edge| edge.get("id").and_then(Value::as_str) != Some(id));
    }
}
