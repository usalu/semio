//! 🖍 `shell-view-paint` staging of `chrome-overlays-and-tour`: RENDERER-WP5, the part of the former chunk that plan Appendix B and C, the WP4 re-homing by the call graph and the WP5 size spill assign to this crate.

use super::*;

//#region ShellChrome
//#region 🔖️ChromeOverlaysAndTour
/// ⏱️ Hover-to-tooltip delay — the React chrome's own `CHROME_CONTROL_TOOLTIP_DELAY_MS`
/// (`💡️ChromeControlHint/🟦️.tsx:20`). It was 500 ms here, which made every wgpu tooltip feel a fifth
/// of a second slower than the same control under React; WGPU-RENDERER-REACT-PARITY packet W1j
/// aligns the number, since the surrounding arm/dismiss behaviour was already identical.
pub(crate) const CHROME_TOOLTIP_DELAY_MS: f64 = 400.0;
//#endregion 🔖️ChromeOverlaysAndTour
//#endregion ShellChrome

//#region ShellChrome
//#region 🔖️ChromeOverlaysAndTour
//#region 🧯️TransientNoticeAndAgentOverlays
/// 📐️ `top-workbench` — React pins the banner `navbar_height + 8px` from the top, horizontally
/// centred (`left-1/2 -translate-x-1/2`).
pub(crate) const TRANSIENT_NOTICE_TOP_GAP: f32 = 8.0;

/// 📐️ Width cap before the message wraps to the banner's own width — the banner is `px-double
/// py-single` around one line of `text-sm` on the React side.
pub(super) const TRANSIENT_NOTICE_MAX_WIDTH: f32 = 520.0;

/// 🆔️ The banner's own dismiss control.
pub(crate) const TRANSIENT_NOTICE_CLOSE_CONTROL_ID: &str = "shell.notice.close";

/// 🎨️ `(border, fill, text)` per severity — the themed counterpart of
/// `TRANSIENT_NOTICE_TONE_CLASS`: `info` takes the neutral popover chrome, `warning` the amber
/// tint, `error`/`fatal` the destructive one. Every channel comes from a `Theme` token, never a
/// literal, so the banner re-tints with the active theme like the rest of the chrome.
pub(crate) fn transient_notice_tone(severity: semio_framework::Severity, theme: &Theme) -> (Rgba, Rgba, Rgba) {
    use semio_framework::Severity;
    match severity {
        Severity::Info => (theme.border_normal, theme.level_bg[Level::Menu.index()], theme.text),
        Severity::Warning => (theme.warning, theme.warning.with_alpha(0.1), theme.warning),
        Severity::Error | Severity::Fatal => (theme.error, theme.error, theme.active_foreground),
    }
}

/// 📐️ The close control inside the banner — right-aligned, the `ml-single underline` button React
/// renders after the message.
pub(crate) fn transient_notice_close_rect(banner: Rect, close_label: &str, theme: &Theme) -> Rect {
    let label_w = (close_label.chars().count() as f32 * theme.font_size_small * 0.6).max(theme.font_size_small);
    Rect::new(banner.x + banner.w - theme.padding_standard - label_w, banner.y + theme.padding_standard, label_w, banner.h - theme.padding_standard * 2.0)
}
//#endregion 🧯️TransientNoticeAndAgentOverlays
//#endregion 🔖️ChromeOverlaysAndTour
//#endregion ShellChrome

//#region ShellChrome
//#region 🔖️ChromeOverlaysAndTour
//#region 🧯️TransientNoticeAndAgentOverlays
//#region 🎬️PluginInstallBand
/// 🆔️ The install band's own cancel control — `cancel_plugin_install`, not a dismiss: the phase is
/// what the band shows, so hiding it without stopping the install would lie about what is running.
pub(crate) const PLUGIN_INSTALL_CANCEL_CONTROL_ID: &str = "shell.plugin-install.cancel";

/// 📐️ The band sits directly under the transient notice's own slot, so an install that raises a
/// notice shows both rather than one covering the other.
pub(crate) const PLUGIN_INSTALL_TOP_GAP: f32 = 8.0;

/// 🗣️ The band's message for one phase, English first and German through the same
/// [`shell_chrome_string`] table the rest of this chrome reads. `Failed` carries its own reason —
/// that string is the install's, already a sentence, so it is appended rather than replaced.
pub(crate) fn plugin_install_banner_text(install: &ShellPluginInstall, is_de: bool) -> String {
    let key = match install.phase {
        ShellPluginInstallPhase::Resolving => "plugin.install.resolving",
        ShellPluginInstallPhase::Loading => "plugin.install.loading",
        ShellPluginInstallPhase::Cancelled => "plugin.install.cancelled",
        ShellPluginInstallPhase::Failed(_) => "plugin.install.failed",
    };
    let head = format!("{} {}", shell_chrome_string(key, is_de), install.plugin_id);
    match &install.phase {
        ShellPluginInstallPhase::Failed(reason) => format!("{head}: {reason}"),
        _ => head,
    }
}

/// 📐️ The band's own right-aligned action control — cancel while it runs, dismiss once it settled.
pub(crate) fn plugin_install_action_rect(band: Rect, action_label: &str, theme: &Theme) -> Rect {
    let label_w = (action_label.chars().count() as f32 * theme.font_size_small * 0.6).max(theme.font_size_small);
    Rect::new(band.x + band.w - theme.padding_standard - label_w, band.y + theme.padding_standard, label_w, band.h - theme.padding_standard * 2.0)
}
//#endregion 🎬️PluginInstallBand
//#endregion 🧯️TransientNoticeAndAgentOverlays
//#endregion 🔖️ChromeOverlaysAndTour
//#endregion ShellChrome

//#region ShellChrome
//#region 🔖️ChromeOverlaysAndTour
//#region 🧯️TransientNoticeAndAgentOverlays
//#region 🚪️DocumentOpenBand
/// 🆔️ The open band's own control: a real cancel while a step is out
/// ([`ShellState::cancel_document_opening`]), a close once the open settled.
pub(crate) const DOCUMENT_OPEN_CANCEL_CONTROL_ID: &str = "shell.document-open.cancel";

/// 🔢️ The steps an open takes (a hub document's component, the app instance, then the document), for the
/// band's `step/total`. An open with no hub document to resolve starts at step 2.
pub(super) const DOCUMENT_OPEN_STEPS: u8 = 3;
//#endregion 🚪️DocumentOpenBand
//#endregion 🧯️TransientNoticeAndAgentOverlays
//#endregion 🔖️ChromeOverlaysAndTour
//#endregion ShellChrome
