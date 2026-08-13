//! 🧩️ `duplicate-selection` command.

use crate::apps::puzzle5d::config::Puzzle5dSelection;
use crate::apps::puzzle5d::{add_palette_part, next_part_id, remove_grips, remove_parts, Puzzle5dActionCtx, Puzzle5dPart};
use semio_framework_plugin::SelectionSet;
use serde_json::{json, Value};

/// 📄️ Clones every selected part at a small flat+volume offset. Aborts (emitting nothing at all) when
/// the selection holds no parts — the pre-migration `return Emit::default()`.
pub fn duplicate_selection(ctx: &mut Puzzle5dActionCtx<'_>) {
    let ids = ctx.scene.runtime.selection.part_ids.clone();
    let clones: Vec<Puzzle5dPart> = ctx
        .scene
        .document
        .parts
        .iter()
        .filter(|part| ids.contains(&part.id))
        .map(|part| {
            let mut clone = part.clone();
            clone.id = next_part_id();
            clone.part_3d.origin[0] += 0.5;
            clone.part_3d.origin[1] += 0.5;
            clone.part_2d.x += 48.0;
            clone.part_2d.y += 24.0;
            clone
        })
        .collect();
    if clones.is_empty() {
        ctx.abort = true;
        return;
    }
    let new_ids: Vec<String> = clones.iter().map(|part| part.id.clone()).collect();
    ctx.scene.document.parts.extend(clones);
    ctx.scene.runtime.selection = Puzzle5dSelection { part_ids: SelectionSet::from_ids(new_ids), grip_ids: SelectionSet::default(), fastener_ids: SelectionSet::default() };
}
