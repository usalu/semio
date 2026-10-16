//! ⏪️ The reserved history-edit verbs of the time-travel session and their finalize dialog, contributed to every hosting app.

use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_manifest_actions::{ActionDefinition, ActionKind};
use semio_framework_manifest_address::ActionRef;
use semio_framework_manifest_app_surface::{DIALOG_CHOICE_ARG, DialogChoice, DialogDefinition, ReservedContribution};
use semio_framework_manifest_arg_schema::{ActionArgDef, ActionArgOption, ArgPresentation, ArgSchema};

//#region 🔖️HistoryEdit
/// ✏️ Opens (or retargets) the history-edit session on one applied mutation: `mutationId`.
pub const HISTORY_EDIT_BEGIN_ACTION_ID: &str = "historyEditBegin";

/// 🎚️ Sets one input of the edited mutation's draft — `path` (RFC 6901 pointer into the payload) and `value` — or, with
/// [`HISTORY_EDIT_ARG_EDIT`], inserts or removes one item of a list input.
pub const HISTORY_EDIT_INPUT_ACTION_ID: &str = "historyEditInput";

/// 🎯️ Sets the reference input at `path` from the current selection of its declared domain.
pub const HISTORY_EDIT_USE_SELECTION_ACTION_ID: &str = "historyEditUseSelection";

/// 🚫️ Drafts a mutation as withdrawn (it folds as a no-op): the one being edited, or — with `mutationId`, a history
/// row's Withdraw — the named one, opening (or retargeting) the session on it; no input schema is needed.
pub const HISTORY_EDIT_WITHDRAW_ACTION_ID: &str = "historyEditWithdraw";

/// ✅️ Accepts the draft and replays everything downstream.
pub const HISTORY_EDIT_ACCEPT_ACTION_ID: &str = "historyEditAccept";

/// ↩️ Drops the draft and returns to the previous stage.
pub const HISTORY_EDIT_DISCARD_ACTION_ID: &str = "historyEditDiscard";

/// 🪃️ Takes the accepted draft of `mutationId` back while reviewing — the mutation applies with its recorded input again —
/// and replays the rest; taking the only accepted draft back leaves history editing with nothing changed.
pub const HISTORY_EDIT_RESTORE_ACTION_ID: &str = "historyEditRestore";

/// 🏁️ Opens the finalize prompt once the replayed history is clean.
pub const HISTORY_EDIT_FINALIZE_ACTION_ID: &str = "historyEditFinalize";

/// 🌿️ Commits the accepted drafts: as a new alternative named `name`, or with `choice: overwrite` over every alternative.
pub const HISTORY_EDIT_COMMIT_ACTION_ID: &str = "historyEditCommit";

/// ⬅️ Leaves the finalize prompt and returns to reviewing.
pub const HISTORY_EDIT_BACK_ACTION_ID: &str = "historyEditBack";

/// 🚪️ Leaves history editing and discards every draft.
pub const HISTORY_EDIT_EXIT_ACTION_ID: &str = "historyEditExit";

/// ⏹️ Cancels the running downstream replay.
pub const HISTORY_EDIT_CANCEL_REPLAY_ACTION_ID: &str = "historyEditCancelReplay";

/// 🔁️ Replays the accepted drafts again after a cancelled or faulted replay.
pub const HISTORY_EDIT_RERUN_ACTION_ID: &str = "historyEditRerun";

/// 🗂️ Every reserved history-edit verb, in lifecycle order. They are host-driven: applied to the session at the head of
/// the dispatch, never queued behind guest work, never recorded as history rows.
pub const HISTORY_EDIT_ACTION_IDS: [&str; 13] = [
    HISTORY_EDIT_BEGIN_ACTION_ID,
    HISTORY_EDIT_INPUT_ACTION_ID,
    HISTORY_EDIT_USE_SELECTION_ACTION_ID,
    HISTORY_EDIT_WITHDRAW_ACTION_ID,
    HISTORY_EDIT_ACCEPT_ACTION_ID,
    HISTORY_EDIT_DISCARD_ACTION_ID,
    HISTORY_EDIT_RESTORE_ACTION_ID,
    HISTORY_EDIT_FINALIZE_ACTION_ID,
    HISTORY_EDIT_COMMIT_ACTION_ID,
    HISTORY_EDIT_BACK_ACTION_ID,
    HISTORY_EDIT_EXIT_ACTION_ID,
    HISTORY_EDIT_CANCEL_REPLAY_ACTION_ID,
    HISTORY_EDIT_RERUN_ACTION_ID,
];

