//! 🧲️ `shell-plugins` staging of `space-admin-and-settle`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellTypes
//#region 🧯️BootProgramSelection
/// 🧯 One plugin whose boot activation faulted, kept as data so the shell can show WHICH plugin is
/// unavailable instead of dying with the plugin. `detail` is the guest-side cause verbatim (a
/// first-step trap message, a missing module, …) — never re-worded, so a browser status line and a
/// console trace read the same.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ShellPluginFault {
    pub plugin_id: String,
    pub app_id: String,
    pub detail: String,
    #[cfg(not(target_arch = "wasm32"))]
    pub frame: Option<semio_framework_actor::pack::PackError>,
    #[cfg(not(target_arch="wasm32"))]
    pub kernel:Option<semio_framework_actor::KernelError>,
    #[cfg(not(target_arch="wasm32"))]
    pub actor_call:Option<semio_framework_plugin_host::original_actor_context::OriginalActorFixedCall>,
}

/// 🧯 One SURFACE whose `render_with_document` failed, kept as data for exactly the reason
/// [`ShellPluginFault`] is: a surface that cannot be read is a per-surface fault, never a boot failure.
/// Propagating it with `?` made `renderDocument promise failed: wgpu-ui.intake-budget-exhausted` become
/// `shell-boot: …`, then `worker-boot-failed`, which closed the Worker and took its transferred
/// `OffscreenCanvas` with it — one unreadable window body killed the whole renderer. `detail` is the
/// cause verbatim, so the fault card and the console trace read the same.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ShellSurfaceFault {
    pub surface_id: String,
    pub body_key: String,
    pub detail: String,
    #[cfg(not(target_arch="wasm32"))]
    pub frame:Option<semio_framework_actor::pack::PackError>,
    #[cfg(not(target_arch="wasm32"))]
    pub kernel:Option<semio_framework_actor::KernelError>,
    #[cfg(not(target_arch="wasm32"))]
    pub actor_call:Option<semio_framework_plugin_host::original_actor_context::OriginalActorFixedCall>,
}

//#endregion 🧯️BootProgramSelection
/// 📥️ One parked `Effect::InvokeExtension`, awaiting the flush that can publish its answer.
#[cfg(target_arch = "wasm32")]
pub(crate) struct PendingExtensionInvocation {
    pub(crate) req: u64,
    pub(crate) extension_id: String,
    pub(crate) capability: String,
    pub(crate) request_json: String,
}

//#endregion ShellTypes

