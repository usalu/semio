//! 🗂 Asks the host to pick a document JSON file and re-dispatch import.

use crate::editor::puzzle5d::Puzzle5dActionCtx;
use semio_framework_plugin::kernel::Effect;

/// 🗂 The request id this app's file picker answers on — distinct from puzzle 3d's (121) and puzzle
/// 2d's (122), so three editors mounted in one shell never collide on one open request.
pub const PUZZLE5D_OPEN_IMPORT_REQUEST: u64 = 123;

/// 🗂 Opens a file picker that re-dispatches `importFixture` once per
/// `semio_framework::kernel::IMPORT_CHUNK_BYTES` page of the picked text.
pub fn open_import_fixture(ctx: &mut Puzzle5dActionCtx<'_>) {
    ctx.effects.push(Effect::RequestFileOpen {
        req: semio_framework_plugin::RequestId(PUZZLE5D_OPEN_IMPORT_REQUEST),
        accept: "application/json,.json".into(),
        read_as: Some("text".into()),
        import_action: "importFixture".into(),
        multiple: false,
    });
    ctx.abort = true;
}
