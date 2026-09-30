//! ⏪️ The wgpu shell's time-travel chrome (ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING, packet W2-C).
//!
//! `HistoryPatch.timeTravel` is the whole input. While it is present the shell paints one persistent band above the
//! footer (stage, the edited mutation, replay progress with Cancel, the worst outcome in words, what blocks finalizing,
//! the accepted drafts, the last fault and the controls the stage offers), an indicator on every window pane saying
//! the pane shows time-travel state, and answers the remappable `ui.timeTravel.*` chords. Every control dispatches the
//! framework's reserved `historyEdit*` verb on the session controller, stamped with the session generation it was shown,
//! so a stale press is refused `timeTravel.stale` instead of acting on a session that moved on. Nothing here decides a
//! stage — the plugin runtime does. Escape is never a time-travel verb, so it never discards.
//!
//! The copy, the controls and the indicator are React's (`🛠️ShellHelpers/⏪️time-travel/🟦️.tsx`), pinned for both
//! shells by the language-neutral corpus `🛠️ShellHelpers/🧫️fixtures/🧫️time-travel-band/🔣️.json`. The history-edit
//! refusals (the hub's `history.*`, the session's `timeTravel.*`) are localized here too.

use super::*;
use semio_framework::kernel::{HistoryTimeTravel, HistoryTimeTravelReview, HistoryTimeTravelStage};
use semio_framework_time_travel::TimeTravelLabel;

//#region ⏪️TimeTravelControls
/// 🆔️ The band's live status node: every stage and progress change is announced through it.
pub(super) const TIME_TRAVEL_BAND_STATUS_ID: &str = "shell.time-travel.status";

/// 📐️ Gap between the band and the footer it floats above.
const TIME_TRAVEL_BAND_GAP: f32 = 8.0;

/// 📐️ The band's widest extent before its message is clipped.
const TIME_TRAVEL_BAND_MAX_WIDTH: f32 = 880.0;

/// 📐️ Height of the replay progress track along the band's lower edge.
const TIME_TRAVEL_PROGRESS_TRACK: f32 = 3.0;

/// ⏱️ How often the native shell re-reads the history while the runtime replays or finalizes: the replay advances in
/// reactor turns that answer no dispatch, so the band's progress is polled rather than pushed.
#[cfg(not(target_arch = "wasm32"))]
pub(super) const TIME_TRAVEL_POLL_MS: f64 = 150.0;

/// ⏪️ One control of the band — a reserved history-edit verb the band, the chords and the keyboard ring can ask for.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum TimeTravelVerb {
    Accept,
    Discard,
    CancelReplay,
    Rerun,
    Finalize,
    Back,
    Exit,
}

impl TimeTravelVerb {
    /// ⌨️ The verbs with a remappable chord, in `SHELL_SHORTCUT_ROWS` order.
    pub(crate) const SHORTCUTS: [Self; 3] = [Self::Accept, Self::Discard, Self::Exit];

    /// 🎬️ The reserved framework action this verb dispatches on the session controller — React's `TIME_TRAVEL_VERBS`.
    pub(crate) fn action_id(self) -> &'static str {
        match self {
            Self::Accept => semio_framework::HISTORY_EDIT_ACCEPT_ACTION_ID,
            Self::Discard => semio_framework::HISTORY_EDIT_DISCARD_ACTION_ID,
            Self::CancelReplay => semio_framework::HISTORY_EDIT_CANCEL_REPLAY_ACTION_ID,
            Self::Rerun => semio_framework::HISTORY_EDIT_RERUN_ACTION_ID,
            Self::Finalize => semio_framework::HISTORY_EDIT_FINALIZE_ACTION_ID,
            Self::Back => semio_framework::HISTORY_EDIT_BACK_ACTION_ID,
            Self::Exit => semio_framework::HISTORY_EDIT_EXIT_ACTION_ID,
        }
    }

    /// 🆔️ The band control's id. A verb with a chord uses its keybinding row id (React's `TIME_TRAVEL_CHORD_IDS`), so
    /// the chord badge, the Keybindings settings row and `aria-keyshortcuts` all resolve from the one remappable table.
    pub(crate) fn control_id(self) -> &'static str {
        match self {
            Self::Accept => "ui.timeTravel.accept",
            Self::Discard => "ui.timeTravel.discard",
            Self::CancelReplay => "shell.time-travel.cancel-replay",
            Self::Rerun => "shell.time-travel.rerun",
            Self::Finalize => "shell.time-travel.finalize",
            Self::Back => "shell.time-travel.back",
            Self::Exit => "ui.timeTravel.exit",
        }
    }

    /// ⌨️ The verb a keybinding row id names.
    pub(crate) fn from_shortcut_id(control_id: &str) -> Option<Self> {
        Self::SHORTCUTS.into_iter().find(|verb| verb.control_id() == control_id)
    }

    /// 🗣️ The button's caption — React's `ui.timeTravel.<control>` normal labels; Rerun reads the session's own
    /// `ActionRerun` label.
    pub(crate) fn label(self, locale: Locale) -> &'static str {
        match (self, locale) {
            (Self::Rerun, locale) => time_travel_label(TimeTravelLabel::ActionRerun, locale),
            (Self::Accept, Locale::En) => "Accept draft",
            (Self::Accept, Locale::De) => "Entwurf übernehmen",
            (Self::Discard, Locale::En) => "Discard draft",
            (Self::Discard, Locale::De) => "Entwurf verwerfen",
            (Self::CancelReplay, Locale::En) => "Cancel replay",
            (Self::CancelReplay, Locale::De) => "Neuanwendung abbrechen",
            (Self::Finalize, Locale::En) => "Finalize…",
            (Self::Finalize, Locale::De) => "Abschließen…",
            (Self::Back, Locale::En) => "Back",
            (Self::Back, Locale::De) => "Zurück",
            (Self::Exit, Locale::En) => "Exit time travel",
            (Self::Exit, Locale::De) => "Zeitreise beenden",
        }
    }

    /// 🎯️ The dispatch this verb sends for `status` on the program `controller_id` names, carrying the generation the
    /// band showed — React's `timeTravelControlActionV1`.
    pub(crate) fn action(self, controller_id: &str, status: &HistoryTimeTravel) -> ActionDescriptor {
        ActionDescriptor { controller_id: controller_id.to_string(), action: self.action_id().to_string(), args: crate::action_args_json!({ "generation": status.generation }) }
    }
}

