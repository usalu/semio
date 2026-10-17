//! 🌍️ Labels of the time-travel session: the code label table, the label enum and its English and German texts. See [`super`].

use super::*;

//#region 🔖️Labels
/// 🗂️ Every `timeTravel.*` code a history-edit verb or driver answers, with the label a host shows for it.
pub const TIME_TRAVEL_CODE_LABELS: [(&str, TimeTravelLabel); 21] = [
    (TIME_TRAVEL_FROZEN_CODE, TimeTravelLabel::Frozen),
    ("timeTravel.illegal", TimeTravelLabel::RefusalIllegal),
    ("timeTravel.stale", TimeTravelLabel::RefusalStale),
    ("timeTravel.blocked", TimeTravelLabel::RefusalBlocked),
    ("timeTravel.empty", TimeTravelLabel::RefusalEmpty),
    ("timeTravel.unchanged", TimeTravelLabel::RefusalUnchanged),
    (TIME_TRAVEL_CANCELLED_CODE, TimeTravelLabel::ReplayCancelled),
    (TIME_TRAVEL_BUSY_CODE, TimeTravelLabel::RefusalBusy),
    (TIME_TRAVEL_UNKNOWN_MUTATION_CODE, TimeTravelLabel::RefusalUnknownMutation),
    (TIME_TRAVEL_NOT_EDITABLE_CODE, TimeTravelLabel::RefusalNotEditable),
    (TIME_TRAVEL_UNKNOWN_INPUT_CODE, TimeTravelLabel::RefusalUnknownInput),
    (TIME_TRAVEL_INVALID_INPUT_CODE, TimeTravelLabel::RefusalInvalidInput),
    (TIME_TRAVEL_NO_SELECTION_CODE, TimeTravelLabel::RefusalNoSelection),
    (TIME_TRAVEL_NAME_REQUIRED_CODE, TimeTravelLabel::RefusalNameRequired),
    (TIME_TRAVEL_NAME_INVALID_CODE, TimeTravelLabel::RefusalNameInvalid),
    (TIME_TRAVEL_SCHEMA_UNAVAILABLE_CODE, TimeTravelLabel::RefusalSchemaUnavailable),
    (TIME_TRAVEL_REPLAY_FAULTED_CODE, TimeTravelLabel::ReplayFaulted),
    (TIME_TRAVEL_COMMIT_FAILED_CODE, TimeTravelLabel::CommitFailed),
    (TIME_TRAVEL_MEMBER_GONE_CODE, TimeTravelLabel::RefusalMemberGone),
    (TIME_TRAVEL_NOT_WITHDRAWABLE_CODE, TimeTravelLabel::RefusalNotWithdrawable),
    (TIME_TRAVEL_EDITOR_CLOSED_CODE, TimeTravelLabel::RefusalEditorClosed),
];

/// 💬️ Framework-owned EN/DE text of history editing, no default locale.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TimeTravelLabel {
    StageInactive,
    StageEditing,
    StageReplaying,
    StageReviewing,
    StageChoosing,
    StageFinalizing,
    RefusalIllegal,
    RefusalStale,
    RefusalBlocked,
    RefusalEmpty,
    RefusalUnchanged,
    Frozen,
    ChoiceOverwrite,
    ChoiceOverwriteDescription,
    ChoiceAlternative,
    ChoiceAlternativeDescription,
    AlternativeNameDefault,
    NoChanges,
    NeedsReplay,
    ReportBlocking,
    ReadyToFinalize,
    ReplayCancelled,
    ActionRerun,
    PreparationProgress,
    PreparationProgressValueText,
    ReplayProgressValueText,
    Processed,
    RefusalBusy,
    RefusalUnknownMutation,
    RefusalNotEditable,
    RefusalUnknownInput,
    RefusalInvalidInput,
    RefusalNoSelection,
    RefusalNameRequired,
    RefusalNameInvalid,
    RefusalSchemaUnavailable,
    ReplayFaulted,
    CommitFailed,
    OutcomeIntroduced,
    RefusalMemberGone,
    MemberEdited,
    RefusalNotWithdrawable,
    RefusalEditorClosed,
    RefusalReadOnly,
}

