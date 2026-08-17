//! 👁️ EPW viewer — the read-only counterpart of `✏️editor` for this subset (ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.2). `EpwViewer` implements
//! `ArtifactViewer`, never `ArtifactEditor`/`ArtifactApp` — `ViewerApp<EpwViewer>` (framework SDK) is
//! the sole runtime adapter, so this file can never structurally emit an artifact mutation. Must not
//! import anything from the sibling mutation-capable surface (policy forbids the substring outright,
//! including inside comments).

use crate::artifacts::epw::{EpwMutation, EpwSnapshot, STDIO_EPW_DOCUMENT_SCHEMA};
use crate::viewer::epw::modes::view;
use crate::viewer::epw::modes::view::windows::main;
use semio_framework_plugin::{ArtifactView, ArtifactViewer, ConfigView, Dialect, Fault, Label, NoConfig, NoConfigMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, StandardId, SubsetId, UiNode, ViewEmit, Viewer};

//#region 🔖️Dialect
/// 🎯️ This surface's dialect coordinate — `s.stdio.epw@energyplus/*`. Kept as its own independent
/// const (never imported from the sibling authoring surface) so this file can never reach the
/// mutation-capable module even transitively.
pub const EPW_VIEWER_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.epw", standard: StandardId("energyplus"), subset: SubsetId("*") };
//#endregion 🔖️Dialect

//#region 🔖️Command
/// 👁️ The viewer declares no actions (no utilities, no mutations), so its typed command channel has
/// exactly one inert variant.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum EpwViewCommand {
    #[default]
    Noop,
}

impl protocol::OpBinary for EpwViewCommand {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(Vec::new())
    }
    fn decode_op(_bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(EpwViewCommand::Noop)
    }
}
//#endregion 🔖️Command

//#region 🔖️Viewer
#[derive(Default, Clone, Copy)]
pub struct EpwViewer;

impl ArtifactViewer for EpwViewer {
    type Snapshot = EpwSnapshot;
    type Mutation = EpwMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = EpwViewCommand;

    const DIALECT: Dialect = EPW_VIEWER_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = STDIO_EPW_DOCUMENT_SCHEMA;

    fn initial_snapshot() -> EpwSnapshot {
        EpwSnapshot::default()
    }

    /// 👁️ Structurally read-only: the sole `EpwViewCommand::Noop` variant never carries a config
    /// change, so this always returns the empty `ViewEmit`.
    fn handle(_command: &Self::Command, _doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, _interaction: &semio_framework_plugin::app::InteractionView<'_>, _engines: &store::EngineHandles) -> Result<ViewEmit<Self::ConfigMutation>, Fault> {
        Ok(ViewEmit::default())
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>) -> UiNode {
        match body_key {
            main::BODY_KEY => main::render(doc.snapshot),
            _ => semio_framework_plugin::ui_text(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️Viewer

//#region 🔖️Manifest
pub fn create_epw_viewer() -> semio_framework_plugin::AppDefinition {
    Viewer::builder(EPW_VIEWER_DIALECT)
        .document(["stdio", "epw"])
        .icon_id("cloud-sun")
        .mode_def(view::definition())
        .default_mode_id(view::EPW_VIEW_MODE_ID)
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
    fn create_epw_viewer_builds_a_definition_for_the_viewer_role() {
        let def = create_epw_viewer();
        assert_eq!(def.role, semio_framework_plugin::AppRole::Viewer);
        assert_eq!(def.dialect, EPW_VIEWER_DIALECT.into());
    }

    #[test]
    fn viewer_dialect_matches_the_artifact_coordinate() {
        assert_eq!(<EpwViewer as ArtifactViewer>::DIALECT, EPW_VIEWER_DIALECT);
    }

    #[test]
    fn viewer_declares_the_main_window() {
        let def = create_epw_viewer();
        assert!(def.window_kinds.iter().any(|window| window.id == main::WINDOW_KIND_ID));
    }
}
//#endregion 🧪️Tests
