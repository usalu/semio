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
use semio_framework::kernel::{HistoryReprojection, HistoryReprojectionStatus, HistoryTimeTravel, HistoryTimeTravelReview, HistoryTimeTravelStage};
use semio_framework_time_travel::TimeTravelLabel;

//#region ⏪️TimeTravelControls
/// 🆔️ The band's live status node: every stage and progress change is announced through it.
pub(super) const TIME_TRAVEL_BAND_STATUS_ID: &str = "shell.time-travel.status";

/// 📶️ The band's replay progress node — a child of its status node, React's `<progress>` inside the band.
pub(super) const TIME_TRAVEL_BAND_PROGRESS_ID: &str = "shell.time-travel.progress";

/// 📐️ The band's widest extent before its message is clipped.
const TIME_TRAVEL_BAND_MAX_WIDTH: f32 = 880.0;

/// 📐️ Height of the replay progress track along the band's lower edge.
const TIME_TRAVEL_PROGRESS_TRACK: f32 = 3.0;

/// 🧱️ A band's fill as an opaque surface: a translucent tone tint composited over the menu-level surface, so the content
/// the band floats over never shows through its text.
fn chrome_band_surface(fill: Rgba, theme: &Theme) -> Rgba {
    let base = theme.level_bg[Level::Menu.index()];
    let alpha = fill.a.clamp(0.0, 1.0);
    Rgba { r: base.r + (fill.r - base.r) * alpha, g: base.g + (fill.g - base.g) * alpha, b: base.b + (fill.b - base.b) * alpha, a: 1.0 }
}

/// ⏱️ How often the native shell re-reads the history while the runtime replays or finalizes: the replay advances in
/// reactor turns that answer no dispatch, so the band's progress is polled rather than pushed.
#[cfg(not(target_arch = "wasm32"))]
pub(super) const TIME_TRAVEL_POLL_MS: f64 = 150.0;

/// ⏪️ One control of the band — a reserved history-edit verb the band, the chords and the keyboard ring can ask for.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum TimeTravelVerb {
    NextProblem,
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
            Self::NextProblem => semio_framework::HISTORY_EDIT_BEGIN_ACTION_ID,
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
            Self::NextProblem => "shell.time-travel.next-problem",
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

    /// 🗣️ The button's caption — the shared corpus label of [`Self::label_key`].
    pub(crate) fn label(self, tongue: impl Into<BandTongue>) -> &'static str {
        band_label(self.label_key(), tongue)
    }

    /// 🔑️ The corpus label key of this control's caption — React's `ui.timeTravel.<control>`.
    pub(crate) fn label_key(self) -> &'static str {
        match self {
            Self::NextProblem => "ui.timeTravel.nextProblem",
            Self::Accept => "ui.timeTravel.accept",
            Self::Discard => "ui.timeTravel.discard",
            Self::CancelReplay => "ui.timeTravel.cancelReplay",
            Self::Rerun => "ui.timeTravel.rerun",
            Self::Finalize => "ui.timeTravel.finalize",
            Self::Back => "ui.timeTravel.back",
            Self::Exit => "ui.timeTravel.exit",
        }
    }

    /// 🎯️ The dispatch this verb sends for `status` on the program `controller_id` names, carrying the generation the
    /// band showed — React's `timeTravelControlActionV1`; Next problem opens the mutation the session's own `next_problem`
    /// names (its id and, for a composed part, its store) and is never computed here.
    pub(crate) fn action(self, controller_id: &str, status: &HistoryTimeTravel) -> ActionDescriptor {
        let args = match (self, status.next_problem.as_ref()) {
            (Self::NextProblem, Some(problem)) => match problem.store.as_ref() {
                Some(store) => crate::action_args_json!({ "mutationId": problem.mutation_id.clone(), "store": store.clone() }),
                None => crate::action_args_json!({ "mutationId": problem.mutation_id.clone() }),
            },
            _ => crate::action_args_json!({ "generation": status.generation }),
        };
        ActionDescriptor { controller_id: controller_id.to_string(), action: self.action_id().to_string(), args }
    }
}

/// 🔘️ One band control: its verb and the session refusal that disables it (`None` when it is enabled).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct TimeTravelControl {
    pub verb: TimeTravelVerb,
    pub disabled_by: Option<TimeTravelLabel>,
}

/// 🚦️ The controls a stage offers, in band order — React's `timeTravelBandControlsV1`: a draft is accepted or
/// discarded while editing, a replay can be cancelled while it runs, a review that blocks offers Next problem first (only
/// when the session names one), a review can replay again only when the session
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
            status.next_problem.iter().map(|_| offer(Verb::NextProblem)).chain([TimeTravelControl { verb: Verb::Rerun, disabled_by: rerun }, TimeTravelControl { verb: Verb::Finalize, disabled_by: finalize }, offer(Verb::Exit)]).collect()
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

//#region 🎯️TimeTravelFocus
/// 🎯️ Where keyboard focus goes after a session change — React's `TimeTravelFocusTargetV1`: the first input of the draft
/// editor, the band, or the finalize prompt.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum TimeTravelFocus {
    Editor,
    Band,
    Dialog,
}

/// 🧭️ What one change of the focused program's session asks of the chrome — React's `TimeTravelTransitionV1`, pinned for
/// both shells by the band corpus's `transitions`: `reveal` opens the History panel, `focus` moves keyboard focus.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct TimeTravelTransition {
    pub reveal: bool,
    pub focus: Option<TimeTravelFocus>,
    pub scroll_to: Option<String>,
}

/// ⏱️ How long a focus target may take to reach the screen before the move is given up — React retries across 30
/// animation frames.
pub(crate) const TIME_TRAVEL_FOCUS_SETTLE_MS: f64 = 1000.0;

/// 🗝️ The history-body nodes the editor target resolves to: the inputs section, and the Accept button standing in when
/// no input takes focus.
pub(crate) const TIME_TRAVEL_EDITOR_INPUTS_KEY: &str = "framework.history.editor.inputs";
pub(crate) const TIME_TRAVEL_EDITOR_ACCEPT_KEY: &str = "framework.history.editor.accept";

/// 🧭️ The chrome's answer to the session moving from `previous` to `next` (`None` = no session) — React's
/// `timeTravelTransitionV1`: the edge into a new session reveals the History panel (whoever began it — a person, a chord
/// or an agent); a draft that starts (Begin, Next problem, another mutation) puts focus on its first input; a replay or
/// a review puts it on the band; the finalize prompt takes it; a progress step, a draft edit, the commit and the close
/// move nothing. The edge into a review that names its first blocking mutation reveals the panel again and names that
/// mutation's row (`scroll_to`) to bring into view.
pub(crate) fn time_travel_transition(previous: Option<&HistoryTimeTravel>, next: Option<&HistoryTimeTravel>) -> TimeTravelTransition {
    let Some(next) = next else { return TimeTravelTransition { reveal: false, focus: None, scroll_to: None } };
    let same = previous.is_some_and(|previous| previous.session_id == next.session_id);
    let moved = !same || previous.is_some_and(|previous| previous.stage != next.stage);
    let focus = match next.stage {
        HistoryTimeTravelStage::Editing => (moved || previous.and_then(|previous| previous.target.as_ref()) != next.target.as_ref()).then_some(TimeTravelFocus::Editor),
        HistoryTimeTravelStage::Replaying | HistoryTimeTravelStage::Reviewing => moved.then_some(TimeTravelFocus::Band),
        HistoryTimeTravelStage::Choosing => moved.then_some(TimeTravelFocus::Dialog),
        HistoryTimeTravelStage::Finalizing => None,
    };
    let problem = next.next_problem.as_ref().filter(|_| moved && next.stage == HistoryTimeTravelStage::Reviewing);
    TimeTravelTransition { reveal: !same || problem.is_some(), focus, scroll_to: problem.map(|problem| format!("{HISTORY_MUTATION_ROW_KEY_PREFIX}{}", problem.mutation_id)) }
}

