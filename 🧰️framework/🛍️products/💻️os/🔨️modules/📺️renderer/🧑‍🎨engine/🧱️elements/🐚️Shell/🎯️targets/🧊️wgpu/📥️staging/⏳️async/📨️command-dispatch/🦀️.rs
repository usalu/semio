//! ⏳️ `shell-async` staging of `command-dispatch`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellActions

//#endregion ShellActions

#[cfg(target_arch = "wasm32")]
pub fn directory_now_ms() -> u64 {
    let millis = chrome_now_ms();
    if millis.is_finite() && millis > 0.0 {
        millis as u64
    } else {
        0
    }
}

/// ⏱️ The clock a directory deadline is measured against — necessarily the SAME one the linked
/// transport compares it to. Native hands the shared `WorkerPool`'s own monotonic millis (what
/// `NativeDirectoryTransport`'s `HttpPool` reads); the browser door has no pool and the page
/// answers every hop with `fetch`'s own timing, so `Date.now()` is the only shared reading.
#[cfg(not(target_arch = "wasm32"))]
pub fn directory_now_ms() -> u64 {
    semio_framework_os_renderer_wgpu_host_base::renderer_worker_pool().now_ms()
}
