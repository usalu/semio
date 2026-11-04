use super::*;
use semio_framework_value::ToValue;

fn event_set(path: &str, value: impl ToValue) -> SnapshotEditEvent {
    SnapshotEditEvent::SetValue { path: path.to_string(), value: value.to_value() }
}

fn event_remove(path: &str) -> SnapshotEditEvent {
    SnapshotEditEvent::RemoveValue { path: path.to_string() }
}

fn base() -> GltfSnapshot {
    let mut base = GltfSnapshot::default();
    base.document.extensions_used = vec!["KHR_a".to_string(), "KHR_b".to_string(), "KHR_c".to_string()];
    let mut parent = GltfNode::from_value(DslValue::Object(Vec::new())).unwrap();
    parent.children = vec![4, 5, 6];
    base.document.nodes = vec![parent];
    base
}

fn special(event: &SnapshotEditEvent) -> Option<Vec<GltfMutation>> {
    resolve_special(&base(), event).unwrap()
}

fn planned(event: &SnapshotEditEvent) -> Option<&'static str> {
    RULES.plan(&base().to_value(), event).unwrap().map(|plan| plan.kind)
}

#[test]
fn a_plain_set_belongs_to_the_declarative_table() {
    assert_eq!(special(&event_set("/document/nodes/0/name", "tree")), None);
    assert_eq!(planned(&event_set("/document/nodes/0/name", "tree")), Some("change-node-name"));
}

#[test]
fn unsetting_an_entity_is_computed() {
    assert_eq!(
        special(&event_remove("/document/nodes/2/name")),
        Some(vec![kinds::change_node_name::mutation(kinds::change_node_name::GltfChangeNodeNamePayload { node: 2, value: None })])
    );
}

#[test]
fn a_reference_edit_raises_a_bind_or_unbind_kind() {
    assert_eq!(
        special(&event_set("/document/meshes/0/primitives/1/attributes/POSITION", 3usize)),
        Some(vec![kinds::bind_primitive_attribute::mutation(kinds::bind_primitive_attribute::GltfBindPrimitiveAttributePayload { mesh: 0, primitive: 1, semantic: "POSITION".to_string(), accessor: 3 })])
    );
    assert_eq!(
        special(&event_remove("/document/nodes/0/mesh")),
        Some(vec![kinds::unbind_node_mesh::mutation(kinds::unbind_node_mesh::GltfUnbindNodeMeshPayload { node: 0 })])
    );
}

#[test]
fn row_inserts_and_removals_by_position_belong_to_the_declarative_table() {
    assert_eq!(special(&event_remove("/document/nodes/1")), None);
    assert_eq!(planned(&event_remove("/document/nodes/0")), Some("delete-node"));
    let insert = SnapshotEditEvent::InsertValue { path: "/document/nodes/-".to_string(), value: DslValue::Object(Vec::new()) };
    assert_eq!(special(&insert), None);
    assert_eq!(planned(&insert), Some("create-node"));
}

#[test]
fn a_move_names_the_entry_it_relocates() {
    assert_eq!(
        special(&SnapshotEditEvent::MoveValue { from: "/document/meshes/2".to_string(), path: "/document/meshes/0".to_string() }),
        Some(vec![kinds::move_mesh::mutation(kinds::move_mesh::GltfMoveMeshPayload { index: 2, position: 0 })])
    );
    assert_eq!(
        special(&SnapshotEditEvent::MoveValue { from: "/document/extensionsUsed/2".to_string(), path: "/document/extensionsUsed/0".to_string() }),
        Some(vec![kinds::move_used_extension::mutation(kinds::move_used_extension::GltfMoveUsedExtensionPayload { extension: "KHR_c".to_string(), position: 0 })])
    );
}

