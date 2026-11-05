//! ✏️ Semio Brep editor — thin, kit-based editor surface (ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.1). `SemioBrepEditor`
//! implements `ArtifactEditor`, wiring the shared `MeshWindowKit` to a single Main window.

use crate::editor::modes::edit;
use crate::editor::modes::edit::windows::main;
use semio_s_artifact_stdio_semio_base::schema::geometry::SemioPoint3;
use crate::schema::mutations::move_vertex::MoveVertex;
use crate::schema::mutations::SemioBrepMutation;
use crate::schema::snapshot::SemioBrepSnapshot;
use semio_framework::DslValue;
use semio_plugin::app::InteractionView;
use semio_plugin::retained_command::ArtifactCommandInputs;
use semio_plugin::retained_command::ArtifactCommandWork;
use semio_plugin::retained_command::ArtifactCommandWorkStep;
use semio_plugin::ActionArgDef;
use semio_plugin::ActionDefinition;
use semio_plugin::ActionKind;
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
pub const SEMIO_BREP_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("brep") };
pub const SEMIO_BREP_DOCUMENT_SCHEMA: &str = "stdio.semio.brep";
//#endregion 🔖️Dialect

//#region 🔖️Command
/// ✏️ `set-vertex`: real, wired end to end (ticket 26/09/03/BREP-KERNEL-DEPENDENCY-FREE-RUNTIME
/// wave W3-A) — `🧬️schema/🧬️mutations/📍move-vertex` HAS a real by-id reposition op
/// (`SemioBrepMutation::MoveVertex { vertex_id, new_point }`), so the old "no by-index replace op
/// exists, report don't invent" rationale no longer applies (it applied to `BrepLoopEdge`-shaped
/// by-INDEX addressing; `move-vertex` addresses by persistent-label id instead, which the
/// selection channel already resolves). The action carries an explicit stable vertex id and target
/// point, so keyboard and automation callers use the same validated address.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
pub struct SemioBrepSetVertexArgs {
    pub vertex_id: String,
    pub point: [f64; 3],
}

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
pub enum SemioBrepEditCommand {
    SetActiveExample { example_id: String },
    SetVertex(SemioBrepSetVertexArgs),
}

impl protocol::OpBinary for SemioBrepEditCommand {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> { Ok(semio_framework_pack_json::to_json_string(self).into_bytes()) }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let value=semio_framework_pack_json::parse_bytes(bytes,semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error|protocol::ProtocolError::Malformed{what:"SemioBrepEditCommand",offset:0,detail:error.to_string()})?;
        <Self as semio_framework_value::FromValue>::from_value(semio_framework_pack_json::to_dsl_value(&value)).map_err(|error|protocol::ProtocolError::Malformed{what:"SemioBrepEditCommand",offset:0,detail:error.to_string()})
    }
}
semio_s_artifact_stdio_contract::snapshot_editing_command_roster!(SemioBrepEditCommand, ["set-vertex", "setActiveExample"]);

fn semio_brep_editor_example_snapshot(example_id:&str)->Result<SemioBrepSnapshot,Fault>{
    semio_s_artifact_stdio_contract::parse_example_snapshot(example_id,semio_s_artifact_stdio_semio_brep_examples::examples::solid::ID,semio_s_artifact_stdio_semio_brep_examples::examples::solid::PRIMARY_TEXT,SEMIO_BREP_DOCUMENT_SCHEMA,|snapshot:&SemioBrepSnapshot|snapshot.schema.as_str())
}

pub fn set_vertex_action() -> ActionDefinition {
    let mut action = ActionDefinition::bounded_catalog("set-vertex", LocalizedLabel::native("Move Vertex", "Vertex verschieben"), ActionKind::Mutation)
        .with_args(vec![ActionArgDef::text("vertexId", LocalizedLabel::native("Vertex ID", "Vertex-ID")).required(), ActionArgDef::vector("point", LocalizedLabel::native("Target Point", "Zielpunkt"), 3).required()]);
    action.semantics.execution.interactive_job = semio_plugin::InteractiveJobClassification::Migrated;
    action
}
//#endregion 🔖️Command