impl TimeTravelLabel {
    pub const ALL: [Self; 44] = [
        Self::StageInactive,
        Self::StageEditing,
        Self::StageReplaying,
        Self::StageReviewing,
        Self::StageChoosing,
        Self::StageFinalizing,
        Self::RefusalIllegal,
        Self::RefusalStale,
        Self::RefusalBlocked,
        Self::RefusalEmpty,
        Self::RefusalUnchanged,
        Self::Frozen,
        Self::ChoiceOverwrite,
        Self::ChoiceOverwriteDescription,
        Self::ChoiceAlternative,
        Self::ChoiceAlternativeDescription,
        Self::AlternativeNameDefault,
        Self::NoChanges,
        Self::NeedsReplay,
        Self::ReportBlocking,
        Self::ReadyToFinalize,
        Self::ReplayCancelled,
        Self::ActionRerun,
        Self::PreparationProgress,
        Self::PreparationProgressValueText,
        Self::ReplayProgressValueText,
        Self::Processed,
        Self::RefusalBusy,
        Self::RefusalUnknownMutation,
        Self::RefusalNotEditable,
        Self::RefusalUnknownInput,
        Self::RefusalInvalidInput,
        Self::RefusalNoSelection,
        Self::RefusalNameRequired,
        Self::RefusalNameInvalid,
        Self::RefusalSchemaUnavailable,
        Self::ReplayFaulted,
        Self::CommitFailed,
        Self::OutcomeIntroduced,
        Self::RefusalMemberGone,
        Self::MemberEdited,
        Self::RefusalNotWithdrawable,
        Self::RefusalEditorClosed,
        Self::RefusalReadOnly,
    ];

