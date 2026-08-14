//! 🧊️ `duplicate-selection` command.

use crate::apps::puzzle3d::panels::inspection;
use serde_json::Value;
use std::collections::HashSet;
use crate::apps::puzzle3d::Puzzle3dActionCtx;
use crate::apps::puzzle3d::next_object_id;
use crate::apps::puzzle3d::resolve_puzzle3d_attractions;
use crate::apps::puzzle3d::Puzzle3dObject;

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM known gap: no longer re-selects the
/// new duplicates afterward — selection is framework-owned and `handle` has no channel to write it
/// (see `select-same-kind`'s doc comment for the same limitation). The document-side duplicate itself
/// is unaffected.
pub fn duplicate_selection(ctx: &mut Puzzle3dActionCtx<'_>) {
    let ids = ctx.selected_object_ids();
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
    ctx.scene.fixture.objects.extend(clones);
    resolve_puzzle3d_attractions(&mut ctx.scene.fixture);
}