/// 🔘️ One band control: its verb and the session refusal that disables it (`None` when it is enabled).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct TimeTravelControl {
    pub verb: TimeTravelVerb,
    pub disabled_by: Option<TimeTravelLabel>,
}

/// 🚦️ The controls a stage offers, in band order — React's `timeTravelBandControlsV1`: a draft is accepted or
/// discarded while editing, a replay can be cancelled while it runs, a review can replay again only when the session
/// says it is `rerunnable`, and finalize only when its `review` is `ready` (else Finalize stays visible and names what
/// blocks it — never inferred from a missing review), the finalize prompt can be left for the review, and every stage
/// but the commit itself can exit.
pub(crate) fn time_travel_band_controls(status: &HistoryTimeTravel) -> Vec<TimeTravelControl> {
    use TimeTravelVerb as Verb;
    let offer = |verb| TimeTravelControl { verb, disabled_by: None };
    match status.stage {
        HistoryTimeTravelStage::Editing => vec![offer(Verb::Accept), offer(Verb::Discard), offer(Verb::Exit)],
        HistoryTimeTravelStage::Replaying => vec![offer(Verb::CancelReplay), offer(Verb::Exit)],
        HistoryTimeTravelStage::Reviewing => {
            let rerun = (!status.rerunnable).then_some(TimeTravelLabel::RefusalIllegal);
            let finalize = match status.review {
                _ if status.review == Some(HistoryTimeTravelReview::NoChanges) || status.accepted_count == 0 => Some(TimeTravelLabel::RefusalEmpty),
                Some(HistoryTimeTravelReview::NeedsReplay | HistoryTimeTravelReview::Blocked) => Some(TimeTravelLabel::RefusalBlocked),
                _ if status.blocking => Some(TimeTravelLabel::RefusalBlocked),
                Some(HistoryTimeTravelReview::Ready) => None,
                _ => Some(TimeTravelLabel::RefusalIllegal),
            };
            vec![TimeTravelControl { verb: Verb::Rerun, disabled_by: rerun }, TimeTravelControl { verb: Verb::Finalize, disabled_by: finalize }, offer(Verb::Exit)]
        }
        HistoryTimeTravelStage::Choosing => vec![offer(Verb::Back), offer(Verb::Exit)],
        HistoryTimeTravelStage::Finalizing => Vec::new(),
    }
}

/// ✅️ Whether `verb` is offered and enabled in the session's stage — the one gate the band and the chords share.
pub(crate) fn time_travel_verb_enabled(status: &HistoryTimeTravel, verb: TimeTravelVerb) -> bool {
    time_travel_band_controls(status).into_iter().any(|control| control.verb == verb && control.disabled_by.is_none())
}

/// ✍️ Whether a time-travel chord yields to the focused retained control: a text or number field keeps its keys,
/// because hotkeys never fire while the user is typing (React's `isEditableEventTarget`).
pub(crate) fn time_travel_chord_yields_to(focus: Option<RetainedNodeFocusKind>) -> bool {
    focus == Some(RetainedNodeFocusKind::Input)
}
//#endregion ⏪️TimeTravelControls

//#region 🗣️TimeTravelCopy
/// 🌐️ One `TimeTravelLabel` in `locale`.
pub(crate) fn time_travel_label(label: TimeTravelLabel, locale: Locale) -> &'static str {
    label.localized(|en, de| match locale {
        Locale::En => en,
        Locale::De => de,
    })
}

/// 🏷️ The shared label of a wire stage.
fn time_travel_stage_label(stage: HistoryTimeTravelStage) -> TimeTravelLabel {
    match stage {
        HistoryTimeTravelStage::Editing => TimeTravelLabel::StageEditing,
        HistoryTimeTravelStage::Replaying => TimeTravelLabel::StageReplaying,
        HistoryTimeTravelStage::Reviewing => TimeTravelLabel::StageReviewing,
        HistoryTimeTravelStage::Choosing => TimeTravelLabel::StageChoosing,
        HistoryTimeTravelStage::Finalizing => TimeTravelLabel::StageFinalizing,
    }
}

