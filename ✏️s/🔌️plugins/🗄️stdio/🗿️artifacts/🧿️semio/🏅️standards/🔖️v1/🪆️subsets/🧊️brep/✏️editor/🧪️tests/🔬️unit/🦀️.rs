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
    semio_framework_plugin::artifact_app_laws::assert_editor_and_viewer_share_dialect::<SemioBrepEditor, crate::viewer::semio_brep::SemioBrepViewer>().await;
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
    let args = DslValue::Object(vec![("vertexId".to_string(), DslValue::String("v1".into())), ("point".to_string(), DslValue::Array(vec![DslValue::float(1.0), DslValue::float(2.0), DslValue::float(3.0)]))]);
    let command = SemioBrepEditor::command_from_action("set-vertex", Some(&args)).expect("set-vertex is supported");
    assert_eq!(command, editing::SnapshotEditingCommand::Native(SemioBrepEditCommand::SetVertex(SemioBrepSetVertexArgs { vertex_id: "v1".into(), point: [1.0, 2.0, 3.0] })));
    assert!(SemioBrepEditor::command_from_action("set-vertex", Some(&DslValue::Object(vec![]))).is_err());
}

#[semio_framework_async_macros::async_test]
async fn command_from_action_rejects_unknown_actions() {
    assert!(SemioBrepEditor::command_from_action("delete-vertex", None).is_err());
}

/// ✏️ Stable vertex id plus little-endian `[f64;3]` `OpBinary` round trip.
#[semio_framework_async_macros::async_test]
async fn set_vertex_op_binary_round_trips() {
    let command = SemioBrepEditCommand::SetVertex(SemioBrepSetVertexArgs { vertex_id: "v1".into(), point: [1.5, -2.25, 4.0] });
    let bytes = protocol::OpBinary::encode_op(&command).expect("encode");
    assert_eq!(bytes.len(), 30);
    let back = <SemioBrepEditCommand as protocol::OpBinary>::decode_op(&bytes).expect("decode");
    assert_eq!(back, command);
}

#[test]
fn set_vertex_op_binary_rejects_a_maximum_length_prefix_without_overflow() {
    let mut bytes = Vec::from(u32::MAX.to_le_bytes());
    bytes.extend_from_slice(&[0; 24]);
    assert!(<SemioBrepEditCommand as protocol::OpBinary>::decode_op(&bytes).is_err());
}

#[test]
fn encoded_set_vertex_admission_counts_the_target_id() {
    let command = editing::SnapshotEditingCommand::Native(SemioBrepEditCommand::SetVertex(SemioBrepSetVertexArgs { vertex_id: "vertex".repeat(43), point: [1.0, 2.0, 3.0] }));
    let encoded = protocol::OpBinary::encode_op(&command).expect("encode set-vertex");
    assert_eq!(editing::admit_bounded_native_command(&command, encoded.len()).expect("exact admission"), encoded.len());
    assert!(editing::admit_bounded_native_command(&command, encoded.len() - 1).is_err());
}

/// ✏️ Ticket goal: "dispatching the action yields the mutation" — `move_vertex_mutation` is
/// `handle`'s exact dispatch core (see its own doc comment for why it's tested directly
/// rather than through a synthesized `InteractionView`).
#[semio_framework_async_macros::async_test]
async fn dispatching_set_vertex_yields_a_move_vertex_mutation() {
    let snapshot = one_vertex_snapshot();
    let mutation = move_vertex_mutation(&snapshot, "v1", [5.0, 6.0, 7.0]).expect("unique target").expect("changed point");
    assert_eq!(mutation, SemioBrepMutation::MoveVertex(MoveVertex { vertex_id: "v1".into(), new_point: SemioPoint3 { x: 5.0, y: 6.0, z: 7.0 } }));
}

#[semio_framework_async_macros::async_test]
async fn dispatching_set_vertex_for_a_stale_selection_is_refused() {
    let snapshot = one_vertex_snapshot();
    assert!(move_vertex_mutation(&snapshot, "does-not-exist", [1.0, 1.0, 1.0]).is_err());
}

#[semio_framework_async_macros::async_test]
async fn dispatching_set_vertex_refuses_duplicate_vertex_ids() {
    let mut snapshot = one_vertex_snapshot();
    snapshot.vertices.push(snapshot.vertices[0].clone());
    assert!(move_vertex_mutation(&snapshot, "v1", [1.0, 1.0, 1.0]).expect_err("ambiguous target").to_string().contains("ambiguous"));
}

#[semio_framework_async_macros::async_test]
async fn set_vertex_is_visible_with_required_localized_arguments() {
    let definition = create_semio_brep_editor();
    let action = definition.actions.iter().find(|action| action.id == "set-vertex").expect("set-vertex action");
    assert_eq!(action.args.len(), 2);
    assert!(action.args.iter().all(|argument| argument.required));
    assert_eq!(action.label.resolve(semio_framework_plugin::Terminology::Native, semio_framework_plugin::Locale::En), "Move Vertex");
    assert_eq!(action.label.resolve(semio_framework_plugin::Terminology::Native, semio_framework_plugin::Locale::De), "Vertex verschieben");
}

