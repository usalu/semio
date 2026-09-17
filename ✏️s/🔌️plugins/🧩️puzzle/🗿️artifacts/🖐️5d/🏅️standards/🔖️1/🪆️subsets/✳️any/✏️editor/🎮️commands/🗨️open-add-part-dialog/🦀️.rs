//! 🗨️ Opens the declared "addPart" dialog, whose submit dispatches `addPartKind`.

use crate::editor::puzzle5d::Puzzle5dActionCtx;
use semio_framework_plugin::kernel::Effect;

/// 🗨️ The dialog id `create_puzzle5d_app` registers, named once so the effect and the
/// `DialogDefinition` can never drift apart.
pub const PUZZLE5D_ADD_PART_DIALOG: &str = "addPart";

/// 🗨️ The request id this app's dialog answers on — distinct from puzzle 3d's `addObject` (120).
pub const PUZZLE5D_ADD_PART_DIALOG_REQUEST: u64 = 124;

/// 🗨️ Shell-only effect: no document mutation, no history row. The dialog's own `partKind` select IS
/// the argument form, so the submit lands as an ordinary `addPartKind` invocation.
pub fn open_add_part_dialog(ctx: &mut Puzzle5dActionCtx<'_>) {
    ctx.effects.push(Effect::OpenDialog { req: semio_framework_plugin::RequestId(PUZZLE5D_ADD_PART_DIALOG_REQUEST), dialog_id: PUZZLE5D_ADD_PART_DIALOG.into(), args: None });
    ctx.abort = true;
}
