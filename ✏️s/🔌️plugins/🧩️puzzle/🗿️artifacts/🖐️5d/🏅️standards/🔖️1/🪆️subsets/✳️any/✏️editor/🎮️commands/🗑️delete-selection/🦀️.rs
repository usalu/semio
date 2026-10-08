//! 🗑️ `delete-selection` command.

use crate::editor::puzzle5d::{puzzle5d_grip_full_id, Puzzle5dActionCtx};
use crate::standards::v1::subsets::any::schema::mutations::{delete_part, disconnect_grips, remove_part_grip, Puzzle5dMutation};
use crate::{Puzzle5dFastener, Puzzle5dSnapshot};

/// 🗑️ The concrete kinds one delete gesture consists of, in base order: the selected fasteners no cascade severs
/// (`disconnect-grips`), the selected grips of surviving parts (`remove-part-grip`, which severs its own fasteners),
/// then the selected parts (`delete-part`, which severs theirs). Ids the base does not hold produce no row.
pub fn delete_selection_mutations(base: &Puzzle5dSnapshot, part_ids: &[String], grip_ids: &[String], fastener_ids: &[String]) -> Vec<Puzzle5dMutation> {
    let deleted: Vec<&str> = base.parts.iter().map(|part| part.id.as_str()).filter(|id| part_ids.iter().any(|selected| selected == id)).collect();
    let removed_grips: Vec<(&str, &str)> = base
        .parts
        .iter()
        .filter(|part| !deleted.contains(&part.id.as_str()))
        .flat_map(|part| part.grips.iter().filter(move |grip| grip_ids.contains(&puzzle5d_grip_full_id(&part.id, &grip.id))).map(move |grip| (part.id.as_str(), grip.id.as_str())))
        .collect();
    let removed_full_ids: Vec<String> = removed_grips.iter().map(|(part, grip)| puzzle5d_grip_full_id(part, grip)).collect();
    let severed = |fastener: &Puzzle5dFastener| {
        [fastener.source.as_str(), fastener.target.as_str()].into_iter().any(|end| removed_full_ids.iter().any(|removed| removed == end) || deleted.iter().any(|part| end.strip_prefix(part).is_some_and(|rest| rest.starts_with(':'))))
    };
    let disconnects = base.fasteners.iter().filter(|fastener| fastener_ids.contains(&fastener.id) && !severed(*fastener)).map(|fastener| disconnect_grips(fastener.id.clone()));
    let grips = removed_grips.iter().map(|(part, grip)| remove_part_grip((*part).to_string(), (*grip).to_string()));
    let parts = deleted.iter().map(|id| delete_part((*id).to_string()));
    disconnects.chain(grips).chain(parts).collect()
}

/// 🗑️ Removes every selected part (and its fasteners), grip and fastener.
///
/// 🔒️ A LOCKED part is never removed, and a selection whose every part is locked refuses with ONE
/// visible notice — deleting nothing silently is indistinguishable from a dead Delete key.
pub fn delete_selection(ctx: &mut Puzzle5dActionCtx<'_>) {
    let selected_parts = ctx.selected_part_ids();
    if ctx.refuse_when_locked(&selected_parts) {
        return;
    }
    let part_ids: Vec<String> = selected_parts.into_iter().filter(|id| !ctx.scene.document.parts.iter().any(|part| &part.id == id && part.part_2d.locked.unwrap_or(false))).collect();
    let grip_ids = ctx.selected_grip_ids();
    let fastener_ids = ctx.selected_fastener_ids();
    let mutations = delete_selection_mutations(ctx.snapshot.typed(), &part_ids, &grip_ids, &fastener_ids);
    ctx.artifact_mutations.extend(mutations);
}
