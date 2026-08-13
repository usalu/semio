//! 🧊️ `delete-selection` command.

use crate::apps::puzzle3d::panels::inspection;
use crate::apps::puzzle3d::config::Puzzle3dSelection;
use semio_framework_plugin::SelectionSet;
use serde_json::Value;
use std::collections::HashSet;
use crate::apps::puzzle3d::Puzzle3dActionCtx;

pub fn delete_selection(ctx: &mut Puzzle3dActionCtx<'_>) {
    let object_ids: Vec<String> = ctx.scene.runtime.selection.object_ids.to_vec();
    let vortex_ids: HashSet<String> = ctx.scene.runtime.selection.vortex_ids.iter().cloned().collect();
    let attraction_ids: Vec<String> = ctx.scene.runtime.selection.attraction_ids.to_vec();
    let target_volume_ids: Vec<String> = ctx.scene.runtime.selection.target_volume_ids.to_vec();
    ctx.scene.fixture.objects.retain(|object| !object_ids.contains(&object.id));
    if !vortex_ids.is_empty() {
        for object in ctx.scene.fixture.objects.iter_mut() {
            object.vortices.retain(|vortex| !vortex_ids.contains(&crate::apps::puzzle3d::puzzle3d_vortex_full_id(&object.id, &vortex.id)));
        }
    }
    ctx.scene.fixture.attractions.retain(|attraction| !attraction_ids.contains(&attraction.id) && !object_ids.iter().any(|id| attraction.attracting.starts_with(&format!("{id}:")) || attraction.attracted.starts_with(&format!("{id}:"))));
    ctx.scene.fixture.target_volumes.retain(|volume| !target_volume_ids.contains(&volume.id));
    let reference_ids: Vec<String> = ctx.scene.runtime.selection.reference_ids.to_vec();
    ctx.scene.fixture.references.retain(|reference| !reference_ids.contains(&reference.id));
    ctx.scene.runtime.selection = Puzzle3dSelection::default();
}
