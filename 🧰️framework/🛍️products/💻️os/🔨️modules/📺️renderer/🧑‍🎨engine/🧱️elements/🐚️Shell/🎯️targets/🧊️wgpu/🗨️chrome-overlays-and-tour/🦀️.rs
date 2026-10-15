//! 🗨️ Chrome overlays: notices, bands, agent approvals, dialog requests and the tour.
//!
//! Tooltips, transient notices, plugin-install and document-open bands, dispatch fault notices, agent approval
//! paint ops, chrome dialog requests and helpers, the chrome tour state and element rects.
//!
//! One chunk of the former `shell` monolith (ticket CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT); [`ShellState`] and the
//! shared imports stay in the parent module and are reached through `use super::*;`.

use super::*;

//#region ShellChrome
//#region 🔖️ChromeOverlaysAndTour
// 🍿️ w3-overlays-chrome-polish (WP15+WP16): tooltips, a generic modal dialog, and the app introduction
// tour — layered on the existing immediate-mode chrome through `ShellChromeBuildState`, while
// `OverlayState` remains dedicated to the pre-existing menu/search overlays.
// Placement math reuses `ui_wgpu`'s w1d-events-overlay manager types (`OverlayKind`, `OverlayPlacement`,
// `resolve_overlay_placement`) even though the manager's own `EventRouter`/`open_overlay` stay
// `pub(crate)` to `ui_wgpu` (an `engine::Ui`/retained-`UiTree` implementation detail) and out of reach
// for this non-tree-based immediate-mode chrome renderer — confirmed unreachable the same way
// `report-w2-text-editor.md` found for its own local-fallback popup.

/// ⏱️ The hover-armed tooltip candidate — (re)armed the first frame `control_id` becomes hovered,
/// painted once `CHROME_TOOLTIP_DELAY_MS` elapses. Dismissed immediately on hover-out: this crate has no
/// animation-clock scaffolding anywhere (`engine::Ui::needs_frame`'s own doc comment admits the same gap
/// for animations generally), matching `OverlayKind::Tooltip::dismiss_policy`'s own documented admission
/// that hover-out-delay isn't actually debounced yet.
#[derive(Clone, Debug)]
pub(crate) struct ChromeTooltipHover {
    pub(crate) control_id: String,
    pub(crate) anchor_x: f32,
    pub(crate) anchor_y: f32,
    pub(crate) started_ms: f64,
}

/// ⏱️ Hover-to-tooltip delay — the React chrome's own `CHROME_CONTROL_TOOLTIP_DELAY_MS`
/// (`💡️ChromeControlHint/🟦️.tsx:20`). It was 500 ms here, which made every wgpu tooltip feel a fifth
/// of a second slower than the same control under React; WGPU-RENDERER-REACT-PARITY packet W1j
/// aligns the number, since the surrounding arm/dismiss behaviour was already identical.
pub(crate) const CHROME_TOOLTIP_DELAY_MS: f64 = 400.0;

/// ⏱️ Pure delay-threshold check, factored out of `render_chrome_tooltip` so it's unit-testable without
/// a `DrawList`/`FontAtlas` fixture.
pub(crate) fn chrome_tooltip_ready(hover: &ChromeTooltipHover, now_ms: f64) -> bool {
    now_ms - hover.started_ms >= CHROME_TOOLTIP_DELAY_MS
}

//#region 🧯️TransientNoticeAndAgentOverlays
// 🧯️ WGPU-RENDERER-REACT-PARITY packet W1j. React's `showTransientNotice` (`🏛️ShellHost/🟦️.tsx`) is
// the ONE channel every non-fatal shell fault reaches the user through — viewer-read-only,
// mutation-rejected, render-error, keybinding-unowned, a guest `Effect::Notify`, a degraded remote
// merge. This shell had no equivalent at all: such faults either set the persistent bottom-left
// `self.error` line (which never clears) or vanished. The three rules below are React's, exactly:
// ONE notice at a time (a new one replaces rather than queues), auto-dismiss after 4 s, and an
// explicit close control.

/// ⏱️ `setTimeout(…, 4000)` in `showTransientNotice` — the auto-dismiss deadline.
pub(crate) const TRANSIENT_NOTICE_AUTO_DISMISS_MS: f64 = 4000.0;

/// 📐️ `top-workbench` — React pins the banner `navbar_height + 8px` from the top, horizontally
/// centred (`left-1/2 -translate-x-1/2`).
pub(crate) const TRANSIENT_NOTICE_TOP_GAP: f32 = 8.0;

/// 📐️ Width cap before the message wraps to the banner's own width — the banner is `px-double
/// py-single` around one line of `text-sm` on the React side.
const TRANSIENT_NOTICE_MAX_WIDTH: f32 = 520.0;

/// 🆔️ The banner's own dismiss control.
pub(crate) const TRANSIENT_NOTICE_CLOSE_CONTROL_ID: &str = "shell.notice.close";

/// 🆔️ The transient notice's own polite status node — the wgpu twin of React's `[data-semio-transient-notice]`.
pub(crate) const TRANSIENT_NOTICE_STATUS_ID: &str = "shell.notice";

/// 🧯️ The single live notice. `severity` is `semio_framework::Severity` — the same frozen four-level
/// vocabulary React's `TransientNotice.kind` carries, not a renderer-local enum — and `code` is the
/// frozen `Fault.code` (`viewer.read-only`, `app.command.rejected`, …) a probe asserts on, mirroring
/// React's `data-notice-code` attribute.
#[derive(Clone, Debug, PartialEq)]
pub struct ShellTransientNotice {
    pub message: String,
    pub severity: semio_framework::Severity,
    pub code: Option<String>,
    pub shown_at_ms: f64,
}

/// ⏱️ Pure deadline check, factored out so the 4 s rule is testable without a frame clock.
pub(crate) fn transient_notice_expired(notice: &ShellTransientNotice, now_ms: f64) -> bool {
    now_ms - notice.shown_at_ms >= TRANSIENT_NOTICE_AUTO_DISMISS_MS
}

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

/// 📐️ The banner's rect: top-centre, `navbar_height + 8px` down, sized to one line of the message
/// plus the close control. The width estimate is the same monospace approximation
/// `render_chrome_tooltip` already uses — this renderer measures glyphs only while painting them.
pub(crate) fn transient_notice_rect(notice: &ShellTransientNotice, close_label: &str, width: f32, theme: &Theme) -> Rect {
    let glyphs = notice.message.chars().count() + close_label.chars().count() + 2;
    let text_w = glyphs as f32 * theme.font_size_small * 0.6;
    let banner_w = (text_w + theme.padding_standard * 4.0).min(TRANSIENT_NOTICE_MAX_WIDTH).min((width - theme.padding_standard * 2.0).max(1.0));
    let banner_h = theme.font_size_small * 1.6 + theme.padding_standard * 2.0;
    Rect::new(((width - banner_w) * 0.5).max(0.0), theme.navbar_height + TRANSIENT_NOTICE_TOP_GAP, banner_w, banner_h)
}

/// 📐️ The close control inside the banner — right-aligned, the `ml-single underline` button React
/// renders after the message.
pub(crate) fn transient_notice_close_rect(banner: Rect, close_label: &str, theme: &Theme) -> Rect {
    let label_w = (close_label.chars().count() as f32 * theme.font_size_small * 0.6).max(theme.font_size_small);
    Rect::new(banner.x + banner.w - theme.padding_standard - label_w, banner.y + theme.padding_standard, label_w, banner.h - theme.padding_standard * 2.0)
}

impl ShellChromeBuildState {
    /// 🧯️ Shows a non-blocking, auto-dismissing notice, replacing whatever is currently showing —
    /// `showTransientNotice`'s own contract ("only one can render at a time").
    pub(crate) fn show_transient_notice(&mut self, message: impl Into<String>, severity: semio_framework::Severity, code: Option<&str>, now_ms: f64) {
        self.transient_notice = Some(ShellTransientNotice { message: message.into(), severity, code: code.map(str::to_string), shown_at_ms: now_ms });
    }

    pub(crate) fn dismiss_transient_notice(&mut self) {
        self.transient_notice = None;
    }

    /// ⏱️ Clears the notice once its 4 s deadline passed; returns `true` when one is still showing.
    pub(crate) fn expire_transient_notice(&mut self, now_ms: f64) -> bool {
        if self.transient_notice.as_ref().is_some_and(|notice| transient_notice_expired(notice, now_ms)) {
            self.transient_notice = None;
        }
        self.transient_notice.is_some()
    }

    /// ⏰️ Earliest wall-clock deadline that can change Shell-owned chrome without input.
    fn next_deadline_ms(&self, now_ms: f64) -> Option<f64> {
        let tooltip = self.tooltip_hover.as_ref().filter(|hover| !chrome_tooltip_ready(hover, now_ms)).map(|hover| hover.started_ms + CHROME_TOOLTIP_DELAY_MS);
        let notice = self.transient_notice.as_ref().map(|notice| notice.shown_at_ms + TRANSIENT_NOTICE_AUTO_DISMISS_MS);
        match (tooltip, notice) {
            (Some(tooltip), Some(notice)) => Some(tooltip.min(notice)),
            (Some(deadline), None) | (None, Some(deadline)) => Some(deadline),
            (None, None) => None,
        }
    }
}

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

/// 🎨️ `(border, fill, text)` for one phase, reusing [`transient_notice_tone`]'s own severity map so
/// the band tints exactly like every other shell banner: a running install is neutral, a cancelled
/// one a warning, a failed one destructive.
pub(crate) fn plugin_install_tone(phase: &ShellPluginInstallPhase, theme: &Theme) -> (Rgba, Rgba, Rgba) {
    match phase {
        ShellPluginInstallPhase::Resolving | ShellPluginInstallPhase::Loading => transient_notice_tone(semio_framework::Severity::Info, theme),
        ShellPluginInstallPhase::Cancelled => transient_notice_tone(semio_framework::Severity::Warning, theme),
        ShellPluginInstallPhase::Failed(_) => transient_notice_tone(semio_framework::Severity::Error, theme),
    }
}

/// 📐️ The band's rect — same top-centre geometry as the notice, one band height further down.
pub(crate) fn plugin_install_rect(message: &str, action_label: &str, width: f32, theme: &Theme) -> Rect {
    let glyphs = message.chars().count() + action_label.chars().count() + 2;
    let text_w = glyphs as f32 * theme.font_size_small * 0.6;
    let band_w = (text_w + theme.padding_standard * 4.0).min(TRANSIENT_NOTICE_MAX_WIDTH).min((width - theme.padding_standard * 2.0).max(1.0));
    let band_h = theme.font_size_small * 1.6 + theme.padding_standard * 2.0;
    Rect::new(((width - band_w) * 0.5).max(0.0), theme.navbar_height + TRANSIENT_NOTICE_TOP_GAP + band_h + PLUGIN_INSTALL_TOP_GAP, band_w, band_h)
}

/// 📐️ The band's own right-aligned action control — cancel while it runs, dismiss once it settled.
pub(crate) fn plugin_install_action_rect(band: Rect, action_label: &str, theme: &Theme) -> Rect {
    let label_w = (action_label.chars().count() as f32 * theme.font_size_small * 0.6).max(theme.font_size_small);
    Rect::new(band.x + band.w - theme.padding_standard - label_w, band.y + theme.padding_standard, label_w, band.h - theme.padding_standard * 2.0)
}

//#endregion 🎬️PluginInstallBand

//#region 🚪️DocumentOpenBand
/// 🆔️ The open band's own control: a real cancel while a step is out
/// ([`ShellState::cancel_document_opening`]), a close once the open settled.
pub(crate) const DOCUMENT_OPEN_CANCEL_CONTROL_ID: &str = "shell.document-open.cancel";

/// 🔢️ The steps an open takes (a hub document's component, the app instance, then the document), for the
/// band's `step/total`. An open with no hub document to resolve starts at step 2.
const DOCUMENT_OPEN_STEPS: u8 = 3;

