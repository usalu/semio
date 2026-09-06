//! 🖌️ `fill-session-clear` command.

use crate::editor::puzzle2d::config::Puzzle2dFillRuntime;

/// 🧹️ Discards the fill session and the requested count together — the "put the brush away" verb,
/// as opposed to `brushFillSessionDiscard`, which drops the session but keeps the count.
pub fn fill_session_clear(runtime: &mut Puzzle2dFillRuntime) {
    crate::editor::puzzle2d::commands::set_fill_count::discard_fill_session(runtime);
    runtime.fill_count = 0;
}