/// 🚦️ A severity's word — React's `ui.mutation.level.*` normal labels, term for term.
pub(crate) fn time_travel_severity_text(severity: semio_framework::Severity, locale: Locale) -> &'static str {
    use semio_framework::Severity;
    match (severity, locale) {
        (Severity::Info, Locale::En) => "Info",
        (Severity::Info, Locale::De) => "Info",
        (Severity::Warning, Locale::En) => "Warning",
        (Severity::Warning, Locale::De) => "Warnung",
        (Severity::Error, Locale::En) => "Error",
        (Severity::Error, Locale::De) => "Fehler",
        (Severity::Fatal, Locale::En) => "Fatal",
        (Severity::Fatal, Locale::De) => "Kritisch",
    }
}

/// 🛑️ The hub's history refusal codes and their notices — byte-identical to React's `ui.history.refusal.*`.
const HISTORY_REFUSALS: [(&str, &str, &str); 3] = [
    ("history.malformed-transition", "History edit refused: the change could not be read.", "Verlaufsbearbeitung abgelehnt: Die Änderung konnte nicht gelesen werden."),
    ("history.unknown-target", "History edit refused: the edited mutation no longer exists.", "Verlaufsbearbeitung abgelehnt: Die bearbeitete Mutation existiert nicht mehr."),
    ("history.transition-refused", "The hub refused a history edit; the document is rebuilt from the hub.", "Der Hub hat eine Verlaufsbearbeitung abgelehnt; das Dokument wird vom Hub neu aufgebaut."),
];

/// 🛑️ A live session's refusal and fault codes — React's `ui.timeTravel.refusal.*`.
const SESSION_REFUSALS: [&str; 7] = ["timeTravel.frozen", "timeTravel.illegal", "timeTravel.stale", "timeTravel.blocked", "timeTravel.empty", "timeTravel.cancelled", "timeTravel.name-invalid"];

/// 🗣️ One session refusal's text: the shared `⏪️time-travel` label where it has one (a cancelled replay through
/// `TimeTravelLabel::for_fault`), else the shell's own copy.
fn session_refusal_text(code: &str, locale: Locale) -> Option<&'static str> {
    let label = match code {
        "timeTravel.frozen" => TimeTravelLabel::Frozen,
        "timeTravel.illegal" => TimeTravelLabel::RefusalIllegal,
        "timeTravel.stale" => TimeTravelLabel::RefusalStale,
        "timeTravel.blocked" => TimeTravelLabel::RefusalBlocked,
        "timeTravel.empty" => TimeTravelLabel::RefusalEmpty,
        "timeTravel.name-invalid" => {
            return Some(match locale {
                Locale::En => "Invalid alternative name: use 1 to 256 characters",
                Locale::De => "Ungültiger Name der Alternative: 1 bis 256 Zeichen verwenden",
            })
        }
        other => TimeTravelLabel::for_fault(other)?,
    };
    Some(time_travel_label(label, locale))
}

/// 🛑️ The history-edit refusal one machine `code` is, as `(code, message, severity)` in `locale` — React's
/// `historyRefusalCodeV1` over `HISTORY_REFUSAL_LABEL_KEYS`: a hub `history.*` transition refusal is an error, a session
/// `timeTravel.*` refusal a warning (the verb was refused, nothing was lost). Only an exact code matches; prose never does.
pub(crate) fn history_refusal_notice(code: &str, locale: Locale) -> Option<(&'static str, &'static str, semio_framework::Severity)> {
    let hub = HISTORY_REFUSALS.iter().find(|(known, _, _)| *known == code).map(|(known, en, de)| {
        (
            *known,
            match locale {
                Locale::En => *en,
                Locale::De => *de,
            },
            semio_framework::Severity::Error,
        )
    });
    hub.or_else(|| SESSION_REFUSALS.into_iter().find(|known| *known == code).and_then(|known| Some((known, session_refusal_text(known, locale)?, semio_framework::Severity::Warning))))
}

/// 🔎️ The history-edit refusal a dispatch-fault string carries — React's `historyRefusalOfFaultV1`. The funnel's string is
/// `code: message`, then ` — code: message [target]; …` for the report's messages (the browser prefixes its bridge
/// call), so the fault's own code comes first and each report code after it; each is read as one whole token.
pub(crate) fn history_refusal_of_fault(fault: &str, locale: Locale) -> Option<(&'static str, &'static str, semio_framework::Severity)> {
    fault.split(|c: char| c.is_whitespace() || matches!(c, ':' | ';' | ',' | '[' | ']' | '(' | ')')).find_map(|token| history_refusal_notice(token, locale))
}

/// 📝️ The band's lines in one locale; a line the session does not carry is `None` — React's `TimeTravelBandTextV1`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct TimeTravelBandLines {
    pub stage: String,
    pub target: Option<String>,
    pub progress: Option<String>,
    pub review: Option<String>,
    pub outcome: Option<String>,
    pub fault: Option<String>,
    pub accepted: Option<String>,
}

impl TimeTravelBandLines {
    /// 🧵️ The lines in the band's reading order — stage, edited mutation, progress, review, worst outcome, fault,
    /// accepted drafts — joined into the one message the band paints and its live node speaks.
    pub(crate) fn message(&self) -> String {
        [Some(&self.stage), self.target.as_ref(), self.progress.as_ref(), self.review.as_ref(), self.outcome.as_ref(), self.fault.as_ref(), self.accepted.as_ref()].into_iter().flatten().cloned().collect::<Vec<_>>().join(" · ")
    }
}

