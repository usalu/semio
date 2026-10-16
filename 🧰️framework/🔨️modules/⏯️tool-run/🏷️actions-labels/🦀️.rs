//! 🏷️ Tool run actions and labels: the framework-reserved action ids, chords and the EN/DE panel text.

use super::*;

//#region 🔖️Actions
/// ▶️ Framework-reserved action id: start a run.
pub const TOOL_RUN_START_ACTION_ID: &str = "toolRunStart";
/// ⏸️ Framework-reserved action id: pause a running run.
pub const TOOL_RUN_PAUSE_ACTION_ID: &str = "toolRunPause";
/// 🔂️ Framework-reserved action id: resume a paused run.
pub const TOOL_RUN_RESUME_ACTION_ID: &str = "toolRunResume";
/// ⏭️ Framework-reserved action id: drive exactly one algorithm unit.
pub const TOOL_RUN_STEP_ACTION_ID: &str = "toolRunStep";
/// ⏹️ Framework-reserved action id: abort without touching the store.
pub const TOOL_RUN_ABORT_ACTION_ID: &str = "toolRunAbort";
/// 🎉️ Framework-reserved action id: publish the complete run as one edit.
pub const TOOL_RUN_FINALIZE_ACTION_ID: &str = "toolRunFinalize";
/// ✖️ Framework-reserved action id: clear a terminal run's trace and panel.
pub const TOOL_RUN_DISMISS_ACTION_ID: &str = "toolRunDismiss";
/// 📚️ Every framework-reserved tool run action id.
pub const TOOL_RUN_ACTION_IDS: [&str; 7] = [TOOL_RUN_START_ACTION_ID, TOOL_RUN_PAUSE_ACTION_ID, TOOL_RUN_RESUME_ACTION_ID, TOOL_RUN_STEP_ACTION_ID, TOOL_RUN_ABORT_ACTION_ID, TOOL_RUN_FINALIZE_ACTION_ID, TOOL_RUN_DISMISS_ACTION_ID];

/// 🎹️ Chord of `toolRunStart`.
pub const TOOL_RUN_START_CHORD: &str = "mod+enter";
/// 🔀️ Toggle chord shared by `toolRunPause` and `toolRunResume`.
pub const TOOL_RUN_PAUSE_RESUME_CHORD: &str = "mod+alt+enter";
/// 👣️ Chord of `toolRunStep`.
pub const TOOL_RUN_STEP_CHORD: &str = "mod+alt+arrowright";
/// ⛔️ Chord of `toolRunAbort`.
pub const TOOL_RUN_ABORT_CHORD: &str = "mod+.";
/// 🥅️ Chord of `toolRunFinalize`.
pub const TOOL_RUN_FINALIZE_CHORD: &str = "mod+shift+enter";
/// 🚪️ Chord of `toolRunDismiss` (panel focus only).
pub const TOOL_RUN_DISMISS_CHORD: &str = "escape";

/// 🆔️ Action argument names.
pub const TOOL_RUN_ARG_TOOL_ID: &str = "toolId";
pub const TOOL_RUN_ARG_WINDOW_ID: &str = "windowId";
pub const TOOL_RUN_ARG_RUN_ID: &str = "runId";
pub const TOOL_RUN_ARG_GENERATION: &str = "generation";

/// 📌️ One declared action argument.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ToolRunActionArg {
    pub name: &'static str,
    pub required: bool,
}

/// 🕹️ The seven generic tool run actions (§2.5).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ToolRunAction {
    Start,
    Pause,
    Resume,
    Step,
    Abort,
    Finalize,
    Dismiss,
}

impl ToolRunAction {
    pub const ALL: [Self; 7] = [Self::Start, Self::Pause, Self::Resume, Self::Step, Self::Abort, Self::Finalize, Self::Dismiss];