/// 🗣️ The band's message: phase, which app, step and elapsed seconds while it runs; a failure appends
/// its own reason. English first, German through [`shell_chrome_string`].
pub(crate) fn document_opening_banner_text(opening: &ShellDocumentOpening, now_ms: f64, is_de: bool) -> String {
    let (key, step) = match opening.phase {
        ShellDocumentOpenPhase::Resolving => ("document.open.resolving", Some(1)),
        ShellDocumentOpenPhase::Instantiating => ("document.open.instantiating", Some(2)),
        ShellDocumentOpenPhase::Seeding => ("document.open.seeding", Some(DOCUMENT_OPEN_STEPS)),
        ShellDocumentOpenPhase::Cancelled => ("document.open.cancelled", None),
        ShellDocumentOpenPhase::Failed(_) => ("document.open.failed", None),
    };
    let head = match opening.phase {
        ShellDocumentOpenPhase::Resolving => {
            let detail =
                ["document.open.resolving.lease", "document.open.resolving.component", "document.open.resolving.descriptor", "document.open.resolving.verified"][usize::from(opening.resolve_step.load(std::sync::atomic::Ordering::Acquire).min(3))];
            format!("{} {} ({})", shell_chrome_string(key, is_de), opening.label, shell_chrome_string(detail, is_de))
        }
        _ => format!("{} {}", shell_chrome_string(key, is_de), opening.label),
    };
    match (&opening.phase, step) {
        (ShellDocumentOpenPhase::Failed(reason), _) => format!("{head}: {reason}"),
        (_, Some(step)) => format!("{head} · {step}/{DOCUMENT_OPEN_STEPS} · {} s", ((now_ms - opening.started_at_ms).max(0.0) / 1000.0).floor() as u64),
        _ => head,
    }
}

/// 🎨️ `(border, fill, text)` for one phase, the same severity map every shell banner uses.
pub(crate) fn document_opening_tone(phase: &ShellDocumentOpenPhase, theme: &Theme) -> (Rgba, Rgba, Rgba) {
    match phase {
        ShellDocumentOpenPhase::Resolving | ShellDocumentOpenPhase::Instantiating | ShellDocumentOpenPhase::Seeding => transient_notice_tone(semio_framework::Severity::Info, theme),
        ShellDocumentOpenPhase::Cancelled => transient_notice_tone(semio_framework::Severity::Warning, theme),
        ShellDocumentOpenPhase::Failed(_) => transient_notice_tone(semio_framework::Severity::Error, theme),
    }
}

/// 📐️ The band's rect: the plugin-install band's geometry, one band height and gap further down, so
/// an open that installs its plugin shows both.
pub(crate) fn document_opening_rect(message: &str, action_label: &str, width: f32, theme: &Theme) -> Rect {
    let band = plugin_install_rect(message, action_label, width, theme);
    Rect::new(band.x, band.y + band.h + PLUGIN_INSTALL_TOP_GAP, band.w, band.h)
}
//#endregion 🚪️DocumentOpenBand

/// 🔒️ Frozen fault codes the dispatch funnel recognises — the exact strings the guest raises
/// (`🔌️plugin/🦀️.rs`'s `viewer.read-only`, and the mutation-outcomes contract's `app.command.rejected`),
/// which is also what React's `SURFACE_FAULT_CODES.ViewerReadOnly`/`COMMAND_REJECTED_FAULT_CODE`
/// match on.
pub(crate) const VIEWER_READ_ONLY_FAULT_CODE: &str = "viewer.read-only";
pub(crate) const COMMAND_REJECTED_FAULT_CODE: &str = "app.command.rejected";

/// 📜️ The suffix of a retained panel's whole-content scroll region id (`framework.panel.history.scroll`).
pub(crate) const RETAINED_PANEL_SCROLL_SUFFIX: &str = ".scroll";

/// 📣️ The structured guest refusal behind one dispatch-fault string (design §20.12): the guest's own `Fault` — its code,
/// severity and the `params` a notice fills its placeholders from — and the refusing app's published notices. Recorded where a
/// program call refuses ([`ShellState::refused_guest_call`]), taken by the funnel ([`ShellState::note_dispatch_fault`]) when
/// the string it classifies names the same code, cleared when the next dispatch starts.
#[derive(Clone, Debug)]
pub(crate) struct RefusedGuestFault {
    pub(crate) fault: semio_framework::Fault,
    pub(crate) notices: Vec<semio_framework::FaultNoticeDefinition>,
}

