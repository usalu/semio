//! ⏯️ The framework surface of the tool run feature: the run vocabulary a tool or utility declares and the reserved tool run actions.

use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_manifest_actions::{ActionDefinition, ActionKind};
use semio_framework_manifest_app_surface::{AppDefinition, ReservedContribution};
use semio_framework_manifest_arg_schema::{ActionArgDef, ArgPresentation, ArgSchema};

//#region 🔖️ToolRun
pub use semio_framework_tool_run::{
    JobKindId, ToolRunAction, ToolRunCounterDefinition, ToolRunDefinition, ToolRunDefinitionError, ToolRunReasonDefinition, ToolRunRebasePolicy, ToolRunReconfigurePolicy, ToolRunSettingsReads, ToolRunStageDefinition, ToolRunTraceCursor, ToolRunTraceKind, ToolRunVerdict, TOOL_RUN_ABORT_ACTION_ID,
    tool_run_panel_new_runs, TOOL_RUN_ACTION_IDS, TOOL_RUN_DISMISS_ACTION_ID, TOOL_RUN_DISMISS_CHORD, TOOL_RUN_FINALIZE_ACTION_ID, TOOL_RUN_PAUSE_ACTION_ID, TOOL_RUN_RESUME_ACTION_ID, TOOL_RUN_START_ACTION_ID, TOOL_RUN_STEP_ACTION_ID,
};

/// ⏯️ Whether any tool or utility of `app` declares a `ToolRunDefinition`.
pub fn app_declares_tool_run(app: &AppDefinition) -> bool {
    app.tools.iter().any(|tool| tool.run.is_some()) || app.utilities.iter().any(|utility| utility.run.is_some())
}

/// 🕹️ The seven reserved tool run actions (§2.5 of the tool run contract), joined to every window kind of any
/// `AppDefinition` whose tools or utilities declare `run` by [`RESERVED`] — mirrors `interaction_action_definitions`. Ids,
/// chords, argument names and EN/DE labels come from `semio_framework_tool_run::ToolRunAction`. They are
/// `History` actions like `undo`: host-routed, never queued behind the run, never in the palette or the
/// Actions panel. `toolRunDismiss` carries no app-wide `keys` because `escape` dismisses only with the run
/// panel focused ([`TOOL_RUN_DISMISS_CHORD`]); pause and resume share one toggle chord.
pub fn tool_run_action_definitions(app: &AppDefinition) -> Vec<ActionDefinition> {
    if !app_declares_tool_run(app) {
        return Vec::new();
    }
    ToolRunAction::ALL
        .into_iter()
        .map(|action| {
            let icon = match action {
                ToolRunAction::Start | ToolRunAction::Resume => "play",
                ToolRunAction::Pause => "pause",
                ToolRunAction::Step => "skip-forward",
                ToolRunAction::Abort => "square",
                ToolRunAction::Finalize => "check",
                ToolRunAction::Dismiss => "x",
            };
            let keys = (action != ToolRunAction::Dismiss).then(|| action.chord().to_string());
            ActionDefinition { keys, in_palette: false, ..ActionDefinition::resumable_framework(action.id(), action.label().localized(), ActionKind::History, icon) }
                .with_args(action.args().iter().map(|arg| tool_run_action_arg(arg.name, arg.required)))
                .describe(tool_run_action_description(action))
        })
        .collect()
}

