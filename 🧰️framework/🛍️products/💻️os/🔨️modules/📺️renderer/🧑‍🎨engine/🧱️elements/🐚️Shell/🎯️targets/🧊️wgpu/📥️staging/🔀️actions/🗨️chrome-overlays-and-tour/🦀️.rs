//! 🔀️ `shell-actions` staging of `chrome-overlays-and-tour`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellChrome
//#region 🔖️ChromeOverlaysAndTour
//#region 🧯️TransientNoticeAndAgentOverlays

/// ⚔️ The notice for one refused command batch, read from the typed rejection contract alone — React's
/// `commandRejectionNoticeV1`, pinned for both shells by `🛠️ShellHelpers/🧫️fixtures/🧫️command-rejection`. A local refusal
/// is told by its `code`; a hub refusal by the `MutationMessage` codes it was graded with: a refused history transition
/// names that refusal ([`time_travel::history_refusal_notice`]), else a concurrent structural constraint
/// (`mutation.invariant`) or region (`mutation.clamped`) is named after "Change refused by the hub", else only that.
/// The producer's English `reason` never reaches the human. Contract:
/// [`CommandRejectionV1`](../../../../../../🏪️store/🔄️sync/🧬️schema/🔣️command-rejection/🔣️.json).
pub fn command_rejection_notice(code: store_sync::sync::CommandRejectionCode, messages: &[store_sync::os_spr::MutationMessage], locale: Locale) -> (&'static str, String, semio_framework::Severity) {
    let de = locale == Locale::De;
    if let Some((code, key, severity)) = local_command_rejection_notice(code) {
        return (code, shell_chrome_string(key, de).to_string(), severity);
    }
    if let Some((code, text, severity)) = messages.iter().find_map(|message| time_travel::history_refusal_notice(&message.code.0, locale)) {
        return (code, text.to_string(), severity);
    }
    let graded = |wanted: &str| messages.iter().any(|message| message.code.0 == wanted);
    let reason = if graded("mutation.invariant") {
        Some("conflict.hubConcurrentInvariant")
    } else if graded("mutation.clamped") {
        Some("conflict.hubConcurrentEdit")
    } else {
        None
    };
    let rejected = shell_chrome_string("conflict.hubRejected", de);
    let text = reason.map_or_else(|| rejected.to_string(), |reason| format!("{rejected}: {}", shell_chrome_string(reason, de)));
    (SYNC_COMMAND_REJECTED_NOTICE_CODE, text, semio_framework::Severity::Warning)
}

//#endregion 🧯️TransientNoticeAndAgentOverlays
//#endregion 🔖️ChromeOverlaysAndTour
//#endregion ShellChrome

