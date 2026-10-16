//! 🎛️ `shell-prefs` staging of `chrome-overlays-and-tour`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellChrome
//#region 🔖️ChromeOverlaysAndTour
//#region 🧯️TransientNoticeAndAgentOverlays
/// 🔒️ Frozen fault codes the dispatch funnel recognises — the exact strings the guest raises
/// (`🔌️plugin/🦀️.rs`'s `viewer.read-only`, and the mutation-outcomes contract's `app.command.rejected`),
/// which is also what React's `SURFACE_FAULT_CODES.ViewerReadOnly`/`COMMAND_REJECTED_FAULT_CODE`
/// match on.
pub const VIEWER_READ_ONLY_FAULT_CODE: &str = "viewer.read-only";

pub const COMMAND_REJECTED_FAULT_CODE: &str = "app.command.rejected";

/// 🧯️ Classifies one dispatch-fault string — and the structured guest refusal behind it, when there is one — into the banner
/// React would show for it: a read-only viewer is an `info` with its own frozen copy, a hub history refusal an `error` with
/// its localized copy (`time_travel::history_refusal_notice`), a guest refusal the framework or its app declared a notice for
/// (`kernel::fault_notice`: the kernel's history-lane refusals — an open or ended tool recording, a full or still replaying
/// history, `{n}` from `Fault.params` — as warnings, then the app's `faultNotices` with the fault's own severity), a rejected
/// mutation an `error` carrying the frozen code, any other guest refusal React's localized `dispatch-failed` notice (never its
/// raw code), and any other shell error its own text. Pure, so the mapping is testable without a live plugin bridge.
pub fn classify_dispatch_fault_notice(error: &str, refused: Option<&RefusedGuestFault>, terminology: Terminology, locale: Locale) -> (String, semio_framework::Severity, Option<String>) {
    use semio_framework::Severity;
    if error.contains(VIEWER_READ_ONLY_FAULT_CODE) {
        let text = LocalizedLabel::native("This is a read-only viewer — editing is disabled.", "Dies ist ein schreibgeschützter Betrachter – Bearbeiten ist deaktiviert.").resolve(terminology, locale).to_string();
        return (text, Severity::Info, Some(VIEWER_READ_ONLY_FAULT_CODE.to_string()));
    }
    if let Some((code, text, severity)) = time_travel::history_refusal_of_fault(error, locale) {
        return (text.to_string(), severity, Some(code.to_string()));
    }
    if let Some((refused, notice)) = refused.and_then(|refused| semio_framework::kernel::fault_notice(&refused.fault, &refused.notices, terminology, locale).map(|notice| (refused, notice))) {
        let severity = if semio_framework::kernel::history_notice(&notice.code).is_some() { Severity::Warning } else { refused.fault.severity };
        return (notice.text, severity, Some(notice.code));
    }
    if error.contains(COMMAND_REJECTED_FAULT_CODE) {
        let title = LocalizedLabel::native("Change rejected", "Änderung abgelehnt").resolve(terminology, locale).to_string();
        return (format!("{title}: {error}"), Severity::Error, Some(COMMAND_REJECTED_FAULT_CODE.to_string()));
    }
    if refused.is_some() {
        return (LocalizedLabel::native("The input could not be delivered.", "Die Eingabe konnte nicht zugestellt werden.").resolve(terminology, locale).to_string(), Severity::Info, None);
    }
    (error.to_string(), Severity::Error, None)
}

/// 📣️ The text a refused program call reads in the shell's language: the localized notice of the structured fault behind it
/// (`kernel::fault_notice` over the framework's tables — the host's guest admission `plugin.channel-mismatch` names both
/// channels), else the call's own text.
pub fn program_fault_text(refusal: &semio_framework_os_renderer_wgpu_program_bridge::ProgramFault, terminology: Terminology, locale: Locale) -> String {
    #[cfg(not(target_arch = "wasm32"))]
    if refusal.frame.is_some() { return LocalizedLabel::native("The input could not be delivered.", "Die Eingabe konnte nicht zugestellt werden.").resolve(terminology, locale).to_string(); }
    refusal.fault.as_ref().and_then(|fault| semio_framework::kernel::fault_notice(fault, &[], terminology, locale)).map_or_else(|| refusal.text.clone(), |notice| notice.text)
}

