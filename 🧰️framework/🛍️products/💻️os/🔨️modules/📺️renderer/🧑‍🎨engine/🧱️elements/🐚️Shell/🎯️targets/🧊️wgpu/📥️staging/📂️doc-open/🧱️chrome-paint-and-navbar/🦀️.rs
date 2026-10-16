//! 📂️ `shell-doc-open` staging of `chrome-paint-and-navbar`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region 📌️HubCheckIn
#[cfg(not(target_arch = "wasm32"))]
pub const HUB_CHECK_IN_POLL_MS: u64 = 250;

#[cfg(not(target_arch = "wasm32"))]
pub fn hub_check_in_local_status(
    request_id: &str,
    phase: semio_framework_os_kernel::os_directory::DocumentCheckInPhaseV1,
    refusal: Option<semio_framework_os_kernel::os_directory::DocumentCheckInRefusalV1>,
) -> semio_framework_os_kernel::os_directory::DocumentCheckInStatusV1 {
    semio_framework_os_kernel::os_directory::DocumentCheckInStatusV1 {
        schema: semio_framework_os_kernel::os_directory::DOCUMENT_CHECK_IN_STATUS_SCHEMA_V1.to_string(),
        request_id: request_id.to_string(),
        phase,
        progress: semio_framework_os_kernel::os_directory::DocumentCheckInProgressV1 { completed_units: 0, total_units: 8 },
        ready: None,
        refusal,
    }
}

//#endregion 📌️HubCheckIn