//#region 🔖️Mutation
/// ✏️ Pure core of `set-vertex`: given the explicit stable vertex id and target point, the mutation to emit —
/// `None` if the id doesn't name a live vertex in `snapshot` (matches `handle`'s own no-op-on-
/// stale-selection behavior). Factored out of `handle` so it is unit-testable without needing a
/// full `InteractionView` (whose fields are private outside the plugin crate — this is the
/// dispatch decision `handle` delegates to, tested directly below).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn move_vertex_mutation(snapshot: &SemioBrepSnapshot, vertex_id: &str, point: [f64; 3]) -> Result<Option<SemioBrepMutation>, Fault> {
    let mut matched = None;
    for (index, _vertex) in snapshot.vertices.iter().enumerate().filter(|(_, vertex)| vertex.id == vertex_id) {
        if matched.replace(index).is_some() {
            return Err(Fault::from(format!("vertex id '{vertex_id}' is ambiguous")));
        }
    }
    let index = matched.ok_or_else(|| Fault::from(format!("vertex '{vertex_id}' does not exist")))?;
    let new_point = SemioPoint3 { x: point[0], y: point[1], z: point[2] };
    if snapshot.vertices[index].point == new_point {
        return Ok(None);
    }
    Ok(Some(SemioBrepMutation::MoveVertex(MoveVertex { vertex_id: vertex_id.to_string(), new_point })))
}

#[derive(Default)]
struct SemioBrepSetVertexWork {
    cursor: usize,
    matched_index: Option<usize>,
    complete: bool,
}