/// 🧯️ Classifies one dispatch-fault string — and the structured guest refusal behind it, when there is one — into the banner
/// React would show for it: a read-only viewer is an `info` with its own frozen copy, a hub history refusal an `error` with
/// its localized copy (`time_travel::history_refusal_notice`), a guest refusal the framework or its app declared a notice for
/// (`kernel::fault_notice`: the kernel's history-lane refusals — an open or ended tool recording, a full or still replaying
/// history, `{n}` from `Fault.params` — as warnings, then the app's `faultNotices` with the fault's own severity), a rejected
/// mutation an `error` carrying the frozen code, any other guest refusal React's localized `dispatch-failed` notice (never its
/// raw code), and any other shell error its own text. Pure, so the mapping is testable without a live plugin bridge.
pub(crate) fn classify_dispatch_fault_notice(error: &str, refused: Option<&RefusedGuestFault>, terminology: Terminology, locale: Locale) -> (String, semio_framework::Severity, Option<String>) {
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
pub(crate) fn program_fault_text(refusal: &crate::program_bridge::ProgramFault, terminology: Terminology, locale: Locale) -> String {
    #[cfg(not(target_arch = "wasm32"))]
    if refusal.frame.is_some() { return LocalizedLabel::native("The input could not be delivered.", "Die Eingabe konnte nicht zugestellt werden.").resolve(terminology, locale).to_string(); }
    refusal.fault.as_ref().and_then(|fault| semio_framework::kernel::fault_notice(fault, &[], terminology, locale)).map_or_else(|| refusal.text.clone(), |notice| notice.text)
}

/// 🛟️ The native input draft disposition of an exact typed history refusal.
pub(crate) fn dispatch_refusal_discards_input_draft(error: &str) -> bool {
    time_travel::history_refusal_discards_input_draft(error)
}

/// ✂️ Whether a dispatch-fault string names `code` as one whole token — the string the funnel classifies and the structured
/// refusal recorded beside it describe the same fault.
fn dispatch_fault_names_code(error: &str, code: &str) -> bool {
    error.split(|c: char| c.is_whitespace() || matches!(c, ':' | ';' | ',' | '[' | ']' | '(' | ')')).any(|token| token == code)
}

/// ⚔️ The notice code a hub refusal naming no history transition is told under — React's `sync.command.rejected`.
const SYNC_COMMAND_REJECTED_NOTICE_CODE: &str = "sync.command.rejected";

/// 🔀️ The notice code of a batch the hub applied in adjusted form — React's `sync.command.transformed`.
pub(crate) const SYNC_COMMAND_TRANSFORMED_NOTICE_CODE: &str = "sync.command.transformed";

/// 🚫️ The notice code, chrome line and severity of each local refusal — React's `LOCAL_COMMAND_REJECTION_NOTICES_V1`: one
/// the human can wait out or cannot act on here is a warning, one meaning this device produced a change it cannot send is
/// an error. `None` for the hub's own two codes.
fn local_command_rejection_notice(code: store_sync::sync::CommandRejectionCode) -> Option<(&'static str, &'static str, semio_framework::Severity)> {
    use semio_framework::Severity;
    use store_sync::sync::CommandRejectionCode as Code;
    Some(match code {
        Code::HubRefused | Code::HubUnreadable => return None,
        Code::LocalReadOnly => ("local.read-only", "conflict.local.readOnly", Severity::Warning),
        Code::LocalQueueFull => ("local.queue-full", "conflict.local.queueFull", Severity::Warning),
        Code::LocalBackboneCapacity => ("local.backbone-capacity", "conflict.local.queueFull", Severity::Warning),
        Code::LocalBackboneDuplicate => ("local.backbone-duplicate", "conflict.local.duplicate", Severity::Warning),
        Code::LocalBackbonePairUnavailable => ("local.backbone-pair-unavailable", "conflict.local.notReady", Severity::Warning),
        Code::LocalBackboneScopeMismatch => ("local.backbone-scope-mismatch", "conflict.local.foreignDocument", Severity::Error),
        Code::LocalBackboneMalformed => ("local.backbone-malformed", "conflict.local.unreadable", Severity::Error),
        Code::LocalSocketFrameCeiling => ("local.socket-frame-ceiling", "conflict.local.tooLarge", Severity::Error),
    })
}

/// ⚔️ The notice for one refused command batch, read from the typed rejection contract alone — React's
/// `commandRejectionNoticeV1`, pinned for both shells by `🛠️ShellHelpers/🧫️fixtures/🧫️command-rejection`. A local refusal
/// is told by its `code`; a hub refusal by the `MutationMessage` codes it was graded with: a refused history transition
/// names that refusal ([`time_travel::history_refusal_notice`]), else a concurrent structural constraint
/// (`mutation.invariant`) or region (`mutation.clamped`) is named after "Change refused by the hub", else only that.
/// The producer's English `reason` never reaches the human. Contract:
/// [`CommandRejectionV1`](../../../../../../🏪️store/🔄️sync/🧬️schema/🔣️command-rejection/🔣️.json).
pub(crate) fn command_rejection_notice(code: store_sync::sync::CommandRejectionCode, messages: &[store_sync::os_spr::MutationMessage], locale: Locale) -> (&'static str, String, semio_framework::Severity) {
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

/// ✅️ One step of the approvals modal's flat paint program — the retained chrome step walks this
/// list by index, so a glyph run that needs another opportunity simply does not advance the index.
pub(crate) enum AgentApprovalPaintOp {
    Scrim,
    Modal(Rect),
    Fill { rect: Rect, color: Rgba },
    Text { value: String, x: f32, y: f32, max_w: f32, size: f32, color: Rgba },
    Hit { rect: Rect, control_id: String },
    Clicks,
}

impl ShellState {
    /// 🧯️ The shell-wide entry point every non-fatal fault path calls — the wgpu twin of React's
    /// `showTransientNoticeRef.current(...)`.
    pub fn show_transient_notice(&mut self, message: impl Into<String>, severity: semio_framework::Severity, code: Option<&str>) {
        let now = chrome_now_ms();
        self.chrome_build.show_transient_notice(message, severity, code, now);
    }

    /// 🧯️ Read side for probes and tests — the live notice, if any.
    pub fn transient_notice(&self) -> Option<&ShellTransientNotice> {
        self.chrome_build.transient_notice.as_ref()
    }

    /// ⏰️ Translates the next epoch-based Shell chrome deadline onto the process monotonic clock.
    pub(crate) fn next_chrome_deadline(&self, monotonic_now_seconds: f64, wall_now_ms: f64) -> Option<f64> {
        if !monotonic_now_seconds.is_finite() || !wall_now_ms.is_finite() {
            return None;
        }
        let chrome = self.chrome_build.next_deadline_ms(wall_now_ms);
        let tutorial = self.next_tutorial_deadline_ms(wall_now_ms);
        match (chrome, tutorial) {
            (Some(chrome), Some(tutorial)) => Some(chrome.min(tutorial)),
            (Some(deadline), None) | (None, Some(deadline)) => Some(deadline),
            (None, None) => None,
        }
        .map(|deadline_ms| monotonic_now_seconds + ((deadline_ms - wall_now_ms).max(0.0) / 1000.0))
    }

    /// 🧯️ The dispatch-fault funnel: one failed `dispatch_action` becomes one banner, classified the
    /// way React's three call sites classify theirs. This replaces parking the string in
    /// `self.error`, which never cleared and sat in the bottom-left corner far from the gesture.
    pub fn note_dispatch_fault(&mut self, error: &str) {
        let refused = self.chrome_build.refused_guest_fault.take().filter(|refused| dispatch_fault_names_code(error, &refused.fault.code.0));
        let (message, severity, code) = classify_dispatch_fault_notice(error, refused.as_ref(), self.active_terminology(), self.active_locale());
        if code.as_deref().is_some_and(time_travel::history_refusal_is_silent) {
            return;
        }
        self.show_transient_notice(message, severity, code.as_deref());
    }

    /// 📣️ Records one refused program call's structured guest fault beside the refusing app's notices for the funnel, and
    /// answers the `code: message` text every `String` dispatch channel carries.
    pub(crate) fn refused_guest_call(&mut self, refusal: crate::program_bridge::ProgramFault, app: &AppDefinition) -> String {
        #[cfg(not(target_arch = "wasm32"))]
        { if self.chrome_build.refused_frame_fault.is_none() { self.chrome_build.refused_frame_fault = refusal.frame; }if self.chrome_build.refused_kernel_fault.is_none(){self.chrome_build.refused_kernel_fault=refusal.kernel;}if self.chrome_build.refused_actor_call.is_none(){self.chrome_build.refused_actor_call=refusal.actor_call;} }
        #[cfg(not(target_arch = "wasm32"))]
        let detail = if refusal.frame.is_some() { program_fault_text(&refusal, self.active_terminology(), self.active_locale()) } else { refusal.text };
        #[cfg(target_arch = "wasm32")]
        let detail = refusal.text;
        self.chrome_build.refused_guest_fault = refusal.fault.map(|fault| RefusedGuestFault { fault, notices: app.fault_notices.clone() });
        detail
    }

    /// 🤝️ Tells one refused instance open — the host's guest admission (`plugin.channel-mismatch`, both channels named) or any
    /// other structured refusal — through the dispatch-fault funnel, so it reaches the person as its localized notice and never
    /// as a raw code; answers the text the open's own status keeps ([`program_fault_text`]).
    pub(crate) fn note_refused_open(&mut self, refusal: crate::program_bridge::ProgramFault) -> String {
        #[cfg(not(target_arch = "wasm32"))]
        { if self.chrome_build.refused_frame_fault.is_none() { self.chrome_build.refused_frame_fault = refusal.frame; }if self.chrome_build.refused_kernel_fault.is_none(){self.chrome_build.refused_kernel_fault=refusal.kernel;}if self.chrome_build.refused_actor_call.is_none(){self.chrome_build.refused_actor_call=refusal.actor_call;} }
        let detail = program_fault_text(&refusal, self.active_terminology(), self.active_locale());
        self.chrome_build.refused_guest_fault = refusal.fault.map(|fault| RefusedGuestFault { fault, notices: Vec::new() });
        #[cfg(not(target_arch = "wasm32"))]
        if refusal.frame.is_some() { self.note_dispatch_fault(&detail); return detail; }
        self.note_dispatch_fault(&refusal.text);
        detail
    }

    /// 🌉️ The agent bridge consumer this shell drives — a transport hands it one socket message at a
    /// time and drains [`crate::agent_bridge::AgentBridgeState::take_outbox_encoded`] afterwards.
    pub fn apply_agent_bridge_frame(&mut self, bytes: &[u8]) -> Result<(), crate::agent_bridge::BridgeFrameFault> {
        let now = chrome_now_ms();
        self.chrome_build.agent.apply_encoded_frame(bytes, now)
    }

    /// 🌉️ Frames the transport must write back (approval decisions, hello, ping, bye).
    pub fn take_agent_bridge_outbox(&mut self) -> Vec<Vec<u8>> {
        self.chrome_build.agent.take_outbox_encoded()
    }

    /// 🚦️ Live agent status/presence — what the chat panel header's indicator paints.
    pub fn agent_presence_tone(&self) -> crate::agent_presence::AgentPresenceTone {
        crate::agent_presence::agent_presence_tone(self.chrome_build.agent.status, &self.chrome_build.agent.presence)
    }

    /// 🔗️ Installs (or clears) the gateway config the local supervisor hands over — packet W15e.
    /// This is the ONLY way a bridge socket is ever dialled: `discover_agent_bridge_config()` answers
    /// `None` on both targets, deliberately, exactly as React's does, so a shell nobody supervises
    /// stays `Disabled` instead of guessing a url.
    pub fn set_agent_bridge_config(&mut self, config: Option<crate::agent_bridge::AgentBridgeConfig>) -> bool {
        self.agent_bridge.set_config(config, &mut self.chrome_build.agent)
    }

    /// 🛰️ One bounded offer-scope turn on the directory pump: follows sign-in and the space the shell is in, re-reads this
    /// human's own live delegations when due (one `GET /auth/agent-delegations`, [`AgentBridgeOfferScopeOwnerV1`]) and
    /// publishes the scope for the page realm.
    pub(crate) async fn pump_agent_bridge_offer_scope(&mut self) {
        let target = self.identity.as_ref().zip(self.open_space_id.as_ref()).filter(|(_, space_id)| !space_id.is_empty()).map(|(identity, space_id)| (identity.hub_base_url.trim_end_matches('/').to_string(), space_id.clone()));
        if let Some(due) = self.agent_bridge_offer_scope.observe(target, chrome_now_ms()) {
            let principals = match self.directory_client.clone() {
                Some(client) => client.live_agent_delegation_principals(&self.directory_command_ctx(), &due.1, i64::try_from(Self::directory_now_ms()).unwrap_or(i64::MAX)).await.ok(),
                None => None,
            };
            self.agent_bridge_offer_scope.settle(&due, principals, chrome_now_ms());
        }
        publish_agent_bridge_offer_scope(self.agent_bridge_offer_scope.page_json());
    }

    /// 🎬️ One bounded agent-bridge turn, folded into the same 100 ms `PumpSync` slot the directory
    /// lane already rides. Answers whether the chrome must repaint.
    ///
    /// 🧾️ `principal_actor` is React's own value for a shell with no signed-in identity
    /// (`"agent:unknown"`); a signed-in shell announces its hub user id, which is the `{userId}` half
    /// of the same actor grammar `shell_actor` already uses.
    pub fn pump_agent_bridge(&mut self) -> bool {
        let mut changed = false;
        if let Some(config) = take_pending_agent_bridge_config() {
            changed = self.set_agent_bridge_config(config);
        }
        if !self.agent_bridge.is_armed() {
            return changed;
        }
        let session = self.shell_session_id.clone();
        let principal = self.identity.as_ref().map(|identity| identity.user_id.clone()).unwrap_or_else(|| "agent:unknown".to_string());
        let now = chrome_now_ms();
        changed |= self.agent_bridge.pump(&mut self.chrome_build.agent, &session, &principal, now);
        changed | self.apply_inbound_agent_shell_commands()
    }

    /// 🤖️ Carries out the `ui_focus`/`ui_reveal` the connected agent sent, on THIS shell's real
    /// chrome, and only then acknowledges them. Answering the gateway before the dock moved is the
    /// exact defect slice M7 measured on the React host, where the bridge reduced against a private
    /// mirror and reported `ok` while nothing on screen changed.
    fn apply_inbound_agent_shell_commands(&mut self) -> bool {
        let mut changed = false;
        for inbound in self.chrome_build.agent.take_inbound_shell_commands() {
            let seq = inbound.seq();
            let (ok, fault) = match inbound {
                crate::agent_bridge::InboundShellCommand::FocusWindow { window_id, .. } => {
                    self.active_window_id = window_id;
                    changed = true;
                    (true, None)
                }
                crate::agent_bridge::InboundShellCommand::RevealPanelTab { tab_id, .. } => match self.reveal_dock_tab(&tab_id) {
                    Some(_) => {
                        changed = true;
                        (true, None)
                    }
                    None => (false, Some(format!("this shell's dock hosts no `{tab_id}` panel tab"))),
                },
                crate::agent_bridge::InboundShellCommand::Acknowledge { .. } => (true, None),
            };
            self.chrome_build.agent.settle_shell_command(seq, ok, fault);
        }
        changed
    }
}
//#endregion 🧯️TransientNoticeAndAgentOverlays

//#region 🗨️ChromeDialog
/// 🗨️ The wgpu twin of React's `UIDialog`: one declared `DialogDefinition` resolved to the active locale —
/// title, body, its staged fields, its decision choices, the submit and Cancel. Every button dispatches
/// through the generic chrome hit pipeline with the args effective in that frame (`effective_action_args`
/// over the staged values and the `Effect::OpenDialog` seed), a choice adds its own id
/// (`DialogChoice::dispatch_args`, gated on the args it requires), and every dismissal (Cancel, Escape, a veil click) sends the cancel
/// action. Focus walks the fields (a reference field's chips right after it), Cancel, the choices and the
/// submit in that order — the order `🛂️manifest/🧫️fixtures/🧫️dialog-choices` pins for every renderer.
///
/// 🎛️ Every `ActionArgControl` has its own staged field: text, a number (a stepper adds −/+ buttons), a
/// slider or dial with its detents painted as ticks, a select, a segmented choice of pressed buttons, a
/// toggle, one number field per vector axis sharing the vector's bounds, precision and detents, a colour
/// (the `color_input` recipe: a swatch beside its hex text and, with alpha, an opacity slider), and a
/// reference list of removable chips with a "use current selection" button reading the shell's live
/// interaction selection.
#[derive(Clone, Debug)]
pub(crate) struct ChromeDialogRequest {
    pub(crate) id: String,
    pub(crate) controller_id: String,
    pub(crate) title: String,
    pub(crate) body: String,
    pub(crate) defs: Vec<semio_framework::ActionArgDef>,
    pub(crate) fields: Vec<ChromeDialogField>,
    pub(crate) choices: Vec<ChromeDialogChoice>,
    pub(crate) confirm_label: String,
    pub(crate) confirm_action: String,
    pub(crate) cancel_label: String,
    pub(crate) cancel_action: Option<String>,
    pub(crate) seed: Option<DslValue>,
    pub(crate) staged: Vec<(String, DslValue)>,
    pub(crate) focus: usize,
    pub(crate) labels: ChromeDialogLabels,
}

/// 🏷️ The dialog chrome's own copy, resolved once per opening from exhaustive `LocalizedLabel`s.
#[derive(Clone, Debug)]
struct ChromeDialogLabels {
    pub(crate) on: String,
    off: String,
    pub(crate) choose: String,
    pub(crate) use_selection: String,
    pub(crate) remove: String,
    pub(crate) empty: String,
    pub(crate) decrease: String,
    pub(crate) increase: String,
}

/// ✍️ One staged field: its arg id, resolved label, control and the draft text a text/number field edits.
#[derive(Clone, Debug)]
struct ChromeDialogField {
    pub(crate) id: String,
    pub(crate) label: String,
    pub(crate) required: bool,
    pub(crate) kind: ChromeDialogFieldKind,
    pub(crate) draft: String,
}

/// 🎛️ The staged controls this renderer paints — one per `ActionArgControl`; a vector arg becomes one
/// `Axis` field per component, so every component is its own labelled focus stop, and a colour with alpha
/// a `Color` field plus a `Slider` over its fourth `component` (index, dims, the value when absent).
#[derive(Clone, Debug)]
pub(crate) enum ChromeDialogFieldKind {
    Text,
    Number { step: f64, precision: Option<u16>, min: Option<f64>, max: Option<f64>, stepper: bool },
    Slider { min: f64, max: f64, step: f64, precision: Option<u16>, unit: Option<String>, snaps: Vec<f64>, component: Option<(usize, usize, f64)> },
    Select { options: Vec<(String, String)> },
    Segmented { options: Vec<(String, String)> },
    Toggle,
    Axis { axis: usize, dims: usize, step: f64, precision: Option<u16>, min: Option<f64>, max: Option<f64>, snaps: Vec<f64> },
    Color { alpha: bool },
    Reference { domain: Option<String>, granularity: Option<String>, many: bool },
}

/// 🔀️ One decision button with its resolved label and consequence text.
#[derive(Clone, Debug)]
struct ChromeDialogChoice {
    pub(crate) choice: semio_framework::DialogChoice,
    pub(crate) label: String,
    pub(crate) description: Option<String>,
}

/// 🎯️ One focus stop of the dialog, in visiting order. `Chip(field, index)` is one removable reference of a
/// reference field.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ChromeDialogStop {
    Field(usize),
    Chip(usize, usize),
    Cancel,
    Choice(usize),
    Confirm,
}

/// ♿️ What a reader hears beyond a control's name: its role, pressed state and value.
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct ChromeDialogSemantics {
    pub(crate) role: Option<&'static str>,
    pub(crate) pressed: Option<bool>,
    pub(crate) value_text: Option<String>,
    pub(crate) value_range: Option<[f64; 3]>,
}

/// 🖌️ One step of the dialog's flat paint program — the retained chrome step walks it by index, so a
/// glyph run that needs another opportunity simply does not advance. `Segment` and `Nudge` are the pressable
/// parts of a segmented and a stepper field.
pub(crate) enum ChromeDialogPaintOp {
    Scrim,
    Modal(Rect),
    Fill { rect: Rect, color: Rgba },
    Text { value: String, x: f32, y: f32, max_w: f32, size: f32, color: Rgba },
    Hit { rect: Rect, stop: ChromeDialogStop, control_id: String, label: String, description: Option<String>, kind: HitKind, event: Option<ActionDescriptor>, semantics: ChromeDialogSemantics, disabled: bool },
    Segment { rect: Rect, field: usize, option: usize, control_id: String, label: String, selected: bool },
    Nudge { rect: Rect, field: usize, sign: f64, control_id: String, label: String },
    Clicks { dialog: Rect },
}

/// 👆️ What one click on the open dialog landed on.
#[derive(Clone, Copy, Debug)]
pub(crate) enum ChromeDialogPress {
    Stop { stop: ChromeDialogStop, enabled: bool, rect: Rect },
    Segment { field: usize, option: usize },
    Nudge { field: usize, sign: f64 },
}

impl ChromeDialogRequest {
    /// 🗨️ Resolves `dialog` against `terminology`/`locale`; `seed` is the `Effect::OpenDialog` args.
    pub(crate) fn from_definition(controller_id: &str, dialog: &semio_framework::DialogDefinition, seed: Option<DslValue>, terminology: Terminology, locale: Locale) -> Self {
        use semio_framework::ActionArgControl as Control;
        let resolve = |label: &LocalizedLabel| label.resolve(terminology, locale).to_string();
        let positive = |step: Option<f64>| step.filter(|step| step.is_finite() && *step > 0.0).unwrap_or(1.0);
        let digits = |precision: Option<u32>| precision.map(|precision| u16::try_from(precision).unwrap_or(ui_contract::UI_NUMBER_PRECISION_MAX));
        let initial = semio_framework::effective_action_args(&dialog.args, &semio_framework_value::DslValue::Object(Vec::new()), seed.as_ref());
        let fields = dialog
            .args
            .iter()
            .flat_map(|def| {
                let label = resolve(&def.label);
                let kinds: Vec<(String, ChromeDialogFieldKind)> = match def.control() {
                    Control::Number { min, max, step, precision, .. } => vec![(label, ChromeDialogFieldKind::Number { step: positive(step), precision: digits(precision), min, max, stepper: false })],
                    Control::Stepper { min, max, step, precision, .. } => vec![(label, ChromeDialogFieldKind::Number { step: positive(step), precision: digits(precision), min, max, stepper: true })],
                    Control::Slider { min, max, step, precision, unit, snaps, .. } | Control::Dial { min, max, step, precision, unit, snaps, .. } => {
                        let snaps = if ui_contract::snaps_are_valid(snaps.iter().copied(), min, max) { snaps } else { Vec::new() };
                        vec![(label, ChromeDialogFieldKind::Slider { min, max: max.max(min), step: positive(step), precision: digits(precision), unit, snaps, component: None })]
                    }
                    Control::Select { options } => vec![(label, ChromeDialogFieldKind::Select { options: options.iter().map(|option| (option.value.clone(), resolve(&option.label))).collect() })],
                    Control::Segmented { options } => vec![(label, ChromeDialogFieldKind::Segmented { options: options.iter().map(|option| (option.value.clone(), resolve(&option.label))).collect() })],
                    Control::Toggle => vec![(label, ChromeDialogFieldKind::Toggle)],
                    Control::Vector { dims, step, precision, min, max, snaps, .. } => {
                        let dims = dims as usize;
                        let precision = digits(precision);
                        let step = step.filter(|step| step.is_finite() && *step > 0.0).or_else(|| precision.map(|precision| 10_f64.powi(-i32::from(precision)))).unwrap_or(1.0);
                        let snaps = if ui_contract::snaps_are_valid(snaps.iter().copied(), min.unwrap_or(f64::NEG_INFINITY), max.unwrap_or(f64::INFINITY)) { snaps } else { Vec::new() };
                        (0..dims).map(|axis| (format!("{label} {}", chrome_dialog_axis_name(axis)), ChromeDialogFieldKind::Axis { axis, dims, step, precision, min, max, snaps: snaps.clone() })).collect()
                    }
                    Control::Color { alpha } => {
                        let opacity = LocalizedLabel::native("Opacity", "Deckkraft").resolve(terminology, locale).to_string();
                        let mut kinds = vec![(label.clone(), ChromeDialogFieldKind::Color { alpha })];
                        if alpha {
                            kinds.push((format!("{label} {opacity}"), ChromeDialogFieldKind::Slider { min: 0.0, max: 1.0, step: 0.01, precision: Some(2), unit: None, snaps: Vec::new(), component: Some((3, 4, 1.0)) }));
                        }
                        kinds
                    }
                    Control::Reference { domain, granularity, many, .. } => vec![(label, ChromeDialogFieldKind::Reference { domain, granularity, many })],
                    _ => vec![(label, ChromeDialogFieldKind::Text)],
                };
                let value = initial.get(&def.id).cloned();
                kinds.into_iter().map(move |(label, kind)| {
                    let draft = match (&kind, value.as_ref()) {
                        (ChromeDialogFieldKind::Number { precision, .. }, Some(value)) => value.as_f64().map_or_else(String::new, |value| chrome_dialog_number_text(value, *precision)),
                        (ChromeDialogFieldKind::Axis { axis, precision, .. }, Some(value)) => value.as_array().and_then(|tuple| tuple.get(*axis)).and_then(semio_framework_value::DslValue::as_f64).map_or_else(String::new, |value| chrome_dialog_number_text(value, *precision)),
                        (ChromeDialogFieldKind::Color { alpha }, Some(value)) => ui_contract::ui_color_hex(&value.as_array().map(|components| components.iter().filter_map(semio_framework_value::DslValue::as_f64).collect::<Vec<_>>()).unwrap_or_default(), *alpha),
                        (ChromeDialogFieldKind::Text, Some(value)) => value.as_str().unwrap_or_default().to_string(),
                        _ => String::new(),
                    };
                    ChromeDialogField { id: def.id.clone(), label, required: def.required, kind, draft }
                })
            })
            .collect();
        let native = |en: &str, de: &str| LocalizedLabel::native(en, de).resolve(terminology, locale).to_string();
        Self {
            id: dialog.id.clone(),
            controller_id: controller_id.to_string(),
            title: resolve(&dialog.title),
            body: dialog.body.as_ref().map(resolve).unwrap_or_default(),
            defs: dialog.args.clone(),
            fields,
            choices: dialog.choices.iter().map(|choice| ChromeDialogChoice { choice: choice.clone(), label: resolve(&choice.label), description: choice.description.as_ref().map(resolve) }).collect(),
            confirm_label: resolve(&dialog.submit_label),
            confirm_action: dialog.submit_action.as_str().to_string(),
            cancel_label: dialog.cancel_label.as_ref().map_or_else(|| native("Cancel", "Abbrechen"), resolve),
            cancel_action: dialog.cancel_action.as_ref().map(|action| action.as_str().to_string()),
            seed,
            staged: Vec::new(),
            focus: 0,
            labels: ChromeDialogLabels {
                on: native("On", "Ein"),
                off: native("Off", "Aus"),
                choose: native("Choose…", "Auswählen…"),
                use_selection: LocalizedLabel::native("Use current selection", "Aktuelle Auswahl verwenden").resolve(terminology, locale).to_string(),
                remove: LocalizedLabel::native("Remove {item}", "{item} entfernen").resolve(terminology, locale).to_string(),
                empty: LocalizedLabel::native("Nothing selected", "Nichts ausgewählt").resolve(terminology, locale).to_string(),
                decrease: LocalizedLabel::native("Decrease", "Verringern").resolve(terminology, locale).to_string(),
                increase: LocalizedLabel::native("Increase", "Erhöhen").resolve(terminology, locale).to_string(),
            },
        }
    }

    /// 🧮️ The args every button dispatches this frame: staged values over the seed and the defaults, and a colour
    /// field's edited hex (`ui_contract::parse_ui_color_hex`) over its colour — keeping the alpha when the text carries
    /// none, so a hex passing through a short form while it is typed never loses the opacity. An unedited hex (the
    /// one the colour prints as) changes nothing, so an untouched colour keeps its exact components.
    pub(crate) fn effective(&self) -> DslValue {
        let mut effective = semio_framework::effective_action_args(&self.defs, &semio_framework_value::DslValue::Object(self.staged.clone()), self.seed.as_ref());
        for field in &self.fields {
            let ChromeDialogFieldKind::Color { alpha } = field.kind else { continue };
            let base: Vec<f64> = effective.get(&field.id).and_then(|value| value.as_array().map(|components| components.iter().filter_map(semio_framework_value::DslValue::as_f64).collect())).unwrap_or_default();
            let Some(parsed) = ui_contract::parse_ui_color_hex(&field.draft).filter(|_| field.draft != ui_contract::ui_color_hex(&base, alpha)) else { continue };
            let typed_alpha = matches!(field.draft.trim().trim_start_matches('#').len(), 4 | 8);
            let rgba = [parsed[0], parsed[1], parsed[2], if typed_alpha { parsed[3] } else { base.get(3).copied().unwrap_or(1.0) }];
            let value = semio_framework_value::DslValue::Array(rgba[..if alpha { 4 } else { 3 }].iter().map(|component| semio_framework_value::DslValue::float(*component)).collect());
            if let semio_framework_value::DslValue::Object(entries) = &mut effective {
                match entries.iter_mut().find(|(key, _)| *key == field.id) {
                    Some(entry) => entry.1 = value,
                    None => entries.push((field.id.clone(), value)),
                }
            }
        }
        effective
    }

    /// 🛑️ Whether a required field is still unset or a choice left its catalog — the submit gate.
    pub(crate) fn resolved(&self) -> bool {
        semio_framework::unresolved_action_args(&self.defs, &self.effective()).is_empty()
    }

    /// 🎯️ Every focus stop in visiting order: each field (a reference field followed by its chips), Cancel, the
    /// choices, the submit.
    pub(crate) fn stops(&self) -> Vec<ChromeDialogStop> {
        let mut stops = Vec::new();
        for (index, field) in self.fields.iter().enumerate() {
            stops.push(ChromeDialogStop::Field(index));
            if matches!(field.kind, ChromeDialogFieldKind::Reference { .. }) {
                stops.extend((0..self.reference_ids(index).len()).map(|chip| ChromeDialogStop::Chip(index, chip)));
            }
        }
        stops.push(ChromeDialogStop::Cancel);
        stops.extend((0..self.choices.len()).map(ChromeDialogStop::Choice));
        stops.push(ChromeDialogStop::Confirm);
        stops
    }

    pub(crate) fn focused(&self) -> ChromeDialogStop {
        let stops = self.stops();
        stops[self.focus.min(stops.len() - 1)]
    }

    pub(crate) fn move_focus(&mut self, backwards: bool) {
        let count = self.stops().len();
        self.focus = if backwards { (self.focus + count - 1) % count } else { (self.focus + 1) % count };
    }

    pub(crate) fn focus_stop(&mut self, stop: ChromeDialogStop) {
        if let Some(index) = self.stops().iter().position(|candidate| *candidate == stop) {
            self.focus = index;
        }
    }

    pub(crate) fn stage(&mut self, id: &str, value: DslValue) {
        match self.staged.iter_mut().find(|(key, _)| key == id) {
            Some(entry) => entry.1 = value,
            None => self.staged.push((id.to_string(), value)),
        }
    }

    pub(crate) fn field_value(&self, index: usize) -> Option<DslValue> {
        self.effective().get(&self.fields.get(index)?.id).cloned()
    }

    /// ✍️ Replaces a text/number/axis/colour field's draft and stages the value it means (`Null` for an unreadable
    /// number); an axis splices its component into the vector, clamped to the vector's bounds and rounded to its
    /// precision, and a colour's draft is read by [`Self::effective`].
    pub(crate) fn set_draft(&mut self, index: usize, draft: String) {
        let Some(field) = self.fields.get(index) else { return };
        let number = draft.trim().parse::<f64>().ok().filter(|value| value.is_finite());
        let value = match field.kind {
            ChromeDialogFieldKind::Number { precision, .. } => number.map_or(semio_framework_value::DslValue::Null, |value| semio_framework_value::DslValue::float(precision.map_or(value, |precision| ui_contract::round_ui_number(value, precision)))),
            ChromeDialogFieldKind::Axis { axis, dims, precision, min, max, .. } => {
                let mut tuple: Vec<DslValue> = self.field_value(index).and_then(|value| value.as_array().map(<[DslValue]>::to_vec)).unwrap_or_default();
                tuple.resize(dims, semio_framework_value::DslValue::float(0.0));
                tuple[axis] = number.map_or(semio_framework_value::DslValue::Null, |value| semio_framework_value::DslValue::float(precision.map_or(value, |precision| ui_contract::round_ui_number(value, precision)).max(min.unwrap_or(f64::NEG_INFINITY)).min(max.unwrap_or(f64::INFINITY))));
                semio_framework_value::DslValue::Array(tuple)
            }
            ChromeDialogFieldKind::Color { .. } => {
                self.fields[index].draft = draft;
                return;
            }
            _ => semio_framework_value::DslValue::String(draft.clone()),
        };
        let id = field.id.clone();
        self.fields[index].draft = draft;
        self.stage(&id, value);
    }

    /// 🔀️ Flips a toggle field.
    pub(crate) fn flip(&mut self, index: usize) {
        let on = matches!(self.field_value(index), Some(DslValue::Bool(true)));
        if let Some(id) = self.fields.get(index).filter(|field| matches!(field.kind, ChromeDialogFieldKind::Toggle)).map(|field| field.id.clone()) {
            self.stage(&id, semio_framework_value::DslValue::Bool(!on));
        }
    }

    /// 📋️ A select or segmented field's options.
    pub(crate) fn options(&self, index: usize) -> &[(String, String)] {
        match self.fields.get(index).map(|field| &field.kind) {
            Some(ChromeDialogFieldKind::Select { options } | ChromeDialogFieldKind::Segmented { options }) => options,
            _ => &[],
        }
    }

    /// 🔽️ Moves a select or segmented field to its next (`forward`) or previous option, wrapping.
    pub(crate) fn cycle(&mut self, index: usize, forward: bool) {
        let options = self.options(index);
        if options.is_empty() {
            return;
        }
        let current = self.field_value(index).and_then(|value| value.as_str().map(str::to_string));
        let position = current.and_then(|current| options.iter().position(|(value, _)| *value == current));
        let next = match (position, forward) {
            (None, true) => 0,
            (None, false) => options.len() - 1,
            (Some(position), true) => (position + 1) % options.len(),
            (Some(position), false) => (position + options.len() - 1) % options.len(),
        };
        self.choose(index, next);
    }

    /// 🔘️ Picks one option of a select or segmented field.
    pub(crate) fn choose(&mut self, index: usize, option: usize) {
        let Some(value) = self.options(index).get(option).map(|(value, _)| value.clone()) else { return };
        let id = self.fields[index].id.clone();
        self.stage(&id, semio_framework_value::DslValue::String(value));
    }

    /// 🔢️ Steps a number or axis field through the shared keyboard law (`ui_contract::ui_number_key_value`): a
    /// sign walks one rung of its step ladder, a page (`page`) jumps to the adjacent detent of an axis or walks ten
    /// rungs, inside its bounds.
    pub(crate) fn nudge_by(&mut self, index: usize, sign: f64, page: bool) {
        let Some(field) = self.fields.get(index) else { return };
        let (step, precision, min, max, snaps) = match &field.kind {
            ChromeDialogFieldKind::Number { step, precision, min, max, .. } => (*step, *precision, *min, *max, Vec::new()),
            ChromeDialogFieldKind::Axis { step, precision, min, max, snaps, .. } => (*step, *precision, *min, *max, snaps.clone()),
            _ => return,
        };
        let current = field.draft.trim().parse::<f64>().ok().filter(|value| value.is_finite()).unwrap_or(0.0);
        let key = match (page, sign > 0.0) {
            (false, true) => ui_contract::SliderKey::Increment,
            (false, false) => ui_contract::SliderKey::Decrement,
            (true, true) => ui_contract::SliderKey::PageUp,
            (true, false) => ui_contract::SliderKey::PageDown,
        };
        self.set_draft(index, chrome_dialog_number_text(ui_contract::ui_number_key_value(current, min, max, step, precision, None, snaps, key, false), precision));
    }

    /// 🔢️ Steps a number or axis field one rung of its step ladder.
    pub(crate) fn nudge(&mut self, index: usize, sign: f64) {
        self.nudge_by(index, sign, false);
    }

    /// 🎚️ A slider field's current value, on its travel — a component slider's component of its vector.
    pub(crate) fn slider_value(&self, index: usize) -> Option<f64> {
        let ChromeDialogFieldKind::Slider { min, max, component, .. } = self.fields.get(index)?.kind else { return None };
        let value = self.field_value(index);
        let current = match component {
            Some((at, _, absent)) => value.and_then(|value| value.as_array().and_then(|components| components.get(at)).and_then(semio_framework_value::DslValue::as_f64)).unwrap_or(absent),
            None => value.and_then(|value| value.as_f64()).unwrap_or(min),
        };
        Some(current.clamp(min, max))
    }

    /// 🎚️ Sets a slider field, clamped to its travel and rounded to its precision — a component slider splices it
    /// into its vector.
    pub(crate) fn set_slider(&mut self, index: usize, value: f64) {
        let Some(ChromeDialogFieldKind::Slider { min, max, precision, component, .. }) = self.fields.get(index).map(|field| field.kind.clone()) else { return };
        let value = semio_framework_value::DslValue::float(precision.map_or(value.clamp(min, max), |precision| ui_contract::round_ui_number(value.clamp(min, max), precision)));
        let staged = match component {
            Some((at, dims, _)) => {
                let mut tuple: Vec<DslValue> = self.field_value(index).and_then(|current| current.as_array().map(<[DslValue]>::to_vec)).unwrap_or_default();
                tuple.resize(dims, semio_framework_value::DslValue::float(0.0));
                tuple[at] = value;
                semio_framework_value::DslValue::Array(tuple)
            }
            None => value,
        };
        let id = self.fields[index].id.clone();
        let printed = component.and_then(|_| staged.as_array().map(|components| components.iter().filter_map(semio_framework_value::DslValue::as_f64).collect::<Vec<_>>()));
        self.stage(&id, staged);
        if let Some(components) = printed {
            for field in self.fields.iter_mut().filter(|field| field.id == id) {
                if let ChromeDialogFieldKind::Color { alpha } = field.kind {
                    field.draft = ui_contract::ui_color_hex(&components, alpha);
                }
            }
        }
    }

    /// ⌨️ One slider key through the shared keyboard law (`ui_contract::slider_key_value`, pinned by
    /// `🧫️number-controls` for every renderer): arrows walk the step ladder and never snap (`large` — a `Shift`
    /// chord — walks ten rungs), PageUp/PageDown jump to the adjacent detent (ten rungs without one), Home/End go
    /// to the ends. `false` for a key a slider does not take.
    pub(crate) fn slide(&mut self, index: usize, action: &ui_wgpu::wgpu::KeyAction, large: bool) -> bool {
        use ui_contract::SliderKey;
        use ui_wgpu::wgpu::KeyAction;
        let (Some(ChromeDialogFieldKind::Slider { min, max, step, precision, snaps, .. }), Some(value)) = (self.fields.get(index).map(|field| field.kind.clone()), self.slider_value(index)) else { return false };
        let key = match action {
            KeyAction::ArrowLeft | KeyAction::ArrowDown => SliderKey::Decrement,
            KeyAction::ArrowRight | KeyAction::ArrowUp => SliderKey::Increment,
            KeyAction::PageUp => SliderKey::PageUp,
            KeyAction::PageDown => SliderKey::PageDown,
            KeyAction::Home => SliderKey::Home,
            KeyAction::End => SliderKey::End,
            _ => return false,
        };
        self.set_slider(index, ui_contract::slider_key_value(value, min, max, step, precision, None, snaps, key, large));
        true
    }

    /// 👆️ A pointer at `share` of a slider's rail: the shared detent law of every renderer.
    pub(crate) fn point_slider(&mut self, index: usize, share: f64) {
        let Some(ChromeDialogFieldKind::Slider { min, max, step, snaps, .. }) = self.fields.get(index).map(|field| field.kind.clone()) else { return };
        self.set_slider(index, ui_contract::slider_pointer_value(min + share.clamp(0.0, 1.0) * (max - min), min, max, step, snaps, ui_contract::UiNumberScale::Linear));
    }

    /// 🧷️ A reference field's referenced ids, in order.
    pub(crate) fn reference_ids(&self, index: usize) -> Vec<String> {
        match self.field_value(index) {
            Some(semio_framework_value::DslValue::Array(ids)) => ids.iter().filter_map(|id| id.as_str().map(str::to_string)).collect(),
            Some(semio_framework_value::DslValue::String(id)) if !id.is_empty() => vec![id],
            _ => Vec::new(),
        }
    }

    /// 🧷️ Stages `ids` into a reference field — a list when it takes many, else the first id.
    fn stage_references(&mut self, index: usize, ids: Vec<String>) {
        let Some(ChromeDialogFieldKind::Reference { many, .. }) = self.fields.get(index).map(|field| field.kind.clone()) else { return };
        let value = if many { semio_framework_value::DslValue::Array(ids.into_iter().map(semio_framework_value::DslValue::String).collect()) } else { ids.into_iter().next().map_or(semio_framework_value::DslValue::Null, semio_framework_value::DslValue::String) };
        let id = self.fields[index].id.clone();
        self.stage(&id, value);
    }

    /// 🎯️ "Use current selection": the live selection of the field's domain replaces its references.
    pub(crate) fn use_selection(&mut self, index: usize, selection: &HashMap<String, semio_framework::DomainSelection>) {
        let Some(ChromeDialogFieldKind::Reference { domain, granularity, .. }) = self.fields.get(index).map(|field| field.kind.clone()) else { return };
        self.stage_references(index, reference_selection_ids(selection, domain.as_deref(), granularity.as_deref()));
    }

    /// ✖️ Removes one reference; focus stays on the chip that takes its place, else returns to the field.
    pub(crate) fn remove_chip(&mut self, index: usize, chip: usize) {
        let mut ids = self.reference_ids(index);
        if chip >= ids.len() {
            return;
        }
        ids.remove(chip);
        let left = ids.len();
        self.stage_references(index, ids);
        self.focus_stop(if left == 0 { ChromeDialogStop::Field(index) } else { ChromeDialogStop::Chip(index, chip.min(left - 1)) });
    }

    /// ▶️ The action a stop dispatches now: `None` for a field or chip, for a gated submit or choice, and for a
    /// Cancel without a declared cancel action.
    pub(crate) fn action(&self, stop: ChromeDialogStop) -> Option<ActionDescriptor> {
        let descriptor = |action: &str, args: Option<DslValue>| ActionDescriptor { controller_id: self.controller_id.clone(), action: action.to_string(), args };
        match stop {
            ChromeDialogStop::Field(_) | ChromeDialogStop::Chip(..) => None,
            ChromeDialogStop::Cancel => self.cancel_action.as_deref().map(|action| descriptor(action, None)),
            ChromeDialogStop::Choice(index) => {
                let choice = &self.choices.get(index)?.choice;
                let effective = self.effective();
                choice.unresolved_args(&self.defs, &effective).is_empty().then(|| descriptor(choice.action.as_str(), Some(choice.dispatch_args(&self.defs, &effective))))
            }
            ChromeDialogStop::Confirm => self.resolved().then(|| descriptor(&self.confirm_action, Some(self.effective()))),
        }
    }

    /// 🔏️ The key of the dialog's own accessibility node; every control it paints registers under `{key}.…`.
    pub(crate) fn accessibility_key(&self) -> String {
        format!("shell.dialog.{}", self.id)
    }

    /// 🧿️ The painted control `key` names: a focus stop, or a segment (`Some(option)`) or stepper button (the sign)
    /// of a field.
    pub(crate) fn accessibility_target(&self, key: &str) -> Option<(ChromeDialogStop, Option<usize>, f64)> {
        if let Some(stop) = self.stops().into_iter().find(|stop| self.control_id(*stop) == key) {
            return Some((stop, None, 0.0));
        }
        (0..self.fields.len()).find_map(|index| {
            let part = key.strip_prefix(&self.control_id(ChromeDialogStop::Field(index)))?;
            match part {
                ".decrement" => Some((ChromeDialogStop::Field(index), None, -1.0)),
                ".increment" => Some((ChromeDialogStop::Field(index), None, 1.0)),
                _ => part.strip_prefix(".option.").and_then(|value| self.options(index).iter().position(|(option, _)| option == value)).map(|option| (ChromeDialogStop::Field(index), Some(option), 0.0)),
            }
        })
    }

    /// 🏷️ The chrome control id a stop registers its hit and accessible name under.
    pub(crate) fn control_id(&self, stop: ChromeDialogStop) -> String {
        match stop {
            ChromeDialogStop::Field(index) => match self.fields[index].kind {
                ChromeDialogFieldKind::Axis { axis, .. } => format!("shell.dialog.{}.field.{}.{}", self.id, self.fields[index].id, chrome_dialog_axis_name(axis)),
                ChromeDialogFieldKind::Slider { component: Some(_), .. } => format!("shell.dialog.{}.field.{}.alpha", self.id, self.fields[index].id),
                _ => format!("shell.dialog.{}.field.{}", self.id, self.fields[index].id),
            },
            ChromeDialogStop::Chip(index, chip) => format!("shell.dialog.{}.field.{}.chip.{}", self.id, self.fields[index].id, self.reference_ids(index).get(chip).cloned().unwrap_or_default()),
            ChromeDialogStop::Cancel => format!("shell.dialog.{}.cancel", self.id),
            ChromeDialogStop::Choice(index) => format!("shell.dialog.{}.choice.{}", self.id, self.choices[index].choice.id),
            ChromeDialogStop::Confirm => format!("shell.dialog.{}.confirm", self.id),
        }
    }

    /// 📝️ What a field shows: its draft, the chosen option's label, the toggle state, the slider value or the
    /// references — and whether that is placeholder text.
    pub(crate) fn field_text(&self, index: usize) -> (String, bool) {
        let field = &self.fields[index];
        match &field.kind {
            ChromeDialogFieldKind::Text | ChromeDialogFieldKind::Number { .. } | ChromeDialogFieldKind::Axis { .. } | ChromeDialogFieldKind::Color { .. } => (field.draft.clone(), false),
            ChromeDialogFieldKind::Toggle => (if matches!(self.field_value(index), Some(DslValue::Bool(true))) { self.labels.on.clone() } else { self.labels.off.clone() }, false),
            ChromeDialogFieldKind::Select { options } | ChromeDialogFieldKind::Segmented { options } => {
                let current = self.field_value(index).and_then(|value| value.as_str().map(str::to_string));
                match current.and_then(|current| options.iter().find(|(value, _)| *value == current)) {
                    Some((_, label)) => (label.clone(), false),
                    None => (self.labels.choose.clone(), true),
                }
            }
            ChromeDialogFieldKind::Slider { precision, unit, .. } => {
                let value = self.slider_value(index).map_or_else(String::new, |value| chrome_dialog_number_text(value, *precision));
                (unit.as_deref().map_or_else(|| value.clone(), |unit| format!("{value} {unit}")), false)
            }
            ChromeDialogFieldKind::Reference { .. } => {
                let ids = self.reference_ids(index);
                if ids.is_empty() {
                    (self.labels.empty.clone(), true)
                } else {
                    (ids.join(", "), false)
                }
            }
        }
    }

    #[cfg(test)]
    pub(crate) fn confirm(id: &str, title: &str, action: &str) -> Self {
        let dialog = semio_framework::DialogDefinition::new(id, LocalizedLabel::native(title, title), semio_framework::ActionRef::new(action));
        Self::from_definition("test", &dialog, None, Terminology::Native, Locale::En)
    }
}

/// 🔢️ A number field's text: at its precision when it has one, else twelve significant digits.
fn chrome_dialog_number_text(value: f64, precision: Option<u16>) -> String {
    precision.map_or_else(|| ui_contract::format_ui_number(value), |precision| ui_contract::format_ui_number_fixed(value, precision))
}

/// 🧭️ A vector component's name — React's staged vector row names them `x y z w`, then by index.
fn chrome_dialog_axis_name(axis: usize) -> String {
    ["x", "y", "z", "w"].get(axis).map_or_else(|| axis.to_string(), |name| (*name).to_string())
}

/// 📒️ Records what a reader hears for one painted dialog control — its name, consequence text, role, pressed state,
/// value and whether it is gated — in the registry the chrome accessibility projection reads. Both render paths go
/// through here, so the published tree always describes the frame on screen.
pub(crate) fn note_chrome_dialog_control(op: &ChromeDialogPaintOp) {
    match op {
        ChromeDialogPaintOp::Hit { control_id, label, description, semantics, disabled, .. } => {
            note_chrome_control_name(control_id, Some(label));
            note_chrome_control_description(control_id, description.as_deref());
            note_chrome_control_semantics(control_id, semantics);
            note_chrome_control_disabled(control_id, *disabled);
        }
        ChromeDialogPaintOp::Segment { control_id, label, selected, .. } => {
            note_chrome_control_name(control_id, Some(label));
            note_chrome_control_semantics(control_id, &ChromeDialogSemantics { role: Some("button"), pressed: Some(*selected), value_text: None, value_range: None });
        }
        ChromeDialogPaintOp::Nudge { control_id, label, .. } => note_chrome_control_name(control_id, Some(label)),
        _ => {}
    }
}
//#endregion 🗨️ChromeDialog

/// 🎓️ Live playback state for `AppDefinition.introduction` — which step is showing. Steps themselves are
/// re-read fresh from `session.app.introduction` every frame, never cached, so a plugin hot-reload
/// mid-session can't desync from a stale copy.
#[derive(Clone, Debug)]
pub(crate) struct ChromeTourState {
    pub(crate) step_index: usize,
    /// ✅️ Indices into the active step's `interactions` that are done — reset whenever `step_index` changes.
    pub(crate) completed_interactions: Vec<usize>,
}

impl ShellChromeBuildState {
    #[cfg(test)]
    pub(crate) fn register_tooltip(&mut self, control_id: impl Into<String>, title: impl Into<String>) {
        let title = title.into();
        if !title.is_empty() {
            self.tooltip_titles.insert(control_id.into(), title);
        }
    }

    /// 💡️ Whether the ACTIVE driver lets a chrome control show a hover tooltip at all —
    /// `useControlTooltipText`'s own first two guards (`🏷️Label/🟦️.tsx:186-190`). `label_visible`
    /// says whether the control already paints its caption beside its icon, which React reads off
    /// `useControlInlineText` and never repeats in a tooltip.
    pub(crate) fn tooltip_shows(&self, label_visible: bool) -> bool {
        self.driver.tooltip_shows(label_visible)
    }

    pub(crate) fn compute_click_edge(&mut self, pointer_down: bool) {
        self.clicked_this_frame = pointer_down && !self.previous_pointer_down;
        self.previous_pointer_down = pointer_down;
    }

    pub(crate) fn dialog_open(&self) -> bool {
        !self.dialog_stack.is_empty()
    }

    /// 🎓️ Whether the introduction is PLAYING, i.e. its veil and card own the pointer.
    pub(crate) fn tour_is_open(&self) -> bool {
        self.tour_state.is_some()
    }

    pub(crate) fn open_dialog(&mut self, request: ChromeDialogRequest) {
        self.dialog_stack.push(request);
    }

    pub(crate) fn close_topmost_dialog(&mut self) {
        self.dialog_stack.pop();
    }

    pub(crate) fn start_introduction(&mut self) {
        self.tour_state = Some(ChromeTourState { step_index: 0, completed_interactions: Vec::new() });
    }

    pub(crate) fn skip_introduction(&mut self) {
        self.tour_state = None;
    }

    pub(crate) fn advance_introduction(&mut self, step_count: usize) {
        if let Some(tour) = self.tour_state.as_mut() {
            if tour.step_index + 1 >= step_count {
                self.tour_state = None;
            } else {
                tour.step_index += 1;
                tour.completed_interactions.clear();
            }
        }
    }

    pub(crate) fn back_introduction(&mut self) {
        if let Some(tour) = self.tour_state.as_mut() {
            tour.step_index = tour.step_index.saturating_sub(1);
            tour.completed_interactions.clear();
        }
    }

    pub(crate) fn introduction_was_seen(&self, app_id: &str) -> bool {
        self.introduction_seen.get(app_id).copied().unwrap_or(false)
    }

    pub(crate) fn mark_introduction_seen(&mut self, app_id: &str) {
        if !self.introduction_was_seen(app_id) {
            self.introduction_seen.insert(app_id.to_string(), true);
            self.introduction_seen_writes.push(app_id.to_string());
        }
    }

    /// 🎓️ Ends the tour AND records the answer, the wgpu twin of `🏛️ShellHost/🟦️.tsx`'s
    /// `dismissIntroduction` — which persists the device-local seen flag on Skip exactly as it does on
    /// Done, so a tour the user answered never comes back. [`skip_introduction`] stays the
    /// answer-less variant, for the two places a TUTORIAL takes the surface over (Design Decision 8).
    pub(crate) fn dismiss_introduction(&mut self, app_id: &str) {
        self.tour_state = None;
        self.mark_introduction_seen(app_id);
    }
}

/// 🎓️ Matches React's first-frame arming predicate with an independently owned host policy.
pub(crate) fn should_auto_start_introduction(seen_key: &str, has_introduction: bool, tutorial_active: bool, seen: bool, replay_on_load: bool, suppressed: bool) -> bool {
    !seen_key.is_empty() && has_introduction && !tutorial_active && !suppressed && (replay_on_load || !seen)
}

/// 🧭️ Item 5's "active-path tracking": true if `nodes` (recursively, through nested `Collection`s)
/// contains a pressed `Toggle` — used so a *collapsed* ribbon `Collection` still highlights when the
/// user's current selection lives inside it, mirroring `ui/js/react/index.tsx`'s recursive picker
/// active-path reconciliation. Called from `render_utility_nodes` (an off-limits `ShellInput`-
/// adjacent function this wave — see the report's coordination note).
#[cfg(test)]
pub(crate) fn utility_subtree_has_active_path(nodes: &[UtilityNode]) -> bool {
    nodes.iter().any(|node| match node {
        UtilityNode::Toggle { pressed, .. } => pressed.unwrap_or(false),
        UtilityNode::Collection { children, .. } => utility_subtree_has_active_path(children),
        _ => false,
    })
}

/// 🖱️ Edge-detects "pointer just went down this frame", computed once per `render_chrome` call —
/// `InputState::pointer_down` is level-triggered (true every frame the button is held), so the
/// chrome-owned click handling below (dialogs, tour controls, the tour trigger) that lives outside the
/// `ActionDescriptor`/`ShellActions` dispatch pipeline needs its own edge tracking to avoid re-firing on
/// every held frame. Reads afterward (`chrome_clicked_this_frame`) are pure.
/// ⏱️ Self-contained wall-clock reader for the tooltip hover-delay timer — deliberately not sharing the
/// file's other `*_now_ms` helper (its exact module/scope kept shifting under concurrent edits from
/// `w3-prefs-i18n-themes` while this region was being written), same cfg-gated `js_sys::Date::now()` /
/// `SystemTime` split as the rest of this file already uses elsewhere.
#[cfg(target_arch = "wasm32")]
pub(crate) fn chrome_now_ms() -> f64 {
    js_sys::Date::now()
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn chrome_now_ms() -> f64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|duration| duration.as_secs_f64() * 1000.0).unwrap_or(0.0)
}

