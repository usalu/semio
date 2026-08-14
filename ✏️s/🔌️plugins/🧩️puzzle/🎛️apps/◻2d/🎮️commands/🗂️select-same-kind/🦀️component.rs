//! 🗂️ `select-same-kind` command.

use crate::apps::puzzle2d::Puzzle2dActionCtx;

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM known gap: selection is
/// framework-owned now and `handle` has no channel to write it back — see puzzle3d's
/// `select-same-kind` doc comment for the identical limitation. No-ops.
pub fn select_same_kind(_ctx: &mut Puzzle2dActionCtx<'_>) {}
