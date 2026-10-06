//! 🗂 Asks the host to pick a scene_snapshot JSON file and re-dispatch import.

use crate::editor::puzzle3d::Puzzle3dActionCtx;
use semio_framework_plugin::kernel::Effect;

/// 🗂 Opens a file picker that re-dispatches `importSnapshot` with the picked bytes.
pub fn open_import_snapshot(ctx: &mut Puzzle3dActionCtx<'_>) {
    ctx.effects.push(Effect::RequestFileOpen {
        req: semio_framework_plugin::RequestId(121),
        accept: "application/json,.json".into(),
        read_as: Some("text".into()),
        import_action: "importSnapshot".into(),
        multiple: false,
    args: None, });
}
