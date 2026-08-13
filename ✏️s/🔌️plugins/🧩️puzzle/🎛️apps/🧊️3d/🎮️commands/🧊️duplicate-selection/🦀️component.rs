//! 🧊️ `duplicate-selection` command.

use crate::apps::puzzle3d::panels::inspection;
use crate::apps::puzzle3d::config::Puzzle3dSelection;
use semio_framework_plugin::SelectionSet;
use serde_json::Value;
use std::collections::HashSet;
use crate::apps::puzzle3d::Puzzle3dActionCtx;
use crate::apps::puzzle3d::next_object_id;
use crate::apps::puzzle3d::resolve_puzzle3d_attractions;
use crate::apps::puzzle3d::Puzzle3dObject;

pub fn duplicate_selection(ctx: &mut Puzzle3dActionCtx<'_>) {
    let ids = &ctx.scene.runtime.selection.object_ids;
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
    let new_ids: Vec<String> = clones.iter().map(|object| object.id.clone()).collect();
    ctx.scene.fixture.objects.extend(clones);
    ctx.scene.runtime.selection.object_ids = SelectionSet::from(new_ids);
    resolve_puzzle3d_attractions(&mut ctx.scene.fixture);
}
