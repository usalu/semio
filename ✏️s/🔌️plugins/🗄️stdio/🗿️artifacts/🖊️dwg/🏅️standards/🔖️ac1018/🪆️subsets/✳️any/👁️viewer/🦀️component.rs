//! 👁️ DWG AC1018 viewer — the read-only counterpart of `✏️editor` for this subset (ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.2). `DwgAc1018Viewer`
//! implements `ArtifactViewer`, never `ArtifactEditor`/`ArtifactApp` — `ViewerApp<DwgAc1018Viewer>`
//! (framework SDK) is the sole runtime adapter, so this file can never structurally emit an artifact
//! or draft mutation. MUST NOT reference the sibling editor module.

use crate::artifacts::dwg::standards::v_ac1018::subsets::any::schema::snapshot::DwgSnapshot;
use crate::artifacts::dwg::standards::v_ac1018::subsets::any::schema::mutations::DwgMutation;
use crate::viewer::dwg_ac1018::modes::view;
use crate::viewer::dwg_ac1018::modes::view::windows::main;
use semio_framework_plugin::{
    ArtifactView, ArtifactViewer, ConfigView, Dialect, Fault, Label, NoConfig, NoConfigMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, StandardId, SubsetId, UiNode, ViewEmit, Viewer,
};
use semio_framework_plugin::app::InteractionView;
use store::EngineHandles;

//#region 🔖️Dialect
/// 🪪️ Verified against this artifact's own `📸️snapshot/🦀️component.rs`
/// `impl ArtifactAnalysis for …AnalyzerAnalysis { const DIALECT }` row (read, not guessed) — see
/// the packet report for the exact grep evidence per subset.
pub const DWG_AC1018_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.dwg", standard: StandardId("ac1018"), subset: SubsetId::ANY };
pub const DWG_AC1018_DOCUMENT_SCHEMA: &str = "stdio.dwg";
//#endregion 🔖️Dialect

//#region 🔖️Command
/// 👁️ The viewer declares no actions, so its typed command channel has exactly one inert variant.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DwgAc1018ViewCommand {
    #[default]
    Noop,
}

impl protocol::OpBinary for DwgAc1018ViewCommand {
    async fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(Vec::new())
    }
    async fn decode_op(_bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(DwgAc1018ViewCommand::Noop)
    }
}
//#endregion 🔖️Command

//#region 🔖️Viewer
#[derive(Default, Clone, Copy)]
pub struct DwgAc1018Viewer;

impl ArtifactViewer for DwgAc1018Viewer {
    type Snapshot = DwgSnapshot;
    type Mutation = DwgMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = DwgAc1018ViewCommand;

    const DIALECT: Dialect = DWG_AC1018_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = DWG_AC1018_DOCUMENT_SCHEMA;

    async fn initial_snapshot() -> DwgSnapshot {
        DwgSnapshot::default()
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
pub async fn create_dwg_ac1018_viewer() -> semio_framework_plugin::AppDefinition {
    Viewer::builder(DWG_AC1018_DIALECT)
        .document(["stdio", "dwgac1018"])
        .icon_id("box")
        .mode_def(view::definition())
        .default_mode_id(view::DWG_AC1018_VIEW_MODE_ID)
        .window_kind_def(main::definition())
        .default_layout(view::layout())
        .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    async fn create_viewer_builds_a_definition_for_the_viewer_role() {
        let def = create_dwg_ac1018_viewer();
        assert_eq!(def.role, semio_framework_plugin::AppRole::Viewer);
        assert_eq!(def.dialect, DWG_AC1018_DIALECT.into());
    }

    #[test]
    async fn viewer_dialect_matches_the_artifact_coordinate() {
        assert_eq!(<DwgAc1018Viewer as ArtifactViewer>::DIALECT, DWG_AC1018_DIALECT);
    }

    #[test]
    async fn viewer_never_mutates_the_document_or_draft_store() {
        semio_framework_plugin::testkit::assert_viewer_never_mutates::<DwgAc1018Viewer>();
    }
}
//#endregion 🧪️Tests
