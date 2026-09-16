//! 📄️ Remodeling play app panel — the framework Document tab: the reconstruction run as the framework
//! ledger reports it (`ArtifactView::tool_run`) beside the framework ToolRun panel itself. Progress, steps,
//! the trace list and the start/pause/step/abort/finalize buttons are the framework panel's
//! (`📋️tool-run-contract.md` §2.5, §4.1); this panel never keeps run state of its own.

use crate::editor::remodeling::modes::model::tools::reconstruction;
use crate::RemodelingSnapshot;
use semio_framework_plugin::{tree_item, BuiltNode, Locale, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, ToolRunView, TreeWindows, UiAssemblyResult, FRAMEWORK_PANEL_TAB_ARTIFACT_ID, FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL, FRAMEWORK_TOOL_RUN_BODY_KEY};
use semio_framework_tool_run::{ToolRunAction, ToolRunState};

//#region 🔖️Constants
pub const REMODELING_PLAY_BODY_PIPELINE: &str = "remodeling.play.pipeline";
const REMODELING_PANEL_TAB_RECONSTRUCTION_ID: &str = "framework.panel.document.reconstruction";
const REMODELING_PANEL_TAB_RUN_ID: &str = "framework.panel.document.run";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🗂️ The Document tab: the reconstruction readout and the framework run panel as nested tabs.
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_ARTIFACT_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL, "Artefakt"),
        group: PanelGroup::Workbench,
        body_key: None,
        children: vec![
            PanelTabDefinition {
                kind: PanelTabKind::App(REMODELING_PANEL_TAB_RECONSTRUCTION_ID.into()),
                label: LocalizedLabel::native("Reconstruction", "Rekonstruktion"),
                group: PanelGroup::Workbench,
                body_key: Some(REMODELING_PLAY_BODY_PIPELINE.into()),
                children: Vec::new(),
            },
            PanelTabDefinition {
                kind: PanelTabKind::App(REMODELING_PANEL_TAB_RUN_ID.into()),
                label: LocalizedLabel::native("Run", "Lauf"),
                group: PanelGroup::Workbench,
                body_key: Some(FRAMEWORK_TOOL_RUN_BODY_KEY.into()),
                children: Vec::new(),
            },
        ],
    }
}
//#endregion 🔖️Definition

//#region 🗣️Language
fn say(locale: Locale, en: &'static str, de: &'static str) -> &'static str {
    if locale == Locale::De {
        de
    } else {
        en
    }
}

/// 🏁️ What the run's state means for the document, beside the framework state label.
fn result_text(state: ToolRunState, locale: Locale) -> &'static str {
    match state {
        ToolRunState::Starting | ToolRunState::Running | ToolRunState::Paused => say(locale, "Cameras and points appear in the Model window as they are reconstructed; nothing is written yet", "Kameras und Punkte erscheinen im Modellfenster, sobald sie rekonstruiert sind; noch wird nichts geschrieben"),
        ToolRunState::Complete => say(locale, "The reconstruction is ready to finalize", "Die Rekonstruktion ist bereit zum Abschließen"),
        ToolRunState::Finalizing | ToolRunState::Finalized => say(locale, "The reconstruction is committed as one undoable change", "Die Rekonstruktion ist als eine rückgängig machbare Änderung übernommen"),
        ToolRunState::Aborting | ToolRunState::Aborted | ToolRunState::Faulted => say(locale, "No reconstruction was committed", "Keine Rekonstruktion wurde übernommen"),
    }
}
//#endregion 🗣️Language

//#region 🔖️Render
/// 🚦️ The run state, what it means for the document, the stored result and the driving chords.
pub fn render(scene: &RemodelingSnapshot, run: Option<&ToolRunView>, locale: Locale, windows: &TreeWindows<'_>) -> UiAssemblyResult<BuiltNode> {
    let run = run.filter(|run| run.tool_id == reconstruction::TOOL_ID);
    let state_label = run.map_or_else(|| say(locale, "No reconstruction run", "Kein Rekonstruktionslauf").to_string(), |run| format!("{} · {} {} · {} {}", run.state.label().text(locale), say(locale, "run", "Lauf"), run.identity.id.run, say(locale, "generation", "Generation"), run.identity.generation));
    let result_label = run.map_or_else(
        || format!("{}: {}", say(locale, "Stored cameras", "Gespeicherte Kameras"), scene.results.trajectory.as_ref().map_or(0, |trajectory| trajectory.poses.len())),
        |run| result_text(run.state, locale).to_string(),
    );
    let mut rows: Vec<(String, String)> = vec![("remodeling-pipeline.run".to_string(), state_label), ("remodeling-pipeline.result".to_string(), result_label)];
    for action in [ToolRunAction::Start, ToolRunAction::Pause, ToolRunAction::Step, ToolRunAction::Abort, ToolRunAction::Finalize] {
        rows.push((format!("remodeling-pipeline.keys.{}", action.id()), format!("{} — {}", action.chord(), action.label().text(locale))));
    }
    PanelTreeBuilder::new("remodeling-pipeline")?
        .window_section(windows, "remodeling-pipeline.reconstruction", Some(crate::editor::remodeling::ui_label(say(locale, "Reconstruction", "Rekonstruktion"))?), true, &rows, |(id, text)| tree_item(id.as_str(), text.as_str()))?
        .build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