/// 🧰️ Whether `target_id` (a bare Button/Toggle utility id) exists anywhere in `nodes`, recursing through
/// nested `Collection`s.
fn utility_node_contains(nodes: &[UtilityNode], target_id: &str) -> bool {
    nodes.iter().any(|node| match node {
        UtilityNode::Button { id, .. } | UtilityNode::Toggle { id, .. } => id == target_id,
        UtilityNode::Collection { id, children, .. } => id == target_id || utility_node_contains(children, target_id),
        UtilityNode::Separator { .. } => false,
    })
}

/// 🧰️ Ancestor `Collection` ids (root → immediate parent) that must be expanded for `target_id` to be
/// visible in the footer utility bar — empty when `target_id` isn't nested in any collection (already
/// visible) or doesn't exist. Pure so it's independently testable from the frame-reveal wiring.
pub(crate) fn utility_collection_path_to_id(nodes: &[UtilityNode], target_id: &str) -> Vec<String> {
    for node in nodes {
        if let UtilityNode::Collection { id, children, .. } = node {
            if utility_node_contains(children, target_id) {
                let mut path = vec![id.clone()];
                path.extend(utility_collection_path_to_id(children, target_id));
                return path;
            }
        }
    }
    Vec::new()
}

/// 🎓️ A rect registered this frame under a logical element id (`register_element_rect`) — `fallback`
/// entries (folded-chrome chips) never override an already-registered primary entry, mirroring
/// `ui/js/react/index.tsx`'s `useIntroductionAnchorRect` "never downgrade" rule.
pub(crate) struct ChromeElementRectEntry {
    pub(crate) rect: Rect,
}

