//! 🧊️ `delete-selection` command.

use std::collections::HashSet;
use crate::editor::puzzle3d::Puzzle3dActionCtx;

pub fn delete_selection(ctx: &mut Puzzle3dActionCtx<'_>) {
    let object_ids: Vec<String> = ctx.selected_object_ids();
    let vortex_ids: HashSet<String> = ctx.selected_vortex_ids().into_iter().collect();
    let attraction_ids: Vec<String> = ctx.selected_attraction_ids();
    let target_volume_ids: Vec<String> = ctx.selected_target_volume_ids();
    let reference_ids: Vec<String> = ctx.selected_reference_ids();
    ctx.scene.fixture.objects.retain(|object| !object_ids.contains(&object.id));
    if !vortex_ids.is_empty() {
        for object in ctx.scene.fixture.objects.iter_mut() {
            object.vortices.retain(|vortex| !vortex_ids.contains(&crate::editor::puzzle3d::puzzle3d_vortex_full_id(&object.id, &vortex.id)));
        }
    }
    ctx.scene.fixture.attractions.retain(|attraction| !attraction_ids.contains(&attraction.id) && !object_ids.iter().any(|id| attraction.attracting.starts_with(&format!("{id}:")) || attraction.attracted.starts_with(&format!("{id}:"))));
    ctx.scene.fixture.target_volumes.retain(|volume| !target_volume_ids.contains(&volume.id));
    ctx.scene.fixture.references.retain(|reference| !reference_ids.contains(&reference.id));
}
