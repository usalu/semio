//! 🪝️ `shell-window-actions` staging of `dock-model-and-measures`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellLifecycle
//#region 📏️WindowMeasures
/// 🩺️ One refresh pass's scope, short enough to read in a console line — which surfaces a settle
/// asked for is the only thing that separates "the guest re-rendered the previous document" from
/// "this pass never looked at that surface".
pub fn refresh_scope_label(scope: &UiDirtyScope) -> String {
    match scope {
        UiDirtyScope::Full => "full".to_string(),
        UiDirtyScope::None => "none".to_string(),
        UiDirtyScope::Partial { window_bodies, panel_bodies, utilities, tools, engagements, measures, labels } => {
            format!("partial windows=[{}] panels=[{}] utilities={utilities} tools={tools} engagements={engagements} measures={measures} labels={labels}", window_bodies.join(","), panel_bodies.join(","))
        }
    }
}

//#endregion 📏️WindowMeasures
//#endregion ShellLifecycle

//#region ShellLifecycle
//#region 📏️WindowMeasures
/// 🏁️ The refresh one guest publication owes — the wgpu half of the shared law
/// `🛠️ShellHelpers/🧫️fixtures/🧫️operation-publication/🔣️.json` (React: `typedOperationCompletionRefreshV1` and the
/// progress subscriber). Its own scope when it has one; a COMPLETION that patched history and dirtied nothing else still
/// owes the reserved History body the row it minted; progress with a patch alone moves the band and re-renders nothing;
/// a lossy lane (`resync`) owes everything.
pub fn operation_publication_refresh(publication: &semio_framework_os_renderer_wgpu_program_bridge::OperationPublication) -> UiDirtyScope {
    if publication.resync {
        return UiDirtyScope::Full;
    }
    if !publication.ui_scope.asks_for_nothing() {
        return publication.ui_scope.clone();
    }
    if publication.completed && publication.history_patch.is_some() {
        return UiDirtyScope::Partial { window_bodies: Vec::new(), panel_bodies: vec![ui_wgpu::wgpu::FRAMEWORK_HISTORY_BODY_KEY.to_string()], utilities: false, tools: false, engagements: false, measures: false, labels: false };
    }
    UiDirtyScope::None
}
//#endregion 📏️WindowMeasures
//#endregion ShellLifecycle
