//! 📂️ `shell-doc-open` staging of `space-admin-and-settle`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellTypes
/// 📤️ One parked `Effect::RequestFileOpen`, awaiting the flush that can open a real picker.
///
/// ⚖️ Parked rather than opened inline for the same reason an extension invocation is: opening a
/// picker is asynchronous (it ends on a user gesture) and `queue_host_effects` is deliberately a
/// plain `fn`. `drain_deferred_actions` owns `&mut self`, so it can await the pick AND dispatch the
/// import chunks it produces through the normal action path, folding each one's own effects back in.
#[cfg(target_arch = "wasm32")]
pub struct PendingFileOpen {
    pub controller_id: String,
    pub accept: String,
    pub read_as: Option<String>,
    pub import_action: String,
    pub args: Option<semio_framework::DslValue>,
    pub multiple: bool,
}

//#endregion ShellTypes

