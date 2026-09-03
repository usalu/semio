//! ✏️ Semio Brep editor — thin, kit-based editor surface (ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.1). `SemioBrepEditor`
//! implements `ArtifactEditor`, wiring the shared `MeshWindowKit` to a single Main window.

use crate::artifacts::semio::standards::v1::subsets::base::schema::geometry::SemioPoint3;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::mutations::move_vertex::MoveVertex;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::mutations::SemioBrepMutation;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot;
use crate::editor::semio_brep::modes::edit;
use crate::editor::semio_brep::modes::edit::windows::main;
use semio_framework::DslValue;
use semio_framework_plugin::app::InteractionView;
use semio_framework_plugin::{
    ArtifactEditor, ArtifactView, ConfigView, Dialect, DraftView, Editor, Emit, Fault, Label, NoConfig, NoConfigMutation, NoDraft, NoDraftMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, StandardId, SubsetId,
};
use store::EngineHandles;

//#region 🔖️Dialect
pub const SEMIO_BREP_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("brep") };
pub const SEMIO_BREP_DOCUMENT_SCHEMA: &str = "stdio.semio.brep";
/// 🕹️ Selection domain this editor reads picked vertex/edge/face ids from — one domain covering
/// every entity kind (matching `process3d`'s own single-domain-per-artifact convention), never a
/// per-kind split, since a `MeshWindowKit` scene has one shared picking channel.
pub const SEMIO_BREP_INTERACTION_DOMAIN: &str = "brep";
//#endregion 🔖️Dialect

//#region 🔖️Command
/// ✏️ `set-vertex`: real, wired end to end (ticket 26/09/03/BREP-KERNEL-DEPENDENCY-FREE-RUNTIME
/// wave W3-A) — `🧬️schema/🧬️mutations/📍move-vertex` HAS a real by-id reposition op
/// (`SemioBrepMutation::MoveVertex { vertex_id, new_point }`), so the old "no by-index replace op
/// exists, report don't invent" rationale no longer applies (it applied to `BrepLoopEdge`-shaped
/// by-INDEX addressing; `move-vertex` addresses by persistent-label id instead, which the
/// selection channel already resolves). The vertex id comes from the current selection (contract
/// §2.6's own "set-vertex" example — the picked vertex, not a payload field); the target point
/// comes from the action's own payload.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SemioBrepSetVertexArgs {
    pub point: [f64; 3],
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SemioBrepEditCommand {
    SetVertex(SemioBrepSetVertexArgs),
}

impl protocol::OpBinary for SemioBrepEditCommand {
    /// 🧬️ Fixed 24-byte little-endian `[f64;3]` payload — no JSON/serde dependency needed for
    /// three floats (first-party, matches the repo's own "no runtime deps on external libraries"
    /// rule better than the sibling `✳️mesh` editor's `serde_json`-based encode).
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let SemioBrepEditCommand::SetVertex(args) = self;
        let mut out = Vec::with_capacity(24);
        for component in args.point {
            out.extend_from_slice(&component.to_le_bytes());
        }
        Ok(out)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        if bytes.len() != 24 {
            return Err(protocol::ProtocolError::Malformed { what: "set-vertex op", offset: 0, detail: format!("expected 24 bytes ([f64;3]), got {}", bytes.len()) });
        }
        let read = |i: usize| f64::from_le_bytes(bytes[i * 8..i * 8 + 8].try_into().expect("8-byte slice"));
        Ok(SemioBrepEditCommand::SetVertex(SemioBrepSetVertexArgs { point: [read(0), read(1), read(2)] }))
    }
}
//#endregion 🔖️Command

//#region 🔖️Mutation
/// ✏️ Pure core of `set-vertex`: given the already-resolved vertex id (from the current
/// selection — see `SEMIO_BREP_INTERACTION_DOMAIN`) and target point, the mutation to emit —
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
    type Command = SemioBrepEditCommand;

    const DIALECT: Dialect = SEMIO_BREP_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = SEMIO_BREP_DOCUMENT_SCHEMA;

    fn initial_snapshot() -> SemioBrepSnapshot {
        SemioBrepSnapshot::default()
    }

    fn handle(
        command: &Self::Command,
        doc: &ArtifactView<'_, Self::Snapshot>,
        _cfg: &ConfigView<'_, Self::Config>,
        interaction: &InteractionView<'_>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<Self::Mutation, Self::ConfigMutation, Self::DraftMutation>, Fault> {
        let SemioBrepEditCommand::SetVertex(args) = command;
        let selection = interaction.selection(SEMIO_BREP_INTERACTION_DOMAIN);
        let Some(vertex_id) = selection.ids.first() else { return Ok(Emit::default()) };
        match move_vertex_mutation(doc.snapshot, vertex_id, args.point) {
            Some(mutation) => Ok(Emit::mutations(vec![mutation])),
            None => Ok(Emit::default()),
        }
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        match body_key {
            main::BODY_KEY => main::render(doc.snapshot).map(semio_framework_plugin::built_to_component_tree),
            _ => return semio_framework_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
        }
    }

    fn command_from_action(action: &str, args: Option<&DslValue>) -> Result<Self::Command, Fault> {
        if action != "set-vertex" {
            return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("app.command.unsupported"), format!("action '{action}' is not supported by SemioBrepEditor")));
        }
        let point = args
            .and_then(|value| value.get("point"))
            .and_then(DslValue::as_array)
            .map(|array| {
                let get = |index: usize| array.get(index).and_then(DslValue::as_f64).unwrap_or(0.0);
                [get(0), get(1), get(2)]
            })
            .unwrap_or([0.0, 0.0, 0.0]);
        Ok(SemioBrepEditCommand::SetVertex(SemioBrepSetVertexArgs { point }))
    }
}
//#endregion 🔖️Editor

