//! 👁️ `svg` viewer (basic) — the read-only counterpart of `✏️editor` for this
//! subset (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.2). `SvgBasicViewer`
//! implements `ArtifactViewer`, never `ArtifactEditor`/`ArtifactApp` — `ViewerApp<SvgBasicViewer>` is
//! the sole runtime adapter, so this file can never structurally emit an artifact or draft mutation.
//! MUST NOT import anything from the sibling `editor` module (`policyViewerPurityBreaches`).

use crate::artifacts::svg::{SVG_BASIC_DIALECT, STDIO_SVG_DOCUMENT_SCHEMA};
use crate::artifacts::svg::standards::v1_1::subsets::basic::schema::mutations::SvgMutation;
use crate::artifacts::svg::standards::v1_1::subsets::basic::schema::snapshot::SvgSnapshot;
use crate::viewer::svg_basic::modes::view;
use crate::viewer::svg_basic::modes::view::windows::main;
use semio_framework_plugin::{ArtifactView, ArtifactViewer, ConfigView, Dialect, Fault, Label, NoConfig, NoConfigMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, UiNode, ViewEmit, Viewer};
use store::EngineHandles;

//#region 🔖️Command
/// 👁️ The viewer declares no actions, so its typed command channel has exactly one inert variant.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SvgBasicViewCommand {
    Noop,
}

impl protocol::OpBinary for SvgBasicViewCommand {
    async fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(Vec::new())
    }
    async fn decode_op(_bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(SvgBasicViewCommand::Noop)
    }
}
//#endregion 🔖️Command

//#region 🔖️Viewer
#[derive(Default, Clone, Copy)]
pub struct SvgBasicViewer;

impl ArtifactViewer for SvgBasicViewer {
    type Snapshot = SvgSnapshot;
    type Mutation = SvgMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = SvgBasicViewCommand;

    const DIALECT: Dialect = SVG_BASIC_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = STDIO_SVG_DOCUMENT_SCHEMA;

    async fn initial_snapshot() -> Self::Snapshot {
        SvgSnapshot::default()
    }

    async fn handle(_command: &Self::Command, _doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, _interaction: &semio_framework_plugin::app::InteractionView<'_>, _engines: &EngineHandles) -> Result<ViewEmit<Self::ConfigMutation>, Fault> {
        Ok(ViewEmit::default())
    }

    async fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>) -> UiNode {
        match body_key {
            main::BODY_KEY => main::render(doc.snapshot).await,
            _ => semio_framework_plugin::ui_text(Label::data(format!("Unknown body: {body_key}"))).await,
        }
    }
}
//#endregion 🔖️Viewer

//#region 🔖️Manifest
pub async fn create_svg_basic_viewer() -> semio_framework_plugin::AppDefinition {
    Viewer::builder(SVG_BASIC_DIALECT)
        .await.document(["semio", "svg"])
        .await.icon_id("image")
        .await.mode_def(view::definition().await)
        .await.default_mode_id(view::MODE_ID)
        .await.window_kind_def(main::definition().await)
        .await.default_layout(view::layout())
        .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn create_viewer_builds_a_definition_for_the_viewer_role() {
        let def = create_svg_basic_viewer();
        assert_eq!(def.role, semio_framework::AppRole::Viewer);
        assert_eq!(def.dialect, SVG_BASIC_DIALECT.into());
    }

    #[semio_framework_async_macros::async_test]
    async fn viewer_dialect_matches_the_artifact_coordinate() {
        assert_eq!(<SvgBasicViewer as ArtifactViewer>::DIALECT, SVG_BASIC_DIALECT);
    }
}
//#endregion 🧪️Tests