/// 🪪️ The mutation id argument (`<editId>#<opIndex>`) of `historyEditBegin`, of a row's `historyEditWithdraw` and of
/// `historyEditRestore`.
pub const HISTORY_EDIT_ARG_MUTATION_ID: &str = "mutationId";

/// 🧩️ The member store argument (`<slot>/<childId>`) beside every `mutationId`: the composed member store that holds the
/// mutation (design §12); absent for the document's own store.
pub const HISTORY_EDIT_ARG_STORE: &str = "store";

/// 🧭️ The RFC 6901 pointer of one input inside the edited mutation's payload.
pub const HISTORY_EDIT_ARG_PATH: &str = "path";

/// 🎚️ `historyEditInput`'s new input value, or the item it inserts.
pub const HISTORY_EDIT_ARG_VALUE: &str = "value";

/// ✂️ `historyEditInput`'s list edit: absent sets the input at `path`; [`HISTORY_EDIT_INPUT_INSERT`] inserts `value` (absent:
/// the item's default) before the item at `path` (`<array>/<index>`, or `<array>/-` to append); [`HISTORY_EDIT_INPUT_REMOVE`]
/// removes the item at `path`.
pub const HISTORY_EDIT_ARG_EDIT: &str = "edit";

/// ➕️ The `historyEditInput` list edit that inserts an item.
pub const HISTORY_EDIT_INPUT_INSERT: &str = "insert";

/// ➖️ The `historyEditInput` list edit that removes an item.
pub const HISTORY_EDIT_INPUT_REMOVE: &str = "remove";

/// 🧿️ The session generation a verb addresses; absent addresses the live session.
pub const HISTORY_EDIT_ARG_GENERATION: &str = "generation";

/// 🏷️ `historyEditCommit`'s new alternative name.
pub const HISTORY_EDIT_ARG_NAME: &str = "name";

/// ✍️ The `historyEditCommit` choice (the finalize dialog's [`DIALOG_CHOICE_ARG`]) that overwrites every alternative.
pub const HISTORY_EDIT_CHOICE_OVERWRITE: &str = "overwrite";

/// 🗳️ The framework-injected finalize dialog of a history edit.
pub const HISTORY_EDIT_FINALIZE_DIALOG_ID: &str = "finalizeHistoryEdit";

