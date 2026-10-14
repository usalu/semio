//! 🧊️ `duplicate-selection` command.

use crate::editor::puzzle3d::snapshot::next_object_id;
use crate::editor::puzzle3d::snapshot::puzzle3d_next_object_label;
use crate::editor::puzzle3d::session::Puzzle3dActionCtx;
use crate::editor::puzzle3d::snapshot::PUZZLE3D_GRANULARITY_OBJECT;
use crate::standards::v1::subsets::any::schema::mutations::create_object;

/// 👯️ Clones every selected object, offset half a unit, and re-selects the clones — each clone is one `create-object`
/// built from the typed base object (anchor and vortex labels included), and the framework applies the re-selection
/// from `Emit.interaction_writes` once the document mutations have landed, so the new ids are already in interaction
/// topology by then (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM).
pub fn duplicate_selection(ctx: &mut Puzzle3dActionCtx<'_>) {
    let ids = ctx.selected_object_ids();
    if ctx.refuse_without_selection(&ids) {
        return;
    }
    let clones: Vec<crate::standards::v1::subsets::any::schema::Puzzle3dObject> = ctx
        .base
        .objects
        .iter()
        .filter(|object| ids.contains(&object.id))
        .map(|object| {
            let mut clone = object.clone();
            clone.id = next_object_id();
            clone.origin[0] += 0.5;
            clone.origin[1] += 0.5;
            if let Some(kind) = clone.object_kind.as_deref() {
                clone.label = Some(puzzle3d_next_object_label(&ctx.scene.scene_snapshot.objects, &ctx.scene.scene_snapshot, kind));
            }
            clone
        })
        .collect();
    let clone_ids: Vec<String> = clones.iter().map(|clone| clone.id.clone()).collect();
    ctx.artifact_mutations.extend(clones.into_iter().map(|clone| create_object(clone, None)));
    ctx.replace_selection(PUZZLE3D_GRANULARITY_OBJECT, clone_ids);
}
