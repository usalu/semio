//! 🕸️ `add-node` command.

use crate::editor::puzzle2d::{add_node_to_fixture, patch_inspector_nodes, Puzzle2dActionCtx};
use serde_json::Value;

pub fn add_node(ctx: &mut Puzzle2dActionCtx<'_>, args: Option<&Value>) {
    let kind = args.and_then(|value| value.get("kind")).and_then(|value| value.as_str());
    add_node_to_fixture(&mut ctx.scene.fixture, kind, args);
}
