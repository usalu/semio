//! 💞️ `create-edge` command.

use crate::editor::puzzle2d::{new_edge_id, puzzle2d_handle_kind, puzzle2d_kinds_compatible, puzzle2d_occupied_handles, Puzzle2dActionCtx};
use semio_framework_plugin::kernel::Effect;
use serde_json::{json, Value};

/// 🔗️ Connects two handles into one edge: the pair must resolve to two different handles of the
/// document, neither already occupied by an edge, whose kinds `meta.kindCompatibility` admits.
/// Every refusal raises exactly one notice and emits no edit, so an engagement line or a menu row
/// that cannot connect says why instead of silently doing nothing.
pub fn create_edge(ctx: &mut Puzzle2dActionCtx<'_>, args: Option<&Value>) {
    let read = |key: &str| args.and_then(|value| value.get(key)).and_then(Value::as_str).filter(|text| !text.is_empty()).map(str::to_string);
    let (Some(source), Some(target)) = (read("source").or_else(|| read("attracting")), read("target").or_else(|| read("attracted"))) else {
        ctx.effects.push(Effect::Notify { message: ctx.labels.connect_needs_two_handles.as_str().to_string() });
        return;
    };
    if source == target {
        ctx.effects.push(Effect::Notify { message: ctx.labels.connect_needs_two_handles.as_str().to_string() });
        return;
    }
    let (Some(source_kind), Some(target_kind)) = (puzzle2d_handle_kind(&ctx.scene.fixture, &source), puzzle2d_handle_kind(&ctx.scene.fixture, &target)) else {
        ctx.effects.push(Effect::Notify { message: ctx.labels.connect_unknown_handle.as_str().to_string() });
        return;
    };
    let occupied = puzzle2d_occupied_handles(&ctx.scene.fixture);
    if occupied.contains(&source) || occupied.contains(&target) {
        ctx.effects.push(Effect::Notify { message: ctx.labels.connect_handle_occupied.as_str().to_string() });
        return;
    }
    if !puzzle2d_kinds_compatible(&ctx.scene.fixture, &source_kind, &target_kind) {
        ctx.effects.push(Effect::Notify { message: ctx.labels.connect_kind_incompatible.as_str().to_string() });
        return;
    }
    let id = new_edge_id(&ctx.scene.fixture);
    let edge_kind = read("edgeKind");
    let mut edge = json!({ "id": id, "source": source, "target": target });
    if let Some(edge_kind) = edge_kind {
        edge["edgeKind"] = json!(edge_kind);
    }
    crate::editor::puzzle2d::puzzle2d_push_edge(&mut ctx.scene.fixture, edge);
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
