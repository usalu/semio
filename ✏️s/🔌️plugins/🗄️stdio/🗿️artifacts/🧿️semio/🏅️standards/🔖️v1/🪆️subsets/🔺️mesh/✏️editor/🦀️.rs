//! ✏️ Semio Mesh editor — thin, kit-based editor surface (ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.1). `SemioMeshEditor`
//! implements `ArtifactEditor`, wiring the shared `MeshWindowKit` to a single Main window.

use crate::editor::modes::edit;
use crate::editor::modes::edit::windows::main;
use crate::schema::mutations::SemioMeshMutation;
use crate::schema::snapshot::SemioMeshSnapshot;
use semio_framework::DslValue;
use semio_plugin::app::InteractionView;
use semio_plugin::retained_command::ArtifactCommandInputs;
use semio_plugin::retained_command::ArtifactCommandWork;
use semio_plugin::retained_command::ArtifactCommandWorkStep;
use semio_plugin::ActionArgDef;
use semio_plugin::ActionDefinition;
use semio_plugin::ActionKind;
use semio_plugin::ArgSchema;
use semio_plugin::ArtifactEditor;
use semio_plugin::ArtifactView;
use semio_plugin::ConfigView;
use {semio_framework_artifact_reference::Dialect};
use semio_plugin::DraftView;
use semio_plugin::Editor;
use semio_plugin::EditorApp;
use semio_plugin::Emit;
use semio_plugin::Fault;
use semio_ui_locale::Label;
use semio_ui_locale::LocalizedLabel;
use semio_plugin::NoConfig;
use semio_plugin::NoConfigMutation;
use semio_plugin::NoDraft;
use semio_plugin::NoDraftMutation;
use semio_plugin::NoPresence;
use semio_plugin::NoPresenceMutation;
use semio_plugin::NoTransient;
use semio_plugin::NoTransientMutation;
use {semio_framework_artifact_reference::StandardId};
use {semio_framework_artifact_reference::SubsetId};
use semio_s_artifact_stdio_contract::editing;
use semio_framework_2d_compute::EngineHandles;

#[path = "📬️preparation/🦀️.rs"]
mod preparation;

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
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
pub struct SemioMeshSetVertexArgs {
    pub mesh_id: String,
    pub primitive_id: String,
    pub vertex_index: usize,
    pub point: [f64; 3],
}

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
pub enum SemioMeshEditCommand {
    SetActiveExample { example_id: String },
    SetVertex(SemioMeshSetVertexArgs),
}

impl protocol::OpBinary for SemioMeshEditCommand {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> { Ok(semio_framework_pack_json::to_json_string(self).into_bytes()) }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let value=semio_framework_pack_json::parse_bytes(bytes,semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error|protocol::ProtocolError::Malformed{what:"SemioMeshEditCommand",offset:0,detail:error.to_string()})?;
        <Self as semio_framework_value::FromValue>::from_value(semio_framework_pack_json::to_dsl_value(&value)).map_err(|error|protocol::ProtocolError::Malformed{what:"SemioMeshEditCommand",offset:0,detail:error.to_string()})
    }
}
semio_s_artifact_stdio_contract::snapshot_editing_command_roster!(SemioMeshEditCommand, ["set-vertex", "setActiveExample"]);

fn semio_mesh_editor_example_snapshot(example_id:&str)->Result<SemioMeshSnapshot,Fault>{
    semio_s_artifact_stdio_contract::parse_example_snapshot(example_id,semio_s_artifact_stdio_semio_mesh_examples::examples::cube::ID,semio_s_artifact_stdio_semio_mesh_examples::examples::cube::PRIMARY_TEXT,SEMIO_MESH_DOCUMENT_SCHEMA,|snapshot:&SemioMeshSnapshot|snapshot.schema.as_str())
}

fn vertex_index_arg() -> ActionArgDef {
    let mut argument = ActionArgDef::number("vertexIndex", LocalizedLabel::native("Vertex Index", "Vertexindex")).required();
    if let ArgSchema::Number { min, step, integer, .. } = &mut argument.schema {
        *min = Some(0.0);
        *step = Some(1.0);
        *integer = true;
    }
    argument
}

pub fn set_vertex_action() -> ActionDefinition {
    let mut action = ActionDefinition::bounded_catalog("set-vertex", LocalizedLabel::native("Move Vertex", "Vertex verschieben"), ActionKind::Mutation).with_args(vec![
        ActionArgDef::text("meshId", LocalizedLabel::native("Mesh ID", "Mesh-ID")).required(),
        ActionArgDef::text("primitiveId", LocalizedLabel::native("Primitive ID", "Primitiv-ID")).required(),
        vertex_index_arg(),
        ActionArgDef::vector("point", LocalizedLabel::native("Target Point", "Zielpunkt"), 3).required(),
    ]);
    action.semantics.execution.interactive_job = semio_plugin::InteractiveJobClassification::Migrated;
    action
}

