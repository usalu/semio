//! 📄️ Remodel play app panel — the framework Document tab: reconstruction job status/progress plus the
//! live viewport session state.

use crate::apps::remodel::config::RemodelConfig;
use crate::apps::remodel::terminology::RemodelLabels;
use crate::artifacts::remodel::engine::stage_display;
use crate::artifacts::remodel::{ReconstructionStage, RemodelProjection};
use semio_framework_plugin::{ui_stack_vertical, ui_text, Label, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, UiNode, FRAMEWORK_PANEL_TAB_DOCUMENT_ID, FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL};

//#region 🔖️Constants
pub const REMODEL_PLAY_BODY_PIPELINE: &str = "remodel.play.pipeline";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_DOCUMENT_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, "Dokument"),
        group: PanelGroup::Workbench,
        body_key: Some(REMODEL_PLAY_BODY_PIPELINE.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 🚦️ `running` is derived from the persisted job stage (not a live engine handle): a synchronous run
/// never leaves the document in a non-terminal stage, so this is effectively always "Idle" once a run
/// finishes — the documented, accepted trade-off of the pure-trait conversion.
pub fn render(scene: &RemodelProjection, config: &RemodelConfig, active_utility: &str, labels: &RemodelLabels) -> UiNode {
    let job = &scene.job;
    let job_label = format!("{}: {} ({:.0}%){}", labels.reconstruction.as_str(), stage_display(job.stage), job.progress_0_1 * 100.0, job.error.as_ref().map(|error| format!(" - {}: {error}", labels.error.as_str())).unwrap_or_default());
    let running = !matches!(job.stage, ReconstructionStage::Idle | ReconstructionStage::Done | ReconstructionStage::Failed);
    let running_label = format!("{}: {}", labels.status.as_str(), if running { labels.running.as_str() } else { labels.idle.as_str() });
    let utility_label = format!("{}: {} - {}: {} ({})", labels.utility.as_str(), active_utility, labels.selection.as_str(), config.selection.mode, config.selection.ids.len());
    ui_stack_vertical(vec![ui_text(Label::data(job_label)), ui_text(Label::data(running_label)), ui_text(Label::data(utility_label))])
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::remodel::testkit::{app, render as render_body};

    #[test]
    fn definition_binds_the_framework_document_tab_to_this_body_key() {
        let definition = definition();
        assert_eq!(definition.id(), FRAMEWORK_PANEL_TAB_DOCUMENT_ID);
        assert_eq!(definition.body_key.as_deref(), Some(REMODEL_PLAY_BODY_PIPELINE));
    }

    #[test]
    fn a_fresh_document_reports_an_idle_job() {
        let mut app = app();
        let body = render_body(&mut app, REMODEL_PLAY_BODY_PIPELINE);
        assert!(body.contains("Idle"), "a fresh document's job is idle: {body}");
    }
}
//#endregion 🧪️Tests