impl ShellChromeBuildState {
    pub(crate) fn register_element_rect(&mut self, id: impl Into<String>, rect: Rect) {
        self.element_rects.insert(id.into(), ChromeElementRectEntry { rect });
    }

    #[cfg(test)]
    pub(crate) fn register_element_rect_fallback(&mut self, id: impl Into<String>, rect: Rect) {
        let id = id.into();
        self.element_rects.entry(id).or_insert(ChromeElementRectEntry { rect });
    }

    pub(crate) fn resolve_element_rect(&self, id: &str) -> Option<Rect> {
        self.element_rects.get(id).map(|entry| entry.rect)
    }
}

/// 🎓️ Punches `hole` out of `band`, returning up to four remaining rectangles (or the original band when
/// they don't overlap). The React shell renders one fullscreen `ui-veil` div and raises the
/// introduced/shown element's chrome unit above it via z-index; wgpu keeps the geometric subtraction
/// because 3D window content lives in separate `scene_passes` that cannot be repainted above an
/// overlay at all, so a cutout is the only way to leave the introduced element unveiled.
pub(crate) fn punch_introduction_cutout(band: Rect, hole: Rect) -> Vec<Rect> {
    let top = band.y.max(hole.y);
    let left = band.x.max(hole.x);
    let bottom = (band.y + band.h).min(hole.y + hole.h);
    let right = (band.x + band.w).min(hole.x + hole.w);
    if right <= left || bottom <= top {
        return vec![band];
    }
    [Rect::new(band.x, band.y, band.w, top - band.y), Rect::new(band.x, bottom, band.w, band.y + band.h - bottom), Rect::new(band.x, top, left - band.x, bottom - top), Rect::new(right, top, band.x + band.w - right, bottom - top)]
        .into_iter()
        .filter(|piece| piece.w > 0.0 && piece.h > 0.0)
        .collect()
}