#[test]
fn a_nested_removal_reads_the_entry_it_removes() {
    assert_eq!(
        special(&event_remove("/document/nodes/0/children/1")),
        Some(vec![kinds::unbind_node_child::mutation(kinds::unbind_node_child::GltfUnbindNodeChildPayload { parent: 0, child: 5 })])
    );
    assert_eq!(
        special(&event_remove("/document/extensionsUsed/1")),
        Some(vec![kinds::remove_used_extension::mutation(kinds::remove_used_extension::GltfWithdrawUsedExtensionPayload { extension: "KHR_b".to_string() })])
    );
}

#[test]
fn a_reorder_keeps_the_members_and_a_membership_change_is_refused() {
    let reordered = event_set("/document/extensionsUsed", vec!["KHR_c".to_string(), "KHR_a".to_string(), "KHR_b".to_string()]);
    assert!(matches!(special(&reordered).unwrap().as_slice(), [GltfMutation::ReorderUsedExtensions(_)]));
    let changed = event_set("/document/extensionsUsed", vec!["KHR_c".to_string()]);
    assert!(resolve_special(&base(), &changed).is_err());
}

#[test]
fn an_edit_no_kind_expresses_is_left_to_the_table_to_refuse() {
    assert_eq!(special(&event_set("/document/nodes/0/unknown", 1usize)), None);
    assert_eq!(special(&SnapshotEditEvent::RenameKey { path: "/document/asset".to_string(), key: "x".to_string() }), None);
    let buffer = SnapshotEditEvent::InsertValue { path: "/document/buffers/0".to_string(), value: DslValue::Object(Vec::new()) };
    assert!(resolve_special(&base(), &buffer).is_err());
}

fn transform_event(source: &str) -> SnapshotEditEvent {
    SnapshotEditEvent::SetValue { path: super::super::editing::TRANSFORM_PATH.to_string(), value: semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::parse(source, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap()) }
}

fn scene_with_roots(nodes: Vec<GltfNode>) -> GltfSnapshot {
    let mut base = GltfSnapshot::default();
    let count = nodes.len();
    base.document.nodes = nodes;
    base.document.scenes = vec![GltfScene::from_value(DslValue::Object(Vec::new())).unwrap()];
    base.document.scenes[0].nodes = (0..count).collect();
    base
}

fn node(source: &str) -> GltfNode {
    GltfNode::from_value(semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::parse(source, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap())).unwrap()
}

#[test]
fn a_transform_gesture_moves_scene_roots_in_their_own_form_and_keeps_children_untouched() {
    let base = scene_with_roots(vec![node(r#"{"translation":[1,0,0],"children":[1]}"#), node(r#"{"matrix":[1,0,0,0,0,1,0,0,0,0,1,0,3,0,0,1]}"#)]);
    let rows = resolve_special(&base, &transform_event(r#"{"translate":[0,5,0],"scale":[2,2,2]}"#)).unwrap().expect("the gesture is computed");
    let expect = |node: usize, transform| kinds::change_node_transform::mutation(kinds::change_node_transform::GltfTransformNodePayload { node, transform });
    assert_eq!(rows, vec![
        expect(0, kinds::change_node_transform::GltfNodeTransform::Trs { translation: Some([2.0, 5.0, 0.0]), rotation: None, scale: Some([2.0, 2.0, 2.0]) }),
        expect(1, kinds::change_node_transform::GltfNodeTransform::Matrix { matrix: [2.0, 0.0, 0.0, 0.0, 0.0, 2.0, 0.0, 0.0, 0.0, 0.0, 2.0, 0.0, 6.0, 5.0, 0.0, 1.0] }),
    ]);
}

#[test]
fn a_transform_gesture_that_would_shear_or_has_no_roots_is_refused() {
    let turned = scene_with_roots(vec![node(r#"{"rotation":[0,0,0.7071067811865476,0.7071067811865476]}"#)]);
    assert!(resolve_special(&turned, &transform_event(r#"{"scale":[1,2,1]}"#)).is_err());
    assert!(resolve_special(&GltfSnapshot::default(), &transform_event(r#"{"translate":[1,0,0]}"#)).is_err());
}
