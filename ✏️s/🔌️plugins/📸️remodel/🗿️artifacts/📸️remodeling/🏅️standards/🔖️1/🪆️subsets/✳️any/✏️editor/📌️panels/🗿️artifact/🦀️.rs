//! 📄️ Remodeling play app panel — the framework Document tab: reconstruction job status/progress plus the
//! live viewport session state.

use crate::artifacts::remodeling::schema::stage_display;
use crate::artifacts::remodeling::{ReconstructionStage, RemodelingSnapshot};
use crate::editor::remodeling::terminology::RemodelingLabels;
use semio_framework_plugin::{ui_stack_vertical, ui_text, Label, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, UiNode, FRAMEWORK_PANEL_TAB_ARTIFACT_ID, FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL};

//#region 🔖️Constants
pub const REMODELING_PLAY_BODY_PIPELINE: &str = "remodeling.play.pipeline";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub async fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_ARTIFACT_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL, "Dokument"),
        group: PanelGroup::Workbench,
        body_key: Some(REMODELING_PLAY_BODY_PIPELINE.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 🚦️ `running` is derived from the persisted job stage (not a live engine handle): a synchronous run
/// never leaves the document in a non-terminal stage, so this is effectively always "Idle" once a run
/// finishes — the documented, accepted trade-off of the pure-trait conversion.
pub async fn render(scene: &RemodelingSnapshot, active_utility: &str, labels: &RemodelingLabels) -> UiNode {
    let job = &scene.job;
    let job_label = format!("{}: {} ({:.0}%){}", labels.reconstruction.as_str(), stage_display(job.stage), job.progress_0_1 * 100.0, job.error.as_ref().map(|error| format!(" - {}: {error}", labels.error.as_str())).unwrap_or_default());
    let running = !matches!(job.stage, ReconstructionStage::Idle | ReconstructionStage::Done | ReconstructionStage::Failed);
    let running_label = format!("{}: {}", labels.status.as_str(), if running { labels.running.as_str() } else { labels.idle.as_str() });
    // 🕹️ Selection now lives in the framework-owned "assets" interaction domain (ticket
    // 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM); `ArtifactEditor::render` carries no
    // `InteractionView`, so this panel can no longer embed a live selection count in its text.
    let utility_label = format!("{}: {}", labels.utility.as_str(), active_utility);
    ui_stack_vertical(vec![ui_text(Label::data(job_label)), ui_text(Label::data(running_label)), ui_text(Label::data(utility_label))])
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::remodeling::testkit::{app, render as render_body};

    #[semio_framework_async_macros::async_test]
    async fn definition_binds_the_framework_document_tab_to_this_body_key() {
        let definition = definition();
        assert_eq!(definition.id(), FRAMEWORK_PANEL_TAB_ARTIFACT_ID);
        assert_eq!(definition.body_key.as_deref(), Some(REMODELING_PLAY_BODY_PIPELINE));
    }

    #[semio_framework_async_macros::async_test]
    async fn a_fresh_document_reports_an_idle_job() {
        let mut app = app();
        let body = render_body(&mut app, REMODELING_PLAY_BODY_PIPELINE);
        assert!(body.contains("Idle"), "a fresh document's job is idle: {body}");
    }
}
//#endregion 🧪️Tests