/// ✏️ The thirteen reserved history-edit verbs, joined to every hosting app roster by [`RESERVED`]: never in the palette, no chords
/// (the hosts bind remappable `ui.timeTravel.*` chords), rejected on a viewer, agent-addressable like undo.
pub fn history_edit_action_definitions() -> Vec<ActionDefinition> {
    let generation = || history_edit_hidden_arg(ActionArgDef::with_schema(HISTORY_EDIT_ARG_GENERATION, LocalizedLabel::native("Generation", "Generation"), ArgSchema::number(Some(0.0), None, Some(1.0), true)));
    let path = || ActionArgDef::text(HISTORY_EDIT_ARG_PATH, LocalizedLabel::native("Input", "Eingabe")).describe(LocalizedLabel::native("RFC 6901 pointer of the input inside the mutation payload, such as /dx.", "RFC-6901-Zeiger der Eingabe in den Nutzdaten der Mutation, etwa /dx.")).required();
    let verb = |id: &str, en: &str, de: &str, icon: &str| ActionDefinition { in_palette: false, ..ActionDefinition::resumable_framework(id, LocalizedLabel::native(en, de), ActionKind::History, icon) };
    let mutation = || history_edit_hidden_arg(ActionArgDef::text(HISTORY_EDIT_ARG_MUTATION_ID, LocalizedLabel::native("Mutation", "Mutation")));
    let store = || history_edit_hidden_arg(ActionArgDef::text(HISTORY_EDIT_ARG_STORE, LocalizedLabel::native("Member store", "Mitgliedsspeicher")));
    vec![
        verb(HISTORY_EDIT_BEGIN_ACTION_ID, "Edit Mutation", "Mutation bearbeiten", "pencil")
            .describe(LocalizedLabel::native(
                "Opens history editing on one applied mutation: the artifact shows the state right before it with its inputs editable, and nothing downstream is applied until the draft is accepted.",
                "Öffnet die Verlaufsbearbeitung für eine angewendete Mutation: Das Artefakt zeigt den Zustand direkt davor mit bearbeitbaren Eingaben, und nichts Späteres wird angewendet, bis der Entwurf übernommen ist.",
            ))
            .use_when(["edit an earlier step", "change a past operation", "fix a mutation in the history"])
            .with_args([mutation().required(), store()]),
        verb(HISTORY_EDIT_INPUT_ACTION_ID, "Set Mutation Input", "Mutationseingabe setzen", "sliders-horizontal")
            .describe(LocalizedLabel::native(
                "Sets one input of the mutation being edited, or inserts or removes one item of a list input; the result is validated against the mutation's input schema and previewed immediately.",
                "Setzt eine Eingabe der bearbeiteten Mutation oder fügt ein Element einer Listeneingabe ein oder entfernt es; das Ergebnis wird gegen das Eingabeschema der Mutation geprüft und sofort in der Vorschau gezeigt.",
            ))
            .use_when(["change this value in the edited step", "set the input of the mutation", "add or remove an item of a list input"])
            .with_args([
                path(),
                ActionArgDef::any(HISTORY_EDIT_ARG_VALUE, LocalizedLabel::native("Value", "Wert")),
                ActionArgDef::select(
                    HISTORY_EDIT_ARG_EDIT,
                    LocalizedLabel::native("List edit", "Listenbearbeitung"),
                    vec![ActionArgOption::new(HISTORY_EDIT_INPUT_INSERT, LocalizedLabel::native("Insert item", "Element einfügen")), ActionArgOption::new(HISTORY_EDIT_INPUT_REMOVE, LocalizedLabel::native("Remove item", "Element entfernen"))],
                ),
                generation(),
            ]),
        verb(HISTORY_EDIT_USE_SELECTION_ACTION_ID, "Use Selection", "Auswahl verwenden", "mouse-pointer")
            .describe(LocalizedLabel::native(
                "Sets a reference input of the mutation being edited to the entities currently selected in its domain.",
                "Setzt eine Referenzeingabe der bearbeiteten Mutation auf die aktuell ausgewählten Elemente ihres Bereichs.",
            ))
            .use_when(["use the current selection as targets"])
            .with_args([path(), generation()]),
        verb(HISTORY_EDIT_WITHDRAW_ACTION_ID, "Withdraw Mutation", "Mutation zurückziehen", "eye-off")
            .describe(LocalizedLabel::native(
                "Drafts a mutation as withdrawn, so it no longer changes the artifact: the one being edited, or the one named by mutationId, which opens history editing on it; use it for a step whose inputs cannot fix its error or cannot be edited at all.",
                "Entwirft eine Mutation als zurückgezogen, sodass sie das Artefakt nicht mehr verändert: die bearbeitete oder die mit mutationId benannte, für die sich die Verlaufsbearbeitung öffnet; für einen Schritt, dessen Fehler sich über die Eingaben nicht beheben lässt oder dessen Eingaben gar nicht bearbeitbar sind.",
            ))
            .use_when(["drop this step", "skip this operation", "withdraw a mutation from the history"])
            .with_args([mutation(), store(), generation()]),
        verb(HISTORY_EDIT_ACCEPT_ACTION_ID, "Accept Draft", "Entwurf übernehmen", "check")
            .describe(LocalizedLabel::native(
                "Accepts the draft of the mutation being edited and replays every later mutation, reporting success, warnings and errors per mutation.",
                "Übernimmt den Entwurf der bearbeiteten Mutation und wendet alle späteren Mutationen neu an; Erfolg, Warnungen und Fehler werden je Mutation gemeldet.",
            ))
            .use_when(["apply the edit", "accept the change to the history"])
            .with_args([generation()]),
        verb(HISTORY_EDIT_DISCARD_ACTION_ID, "Discard Draft", "Entwurf verwerfen", "x")
            .describe(LocalizedLabel::native("Drops the draft of the mutation being edited; accepted drafts stay.", "Verwirft den Entwurf der bearbeiteten Mutation; übernommene Entwürfe bleiben erhalten."))
            .use_when(["discard this draft"])
            .with_args([generation()]),
        verb(HISTORY_EDIT_RESTORE_ACTION_ID, "Restore Mutation", "Mutation wiederherstellen", "undo-2")
            .describe(LocalizedLabel::native(
                "Takes the accepted draft of one mutation back while reviewing, so it applies with its recorded input again, and replays every later mutation; taking the only accepted draft back leaves history editing with nothing changed.",
                "Nimmt beim Prüfen den übernommenen Entwurf einer Mutation zurück, sodass sie wieder mit ihrer aufgezeichneten Eingabe angewendet wird, und wendet alle späteren Mutationen neu an; wird der einzige übernommene Entwurf zurückgenommen, endet die Verlaufsbearbeitung ohne Änderung.",
            ))
            .use_when(["take this draft back", "bring the withdrawn step back", "undo one change of the history edit"])
            .with_args([mutation().required(), store(), generation()]),
        verb(HISTORY_EDIT_FINALIZE_ACTION_ID, "Finalize History Edit", "Verlaufsbearbeitung abschließen", "list-checks")
            .describe(LocalizedLabel::native(
                "Asks how to keep the edited history once no replayed mutation has an error: as a new alternative or by overwriting the existing history.",
                "Fragt, wie der bearbeitete Verlauf behalten wird, sobald keine neu angewendete Mutation einen Fehler hat: als neue Alternative oder durch Überschreiben des bestehenden Verlaufs.",
            ))
            .use_when(["finish editing the history", "keep the edited history"])
            .with_args([generation()]),
        verb(HISTORY_EDIT_COMMIT_ACTION_ID, "Commit History Edit", "Verlaufsbearbeitung festschreiben", "git-commit")
            .destructive()
            .describe(LocalizedLabel::native(
                "Commits the accepted drafts: as a new alternative named by name, keeping the original history, or with choice overwrite into every alternative that contains the edited mutations.",
                "Schreibt die übernommenen Entwürfe fest: als neue Alternative mit dem Namen name, wobei der ursprüngliche Verlauf bleibt, oder mit choice overwrite in jede Alternative, die die bearbeiteten Mutationen enthält.",
            ))
            .use_when(["save the edited history as a new alternative", "overwrite the history with the edit"])
            .with_args([
                ActionArgDef::select(DIALOG_CHOICE_ARG, LocalizedLabel::native("Choice", "Auswahl"), vec![ActionArgOption::new(HISTORY_EDIT_CHOICE_OVERWRITE, LocalizedLabel::native("Overwrite", "Überschreiben"))]),
                ActionArgDef::text(HISTORY_EDIT_ARG_NAME, LocalizedLabel::native("Alternative name", "Name der Alternative")),
                generation(),
            ]),
        verb(HISTORY_EDIT_BACK_ACTION_ID, "Back to Review", "Zurück zur Prüfung", "arrow-left")
            .describe(LocalizedLabel::native("Closes the finalize prompt and returns to reviewing the edited history.", "Schließt die Abschlussabfrage und kehrt zur Prüfung des bearbeiteten Verlaufs zurück."))
            .use_when(["go back to reviewing"])
            .with_args([generation()]),
        verb(HISTORY_EDIT_EXIT_ACTION_ID, "Exit History Editing", "Verlaufsbearbeitung beenden", "rotate-ccw")
            .describe(LocalizedLabel::native("Leaves history editing and discards every draft; the artifact is unchanged.", "Beendet die Verlaufsbearbeitung und verwirft alle Entwürfe; das Artefakt bleibt unverändert."))
            .use_when(["stop editing the history", "cancel the history edit"]),
        verb(HISTORY_EDIT_CANCEL_REPLAY_ACTION_ID, "Cancel Replay", "Neuanwendung abbrechen", "square")
            .describe(LocalizedLabel::native(
                "Cancels the running replay of later mutations; the drafts stay and the replay can be started again by accepting.",
                "Bricht die laufende Neuanwendung späterer Mutationen ab; die Entwürfe bleiben, und die Neuanwendung kann durch Übernehmen erneut gestartet werden.",
            ))
            .use_when(["stop the replay"])
            .with_args([generation()]),
        verb(HISTORY_EDIT_RERUN_ACTION_ID, "Replay Again", "Erneut anwenden", "skip-forward")
            .describe(LocalizedLabel::native(
                "Replays every later mutation again with the accepted drafts after a cancelled or failed replay, so the history can be reviewed and finalized.",
                "Wendet nach einer abgebrochenen oder fehlgeschlagenen Neuanwendung alle späteren Mutationen mit den übernommenen Entwürfen erneut an, damit der Verlauf geprüft und abgeschlossen werden kann.",
            ))
            .use_when(["replay the edited history again", "retry the replay"])
            .with_args([generation()]),
    ]
}