#[derive(Default)]
struct SemioMeshSetVertexWork {
    mesh_cursor: usize,
    primitive_cursor: usize,
    mesh_index: Option<usize>,
    primitive_index: Option<usize>,
    meshes_scanned: bool,
    primitives_scanned: bool,
    complete: bool,
}

fn resolve_unique_mesh_target(snapshot: &SemioMeshSnapshot, mesh_id: &str, primitive_id: &str) -> Result<(usize, usize), Fault> {
    let mut mesh_index = None;
    for (index, _mesh) in snapshot.meshes.iter().enumerate().filter(|(_, mesh)| mesh.id == mesh_id) {
        if mesh_index.replace(index).is_some() {
            return Err(Fault::from(format!("mesh id '{mesh_id}' is ambiguous")));
        }
    }
    let mesh_index = mesh_index.ok_or_else(|| Fault::from(format!("mesh '{mesh_id}' does not exist")))?;
    let mut primitive_index = None;
    for (index, _primitive) in snapshot.meshes[mesh_index].primitives.iter().enumerate().filter(|(_, primitive)| primitive.id == primitive_id) {
        if primitive_index.replace(index).is_some() {
            return Err(Fault::from(format!("primitive id '{primitive_id}' is ambiguous in mesh '{mesh_id}'")));
        }
    }
    let primitive_index = primitive_index.ok_or_else(|| Fault::from(format!("primitive '{primitive_id}' does not exist in mesh '{mesh_id}'")))?;
    Ok((mesh_index, primitive_index))
}

pub fn move_vertex_mutation(snapshot: &SemioMeshSnapshot, args: &SemioMeshSetVertexArgs) -> Result<Option<SemioMeshMutation>, Fault> {
    let (mesh_index, primitive_index) = resolve_unique_mesh_target(snapshot, &args.mesh_id, &args.primitive_id)?;
    let primitive = &snapshot.meshes[mesh_index].primitives[primitive_index];
    let current = primitive.positions.get(args.vertex_index).ok_or_else(|| Fault::from(format!("vertex {} does not exist in primitive '{}'", args.vertex_index, args.primitive_id)))?;
    let new_point = semio_s_artifact_stdio_semio_base::schema::geometry::SemioPoint3 { x: args.point[0], y: args.point[1], z: args.point[2] };
    if *current == new_point {
        return Ok(None);
    }
    Ok(Some(SemioMeshMutation::MoveVertex(crate::schema::mutations::move_vertex::MoveVertex { mesh_id: args.mesh_id.clone(), primitive_id: args.primitive_id.clone(), vertex_index: args.vertex_index, new_point })))
}

