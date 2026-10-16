//! 🎛️ `shell-prefs` staging of `time-travel`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;
use super::time_travel::*;
use semio_framework::kernel::{HistoryReprojection, HistoryReprojectionStatus, HistoryTimeTravel, HistoryTimeTravelReview, HistoryTimeTravelStage};
use semio_framework_time_travel::TimeTravelLabel;

//#region ⏪️TimeTravelControls
/// 📐️ The band's widest extent before its message is clipped.
const TIME_TRAVEL_BAND_MAX_WIDTH: f32 = 880.0;

/// 📐️ Height of the replay progress track along the band's lower edge.
pub(super) const TIME_TRAVEL_PROGRESS_TRACK: f32 = 3.0;

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

//#endregion ⏪️TimeTravelControls
//#region 🗣️TimeTravelCopy
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

pub(super) const BAND_LABEL_TARGET: &str = "ui.timeTravel.target";

pub(super) const BAND_LABEL_PROGRESS: &str = "ui.timeTravel.progress";

pub(super) const BAND_LABEL_PREPARATION_PROGRESS: &str = "ui.timeTravel.preparationProgress";

pub(super) const BAND_LABEL_PROCESSED: &str = "ui.timeTravel.processed";

pub(super) const BAND_LABEL_WORST: &str = "ui.timeTravel.worst";

pub(super) const BAND_LABEL_REPLAY_FAULTED: &str = "ui.timeTravel.refusal.replayFaulted";

pub(super) const BAND_LABEL_ACCEPTED: &str = "ui.timeTravel.accepted";

/// ❔️ What a band string reads when the corpus lacks its key: the replacement character — never empty, in no language, and
/// kept off every screen by the law that requires each key the shell reads.
pub(crate) const BAND_LABEL_MISSING: &str = "\u{FFFD}";

/// 🏷️ One corpus label on `tongue`. Total: a key the corpus lacks reads [`BAND_LABEL_MISSING`], never an empty string.
pub(crate) fn band_label(key: &str, tongue: impl Into<BandTongue>) -> &'static str {
    band_label_in_catalog(key, tongue).unwrap_or(BAND_LABEL_MISSING)
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
pub(super) fn time_travel_stage_key(stage: HistoryTimeTravelStage) -> &'static str {
    match stage {
        HistoryTimeTravelStage::Editing => "ui.timeTravel.stage.editing",
        HistoryTimeTravelStage::Replaying => "ui.timeTravel.stage.replaying",
        HistoryTimeTravelStage::Reviewing => "ui.timeTravel.stage.reviewing",
        HistoryTimeTravelStage::Choosing => "ui.timeTravel.stage.choosing",
        HistoryTimeTravelStage::Finalizing => "ui.timeTravel.stage.finalizing",
    }
}

/// 🔬️ The corpus label key of what a review shows.
pub(super) fn time_travel_review_key(review: HistoryTimeTravelReview) -> &'static str {
    match review {
        HistoryTimeTravelReview::NoChanges => "ui.timeTravel.review.noChanges",
        HistoryTimeTravelReview::NeedsReplay => "ui.timeTravel.review.needsReplay",
        HistoryTimeTravelReview::Blocked => "ui.timeTravel.review.blocked",
        HistoryTimeTravelReview::Ready => "ui.timeTravel.review.ready",
    }
}

/// ✂️ A dispatch-fault string's whole tokens, in order — a code is only ever a whole token.
pub(super) fn fault_tokens(fault: &str) -> impl Iterator<Item = &str> {
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
pub(super) fn time_travel_target_text(status: &HistoryTimeTravel, terminology: Terminology, locale: Locale) -> Option<String> {
    status.target_label.as_ref().map(|label| label.resolve(terminology, locale).to_string()).filter(|target| !target.is_empty())
}

/// ⏳️ Names the running preparation or replay through its canonical band copy.
pub(super) fn time_travel_progress_key(stage: HistoryTimeTravelStage) -> Option<&'static str> {
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

/// 📐️ A bottom-centre chrome band laid out: its box, the message's lines and one rect per button in the order given.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ChromeBandLayout {
    pub band: Rect,
    pub lines: Vec<ChromeBandLine>,
    pub buttons: Vec<Rect>,
}

/// 🧵️ The message's lines within `width`: whole `separator` segments joined while they fit, a segment wider than a line
/// broken at its words, a word wider than a line on a line of its own (clipped when painted).
pub(super) fn chrome_band_wrap(message: &str, separator: &str, width: f32, glyph_w: impl Fn(&str) -> f32) -> Vec<String> {
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

//#endregion 📐️TimeTravelBand
//#region 📡️HistoryReprojectionBand
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

//#endregion 📡️HistoryReprojectionBand
impl PrefsState {
    /// 🗣️ The axes this shell's band copy resolves on: its locale and the UI driver's label tier.
    pub(crate) fn band_tongue(&self, chrome: &ChromeState) -> BandTongue {
        BandTongue { locale: self.active_locale(), beginner: chrome.chrome_build.driver.label_tier == ui_wgpu::wgpu::UiDriverLabelTier::Beginner }
    }

    /// 📐️ This frame's band, localized and with each chorded button's inline badge.
    pub(crate) fn time_travel_band_plan_for(&self, status: &HistoryTimeTravel, theme: &Theme, chrome: &ChromeState) -> TimeTravelBandPlan {
        let tongue = self.band_tongue(chrome);
        let message = time_travel_band_lines(status, self.active_terminology(), tongue).message();
        let buttons = time_travel_band_controls(status).into_iter().map(|control| (control, chrome.chrome_control_label(control.verb.control_id(), control.verb.label(tongue)))).collect();
        time_travel_band_plan(status, message, buttons, chrome.screen_w, chrome.screen_h, theme)
    }

    /// 📢️ The replaying history change in the kernel's one status copy (`kernel::history_reprojection_status`, en/de), or `None`
    /// while none waits.
    pub(crate) fn history_reprojection_status(&self, history: &HistoryState) -> Option<HistoryReprojectionStatus> {
        Some(semio_framework::kernel::history_reprojection_status(history.history_reprojection.as_ref()?, self.active_terminology(), self.active_locale()))
    }

    /// 📐️ This frame's reprojection band, or `None` while no history change waits.
    pub(crate) fn history_reprojection_band_plan_for(&self, theme: &Theme, chrome: &ChromeState, history: &HistoryState) -> Option<(HistoryReprojectionStatus, HistoryReprojectionBandPlan)> {
        let status = self.history_reprojection_status(history)?;
        let floor = history.history_time_travel.as_ref().map_or(chrome.screen_h, |session| self.time_travel_band_plan_for(session, theme, chrome).band.y);
        let control = history_reprojection_control(&status, history.history_time_travel.is_some());
        let plan = history_reprojection_band_plan(&status, control, floor, chrome.screen_w, self.band_tongue(chrome), theme);
        Some((status, plan))
    }
}