/// 🔭️ The shared label of what a review shows — `TimeTravelReview::label` of the `⏪️time-travel` module.
fn time_travel_review_label(review: HistoryTimeTravelReview) -> TimeTravelLabel {
    match review {
        HistoryTimeTravelReview::NoChanges => TimeTravelLabel::NoChanges,
        HistoryTimeTravelReview::NeedsReplay => TimeTravelLabel::NeedsReplay,
        HistoryTimeTravelReview::Blocked => TimeTravelLabel::ReportBlocking,
        HistoryTimeTravelReview::Ready => TimeTravelLabel::ReadyToFinalize,
    }
}

/// 🏷️ The edited mutation's label on the shell's axes, when the session names one.
fn time_travel_target_text(status: &HistoryTimeTravel, terminology: Terminology, locale: Locale) -> Option<String> {
    status.target_label.as_ref().map(|label| label.resolve(terminology, locale).to_string()).filter(|target| !target.is_empty())
}

/// 📝️ [`TimeTravelBandLines`] of `status` — React's `timeTravelBandTextV1`: a review reads what the session states
/// (`review`), never what a missing report might mean; severity is always named in words, never by colour alone; and a
/// fault code the shell knows reads as its localized refusal.
pub(crate) fn time_travel_band_lines(status: &HistoryTimeTravel, terminology: Terminology, locale: Locale) -> TimeTravelBandLines {
    let en = locale == Locale::En;
    TimeTravelBandLines {
        stage: time_travel_label(time_travel_stage_label(status.stage), locale).to_string(),
        target: time_travel_target_text(status, terminology, locale).map(|target| if en { format!("Editing: {target}") } else { format!("Bearbeitet: {target}") }),
        progress: (status.stage == HistoryTimeTravelStage::Replaying).then_some(status.total).flatten().map(|total| time_travel_label(TimeTravelLabel::ReplayProgressValueText, locale).replace("{done}", &status.done.unwrap_or(0).to_string()).replace("{total}", &total.to_string())),
        review: status.review.filter(|_| status.stage == HistoryTimeTravelStage::Reviewing).map(|review| time_travel_label(time_travel_review_label(review), locale).to_string()),
        outcome: status.worst.map(|worst| if en { format!("Worst outcome: {}", time_travel_severity_text(worst, locale)) } else { format!("Schwerstes Ergebnis: {}", time_travel_severity_text(worst, locale)) }),
        fault: status.fault.as_deref().map(|fault| match history_refusal_notice(fault, locale) {
            Some((code, message, _)) if code == fault => message.to_string(),
            _ if en => format!("Replay failed ({fault})"),
            _ => format!("Neuanwendung fehlgeschlagen ({fault})"),
        }),
        accepted: (status.accepted_count > 0).then(|| if en { format!("Accepted changes: {}", status.accepted_count) } else { format!("Übernommene Änderungen: {}", status.accepted_count) }),
    }
}

/// 🏷️ What a window of the editing program says about itself — React's `timeTravelIndicatorTextV1`: the document
/// before the edited mutation while a draft is edited, else the session's stage.
pub(crate) fn time_travel_indicator_text(status: &HistoryTimeTravel, terminology: Terminology, locale: Locale) -> String {
    match (status.stage, time_travel_target_text(status, terminology, locale)) {
        (HistoryTimeTravelStage::Editing, Some(target)) => match locale {
            Locale::En => format!("Time travel: document before {target}"),
            Locale::De => format!("Zeitreise: Dokument vor {target}"),
        },
        (stage, _) => time_travel_label(time_travel_stage_label(stage), locale).to_string(),
    }
}

/// 🏷️ The indicator chip's visible word — React's `ui.timeTravel.indicator`; its accessible name is
/// [`time_travel_indicator_text`].
pub(crate) fn time_travel_indicator_caption(locale: Locale) -> &'static str {
    match locale {
        Locale::En => "Time travel",
        Locale::De => "Zeitreise",
    }
}

/// 🎨️ `(border, fill, text)` of the band — the shell banner severity map: a blocking report is an error; a fault, a
/// review that needs a replay or a worst outcome of warning a warning; everything else the neutral popover chrome.
pub(crate) fn time_travel_band_tone(status: &HistoryTimeTravel, theme: &Theme) -> (Rgba, Rgba, Rgba) {
    use semio_framework::Severity;
    let severity = if status.blocking || status.review == Some(HistoryTimeTravelReview::Blocked) {
        Severity::Error
    } else if status.fault.is_some() || status.review == Some(HistoryTimeTravelReview::NeedsReplay) || status.worst == Some(Severity::Warning) {
        Severity::Warning
    } else {
        Severity::Info
    };
    transient_notice_tone(severity, theme)
}
//#endregion 🗣️TimeTravelCopy

//#region 📐️TimeTravelBand
/// 🔘️ One laid-out band button.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct TimeTravelBandButton {
    pub control: TimeTravelControl,
    pub label: String,
    pub rect: Rect,
}

/// 📐️ The band laid out for one frame: its box, the message and the width it may take, the replay progress track
/// with its filled share, and the buttons right-aligned in stage order.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct TimeTravelBandPlan {
    pub band: Rect,
    pub message: String,
    pub message_x: f32,
    pub message_w: f32,
    pub progress: Option<(Rect, f32)>,
    pub buttons: Vec<TimeTravelBandButton>,
}