/// 🗝️ Whether a projected node key names `key`: the key itself, or a surface-qualified `<surface>/<key>`.
fn projected_key_is(projected: &str, key: &str) -> bool {
    projected == key || projected.rsplit_once('/').is_some_and(|(_, tail)| tail == key)
}

/// 🎯️ The node of one retained window's accessibility projection the editor target resolves to — React's
/// `timeTravelFocusElementV1("editor")`: the first enabled control under the draft editor's inputs section (a row is
/// never the target), else its Accept button. `None` when the window does not show the editor.
pub(crate) fn time_travel_editor_focus_node(nodes: &[ui_contract::AccessibilityProjectionNode]) -> Option<usize> {
    let usable = |node: &ui_contract::AccessibilityProjectionNode| node.focusable && !node.disabled && !node.hidden;
    nodes
        .iter()
        .position(|node| projected_key_is(&node.key, TIME_TRAVEL_EDITOR_INPUTS_KEY))
        .and_then(|section| {
            let depth = nodes[section].depth;
            nodes[section + 1..].iter().take_while(|node| node.depth > depth).position(|node| usable(node) && node.role != "treeitem").map(|offset| section + 1 + offset)
        })
        .or_else(|| nodes.iter().position(|node| projected_key_is(&node.key, TIME_TRAVEL_EDITOR_ACCEPT_KEY) && usable(node)))
}
//#endregion 🎯️TimeTravelFocus

//#region 🗣️TimeTravelCopy
#[path = "🗣️labels/🦀️.rs"]
mod labels;
#[path = "🚦️refusals/🦀️.rs"]
mod refusals;

/// 🗣️ The axes a band string resolves on: the shell's locale, and whether the UI driver asks for the explaining `beginner`
/// tier instead of the terse `normal` one. A bare [`Locale`] is the `normal` tier.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct BandTongue {
    pub locale: Locale,
    pub beginner: bool,
}

impl From<Locale> for BandTongue {
    fn from(locale: Locale) -> Self {
        Self { locale, beginner: false }
    }
}

const BAND_LABEL_TARGET: &str = "ui.timeTravel.target";
const BAND_LABEL_PROGRESS: &str = "ui.timeTravel.progress";
const BAND_LABEL_PREPARATION_PROGRESS: &str = "ui.timeTravel.preparationProgress";
const BAND_LABEL_PROCESSED: &str = "ui.timeTravel.processed";
const BAND_LABEL_WORST: &str = "ui.timeTravel.worst";
const BAND_LABEL_REPLAY_FAULTED: &str = "ui.timeTravel.refusal.replayFaulted";
const BAND_LABEL_ACCEPTED: &str = "ui.timeTravel.accepted";
const BAND_LABEL_INDICATOR_TARGET: &str = "ui.timeTravel.indicatorTarget";
const BAND_LABEL_INDICATOR: &str = "ui.timeTravel.indicator";
const BAND_LABEL_PEER_EDITING_TARGET: &str = "ui.timeTravel.peer.editingTarget";
const BAND_LABEL_PEER_EDITING_HISTORY: &str = "ui.timeTravel.peer.editingHistory";
const BAND_LABEL_PEER_EDITING_ROW: &str = "ui.timeTravel.peer.editingRow";

/// 🧾️ Every fixed corpus label key the band copy reads besides its controls, stages, reviews, severities and refusals — the
/// keys a law requires the corpus to carry.
pub(crate) const BAND_FIXED_LABEL_KEYS: [&str; 12] = [BAND_LABEL_TARGET, BAND_LABEL_PROGRESS, BAND_LABEL_PREPARATION_PROGRESS, BAND_LABEL_PROCESSED, BAND_LABEL_WORST, BAND_LABEL_REPLAY_FAULTED, BAND_LABEL_ACCEPTED, BAND_LABEL_INDICATOR_TARGET, BAND_LABEL_INDICATOR, BAND_LABEL_PEER_EDITING_TARGET, BAND_LABEL_PEER_EDITING_HISTORY, BAND_LABEL_PEER_EDITING_ROW];

/// ❔️ What a band string reads when the corpus lacks its key: the replacement character — never empty, in no language, and
/// kept off every screen by the law that requires each key the shell reads.
pub(crate) const BAND_LABEL_MISSING: &str = "\u{FFFD}";

/// 🔎️ One corpus label on `tongue`, `None` for a key the corpus does not carry or carries empty.
pub(crate) fn band_label_in_catalog(key: &str, tongue: impl Into<BandTongue>) -> Option<&'static str> {
    let tongue: BandTongue = tongue.into();
    labels::label(key, tongue.locale == Locale::De, tongue.beginner).filter(|text| !text.is_empty())
}

/// 🏷️ One corpus label on `tongue`. Total: a key the corpus lacks reads [`BAND_LABEL_MISSING`], never an empty string.
pub(crate) fn band_label(key: &str, tongue: impl Into<BandTongue>) -> &'static str {
    band_label_in_catalog(key, tongue).unwrap_or(BAND_LABEL_MISSING)
}

/// 🔇️ Whether a shell stays silent about the refusal `code` (the corpus's `silent` rows: an outdated request needs no notice).
pub(crate) fn history_refusal_is_silent(code: &str) -> bool {
    refusals::refusal(code).is_some_and(|(_, _, _, silent)| silent)
}

/// 🌐️ One `TimeTravelLabel` in `locale`.
pub(crate) fn time_travel_label(label: TimeTravelLabel, locale: Locale) -> &'static str {
    label.localized(|en, de| match locale {
        Locale::En => en,
        Locale::De => de,
    })
}

/// 🚦️ A severity's word — the corpus label `ui.mutation.level.*`.
pub(crate) fn time_travel_severity_text(severity: semio_framework::Severity, tongue: impl Into<BandTongue>) -> &'static str {
    use semio_framework::Severity;
    band_label(
        match severity {
            Severity::Info => "ui.mutation.level.info",
            Severity::Warning => "ui.mutation.level.warning",
            Severity::Error => "ui.mutation.level.error",
            Severity::Fatal => "ui.mutation.level.fatal",
        },
        tongue,
    )
}

/// 🪧️ The corpus label key of a wire stage.
fn time_travel_stage_key(stage: HistoryTimeTravelStage) -> &'static str {
    match stage {
        HistoryTimeTravelStage::Editing => "ui.timeTravel.stage.editing",
        HistoryTimeTravelStage::Replaying => "ui.timeTravel.stage.replaying",
        HistoryTimeTravelStage::Reviewing => "ui.timeTravel.stage.reviewing",
        HistoryTimeTravelStage::Choosing => "ui.timeTravel.stage.choosing",
        HistoryTimeTravelStage::Finalizing => "ui.timeTravel.stage.finalizing",
    }
}