/// ✏️ Ticket goal: "applying it moves the vertex" — real `protocol::Mutation::diff`/
/// `MutationDiff::apply`, the exact production `mutate()` path, not a hand-rolled shortcut.
#[semio_framework_async_macros::async_test]
async fn applying_the_move_vertex_mutation_actually_moves_the_vertex() {
    use protocol::{Mutation, MutationDiff};
    let snapshot = one_vertex_snapshot();
    let mutation = move_vertex_mutation(&snapshot, "v1", [9.0, 8.0, 7.0]).expect("unique target").expect("changed point");
    let applied = mutation.diff(&snapshot).diff().apply(&snapshot).expect("apply succeeds for a well-formed fixture");
    assert_eq!(applied.vertices[0].point, SemioPoint3 { x: 9.0, y: 8.0, z: 7.0 });
}

#[semio_framework_async_macros::async_test]
async fn registered_set_vertex_publishes_refuses_a_missing_target_and_undoes_redoes() {
    use semio_framework_plugin::{artifact_app_laws, EditorApp, PluginApp};

    let original = one_vertex_snapshot();
    let mut app = artifact_app_laws::new_registered_app::<EditorApp<SemioBrepEditor>, _>(async { semio_framework_plugin::App { definition: create_semio_brep_editor(), examples: Vec::new() } }).await;
    let semio_framework_plugin::Effect::LoadDocument { pack, spr } = semio_s_artifact_stdio_contract::load_example_effect(&original, SEMIO_BREP_DOCUMENT_SCHEMA) else { panic!("B-rep fixture produces a document load") };
    app.load_document_pack(&store::ArtifactPackFiles { pack, spr, ops: String::new() }).await.unwrap();
    let meta = artifact_app_laws::meta("local");
    let arguments = |vertex_id: &str, point: [f64; 3]| DslValue::object([("vertexId".into(), DslValue::String(vertex_id.into())), ("point".into(), DslValue::Array(point.into_iter().map(DslValue::float).collect()))]);

    let edit = arguments("v1", [3.0, 4.0, 5.0]);
    app.handle_action("set-vertex", Some(&edit), &meta).await.unwrap();
    artifact_app_laws::settle_registered_typed_operation(&mut app, meta.instance_id).await.unwrap();
    assert_eq!(app.snapshot().unwrap().vertices[0].point, SemioPoint3 { x: 3.0, y: 4.0, z: 5.0 });
    artifact_app_laws::settle_history_verb(&mut app, "undo", meta.instance_id).await;
    assert_eq!(app.snapshot().unwrap(), original);
    artifact_app_laws::settle_history_verb(&mut app, "redo", meta.instance_id).await;
    assert_eq!(app.snapshot().unwrap().vertices[0].point, SemioPoint3 { x: 3.0, y: 4.0, z: 5.0 });

    let missing = arguments("missing", [7.0, 8.0, 9.0]);
    let history_before = semio_framework::io::resolve_ready(app.history_snapshot()).expect("history before refused edit").upserts.len();
    app.handle_action("set-vertex", Some(&missing), &meta).await.unwrap();
    assert!(artifact_app_laws::settle_registered_typed_operation(&mut app, meta.instance_id).await.is_err());
    assert_eq!(app.snapshot().unwrap().vertices[0].point, SemioPoint3 { x: 3.0, y: 4.0, z: 5.0 });
    assert_eq!(semio_framework::io::resolve_ready(app.history_snapshot()).expect("history after refused edit").upserts.len(), history_before);
    artifact_app_laws::close_registered_fixture_app(&mut app);
}

#[semio_framework_async_macros::async_test]
async fn registered_set_vertex_refuses_duplicate_ids_without_history() {
    use semio_framework_plugin::{artifact_app_laws, EditorApp, PluginApp};

    let mut duplicate = one_vertex_snapshot();
    duplicate.vertices.push(duplicate.vertices[0].clone());
    let mut app = artifact_app_laws::new_registered_app::<EditorApp<SemioBrepEditor>, _>(async { semio_framework_plugin::App { definition: create_semio_brep_editor(), examples: Vec::new() } }).await;
    let semio_framework_plugin::Effect::LoadDocument { pack, spr } = semio_s_artifact_stdio_contract::load_example_effect(&duplicate, SEMIO_BREP_DOCUMENT_SCHEMA) else { panic!("B-rep fixture produces a document load") };
    app.load_document_pack(&store::ArtifactPackFiles { pack, spr, ops: String::new() }).await.unwrap();
    let meta = artifact_app_laws::meta("local");
    let history_before = semio_framework::io::resolve_ready(app.history_snapshot()).expect("initial history").upserts.len();
    let args = DslValue::object([("vertexId".into(), DslValue::String("v1".into())), ("point".into(), DslValue::Array(vec![DslValue::float(1.0), DslValue::float(2.0), DslValue::float(3.0)]))]);
    app.handle_action("set-vertex", Some(&args), &meta).await.unwrap();
    assert!(artifact_app_laws::settle_registered_typed_operation(&mut app, meta.instance_id).await.is_err());
    assert_eq!(app.snapshot().unwrap(), duplicate);
    assert_eq!(semio_framework::io::resolve_ready(app.history_snapshot()).expect("history after duplicate refusal").upserts.len(), history_before);
    artifact_app_laws::close_registered_fixture_app(&mut app);
}
//#endregion 🧪️SetVertex