/// 💬️ What each reserved tool run action does to the run and to the document — a run holds its
/// result as a provisional preview, and only `toolRunFinalize` publishes it as one undoable edit.
fn tool_run_action_description(action: ToolRunAction) -> LocalizedLabel {
    match action {
        ToolRunAction::Start => LocalizedLabel::native(
            "Starts a run of the given tool (such as a solver) in a window; the run shows its result as a provisional preview and changes nothing until it is finalized.",
            "Startet einen Lauf des angegebenen Werkzeugs (etwa eines Lösers) in einem Fenster; der Lauf zeigt sein Ergebnis als vorläufige Vorschau und ändert nichts, bis er abgeschlossen wird.",
        ),
        ToolRunAction::Pause => LocalizedLabel::native(
            "Pauses a running tool run at its current step so its progress can be inspected, stepped or resumed.",
            "Hält einen laufenden Werkzeuglauf beim aktuellen Schritt an, damit sein Fortschritt geprüft, schrittweise fortgeführt oder fortgesetzt werden kann.",
        ),
        ToolRunAction::Resume => LocalizedLabel::native("Continues a paused tool run from the step where it stopped.", "Setzt einen pausierten Werkzeuglauf an dem Schritt fort, an dem er angehalten wurde."),
        ToolRunAction::Step => LocalizedLabel::native("Advances a paused tool run by exactly one step and pauses it again.", "Führt einen pausierten Werkzeuglauf genau einen Schritt weiter und hält ihn wieder an."),
        ToolRunAction::Abort => LocalizedLabel::native(
            "Stops a tool run and discards its provisional result; the document stays exactly as it was before the run started.",
            "Bricht einen Werkzeuglauf ab und verwirft sein vorläufiges Ergebnis; das Dokument bleibt genau so, wie es vor dem Start war.",
        ),
        ToolRunAction::Finalize => LocalizedLabel::native(
            "Commits the provisional result of a running, paused or complete tool run into the document as one undoable edit.",
            "Übernimmt das vorläufige Ergebnis eines laufenden, pausierten oder fertigen Werkzeuglaufs als eine rückgängig machbare Änderung in das Dokument.",
        ),
        ToolRunAction::Dismiss => LocalizedLabel::native(
            "Closes the panel of a finalized, aborted or failed tool run; the document is not touched.",
            "Schließt das Panel eines abgeschlossenen, abgebrochenen oder fehlgeschlagenen Werkzeuglaufs; das Dokument bleibt unberührt.",
        ),
    }
}

fn tool_run_action_arg(name: &'static str, required: bool) -> ActionArgDef {
    let (label, schema) = match name {
        semio_framework_tool_run::TOOL_RUN_ARG_TOOL_ID => (LocalizedLabel::native("Tool", "Werkzeug"), ActionArgDef::plain_string(None)),
        semio_framework_tool_run::TOOL_RUN_ARG_WINDOW_ID => (LocalizedLabel::native("Window", "Fenster"), ActionArgDef::plain_string(None)),
        semio_framework_tool_run::TOOL_RUN_ARG_RUN_ID => (LocalizedLabel::native("Run", "Lauf"), ArgSchema::number(Some(0.0), None, Some(1.0), true)),
        _ => (LocalizedLabel::native("Generation", "Generation"), ArgSchema::number(Some(0.0), None, Some(1.0), true)),
    };
    ActionArgDef { presentation: Some(ArgPresentation::Hidden), required, ..ActionArgDef::with_schema(name, label, schema) }
}

/// 🧩️ The tool run contribution: the seven reserved actions join every window kind of an app that declares a run.
pub const RESERVED: ReservedContribution = ReservedContribution::new("toolRun").with_window_actions(tool_run_action_definitions);

//#endregion 🔖️ToolRun

#[cfg(test)]
use replication::HierarchyProvider;
#[cfg(test)]
use replication::HoverSpec;
#[cfg(test)]
use replication::SelectionSpec;
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
use serde::Deserialize;
#[cfg(test)]
use semio_framework_interaction::InteractionDefinition;
#[cfg(test)]
use semio_framework_manifest_actions::{CLEAR_SELECTION_ACTION_ID, InteractiveJobClassification, SELECT_ALL_ACTION_ID, ToolDefinition, UtilityDefinition, clipboard_action_definitions, history_action_definitions};
#[cfg(test)]
use semio_framework_manifest_app_surface::{interaction_action_definitions, reserved_window_actions};

//#region 🔖️ToolRun
#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
//#endregion 🔖️ToolRun