/// 🔬️ The corpus label key of what a review shows.
fn time_travel_review_key(review: HistoryTimeTravelReview) -> &'static str {
    match review {
        HistoryTimeTravelReview::NoChanges => "ui.timeTravel.review.noChanges",
        HistoryTimeTravelReview::NeedsReplay => "ui.timeTravel.review.needsReplay",
        HistoryTimeTravelReview::Blocked => "ui.timeTravel.review.blocked",
        HistoryTimeTravelReview::Ready => "ui.timeTravel.review.ready",
    }
}

/// 🛑️ The history-edit refusal one machine `code` is, as `(code, message, severity)` in `locale` — React's
/// `historyRefusalCodeV1` over `HISTORY_REFUSAL_LABEL_KEYS`, read from the corpus's `refusals`: a hub `history.*` transition
/// refusal is an error; every `timeTravel.*` code the session, its hosting runtime or its driver answers is a warning,
/// because the verb was refused or the replay stopped and nothing was lost. Only an exact code matches; prose never does.
pub(crate) fn history_refusal_notice(code: &str, tongue: impl Into<BandTongue>) -> Option<(&'static str, &'static str, semio_framework::Severity)> {
    refusals::refusal(code).map(|(code, label, severity, _)| (code, band_label(label, tongue), severity))
}

/// 🔎️ The history-edit refusal a dispatch-fault string carries — React's `historyRefusalOfFaultV1`. The funnel's string is
/// `code: message`, then ` — code: message [target]; …` for the report's messages (the browser prefixes its bridge
/// call), so the fault's own code comes first and each report code after it; each is read as one whole token.
pub(crate) fn history_refusal_of_fault(fault: &str, tongue: impl Into<BandTongue>) -> Option<(&'static str, &'static str, semio_framework::Severity)> {
    let tongue: BandTongue = tongue.into();
    fault_tokens(fault).find_map(|token| history_refusal_notice(token, tongue))
}

/// 🛟️ Whether a typed history refusal discards only the submitted local input draft.
pub(crate) fn history_refusal_discards_input_draft(fault: &str) -> bool {
    fault_tokens(fault).any(|token| refusals::refusal(token).is_some())
}

/// ✂️ A dispatch-fault string's whole tokens, in order — a code is only ever a whole token.
fn fault_tokens(fault: &str) -> impl Iterator<Item = &str> {
    fault.split(|c: char| c.is_whitespace() || matches!(c, ':' | ';' | ',' | '[' | ']' | '(' | ')'))
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

/// 🏷️ The edited mutation's label on the shell's axes, when the session names one.
fn time_travel_target_text(status: &HistoryTimeTravel, terminology: Terminology, locale: Locale) -> Option<String> {
    status.target_label.as_ref().map(|label| label.resolve(terminology, locale).to_string()).filter(|target| !target.is_empty())
}

/// ⏳️ Names the running preparation or replay through its canonical band copy.
fn time_travel_progress_key(stage: HistoryTimeTravelStage) -> Option<&'static str> {
    match stage { HistoryTimeTravelStage::Editing=>Some(BAND_LABEL_PREPARATION_PROGRESS), HistoryTimeTravelStage::Replaying=>Some(BAND_LABEL_PROGRESS), _=>None }
}

/// 📝️ [`TimeTravelBandLines`] of `status` — React's `timeTravelBandTextV1`: a review reads what the session states
/// (`review`), never what a missing report might mean; severity is always named in words, never by colour alone; a fault
/// code the shell knows reads as its localized refusal, any other as the shared replay-failed copy — never the code itself.
pub(crate) fn time_travel_band_lines(status: &HistoryTimeTravel, terminology: Terminology, tongue: impl Into<BandTongue>) -> TimeTravelBandLines {
    let tongue: BandTongue = tongue.into();
    let locale = tongue.locale;
    TimeTravelBandLines {
        stage: band_label(time_travel_stage_key(status.stage), tongue).to_string(),
        target: time_travel_target_text(status, terminology, locale).map(|target| band_label(BAND_LABEL_TARGET, tongue).replace("{target}", &target)),
        progress: time_travel_progress_key(status.stage).zip(status.total).map(|(key,total)| {
            let mut text = band_label(key, tongue).replace("{done}", &status.done.unwrap_or(0).to_string()).replace("{total}", &total.to_string());
            if let Some(processed) = status.processed {
                text.push_str(" · ");
                text.push_str(&band_label(BAND_LABEL_PROCESSED, tongue).replace("{processed}", &processed.to_string()));
            }
            text
        }),
        review: status.review.filter(|_| status.stage == HistoryTimeTravelStage::Reviewing).map(|review| band_label(time_travel_review_key(review), tongue).to_string()),
        outcome: status.worst.map(|worst| band_label(BAND_LABEL_WORST, tongue).replace("{level}", time_travel_severity_text(worst, tongue))),
        fault: status.fault.as_deref().map(|fault| match history_refusal_notice(fault, locale) {
            Some((code, message, _)) if code == fault => message.to_string(),
            _ => band_label(BAND_LABEL_REPLAY_FAULTED, tongue).to_string(),
        }),
        accepted: (status.accepted_count > 0).then(|| band_label(BAND_LABEL_ACCEPTED, tongue).replace("{count}", &status.accepted_count.to_string())),
    }
}

/// 🏷️ What a window of the editing program says about itself — React's `timeTravelIndicatorTextV1`: the document
/// before the edited mutation while a draft is edited, else the session's stage.
pub(crate) fn time_travel_indicator_text(status: &HistoryTimeTravel, terminology: Terminology, tongue: impl Into<BandTongue>) -> String {
    let tongue: BandTongue = tongue.into();
    match (status.stage, time_travel_target_text(status, terminology, tongue.locale)) {
        (HistoryTimeTravelStage::Editing, Some(target)) => band_label(BAND_LABEL_INDICATOR_TARGET, tongue).replace("{target}", &target),
        (stage, _) => band_label(time_travel_stage_key(stage), tongue).to_string(),
    }
}

/// 🏷️ The indicator chip's visible word — React's `ui.timeTravel.indicator`; its accessible name is
/// [`time_travel_indicator_text`].
pub(crate) fn time_travel_indicator_caption(tongue: impl Into<BandTongue>) -> &'static str {
    band_label(BAND_LABEL_INDICATOR, tongue)
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
    let (border, fill, text) = transient_notice_tone(severity, theme);
    (border, chrome_band_surface(fill, theme), text)
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

/// 📐️ The band laid out for one frame: its box, the message's lines, the replay progress track with its filled share,
/// and the buttons in stage order.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct TimeTravelBandPlan {
    pub band: Rect,
    pub lines: Vec<ChromeBandLine>,
    pub progress: Option<(Rect, f32)>,
    pub buttons: Vec<TimeTravelBandButton>,
}

/// 📏️ One line of a laid-out chrome band: its text and the box it paints in (its baseline centred in the box).
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ChromeBandLine {
    pub text: String,
    pub rect: Rect,
}

/// 📐️ A bottom-centre chrome band laid out: its box, the message's lines and one rect per button in the order given.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ChromeBandLayout {
    pub band: Rect,
    pub lines: Vec<ChromeBandLine>,
    pub buttons: Vec<Rect>,
}

