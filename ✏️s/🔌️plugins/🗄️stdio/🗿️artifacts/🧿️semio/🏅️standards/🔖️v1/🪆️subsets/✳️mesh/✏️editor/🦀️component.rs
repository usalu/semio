//! ✏️ Semio Mesh editor — thin, kit-based editor surface (ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.1). `SemioMeshEditor`
//! implements `ArtifactEditor`, wiring the shared `MeshWindowKit` to a single Main window.

use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::mutations::SemioMeshMutation;
use crate::editor::semio_mesh::modes::edit;
use crate::editor::semio_mesh::modes::edit::windows::main;
use semio_framework_plugin::{
    ArtifactEditor, ArtifactView, ConfigView, Dialect, DraftView, Editor, Emit, Fault, Label, NoConfig, NoConfigMutation, NoDraft, NoDraftMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation,
    StandardId, SubsetId, UiNode,
};
use semio_framework_plugin::app::InteractionView;
use store::EngineHandles;

//#region 🔖️Dialect
pub const SEMIO_MESH_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("mesh") };
pub const SEMIO_MESH_DOCUMENT_SCHEMA: &str = "stdio.semio.mesh";
//#endregion 🔖️Dialect

//#region 🔖️Command
/// ✏️ The Main window declares the shared `MeshWindowKit::editable_window_kind()`'s `set-vertex`
/// action. This subset's own `🧬️schema/🧬️mutations/📍move-vertex` is a REAL by-index reposition op
/// (`mesh_id`+`primitive_id`+`vertex_index` address, one `new_point` field) — the one true wired
/// exemplar this packet fully connects end to end (contract §2.6's own example, "set-vertex for
/// meshes"); every other subset in this packet's lease uses the minimal-command pattern instead,
/// reported per-subset in the packet report, because their own schemas expose no by-index "replace"
/// mutation today (only insert/remove/whole-document `SetSnapshot`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SemioMeshSetVertexArgs {
    pub mesh_index: usize,
    pub primitive_index: usize,
    pub vertex_index: usize,
    pub point: [f64; 3],
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SemioMeshEditCommand {
    SetVertex(SemioMeshSetVertexArgs),
}

impl protocol::OpBinary for SemioMeshEditCommand {
    async fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let SemioMeshEditCommand::SetVertex(args) = self;
        let payload = serde_json::json!({ "meshIndex": args.mesh_index, "primitiveIndex": args.primitive_index, "vertexIndex": args.vertex_index, "point": args.point });
        Ok(serde_json::to_vec(&payload).unwrap_or_default())
    }
    async fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let value: serde_json::Value = serde_json::from_slice(bytes).map_err(|error| protocol::ProtocolError::Malformed { what: "set-vertex op", offset: 0, detail: error.to_string() })?;
        let mesh_index = value.get("meshIndex").and_then(|v| v.as_u64()).unwrap_or(0) as usize;
        let primitive_index = value.get("primitiveIndex").and_then(|v| v.as_u64()).unwrap_or(0) as usize;
        let vertex_index = value.get("vertexIndex").and_then(|v| v.as_u64()).unwrap_or(0) as usize;
        let point = value.get("point").and_then(|v| v.as_array()).map(|array| {
            let get = |index: usize| array.get(index).and_then(|value| value.as_f64()).unwrap_or(0.0);
            [get(0), get(1), get(2)]
        }).unwrap_or([0.0, 0.0, 0.0]);
        Ok(SemioMeshEditCommand::SetVertex(SemioMeshSetVertexArgs { mesh_index, primitive_index, vertex_index, point }))
    }
}
//#endregion 🔖️Command

//#region 🔖️Editor
#[derive(Default, Clone, Copy)]
pub struct SemioMeshEditor;

impl ArtifactEditor for SemioMeshEditor {
    type Snapshot = SemioMeshSnapshot;
    type Mutation = SemioMeshMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = SemioMeshEditCommand;

    const DIALECT: Dialect = SEMIO_MESH_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = SEMIO_MESH_DOCUMENT_SCHEMA;

    async fn initial_snapshot() -> SemioMeshSnapshot {
        SemioMeshSnapshot::default()
    }

