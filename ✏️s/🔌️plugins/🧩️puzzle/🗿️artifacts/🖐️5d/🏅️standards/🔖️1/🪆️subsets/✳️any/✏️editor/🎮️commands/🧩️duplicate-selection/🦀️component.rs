//! 🧩️ `duplicate-selection` command.

use crate::editor::puzzle5d::{next_part_id, Puzzle5dActionCtx, Puzzle5dPart};

/// 📄️ Clones every selected part at a small flat+volume offset. Aborts (emitting nothing at all) when
/// the selection holds no parts — the pre-migration `return Emit::default()`. 🕹️ ticket
/// 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM known gap: no longer re-selects the new
/// duplicates afterward — see puzzle3d's `duplicate-selection` doc comment for the identical
/// limitation.
pub async fn duplicate_selection(ctx: &mut Puzzle5dActionCtx<'_>) {
    let ids = ctx.selected_part_ids();
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
    ctx.scene.document.parts.extend(clones);
}
