//! 👁️ `jpg` viewer (baseline) — the read-only counterpart of `✏️editor` for this
//! subset (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.2). `JpgBaselineViewer`
//! implements `ArtifactViewer`, never `ArtifactEditor`/`ArtifactApp` — `ViewerApp<JpgBaselineViewer>` is
//! the sole runtime adapter, so this file can never structurally emit an artifact or draft mutation.
//! MUST NOT import anything from the sibling `editor` module (`policyViewerPurityBreaches`).

use crate::artifacts::jpg::standards::v_jfif_1_01::subsets::baseline::schema::mutations::JpgMutation;
use crate::artifacts::jpg::standards::v_jfif_1_01::subsets::baseline::schema::snapshot::JpgSnapshot;
use crate::artifacts::jpg::{JPG_BASELINE_DIALECT, STDIO_JPG_DOCUMENT_SCHEMA};
use crate::viewer::jpg_baseline::modes::view;
use crate::viewer::jpg_baseline::modes::view::windows::main;
use semio_framework_plugin::{ArtifactView, ArtifactViewer, ConfigView, Dialect, Fault, Label, NoConfig, NoConfigMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, UiNode, ViewEmit, Viewer};
use store::EngineHandles;

//#region 🔖️Command
/// 👁️ The viewer declares no actions, so its typed command channel has exactly one inert variant.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JpgBaselineViewCommand {
    Noop,
}

impl protocol::OpBinary for JpgBaselineViewCommand {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(Vec::new())
    }
    fn decode_op(_bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(JpgBaselineViewCommand::Noop)
    }
}
//#endregion 🔖️Command

//#region 🔖️Viewer
#[derive(Default, Clone, Copy)]
pub struct JpgBaselineViewer;

impl ArtifactViewer for JpgBaselineViewer {
    type Snapshot = JpgSnapshot;
    type Mutation = JpgMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = JpgBaselineViewCommand;

    const DIALECT: Dialect = JPG_BASELINE_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = STDIO_JPG_DOCUMENT_SCHEMA;

    async fn initial_snapshot() -> Self::Snapshot {
        JpgSnapshot::default()
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
pub fn create_jpg_baseline_viewer() -> semio_framework_plugin::AppDefinition {
    Viewer::builder(JPG_BASELINE_DIALECT).document(["semio", "jpg"]).icon_id("image").mode_def(view::definition()).default_mode_id(view::MODE_ID).window_kind_def(main::definition()).default_layout(view::layout()).build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn create_viewer_builds_a_definition_for_the_viewer_role() {
        let def = create_jpg_baseline_viewer();
        assert_eq!(def.role, semio_framework::AppRole::Viewer);
        assert_eq!(def.dialect, JPG_BASELINE_DIALECT.into());
    }

    #[semio_framework_async_macros::async_test]
    async fn viewer_dialect_matches_the_artifact_coordinate() {
        assert_eq!(<JpgBaselineViewer as ArtifactViewer>::DIALECT, JPG_BASELINE_DIALECT);
    }
}
//#endregion 🧪️Tests