/// 🧵️ The message's lines within `width`: whole `separator` segments joined while they fit, a segment wider than a line
/// broken at its words, a word wider than a line on a line of its own (clipped when painted).
fn chrome_band_wrap(message: &str, separator: &str, width: f32, glyph_w: impl Fn(&str) -> f32) -> Vec<String> {
    let mut lines = Vec::new();
    let mut line = String::new();
    let push = |piece: &str, joint: &str, line: &mut String, lines: &mut Vec<String>| {
        let joined = if line.is_empty() { piece.to_string() } else { format!("{line}{joint}{piece}") };
        if line.is_empty() || glyph_w(&joined) <= width {
            *line = joined;
        } else {
            lines.push(std::mem::replace(line, piece.to_string()));
        }
    };
    for segment in message.split(separator) {
        if glyph_w(segment) <= width {
            push(segment, separator, &mut line, &mut lines);
            continue;
        }
        for (index, word) in segment.split(' ').enumerate() {
            push(word, if index == 0 { separator } else { " " }, &mut line, &mut lines);
        }
    }
    if !line.is_empty() || lines.is_empty() {
        lines.push(line);
    }
    lines
}

/// 📐️ Lays a chrome band out centred with its lower edge on `floor` — one row of the shell's subfooter — the way React's bottom bands wrap
/// (`max-w-[90vw] flex-wrap`): one row — the message, then the buttons right-aligned — while it fits `max_w` and 90 % of
/// the viewport; else compact for a phone-width viewport: the message wrapped over full-width lines
/// ([`chrome_band_wrap`] at `separator`), the buttons flowing in rows below it. Widths use the monospace estimate every
/// shell band uses; the renderer measures glyphs only while painting them. Every button is a touch target of the `large`
/// size token on both axes ([`Theme::size_large`], React's `min-h-large min-w-large`).
pub(crate) fn chrome_band_layout(message: &str, separator: &str, buttons: &[&str], floor: f32, width: f32, max_w: f32, theme: &Theme) -> ChromeBandLayout {
    let (pad, gap, small) = (theme.padding_standard, theme.gap_standard, theme.font_size_small);
    let glyph_w = |text: &str| text.chars().count() as f32 * small * 0.6;
    let target = theme.size_large();
    let button_w = |label: &str| (glyph_w(label) + pad * 2.0).max(target);
    let available = (width * 0.9).min(max_w).max(1.0);
    let row_h = (small * 1.6 + pad * 2.0).max(target + pad);
    let buttons_w: f32 = buttons.iter().map(|label| button_w(*label) + gap).sum();
    let single_w = pad * 2.0 + glyph_w(message) + gap + buttons_w;
    if single_w <= available {
        let band = Rect::new(((width - single_w) * 0.5).max(0.0), (floor - row_h).max(0.0), single_w, row_h);
        let mut right = band.x + band.w - pad;
        let mut rects: Vec<Rect> = buttons
            .iter()
            .rev()
            .map(|label| {
                right -= button_w(*label);
                let rect = Rect::new(right, band.y + pad * 0.5, button_w(*label), band.h - pad);
                right -= gap;
                rect
            })
            .collect();
        rects.reverse();
        let line = ChromeBandLine { text: message.to_string(), rect: Rect::new(band.x + pad, band.y, (right - band.x - pad).max(1.0), band.h) };
        return ChromeBandLayout { band, lines: vec![line], buttons: rects };
    }
    let inner = (available - pad * 2.0).max(1.0);
    let line_h = small * 1.6;
    let button_h = (small * 1.6 + pad).max(target);
    let wrapped = chrome_band_wrap(message, separator, inner, glyph_w);
    let mut placed = Vec::with_capacity(buttons.len());
    let (mut x, mut row) = (0.0_f32, 0_usize);
    for label in buttons {
        let w = button_w(*label).min(inner);
        if x > 0.0 && x + w > inner {
            (x, row) = (0.0, row + 1);
        }
        placed.push((x, row, w));
        x += w + gap;
    }
    let rows = placed.last().map_or(0, |(_, row, _)| row + 1);
    let lines_h = wrapped.len() as f32 * line_h;
    let buttons_h = rows as f32 * button_h + rows.saturating_sub(1) as f32 * gap * 0.5;
    let band_h = pad + lines_h + if rows > 0 { gap * 0.5 + buttons_h } else { 0.0 };
    let band = Rect::new(((width - available) * 0.5).max(0.0), (floor - band_h).max(0.0), available, band_h);
    let lines = wrapped.into_iter().enumerate().map(|(index, text)| ChromeBandLine { text, rect: Rect::new(band.x + pad, band.y + pad * 0.5 + index as f32 * line_h, inner, line_h) }).collect();
    let top = band.y + pad * 0.5 + lines_h + gap * 0.5;
    let buttons = placed.into_iter().map(|(x, row, w)| Rect::new(band.x + pad + x, top + row as f32 * (button_h + gap * 0.5), w, button_h)).collect();
    ChromeBandLayout { band, lines, buttons }
}

/// 📐️ Lays the band out as the subfooter's last row, under the footer, through [`chrome_band_layout`] — away from the transient notice
/// stack at the top, so a notice raised while time travelling never covers it — with the replay track along its lower
/// edge. Its message wraps at the " · " between its lines on a phone-width viewport.
pub(crate) fn time_travel_band_plan(status: &HistoryTimeTravel, message: String, buttons: Vec<(TimeTravelControl, String)>, width: f32, height: f32, theme: &Theme) -> TimeTravelBandPlan {
    let labels: Vec<&str> = buttons.iter().map(|(_, label)| label.as_str()).collect();
    let layout = chrome_band_layout(&message, " · ", &labels, height, width, TIME_TRAVEL_BAND_MAX_WIDTH, theme);
    let band = layout.band;
    let pad = theme.padding_standard;
    let progress = match (status.stage, status.total) {
        (HistoryTimeTravelStage::Editing | HistoryTimeTravelStage::Replaying, Some(total)) if total > 0 => {
            Some((Rect::new(band.x + pad, band.y + band.h - TIME_TRAVEL_PROGRESS_TRACK - 1.0, band.w - pad * 2.0, TIME_TRAVEL_PROGRESS_TRACK), (status.done.unwrap_or(0) as f32 / total as f32).clamp(0.0, 1.0)))
        }
        _ => None,
    };
    let buttons = buttons.into_iter().zip(layout.buttons).map(|((control, label), rect)| TimeTravelBandButton { control, label, rect }).collect();
    TimeTravelBandPlan { band, lines: layout.lines, progress, buttons }
}

/// 🖌️ One opportunity of a bottom chrome band's frame — its fill (0), its four edges (1–4), the replay track (5) and the track's
/// filled share (6) — shared by the session band and the reprojection band.
fn paint_chrome_band_frame(scalar: usize, overlay: &mut DrawList, band: Rect, progress: Option<(Rect, f32)>, (border, fill): (Rgba, Rgba), theme: &Theme) {
    let hair = theme.stroke_hairline;
    match scalar {
        0 => overlay.push_rounded([band.x, band.y, band.w, band.h], fill, theme.border_radius),
        1 => overlay.push_solid([band.x, band.y, band.w, hair], border),
        2 => overlay.push_solid([band.x, band.y + band.h - hair, band.w, hair], border),
        3 => overlay.push_solid([band.x, band.y, hair, band.h], border),
        4 => overlay.push_solid([band.x + band.w - hair, band.y, hair, band.h], border),
        5 => {
            if let Some((track, _)) = progress {
                overlay.push_solid([track.x, track.y, track.w, track.h], theme.border_normal);
            }
        }
        _ => {
            if let Some((track, share)) = progress {
                overlay.push_solid([track.x, track.y, track.w * share, track.h], theme.progress);
            }
        }
    }
}