/// 📐️ Lays the band out bottom-centre, `TIME_TRAVEL_BAND_GAP` above the footer — away from the transient notice
/// stack at the top, so a notice raised while time travelling never covers it. Widths use the same monospace estimate
/// every shell band uses; the renderer measures glyphs only while painting them.
pub(crate) fn time_travel_band_plan(status: &HistoryTimeTravel, message: String, buttons: Vec<(TimeTravelControl, String)>, width: f32, height: f32, theme: &Theme) -> TimeTravelBandPlan {
    let (pad, gap, small) = (theme.padding_standard, theme.gap_standard, theme.font_size_small);
    let glyph_w = |text: &str| text.chars().count() as f32 * small * 0.6;
    let button_w = |label: &str| glyph_w(label) + pad * 2.0;
    let buttons_w: f32 = buttons.iter().map(|(_, label)| button_w(label) + gap).sum();
    let band_w = (pad * 2.0 + glyph_w(&message) + gap + buttons_w).min(TIME_TRAVEL_BAND_MAX_WIDTH).min((width - pad * 2.0).max(1.0));
    let band_h = small * 1.6 + pad * 2.0;
    let band = Rect::new(((width - band_w) * 0.5).max(0.0), (height - theme.footer_height - TIME_TRAVEL_BAND_GAP - band_h).max(0.0), band_w, band_h);
    let mut right = band.x + band.w - pad;
    let mut laid: Vec<TimeTravelBandButton> = buttons
        .into_iter()
        .rev()
        .map(|(control, label)| {
            let w = button_w(&label);
            right -= w;
            let rect = Rect::new(right, band.y + pad * 0.5, w, band.h - pad);
            right -= gap;
            TimeTravelBandButton { control, label, rect }
        })
        .collect();
    laid.reverse();
    let message_x = band.x + pad;
    let message_w = (right - message_x).max(1.0);
    let progress = match (status.stage, status.total) {
        (HistoryTimeTravelStage::Replaying, Some(total)) if total > 0 => {
            Some((Rect::new(band.x + pad, band.y + band.h - TIME_TRAVEL_PROGRESS_TRACK - 1.0, band.w - pad * 2.0, TIME_TRAVEL_PROGRESS_TRACK), (status.done.unwrap_or(0) as f32 / total as f32).clamp(0.0, 1.0)))
        }
        _ => None,
    };
    TimeTravelBandPlan { band, message, message_x, message_w, progress, buttons: laid }
}
//#endregion 📐️TimeTravelBand

//#region 🪟️TimeTravelIndicator
/// 🆔️ One pane's time-travel indicator — under React's `framework.window.<id>` parent like every pane chip.
pub(crate) fn time_travel_indicator_control_id(window_id: &str) -> String {
    semio_framework::child_element_id(WINDOW_PANE_CHIP_PARENT, &[window_id, "timeTravel", "indicator"])
}
//#endregion 🪟️TimeTravelIndicator

//#region 👥️PeerHistoryEdits
/// ⏪️ The badge a peer's roster row wears while that peer edits the history — React's `timeTravelPeerPresenceV1`.
pub(crate) const TIME_TRAVEL_PEER_BADGE: &str = "⏪";

/// 🗝️ The node keys the framework history body gives a history row and a mutation child (`🔌️plugin/🦀️.rs`
/// `ui_history_panel`), which a peer's open history edit marks — React's `HISTORY_ROW_KEY_PREFIX` and
/// `HISTORY_MUTATION_ROW_KEY_PREFIX`.
pub(crate) const HISTORY_ROW_KEY_PREFIX: &str = "framework.history.entry.";
pub(crate) const HISTORY_MUTATION_ROW_KEY_PREFIX: &str = "framework.history.mutation.";

/// 🗣️ `ui.timeTravel.peer.editingTarget`, or `.editingHistory` without a target.
fn peer_editing_text(name: &str, target: Option<&str>, locale: Locale) -> String {
    match (target, locale) {
        (Some(target), Locale::En) => format!("{name} is editing {target} in time travel"),
        (Some(target), Locale::De) => format!("{name} bearbeitet {target} in der Zeitreise"),
        (None, Locale::En) => format!("{name} is editing the history in time travel"),
        (None, Locale::De) => format!("{name} bearbeitet den Verlauf in der Zeitreise"),
    }
}

/// 🗣️ `ui.timeTravel.peer.editingRow`.
fn peer_editing_row_text(name: &str, locale: Locale) -> String {
    match locale {
        Locale::En => format!("{name} is editing this in time travel"),
        Locale::De => format!("{name} bearbeitet dies in der Zeitreise"),
    }
}

/// 👥️ What the shell shows about peers' open history edits: each editing peer's roster activity by actor, and the
/// notes by history-body node key, sorted by key with one key's lines joined by ` · `.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct TimeTravelPeerPresence {
    pub activities: Vec<(String, ui_wgpu::wgpu::PresenceActivity)>,
    pub notes: Vec<(String, String)>,
}

