//! ✏️ Semio Mesh editor — thin, kit-based editor surface (ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.1). `SemioMeshEditor`
//! implements `ArtifactEditor`, wiring the shared `MeshWindowKit` to a single Main window.

use crate::editor::semio_mesh::modes::edit;
use crate::editor::semio_mesh::modes::edit::windows::main;
use crate::standards::v1::subsets::mesh::schema::mutations::{set_snapshot, SemioMeshMutation};
use crate::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;
use semio_framework::DslValue;
use semio_framework_plugin::app::InteractionView;
use semio_framework_plugin::{
    ArtifactEditor, ArtifactView, ConfigView, Dialect, DraftView, Editor, Emit, Fault, Label, NoConfig, NoConfigMutation, NoDraft, NoDraftMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, StandardId, SubsetId,
};
use store::EngineHandles;
use semio_s_artifact_stdio_contract::editing;

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
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let SemioMeshEditCommand::SetVertex(args) = self;
        let payload = serde_json::json!({ "meshIndex": args.mesh_index, "primitiveIndex": args.primitive_index, "vertexIndex": args.vertex_index, "point": args.point });
        Ok(serde_json::to_vec(&payload).unwrap_or_default())
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let value: serde_json::Value = serde_json::from_slice(bytes).map_err(|error| protocol::ProtocolError::Malformed { what: "set-vertex op", offset: 0, detail: error.to_string() })?;
        let mesh_index = value.get("meshIndex").and_then(|v| v.as_u64()).unwrap_or(0) as usize;
        let primitive_index = value.get("primitiveIndex").and_then(|v| v.as_u64()).unwrap_or(0) as usize;
        let vertex_index = value.get("vertexIndex").and_then(|v| v.as_u64()).unwrap_or(0) as usize;
        let point = value.get("point").and_then(|v| v.as_array()).map_or([0.0, 0.0, 0.0], |array| {
            let get = |index: usize| array.get(index).and_then(|value| value.as_f64()).unwrap_or(0.0);
            [get(0), get(1), get(2)]
        });
        Ok(SemioMeshEditCommand::SetVertex(SemioMeshSetVertexArgs { mesh_index, primitive_index, vertex_index, point }))
    }
}
semio_s_artifact_stdio_contract::snapshot_editing_command_roster!(SemioMeshEditCommand, ["set-vertex"]);
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
    type Command = editing::SnapshotEditingCommand<SemioMeshEditCommand>;

    const DIALECT: Dialect = SEMIO_MESH_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = SEMIO_MESH_DOCUMENT_SCHEMA;

    semio_s_artifact_stdio_contract::snapshot_details_editor_support! {
        owner_file: "semio/v1/mesh/editor",
        controller: "s.stdio.semio@v1/mesh#editor",
        artifact_schema: "stdio.semio.mesh",
        preparation: "stdio-semio-mesh-snapshot-edit"
    }

    fn command_id(command: &Self::Command) -> &'static str {
        editing::snapshot_editing_command_id(command, |_| "set-vertex")
    }

    fn initial_snapshot() -> SemioMeshSnapshot {
        SemioMeshSnapshot::default()
    }

    fn handle(
        command: &Self::Command,
        doc: &ArtifactView<'_, Self::Snapshot>,
        _cfg: &ConfigView<'_, Self::Config>,
        _interaction: &InteractionView<'_>,
        _view_state: Option<&semio_framework_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<Self::Mutation, Self::ConfigMutation, Self::DraftMutation>, Fault> {
        let editing::SnapshotEditingCommand::Native(SemioMeshEditCommand::SetVertex(args)) = command else {
            let editing::SnapshotEditingCommand::Edit(event) = command else { unreachable!() };
            return <Self as editing::SnapshotEditingEditor>::snapshot_edit_emit(event, doc.snapshot);
        };
        let Some(mesh) = doc.snapshot.meshes.get(args.mesh_index) else {
            return Ok(Emit::default());
        };
        let Some(primitive) = mesh.primitives.get(args.primitive_index) else {
            return Ok(Emit::default());
        };
        if primitive.positions.get(args.vertex_index).is_none() {
            return Ok(Emit::default());
        }
        let new_point = crate::standards::v1::subsets::base::schema::geometry::SemioPoint3 { x: args.point[0], y: args.point[1], z: args.point[2] };
        let mutation = SemioMeshMutation::MoveVertex(crate::standards::v1::subsets::mesh::schema::mutations::move_vertex::MoveVertex { mesh_id: mesh.id.clone(), primitive_id: primitive.id.clone(), vertex_index: args.vertex_index, new_point });
        Ok(Emit::mutations(vec![mutation]))
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        match body_key {
            main::BODY_KEY => main::render(doc.snapshot).map(semio_framework_plugin::built_to_component_tree),
            editing::SNAPSHOT_DETAILS_BODY_KEY => editing::render_snapshot_details(doc.snapshot, view_state.locale, "s.stdio.semio@v1/mesh#editor", &semio_framework_plugin::TreeWindows::for_body(view_state, editing::SNAPSHOT_DETAILS_BODY_KEY)).map(semio_framework_plugin::built_to_component_tree),
            _ => semio_framework_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
        }
    }

    fn command_from_action(action: &str, args: Option<&DslValue>) -> Result<Self::Command, Fault> {
        editing::snapshot_editing_command_from_action(action, args, |action, args| {
            if action != "set-vertex" {
                return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("app.command.unsupported"), format!("action '{action}' is not supported by SemioMeshEditor")));
            }
            let field = |key: &str| args.and_then(|value| value.get(key));
            let unsigned_field = |key: &str| field(key).and_then(DslValue::as_f64).filter(|number| number.is_finite() && *number >= 0.0).map(|number| number as usize);
            let mesh_index = unsigned_field("meshIndex").unwrap_or(0);
            let primitive_index = unsigned_field("primitiveIndex").unwrap_or(0);
            let vertex_index = unsigned_field("vertexIndex").unwrap_or(0);
            let point = field("point").and_then(DslValue::as_array).map_or([0.0, 0.0, 0.0], |array| {
                let get = |index: usize| array.get(index).and_then(DslValue::as_f64).unwrap_or(0.0);
                [get(0), get(1), get(2)]
            });
            Ok(SemioMeshEditCommand::SetVertex(SemioMeshSetVertexArgs { mesh_index, primitive_index, vertex_index, point }))
        })
    }
}

