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
    semio_framework_plugin::testkit::assert_editor_and_viewer_share_dialect::<SemioBrepEditor, crate::viewer::semio_brep::SemioBrepViewer>().await;
}

//#region 🧪️SetVertex
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn one_vertex_snapshot() -> SemioBrepSnapshot {
    let mut s = SemioBrepSnapshot::default();
    s.vertices = vec![crate::standards::v1::subsets::brep::schema::snapshot::BrepVertex { id: "v1".into(), point: SemioPoint3 { x: 0.0, y: 0.0, z: 0.0 }, tol: 0.0 }];
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
    let back = <SemioBrepEditCommand as protocol::OpBinary>::decode_op(&bytes).expect("decode");
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