/// 👥️ Peers' open history edits labelled from THIS replica's own history rows — React's `timeTravelPeerPresenceV1`
/// (the wire carries only the mutation id, the stage and the draft count, never locale text). `editing` is each editing
/// peer as `(actor, name, mutation id)`: its chip reads "Ada is editing Drag selection in time travel" (the history in
/// general when the mutation is not among the local rows), and the mutation's node and its history row get a note
/// naming who edits them.
pub(crate) fn time_travel_peer_presence<'a>(editing: impl IntoIterator<Item = (&'a str, &'a str, &'a str)>, entries: &BTreeMap<String, semio_framework::kernel::HistoryEntry>, terminology: Terminology, locale: Locale) -> TimeTravelPeerPresence {
    let mut notes: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let activities = editing
        .into_iter()
        .map(|(actor, name, mutation_id)| {
            let row = entries.values().find(|entry| entry.mutations.iter().any(|mutation| mutation.mutation_id == mutation_id));
            let target = row.and_then(|entry| entry.mutations.iter().find(|mutation| mutation.mutation_id == mutation_id)).map(|mutation| mutation.label.resolve(terminology, locale).to_string()).filter(|target| !target.is_empty());
            let note = peer_editing_row_text(name, locale);
            if let Some(entry) = row {
                notes.entry(format!("{HISTORY_ROW_KEY_PREFIX}{}", entry.seq)).or_default().push(note.clone());
            }
            notes.entry(format!("{HISTORY_MUTATION_ROW_KEY_PREFIX}{mutation_id}")).or_default().push(note);
            (actor.to_string(), ui_wgpu::wgpu::PresenceActivity { text: peer_editing_text(name, target.as_deref(), locale), badge: TIME_TRAVEL_PEER_BADGE.to_string() })
        })
        .collect();
    TimeTravelPeerPresence { activities, notes: notes.into_iter().map(|(key, lines)| (key, lines.join(" · "))).collect() }
}
//#endregion 👥️PeerHistoryEdits

impl ShellState {
    /// ⏪️ Takes the session status one history patch carries (absent = no session). The edge into a session reveals
    /// the History panel tab, whose Rust-built body holds the editor — the twin of the tool-run panel reveal.
    pub(crate) fn observe_history_time_travel(&mut self, time_travel: Option<&HistoryTimeTravel>) {
        let began = self.history_time_travel.is_none() && time_travel.is_some();
        self.history_time_travel = time_travel.cloned();
        if began {
            self.reveal_dock_tab(FRAMEWORK_PANEL_TAB_HISTORY_ID);
        }
    }

    /// ⏪️ The live session status, if a history edit is open.
    pub fn history_time_travel(&self) -> Option<&HistoryTimeTravel> {
        self.history_time_travel.as_ref()
    }

    /// 👥️ The open history edits of the peers on the roster this shell paints (the attached surface's, like
    /// `footer_presence_rows`), labelled from this replica's history rows on the shell's axes.
    pub(crate) fn peer_time_travel_presence(&self) -> TimeTravelPeerPresence {
        let Some(surface) = self.presence_surface.as_deref() else { return TimeTravelPeerPresence::default() };
        let editing = self.presence_peers.iter().filter(|peer| peer.surface.as_deref() == Some(surface)).filter_map(|peer| Some((peer.actor.as_str(), peer.label.as_deref().unwrap_or(peer.actor.as_str()), peer.history_edit.as_ref()?.mutation_id.as_str())));
        time_travel_peer_presence(editing, &self.history_entries, self.active_terminology(), self.active_locale())
    }

    /// 👥️ Hands the retained history body the peers' row notes — React's shell-root presence overlay. Runs every frame;
    /// with no peer editing it is one empty comparison per window. `true` when a window's notes changed.
    pub(crate) fn publish_peer_time_travel_notes(&self) -> bool {
        crate::interpreter::set_ui_presence_notes(&self.peer_time_travel_presence().notes)
    }