    /// 🔑️ `(key, en, de)` row of this label.
    pub(super) fn row(self) -> (&'static str, &'static str, &'static str) {
        match self {
            Self::StageInactive => ("stageInactive", "Not editing history", "Verlauf wird nicht bearbeitet"),
            Self::StageEditing => ("stageEditing", "Editing a mutation", "Mutation wird bearbeitet"),
            Self::StageReplaying => ("stageReplaying", "Replaying later mutations", "Spätere Mutationen werden neu angewendet"),
            Self::StageReviewing => ("stageReviewing", "Reviewing the edited history", "Bearbeiteter Verlauf wird geprüft"),
            Self::StageChoosing => ("stageChoosing", "Choose how to finalize", "Art des Abschlusses wählen"),
            Self::StageFinalizing => ("stageFinalizing", "Finalizing the history edit", "Verlaufsbearbeitung wird abgeschlossen"),
            Self::RefusalIllegal => ("refusalIllegal", "Not possible right now", "Derzeit nicht möglich"),
            Self::RefusalStale => ("refusalStale", "Outdated request ignored", "Veraltete Anfrage ignoriert"),
            Self::RefusalBlocked => ("refusalBlocked", "Blocked: resolve the pending change or the errors first", "Blockiert: zuerst die offene Änderung oder die Fehler auflösen"),
            Self::RefusalEmpty => ("refusalEmpty", "Nothing to finalize: no accepted changes", "Nichts abzuschließen: keine übernommenen Änderungen"),
            Self::RefusalUnchanged => ("refusalUnchanged", "Change an input before accepting", "Vor dem Übernehmen eine Eingabe ändern"),
            Self::Frozen => ("frozen", "Editing is paused while history is being edited", "Bearbeiten ist pausiert, solange der Verlauf bearbeitet wird"),
            Self::ChoiceOverwrite => ("choiceOverwrite", "Overwrite history", "Verlauf überschreiben"),
            Self::ChoiceOverwriteDescription => ("choiceOverwriteDescription", "Replaces the inputs in every alternative that contains these mutations", "Ersetzt die Eingaben in jeder Alternative, die diese Mutationen enthält"),
            Self::ChoiceAlternative => ("choiceAlternative", "New alternative", "Neue Alternative"),
            Self::ChoiceAlternativeDescription => ("choiceAlternativeDescription", "Keeps the original history and continues in a new alternative", "Behält den ursprünglichen Verlauf und arbeitet in einer neuen Alternative weiter"),
            Self::AlternativeNameDefault => ("alternativeNameDefault", "Edited history", "Bearbeiteter Verlauf"),
            Self::NoChanges => ("noChanges", "No changes: showing the current history", "Keine Änderungen: aktueller Verlauf wird angezeigt"),
            Self::NeedsReplay => ("needsReplay", "Replay needed: later mutations are not checked yet", "Neu anwenden nötig: spätere Mutationen sind noch nicht geprüft"),
            Self::ReportBlocking => ("reportBlocking", "Errors must be fixed or withdrawn before finalizing", "Fehler müssen vor dem Abschließen behoben oder zurückgezogen werden"),
            Self::ReadyToFinalize => ("readyToFinalize", "Ready to finalize", "Bereit zum Abschließen"),
            Self::ReplayCancelled => ("replayCancelled", "Replay cancelled", "Neu anwenden abgebrochen"),
            Self::ActionRerun => ("actionRerun", "Replay again", "Erneut anwenden"),
            Self::PreparationProgress => ("preparationProgress", "Preparing history preview", "Verlaufsvorschau wird vorbereitet"),
            Self::PreparationProgressValueText => ("preparationProgressValueText", "Preparing history preview: {done} of {total} steps", "Verlaufsvorschau wird vorbereitet: {done} von {total} Schritten"),
            Self::ReplayProgressValueText => ("replayProgressValueText", "Replaying history: {done} of {total} steps", "Verlauf wird neu angewendet: {done} von {total} Schritten"),
            Self::Processed => ("processed", "Work completed: {processed}", "Arbeitsfortschritt: {processed}"),
            Self::RefusalBusy => ("refusalBusy", "History editing is busy: finish the running tool or the other history edit first", "Verlaufsbearbeitung beschäftigt: zuerst das laufende Werkzeug oder die andere Verlaufsbearbeitung abschließen"),
            Self::RefusalUnknownMutation => ("refusalUnknownMutation", "This mutation is no longer in the history", "Diese Mutation ist nicht mehr im Verlauf"),
            Self::RefusalNotEditable => ("refusalNotEditable", "The inputs of this mutation cannot be edited", "Die Eingaben dieser Mutation können nicht bearbeitet werden"),
            Self::RefusalUnknownInput => ("refusalUnknownInput", "This input does not exist in the mutation", "Diese Eingabe gibt es in der Mutation nicht"),
            Self::RefusalInvalidInput => ("refusalInvalidInput", "Invalid value: the input keeps its previous value", "Ungültiger Wert: Die Eingabe behält ihren bisherigen Wert"),
            Self::RefusalNoSelection => ("refusalNoSelection", "Nothing suitable is selected for this input", "Für diese Eingabe ist nichts Passendes ausgewählt"),
            Self::RefusalNameRequired => ("refusalNameRequired", "Name the new alternative", "Einen Namen für die neue Alternative eingeben"),
            Self::RefusalNameInvalid => ("refusalNameInvalid", "Invalid alternative name: use 1 to 256 characters", "Ungültiger Name der Alternative: 1 bis 256 Zeichen verwenden"),
            Self::RefusalSchemaUnavailable => ("refusalSchemaUnavailable", "The input schema of this mutation is unavailable", "Das Eingabeschema dieser Mutation ist nicht verfügbar"),
            Self::ReplayFaulted => ("replayFaulted", "Replay failed: later mutations could not be checked", "Erneutes Anwenden fehlgeschlagen: Spätere Mutationen konnten nicht geprüft werden"),
            Self::CommitFailed => ("commitFailed", "Finalizing failed: the history is unchanged", "Abschließen fehlgeschlagen: Der Verlauf ist unverändert"),
            Self::OutcomeIntroduced => ("outcomeIntroduced", "New since this edit", "Neu durch diese Bearbeitung"),
            Self::RefusalMemberGone => ("refusalMemberGone", "The part this history edit targets was closed", "Der Teil, den diese Verlaufsbearbeitung betrifft, wurde geschlossen"),
            Self::MemberEdited => ("memberEdited", "History of a composed part edited", "Verlauf eines eingebetteten Teils bearbeitet"),
            Self::RefusalNotWithdrawable => ("refusalNotWithdrawable", "This mutation cannot be withdrawn here", "Diese Mutation kann hier nicht zurückgezogen werden"),
            Self::RefusalEditorClosed => ("refusalEditorClosed", "The draft editor is closed: open the mutation again", "Der Entwurfseditor ist geschlossen: die Mutation erneut öffnen"),
            Self::RefusalReadOnly => ("refusalReadOnly", "History cannot be edited in a read-only view", "Der Verlauf kann in einer schreibgeschützten Ansicht nicht bearbeitet werden"),
        }
    }

    pub fn key(self) -> &'static str {
        self.row().0
    }

    pub fn parse(key: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|label| label.key() == key)
    }

    pub fn en(self) -> &'static str {
        self.row().1
    }

    pub fn de(self) -> &'static str {
        self.row().2
    }

    /// 🩹️ Label of every `timeTravel.*` code a host shows — the session's refusals, the frozen and cancelled codes, the
    /// hosting runtime's refusals and driver faults ([`TIME_TRAVEL_CODE_LABELS`]); `None` for any other code.
    pub fn for_code(code: &str) -> Option<Self> {
        TIME_TRAVEL_CODE_LABELS.iter().find(|(known, _)| *known == code).map(|(_, label)| *label)
    }

    /// 🌐️ Hands both locales to a carrier constructor, e.g. `label.localized(LocalizedLabel::native)`.
    pub fn localized<L>(self, native: impl FnOnce(&'static str, &'static str) -> L) -> L {
        let (_, en, de) = self.row();
        native(en, de)
    }
}
//#endregion 🔖️Labels
