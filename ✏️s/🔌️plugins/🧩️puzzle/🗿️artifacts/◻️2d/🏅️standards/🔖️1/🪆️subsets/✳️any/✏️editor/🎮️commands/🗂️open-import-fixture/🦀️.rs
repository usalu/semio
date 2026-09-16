//! 🗂 Asks the host to pick a fixture JSON file and re-dispatch import.

use crate::editor::puzzle2d::Puzzle2dActionCtx;
use semio_framework::kernel::UiDirtyScope;
use semio_framework_plugin::kernel::Effect;

/// 🗂 Opens a file picker that re-dispatches `importFixture` with the picked text.
pub fn open_import_fixture(ctx: &mut Puzzle2dActionCtx<'_>) {
    ctx.effects.push(Effect::RequestFileOpen { req: semio_framework_plugin::RequestId(122), accept: "application/json,.json".into(), read_as: Some("text".into()), import_action: "importFixture".into(), multiple: false });
    *ctx.ui_scope = UiDirtyScope::None;
}
