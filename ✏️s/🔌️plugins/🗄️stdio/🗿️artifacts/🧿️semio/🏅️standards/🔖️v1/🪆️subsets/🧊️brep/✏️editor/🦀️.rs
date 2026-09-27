//! ✏️ Semio Brep editor — thin, kit-based editor surface (ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.1). `SemioBrepEditor`
//! implements `ArtifactEditor`, wiring the shared `MeshWindowKit` to a single Main window.

use crate::editor::semio_brep::modes::edit;
use crate::editor::semio_brep::modes::edit::windows::main;
use crate::standards::v1::subsets::base::schema::geometry::SemioPoint3;
use crate::standards::v1::subsets::brep::schema::mutations::move_vertex::MoveVertex;
use crate::standards::v1::subsets::brep::schema::mutations::{set_snapshot, SemioBrepMutation};
use crate::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot;
use semio_framework::DslValue;
use semio_framework_plugin::app::InteractionView;
use semio_framework_plugin::{
    ActionArgDef, ActionDefinition, ActionKind, ArtifactEditor, ArtifactView, ConfigView, Dialect, DraftView, Editor, Emit, Fault, Label, LocalizedLabel, NoConfig, NoConfigMutation, NoDraft, NoDraftMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, StandardId, SubsetId,
};
use store::EngineHandles;
use semio_s_artifact_stdio_contract::editing;

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
#[derive(Clone, Debug, PartialEq)]
pub struct SemioBrepSetVertexArgs {
    pub vertex_id: String,
    pub point: [f64; 3],
}

#[derive(Clone, Debug, PartialEq)]
pub enum SemioBrepEditCommand {
    SetVertex(SemioBrepSetVertexArgs),
}

impl protocol::OpBinary for SemioBrepEditCommand {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let SemioBrepEditCommand::SetVertex(args) = self;
        let id = args.vertex_id.as_bytes();
        let length = u32::try_from(id.len()).map_err(|_| protocol::ProtocolError::Malformed { what: "set-vertex op", offset: 0, detail: "vertexId is too long".into() })?;
        let mut out = Vec::with_capacity(4 + id.len() + 24);
        out.extend_from_slice(&length.to_le_bytes());
        out.extend_from_slice(id);
        for component in args.point {
            out.extend_from_slice(&component.to_le_bytes());
        }
        Ok(out)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        if bytes.len() < 28 {
            return Err(protocol::ProtocolError::Malformed { what: "set-vertex op", offset: 0, detail: "missing vertexId or point".into() });
        }
        let length = u32::from_le_bytes(bytes[..4].try_into().expect("four-byte slice")) as usize;
        if length == 0 || bytes.len() != 4 + length + 24 {
            return Err(protocol::ProtocolError::Malformed { what: "set-vertex op", offset: 0, detail: "invalid vertexId length".into() });
        }
        let vertex_id = std::str::from_utf8(&bytes[4..4 + length]).map_err(|error| protocol::ProtocolError::Malformed { what: "set-vertex op", offset: 4, detail: error.to_string() })?.to_owned();
        let point_bytes = &bytes[4 + length..];
        let read = |i: usize| f64::from_le_bytes(point_bytes[i * 8..i * 8 + 8].try_into().expect("eight-byte slice"));
        let point = [read(0), read(1), read(2)];
        if point.iter().any(|value| !value.is_finite()) {
            return Err(protocol::ProtocolError::Malformed { what: "set-vertex op", offset: 4 + length, detail: "point must contain finite numbers".into() });
        }
        Ok(SemioBrepEditCommand::SetVertex(SemioBrepSetVertexArgs { vertex_id, point }))
    }
}
semio_s_artifact_stdio_contract::snapshot_editing_command_roster!(SemioBrepEditCommand, ["set-vertex"]);

pub fn set_vertex_action() -> ActionDefinition {
    ActionDefinition::bounded_catalog("set-vertex", LocalizedLabel::native("Move Vertex", "Vertex verschieben"), ActionKind::Mutation).with_args(vec![
        ActionArgDef::text("vertexId", LocalizedLabel::native("Vertex ID", "Vertex-ID")).required(),
        ActionArgDef::vec3("point", LocalizedLabel::native("Target Point", "Zielpunkt")).required(),
    ])
}
//#endregion 🔖️Command