/// 🧮️ The opportunities [`paint_chrome_band_frame`] takes before a band's text.
const CHROME_BAND_FRAME_STEPS: usize = 7;
//#endregion 📐️TimeTravelBand

//#region 📡️HistoryReprojectionBand
/// 🗝️ The mirror key of the polite status that announces a history change replaying before this replica adopts it, outside the
/// History panel — React's `[data-semio-history-reprojection]` (audit W1E-3).
pub(crate) const HISTORY_REPROJECTION_STATUS_ID: &str = "shell.history.reprojection";

/// 🧮️ The mirror key of that change's progress bar — React's `[data-semio-history-reprojection-progress]`.
pub(crate) const HISTORY_REPROJECTION_PROGRESS_ID: &str = "shell.history.reprojection.progress";

/// 🏃️ Whether the announced change still replays: operations remain and nobody paused it (a refusal has nothing left).
pub(crate) fn history_reprojection_running(status: &HistoryReprojectionStatus) -> bool {
    !status.paused && status.total > 0
}

/// 📢️ What the band paints and its live node speaks — React's `[data-semio-history-reprojection]` status, `<title>: <text>`.
pub(crate) fn history_reprojection_message(status: &HistoryReprojectionStatus) -> String {
    format!("{}: {}", status.title, status.text)
}

/// 🔘️ The band's control while no session is open (a session's own band addresses the session) — React's reprojection status
/// buttons, the history body section's twin: Cancel replay while it replays (a step or a load is dropped with zero trace, a
/// remote change pauses), Replay again while a remote change is paused; none for a refusal. Sent without a generation.
pub(crate) fn history_reprojection_control(status: &HistoryReprojectionStatus, session_open: bool) -> Option<TimeTravelVerb> {
    match (session_open, status.paused, history_reprojection_running(status)) {
        (true, _, _) => None,
        (false, true, _) => Some(TimeTravelVerb::Rerun),
        (false, false, true) => Some(TimeTravelVerb::CancelReplay),
        (false, false, false) => None,
    }
}

/// 🆔️ The reprojection band control's id: `shell.history.reprojection.<cancel-replay|rerun>`.
pub(crate) fn history_reprojection_control_id(verb: TimeTravelVerb) -> &'static str {
    match verb {
        TimeTravelVerb::Rerun => "shell.history.reprojection.rerun",
        _ => "shell.history.reprojection.cancel-replay",
    }
}

/// 📐️ The reprojection band laid out for one frame: its box, the status line's lines, the replay track with its filled share and
/// the rect of its one control.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct HistoryReprojectionBandPlan {
    pub band: Rect,
    pub lines: Vec<ChromeBandLine>,
    pub progress: Option<(Rect, f32)>,
    pub control: Option<(TimeTravelVerb, Rect)>,
}

/// 📐️ Lays the reprojection band out bottom-centre through [`chrome_band_layout`] on `floor` (the footer, or the top of the session
/// band it then stacks above), wrapping its message at its words when it outgrows the band, its control right-aligned, with the
/// replay track along its lower edge while it replays.
pub(crate) fn history_reprojection_band_plan(status: &HistoryReprojectionStatus, control: Option<TimeTravelVerb>, floor: f32, width: f32, tongue: impl Into<BandTongue>, theme: &Theme) -> HistoryReprojectionBandPlan {
    let labels: Vec<&str> = control.map(|verb| verb.label(tongue)).into_iter().collect();
    let layout = chrome_band_layout(&history_reprojection_message(status), " ", &labels, floor, width, TIME_TRAVEL_BAND_MAX_WIDTH, theme);
    let band = layout.band;
    let pad = theme.padding_standard;
    let progress = history_reprojection_running(status).then(|| (Rect::new(band.x + pad, band.y + band.h - TIME_TRAVEL_PROGRESS_TRACK - 1.0, band.w - pad * 2.0, TIME_TRAVEL_PROGRESS_TRACK), (status.done as f32 / status.total as f32).clamp(0.0, 1.0)));
    HistoryReprojectionBandPlan { band, lines: layout.lines, progress, control: control.zip(layout.buttons.first().copied()) }
}

/// 🎨️ The reprojection band's border, fill and ink: a refused adoption warns, a replaying or paused change informs.
fn history_reprojection_band_tone(status: &HistoryReprojectionStatus, theme: &Theme) -> (Rgba, Rgba, Rgba) {
    let (border, fill, text) = transient_notice_tone(if status.fault.is_some() && status.total == 0 { semio_framework::Severity::Warning } else { semio_framework::Severity::Info }, theme);
    (border, chrome_band_surface(fill, theme), text)
}
//#endregion 📡️HistoryReprojectionBand

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
fn peer_editing_text(name: &str, target: Option<&str>, tongue: BandTongue) -> String {
    match target {
        Some(target) => band_label(BAND_LABEL_PEER_EDITING_TARGET, tongue).replace("{name}", name).replace("{target}", target),
        None => band_label(BAND_LABEL_PEER_EDITING_HISTORY, tongue).replace("{name}", name),
    }
}

/// 🗣️ `ui.timeTravel.peer.editingRow`.
fn peer_editing_row_text(name: &str, tongue: BandTongue) -> String {
    band_label(BAND_LABEL_PEER_EDITING_ROW, tongue).replace("{name}", name)
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
pub(crate) fn time_travel_peer_presence<'a>(editing: impl IntoIterator<Item = (&'a str, &'a str, &'a str)>, entries: &BTreeMap<String, semio_framework::kernel::HistoryEntry>, terminology: Terminology, tongue: impl Into<BandTongue>) -> TimeTravelPeerPresence {
    let tongue: BandTongue = tongue.into();
    let locale = tongue.locale;
    let mut notes: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let activities = editing
        .into_iter()
        .map(|(actor, name, mutation_id)| {
            let row = entries.values().find(|entry| entry.mutations.iter().any(|mutation| mutation.mutation_id == mutation_id));
            let target = row.and_then(|entry| entry.mutations.iter().find(|mutation| mutation.mutation_id == mutation_id)).map(|mutation| mutation.label.resolve(terminology, locale).to_string()).filter(|target| !target.is_empty());
            let note = peer_editing_row_text(name, tongue);
            if let Some(entry) = row {
                notes.entry(format!("{HISTORY_ROW_KEY_PREFIX}{}", entry.seq)).or_default().push(note.clone());
            }
            notes.entry(format!("{HISTORY_MUTATION_ROW_KEY_PREFIX}{mutation_id}")).or_default().push(note);
            (actor.to_string(), ui_wgpu::wgpu::PresenceActivity { text: peer_editing_text(name, target.as_deref(), tongue), badge: TIME_TRAVEL_PEER_BADGE.to_string() })
        })
        .collect();
    TimeTravelPeerPresence { activities, notes: notes.into_iter().map(|(key, lines)| (key, lines.join(" · "))).collect() }
}
//#endregion 👥️PeerHistoryEdits