/// 🫥️ A history-edit argument the chrome fills from the session, never shown in a staged form.
fn history_edit_hidden_arg(arg: ActionArgDef) -> ActionArgDef {
    ActionArgDef { presentation: Some(ArgPresentation::Hidden), ..arg }
}

/// 🗳️ The finalize prompt of a history edit, joined to every hosting app that may edit by [`RESERVED`]: the submit keeps
/// the edit as a new alternative named by the staged `name` (the opener seeds a localized default), the destructive
/// choice `overwrite` replaces the inputs in every alternative, and dismissing it goes back to reviewing.
pub fn history_edit_finalize_dialog() -> DialogDefinition {
    DialogDefinition::new(HISTORY_EDIT_FINALIZE_DIALOG_ID, LocalizedLabel::native("Finish editing history", "Verlaufsbearbeitung abschließen"), ActionRef::new(HISTORY_EDIT_COMMIT_ACTION_ID))
        .body(LocalizedLabel::native("Keep the edit as a new alternative or overwrite the existing history.", "Die Bearbeitung als neue Alternative behalten oder den bestehenden Verlauf überschreiben."))
        .args(vec![ActionArgDef::text(HISTORY_EDIT_ARG_NAME, LocalizedLabel::native("Alternative name", "Name der Alternative")).required()])
        .submit_label(LocalizedLabel::native("New alternative", "Neue Alternative"))
        .cancel_label(LocalizedLabel::native("Back", "Zurück"))
        .on_cancel(ActionRef::new(HISTORY_EDIT_BACK_ACTION_ID))
        .choice(
            DialogChoice::new(HISTORY_EDIT_CHOICE_OVERWRITE, LocalizedLabel::native("Overwrite", "Überschreiben"), ActionRef::new(HISTORY_EDIT_COMMIT_ACTION_ID))
                .description(LocalizedLabel::native("Replaces the edited mutations in every alternative that contains them.", "Ersetzt die bearbeiteten Mutationen in jeder Alternative, die sie enthält."))
                .tone(semio_framework_ui_contract::Tone::Danger)
                .destructive(),
        )
}

/// 🧩️ The history-edit contribution: the thirteen verbs join the app roster, the finalize dialog joins every app that may
/// edit.
pub const RESERVED: ReservedContribution = ReservedContribution::new("historyEdit").with_app_actions(history_edit_action_definitions).with_editor_dialogs(history_edit_finalize_dialog);

//#endregion 🔖️HistoryEdit

#[cfg(test)]
use semio_framework_ui_locale::Locale;
#[cfg(test)]
use semio_framework_ui_locale::Terminology;
#[cfg(test)]
use semio_framework_value::FromValue;
#[cfg(test)]
use semio_framework_value::Number;
#[cfg(test)]
use semio_framework_value::ToValue;
#[cfg(test)]
use std::any::Any;
#[cfg(test)]
use semio_framework_manifest_actions::{CapabilityAudience, InteractiveJobClassification, resolve_audience};
#[cfg(test)]
use semio_framework_manifest_app_surface::{reserved_app_actions, reserved_editor_dialogs};

//#region 🔖️HistoryEdit
#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
//#endregion 🔖️HistoryEdit