/// 🎓️ Splits the viewport into bands tiling the space around every cutout — an empty cutout list
/// returns one full-viewport band. Each band is pushed as a REAL `ui-veil` glass region
/// (`Theme::veil_glass`), so the shell behind it is blurred exactly as React's `backdrop-filter` blurs
/// the page.
///
/// 🩸️ This docstring used to argue that a real glass veil was infeasible, reading the batch renderer's
/// `composite_to_swapchain` — where overlay glass composites BEFORE the overlay's own instance pass,
/// i.e. beneath the chrome it should frost. That path is `#[cfg(test)]`. The production ladder is
/// `Commands → BlurScene → EncodeComposite → GlassCommands → ForegroundCommands` (`🧊️gpu/🦀️.rs`),
/// which encodes every non-glass-content layer of BOTH lists into the scene the blur chain mips, so a
/// glass region composites the frosted navbar, footer and panes — not blurred bare canvas. The
/// argument was true of the batch path and false of the one that actually presents
/// (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY packet W8a).
///
/// ⚠️ One divergence remains: `ForegroundCommands` re-encodes EVERY glass-content layer after the
/// glass pass, so a window cap's own chips (`🛰️Dock`'s `begin_glass_content`) stay crisp over the
/// veil where React blurs them. Ordering foreground content against a LATER glass region is a
/// ladder-level change — see the packet report's hand-offs.
pub(crate) fn introduction_veil_bands(width: f32, height: f32, cutouts: &[Rect]) -> Vec<Rect> {
    let mut bands = vec![Rect::new(0.0, 0.0, width, height)];
    for cutout in cutouts {
        let x = cutout.x.clamp(0.0, width);
        let y = cutout.y.clamp(0.0, height);
        let w = (cutout.x + cutout.w).clamp(0.0, width) - x;
        let h = (cutout.y + cutout.h).clamp(0.0, height) - y;
        if w <= 0.0 || h <= 0.0 {
            continue;
        }
        let clamped = Rect::new(x, y, w, h);
        bands = bands.into_iter().flat_map(|band| punch_introduction_cutout(band, clamped)).collect();
    }
    bands
}

