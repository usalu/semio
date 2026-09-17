//! 🕸️ `patch-inspector` command.

use crate::editor::puzzle2d::{patch_inspector_nodes, Puzzle2dActionCtx};
use serde_json::Value;

/// 🩹️ Writes one inspector field over the addressed entities. A LOCKED entity refuses the whole patch
/// with one visible sentence: an editable stepper that silently swallows its own value is the defect
/// the 2026-09-17 battery named. `hidden`/`locked` themselves are never gated — the lock row has to
/// stay pressable, or a locked node could never be unlocked again (the inspector's flag rows send
/// `setSelectionFlag`, but the field names are honoured here too so no route can wedge the document).
pub fn patch_inspector(ctx: &mut Puzzle2dActionCtx<'_>, args: Option<&Value>) {
    let ids: Vec<String> = args.and_then(|value| value.get("ids")).and_then(|value| serde_json::from_value(value.clone()).ok()).unwrap_or_else(|| ctx.selected_ids());
    let field = args.and_then(|value| value.get("field")).and_then(|value| value.as_str()).unwrap_or("");
    let value = args.and_then(|value| value.get("value"));
    let delta = args.and_then(|value| value.get("delta"));
    if field.is_empty() {
        return;
    }
    if !matches!(field, "hidden" | "locked") && ctx.refuse_when_locked(&ids) {
        return;
    }
    patch_inspector_nodes(&mut ctx.scene.fixture, &ids, field, value, delta);
}
