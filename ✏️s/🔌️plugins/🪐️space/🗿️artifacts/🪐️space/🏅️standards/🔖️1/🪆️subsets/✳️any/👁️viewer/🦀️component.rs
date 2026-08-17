//! 👁️ S Space index viewer — the read-only counterpart of `✏️editor` for this subset (ticket
//! 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS §C4, contract
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET §2.2). `SpaceIndexViewer` implements
//! `ArtifactViewer`, never `ArtifactEditor`/`ArtifactApp` — `ViewerApp<SpaceIndexViewer>` (framework
//! SDK) is the sole runtime adapter, so this file can never structurally emit an artifact or draft
//! mutation. MUST NOT import anything from the sibling `✏️editor` (`policyViewerPurityBreaches`).

use crate::artifacts::space::standards::v1::subsets::any::schema::snapshot::SSpaceSnapshot;
use crate::artifacts::space::SPACE_INDEX_DIALECT;
use crate::viewer::space_index::modes::view;
use crate::viewer::space_index::modes::view::windows::main;
use semio_framework_plugin::app::{Dialect, InteractionView};
use semio_framework_plugin::{ArtifactView, ArtifactViewer, ConfigView, Fault, NoConfig, NoConfigMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, UiNode, ViewEmit, Viewer};
use store::EngineHandles;

//#region 🔖️Command
/// 👁️ The viewer declares no actions this wave (no utilities, no mutations), so its typed command
/// channel has exactly one inert variant — mirrors `Din4108ViewCommand`'s own precedent.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SpaceIndexViewCommand {
    #[default]
    Noop,
}

impl protocol::OpBinary for SpaceIndexViewCommand {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(Vec::new())
    }
    fn decode_op(_bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(SpaceIndexViewCommand::Noop)
    }
}
//#endregion 🔖️Command

//#region 🔖️Viewer
#[derive(Default, Clone, Copy)]
pub struct SpaceIndexViewer;

impl ArtifactViewer for SpaceIndexViewer {
    type Snapshot = SSpaceSnapshot;
    type Mutation = crate::artifacts::space::standards::v1::subsets::any::schema::mutations::SSpaceMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = SpaceIndexViewCommand;

    const DIALECT: Dialect = SPACE_INDEX_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = crate::artifacts::space::S_SPACE_INDEX_DOCUMENT_SCHEMA;

    fn initial_snapshot() -> SSpaceSnapshot {
        SSpaceSnapshot::default()
    }

    /// 👁️ Structurally read-only: the sole `SpaceIndexViewCommand::Noop` variant never carries a config
    /// change, so this always returns the empty `ViewEmit`.
    fn handle(_command: &Self::Command, _doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, _interaction: &InteractionView<'_>, _engines: &EngineHandles) -> Result<ViewEmit<Self::ConfigMutation>, Fault> {
        Ok(ViewEmit::default())
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>) -> UiNode {
        match body_key {
            main::BODY_KEY => main::render(doc.snapshot),
            _ => semio_framework_plugin::ui_text(semio_framework_plugin::Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️Viewer

//#region 🔖️Manifest
pub fn create_space_index_viewer() -> semio_framework_plugin::AppDefinition {
    Viewer::builder(SPACE_INDEX_DIALECT)
        .document(["semio", "s", "space", "index"])
        .icon_id("layout-grid")
        .mode_def(view::definition())
        .default_mode_id(view::SPACE_INDEX_MODE_VIEW)
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
    fn create_space_index_viewer_builds_a_definition_for_this_dialect() {
        let def = create_space_index_viewer();
        assert_eq!(def.dialect, SPACE_INDEX_DIALECT.into());
    }

    #[test]
    fn viewer_dialect_matches_the_artifact_coordinate() {
        assert_eq!(<SpaceIndexViewer as ArtifactViewer>::DIALECT, SPACE_INDEX_DIALECT);
    }

    #[test]
    fn an_unknown_body_key_falls_back_to_a_text_node() {
        let snapshot = SSpaceSnapshot::default();
        let history = semio_framework_plugin::HistoryView::empty();
        let doc = ArtifactView::new(&snapshot, &history);
        let json = serde_json::to_string(&<SpaceIndexViewer as ArtifactViewer>::render("nope", &doc, &ConfigView { snapshot: &NoConfig::default() })).expect("json");
        assert!(json.contains("Unknown body"));
    }
}
//#endregion 🧪️Tests
