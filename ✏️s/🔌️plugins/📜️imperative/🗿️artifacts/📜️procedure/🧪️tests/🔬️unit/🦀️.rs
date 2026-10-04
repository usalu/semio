use super::*;

/// 🪪️ `artifact_kind().schema` IS `PROCEDURE_DOCUMENT_SCHEMA`: a document kind has ONE schema identity — the hub's codec rows, document-open targets and genesis, the MCP workspace
/// store and host-media contributions all key on it (ticket 26/09/23 W4: a distinct "media schema" left the package without a
/// codec owner, so the trusted catalog refused it). The former media string stays declared as `source_format`.
#[semio_framework_async_macros::async_test]
async fn artifact_kind_names_the_store_schema() {
    assert_eq!(artifact_kind().schema, PROCEDURE_DOCUMENT_SCHEMA);
    assert_eq!(artifact_kind().source_format, "procedure.document");
}

/// 🫙️ The default parent names the derivable empty program and seed.
#[semio_framework_async_macros::async_test]
async fn default_snapshot_names_the_empty_program() {
    let snapshot = ProcedureSnapshot::default();
    assert_eq!(snapshot.schema, "procedure.document");
    let scene = procedure_derivable_scene(&snapshot).expect("the empty program is derivable");
    assert!(scene.path.steps.is_empty());
    assert!(scene.seed.is_empty());
}

/// 🌉️ Programs (nested bodies, params, order) and seeds round-trip through the composed `flow`/`text` child snapshots exactly.
#[semio_framework_async_macros::async_test]
async fn program_and_seed_round_trip_through_the_composed_child_snapshots() {
    let mut body = Path::new();
    body.steps.push(Step { id: "inner".into(), kind: "log.print".into(), params: Dictionary::new().insert("message", Value::Atom(neural_engine::Atom::String("loop".into()))), bodies: Default::default() });
    let mut path = schema::default_path();
    path.steps.push(Step { id: "loop".into(), kind: "control.repeat".into(), params: Dictionary::new(), bodies: BTreeMap::from([("body".to_string(), body)]) });
    assert_eq!(path_from_flow_content_snapshot(&flow_content_snapshot_from_path(&path)), path);
    let seed = BTreeMap::from([("counter".to_string(), Value::Atom(neural_engine::Atom::Integer(3)))]);
    assert_eq!(seed_from_text_content_snapshot(&text_content_snapshot_from_seed(&seed)), seed);
    neural_engine::ColdRetire::retire_cold(seed);
}

/// 🌱️ Only the derivable children (bundled demo, empty program) have a genesis pack; any other child id is not derivable,
/// so a decoded parent never needs a working scene on its handles (design §20.15).
#[semio_framework_async_macros::async_test]
async fn only_derivable_children_have_a_genesis_pack() {
    use store::ArtifactPack;
    let demo = schema::default_snapshot();
    let flow = genesis_procedure_child_pack(&demo, "flow", &demo.flow.child_id).expect("the demo flow is derivable");
    assert_eq!(path_from_flow_content_snapshot(&SemioFlowSnapshot::decode_pack(&flow).expect("genesis flow pack decodes")), examples::demo::scene().path);
    assert!(genesis_procedure_child_pack(&demo, "text", &demo.text.child_id).is_some(), "the demo seed is derivable");
    let empty = ProcedureSnapshot::default();
    assert!(genesis_procedure_child_pack(&empty, "flow", &empty.flow.child_id).is_some(), "the empty program is derivable");
    let mut other_path = Path::new();
    other_path.steps.push(Step { id: "x".into(), kind: "log.print".into(), params: Dictionary::new(), bodies: Default::default() });
    let other = procedure_snapshot_naming(&other_path, &BTreeMap::new());
    assert!(genesis_procedure_child_pack(&other, "flow", &other.flow.child_id).is_none());
    assert!(genesis_procedure_child_pack(&demo, "other-slot", &demo.flow.child_id).is_none());
}

/// 🧮️ A program edit yields point-invertible flow leaves: severed edges before removed nodes, inserted nodes before their edges.
#[semio_framework_async_macros::async_test]
async fn flow_leaves_order_edges_around_their_nodes() {
    let base = schema::default_path();
    let mut next = base.clone();
    next.steps.remove(0);
    let leaves = procedure_flow_leaves(&base, &next);
    let position = |kind: &str| leaves.iter().position(|leaf| format!("{leaf:?}").starts_with(kind)).unwrap_or_else(|| panic!("{kind} in {leaves:?}"));
    assert!(position("RemoveEdge") < position("RemoveNode"), "{leaves:?}");
    let restore = procedure_flow_leaves(&next, &base);
    assert!(restore.iter().position(|leaf| format!("{leaf:?}").starts_with("InsertNode")) < restore.iter().position(|leaf| format!("{leaf:?}").starts_with("InsertEdge")), "{restore:?}");
    assert!(procedure_flow_leaves(&base, &base).is_empty());
}

/// 🧬️ The projection names exactly the snapshot's declared child slots — what the live envelope load checks
/// before a decoded document may replace the store.
#[test]
fn the_child_restore_projection_names_every_declared_child_slot() {
    let snapshot = crate::schema::default_snapshot();
    let projection = crate::procedure_child_restore_projection(&snapshot).expect("the loaded-parent child projection");
    assert_eq!(projection.len(), <crate::ProcedureSnapshot as semio_framework_schema_composition::ArtifactCompositionFields>::child_slots().len());
}
