//! 🚩️ `set-selection-flag` command, and the two set-verbs a document-tree part row names (`setSelectionHidden`,
//! `setSelectionLocked`).

use crate::editor::puzzle5d::{Puzzle5dActionCtx, PUZZLE5D_GRANULARITY_PART};
use crate::standards::v1::subsets::any::schema::mutations::{change_part_2d_hidden, change_part_2d_locked, Puzzle5dMutation};
use crate::Puzzle5dSnapshot;
use semio_framework_pack_json::Value;

/// 🙈️ The concrete kinds that put `flag` (`locked`, else `hidden`) at `value` on the named parts — one row per part
/// whose flag differs in `base`. Only a part carries these flags in 5d (a grip and a fastener have no such field), so
/// any other `entity` produces nothing rather than a fault.
pub fn selection_flag_mutations(base: &Puzzle5dSnapshot, entity: &str, ids: &[String], flag: &str, value: bool) -> Vec<Puzzle5dMutation> {
    if entity != PUZZLE5D_GRANULARITY_PART {
        return Vec::new();
    }
    base.parts
        .iter()
        .filter(|part| ids.contains(&part.id))
        .filter_map(|part| match flag {
            "hidden" if part.part_2d.hidden != Some(value) => Some(change_part_2d_hidden(part.id.clone(), Some(value))),
            "locked" if part.part_2d.locked != Some(value) => Some(change_part_2d_locked(part.id.clone(), Some(value))),
            _ => None,
        })
        .collect()
}

/// 👁️ Explicit `{entity, ids}` (the document tree's row actions) flags exactly those; otherwise the
/// whole live part selection is flagged at once (the context menu's and inspector's path).
pub fn set_selection_flag(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    let flag = args.and_then(|value| value.get("flag")).and_then(|value| value.as_str()).unwrap_or("hidden");
    let value = args.and_then(|value| value.get("value")).and_then(|value| value.as_bool()).unwrap_or(true);
    apply(ctx, args, flag, value);
}

/// 🎯️ `setSelectionHidden{hidden}`/`setSelectionLocked{locked}`: `flag` set to exactly the boolean the arguments carry —
/// the row target's explicit next state, so replaying it changes nothing. `command_from_action` already refused a
/// missing value, so there is no default to fall back to here.
pub fn set_selection_flag_value(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>, flag: &str) {
    if let Some(value) = args.and_then(|value| value.get(flag)).and_then(Value::as_bool) {
        apply(ctx, args, flag, value);
    }
}

fn apply(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>, flag: &str, value: bool) {
    let entity = args.and_then(|value| value.get("entity")).and_then(|value| value.as_str());
    let explicit_ids: Option<Vec<String>> = args.and_then(|value| value.get("ids")).and_then(|value| semio_framework_value::FromValue::from_value(semio_framework_pack_json::to_dsl_value(value)).ok());
    let mutations = match (entity, explicit_ids) {
        (Some(entity), Some(ids)) => selection_flag_mutations(ctx.snapshot.typed(), entity, &ids, flag, value),
        _ => selection_flag_mutations(ctx.snapshot.typed(), PUZZLE5D_GRANULARITY_PART, &ctx.selected_part_ids(), flag, value),
    };
    ctx.artifact_mutations.extend(mutations);
}
