//! 🧊️ Puzzle 3d play app commands — the object vocabulary: placing a catalogue kind (seeded with its
//! rim vortices), deleting or duplicating the selection, the hide/lock flag toggles shared with the
//! document tree's inline row actions, and the generic inspector field patch.

use crate::apps::puzzle3d::panels::inspection;
use crate::apps::puzzle3d::{
    apply_puzzle3d_inspector_patch, apply_puzzle3d_selection_flag, next_object_id, puzzle3d_vortices_from_kind_template, resolve_puzzle3d_attractions, Puzzle3dActionCtx, Puzzle3dObject,
};
use crate::apps::puzzle3d::config::Puzzle3dSelection;
use semio_framework_plugin::SelectionSet;
use serde_json::Value;
use std::collections::HashSet;

pub fn add_object_kind(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    let object_kind = args.and_then(|value| value.get("objectKind")).and_then(|value| value.as_str()).unwrap_or("Object");
    let id = next_object_id();
    let catalog_entry = ctx.scene.fixture.meta.kind_catalogs.as_ref().and_then(|catalogs| catalogs.get("objects")?.as_array()?.iter().find(|entry| entry.get("id").and_then(|v| v.as_str()) == Some(object_kind)).cloned());
    let mesh_url = catalog_entry.as_ref().and_then(|entry| entry.get("meshUrl").and_then(|v| v.as_str()).map(str::to_string));
    let vortices = catalog_entry.as_ref().map(puzzle3d_vortices_from_kind_template).unwrap_or_default();
    let origin = args
        .and_then(|value| value.get("origin"))
        .and_then(|value| value.as_array())
        .map_or([0.0, 0.0, 0.0], |values| [values.first().and_then(|v| v.as_f64()).unwrap_or(0.0), values.get(1).and_then(|v| v.as_f64()).unwrap_or(0.0), values.get(2).and_then(|v| v.as_f64()).unwrap_or(0.0)]);
    ctx.scene.fixture.objects.push(Puzzle3dObject {
        id: id.clone(),
        label: Some(object_kind.into()),
        object_kind: Some(object_kind.into()),
        origin,
        orientation: Some([0.0, 0.0, 0.0, 1.0]),
        scale: None,
        mesh_url,
        vortices,
        hidden: false,
        locked: false,
        reveal_index: None,
    });
    ctx.scene.runtime.selection.object_ids = SelectionSet::from(vec![id]);
    resolve_puzzle3d_attractions(&mut ctx.scene.fixture);
}

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

/// 🙈️ Explicit `{entity, ids}` (the document tree's row actions) patches exactly those; otherwise the
/// whole live object/vortex/target-volume selection is flagged at once (the context menu's path).
pub fn set_selection_flag(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    let flag = args.and_then(|value| value.get("flag")).and_then(|value| value.as_str()).unwrap_or("hidden");
    let value = args.and_then(|value| value.get("value")).and_then(|value| value.as_bool()).unwrap_or(true);
    let entity = args.and_then(|value| value.get("entity")).and_then(|value| value.as_str());
    let explicit_ids: Option<Vec<String>> = args.and_then(|value| value.get("ids")).and_then(|value| serde_json::from_value(value.clone()).ok());
    match (entity, explicit_ids) {
        (Some(entity), Some(ids)) => apply_puzzle3d_selection_flag(&mut ctx.scene.fixture, entity, &ids, flag, value),
        _ => {
            let selection = ctx.scene.runtime.selection.clone();
            apply_puzzle3d_selection_flag(&mut ctx.scene.fixture, "object", selection.object_ids.as_slice(), flag, value);
            apply_puzzle3d_selection_flag(&mut ctx.scene.fixture, "vortex", selection.vortex_ids.as_slice(), flag, value);
            apply_puzzle3d_selection_flag(&mut ctx.scene.fixture, "targetVolume", selection.target_volume_ids.as_slice(), flag, value);
        }
    }
}

pub fn patch_inspector(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    let entity = args.and_then(|value| value.get("entity")).and_then(|value| value.as_str()).unwrap_or("");
    let field = args.and_then(|value| value.get("field")).and_then(|value| value.as_str()).unwrap_or("");
    let ids = args
        .and_then(|value| value.get("ids"))
        .and_then(|value| serde_json::from_value::<Vec<String>>(value.clone()).ok())
        .filter(|ids| !ids.is_empty())
        .unwrap_or_else(|| inspection::target_ids(entity, &ctx.scene.runtime.selection));
    let value = args.and_then(|value| value.get("value"));
    let delta = args.and_then(|value| value.get("delta"));
    apply_puzzle3d_inspector_patch(&mut ctx.scene.fixture, entity, &ids, field, value, delta);
    resolve_puzzle3d_attractions(&mut ctx.scene.fixture);
}
