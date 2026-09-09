//! 🧊️ `duplicate-selection` command.

use crate::editor::puzzle3d::next_object_id;
use crate::editor::puzzle3d::resolve_puzzle3d_attractions;
use crate::editor::puzzle3d::Puzzle3dActionCtx;
use crate::editor::puzzle3d::Puzzle3dObject;
use crate::editor::puzzle3d::PUZZLE3D_GRANULARITY_OBJECT;

/// 👯️ Clones every selected object, offset half a unit, and re-selects the clones — the framework
/// applies that re-selection from `Emit.interaction_writes` once the document mutations have landed,
/// so the new ids are already in interaction topology by then (ticket
/// 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM).
pub fn duplicate_selection(ctx: &mut Puzzle3dActionCtx<'_>) {
    let ids = ctx.selected_object_ids();
    if ctx.refuse_without_selection(&ids) {
        return;
    }
    let clones: Vec<Puzzle3dObject> = ctx
        .scene
        .fixture
        .objects
        .iter()
        .filter(|object| ids.contains(&object.id))
        .map(|object| {
            let mut clone = object.clone();
            clone.id = next_object_id();
            clone.origin[0] += 0.5;
            clone.origin[1] += 0.5;
            clone
        })
        .collect();
    let clone_ids: Vec<String> = clones.iter().map(|clone| clone.id.clone()).collect();
    ctx.scene.fixture.objects.extend(clones);
    resolve_puzzle3d_attractions(&mut ctx.scene.fixture);
    ctx.replace_selection(PUZZLE3D_GRANULARITY_OBJECT, clone_ids);
}
