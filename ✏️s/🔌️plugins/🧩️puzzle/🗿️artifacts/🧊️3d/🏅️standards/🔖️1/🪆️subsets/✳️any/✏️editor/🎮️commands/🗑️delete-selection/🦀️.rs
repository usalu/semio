//! 🧊️ `delete-selection` command.

use crate::editor::puzzle3d::{puzzle3d_vortex_full_id, Puzzle3dActionCtx};
use crate::standards::v1::subsets::any::schema::mutations::{delete_object, delete_reference, delete_target_volume, disconnect_vortices, remove_object_vortex, Puzzle3dMutation};
use crate::{Puzzle3dAttraction, Puzzle3dSnapshot};
use std::collections::HashSet;

/// 🗑️ The concrete kinds one delete gesture consists of, in base order: the explicitly selected attractions no
/// cascade severs (`disconnect-vortices`), the selected vortices of surviving objects (`remove-object-vortex`), the
/// selected objects (`delete-object`, which severs its own attractions), target volumes and references. Ids the base
/// does not hold name nothing and produce no row.
pub fn delete_selection_mutations(base: &Puzzle3dSnapshot, object_ids: &[String], vortex_ids: &HashSet<String>, attraction_ids: &[String], target_volume_ids: &[String], reference_ids: &[String]) -> Vec<Puzzle3dMutation> {
    let deleted: Vec<&str> = base.objects.iter().map(|object| object.id.as_str()).filter(|id| object_ids.iter().any(|selected| selected == id)).collect();
    let removed_vortices: Vec<(&str, &str)> = base
        .objects
        .iter()
        .filter(|object| !deleted.contains(&object.id.as_str()))
        .flat_map(|object| object.vortices.iter().filter(move |vortex| vortex_ids.contains(&puzzle3d_vortex_full_id(&object.id, &vortex.id))).map(move |vortex| (object.id.as_str(), vortex.id.as_str())))
        .collect();
    let removed_full_ids: Vec<String> = removed_vortices.iter().map(|(object, vortex)| puzzle3d_vortex_full_id(object, vortex)).collect();
    let severed = |attraction: &Puzzle3dAttraction| {
        [attraction.attracting.as_str(), attraction.attracted.as_str()].into_iter().any(|end| removed_full_ids.iter().any(|removed| removed == end) || deleted.iter().any(|object| end.strip_prefix(object).is_some_and(|rest| rest.starts_with(':'))))
    };
    let disconnects = base.attractions.iter().filter(|attraction| attraction_ids.contains(&attraction.id) && !severed(*attraction)).map(|attraction| disconnect_vortices(attraction.id.clone()));
    let vortices = removed_vortices.iter().map(|(object, vortex)| remove_object_vortex((*object).to_string(), (*vortex).to_string()));
    let objects = deleted.iter().map(|id| delete_object((*id).to_string()));
    let volumes = base.target_volumes.iter().filter(|volume| target_volume_ids.contains(&volume.id)).map(|volume| delete_target_volume(volume.id.clone()));
    let references = base.references.iter().filter(|reference| reference_ids.contains(&reference.id)).map(|reference| delete_reference(reference.id.clone()));
    disconnects.chain(vortices).chain(objects).chain(volumes).chain(references).collect()
}

/// 🗑️ Removes every selected entity from the document AND retires the ids it just destroyed from the
/// framework-owned `vortex` selection through the one sanctioned reducer channel
/// ([`Puzzle3dActionCtx::clear_selection`]) — a delete that leaves its own victims selected turns the
/// NEXT `Delete` into an empty edit against a phantom id, and leaves the gumball and the inspector
/// bound to an object the world no longer carries (ticket 26/09/02/PUZZLE-3D-END-TO-END wave B31: the
/// browser census read `before=2 after=2` for 30 s while the command log recorded
/// `delete-object id=object-1`). That write is why this tool's publication contract carries the
/// `Interaction` lane beside `Artifact`.
pub fn delete_selection(ctx: &mut Puzzle3dActionCtx<'_>) {
    let object_ids: Vec<String> = ctx.selected_object_ids();
    let vortex_ids: HashSet<String> = ctx.selected_vortex_ids().into_iter().collect();
    let attraction_ids: Vec<String> = ctx.selected_attraction_ids();
    let target_volume_ids: Vec<String> = ctx.selected_target_volume_ids();
    let reference_ids: Vec<String> = ctx.selected_reference_ids();
    let marked: Vec<String> = object_ids.iter().chain(vortex_ids.iter()).chain(attraction_ids.iter()).chain(target_volume_ids.iter()).chain(reference_ids.iter()).cloned().collect();
    if ctx.refuse_without_selection(&marked) {
        return;
    }
    // 🔒️ A lock has to mean the same thing to every verb. `translateSelection` has refused a locked
    // grab since wave B31 while `deleteSelection` erased the very same object without a word — the
    // 2026-09-17 ◻️2d battery measured the twin defect there (`before=12 after=0` on a node whose
    // inspector flag read `locked true` and whose drag WAS refused). One notice, no edit, no fault.
    let locked = ctx.scene.scene_snapshot.objects.iter().any(|object| object_ids.contains(&object.id) && object.locked)
        || ctx.scene.scene_snapshot.objects.iter().any(|object| {
            object.vortices.iter().any(|vortex| vortex.locked && vortex_ids.contains(&crate::editor::puzzle3d::puzzle3d_vortex_full_id(&object.id, &vortex.id)))
        })
        || ctx.scene.scene_snapshot.target_volumes.iter().any(|volume| target_volume_ids.contains(&volume.id) && volume.locked)
        || ctx.scene.scene_snapshot.references.iter().any(|reference| reference_ids.contains(&reference.id) && reference.locked);
    if locked && ctx.refuse_when_locked() {
        return;
    }
    let mutations = delete_selection_mutations(&**ctx.base, &object_ids, &vortex_ids, &attraction_ids, &target_volume_ids, &reference_ids);
    ctx.artifact_mutations.extend(mutations);
    ctx.clear_selection();
}
