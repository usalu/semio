//! 🧊️ `set-selection-flag` command, and the two set-verbs a document-tree row names (`setSelectionHidden`,
//! `setSelectionLocked`).

use crate::editor::puzzle3d::{puzzle3d_vortex_full_id, Puzzle3dActionCtx};
use crate::standards::v1::subsets::any::schema::mutations::{change_object_hidden, change_object_locked, change_reference_hidden, change_reference_locked, change_target_volume_hidden, change_target_volume_locked, replace_object_vortex, Puzzle3dMutation};
use crate::Puzzle3dSnapshot;
use semio_framework_pack_json::Value;

/// 🙈️ The concrete kinds that put `flag` (`locked`, else `hidden`) at `value` on the named entities of one `entity`
/// kind (`object`, `vortex` by full id, `reference`, `targetVolume`) — one row per entity whose flag differs in `base`.
pub fn selection_flag_mutations(base: &Puzzle3dSnapshot, entity: &str, ids: &[String], flag: &str, value: bool) -> Vec<Puzzle3dMutation> {
    let locked = flag == "locked";
    match entity {
        "object" => base
            .objects
            .iter()
            .filter(|object| ids.contains(&object.id) && (if locked { object.locked } else { object.hidden }) != value)
            .map(|object| if locked { change_object_locked(object.id.clone(), value) } else { change_object_hidden(object.id.clone(), value) })
            .collect(),
        "vortex" => base
            .objects
            .iter()
            .flat_map(|object| {
                object.vortices.iter().filter(move |vortex| ids.contains(&puzzle3d_vortex_full_id(&object.id, &vortex.id)) && (if locked { vortex.locked } else { vortex.hidden }) != value).map(move |vortex| {
                    let mut flagged = vortex.clone();
                    if locked {
                        flagged.locked = value;
                    } else {
                        flagged.hidden = value;
                    }
                    replace_object_vortex(object.id.clone(), vortex.id.clone(), flagged)
                })
            })
            .collect(),
        "reference" => base
            .references
            .iter()
            .filter(|reference| ids.contains(&reference.id) && (if locked { reference.locked } else { reference.hidden }) != value)
            .map(|reference| if locked { change_reference_locked(reference.id.clone(), value) } else { change_reference_hidden(reference.id.clone(), value) })
            .collect(),
        "targetVolume" => base
            .target_volumes
            .iter()
            .filter(|volume| ids.contains(&volume.id) && (if locked { volume.locked } else { volume.hidden }) != value)
            .map(|volume| if locked { change_target_volume_locked(volume.id.clone(), value) } else { change_target_volume_hidden(volume.id.clone(), value) })
            .collect(),
        _ => Vec::new(),
    }
}

/// 🙈️ Explicit `{entity, ids}` (the document tree's row actions) patches exactly those; otherwise the
/// whole live object/vortex/target-volume selection is flagged at once (the context menu's path).
pub fn set_selection_flag(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    let flag = args.and_then(|value| value.get("flag")).and_then(|value| value.as_str()).unwrap_or("hidden");
    let value = args.and_then(|value| value.get("value")).and_then(|value| value.as_bool()).unwrap_or(true);
    apply(ctx, args, flag, value);
}

/// 🎯️ `setSelectionHidden{hidden}`/`setSelectionLocked{locked}`: `flag` set to exactly the boolean the arguments carry —
/// the row target's explicit next state, so replaying it changes nothing. `command_from_action` already refused a
/// missing value, so there is no default to fall back to here.
pub fn set_selection_flag_value(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>, flag: &str) {
    if let Some(value) = args.and_then(|value| value.get(flag)).and_then(|value| value.as_bool()) {
        apply(ctx, args, flag, value);
    }
}

fn apply(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>, flag: &str, value: bool) {
    let entity = args.and_then(|value| value.get("entity")).and_then(|value| value.as_str());
    let explicit_ids: Option<Vec<String>> = args.and_then(|value| value.get("ids")).and_then(|value| semio_framework_value::FromValue::from_value(semio_framework_pack_json::to_dsl_value(value)).ok());
    let mutations: Vec<Puzzle3dMutation> = match (entity, explicit_ids) {
        (Some(entity), Some(ids)) => selection_flag_mutations(&**ctx.base, entity, &ids, flag, value),
        _ => {
            let object_ids = ctx.selected_object_ids();
            let vortex_ids = ctx.selected_vortex_ids();
            let target_volume_ids = ctx.selected_target_volume_ids();
            let base = &**ctx.base;
            [("object", object_ids), ("vortex", vortex_ids), ("targetVolume", target_volume_ids)].into_iter().flat_map(|(entity, ids)| selection_flag_mutations(base, entity, &ids, flag, value)).collect()
        }
    };
    ctx.artifact_mutations.extend(mutations);
}
