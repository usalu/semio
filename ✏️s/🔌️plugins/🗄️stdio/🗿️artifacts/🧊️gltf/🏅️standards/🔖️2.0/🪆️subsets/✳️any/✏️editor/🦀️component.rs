//! ✏️ glTF editor — thin, kit-based editor surface (ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.1). `GltfAnyEditor`
//! implements `ArtifactEditor`, wiring the shared `MeshWindowKit` to a single Main window.

use crate::artifacts::gltf::standards::v2_0::subsets::any::schema::snapshot::GltfSnapshot;
use crate::artifacts::gltf::GltfMutation;
use crate::editor::gltf::modes::edit;
use crate::editor::gltf::modes::edit::windows::main;
use semio_framework_plugin::app::InteractionView;
use semio_framework_plugin::{
    ArtifactEditor, ArtifactView, ConfigView, Dialect, DraftView, Editor, Emit, Fault, Label, NoConfig, NoConfigMutation, NoDraft, NoDraftMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, StandardId, SubsetId, UiNode,
};
use store::EngineHandles;

//#region 🔖️Dialect
pub const GLTF_ANY_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.gltf", standard: StandardId("2.0"), subset: SubsetId::ANY };
pub const GLTF_ANY_DOCUMENT_SCHEMA: &str = "stdio.gltf";
//#endregion 🔖️Dialect

//#region 🔖️Command
/// ✏️ The Main window declares the shared `MeshWindowKit::editable_window_kind()`'s
/// `set-vertex` action (contract §2.6), but this subset's own `🧬️schema/🧬️mutations` declares
/// no by-index "replace"/"set" op that action could honestly back today (only insert/remove and
/// whole-document `SetSnapshot`) — per this ticket's explicit allowance, the editor still exists
/// with a MINIMAL command set: the window really advertises the action, `handle` is a real dispatch
/// (not `unreachable!()`) that is a no-op today, rather than inventing a mutation the schema does
/// not have. Report, don't invent.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum GltfAnyEditCommand {
    #[default]
    SetVertex,
}

impl protocol::OpBinary for GltfAnyEditCommand {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(Vec::new())
    }
    fn decode_op(_bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(GltfAnyEditCommand::SetVertex)
    }
}
//#endregion 🔖️Command

//#region 🔖️Editor
#[derive(Default, Clone, Copy)]
pub struct GltfAnyEditor;

impl ArtifactEditor for GltfAnyEditor {
    type Snapshot = GltfSnapshot;
    type Mutation = GltfMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = GltfAnyEditCommand;

    const DIALECT: Dialect = GLTF_ANY_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = GLTF_ANY_DOCUMENT_SCHEMA;

    async fn initial_snapshot() -> GltfSnapshot {
        GltfSnapshot::default()
    }

    async fn handle(
        command: &Self::Command,
        _doc: &ArtifactView<'_, Self::Snapshot>,
        _cfg: &ConfigView<'_, Self::Config>,
        _interaction: &InteractionView<'_>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<Self::Mutation, Self::ConfigMutation, Self::DraftMutation>, Fault> {
        let _ = command;
        Ok(Emit::default())
    }

    async fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>) -> semio_framework_plugin::ComponentTree {
        semio_framework_plugin::built_to_component_tree(match body_key {
            main::BODY_KEY => main::render(doc.snapshot),
            _ => semio_framework_plugin::built_text_node(Label::data(format!("Unknown body: {body_key}"))),
        })
    }
}
//#endregion 🔖️Editor

//#region 🔖️Manifest
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn create_gltf_any_editor() -> semio_framework_plugin::AppDefinition {
    Editor::builder(GLTF_ANY_DIALECT).document(["stdio", "gltf"]).icon_id("box").mode_def(edit::definition()).default_mode_id(edit::GLTF_ANY_EDIT_MODE_ID).window_kind_def(main::definition()).default_layout(edit::layout()).build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn create_editor_builds_a_definition_for_the_editor_role() {
        let def = create_gltf_any_editor();
        assert_eq!(def.role, semio_framework_plugin::AppRole::Editor);
        assert_eq!(def.dialect, GLTF_ANY_DIALECT.into());
    }

    #[semio_framework_async_macros::async_test]
    async fn editor_dialect_matches_the_artifact_coordinate() {
        assert_eq!(<GltfAnyEditor as ArtifactEditor>::DIALECT, GLTF_ANY_DIALECT);
    }

    #[semio_framework_async_macros::async_test]
    async fn editor_and_viewer_share_one_dialect() {
        semio_framework_plugin::testkit::assert_editor_and_viewer_share_dialect::<GltfAnyEditor, crate::viewer::gltf::GltfAnyViewer>();
    }
}
//#endregion 🧪️Tests