impl ArtifactCommandWork<EditorApp<SemioBrepEditor>> for SemioBrepSetVertexWork {
    fn tool_id(&self) -> &'static str {
        "set-vertex"
    }

    fn extent(
        &self,
        command: &editing::SnapshotEditingCommand<SemioBrepEditCommand>,
        _snapshot: &SemioBrepSnapshot,
        _interaction: &protocol::InteractionState,
        _context: Option<&semio_plugin::app::ArtifactOwnedToolJobContext<EditorApp<SemioBrepEditor>>>,
    ) -> Option<usize> {
        let editing::SnapshotEditingCommand::Native(SemioBrepEditCommand::SetVertex(args)) = command else { return None };
        (!args.vertex_id.is_empty() && args.point.iter().all(|value| value.is_finite())).then_some(2)
    }

    fn work_demands(&self, input: &ArtifactCommandInputs<'_, EditorApp<SemioBrepEditor>>, _maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        let editing::SnapshotEditingCommand::Native(SemioBrepEditCommand::SetVertex(args)) = input.command else {
            return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "stdio.semio.brep.set-vertex.command-mismatch"));
        };
        Ok(semio_framework_value::RetirementDemand { copy_bytes: args.vertex_id.len(), capacity_bytes: args.vertex_id.len().saturating_add(size_of::<SemioBrepMutation>()), release_bytes: 0, depth: 1 })
    }

    fn step(&mut self, input: &ArtifactCommandInputs<'_, EditorApp<SemioBrepEditor>>, _cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactCommandWorkStep<EditorApp<SemioBrepEditor>>, semio_framework_job::OriginalTurnFailure> {
        let original: Result<_, Fault> = (|| {
        if self.complete {
            return Err(Fault::from("stdio.semio.brep.set-vertex.work-closed"));
        }
        let editing::SnapshotEditingCommand::Native(SemioBrepEditCommand::SetVertex(args)) = input.command else {
            return Err(Fault::from("stdio.semio.brep.set-vertex.command-mismatch"));
        };
        if let Some(vertex) = input.snapshot.vertices.get(self.cursor) {
            let index = self.cursor;
            self.cursor += 1;
            if vertex.id == args.vertex_id && self.matched_index.replace(index).is_some() {
                return Err(Fault::from(format!("vertex id '{}' is ambiguous", args.vertex_id)));
            }
            return Ok(ArtifactCommandWorkStep::Replay { stage: "semio-brep-resolve-vertex", preview: br#"{"en":"Checking vertices","de":"Vertices werden gepr\u00fcft"}"# });
        }
        let index = self.matched_index.ok_or_else(|| Fault::from(format!("vertex '{}' does not exist", args.vertex_id)))?;
        let vertex = &input.snapshot.vertices[index];
        self.complete = true;
        let point = SemioPoint3 { x: args.point[0], y: args.point[1], z: args.point[2] };
        if vertex.point == point {
            return Ok(ArtifactCommandWorkStep::Complete(Emit::default()));
        }
        Ok(ArtifactCommandWorkStep::Complete(Emit::mutations(vec![SemioBrepMutation::MoveVertex(MoveVertex { vertex_id: args.vertex_id.clone(), new_point: point })])))
    
        })();
        original.map_err(semio_framework_job::OriginalTurnFailure::from)
    }

    fn begin_close(&mut self) {
        self.complete = true;
    }
}

fn semio_brep_set_vertex_work(tool_id: &'static str) -> Box<dyn ArtifactCommandWork<EditorApp<SemioBrepEditor>>> {
    if tool_id==semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID{return editing::bounded_native_edit_work::<SemioBrepEditor>(tool_id)}
    Box::new(SemioBrepSetVertexWork::default())
}
//#endregion 🔖️Mutation

pub use semio_s_artifact_stdio_semio_brep_viewer::editor::edit_rules;

//#region 🔖️Editor
#[derive(Default, Clone, Copy)]
pub struct SemioBrepEditor;

impl ArtifactEditor for SemioBrepEditor {
    fn examples()->Vec<semio_framework_plugin::ExampleSource>{vec![semio_s_artifact_stdio_semio_brep_examples::examples::solid::source()]}

    type Snapshot = SemioBrepSnapshot;
    type Mutation = SemioBrepMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = editing::SnapshotEditingCommand<SemioBrepEditCommand>;

    const DIALECT: Dialect = SEMIO_BREP_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = SEMIO_BREP_DOCUMENT_SCHEMA;

    semio_s_artifact_stdio_semio_base::semio_natural_file!(SemioBrepSnapshot, "stdio.semio.brep");

    semio_s_artifact_stdio_contract::snapshot_details_editor_support! {
        owner_file: "semio/v1/brep/editor",
        controller: "s.stdio.semio@v1/brep#editor",
        artifact_schema: "stdio.semio.brep",
        preparation: "stdio-semio-brep-snapshot-edit",
        bounded_native: true
    }

    fn command_id(command: &Self::Command) -> &'static str {
        editing::snapshot_editing_command_id(command, |command|match command{SemioBrepEditCommand::SetVertex(_)=>"set-vertex",SemioBrepEditCommand::SetActiveExample{..}=>semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID})
    }

    fn reference_birth()->Option<fn()->Result<SemioBrepSnapshot,semio_plugin::Fault>>{Some(||{
        Ok(SemioBrepSnapshot::default())
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
        if let editing::SnapshotEditingCommand::Native(SemioBrepEditCommand::SetActiveExample{example_id})=command{return Ok(Emit{effects:vec![semio_s_artifact_stdio_contract::load_example_effect(&semio_brep_editor_example_snapshot(example_id)?,SEMIO_BREP_DOCUMENT_SCHEMA)],..Default::default()})}
        let editing::SnapshotEditingCommand::Native(SemioBrepEditCommand::SetVertex(args)) = command else {
            let editing::SnapshotEditingCommand::Edit(event) = command else { unreachable!() };
            return <Self as editing::SnapshotEditingEditor>::snapshot_edit_emit(event, doc.snapshot);
        };
        Ok(match move_vertex_mutation(doc.snapshot, &args.vertex_id, args.point)? {
            Some(mutation) => Emit::mutations(vec![mutation]),
            None => Emit::default(),
        })
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, view_state: &semio_plugin::ViewModel) -> semio_plugin::UiAssemblyResult<semio_plugin::ComponentTree> {
        match body_key {
            main::BODY_KEY => main::render(doc.snapshot).map(semio_plugin::built_to_component_tree),
            editing::SNAPSHOT_DETAILS_BODY_KEY => editing::render_snapshot_details(doc, view_state.locale, "s.stdio.semio@v1/brep#editor", &semio_plugin::TreeWindows::for_body(view_state, editing::SNAPSHOT_DETAILS_BODY_KEY))
                .map(semio_plugin::built_to_component_tree),
            _ => semio_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
        }
    }

    fn command_from_action(action: &str, args: &mut Option<DslValue>, original_fault: &mut Option<Fault>) -> Result<Self::Command, Fault> {
        if let Some(fault) = original_fault.take() { return Err(fault); }
        let args = args.as_ref();
        editing::snapshot_editing_command_from_action(action, args, |action, args| {
            if action==semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID{return Ok(SemioBrepEditCommand::SetActiveExample{example_id:semio_s_artifact_stdio_contract::example_id_argument(args,"")})}
            if action != "set-vertex" {
                return Err(Fault::new(semio_plugin::FaultOrigin::App, semio_plugin::FaultCode::new("app.command.unsupported"), format!("action '{action}' is not supported by SemioBrepEditor")));
            }
            let field = |key: &str| args.and_then(|value| value.get(key)).ok_or_else(|| Fault::from(format!("set-vertex requires '{key}'")));
            let vertex_id = field("vertexId")?.as_str().filter(|value| !value.is_empty()).ok_or_else(|| Fault::from("set-vertex vertexId must be non-empty text"))?.to_owned();
            let point_value = field("point")?.as_array().filter(|value| value.len() == 3).ok_or_else(|| Fault::from("set-vertex point must contain three finite numbers"))?;
            let component = |index: usize| point_value[index].as_f64().filter(|value| value.is_finite()).ok_or_else(|| Fault::from("set-vertex point must contain three finite numbers"));
            Ok(SemioBrepEditCommand::SetVertex(SemioBrepSetVertexArgs { vertex_id, point: [component(0)?, component(1)?, component(2)?] }))
        })
    }
}

impl editing::SnapshotEditingEditor for SemioBrepEditor {
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
    editor: SemioBrepEditor,
    tools: ["set-vertex"],
    payload_schema: "stdio.semio.brep.tool-command.v1",
    reduce: |command, snapshot| {
        if let editing::SnapshotEditingCommand::Native(SemioBrepEditCommand::SetActiveExample{example_id})=command{return Ok(Emit{effects:vec![semio_s_artifact_stdio_contract::load_example_effect(&semio_brep_editor_example_snapshot(example_id)?,SEMIO_BREP_DOCUMENT_SCHEMA)],..Default::default()})}

        let editing::SnapshotEditingCommand::Native(SemioBrepEditCommand::SetVertex(args)) = command else {
            return Err(Fault::from("stdio.semio.brep.set-vertex.command-mismatch"));
        };
        Ok(match move_vertex_mutation(snapshot, &args.vertex_id, args.point)? {
            Some(mutation) => Emit::mutations(vec![mutation]),
            None => Emit::default(),
        })
    },
    host_tools: ["setActiveExample"],
    work: semio_brep_set_vertex_work,
    preparation_route: preparation::route,
}
//#endregion 🔖️Editor

//#region 🔖️Manifest
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn create_semio_brep_editor() -> semio_plugin::AppDefinition {
    let builder = Editor::builder(SEMIO_BREP_DIALECT)
        .document(["stdio", "semio"])
        .icon_id("box")
        .mode_def(edit::definition())
        .default_mode_id(edit::SEMIO_BREP_EDIT_MODE_ID)
        .window_kind_def(main::definition())
        .window_kind_def(editing::snapshot_details_window_definition())
        .default_layout(edit::layout())
        .action_with(set_vertex_action())
        .action_with(semio_s_artifact_stdio_contract::set_active_example_action())
        .action_args(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID,semio_s_artifact_stdio_contract::set_active_example_args(&[(semio_s_artifact_stdio_semio_brep_examples::examples::solid::ID,semio_s_artifact_stdio_semio_brep_examples::examples::solid::label())],semio_s_artifact_stdio_semio_brep_examples::examples::solid::ID))
        .action_destructive(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID)
        .action_describe(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID,semio_s_artifact_stdio_contract::set_active_example_description())
        .action_interactive_job(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID,semio_framework_plugin::InteractiveJobClassification::Migrated);
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
    for id in ["","unknown-example","demo"]{assert!(semio_brep_editor_example_snapshot(id).is_err());}
    for source in ["","not a semio document"]{assert!(semio_s_artifact_stdio_contract::parse_example_snapshot::<SemioBrepSnapshot>(semio_s_artifact_stdio_semio_brep_examples::examples::solid::ID,semio_s_artifact_stdio_semio_brep_examples::examples::solid::ID,source,SEMIO_BREP_DOCUMENT_SCHEMA,|snapshot|snapshot.schema.as_str()).is_err());}
}

#[cfg(test)]
#[test]
fn example_picker_declares_only_the_host_load_lane(){
    let contract=<SemioBrepEditor as editing::BoundedNativeEditingEditor>::NATIVE_PUBLICATION_CONTRACTS.iter().find(|contract|contract.tool_id==semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID).unwrap();
    assert_eq!(contract.lanes,&[semio_framework_plugin::ArtifactToolPublicationLane::HostOnly]);
}