impl ShellState {
    /// ⏪️ Takes the session status one history patch carries (absent = no session) and answers its transition
    /// ([`time_travel_transition`]): the edge into a new session reveals the History panel tab — or, below the mobile
    /// breakpoint, opens the one mobile panel on it — whose Rust-built body holds the editor, and every edge that names a
    /// focus target leaves it waiting for [`Self::resolve_time_travel_focus`]. A closed session drops a focus still waiting.
    pub(crate) fn observe_history_time_travel(&mut self, time_travel: Option<&HistoryTimeTravel>) {
        let transition = time_travel_transition(self.history_time_travel.as_ref(), time_travel);
        if let Some(focus) = transition.focus {
            self.time_travel_focus = Some((focus, chrome_now_ms() + TIME_TRAVEL_FOCUS_SETTLE_MS));
        } else if time_travel.is_none() {
            self.time_travel_focus = None;
        }
        self.history_time_travel = time_travel.cloned();
        if transition.reveal {
            self.reveal_dock_tab(FRAMEWORK_PANEL_TAB_HISTORY_ID);
            if self.mobile_panel_active() {
                self.reveal_mobile_panel_tab(FRAMEWORK_PANEL_TAB_HISTORY_ID);
            }
        }
    }

    /// 📱️ Opens the one mobile panel on `tab_id` — the phone-width twin of the anchor reveal, since below the mobile
    /// breakpoint no anchor paints and the History body (the editor) lives in that panel.
    fn reveal_mobile_panel_tab(&mut self, tab_id: &str) {
        let Some((_, path)) = self.dock_tabs.locate(tab_id) else { return };
        self.mobile_panel_path = mobile_panel_reconcile_path(&self.mobile_panel_tabs(), &path);
        self.mobile_panel_visible = true;
    }

    /// 🎯️ Moves keyboard focus to the waiting time-travel target the moment a presented frame shows it — React's
    /// `scheduleTimeTravelFocusV1` — so the ARIA mirror (which follows the projected `focused` node) lands on it: the
    /// editor's first input receives an accessibility focus in its retained window, the band's live status node becomes
    /// the focused chrome node, and the open finalize prompt already focuses its first stop. A person typing in a field
    /// elsewhere keeps their focus, and a target that has not appeared within [`TIME_TRAVEL_FOCUS_SETTLE_MS`] is given up.
    pub(crate) fn resolve_time_travel_focus(&mut self, input: &mut InputState<ActionDescriptor>) {
        let Some((focus, deadline)) = self.time_travel_focus else { return };
        if self.time_travel_focus_is_held(input) || chrome_now_ms() > deadline {
            self.time_travel_focus = None;
            return;
        }
        let moved = match focus {
            TimeTravelFocus::Dialog => self.chrome_build.dialog_open(),
            TimeTravelFocus::Band => self.history_time_travel.is_some() && !self.chrome_build.dialog_open() && {
                input.blur_input();
                self.accessibility_focused_control_id = Some(TIME_TRAVEL_BAND_STATUS_ID.to_string());
                true
            },
            TimeTravelFocus::Editor => crate::interpreter::visible_retained_accessibility_target(time_travel_editor_focus_node).is_some_and(|target| self.focus_retained_accessibility_target(&target, input)),
        };
        if moved {
            self.time_travel_focus = None;
        }
    }

    /// ✋️ Whether moving focus now would take it from someone typing elsewhere — React's `timeTravelFocusIsHeldV1`: a
    /// chrome text field outside the finalize prompt, or a retained input outside the History panel, keeps its focus.
    fn time_travel_focus_is_held(&self, input: &InputState<ActionDescriptor>) -> bool {
        let chrome = input.focused_id.is_some() && !self.chrome_build.dialog_open();
        let retained = self.chrome_build.focused_retained_surface.as_deref().is_some_and(|surface| surface != FRAMEWORK_PANEL_TAB_HISTORY_ID && self.chrome_build.content_focus.get(surface).and_then(Option::as_ref).is_some_and(|focus| focus.kind == RetainedNodeFocusKind::Input));
        chrome || retained
    }

    /// 🎯️ Gives one retained node the same accessibility focus a reader's focus would give it, and hands the keyboard
    /// to its window; `false` when the node's window no longer presents it.
    fn focus_retained_accessibility_target(&mut self, target: &ui_render::AccessibilityTarget, input: &mut InputState<ActionDescriptor>) -> bool {
        let Some(commands) = crate::interpreter::dispatch_accessibility_event(&target.window_id, target.window_generation, target.node_id, &target.node_key, ui_wgpu::wgpu::AccessibilityUiEvent::Focus, input) else { return false };
        self.chrome_build.note_content_focus_commands(&commands);
        self.accessibility_focused_control_id = None;
        true
    }

    /// 🗣️ The axes this shell's band copy resolves on: its locale and the UI driver's label tier.
    pub(crate) fn band_tongue(&self) -> BandTongue {
        BandTongue { locale: self.active_locale(), beginner: self.chrome_build.driver.label_tier == ui_wgpu::wgpu::UiDriverLabelTier::Beginner }
    }

    /// 🧱️ The height of the subfooter — the shell's last layout row, under the footer, in which the session band, the
    /// reprojection band and the folder offer are stacked (React's `subfooter`); zero while none shows. The body and the footer
    /// end above it, so a band never covers content.
    pub(crate) fn subfooter_height(&self, theme: &Theme) -> f32 {
        let session = self.history_time_travel.as_ref().map(|status| self.time_travel_band_plan_for(status, theme).band.y);
        let reprojection = self.history_reprojection_band_plan_for(theme).map(|(_, plan)| plan.band.y);
        let folder = self.folder_reconnect_band_offer().map(|offer| self.folder_reconnect_band_plan_for(&offer, theme).band.y);
        (self.screen_h - [session, reprojection, folder].into_iter().flatten().fold(self.screen_h, f32::min)).max(0.0)
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
        time_travel_peer_presence(editing, &self.history_entries, self.active_terminology(), self.band_tongue())
    }

    /// 👥️ Hands the retained history body the peers' row notes — React's shell-root presence overlay. Runs every frame;
    /// with no peer editing it is one empty comparison per window. `true` when a window's notes changed.
    pub(crate) fn publish_peer_time_travel_notes(&self) -> bool {
        crate::interpreter::set_ui_presence_notes(&self.peer_time_travel_presence().notes)
    }

