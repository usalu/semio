//! 🔲️ `shell-windows` staging of `pointer-input`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellInput
/// 📨️ `hostEvent{windowId, kind}` on `controller_id`: the window fact the program answers by ending an open gesture there —
/// the wgpu twin of React's `windowHostEventHandlersV1` (a pane that loses the activation is blurred, a cancelled pointer
/// sequence loses its capture).
pub(crate) fn window_host_event_action(controller_id: &str, window_id: &str, kind: &str) -> ActionDescriptor {
    ActionDescriptor { controller_id: controller_id.to_string(), action: semio_framework::HOST_EVENT_ACTION_ID.into(), args: crate::action_args_json!({ "windowId": window_id, "kind": kind }) }
}

//#endregion ShellInput