impl editing::SnapshotEditingEditor for SemioMeshEditor {
    fn snapshot_edit_event(command: &Self::Command) -> Option<&editing::SnapshotEditEvent> {
        match command { editing::SnapshotEditingCommand::Edit(event) => Some(event), _ => None }
    }

    fn snapshot_edit_is_admitted(event: &editing::SnapshotEditEvent, snapshot: &Self::Snapshot) -> bool {
        editing::snapshot_edit_value_is_admitted(event, snapshot)
    }

    fn snapshot_edit_emit(event: &editing::SnapshotEditEvent, snapshot: &Self::Snapshot) -> Result<Emit<Self::Mutation, Self::ConfigMutation, Self::DraftMutation>, Fault> {
        editing::snapshot_edit_set_snapshot(event, snapshot, |snapshot| SemioMeshMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }))
    }
}
//#endregion 🔖️Editor

//#region 🔖️Manifest
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn create_semio_mesh_editor() -> semio_framework_plugin::AppDefinition {
    let builder = Editor::builder(SEMIO_MESH_DIALECT).document(["stdio", "semio"]).icon_id("box").mode_def(edit::definition()).default_mode_id(edit::SEMIO_MESH_EDIT_MODE_ID).window_kind_def(main::definition()).window_kind_def(editing::snapshot_details_window_definition()).default_layout(edit::layout());
    editing::snapshot_edit_actions_with(builder).build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