    /// ⏱️ While the runtime prepares, replays or finalizes a session, or a history change replays before adoption, re-reads the history at
    /// most every [`TIME_TRAVEL_POLL_MS`] and folds the snapshot — rows, cursor, session and reprojection status — so a reply older
    /// than it can no longer roll the bands back. The
    /// runtime also pushes throttled progress patches on uncorrelated `AppFrame::Invocation` frames, which reach
    /// [`ShellState::observe_invocation_history`] with the next exchange; this poll is what moves the band between two
    /// exchanges. `true` when the status changed.
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) async fn poll_time_travel_progress(&mut self) -> bool {
        let polling = self.history_time_travel.as_ref().is_some_and(|status| (matches!(status.stage, HistoryTimeTravelStage::Replaying | HistoryTimeTravelStage::Finalizing) || status.stage == HistoryTimeTravelStage::Editing && status.total.is_some())) || self.history_reprojection.as_ref().is_some_and(|reprojection| !reprojection.paused && reprojection.total > 0);
        let now = chrome_now_ms();
        if !polling || now - self.time_travel_polled_at_ms < TIME_TRAVEL_POLL_MS {
            return false;
        }
        self.time_travel_polled_at_ms = now;
        let Some((plugin, instance_id)) = self.session.as_ref().and_then(|session| self.plugins.iter().find(|entry| entry.plugin_id == session.plugin_id).cloned().map(|plugin| (plugin, session.instance_id))) else { return false };
        match plugin.read_history(instance_id).await {
            Ok(patch) if patch.cursor >= self.history_cursor => {
                let changed = patch.time_travel != self.history_time_travel || patch.reprojection != self.history_reprojection;
                fold_history_patch(&mut self.history_entries, &mut self.history_cursor, &patch, true);
                self.observe_history_time_travel(patch.time_travel.as_ref());
                self.observe_history_reprojection(patch.reprojection.as_ref());
                changed
            }
            Ok(_) => false,
            Err(error) => {
                Self::debug_log(&format!("[TRACE] wgpu shell time-travel poll failed: {error}"));
                false
            }
        }
    }

    /// 🏁️ Applies what the session's guest published between two dispatches — the wgpu twin of React's
    /// `subscribeOperationCompletions` and `subscribeOperationProgress` passes. A typed operation that outlives the host
    /// call that started it (an example load of a few hundred steps, the replay behind an accepted history edit) ends
    /// on the bridge's standing drain, and its completion is the only carrier of its dirty scope and command-log
    /// delta. Each publication's history patch takes the reply's own stale-guarded fold
    /// ([`ShellState::observe_invocation_history`]) and the union of what they owe ([`operation_publication_refresh`])
    /// goes to the settle lane ([`ShellState::owe_refresh`]) — the surface refreshes and the rows land with no further
    /// dispatch. A drain `fault` is told through the dispatch-fault funnel; `resync` re-reads the whole projection.
    /// `true` when the session, the rows or the owed refresh changed, or a fault was told.
    pub(crate) async fn drain_operation_publications(&mut self) -> bool {
        let publications = self.session.as_ref().and_then(|session| self.plugins.iter().find(|entry| entry.plugin_id == session.plugin_id).map(|plugin| plugin.take_operation_publications(session.instance_id))).unwrap_or_default();
        if publications.is_empty() {
            return false;
        }
        let before = (self.history_time_travel.clone(), self.history_reprojection.clone(), self.history_cursor, self.history_entries.len());
        let mut owed = UiDirtyScope::None;
        let mut told = false;
        for publication in &publications {
            if let Some(fault) = publication.fault.as_deref() {
                Self::debug_log(&format!("[TRACE] wgpu shell operation publication fault: {fault}"));
                self.note_dispatch_fault(fault);
                told = true;
            }
            if publication.resync {
                self.reread_history_projection().await;
            }
            self.observe_invocation_history(publication.history_patch.as_ref()).await;
            owed = owed.merged_with(operation_publication_refresh(publication));
        }
        let owes = !owed.asks_for_nothing();
        self.owe_refresh(owed);
        told || owes || before != (self.history_time_travel.clone(), self.history_reprojection.clone(), self.history_cursor, self.history_entries.len())
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
        let tongue = self.band_tongue();
        let message = time_travel_band_lines(status, self.active_terminology(), tongue).message();
        let buttons = time_travel_band_controls(status).into_iter().map(|control| (control, self.chrome_control_label(control.verb.control_id(), control.verb.label(tongue)))).collect();
        time_travel_band_plan(status, message, buttons, self.screen_w, self.screen_h, theme)
    }

    /// ⏪️ Paints the persistent time-travel band, one scalar, glyph run or hit per opportunity: fill, edges, the replay
    /// track and its fill, each line of the message, then per button its fill, caption and hit. An enabled button's hit
    /// carries its verb, so a click, the keyboard ring and an assistive technology all dispatch it the same way; a
    /// disabled one paints muted, dispatches nothing and names the refusal that disables it. No button registers while a
    /// modal dialog (the finalize prompt) owns the pointer.
    pub(super) fn render_time_travel_band_step(&mut self, cursor: &mut ShellChromeChildCursor, overlay: &mut DrawList, atlas: &mut FontAtlas, input: &mut InputState<ActionDescriptor>, theme: &Theme) -> bool {
        let Some(status) = self.history_time_travel.clone() else { return true };
        let plan = self.time_travel_band_plan_for(&status, theme);
        let (border, fill, text_color) = time_travel_band_tone(&status, theme);
        let baseline = |rect: Rect| rect.y + (rect.h + theme.font_size_small) * 0.5 - 1.0;
        let buttons_from = CHROME_BAND_FRAME_STEPS + plan.lines.len();
        match cursor.scalar {
            scalar if scalar < CHROME_BAND_FRAME_STEPS => paint_chrome_band_frame(scalar, overlay, plan.band, plan.progress, (border, fill), theme),
            scalar if scalar < buttons_from => {
                let line = &plan.lines[scalar - CHROME_BAND_FRAME_STEPS];
                match chrome_text_complete_step(overlay, atlas, &line.text, line.rect.x, baseline(line.rect), line.rect.w, theme.font_size_small, text_color, &mut cursor.glyph) {
                    Ok(false) => return false,
                    Ok(true) => {}
                    Err(()) => {
                        self.error = Some("Shell time-travel band text exceeded the retained glyph boundary".to_string());
                        cursor.glyph.reset();
                    }
                }
            }
            scalar => {
                let Some(button) = plan.buttons.get((scalar - buttons_from) / 3) else { return true };
                let enabled = button.control.disabled_by.is_none();
                match (scalar - buttons_from) % 3 {
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
                            note_chrome_control_name(control_id, Some(button.control.verb.label(self.band_tongue())));
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

    /// 🔊️ The band's live node: ONE polite status in every stage — its role never changes — named by the band's own message
    /// so a reader hears exactly what is painted, busy while the runtime replays or finalizes. It takes focus programmatically
    /// (never by Tab) — React's band region — so a replay or a review moves the reader onto the band. A running replay's
    /// numbers are [`Self::time_travel_progress_accessibility_node`].
    pub(crate) fn time_travel_status_accessibility_node(&self, node_id: u64) -> Option<ui_contract::AccessibilityProjectionNode> {
        let status = self.history_time_travel.as_ref()?;
        let mut node = chrome_status_accessibility_node(node_id, TIME_TRAVEL_BAND_STATUS_ID, time_travel_band_lines(status, self.active_terminology(), self.band_tongue()).message());
        node.focusable = true;
        node.focused = self.accessibility_focused_control_id.as_deref() == Some(TIME_TRAVEL_BAND_STATUS_ID);
        node.busy = matches!(status.stage, HistoryTimeTravelStage::Replaying | HistoryTimeTravelStage::Finalizing);
        Some(node)
    }

    /// 📶️ Preparation or replay progress as its own node under the band's status — React's `<progress>` inside the band: a progress
    /// bar named and valued by the progress line while known-total work runs, `None` otherwise.
    pub(crate) fn time_travel_progress_accessibility_node(&self, node_id: u64) -> Option<ui_contract::AccessibilityProjectionNode> {
        let status = self.history_time_travel.as_ref()?;
        let total = status.total.filter(|_| time_travel_progress_key(status.stage).is_some())?;
        let line = time_travel_band_lines(status, self.active_terminology(), self.band_tongue()).progress?;
        let mut node = chrome_status_accessibility_node(node_id, TIME_TRAVEL_BAND_PROGRESS_ID, line);
        node.role = "progressbar".into();
        node.depth = 1;
        node.live = ui_contract::liveness_name(ui_contract::Liveness::Off).to_string();
        node.value_min = Some(0.0);
        node.value_max = Some(f64::from(total));
        node.value_now = Some(f64::from(status.done.unwrap_or(0)));
        node.value_text = node.label.clone();
        Some(node)
    }

    /// 📡️ Takes the history change one history patch says replays before this replica adopts it (absent = none waits).
    pub(crate) fn observe_history_reprojection(&mut self, reprojection: Option<&HistoryReprojection>) {
        self.history_reprojection = reprojection.cloned();
    }

    /// 📢️ The replaying history change in the kernel's one status copy (`kernel::history_reprojection_status`, en/de), or `None`
    /// while none waits.
    pub(crate) fn history_reprojection_status(&self) -> Option<HistoryReprojectionStatus> {
        Some(semio_framework::kernel::history_reprojection_status(self.history_reprojection.as_ref()?, self.active_terminology(), self.active_locale()))
    }

    /// 📐️ This frame's reprojection band, or `None` while no history change waits.
    pub(crate) fn history_reprojection_band_plan_for(&self, theme: &Theme) -> Option<(HistoryReprojectionStatus, HistoryReprojectionBandPlan)> {
        let status = self.history_reprojection_status()?;
        let floor = self.history_time_travel.as_ref().map_or(self.screen_h, |session| self.time_travel_band_plan_for(session, theme).band.y);
        let control = history_reprojection_control(&status, self.history_time_travel.is_some());
        let plan = history_reprojection_band_plan(&status, control, floor, self.screen_w, self.band_tongue(), theme);
        Some((status, plan))
    }

    /// 📡️ Paints the band of a history change replaying before adoption, one scalar, glyph run or hit per opportunity — the shared
    /// band frame with its replay track, each line of its message, then its control's fill, caption and hit (Cancel replay or Replay
    /// again on the session controller, no generation) — stacked above the session band while one is open, which then owns the
    /// controls. No control registers while a modal dialog owns the pointer.
    pub(super) fn render_history_reprojection_band_step(&mut self, cursor: &mut ShellChromeChildCursor, overlay: &mut DrawList, atlas: &mut FontAtlas, input: &mut InputState<ActionDescriptor>, theme: &Theme) -> bool {
        let Some((status, plan)) = self.history_reprojection_band_plan_for(theme) else { return true };
        let (border, fill, text_color) = history_reprojection_band_tone(&status, theme);
        let baseline = |rect: Rect| rect.y + (rect.h + theme.font_size_small) * 0.5 - 1.0;
        let control_from = CHROME_BAND_FRAME_STEPS + plan.lines.len();
        match cursor.scalar {
            scalar if scalar < CHROME_BAND_FRAME_STEPS => paint_chrome_band_frame(scalar, overlay, plan.band, plan.progress, (border, fill), theme),
            scalar if scalar < control_from => {
                let line = &plan.lines[scalar - CHROME_BAND_FRAME_STEPS];
                match chrome_text_complete_step(overlay, atlas, &line.text, line.rect.x, baseline(line.rect), line.rect.w, theme.font_size_small, text_color, &mut cursor.glyph) {
                    Ok(false) => return false,
                    Ok(true) => {}
                    Err(()) => {
                        self.error = Some("Shell history reprojection band text exceeded the retained glyph boundary".to_string());
                        cursor.glyph.reset();
                    }
                }
            }
            scalar => {
                let Some((verb, rect)) = plan.control.filter(|_| scalar < control_from + 3) else { return true };
                let tongue = self.band_tongue();
                match scalar - control_from {
                    0 => overlay.push_rounded([rect.x, rect.y, rect.w, rect.h], theme.button, theme.border_radius),
                    1 => match chrome_text_complete_step(overlay, atlas, verb.label(tongue), rect.x + theme.padding_standard, baseline(rect), (rect.w - theme.padding_standard).max(1.0), theme.font_size_small, theme.text, &mut cursor.glyph) {
                        Ok(false) => return false,
                        Ok(true) => {}
                        Err(()) => {
                            self.error = Some("Shell history reprojection band button text exceeded the retained glyph boundary".to_string());
                            cursor.glyph.reset();
                        }
                    },
                    _ => {
                        if let Some(controller_id) = self.session.as_ref().map(|session| session.app.controller_id.clone()).filter(|_| !self.chrome_build.dialog_open()) {
                            let control_id = history_reprojection_control_id(verb);
                            note_chrome_control_name(control_id, Some(verb.label(tongue)));
                            input.register_hit(HitTarget { rect, event: Some(ActionDescriptor { controller_id, action: verb.action_id().to_string(), args: None }), control_id: Some(control_id.to_string()), kind: HitKind::Button, drag_axis: None, drag_data: None });
                        }
                    }
                }
            }
        }
        cursor.scalar += 1;
        false
    }

    /// 🔊️ The reprojection band's live node (audit W1E-3): ONE polite status whether the change replays, is paused or was
    /// refused — its role never changes — named by exactly the painted, localized message `<title>: <text>` (a refusal's code is
    /// never announced) and busy while it replays, so a reader hears an interior Undo, a peer's history change or a document load
    /// replaying without opening the History panel. Its numbers are [`Self::history_reprojection_progress_accessibility_node`].
    pub(crate) fn history_reprojection_accessibility_node(&self, node_id: u64) -> Option<ui_contract::AccessibilityProjectionNode> {
        let status = self.history_reprojection_status()?;
        let mut node = chrome_status_accessibility_node(node_id, HISTORY_REPROJECTION_STATUS_ID, history_reprojection_message(&status));
        node.busy = history_reprojection_running(&status);
        Some(node)
    }

    /// 🧭️ A replaying history change's progress as its own node beside the status — React's `<progress>` of the reprojection
    /// status: named by the title, valued by done of total, its value text the status line; `None` while nothing replays.
    pub(crate) fn history_reprojection_progress_accessibility_node(&self, node_id: u64) -> Option<ui_contract::AccessibilityProjectionNode> {
        let status = self.history_reprojection_status().filter(history_reprojection_running)?;
        let mut node = chrome_status_accessibility_node(node_id, HISTORY_REPROJECTION_PROGRESS_ID, status.title.clone());
        node.role = "progressbar".into();
        node.live = ui_contract::liveness_name(ui_contract::Liveness::Off).to_string();
        node.value_min = Some(0.0);
        node.value_max = Some(f64::from(status.total));
        node.value_now = Some(f64::from(status.done));
        node.value_text = Some(status.text.clone());
        Some(node)
    }

    /// 📣️ Both bands' live nodes in reading order — each status followed by its progress — numbered from `first_id`.
    pub(crate) fn band_accessibility_nodes(&self, first_id: u64) -> Vec<ui_contract::AccessibilityProjectionNode> {
        let mut nodes = Vec::new();
        nodes.extend(self.time_travel_status_accessibility_node(first_id));
        nodes.extend(self.time_travel_progress_accessibility_node(first_id + nodes.len() as u64));
        nodes.extend(self.history_reprojection_accessibility_node(first_id + nodes.len() as u64));
        nodes.extend(self.history_reprojection_progress_accessibility_node(first_id + nodes.len() as u64));
        nodes
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
        let tongue = self.band_tongue();
        let description = time_travel_indicator_text(status, self.active_terminology(), tongue);
        let control_id = time_travel_indicator_control_id(window_id);
        let item = ChromeGroupItem { control_id: control_id.as_str(), icon_id: Some("clock"), label: Some(time_travel_indicator_caption(tongue)), active: true, disabled: false, kind: HitKind::Generic };
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