#[cfg(test)]
pub(crate) const INTRODUCED_PULSE_PERIOD_MS: f64 = 1600.0;

/// 🎓️ Raised-cosine breathing thickness for the introduced-element pulse ring — mirrors the
/// `data-introduced` CSS keyframes (`ui/styling/js/🎨️ui.css`: hairline → focus → hairline over 1.6s,
/// ease-in-out), which are exactly a raised cosine.
#[cfg(test)]
pub(crate) fn introduced_pulse_thickness(now_ms: f64, hairline: f32, focus: f32) -> f32 {
    let phase = (now_ms.rem_euclid(INTRODUCED_PULSE_PERIOD_MS) / INTRODUCED_PULSE_PERIOD_MS) as f32;
    hairline + (focus - hairline) * 0.5 * (1.0 - (phase * std::f32::consts::TAU).cos())
}

pub(crate) const INTRODUCTION_INFO_BOX_GAP: f32 = 16.0;

/// 🎓️ Where the info box sits relative to `anchor` — byte-for-byte port of `resolveIntroductionPlacement`
/// (`ui/js/react/index.tsx`). `auto` picks the side with the most free viewport space; `center` (and any
/// anchor-less step) centers the box.
pub(crate) fn resolve_introduction_placement(placement: semio_framework::IntroductionPlacement, anchor: Option<Rect>, box_size: (f32, f32), viewport: (f32, f32)) -> (f32, f32) {
    use semio_framework::IntroductionPlacement;
    let (box_w, box_h) = box_size;
    let (vw, vh) = viewport;
    let centered = ((vw - box_w) / 2.0, (vh - box_h) / 2.0);
    let Some(anchor) = anchor else {
        return centered;
    };
    if matches!(placement, IntroductionPlacement::Center) {
        return centered;
    }
    let gap = INTRODUCTION_INFO_BOX_GAP;
    let clamp_left = |left: f32| left.max(gap).min((vw - box_w - gap).max(gap));
    let clamp_top = |top: f32| top.max(gap).min((vh - box_h - gap).max(gap));
    let space_top = anchor.y;
    let space_bottom = vh - (anchor.y + anchor.h);
    let space_left = anchor.x;
    let space_right = vw - (anchor.x + anchor.w);
    let side = if matches!(placement, IntroductionPlacement::Auto) {
        [("top", space_top), ("bottom", space_bottom), ("left", space_left), ("right", space_right)].into_iter().max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal)).map(|(side, _)| side).unwrap_or("bottom")
    } else {
        match placement {
            IntroductionPlacement::Top => "top",
            IntroductionPlacement::Bottom => "bottom",
            IntroductionPlacement::Left => "left",
            IntroductionPlacement::Right => "right",
            _ => "bottom",
        }
    };
    match side {
        "top" => (clamp_left(anchor.x + anchor.w / 2.0 - box_w / 2.0), clamp_top(anchor.y - box_h - gap)),
        "bottom" => (clamp_left(anchor.x + anchor.w / 2.0 - box_w / 2.0), clamp_top(anchor.y + anchor.h + gap)),
        "left" => (clamp_left(anchor.x - box_w - gap), clamp_top(anchor.y + anchor.h / 2.0 - box_h / 2.0)),
        "right" => (clamp_left(anchor.x + anchor.w + gap), clamp_top(anchor.y + anchor.h / 2.0 - box_h / 2.0)),
        _ => centered,
    }
}

/// 🎓️ React's card is `w-fit max-w-sm` (`UIIntroduction`, `🖱️ui/🎯️targets/⚛️react/🟦️.tsx`) — 24rem of
/// content at the widest, and never so narrow that a two-word step reads as a label.
const INTRODUCTION_BOX_MAX_WIDTH: f32 = 384.0;
const INTRODUCTION_BOX_MIN_WIDTH: f32 = 240.0;

/// 🎓️ Port of `splitIntroductionBodyParagraphs` (`🖱️ui/🎯️targets/⚛️react/🟦️.tsx:4900`): a blank line
/// starts a new `[data-slot="introduction-body-paragraph"]`, empties are dropped.
fn split_introduction_body_paragraphs(body: &str) -> Vec<String> {
    let mut paragraphs = Vec::new();
    for raw in body.split("\n\n") {
        let trimmed = raw.trim();
        if !trimmed.is_empty() {
            paragraphs.push(trimmed.to_string());
        }
    }
    paragraphs
}

/// 🆔️ React's `panelTabElementId` (`🖥️platform/🟦️.ts`): the id an introduction step names a panel tab
/// by. The tab id is already dotted, so it is appended verbatim — no `element_id_segment` pass.
pub(crate) fn panel_tab_introduction_element_id(tab_id: &str) -> String {
    format!("framework.panelTab.{tab_id}")
}

/// 🎓️ Where the element `step.introduce`/`step.show` names actually IS on this frame's screen — the
/// wgpu twin of React's `useIntroductionAnchorRect`/`useIntroductionElevation`, which resolve the id
/// through `elementIdSelector` and read the DOM rect. Three registries answer here, in React's own
/// order of specificity: the per-frame logical rect registry (`register_element_rect` — utility bar
/// buttons, panel tab chips, window bodies), then the dock's own window content rects for a
/// `framework.window.<kind>` id, then nothing.
fn chrome_tour_anchor_rect(state: &ShellState, element_id: &str) -> Option<Rect> {
    if let Some(rect) = state.chrome_build.resolve_element_rect(element_id) {
        return Some(rect);
    }
    let rest = element_id.strip_prefix("framework.window.")?;
    let segment = rest.split_once(".action.").map_or(rest, |(segment, _)| segment);
    let session = state.session.as_ref()?;
    let kind = session.app.window_kinds.iter().find(|kind| semio_framework::element_id_segment(&kind.id) == segment)?;
    state.dock.window_instances().into_iter().filter(|(_, kind_id)| kind_id == &kind.id).find_map(|(window_id, _)| state.window_content_rects.get(&window_id).copied())
}

/// 🫧 Closes an overlay sheet's glass-content layer, if this walk opened one — the tour card and the
/// confirm dialog both. `cursor.depth` carries `region + 1` so `0` is "never opened": these ladders
/// are stepped ACROSS host opportunities and can be abandoned at a fault, a closed session, a
/// vanished step or a popped dialog, and an unbalanced `begin_glass_content` would classify every
/// LATER overlay child as this sheet's foreground.
pub(crate) fn open_chrome_overlay_glass_content(cursor: &mut ShellChromeChildCursor, overlay: &mut DrawList, rect: Rect, radius: f32, style: ui_wgpu::wgpu::GlassStyle) {
    close_chrome_overlay_glass_content(cursor, overlay);
    let region = overlay.push_glass([rect.x, rect.y, rect.w, rect.h], radius, style);
    cursor.depth = region.saturating_add(1);
    overlay.begin_glass_content(region);
}

pub(crate) fn close_chrome_overlay_glass_content(cursor: &mut ShellChromeChildCursor, overlay: &mut DrawList) {
    if cursor.depth == 0 {
        return;
    }
    overlay.end_glass_content();
    cursor.depth = 0;
}

/// ⌨️ The introduction's own chords, the three rows of React's `SHELL_KEYBINDINGS`
/// (`🖱️ui/🔨️modules/🕹️control-keybinding-context/🟦️.tsx:163-166`) that [`SHELL_SHORTCUT_ROWS`] does not
/// carry: that table is the ACCELERATOR table, every row of which must resolve to a [`ShellShortcut`]
/// verb, and these three are answered by the tour's own ladder instead. They are listed here so the
/// card's buttons can wear their chord the way React's `ControlHotkeyBadge` does.
const INTRODUCTION_SHORTCUT_ROWS: &[(&str, &str)] = &[(UI_INTRODUCTION_SKIP_CONTROL_ID, "escape"), (UI_INTRODUCTION_NEXT_CONTROL_ID, "enter,arrowright"), (UI_INTRODUCTION_BACK_CONTROL_ID, "arrowleft")];

/// ⌨️ The chord badge one introduction control wears — its user override first, then its React
/// default. `Next ↵`, exactly as `🗑️generated/react-6313/final.png` reads it.
fn introduction_control_hotkey(overrides: &HashMap<String, String>, control_id: &str) -> Option<String> {
    let keys = overrides.get(control_id).filter(|keys| keys.split(',').any(|chord| !chord.trim().is_empty())).map(String::as_str).or_else(|| INTRODUCTION_SHORTCUT_ROWS.iter().find(|(id, _)| *id == control_id).map(|(_, keys)| *keys))?;
    let badge = format_keybinding_shortcut(keys);
    (!badge.is_empty()).then_some(badge)
}

/// 🎓️ One chip of the tour card, in React's own reading order: the inline label, then the
/// `ControlHotkeyBadge` chord, then the icon — `ButtonGroupItem` renders exactly that sequence
/// (`🖱️ui/🧱️elements/🔳️ButtonGroup/🟦️.tsx:110-118`), which is why React's footer reads `Next ↵ ›`
/// with the chevron LAST. `leading_icon` is the one inversion React itself makes: `WindowChrome`'s
/// `close` control paints its `CloseIcon` before the label, so Skip reads `✕ Skip`.
struct ChromeTourChip {
    pub(crate) rect: Rect,
    pub(crate) label: &'static str,
    pub(crate) hotkey: Option<String>,
    pub(crate) icon_id: &'static str,
    pub(crate) leading_icon: bool,
    filled: bool,
}

/// 🎓️ Where one tour chip's leading or trailing icon sits — React's `ButtonGroupItem` puts the icon
/// LAST, after the label and the chord badge, and only `WindowChrome`'s `close` control leads with it.
pub(crate) fn chrome_tour_chip_icon_x(chip: &ChromeTourChip, theme: &Theme) -> f32 {
    if chip.leading_icon {
        chip.rect.x + theme.padding_standard * 2.0
    } else {
        chip.rect.x + chip.rect.w - theme.padding_standard * 2.0 - CHROME_ICON_TINY
    }
}

/// 🎓️ The `(x, baseline, width)` one tour chip's inline label flows in — `px-double` from the chip's
/// own edge, after a leading icon when it carries one.
pub(crate) fn chrome_tour_chip_label_box(chip: &ChromeTourChip, atlas: &mut FontAtlas, theme: &Theme) -> (f32, f32, f32) {
    let leading = if chip.leading_icon { CHROME_ICON_TINY + theme.gap_standard } else { 0.0 };
    let x = chip.rect.x + theme.padding_standard * 2.0 + leading;
    let width = atlas.measure_text(chip.label, theme.font_size_small).0.max(1.0);
    (x, chip.rect.y + (chip.rect.h + theme.font_size_small) * 0.5 - 1.0, width)
}

/// ⌨️ The chord badge box beside one tour chip's label — React's `ControlHotkeyBadge`, which sits
/// BETWEEN the inline label and the icon (`🖱️ui/🧱️elements/🔳️ButtonGroup/🟦️.tsx:110-118`).
pub(crate) fn chrome_tour_chip_hotkey_box(chip: &ChromeTourChip, atlas: &mut FontAtlas, theme: &Theme) -> Option<(String, (f32, f32, f32))> {
    let badge = chip.hotkey.clone()?;
    let (label_x, baseline, label_w) = chrome_tour_chip_label_box(chip, atlas, theme);
    let width = atlas.measure_text(&badge, theme.font_size_small).0.max(1.0);
    Some((badge, (label_x + label_w + theme.gap_standard, baseline, width)))
}

