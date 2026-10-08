//! 🕸️ `patch-inspector` command.

use crate::editor::puzzle2d::{Puzzle2dActionCtx, Puzzle2dSelectionRecord};
use semio_framework_pack_json::Value;

/// 🩹️ Writes one inspector field over the addressed entities. A position `delta` (`x`/`y` stepped by the
/// inspector) is a selection transform: ONE select-tool transaction yielding the same `drag-selection` leaf a
/// board drag yields, over the addressed nodes as literal targets (every node when the row names none). Any
/// other field — and an absolute position `value` — records the one concrete kind that owns the field (`move-node`,
/// `replace-node-geometry`, `edit-node-text`, `replace-node-handle`, …). A LOCKED entity refuses the patch with
/// one visible sentence: an editable stepper that silently swallows its own value is the defect the 2026-09-17
/// battery named. `hidden`/`locked` themselves are never gated — the lock row has to stay pressable, or a locked
/// node could never be unlocked again (the inspector's flag rows send `setSelectionFlag`, but the field names are
/// honoured here too so no route can wedge the document).
pub fn patch_inspector(ctx: &mut Puzzle2dActionCtx<'_>, args: Option<&Value>) {
    let ids: Vec<String> = args.and_then(|value| value.get("ids")).and_then(|value| semio_framework_value::FromValue::from_value(semio_framework_value::ToValue::to_value(&value.clone())).ok()).unwrap_or_else(|| ctx.selected_ids());
    let field = args.and_then(|value| value.get("field")).and_then(|value| value.as_str()).unwrap_or("");
    let value = args.and_then(|value| value.get("value"));
    let delta = args.and_then(|value| value.get("delta"));
    if field.is_empty() {
        return;
    }
    if let (Some(offset), None, "x" | "y") = (delta.and_then(Value::as_f64).filter(|offset| offset.is_finite()), value, field) {
        let document = ctx.base;
        let targets: Vec<String> = document.typed().nodes.iter().filter(|node| ids.is_empty() || ids.iter().any(|id| node.id.eq_str(id))).map(|node| node.id.to_string_owner()).collect();
        let (dx, dy) = if field == "x" { (offset, 0.0) } else { (0.0, offset) };
        ctx.commit_selection("patchInspectorNodes", vec![Puzzle2dSelectionRecord { connect: false, ..Puzzle2dSelectionRecord::drag(targets, dx, dy) }]);
        return;
    }
    if !matches!(field, "hidden" | "locked") && ctx.refuse_when_locked(&ids) {
        return;
    }
    ctx.recorder.patch_fields(&ids, field, value, delta);
}