    async fn handle(command: &Self::Command, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, _interaction: &InteractionView<'_>, _draft: &DraftView<'_, Self::Draft>, _engines: &EngineHandles) -> Result<Emit<Self::Mutation, Self::ConfigMutation, Self::DraftMutation>, Fault> {
        let SemioMeshEditCommand::SetVertex(args) = command;
        let Some(mesh) = doc.snapshot.meshes.get(args.mesh_index) else { return Ok(Emit::default()); };
        let Some(primitive) = mesh.primitives.get(args.primitive_index) else { return Ok(Emit::default()); };
        if primitive.positions.get(args.vertex_index).is_none() {
            return Ok(Emit::default());
        }
        let new_point = crate::artifacts::semio::standards::v1::subsets::any::schema::geometry::SemioPoint3 { x: args.point[0], y: args.point[1], z: args.point[2] };
        let mutation = SemioMeshMutation::MoveVertex(crate::artifacts::semio::standards::v1::subsets::mesh::schema::mutations::move_vertex::mutation::MoveVertex {
            mesh_id: mesh.id.clone(),
            primitive_id: primitive.id.clone(),
            vertex_index: args.vertex_index,
            new_point,
        });
        Ok(Emit::mutations(vec![mutation]).await)
    }

    async fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>) -> UiNode {
        match body_key {
            main::BODY_KEY => main::render(doc.snapshot).await,
            _ => semio_framework_plugin::ui_text(Label::data(format!("Unknown body: {body_key}"))).await,
        }
    }

    async fn command_from_action(action: &str, args: Option<&serde_json::Value>) -> Result<Self::Command, Fault> {
        if action != "set-vertex" {
            return Err(Fault::new(
                semio_framework_plugin::FaultOrigin::App,
                semio_framework_plugin::FaultCode::new("app.command.unsupported"),
                format!("action '{action}' is not supported by SemioMeshEditor"),
            ));
        }
        let value = args.cloned().unwrap_or(serde_json::Value::Null);
        let mesh_index = value.get("meshIndex").and_then(|v| v.as_u64()).unwrap_or(0) as usize;
        let primitive_index = value.get("primitiveIndex").and_then(|v| v.as_u64()).unwrap_or(0) as usize;
        let vertex_index = value.get("vertexIndex").and_then(|v| v.as_u64()).unwrap_or(0) as usize;
        let point = value.get("point").and_then(|v| v.as_array()).map(|array| {
            let get = |index: usize| array.get(index).and_then(|value| value.as_f64()).unwrap_or(0.0);
            [get(0), get(1), get(2)]
        }).unwrap_or([0.0, 0.0, 0.0]);
        Ok(SemioMeshEditCommand::SetVertex(SemioMeshSetVertexArgs { mesh_index, primitive_index, vertex_index, point }))
    }
}
//#endregion 🔖️Editor

//#region 🔖️Manifest
pub async fn create_semio_mesh_editor() -> semio_framework_plugin::AppDefinition {
    Editor::builder(SEMIO_MESH_DIALECT)
        .await.document(["stdio", "semio"])
        .await.icon_id("box")
        .await.mode_def(edit::definition().await)
        .await.default_mode_id(edit::SEMIO_MESH_EDIT_MODE_ID)
        .await.window_kind_def(main::definition().await)
        .await.default_layout(edit::layout())
        .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn create_editor_builds_a_definition_for_the_editor_role() {
        let def = create_semio_mesh_editor();
        assert_eq!(def.role, semio_framework_plugin::AppRole::Editor);
        assert_eq!(def.dialect, SEMIO_MESH_DIALECT.into());
    }

    #[semio_framework_async_macros::async_test]
    async fn editor_dialect_matches_the_artifact_coordinate() {
        assert_eq!(<SemioMeshEditor as ArtifactEditor>::DIALECT, SEMIO_MESH_DIALECT);
    }

    #[semio_framework_async_macros::async_test]
    async fn editor_and_viewer_share_one_dialect() {
        semio_framework_plugin::testkit::assert_editor_and_viewer_share_dialect::<SemioMeshEditor, crate::viewer::semio_mesh::SemioMeshViewer>();
    }
}
//#endregion 🧪️Tests
