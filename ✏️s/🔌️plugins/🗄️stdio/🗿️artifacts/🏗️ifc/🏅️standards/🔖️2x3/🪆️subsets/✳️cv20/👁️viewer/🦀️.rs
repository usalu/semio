//! 👁️ IFC 2x3 Cv20 viewer — the read-only counterpart of `✏️editor` for this subset (ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.2). `Ifc2x3Cv20Viewer`
//! implements `ArtifactViewer`, never `ArtifactEditor`/`ArtifactApp` — `ViewerApp<Ifc2x3Cv20Viewer>`
//! (framework SDK) is the sole runtime adapter, so this file can never structurally emit an artifact
//! or draft mutation. MUST NOT reference the sibling editor module.

use crate::artifacts::ifc::standards::v2x3::subsets::cv20::schema::mutations::Ifc2x3Mutation;
use crate::artifacts::ifc::standards::v2x3::subsets::cv20::schema::snapshot::Ifc2x3Snapshot;
use crate::viewer::ifc2x3_cv20::modes::view;
use crate::viewer::ifc2x3_cv20::modes::view::windows::main;
use semio_framework_plugin::app::InteractionView;
use semio_framework_plugin::{ArtifactView, ArtifactViewer, ConfigView, Dialect, Fault, Label, NoConfig, NoConfigMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, StandardId, SubsetId, ViewEmit, Viewer};
use store::EngineHandles;

//#region 🔖️Dialect
/// 🪪️ Verified against this artifact's own `📸️snapshot/🦀️.rs`
/// `impl ArtifactAnalysis for …AnalyzerAnalysis { const DIALECT }` row (read, not guessed) — see
/// the packet report for the exact grep evidence per subset.
pub const IFC2X3_CV20_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.ifc", standard: StandardId("2x3"), subset: SubsetId("cv20") };
pub const IFC2X3_CV20_DOCUMENT_SCHEMA: &str = "stdio.ifc.2x3";
//#endregion 🔖️Dialect

//#region 🔖️Command
/// 👁️ The viewer declares no actions, so its typed command channel has exactly one inert variant.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Ifc2x3Cv20ViewCommand {
    #[default]
    Noop,
}

impl protocol::OpBinary for Ifc2x3Cv20ViewCommand {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(Vec::new())
    }
    fn decode_op(_bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(Ifc2x3Cv20ViewCommand::Noop)
    }
}
//#endregion 🔖️Command

//#region 🔖️Viewer
#[derive(Default, Clone, Copy)]
pub struct Ifc2x3Cv20Viewer;

impl ArtifactViewer for Ifc2x3Cv20Viewer {
    type Snapshot = Ifc2x3Snapshot;
    type Mutation = Ifc2x3Mutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = Ifc2x3Cv20ViewCommand;

    const DIALECT: Dialect = IFC2X3_CV20_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = IFC2X3_CV20_DOCUMENT_SCHEMA;

    fn initial_snapshot() -> Ifc2x3Snapshot {
        Ifc2x3Snapshot::default()
    }

    /// 👁️ Structurally read-only: the sole `Noop` variant never carries a config change.
    fn handle(_command: &Self::Command, _doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, _interaction: &InteractionView<'_>, _engines: &EngineHandles) -> Result<ViewEmit<Self::ConfigMutation>, Fault> {
        Ok(ViewEmit::default())
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        match body_key {
            main::BODY_KEY => main::render(doc.snapshot).map(semio_framework_plugin::built_to_component_tree),
            _ => return semio_framework_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️Viewer

//#region 🔖️Manifest
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn create_ifc2x3_cv20_viewer() -> semio_framework_plugin::AppDefinition {
    Viewer::builder(IFC2X3_CV20_DIALECT).document(["stdio", "ifc2x3"]).icon_id("box").mode_def(view::definition()).default_mode_id(view::IFC2X3_CV20_VIEW_MODE_ID).window_kind_def(main::definition()).default_layout(view::layout()).build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn create_viewer_builds_a_definition_for_the_viewer_role() {
        let def = create_ifc2x3_cv20_viewer();
        assert_eq!(def.role, semio_framework_plugin::AppRole::Viewer);
        assert_eq!(def.dialect, IFC2X3_CV20_DIALECT.into());
    }

    #[semio_framework_async_macros::async_test]
    async fn viewer_dialect_matches_the_artifact_coordinate() {
        assert_eq!(<Ifc2x3Cv20Viewer as ArtifactViewer>::DIALECT, IFC2X3_CV20_DIALECT);
    }

    #[semio_framework_async_macros::async_test]
    async fn viewer_never_mutates_the_document_or_draft_store() {
        semio_framework_plugin::testkit::assert_viewer_never_mutates::<Ifc2x3Cv20Viewer>();
    }
}
//#endregion 🧪️Tests