//#region 🔖️Manifest
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn create_semio_brep_editor() -> semio_framework_plugin::AppDefinition {
    Editor::builder(SEMIO_BREP_DIALECT).document(["stdio", "semio"]).icon_id("box").mode_def(edit::definition()).default_mode_id(edit::SEMIO_BREP_EDIT_MODE_ID).window_kind_def(main::definition()).default_layout(edit::layout()).build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn create_editor_builds_a_definition_for_the_editor_role() {
        let def = create_semio_brep_editor();
        assert_eq!(def.role, semio_framework_plugin::AppRole::Editor);
        assert_eq!(def.dialect, SEMIO_BREP_DIALECT.into());
    }

    #[semio_framework_async_macros::async_test]
    async fn editor_dialect_matches_the_artifact_coordinate() {
        assert_eq!(<SemioBrepEditor as ArtifactEditor>::DIALECT, SEMIO_BREP_DIALECT);
    }

    #[semio_framework_async_macros::async_test]
    async fn editor_and_viewer_share_one_dialect() {
        semio_framework_plugin::testkit::assert_editor_and_viewer_share_dialect::<SemioBrepEditor, crate::viewer::semio_brep::SemioBrepViewer>();
    }

    //#region 🧪️SetVertex
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn one_vertex_snapshot() -> SemioBrepSnapshot {
        let mut s = SemioBrepSnapshot::default();
        s.vertices = vec![crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::BrepVertex { id: "v1".into(), point: SemioPoint3 { x: 0.0, y: 0.0, z: 0.0 }, tol: 0.0 }];
        s
    }

    /// ✏️ `command_from_action("set-vertex", …)` parses the payload's `point` field — the same
    /// shape the shared `MeshWindowKit`'s `set-vertex` action carries.
    #[semio_framework_async_macros::async_test]
    async fn command_from_action_parses_the_point_payload() {
        let args = DslValue::Object(vec![("point".to_string(), DslValue::Array(vec![DslValue::float(1.0), DslValue::float(2.0), DslValue::float(3.0)]))]);
        let command = SemioBrepEditor::command_from_action("set-vertex", Some(&args)).expect("set-vertex is supported");
        assert_eq!(command, SemioBrepEditCommand::SetVertex(SemioBrepSetVertexArgs { point: [1.0, 2.0, 3.0] }));
    }

    #[semio_framework_async_macros::async_test]
    async fn command_from_action_rejects_unknown_actions() {
        assert!(SemioBrepEditor::command_from_action("delete-vertex", None).is_err());
    }

    /// ✏️ 24-byte little-endian `[f64;3]` `OpBinary` round trip.
    #[semio_framework_async_macros::async_test]
    async fn set_vertex_op_binary_round_trips() {
        let command = SemioBrepEditCommand::SetVertex(SemioBrepSetVertexArgs { point: [1.5, -2.25, 4.0] });
        let bytes = protocol::OpBinary::encode_op(&command).expect("encode");
        assert_eq!(bytes.len(), 24);
        let back = SemioBrepEditCommand::decode_op(&bytes).expect("decode");
        assert_eq!(back, command);
    }

    /// ✏️ Ticket goal: "dispatching the action yields the mutation" — `move_vertex_mutation` is
    /// `handle`'s exact dispatch core (see its own doc comment for why it's tested directly
    /// rather than through a synthesized `InteractionView`).
    #[semio_framework_async_macros::async_test]
    async fn dispatching_set_vertex_yields_a_move_vertex_mutation() {
        let snapshot = one_vertex_snapshot();
        let mutation = move_vertex_mutation(&snapshot, "v1", [5.0, 6.0, 7.0]).expect("v1 exists");
        assert_eq!(mutation, SemioBrepMutation::MoveVertex(MoveVertex { vertex_id: "v1".into(), new_point: SemioPoint3 { x: 5.0, y: 6.0, z: 7.0 } }));
    }

    #[semio_framework_async_macros::async_test]
    async fn dispatching_set_vertex_for_a_stale_selection_is_a_no_op() {
        let snapshot = one_vertex_snapshot();
        assert_eq!(move_vertex_mutation(&snapshot, "does-not-exist", [1.0, 1.0, 1.0]), None);
    }

    /// ✏️ Ticket goal: "applying it moves the vertex" — real `protocol::Mutation::diff`/
    /// `MutationDiff::apply`, the exact production `mutate()` path, not a hand-rolled shortcut.
    #[semio_framework_async_macros::async_test]
    async fn applying_the_move_vertex_mutation_actually_moves_the_vertex() {
        use protocol::{Mutation, MutationDiff};
        let snapshot = one_vertex_snapshot();
        let mutation = move_vertex_mutation(&snapshot, "v1", [9.0, 8.0, 7.0]).expect("v1 exists");
        let applied = mutation.diff(&snapshot).diff().apply(&snapshot).expect("apply succeeds for a well-formed fixture");
        assert_eq!(applied.vertices[0].point, SemioPoint3 { x: 9.0, y: 8.0, z: 7.0 });
    }
    //#endregion 🧪️SetVertex
}
//#endregion 🧪️Tests