    pub fn id(self) -> &'static str {
        TOOL_RUN_ACTION_IDS[self as usize]
    }

    pub fn from_id(id: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|action| action.id() == id)
    }

    pub fn chord(self) -> &'static str {
        match self {
            Self::Start => TOOL_RUN_START_CHORD,
            Self::Pause | Self::Resume => TOOL_RUN_PAUSE_RESUME_CHORD,
            Self::Step => TOOL_RUN_STEP_CHORD,
            Self::Abort => TOOL_RUN_ABORT_CHORD,
            Self::Finalize => TOOL_RUN_FINALIZE_CHORD,
            Self::Dismiss => TOOL_RUN_DISMISS_CHORD,
        }
    }

    pub fn label(self) -> ToolRunLabel {
        match self {
            Self::Start => ToolRunLabel::ActionStart,
            Self::Pause => ToolRunLabel::ActionPause,
            Self::Resume => ToolRunLabel::ActionResume,
            Self::Step => ToolRunLabel::ActionStep,
            Self::Abort => ToolRunLabel::ActionAbort,
            Self::Finalize => ToolRunLabel::ActionFinalize,
            Self::Dismiss => ToolRunLabel::ActionDismiss,
        }
    }

    pub fn args(self) -> &'static [ToolRunActionArg] {
        const START: [ToolRunActionArg; 2] = [ToolRunActionArg { name: TOOL_RUN_ARG_TOOL_ID, required: true }, ToolRunActionArg { name: TOOL_RUN_ARG_WINDOW_ID, required: false }];
        const TARGETED: [ToolRunActionArg; 2] = [ToolRunActionArg { name: TOOL_RUN_ARG_RUN_ID, required: true }, ToolRunActionArg { name: TOOL_RUN_ARG_GENERATION, required: true }];
        const DISMISS: [ToolRunActionArg; 1] = [ToolRunActionArg { name: TOOL_RUN_ARG_RUN_ID, required: true }];
        match self {
            Self::Start => &START,
            Self::Dismiss => &DISMISS,
            _ => &TARGETED,
        }
    }

    /// 🟢️ Whether the action is enabled for a slot in `state` (`None` = no run).
    pub fn is_legal_in(self, state: Option<ToolRunState>) -> bool {
        use ToolRunState as S;
        match (self, state) {
            (Self::Start, None) => true,
            (Self::Start | Self::Dismiss, Some(state)) => state.is_terminal(),
            (Self::Pause, Some(S::Running)) | (Self::Resume | Self::Step, Some(S::Paused)) | (Self::Finalize, Some(S::Running | S::Paused | S::Complete)) => true,
            (Self::Abort, Some(S::Starting | S::Running | S::Paused | S::Complete | S::Finalizing)) => true,
            _ => false,
        }
    }
}
//#endregion 🔖️Actions

//#region 🔖️Labels
/// 💬️ Framework-owned EN/DE text of the tool run panel (§2.5), no default locale.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ToolRunLabel {
    StateStarting,
    StateRunning,
    StatePaused,
    StateComplete,
    StateFinalizing,
    StateFinalized,
    StateAborting,
    StateAborted,
    StateFaulted,
    ActionStart,
    ActionPause,
    ActionResume,
    ActionStep,
    ActionAbort,
    ActionFinalize,
    ActionDismiss,
    FinalizeDisabled,
    ReadyToStart,
    RebasingStep,
    ConflictStep,
    TraceTruncatedStep,
    ProvisionalCapStep,
    ProgressValueText,
}

impl ToolRunLabel {
    pub const ALL: [Self; 23] = [
        Self::StateStarting,
        Self::StateRunning,
        Self::StatePaused,
        Self::StateComplete,
        Self::StateFinalizing,
        Self::StateFinalized,
        Self::StateAborting,
        Self::StateAborted,
        Self::StateFaulted,
        Self::ActionStart,
        Self::ActionPause,
        Self::ActionResume,
        Self::ActionStep,
        Self::ActionAbort,
        Self::ActionFinalize,
        Self::ActionDismiss,
        Self::FinalizeDisabled,
        Self::ReadyToStart,
        Self::RebasingStep,
        Self::ConflictStep,
        Self::TraceTruncatedStep,
        Self::ProvisionalCapStep,
        Self::ProgressValueText,
    ];

