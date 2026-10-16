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
            Self::NextProblem => semio_framework::history_edit::HISTORY_EDIT_BEGIN_ACTION_ID,
            Self::Accept => semio_framework::history_edit::HISTORY_EDIT_ACCEPT_ACTION_ID,
            Self::Discard => semio_framework::history_edit::HISTORY_EDIT_DISCARD_ACTION_ID,
            Self::CancelReplay => semio_framework::history_edit::HISTORY_EDIT_CANCEL_REPLAY_ACTION_ID,
            Self::Rerun => semio_framework::history_edit::HISTORY_EDIT_RERUN_ACTION_ID,
            Self::Finalize => semio_framework::history_edit::HISTORY_EDIT_FINALIZE_ACTION_ID,
            Self::Back => semio_framework::history_edit::HISTORY_EDIT_BACK_ACTION_ID,
            Self::Exit => semio_framework::history_edit::HISTORY_EDIT_EXIT_ACTION_ID,
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

//#endregion 🎯️TimeTravelFocus

//#region 🗣️TimeTravelCopy
#[path = "🗣️labels/🦀️.rs"]
mod labels;
#[path = "🚦️refusals/🦀️.rs"]
mod refusals;

/// 🧾️ Every fixed corpus label key the band copy reads besides its controls, stages, reviews, severities and refusals — the
/// keys a law requires the corpus to carry.
pub(crate) const BAND_FIXED_LABEL_KEYS: [&str; 12] = [BAND_LABEL_TARGET, BAND_LABEL_PROGRESS, BAND_LABEL_PREPARATION_PROGRESS, BAND_LABEL_PROCESSED, BAND_LABEL_WORST, BAND_LABEL_REPLAY_FAULTED, BAND_LABEL_ACCEPTED, BAND_LABEL_INDICATOR_TARGET, BAND_LABEL_INDICATOR, BAND_LABEL_PEER_EDITING_TARGET, BAND_LABEL_PEER_EDITING_HISTORY, BAND_LABEL_PEER_EDITING_ROW];

/// 🔎️ One corpus label on `tongue`, `None` for a key the corpus does not carry or carries empty.
pub(crate) fn band_label_in_catalog(key: &str, tongue: impl Into<BandTongue>) -> Option<&'static str> {
    let tongue: BandTongue = tongue.into();
    labels::label(key, tongue.locale == Locale::De, tongue.beginner).filter(|text| !text.is_empty())
}

/// 🔇️ Whether a shell stays silent about the refusal `code` (the corpus's `silent` rows: an outdated request needs no notice).
pub(crate) fn history_refusal_is_silent(code: &str) -> bool {
    refusals::refusal(code).is_some_and(|(_, _, _, silent)| silent)
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

//#endregion 🗣️TimeTravelCopy

//#region 📐️TimeTravelBand

/// 📏️ One line of a laid-out chrome band: its text and the box it paints in (its baseline centred in the box).
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ChromeBandLine {
    pub text: String,
    pub rect: Rect,
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

//#endregion 📐️TimeTravelBand



//#region 👥️PeerHistoryEdits

/// 🗝️ The node keys the framework history body gives a history row and a mutation child (`🔌️plugin/🦀️.rs`
/// `ui_history_panel`), which a peer's open history edit marks — React's `HISTORY_ROW_KEY_PREFIX` and
/// `HISTORY_MUTATION_ROW_KEY_PREFIX`.
pub(crate) const HISTORY_ROW_KEY_PREFIX: &str = "framework.history.entry.";
pub(crate) const HISTORY_MUTATION_ROW_KEY_PREFIX: &str = "framework.history.mutation.";

//#endregion 👥️PeerHistoryEdits

#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "../../../🧪️tests/🧪️wgpu-time-travel/🦀️.rs"]
mod tests;