impl ArtifactCommandWork<EditorApp<SemioMeshEditor>> for SemioMeshSetVertexWork {
    fn terminal_frame_release_bytes(&self)->Option<usize>{Some(size_of::<Self>())}
    fn tool_id(&self) -> &'static str {
        "set-vertex"
    }

    fn extent(
        &self,
        command: &editing::SnapshotEditingCommand<SemioMeshEditCommand>,
        _snapshot: &SemioMeshSnapshot,
        _interaction: &protocol::InteractionState,
        _context: Option<&semio_plugin::app::ArtifactOwnedToolJobContext<EditorApp<SemioMeshEditor>>>,
    ) -> Option<usize> {
        let editing::SnapshotEditingCommand::Native(SemioMeshEditCommand::SetVertex(args)) = command else { return None };
        (!args.mesh_id.is_empty() && !args.primitive_id.is_empty() && args.point.iter().all(|value| value.is_finite())).then_some(2)
    }

    fn work_demands(&self, input: &ArtifactCommandInputs<'_, EditorApp<SemioMeshEditor>>, _maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        let editing::SnapshotEditingCommand::Native(SemioMeshEditCommand::SetVertex(args)) = input.command else {
            return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "stdio.semio.mesh.set-vertex.command-mismatch"));
        };
        let text = args.mesh_id.len().saturating_add(args.primitive_id.len());
        Ok(semio_framework_value::RetirementDemand { copy_bytes: text, capacity_bytes: text.saturating_add(size_of::<SemioMeshMutation>()), release_bytes: 0, depth: 1 })
    }

    fn step(&mut self, input: &ArtifactCommandInputs<'_, EditorApp<SemioMeshEditor>>, _cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactCommandWorkStep<EditorApp<SemioMeshEditor>>, semio_framework_job::OriginalTurnFailure> {
        let original: Result<_, Fault> = (|| {
        if self.complete {
            return Err(Fault::from("stdio.semio.mesh.set-vertex.work-closed"));
        }
        let editing::SnapshotEditingCommand::Native(SemioMeshEditCommand::SetVertex(args)) = input.command else {
            return Err(Fault::from("stdio.semio.mesh.set-vertex.command-mismatch"));
        };
        if !self.meshes_scanned {
            let Some(mesh) = input.snapshot.meshes.get(self.mesh_cursor) else {
                self.meshes_scanned = true;
                if self.mesh_index.is_none() {
                    return Err(Fault::from(format!("mesh '{}' does not exist", args.mesh_id)));
                }
                return Ok(ArtifactCommandWorkStep::Replay { stage: "semio-mesh-resolve-mesh", preview: br#"{"en":"Checking meshes","de":"Netze werden gepr\u00fcft"}"# });
            };
            let index = self.mesh_cursor;
            self.mesh_cursor += 1;
            if mesh.id == args.mesh_id {
                if self.mesh_index.replace(index).is_some() {
                    return Err(Fault::from(format!("mesh id '{}' is ambiguous", args.mesh_id)));
                }
            }
            return Ok(ArtifactCommandWorkStep::Replay { stage: "semio-mesh-resolve-mesh", preview: br#"{"en":"Checking meshes","de":"Netze werden gepr\u00fcft"}"# });
        }
        let mesh = &input.snapshot.meshes[self.mesh_index.expect("resolved mesh index")];
        if !self.primitives_scanned {
            let Some(primitive) = mesh.primitives.get(self.primitive_cursor) else {
                self.primitives_scanned = true;
                if self.primitive_index.is_none() {
                    return Err(Fault::from(format!("primitive '{}' does not exist in mesh '{}'", args.primitive_id, args.mesh_id)));
                }
                return Ok(ArtifactCommandWorkStep::Replay { stage: "semio-mesh-resolve-primitive", preview: br#"{"en":"Checking mesh primitives","de":"Netzprimitive werden gepr\u00fcft"}"# });
            };
            let index = self.primitive_cursor;
            self.primitive_cursor += 1;
            if primitive.id == args.primitive_id && self.primitive_index.replace(index).is_some() {
                return Err(Fault::from(format!("primitive id '{}' is ambiguous in mesh '{}'", args.primitive_id, args.mesh_id)));
            }
            return Ok(ArtifactCommandWorkStep::Replay { stage: "semio-mesh-resolve-primitive", preview: br#"{"en":"Checking mesh primitives","de":"Netzprimitive werden gepr\u00fcft"}"# });
        }
        let primitive = &mesh.primitives[self.primitive_index.expect("resolved primitive index")];
        let Some(current) = primitive.positions.get(args.vertex_index) else {
            return Err(Fault::from(format!("vertex {} does not exist in primitive '{}'", args.vertex_index, args.primitive_id)));
        };
        self.complete = true;
        let new_point = semio_s_artifact_stdio_semio_base::schema::geometry::SemioPoint3 { x: args.point[0], y: args.point[1], z: args.point[2] };
        if *current == new_point {
            return Ok(ArtifactCommandWorkStep::Complete(Emit::default()));
        }
        Ok(ArtifactCommandWorkStep::Complete(Emit::mutations(vec![SemioMeshMutation::MoveVertex(crate::schema::mutations::move_vertex::MoveVertex {
            mesh_id: args.mesh_id.clone(),
            primitive_id: args.primitive_id.clone(),
            vertex_index: args.vertex_index,
            new_point,
        })])))
    
        })();
        original.map_err(semio_framework_job::OriginalTurnFailure::from)
    }

    fn begin_close(&mut self) {
        self.complete = true;
    }
}

fn semio_mesh_set_vertex_work(tool_id: &'static str) -> Box<dyn ArtifactCommandWork<EditorApp<SemioMeshEditor>>> {
    if tool_id==semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID{return editing::bounded_native_edit_work::<SemioMeshEditor>(tool_id)}
    Box::new(SemioMeshSetVertexWork::default())
}
//#endregion 🔖️Command

pub use semio_s_artifact_stdio_semio_mesh_io::editor::edit_rules;

//#region 🔖️Editor
#[derive(Default, Clone, Copy)]
pub struct SemioMeshEditor;

impl ArtifactEditor for SemioMeshEditor {
    fn examples()->Vec<semio_plugin::ExampleSource>{vec![semio_s_artifact_stdio_semio_mesh_examples::examples::cube::source()]}

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

    semio_s_artifact_stdio_semio_base::semio_natural_file!(SemioMeshSnapshot, "stdio.semio.mesh");

    semio_s_artifact_stdio_contract::snapshot_details_editor_support! {
        owner_file: "semio/v1/mesh/editor",
        controller: "s.stdio.semio@v1/mesh#editor",
        artifact_schema: "stdio.semio.mesh",
        preparation: "stdio-semio-mesh-snapshot-edit",
        bounded_native: true
    }

    fn command_id(command: &Self::Command) -> &'static str {
        editing::snapshot_editing_command_id(command, |command|match command{SemioMeshEditCommand::SetVertex(_)=>"set-vertex",SemioMeshEditCommand::SetActiveExample{..}=>semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID})
    }

    fn reference_birth()->Option<fn()->Result<SemioMeshSnapshot,semio_plugin::Fault>>{Some(||{
        Ok(SemioMeshSnapshot::default())
    })}

    fn handle(
        command: &Self::Command,
        doc: &ArtifactView<'_, Self::Snapshot>,
        _cfg: &ConfigView<'_, Self::Config>,
        _interaction: &InteractionView<'_>,
        _view_state: Option<&semio_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<Self::Mutation, Self::ConfigMutation, Self::DraftMutation>, Fault> {
        if let editing::SnapshotEditingCommand::Native(SemioMeshEditCommand::SetActiveExample{example_id})=command{return Ok(Emit{effects:vec![semio_s_artifact_stdio_contract::load_example_effect(&semio_mesh_editor_example_snapshot(example_id)?,SEMIO_MESH_DOCUMENT_SCHEMA)],..Default::default()})}
        let editing::SnapshotEditingCommand::Native(SemioMeshEditCommand::SetVertex(args)) = command else {
            let editing::SnapshotEditingCommand::Edit(event) = command else { unreachable!() };
            return <Self as editing::SnapshotEditingEditor>::snapshot_edit_emit(event, doc.snapshot);
        };
        Ok(match move_vertex_mutation(doc.snapshot, args)? {
            Some(mutation) => Emit::mutations(vec![mutation]),
            None => Emit::default(),
        })
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, view_state: &semio_plugin::ViewModel) -> semio_plugin::UiAssemblyResult<semio_plugin::ComponentTree> {
        match body_key {
            main::BODY_KEY => main::render(doc.snapshot).map(semio_plugin::built_to_component_tree),
            editing::SNAPSHOT_DETAILS_BODY_KEY => editing::render_snapshot_details(doc, view_state.locale, "s.stdio.semio@v1/mesh#editor", &semio_plugin::TreeWindows::for_body(view_state, editing::SNAPSHOT_DETAILS_BODY_KEY))
                .map(semio_plugin::built_to_component_tree),
            _ => semio_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
        }
    }

    fn command_from_action(action: &str, args: &mut Option<DslValue>, original_fault: &mut Option<Fault>) -> Result<Self::Command, Fault> {
        if let Some(fault) = original_fault.take() { return Err(fault); }
        let args = args.as_ref();
        editing::snapshot_editing_command_from_action(action, args, |action, args| {
            if action==semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID{return Ok(SemioMeshEditCommand::SetActiveExample{example_id:semio_s_artifact_stdio_contract::example_id_argument(args,"")})}
            if action != "set-vertex" {
                return Err(Fault::new(semio_plugin::FaultOrigin::App, semio_plugin::FaultCode::new("app.command.unsupported"), format!("action '{action}' is not supported by SemioMeshEditor")));
            }
            let field = |key: &str| args.and_then(|value| value.get(key)).ok_or_else(|| Fault::from(format!("set-vertex requires '{key}'")));
            let mesh_id = field("meshId")?.as_str().filter(|value| !value.is_empty()).ok_or_else(|| Fault::from("set-vertex meshId must be non-empty text"))?.to_owned();
            let primitive_id = field("primitiveId")?.as_str().filter(|value| !value.is_empty()).ok_or_else(|| Fault::from("set-vertex primitiveId must be non-empty text"))?.to_owned();
            let vertex_number =
                field("vertexIndex")?.as_f64().filter(|value| value.is_finite() && *value >= 0.0 && value.fract() == 0.0 && *value <= usize::MAX as f64).ok_or_else(|| Fault::from("set-vertex vertexIndex must be a non-negative integer"))?;
            let point_value = field("point")?.as_array().filter(|value| value.len() == 3).ok_or_else(|| Fault::from("set-vertex point must contain three finite numbers"))?;
            let component = |index: usize| point_value[index].as_f64().filter(|value| value.is_finite()).ok_or_else(|| Fault::from("set-vertex point must contain three finite numbers"));
            Ok(SemioMeshEditCommand::SetVertex(SemioMeshSetVertexArgs { mesh_id, primitive_id, vertex_index: vertex_number as usize, point: [component(0)?, component(1)?, component(2)?] }))
        })
    }
}

impl editing::SnapshotEditingEditor for SemioMeshEditor {
    fn snapshot_edit_event(command: &Self::Command) -> Option<&editing::SnapshotEditEvent> {
        match command {
            editing::SnapshotEditingCommand::Edit(event) => Some(event),
            _ => None,
        }
    }

    fn snapshot_edit_rules() -> &'static editing::EditRules {
        &edit_rules::EDIT_RULES
    }
    fn snapshot_edit_special(event: &editing::SnapshotEditEvent, snapshot: &Self::Snapshot) -> Result<Option<Vec<Self::Mutation>>, Fault> {
        edit_rules::special(event, snapshot)
    }
}

semio_s_artifact_stdio_contract::bounded_native_editing_editor! {
    editor: SemioMeshEditor,
    tools: ["set-vertex"],
    payload_schema: "stdio.semio.mesh.tool-command.v1",
    reduce: |command, snapshot| {
        if let editing::SnapshotEditingCommand::Native(SemioMeshEditCommand::SetActiveExample{example_id})=command{return Ok(Emit{effects:vec![semio_s_artifact_stdio_contract::load_example_effect(&semio_mesh_editor_example_snapshot(example_id)?,SEMIO_MESH_DOCUMENT_SCHEMA)],..Default::default()})}

        let editing::SnapshotEditingCommand::Native(SemioMeshEditCommand::SetVertex(args)) = command else {
            return Err(Fault::from("stdio.semio.mesh.set-vertex.command-mismatch"));
        };
        Ok(match move_vertex_mutation(snapshot, args)? {
            Some(mutation) => Emit::mutations(vec![mutation]),
            None => Emit::default(),
        })
    },
    host_tools: ["setActiveExample"],
    work: semio_mesh_set_vertex_work,
    preparation_route: preparation::route,
}
//#endregion 🔖️Editor

//#region 🔖️Manifest
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn create_semio_mesh_editor() -> semio_plugin::AppDefinition {
    let builder = Editor::builder(SEMIO_MESH_DIALECT)
        .document(["stdio", "semio"])
        .icon_id("box")
        .mode_def(edit::definition())
        .default_mode_id(edit::SEMIO_MESH_EDIT_MODE_ID)
        .window_kind_def(main::definition())
        .window_kind_def(editing::snapshot_details_window_definition())
        .default_layout(edit::layout())
        .action_with(set_vertex_action())
        .action_with(semio_s_artifact_stdio_contract::set_active_example_action())
        .action_args(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID,semio_s_artifact_stdio_contract::set_active_example_args(&[(semio_s_artifact_stdio_semio_mesh_examples::examples::cube::ID,semio_s_artifact_stdio_semio_mesh_examples::examples::cube::label())],semio_s_artifact_stdio_semio_mesh_examples::examples::cube::ID))
        .action_destructive(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID)
        .action_describe(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID,semio_s_artifact_stdio_contract::set_active_example_description())
        .action_interactive_job(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID,semio_plugin::InteractiveJobClassification::Migrated);
    editing::snapshot_edit_actions_with(builder).build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

#[cfg(test)]
#[test]
fn typed_example_selection_refuses_unknown_and_malformed_sources(){
    for id in ["","unknown-example","demo"]{assert!(semio_mesh_editor_example_snapshot(id).is_err());}
    for source in ["","not a semio document"]{assert!(semio_s_artifact_stdio_contract::parse_example_snapshot::<SemioMeshSnapshot>(semio_s_artifact_stdio_semio_mesh_examples::examples::cube::ID,semio_s_artifact_stdio_semio_mesh_examples::examples::cube::ID,source,SEMIO_MESH_DOCUMENT_SCHEMA,|snapshot|snapshot.schema.as_str()).is_err());}
}

#[cfg(test)]
#[test]
fn example_picker_declares_only_the_host_load_lane(){
    let contract=<SemioMeshEditor as editing::BoundedNativeEditingEditor>::NATIVE_PUBLICATION_CONTRACTS.iter().find(|contract|contract.tool_id==semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID).unwrap();
    assert_eq!(contract.lanes,&[semio_plugin::ArtifactToolPublicationLane::HostOnly]);
}

