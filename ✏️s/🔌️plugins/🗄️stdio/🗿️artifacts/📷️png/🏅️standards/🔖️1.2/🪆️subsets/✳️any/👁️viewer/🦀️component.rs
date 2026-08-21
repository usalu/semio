//! 👁️ `png` viewer (any) — the read-only counterpart of `✏️editor` for this
//! subset (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.2). `PngViewer`
//! implements `ArtifactViewer`, never `ArtifactEditor`/`ArtifactApp` — `ViewerApp<PngViewer>` is
//! the sole runtime adapter, so this file can never structurally emit an artifact or draft mutation.
//! MUST NOT import anything from the sibling `editor` module (`policyViewerPurityBreaches`).

use crate::artifacts::png::standards::v1_2::subsets::any::schema::mutations::PngMutation;
use crate::artifacts::png::standards::v1_2::subsets::any::schema::snapshot::PngSnapshot;
use crate::artifacts::png::{PNG_DIALECT, STDIO_PNG_DOCUMENT_SCHEMA};
use crate::viewer::png::modes::view;
use crate::viewer::png::modes::view::windows::main;
use semio_framework_plugin::{ArtifactView, ArtifactViewer, ConfigView, Dialect, Fault, Label, NoConfig, NoConfigMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, UiNode, ViewEmit, Viewer};
use store::EngineHandles;

//#region 🔖️Command
/// 👁️ The viewer declares no actions, so its typed command channel has exactly one inert variant.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PngViewCommand {
    Noop,
}

impl protocol::OpBinary for PngViewCommand {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(Vec::new())
    }
    fn decode_op(_bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(PngViewCommand::Noop)
    }
}
//#endregion 🔖️Command

//#region 🔖️Viewer
#[derive(Default, Clone, Copy)]
pub struct PngViewer;

impl ArtifactViewer for PngViewer {
    type Snapshot = PngSnapshot;
    type Mutation = PngMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = PngViewCommand;

    const DIALECT: Dialect = PNG_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = STDIO_PNG_DOCUMENT_SCHEMA;

    async fn initial_snapshot() -> Self::Snapshot {
        PngSnapshot::default()
    }

    async fn handle(
        _command: &Self::Command,
        _doc: &ArtifactView<'_, Self::Snapshot>,
        _cfg: &ConfigView<'_, Self::Config>,
        _interaction: &semio_framework_plugin::app::InteractionView<'_>,
        _engines: &EngineHandles,
    ) -> Result<ViewEmit<Self::ConfigMutation>, Fault> {
        Ok(ViewEmit::default())
    }

    async fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>) -> semio_framework_plugin::ComponentTree {
        semio_framework_plugin::built_to_component_tree(match body_key {
            main::BODY_KEY => main::render(doc.snapshot),
            _ => semio_framework_plugin::built_text_node(Label::data(format!("Unknown body: {body_key}"))),
        })
    }
}
//#endregion 🔖️Viewer

//#region 🔖️Manifest
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn create_png_viewer() -> semio_framework_plugin::AppDefinition {
    Viewer::builder(PNG_DIALECT).document(["semio", "png"]).icon_id("image").mode_def(view::definition()).default_mode_id(view::MODE_ID).window_kind_def(main::definition()).default_layout(view::layout()).build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn create_viewer_builds_a_definition_for_the_viewer_role() {
        let def = create_png_viewer();
        assert_eq!(def.role, semio_framework::AppRole::Viewer);
        assert_eq!(def.dialect, PNG_DIALECT.into());
    }

    #[semio_framework_async_macros::async_test]
    async fn viewer_dialect_matches_the_artifact_coordinate() {
        assert_eq!(<PngViewer as ArtifactViewer>::DIALECT, PNG_DIALECT);
    }
}
//#endregion 🧪️Tests
