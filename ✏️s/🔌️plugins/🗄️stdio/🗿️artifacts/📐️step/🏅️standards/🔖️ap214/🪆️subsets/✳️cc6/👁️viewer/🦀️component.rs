//! 👁️ Step CC6 viewer — the read-only counterpart of `✏️editor` for this subset (ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.2). `StepCc6Viewer`
//! implements `ArtifactViewer`, never `ArtifactEditor`/`ArtifactApp` — `ViewerApp<StepCc6Viewer>`
//! (framework SDK) is the sole runtime adapter, so this file can never structurally emit an artifact
//! or draft mutation. MUST NOT reference the sibling editor module.

use crate::artifacts::step::standards::v_ap214::subsets::cc6::schema::snapshot::StepSnapshot;
use crate::artifacts::step::standards::v_ap214::subsets::cc6::schema::mutations::StepMutation;
use crate::viewer::step_cc6::modes::view;
use crate::viewer::step_cc6::modes::view::windows::main;
use semio_framework_plugin::{
    ArtifactView, ArtifactViewer, ConfigView, Dialect, Fault, Label, NoConfig, NoConfigMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, StandardId, SubsetId, UiNode, ViewEmit, Viewer,
};
use semio_framework_plugin::app::InteractionView;
use store::EngineHandles;

//#region 🔖️Dialect
/// 🪪️ Verified against this artifact's own `📸️snapshot/🦀️component.rs`
/// `impl ArtifactAnalysis for …AnalyzerAnalysis { const DIALECT }` row (read, not guessed) — see
/// the packet report for the exact grep evidence per subset.
pub const STEP_CC6_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.step", standard: StandardId("ap214"), subset: SubsetId("cc6") };
pub const STEP_CC6_DOCUMENT_SCHEMA: &str = "stdio.step";
//#endregion 🔖️Dialect

//#region 🔖️Command
/// 👁️ The viewer declares no actions, so its typed command channel has exactly one inert variant.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum StepCc6ViewCommand {
    #[default]
    Noop,
}

impl protocol::OpBinary for StepCc6ViewCommand {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(Vec::new())
    }
    fn decode_op(_bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(StepCc6ViewCommand::Noop)
    }
}
//#endregion 🔖️Command

//#region 🔖️Viewer
#[derive(Default, Clone, Copy)]
pub struct StepCc6Viewer;

impl ArtifactViewer for StepCc6Viewer {
    type Snapshot = StepSnapshot;
    type Mutation = StepMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = StepCc6ViewCommand;

    const DIALECT: Dialect = STEP_CC6_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = STEP_CC6_DOCUMENT_SCHEMA;

    async fn initial_snapshot() -> StepSnapshot {
        StepSnapshot::default()
    }

    /// 👁️ Structurally read-only: the sole `Noop` variant never carries a config change.
    async fn handle(_command: &Self::Command, _doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, _interaction: &InteractionView<'_>, _engines: &EngineHandles) -> Result<ViewEmit<Self::ConfigMutation>, Fault> {
        Ok(ViewEmit::default())
    }

    async fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>) -> UiNode {
        match body_key {
            main::BODY_KEY => main::render(doc.snapshot),
            _ => semio_framework_plugin::ui_text(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️Viewer

//#region 🔖️Manifest
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn create_step_cc6_viewer() -> semio_framework_plugin::AppDefinition {
    Viewer::builder(STEP_CC6_DIALECT)
        .document(["stdio", "step"])
        .icon_id("box")
        .mode_def(view::definition())
        .default_mode_id(view::STEP_CC6_VIEW_MODE_ID)
        .window_kind_def(main::definition())
        .default_layout(view::layout())
        .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn create_viewer_builds_a_definition_for_the_viewer_role() {
        let def = create_step_cc6_viewer();
        assert_eq!(def.role, semio_framework_plugin::AppRole::Viewer);
        assert_eq!(def.dialect, STEP_CC6_DIALECT.into());
    }

    #[semio_framework_async_macros::async_test]
    async fn viewer_dialect_matches_the_artifact_coordinate() {
        assert_eq!(<StepCc6Viewer as ArtifactViewer>::DIALECT, STEP_CC6_DIALECT);
    }

    #[semio_framework_async_macros::async_test]
    async fn viewer_never_mutates_the_document_or_draft_store() {
        semio_framework_plugin::testkit::assert_viewer_never_mutates::<StepCc6Viewer>();
    }
}
//#endregion 🧪️Tests
