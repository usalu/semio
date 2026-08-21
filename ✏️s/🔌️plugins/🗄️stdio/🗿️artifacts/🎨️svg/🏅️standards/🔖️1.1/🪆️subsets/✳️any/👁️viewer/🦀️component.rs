//! 👁️ `svg` viewer (any) — the read-only counterpart of `✏️editor` for this
//! subset (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.2). `SvgAnyViewer`
//! implements `ArtifactViewer`, never `ArtifactEditor`/`ArtifactApp` — `ViewerApp<SvgAnyViewer>` is
//! the sole runtime adapter, so this file can never structurally emit an artifact or draft mutation.
//! MUST NOT import anything from the sibling `editor` module (`policyViewerPurityBreaches`).

use crate::artifacts::svg::standards::v1_1::subsets::any::schema::mutations::SvgMutation;
use crate::artifacts::svg::standards::v1_1::subsets::any::schema::snapshot::SvgSnapshot;
use crate::artifacts::svg::{STDIO_SVG_DOCUMENT_SCHEMA, SVG_ANY_DIALECT};
use crate::viewer::svg_any::modes::view;
use crate::viewer::svg_any::modes::view::windows::main;
use semio_framework_plugin::{ArtifactView, ArtifactViewer, ConfigView, Dialect, Fault, Label, NoConfig, NoConfigMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, UiNode, ViewEmit, Viewer};
use store::EngineHandles;

//#region 🔖️Command
/// 👁️ The viewer declares no actions, so its typed command channel has exactly one inert variant.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SvgAnyViewCommand {
    Noop,
}

impl protocol::OpBinary for SvgAnyViewCommand {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(Vec::new())
    }
    fn decode_op(_bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(SvgAnyViewCommand::Noop)
    }
}
//#endregion 🔖️Command

//#region 🔖️Viewer
#[derive(Default, Clone, Copy)]
pub struct SvgAnyViewer;

impl ArtifactViewer for SvgAnyViewer {
    type Snapshot = SvgSnapshot;
    type Mutation = SvgMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = SvgAnyViewCommand;

    const DIALECT: Dialect = SVG_ANY_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = STDIO_SVG_DOCUMENT_SCHEMA;

    async fn initial_snapshot() -> Self::Snapshot {
        SvgSnapshot::default()
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
pub fn create_svg_any_viewer() -> semio_framework_plugin::AppDefinition {
    Viewer::builder(SVG_ANY_DIALECT).document(["semio", "svg"]).icon_id("image").mode_def(view::definition()).default_mode_id(view::MODE_ID).window_kind_def(main::definition()).default_layout(view::layout()).build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn create_viewer_builds_a_definition_for_the_viewer_role() {
        let def = create_svg_any_viewer();
        assert_eq!(def.role, semio_framework::AppRole::Viewer);
        assert_eq!(def.dialect, SVG_ANY_DIALECT.into());
    }

    #[semio_framework_async_macros::async_test]
    async fn viewer_dialect_matches_the_artifact_coordinate() {
        assert_eq!(<SvgAnyViewer as ArtifactViewer>::DIALECT, SVG_ANY_DIALECT);
    }
}
//#endregion 🧪️Tests
