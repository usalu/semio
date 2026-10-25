//! 🧱️ instance. The 2d twin of puzzle3d's `setSelectableKind` over `objects`/`vortices`/`attractions`.
//! 🎯️ `set-selectable-kind` command — which `vortex` granularity a pick may even reach, per window

use {crate::editor::puzzle2d::snapshot::puzzle2d_window_and_measures_scope, crate::editor::puzzle2d::interaction::Puzzle2dActionCtx, crate::editor::puzzle2d::snapshot::PUZZLE2D_GRANULARITY_EDGE, crate::editor::puzzle2d::snapshot::PUZZLE2D_GRANULARITY_HANDLE, crate::editor::puzzle2d::snapshot::PUZZLE2D_GRANULARITY_NODE};
use semio_framework_pack_json::Value;

/// 🗂️ `kind` names one granularity (`node` / `handle` / `edge`, plurals accepted for symmetry with
/// puzzle3d's own arg vocabulary); a missing `pressed` flips that one flag.
pub fn set_selectable_kind(ctx: &mut Puzzle2dActionCtx<'_>, args: Option<&Value>) {
    let kind = args.and_then(|value| value.get("kind")).and_then(Value::as_str).unwrap_or("");
    let pressed = args.and_then(|value| value.get("pressed")).and_then(Value::as_bool);
    let kinds = &mut ctx.scene.runtime.selectable_kinds;
    match kind {
        PUZZLE2D_GRANULARITY_NODE | "nodes" => kinds.nodes = pressed.unwrap_or(!kinds.nodes),
        PUZZLE2D_GRANULARITY_HANDLE | "handles" => kinds.handles = pressed.unwrap_or(!kinds.handles),
        PUZZLE2D_GRANULARITY_EDGE | "edges" => kinds.edges = pressed.unwrap_or(!kinds.edges),
        _ => return,
    }
    let kinds = *kinds;
    ctx.host.borrow_mut().focus.set_selection_options("rectangle", "replace", kinds.nodes, kinds.edges, kinds.handles);
    *ctx.ui_scope = puzzle2d_window_and_measures_scope();
}
