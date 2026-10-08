//! 🚩️ `set-target-volume-flag` command, and the two set-verbs an outliner target-volume row names
//! (`setTargetVolumeHidden`, `setTargetVolumeLocked`).

use crate::editor::puzzle5d::Puzzle5dActionCtx;
use crate::standards::v1::subsets::any::schema::mutations::{change_target_volume_hidden, change_target_volume_locked};
use semio_framework_pack_json::Value;

/// 🚩️ One flag of one target volume by name. An unknown flag name writes nothing, so a stale caller cannot corrupt the
/// other flag.
pub fn set_target_volume_flag(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    let flag = args.and_then(|value| value.get("flag")).and_then(Value::as_str).unwrap_or("");
    let value = args.and_then(|value| value.get("value")).and_then(Value::as_bool).unwrap_or(false);
    apply(ctx, args, flag, value);
}

/// 🎯️ The outliner's show/hide and lock/unlock toggles for one target volume — `setTargetVolumeHidden{hidden}`/
/// `setTargetVolumeLocked{locked}` set `flag` to exactly the boolean the arguments carry, the row target's explicit next
/// state, so replaying one changes nothing. `command_from_action` already refused a missing value.
pub fn set_target_volume_flag_value(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>, flag: &str) {
    if let Some(value) = args.and_then(|value| value.get(flag)).and_then(Value::as_bool) {
        apply(ctx, args, flag, value);
    }
}

fn apply(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>, flag: &str, value: bool) {
    let id = args.and_then(|value| value.get("id")).and_then(Value::as_str).unwrap_or("");
    let Some(volume) = ctx.snapshot.typed().target_volumes.iter().find(|volume| volume.id == id) else { return };
    let mutation = match flag {
        "hidden" if volume.hidden != value => change_target_volume_hidden(id.to_string(), value),
        "locked" if volume.locked != value => change_target_volume_locked(id.to_string(), value),
        _ => return,
    };
    ctx.artifact_mutations.push(mutation);
}