    /// 🔑️ `(key, en, de)` row of this label.
    pub(crate) fn row(self) -> (&'static str, &'static str, &'static str) {
        match self {
            Self::StateStarting => ("stateStarting", "Starting", "Wird gestartet"),
            Self::StateRunning => ("stateRunning", "Running", "Läuft"),
            Self::StatePaused => ("statePaused", "Paused", "Pausiert"),
            Self::StateComplete => ("stateComplete", "Complete, ready to finalize", "Fertig, bereit zum Abschließen"),
            Self::StateFinalizing => ("stateFinalizing", "Finalizing", "Wird abgeschlossen"),
            Self::StateFinalized => ("stateFinalized", "Finalized", "Abgeschlossen"),
            Self::StateAborting => ("stateAborting", "Aborting", "Wird abgebrochen"),
            Self::StateAborted => ("stateAborted", "Aborted, nothing was changed", "Abgebrochen, nichts wurde geändert"),
            Self::StateFaulted => ("stateFaulted", "Failed, nothing was changed", "Fehlgeschlagen, nichts wurde geändert"),
            Self::ActionStart => ("actionStart", "Start", "Starten"),
            Self::ActionPause => ("actionPause", "Pause", "Pausieren"),
            Self::ActionResume => ("actionResume", "Resume", "Fortsetzen"),
            Self::ActionStep => ("actionStep", "Step", "Einzelschritt"),
            Self::ActionAbort => ("actionAbort", "Abort", "Abbrechen"),
            Self::ActionFinalize => ("actionFinalize", "Finalize", "Abschließen"),
            Self::ActionDismiss => ("actionDismiss", "Dismiss", "Schließen"),
            Self::FinalizeDisabled => ("finalizeDisabled", "Available once the run has started", "Verfügbar, sobald der Lauf gestartet ist"),
            Self::ReadyToStart => ("readyToStart", "Ready to start", "Bereit zum Starten"),
            Self::RebasingStep => ("rebasingStep", "Artifact changed, re-applying provisional result", "Artefakt geändert, vorläufiges Ergebnis wird neu angewendet"),
            Self::ConflictStep => ("conflictStep", "{0} provisional changes conflict with the current artifact", "{0} vorläufige Änderungen stehen im Konflikt mit dem aktuellen Artefakt"),
            Self::TraceTruncatedStep => ("traceTruncatedStep", "Oldest {0} rejected attempts are no longer shown", "Die ältesten {0} verworfenen Versuche werden nicht mehr angezeigt"),
            Self::ProvisionalCapStep => ("provisionalCapStep", "Provisional change limit of {0} reached, run completed", "Grenze von {0} vorläufigen Änderungen erreicht, Lauf abgeschlossen"),
            Self::ProgressValueText => ("progressValueText", "{stage} ({i}/{n}): {completed} of {total} {unit} ({pct} %)", "{stage} ({i}/{n}): {completed} von {total} {unit} ({pct} %)"),
        }
    }

    pub fn key(self) -> &'static str {
        self.row().0
    }

    pub fn parse(key: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|label| label.key() == key)
    }

    pub fn text(self, locale: Locale) -> &'static str {
        let (_, en, de) = self.row();
        match locale {
            Locale::En => en,
            Locale::De => de,
        }
    }

    /// 🌐️ Terminology-invariant `LocalizedLabel::native(en, de)`.
    pub fn localized(self) -> LocalizedLabel {
        let (_, en, de) = self.row();
        LocalizedLabel::native(en, de)
    }

    /// 🚨️ Label of a framework-reserved reason code.
    pub fn for_reason(code: u16) -> Option<Self> {
        match code {
            TOOL_RUN_REASON_REBASING => Some(Self::RebasingStep),
            TOOL_RUN_REASON_CONFLICT => Some(Self::ConflictStep),
            TOOL_RUN_REASON_TRACE_TRUNCATED => Some(Self::TraceTruncatedStep),
            TOOL_RUN_REASON_PROVISIONAL_CAP => Some(Self::ProvisionalCapStep),
            _ => None,
        }
    }
}

/// 🔣️ Single-pass `{name}` substitution; unknown or unterminated placeholders stay literal.
pub fn tool_run_format(template: &str, value: impl Fn(&str) -> Option<String>) -> String {
    let mut out = String::with_capacity(template.len());
    let mut rest = template;
    while let Some(open) = rest.find('{') {
        out.push_str(&rest[..open]);
        let after = &rest[open + 1..];
        match after.find(['{', '}']).filter(|close| after.as_bytes()[*close] == b'}').and_then(|close| value(&after[..close]).map(|text| (close, text))) {
            Some((close, text)) => {
                out.push_str(&text);
                rest = &after[close + 1..];
            }
            None => {
                out.push('{');
                rest = after;
            }
        }
    }
    out.push_str(rest);
    out
}
//#endregion 🔖️Labels