    /// ⏱️ While the runtime replays or finalizes, re-reads the history at most every [`TIME_TRAVEL_POLL_MS`] and folds the
    /// snapshot — rows, cursor and session status — so a reply older than it can no longer roll the band back. The
    /// runtime also pushes throttled progress patches on uncorrelated `AppFrame::Invocation` frames, which reach
    /// [`ShellState::observe_invocation_history`] with the next exchange; this poll is what moves the band between two
    /// exchanges. `true` when the status changed.
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) async fn poll_time_travel_progress(&mut self) -> bool {
        let polling = self.history_time_travel.as_ref().is_some_and(|status| matches!(status.stage, HistoryTimeTravelStage::Replaying | HistoryTimeTravelStage::Finalizing));
        let now = chrome_now_ms();
        if !polling || now - self.time_travel_polled_at_ms < TIME_TRAVEL_POLL_MS {
            return false;
        }
        self.time_travel_polled_at_ms = now;
        let Some((plugin, instance_id)) = self.session.as_ref().and_then(|session| self.plugins.iter().find(|entry| entry.plugin_id == session.plugin_id).cloned().map(|plugin| (plugin, session.instance_id))) else { return false };
        match plugin.read_history(instance_id).await {
            Ok(patch) if patch.cursor >= self.history_cursor => {
                let changed = patch.time_travel != self.history_time_travel;
                fold_history_patch(&mut self.history_entries, &mut self.history_cursor, &patch, true);
                self.observe_history_time_travel(patch.time_travel.as_ref());
                changed
            }
            Ok(_) => false,
            Err(error) => {
                Self::debug_log(&format!("[TRACE] wgpu shell time-travel poll failed: {error}"));
                false
            }
        }
    }

    /// ⏪️ Folds the history patches the session's guest pushed on uncorrelated UI-progress frames since the last frame —
    /// the wgpu twin of React's `subscribeOperationProgress` → `applyHistoryPatch`. Each one takes the same stale-guarded
    /// path a dispatch reply does ([`ShellState::observe_invocation_history`]), so the band moves with every throttled
    /// replay step on the browser build too. `true` when the session status changed.
    pub(crate) async fn drain_progress_history_patches(&mut self) -> bool {
        let patches = self.session.as_ref().and_then(|session| self.plugins.iter().find(|entry| entry.plugin_id == session.plugin_id).map(|plugin| plugin.take_progress_history_patches(session.instance_id))).unwrap_or_default();
        if patches.is_empty() {
            return false;
        }
        let before = self.history_time_travel.clone();
        for patch in &patches {
            self.observe_invocation_history(Some(patch)).await;
        }
        before != self.history_time_travel
    }

    /// ⌨️ The dispatch a `ui.timeTravel.*` chord asks for, or `None` when it must fall through: no session, a verb the
    /// stage does not offer or has disabled, or a focused text field (hotkeys never fire while the user is typing).
    pub(crate) fn time_travel_shortcut_action(&self, verb: TimeTravelVerb) -> Option<ActionDescriptor> {
        let status = self.history_time_travel.as_ref()?;
        if !time_travel_verb_enabled(status, verb) || self.retained_text_field_has_focus() {
            return None;
        }
        Some(verb.action(&self.session.as_ref()?.app.controller_id, status))
    }

    /// ✍️ Whether a retained text or number field holds keyboard focus.
    fn retained_text_field_has_focus(&self) -> bool {
        time_travel_chord_yields_to(self.chrome_build.focused_retained_surface.as_ref().and_then(|surface| self.chrome_build.content_focus.get(surface)).and_then(Option::as_ref).map(|focus| focus.kind))
    }

    /// 📐️ This frame's band, localized and with each chorded button's inline badge.
    pub(crate) fn time_travel_band_plan_for(&self, status: &HistoryTimeTravel, theme: &Theme) -> TimeTravelBandPlan {
        let locale = self.active_locale();
        let message = time_travel_band_lines(status, self.active_terminology(), locale).message();
        let buttons = time_travel_band_controls(status).into_iter().map(|control| (control, self.chrome_control_label(control.verb.control_id(), control.verb.label(locale)))).collect();
        time_travel_band_plan(status, message, buttons, self.screen_w, self.screen_h, theme)
    }

    /// ⏪️ Paints the persistent time-travel band, one scalar, glyph run or hit per opportunity: fill, edges, the replay
    /// track and its fill, the message, then per button its fill, caption and hit. An enabled button's hit carries its
    /// verb, so a click, the keyboard ring and an assistive technology all dispatch it the same way; a disabled one
    /// paints muted, dispatches nothing and names the refusal that disables it. No button registers while a modal
    /// dialog (the finalize prompt) owns the pointer.
    pub(super) fn render_time_travel_band_step(&mut self, cursor: &mut ShellChromeChildCursor, overlay: &mut DrawList, atlas: &mut FontAtlas, input: &mut InputState<ActionDescriptor>, theme: &Theme) -> bool {
        let Some(status) = self.history_time_travel.clone() else { return true };
        let plan = self.time_travel_band_plan_for(&status, theme);
        let (border, fill, text_color) = time_travel_band_tone(&status, theme);
        let band = plan.band;
        let hair = theme.stroke_hairline;
        let baseline = |rect: Rect| rect.y + (rect.h + theme.font_size_small) * 0.5 - 1.0;
        match cursor.scalar {
            0 => overlay.push_rounded([band.x, band.y, band.w, band.h], fill, theme.border_radius),
            1..=4 => {
                let edge = match cursor.scalar {
                    1 => [band.x, band.y, band.w, hair],
                    2 => [band.x, band.y + band.h - hair, band.w, hair],
                    3 => [band.x, band.y, hair, band.h],
                    _ => [band.x + band.w - hair, band.y, hair, band.h],
                };
                overlay.push_solid(edge, border);
            }
            5 => {
                if let Some((track, _)) = plan.progress {
                    overlay.push_solid([track.x, track.y, track.w, track.h], theme.border_normal);
                }
            }
            6 => {
                if let Some((track, share)) = plan.progress {
                    overlay.push_solid([track.x, track.y, track.w * share, track.h], theme.progress);
                }
            }
            7 => match chrome_text_complete_step(overlay, atlas, &plan.message, plan.message_x, baseline(band), plan.message_w, theme.font_size_small, text_color, &mut cursor.glyph) {
                Ok(false) => return false,
                Ok(true) => {}
                Err(()) => {
                    self.error = Some("Shell time-travel band text exceeded the retained glyph boundary".to_string());
                    cursor.glyph.reset();
                }
            },
            scalar => {
                let Some(button) = plan.buttons.get((scalar - 8) / 3) else { return true };
                let enabled = button.control.disabled_by.is_none();
                match (scalar - 8) % 3 {
                    0 => overlay.push_rounded([button.rect.x, button.rect.y, button.rect.w, button.rect.h], if enabled { theme.button } else { theme.button.with_alpha(theme.button.a * 0.5) }, theme.border_radius),
                    1 => {
                        let ink = if enabled { theme.text } else { theme.text_muted };
                        match chrome_text_complete_step(overlay, atlas, &button.label, button.rect.x + theme.padding_standard, baseline(button.rect), (button.rect.w - theme.padding_standard).max(1.0), theme.font_size_small, ink, &mut cursor.glyph) {
                            Ok(false) => return false,
                            Ok(true) => {}
                            Err(()) => {
                                self.error = Some("Shell time-travel band button text exceeded the retained glyph boundary".to_string());
                                cursor.glyph.reset();
                            }
                        }
                    }
                    _ => {
                        if let Some(controller_id) = self.session.as_ref().map(|session| session.app.controller_id.clone()).filter(|_| !self.chrome_build.dialog_open()) {
                            let locale = self.active_locale();
                            let control_id = button.control.verb.control_id();
                            note_chrome_control_name(control_id, Some(button.control.verb.label(locale)));
                            note_chrome_control_description(control_id, button.control.disabled_by.map(|label| time_travel_label(label, locale)));
                            note_chrome_control_disabled(control_id, !enabled);
                            input.register_hit(HitTarget { rect: button.rect, event: enabled.then(|| button.control.verb.action(&controller_id, &status)), control_id: Some(control_id.to_string()), kind: HitKind::Button, drag_axis: None, drag_data: None });
                        }
                    }
                }
            }
        }
        cursor.scalar += 1;
        false
    }

    /// 🔊️ The band's live node: a progress bar while replaying with a known total, a polite status otherwise, named by
    /// the band's own message so a reader hears exactly what is painted.
    pub(crate) fn time_travel_status_accessibility_node(&self, node_id: u64) -> Option<ui_contract::AccessibilityProjectionNode> {
        let status = self.history_time_travel.as_ref()?;
        let mut node = chrome_status_accessibility_node(node_id, TIME_TRAVEL_BAND_STATUS_ID, time_travel_band_lines(status, self.active_terminology(), self.active_locale()).message());
        if let (HistoryTimeTravelStage::Replaying, Some(total)) = (status.stage, status.total) {
            node.role = "progressbar".into();
            node.value_min = Some(0.0);
            node.value_max = Some(f64::from(total));
            node.value_now = Some(f64::from(status.done.unwrap_or(0)));
            node.value_text = node.label.clone();
        }
        node.busy = matches!(status.stage, HistoryTimeTravelStage::Replaying | HistoryTimeTravelStage::Finalizing);
        Some(node)
    }

    /// 🪟️ One paint opportunity of ONE pane's time-travel indicator, painted after the pane's own chips (`index` is
    /// their count) at the bottom-middle anchor none of them uses — React's `TimeTravelWindowIndicator`: a clock and the
    /// word "Time travel", announced as a note named by what the window shows. It dispatches nothing.
    #[allow(clippy::too_many_arguments, reason = "the chrome walk's own paint context, forwarded unchanged")]
    pub(super) fn paint_window_time_travel_indicator_step(&mut self, cursor: &mut ShellChromeChildCursor, draw: &mut DrawList, atlas: &mut FontAtlas, icons: &IconAtlas, input: &mut InputState<ActionDescriptor>, theme: &Theme, window_id: &str, window_rect: Rect, index: usize) -> bool {
        if cursor.scalar != index {
            return true;
        }
        let Some(status) = self.history_time_travel.as_ref() else { return true };
        let locale = self.active_locale();
        let description = time_travel_indicator_text(status, self.active_terminology(), locale);
        let control_id = time_travel_indicator_control_id(window_id);
        let item = ChromeGroupItem { control_id: control_id.as_str(), icon_id: Some("clock"), label: Some(time_travel_indicator_caption(locale)), active: true, disabled: false, kind: HitKind::Generic };
        if cursor.rect.is_none() {
            let Some(width) = retained_chrome_group_item_width(atlas, theme, &item) else {
                self.error = Some("Shell time-travel indicator exceeded the retained chrome boundary".to_string());
                return true;
            };
            cursor.rect = Some(window_pane_chip_rect(theme, window_rect, PanelAnchor::BottomMiddle, width));
        }
        let Some(rect) = cursor.rect else { return true };
        if cursor.depth == 0 {
            cursor.depth = draw.push_glass([rect.x, rect.y, rect.w, rect.h], 0.0, theme.glass(Level::Window)).saturating_add(1);
        }
        draw.begin_glass_content(cursor.depth - 1);
        let step = render_retained_chrome_group_item_step(&mut cursor.group_phase, &mut cursor.glyph, draw, atlas, icons, input, theme, rect, &item, false);
        draw.end_glass_content();
        match step {
            RetainedChromeGroupStep::Pending => return false,
            RetainedChromeGroupStep::Complete => {}
            RetainedChromeGroupStep::Fault => self.error = Some("Shell time-travel indicator exceeded the retained glyph boundary".to_string()),
        }
        note_chrome_control_name(&control_id, Some(&description));
        note_chrome_control_semantics(&control_id, &ChromeDialogSemantics { role: Some("note"), ..ChromeDialogSemantics::default() });
        self.pane_overlay_hits.push(HitTarget { rect, event: None, control_id: Some(control_id), kind: HitKind::Generic, drag_axis: None, drag_data: None });
        cursor.rect = None;
        cursor.depth = 0;
        cursor.scalar += 1;
        false
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "../../../🧪️tests/🧪️wgpu-time-travel/🦀️.rs"]
mod tests;