/// ✂️ Whether a dispatch-fault string names `code` as one whole token — the string the funnel classifies and the structured
/// refusal recorded beside it describe the same fault.
fn dispatch_fault_names_code(error: &str, code: &str) -> bool {
    error.split(|c: char| c.is_whitespace() || matches!(c, ':' | ';' | ',' | '[' | ']' | '(' | ')')).any(|token| token == code)
}

impl PrefsState {
    /// 🧯️ The dispatch-fault funnel: one failed `dispatch_action` becomes one banner, classified the
    /// way React's three call sites classify theirs. This replaces parking the string in
    /// `self.error`, which never cleared and sat in the bottom-left corner far from the gesture.
    pub fn note_dispatch_fault(&mut self, error: &str, chrome: &mut ChromeState) {
        let refused = chrome.chrome_build.refused_guest_fault.take().filter(|refused| dispatch_fault_names_code(error, &refused.fault.code.0));
        let (message, severity, code) = classify_dispatch_fault_notice(error, refused.as_ref(), self.active_terminology(), self.active_locale());
        if code.as_deref().is_some_and(time_travel::history_refusal_is_silent) {
            return;
        }
        chrome.show_transient_notice(message, severity, code.as_deref());
    }

    /// 📣️ Records one refused program call's structured guest fault beside the refusing app's notices for the funnel, and
    /// answers the `code: message` text every `String` dispatch channel carries.
    pub fn refused_guest_call(&mut self, refusal: semio_framework_os_renderer_wgpu_program_bridge::ProgramFault, app: &AppDefinition, chrome: &mut ChromeState) -> String {
        #[cfg(not(target_arch = "wasm32"))]
        { if chrome.chrome_build.refused_frame_fault.is_none() { chrome.chrome_build.refused_frame_fault = refusal.frame; }if chrome.chrome_build.refused_kernel_fault.is_none(){chrome.chrome_build.refused_kernel_fault=refusal.kernel;}if chrome.chrome_build.refused_actor_call.is_none(){chrome.chrome_build.refused_actor_call=refusal.actor_call;} }
        #[cfg(not(target_arch = "wasm32"))]
        let detail = if refusal.frame.is_some() { program_fault_text(&refusal, self.active_terminology(), self.active_locale()) } else { refusal.text };
        #[cfg(target_arch = "wasm32")]
        let detail = refusal.text;
        chrome.chrome_build.refused_guest_fault = refusal.fault.map(|fault| RefusedGuestFault { fault, notices: app.fault_notices.clone() });
        detail
    }

    /// 🤝️ Tells one refused instance open — the host's guest admission (`plugin.channel-mismatch`, both channels named) or any
    /// other structured refusal — through the dispatch-fault funnel, so it reaches the person as its localized notice and never
    /// as a raw code; answers the text the open's own status keeps ([`program_fault_text`]).
    pub fn note_refused_open(&mut self, refusal: semio_framework_os_renderer_wgpu_program_bridge::ProgramFault, chrome: &mut ChromeState) -> String {
        #[cfg(not(target_arch = "wasm32"))]
        { if chrome.chrome_build.refused_frame_fault.is_none() { chrome.chrome_build.refused_frame_fault = refusal.frame; }if chrome.chrome_build.refused_kernel_fault.is_none(){chrome.chrome_build.refused_kernel_fault=refusal.kernel;}if chrome.chrome_build.refused_actor_call.is_none(){chrome.chrome_build.refused_actor_call=refusal.actor_call;} }
        let detail = program_fault_text(&refusal, self.active_terminology(), self.active_locale());
        chrome.chrome_build.refused_guest_fault = refusal.fault.map(|fault| RefusedGuestFault { fault, notices: Vec::new() });
        #[cfg(not(target_arch = "wasm32"))]
        if refusal.frame.is_some() { self.note_dispatch_fault(&detail, chrome); return detail; }
        self.note_dispatch_fault(&refusal.text, chrome);
        detail
    }
}

//#endregion 🧯️TransientNoticeAndAgentOverlays
//#endregion 🔖️ChromeOverlaysAndTour
//#endregion ShellChrome

//#region ShellChrome
//#region 🔖️ChromeOverlaysAndTour
//#region 🧯️TransientNoticeAndAgentOverlays
/// 🛟️ The native input draft disposition of an exact typed history refusal.
pub fn dispatch_refusal_discards_input_draft(error: &str) -> bool {
    time_travel::history_refusal_discards_input_draft(error)
}
//#endregion 🧯️TransientNoticeAndAgentOverlays
//#endregion 🔖️ChromeOverlaysAndTour
//#endregion ShellChrome