//#region 🔖️Mutation
/// ✏️ Pure core of `set-vertex`: given the explicit stable vertex id and target point, the mutation to emit —
/// `None` if the id doesn't name a live vertex in `snapshot` (matches `handle`'s own no-op-on-
/// stale-selection behavior). Factored out of `handle` so it is unit-testable without needing a
/// full `InteractionView` (whose fields are private outside the plugin crate — this is the
/// dispatch decision `handle` delegates to, tested directly below).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn move_vertex_mutation(snapshot: &SemioBrepSnapshot, vertex_id: &str, point: [f64; 3]) -> Option<SemioBrepMutation> {
    if !snapshot.vertices.iter().any(|vertex| vertex.id == vertex_id) {
        return None;
    }
    Some(SemioBrepMutation::MoveVertex(MoveVertex { vertex_id: vertex_id.to_string(), new_point: SemioPoint3 { x: point[0], y: point[1], z: point[2] } }))
}
//#endregion 🔖️Mutation

//#region 🔖️Editor
#[derive(Default, Clone, Copy)]
pub struct SemioBrepEditor;

impl ArtifactEditor for SemioBrepEditor {
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

    semio_s_artifact_stdio_contract::snapshot_details_editor_support! {
        owner_file: "semio/v1/brep/editor",
        controller: "s.stdio.semio@v1/brep#editor",
        artifact_schema: "stdio.semio.brep",
        preparation: "stdio-semio-brep-snapshot-edit"
    }

    fn command_id(command: &Self::Command) -> &'static str {
        editing::snapshot_editing_command_id(command, |_| "set-vertex")
    }

    fn initial_snapshot() -> SemioBrepSnapshot {
        SemioBrepSnapshot::default()
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
        let editing::SnapshotEditingCommand::Native(SemioBrepEditCommand::SetVertex(args)) = command else {
            let editing::SnapshotEditingCommand::Edit(event) = command else { unreachable!() };
            return <Self as editing::SnapshotEditingEditor>::snapshot_edit_emit(event, doc.snapshot);
        };
        match move_vertex_mutation(doc.snapshot, &args.vertex_id, args.point) {
            Some(mutation) => Ok(Emit::mutations(vec![mutation])),
            None => Err(Fault::from(format!("vertex '{}' does not exist", args.vertex_id))),
        }
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        match body_key {
            main::BODY_KEY => main::render(doc.snapshot).map(semio_framework_plugin::built_to_component_tree),
            editing::SNAPSHOT_DETAILS_BODY_KEY => editing::render_snapshot_details(doc.snapshot, view_state.locale, "s.stdio.semio@v1/brep#editor", &semio_framework_plugin::TreeWindows::for_body(view_state, editing::SNAPSHOT_DETAILS_BODY_KEY)).map(semio_framework_plugin::built_to_component_tree),
            _ => semio_framework_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
        }
    }

    fn command_from_action(action: &str, args: Option<&DslValue>) -> Result<Self::Command, Fault> {
        editing::snapshot_editing_command_from_action(action, args, |action, args| {
            if action != "set-vertex" {
                return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("app.command.unsupported"), format!("action '{action}' is not supported by SemioBrepEditor")));
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
        match command { editing::SnapshotEditingCommand::Edit(event) => Some(event), _ => None }
    }


    fn snapshot_edit_mutations(event: &editing::SnapshotEditEvent, snapshot: &Self::Snapshot) -> Result<Emit<Self::Mutation, Self::ConfigMutation, Self::DraftMutation>, Fault> {
        editing::snapshot_edit_set_snapshot(event, snapshot, |snapshot| SemioBrepMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }))
    }
}
//#endregion 🔖️Editor

//#region 🔖️Manifest
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn create_semio_brep_editor() -> semio_framework_plugin::AppDefinition {
    let builder = Editor::builder(SEMIO_BREP_DIALECT).document(["stdio", "semio"]).icon_id("box").mode_def(edit::definition()).default_mode_id(edit::SEMIO_BREP_EDIT_MODE_ID).window_kind_def(main::definition()).window_kind_def(editing::snapshot_details_window_definition()).default_layout(edit::layout()).action_with(set_vertex_action());
    editing::snapshot_edit_actions_with(builder).build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