/// 🎓️ The hairline outline every tour chip and the card's own title chip wear — React's
/// `windowChromeTitleChipClass` box, four `--border-normal` strokes, the same idiom
/// [`render_retained_chrome_group_item_step`] uses for a navbar chip.
pub(crate) fn push_chrome_tour_chip_border(overlay: &mut DrawList, rect: Rect, theme: &Theme) {
    overlay.push_solid([rect.x, rect.y, rect.w, theme.stroke_hairline], theme.border_normal);
    overlay.push_solid([rect.x, rect.y + rect.h - theme.stroke_hairline, rect.w, theme.stroke_hairline], theme.border_normal);
    overlay.push_solid([rect.x, rect.y, theme.stroke_hairline, rect.h], theme.border_normal);
    overlay.push_solid([rect.x + rect.w - theme.stroke_hairline, rect.y, theme.stroke_hairline, rect.h], theme.border_normal);
}

/// 🎓️ One tour step's whole geometry, measured once per walk step so the card, the veil holes and the
/// hit targets can never disagree about where anything is.
pub(crate) struct ChromeTourLayout {
    pub(crate) card: Rect,
    pub(crate) title: String,
    pub(crate) title_chip: Rect,
    pub(crate) title_rect: Rect,
    pub(crate) grip: Rect,
    pub(crate) skip: ChromeTourChip,
    pub(crate) back: Option<ChromeTourChip>,
    pub(crate) next: Option<ChromeTourChip>,
    pub(crate) counter: Rect,
    pub(crate) counter_text: String,
    pub(crate) footer_rule: Rect,
    pub(crate) paragraphs: Vec<(String, Rect)>,
    pub(crate) checklist: Vec<(String, bool, Rect)>,
    pub(crate) cutouts: Vec<Rect>,
    pub(crate) veil_blocks_pointer: bool,
}

/// 🎓️ Measures one step into a [`ChromeTourLayout`] — React's own card, term for term: Skip in the cap
/// row beside the title, the body's blank-line paragraphs, the interaction checklist with its
/// check/circle bullets, and a footer of Back (from step 2 on) / `i / n` / Next-or-Done. The box is
/// sized to its copy and placed by `resolve_introduction_placement` against the step's own anchor.
///
/// 🩸️ What this replaces: a `Rect::new((width - 320.0) * 0.5, (height - 168.0) * 0.5, 320.0, 168.0)`
/// — a fixed box, always centred, which clipped any step whose copy outgrew 168px, ignored
/// `step.placement` entirely (`resolve_introduction_placement` was `#[cfg(test)]`, i.e. production-dead)
/// and painted neither the checklist nor the spotlight.
///
/// 🧯️ React's `veilBlocksPointer`: only a screen-style step (`introduce == null`) or one whose
/// target actually resolved owns every pointer — an unresolved target would otherwise trap the
/// user behind an opaque veil with no way to reveal it (`UIIntroduction`).
pub(crate) fn chrome_tour_layout(state: &ShellState, atlas: &mut FontAtlas, theme: &Theme, step: &semio_framework::IntroductionStepDefinition, step_index: usize, step_count: usize, width: f32, height: f32) -> ChromeTourLayout {
    let is_de = state.locale_id == "de";
    let is_last = step_index + 1 == step_count;
    let advances_by_button = step.interactions.is_empty();
    let shows_back = step_index > 0;
    let pad = theme.padding_standard * 2.0;
    let gap = theme.gap_standard;
    let body_gap = theme.gap_standard * 2.0;
    let cap_h = theme.control_height;
    let line_h = theme.font_size_small * 1.6;
    let overrides = &state.chrome_build.preferences.keybinding_overrides;
    let title = step.title.resolve(state.active_terminology(), state.active_locale()).to_string();
    let paragraph_texts = split_introduction_body_paragraphs(&step.body.resolve(state.active_terminology(), state.active_locale()));
    let checklist_texts: Vec<(String, bool)> = step
        .interactions
        .iter()
        .enumerate()
        .map(|(index, interaction)| {
            let done = state.chrome_build.tour_state.as_ref().is_some_and(|tour| tour.completed_interactions.contains(&index));
            let label = if step.ordered { format!("{}. {}", index + 1, interaction.label) } else { interaction.label.clone() };
            (label, done)
        })
        .collect();
    let skip_label = shell_chrome_string("introduction.skip", is_de);
    let back_label = shell_chrome_string("introduction.back", is_de);
    let next_label = shell_chrome_string(if is_last { "introduction.done" } else { "introduction.next" }, is_de);
    let next_hotkey = introduction_control_hotkey(overrides, UI_INTRODUCTION_NEXT_CONTROL_ID);
    let back_hotkey = introduction_control_hotkey(overrides, UI_INTRODUCTION_BACK_CONTROL_ID);
    let counter_text = format!("{} / {step_count}", step_index + 1);
    let chip_w = |atlas: &mut FontAtlas, label: &str, hotkey: Option<&String>, icons: usize| {
        let hotkey_w = hotkey.map_or(0.0, |badge| gap + atlas.measure_text(badge, theme.font_size_small).0);
        theme.padding_standard * 4.0 + atlas.measure_text(label, theme.font_size_small).0 + hotkey_w + icons as f32 * (gap + CHROME_ICON_TINY)
    };
    let skip_w = chip_w(atlas, skip_label, None, 1);
    let back_w = chip_w(atlas, back_label, back_hotkey.as_ref(), 1);
    let next_w = chip_w(atlas, next_label, next_hotkey.as_ref(), 1);
    let counter_w = chip_w(atlas, &counter_text, None, 0);
    let title_chip_w = theme.padding_standard * 4.0 + atlas.measure_text(&title, theme.font_size_body).0 + gap + CHROME_ICON_TINY;
    let footer_w = if shows_back { back_w + gap } else { 0.0 } + counter_w + gap + if advances_by_button { next_w } else { 0.0 };
    let mut content_w = (title_chip_w + gap + skip_w).max(footer_w);
    for paragraph in &paragraph_texts {
        content_w = content_w.max(atlas.measure_text(paragraph, theme.font_size_small).0);
    }
    for (label, _) in &checklist_texts {
        content_w = content_w.max(theme.font_size_small + gap + atlas.measure_text(label, theme.font_size_small).0);
    }
    let content_w = content_w.clamp(INTRODUCTION_BOX_MIN_WIDTH, (INTRODUCTION_BOX_MAX_WIDTH - pad * 2.0).max(INTRODUCTION_BOX_MIN_WIDTH));
    let paragraph_heights: Vec<f32> = paragraph_texts.iter().map(|paragraph| atlas.measure_text_wrapped(paragraph, content_w, theme.font_size_small).1.max(line_h)).collect();
    let body_h = paragraph_heights.iter().sum::<f32>() + body_gap * paragraph_heights.len().saturating_sub(1) as f32;
    let checklist_h = line_h * checklist_texts.len() as f32;
    let card_w = content_w + pad * 2.0;
    let card_h = pad * 2.0 + cap_h + if body_h > 0.0 { body_gap + body_h } else { 0.0 } + if checklist_h > 0.0 { body_gap + checklist_h } else { 0.0 } + body_gap + theme.control_height;
    let mut cutouts: Vec<Rect> = Vec::new();
    let anchor = step.introduce.as_deref().and_then(|id| chrome_tour_anchor_rect(state, id));
    if let Some(anchor) = anchor {
        cutouts.push(anchor);
    }
    for id in &step.show {
        if let Some(rect) = chrome_tour_anchor_rect(state, id) {
            cutouts.push(rect);
        }
    }
    let veil_blocks_pointer = step.introduce.is_none() || anchor.is_some() || !cutouts.is_empty();
    let (left, top) = resolve_introduction_placement(step.placement, anchor, (card_w, card_h), (width, height));
    let card = Rect::new(left, top, card_w, card_h);
    let cap_y = card.y + pad;
    let skip_rect = Rect::new(card.x + card.w - pad - skip_w, cap_y, skip_w, cap_h);
    let title_chip = Rect::new(card.x + pad, cap_y, title_chip_w.min((skip_rect.x - gap - card.x - pad).max(1.0)), cap_h);
    let grip = Rect::new(title_chip.x + title_chip.w - theme.padding_standard * 2.0 - CHROME_ICON_TINY, title_chip.y + (title_chip.h - CHROME_ICON_TINY) * 0.5, CHROME_ICON_TINY, CHROME_ICON_TINY);
    let title_rect = Rect::new(title_chip.x + theme.padding_standard * 2.0, cap_y, (grip.x - gap - title_chip.x - theme.padding_standard * 2.0).max(1.0), cap_h);
    let mut flow_y = cap_y + cap_h + if body_h > 0.0 { body_gap } else { 0.0 };
    let mut paragraphs = Vec::new();
    for (paragraph, paragraph_h) in paragraph_texts.into_iter().zip(paragraph_heights) {
        paragraphs.push((paragraph, Rect::new(card.x + pad, flow_y, content_w, paragraph_h)));
        flow_y += paragraph_h + body_gap;
    }
    if !paragraphs.is_empty() {
        flow_y -= body_gap;
    }
    if checklist_h > 0.0 {
        flow_y += body_gap;
    }
    let mut checklist = Vec::new();
    for (label, done) in checklist_texts {
        checklist.push((label, done, Rect::new(card.x + pad, flow_y, content_w, line_h)));
        flow_y += line_h;
    }
    let footer_y = card.y + card.h - pad - theme.control_height;
    ChromeTourLayout {
        card,
        title,
        title_chip,
        title_rect,
        grip,
        skip: ChromeTourChip { rect: skip_rect, label: skip_label, hotkey: None, icon_id: "x", leading_icon: true, filled: false },
        back: shows_back.then(|| ChromeTourChip { rect: Rect::new(card.x + pad, footer_y, back_w, theme.control_height), label: back_label, hotkey: back_hotkey, icon_id: "chevron-left", leading_icon: false, filled: false }),
        next: advances_by_button.then(|| ChromeTourChip {
            rect: Rect::new(card.x + card.w - pad - next_w, footer_y, next_w, theme.control_height),
            label: next_label,
            hotkey: next_hotkey,
            icon_id: if is_last { "check" } else { "chevron-right" },
            leading_icon: false,
            filled: true,
        }),
        counter: Rect::new(card.x + (card.w - counter_w) * 0.5, footer_y, counter_w, theme.control_height),
        counter_text,
        footer_rule: Rect::new(card.x, footer_y - theme.padding_standard, card.w, theme.stroke_hairline),
        paragraphs,
        checklist,
        cutouts,
        veil_blocks_pointer,
    }
}

/// ⌨️ Ports `engagementInlineCompletion`/`engagementCompletionSuffix` (`ui/js/react/index.tsx`) to this
/// renderer: the first `possible` (in the order the host already gave them — wgpu's engagement rail has
/// no ranked-match dropdown to reorder by) whose label case-insensitively prefix-matches `query`, sliced
/// on a char boundary (never a byte index) so a multi-byte label can't panic.
#[cfg(test)]
pub(crate) fn engagement_completion_suffix(query: &str, possibles: Option<&[ui_wgpu::wgpu::WindowEngagementPossible]>) -> String {
    let query = query.trim();
    if query.is_empty() {
        return String::new();
    }
    let Some(possibles) = possibles else {
        return String::new();
    };
    let query_lower = query.to_lowercase();
    for possible in possibles {
        let label_lower = possible.label.to_lowercase();
        if !label_lower.starts_with(&query_lower) {
            continue;
        }
        let query_chars = query.chars().count();
        let split_at = possible.label.char_indices().nth(query_chars).map(|(byte_index, _)| byte_index).unwrap_or(possible.label.len());
        let suffix = &possible.label[split_at..];
        if !suffix.is_empty() {
            return suffix.to_string();
        }
    }
    String::new()
}

/// 👻️ Pure accept-decision for the ghost-text click affordance — factored out of
/// `render_engagement_input` so it's unit-testable without a `GpuContext` fixture (that function
/// unconditionally needs a real wgpu device, per its own `_gpu` parameter).
#[cfg(test)]
pub(crate) fn engagement_ghost_accept_on_click(ghost_rect: Rect, pointer_x: f32, pointer_y: f32, clicked_this_frame: bool, query: &str, suffix: &str) -> Option<String> {
    if clicked_this_frame && ghost_rect.contains(pointer_x, pointer_y) {
        Some(format!("{query}{suffix}"))
    } else {
        None
    }
}
//#endregion 🔖️ChromeOverlaysAndTour
//#endregion ShellChrome
