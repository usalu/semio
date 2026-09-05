//! 👯️ `duplicate-selection` command.

use crate::editor::puzzle5d::{Puzzle5dActionCtx, Puzzle5dFreshIds, Puzzle5dPart};

/// 📄️ Clones every selected part at a small flat+volume offset. Aborts (emitting nothing at all) when
/// the selection holds no parts — the pre-migration `return Emit::default()`. 🕹️ ticket
/// 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM known gap: no longer re-selects the new
/// duplicates afterward — see puzzle3d's `duplicate-selection` doc comment for the identical
/// limitation.
pub fn duplicate_selection(ctx: &mut Puzzle5dActionCtx<'_>) {
    let ids = ctx.selected_part_ids();
    let mut clones: Vec<Puzzle5dPart> = ctx
        .scene
        .document
        .parts
        .iter()
        .filter(|part| ids.contains(&part.id))
        .map(|part| {
            let mut clone = part.clone();
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
    let mut fresh_ids = Puzzle5dFreshIds::from_document(&ctx.scene.document);
    for clone in &mut clones {
        clone.id = fresh_ids.next_part();
    }
    ctx.scene.document.parts.extend(clones);
}
