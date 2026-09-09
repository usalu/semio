use crate::JackWorkingScene;
use super::*;
use crate::standards::v1::subsets::any::schema::mutations::CreateNode;
use crate::{Camera, Manifest, PortDirection};

fn mini_fixture() -> JackSnapshot {
    JackSnapshot::with_content(JackSnapshot::SCHEMA.into(), "mini".into(), Some("nakagin".into()), Manifest::nakagin_default(), Camera::default(), JackWorkingScene { nodes: vec![
            Node {
                id: "root".into(),
                kind: "Piece".into(),
                name: "core".into(),
                x: 0.0,
                y: 0.0,
                width: 80.0,
                height: 40.0,
                properties: PropertyBag::new(),
                ports: vec![Port { id: "out-a".into(), kind: "Connector".into(), direction: PortDirection::Out, properties: PropertyBag::new() }],
            },
            Node {
                id: "child".into(),
                kind: "Piece".into(),
                name: "capsule".into(),
                x: 120.0,
                y: 0.0,
                width: 80.0,
                height: 40.0,
                properties: PropertyBag::new(),
                ports: vec![Port { id: "in-a".into(), kind: "Connector".into(), direction: PortDirection::In, properties: PropertyBag::new() }],
            },
        ], edges: vec![Edge {
            id: "e1".into(),
            kind: "Connection".into(),
            source: "root@out-a".into(),
            target: "child@in-a".into(),
            properties: {
                let mut p = PropertyBag::new();
                p.insert("u".into(), PropertyValue::Number(1.2));
                p.insert("v".into(), PropertyValue::Number(-0.6));
                p
            },
        }] }, Some("root".into()))
}

fn mini_node(id: &str, x: f64, y: f64, ports: Vec<Port>) -> Node {
    Node { id: id.into(), kind: "Piece".into(), name: id.into(), x, y, width: 80.0, height: 40.0, properties: PropertyBag::new(), ports }
}

#[semio_framework_async_macros::async_test]
async fn graph_op_rejects_port_kind_not_declared_on_operation() {
    let mut fixture = mini_fixture();
    fixture.manifest = Manifest {
        node_kinds: vec![semio_framework_graph::manifest::TrinityNodeKindDef { name: "Piece".into(), properties: vec![], port_kinds: vec!["Connector".into()] }],
        edge_kinds: vec![semio_framework_graph::manifest::TrinityEdgeKindDef { name: "Connection".into(), properties: vec![] }],
        port_kinds: vec![
            semio_framework_graph::manifest::TrinityPortKindDef { name: "Connector".into(), direction: PortDirection::Out, properties: vec![] },
            semio_framework_graph::manifest::TrinityPortKindDef { name: "Other".into(), direction: PortDirection::In, properties: vec![] },
        ],
    };
    let op = create_node(mini_node("new", 0.0, 0.0, vec![Port { id: "p".into(), kind: "Other".into(), direction: PortDirection::In, properties: PropertyBag::new() }]));
    let err = validate_trinity_graph_operation(&op, &fixture).expect_err("bad port kind");
    assert!(matches!(err, crate::TrinityRamError::PortKindNotDeclaredOnMutation { .. }));
}

#[semio_framework_async_macros::async_test]
async fn graph_op_create_edge_rejects_invalid_port_keys() {
    let fixture = mini_fixture();
    let bad_source = create_edge(Edge { id: "e2".into(), kind: "Connection".into(), source: "noAt".into(), target: crate::port_key("child", "in-a"), properties: PropertyBag::new() });
    assert!(matches!(validate_trinity_graph_operation(&bad_source, &fixture), Err(crate::TrinityRamError::InvalidSourcePortKey(_))));
    let bad_target = create_edge(Edge { id: "e3".into(), kind: "Connection".into(), source: crate::port_key("root", "out-a"), target: "noAt".into(), properties: PropertyBag::new() });
    assert!(matches!(validate_trinity_graph_operation(&bad_target, &fixture), Err(crate::TrinityRamError::InvalidTargetPortKey(_))));
}

#[semio_framework_async_macros::async_test]
async fn graph_op_create_edge_rejects_missing_source_and_target_nodes() {
    let fixture = mini_fixture();
    let missing_source = create_edge(Edge { id: "e2".into(), kind: "Connection".into(), source: crate::port_key("ghost", "out"), target: crate::port_key("child", "in-a"), properties: PropertyBag::new() });
    assert!(matches!(validate_trinity_graph_operation(&missing_source, &fixture), Err(crate::TrinityRamError::SourceNodeNotFound(_))));
    let missing_target = create_edge(Edge { id: "e3".into(), kind: "Connection".into(), source: crate::port_key("root", "out-a"), target: crate::port_key("ghost", "in"), properties: PropertyBag::new() });
    assert!(matches!(validate_trinity_graph_operation(&missing_target, &fixture), Err(crate::TrinityRamError::TargetNodeNotFound(_))));
}

#[semio_framework_async_macros::async_test]
async fn graph_op_rejects_duplicate_node_and_edge_ids() {
    let fixture = mini_fixture();
    let dup_node = create_node(mini_node("root", 0.0, 0.0, vec![]));
    assert!(matches!(validate_trinity_graph_operation(&dup_node, &fixture), Err(crate::TrinityRamError::NodeAlreadyExists(_))));
    let dup_edge = create_edge(Edge { id: "e1".into(), kind: "Connection".into(), source: crate::port_key("root", "out-a"), target: crate::port_key("child", "in-a"), properties: PropertyBag::new() });
    assert!(matches!(validate_trinity_graph_operation(&dup_edge, &fixture), Err(crate::TrinityRamError::EdgeAlreadyExists(_))));
}

#[semio_framework_async_macros::async_test]
async fn graph_op_rejects_missing_entities_on_delete_rename_reposition() {
    let fixture = mini_fixture();
    assert!(matches!(validate_trinity_graph_operation(&delete_node("ghost".into()), &fixture), Err(crate::TrinityRamError::NodeNotFound(_))));
    assert!(matches!(validate_trinity_graph_operation(&delete_edge("ghost".into()), &fixture), Err(crate::TrinityRamError::EdgeNotFound(_))));
    assert!(matches!(validate_trinity_graph_operation(&rename_node("ghost".into(), "x".into()), &fixture), Err(crate::TrinityRamError::NodeNotFound(_))));
    assert!(matches!(validate_trinity_graph_operation(&move_node("ghost".into(), 0.0, 0.0), &fixture), Err(crate::TrinityRamError::NodeNotFound(_))));
}

#[semio_framework_async_macros::async_test]
async fn graph_op_set_data_property_rejects_unknown_entity_kind() {
    let fixture = mini_fixture();
    let mut nodes = fixture.nodes();
    nodes[0].kind = "Ghost".into();
    let fixture = JackSnapshot::with_content(fixture.schema.clone(), fixture.name.clone(), fixture.manifest_id.clone(), fixture.manifest.clone(), fixture.camera.clone(), JackWorkingScene { nodes: nodes, edges: fixture.edges() }, fixture.root_node_id.clone());
    let err = validate_trinity_graph_operation(&change_data_property(EntityRef::Node("root".into()), "label".into(), PropertyValue::String("x".into())), &fixture).expect_err("unknown entity kind");
    assert!(matches!(err, crate::TrinityRamError::UnknownEntityKind { .. }));
}

#[semio_framework_async_macros::async_test]
async fn graph_op_set_data_property_rejects_unknown_property_key() {
    let fixture = mini_fixture();
    let err = validate_trinity_graph_operation(&change_data_property(EntityRef::Node("root".into()), "bogus".into(), PropertyValue::Null), &fixture).expect_err("unknown key");
    assert!(matches!(err, crate::TrinityRamError::UnknownPropertyAtPath { .. }));
}

#[semio_framework_async_macros::async_test]
async fn graph_op_set_data_property_rejects_type_mismatch() {
    let fixture = mini_fixture();
    let err = validate_trinity_graph_operation(&change_data_property(EntityRef::Node("root".into()), "label".into(), PropertyValue::Number(1.0)), &fixture).expect_err("type mismatch");
    assert!(matches!(err, crate::TrinityRamError::PropertyTypeMismatch { .. }));
}

#[semio_framework_async_macros::async_test]
async fn graph_op_clear_data_property_rejects_missing_entities() {
    let fixture = mini_fixture();
    assert!(matches!(validate_trinity_graph_operation(&remove_data_property(EntityRef::Node("ghost".into()), "label".into()), &fixture), Err(crate::TrinityRamError::NodeNotFound(_))));
    assert!(matches!(validate_trinity_graph_operation(&remove_data_property(EntityRef::Edge("ghost".into()), "u".into()), &fixture), Err(crate::TrinityRamError::EdgeNotFound(_))));
}

#[semio_framework_async_macros::async_test]
async fn apply_trinity_graph_mutations_applies_valid_sequence_and_rejects_invalid() {
    let fixture = mini_fixture();
    let ok = apply_trinity_graph_mutations(fixture.clone(), &[rename_node("root".into(), "renamed".into())]).expect("rename applies");
    assert_eq!(ok.nodes().iter().find(|n| n.id == "root").unwrap().name, "renamed");

    let err = apply_trinity_graph_mutations(fixture, &[delete_node("ghost".into())]).expect_err("missing node");
    assert!(matches!(err, crate::TrinityRamError::NodeNotFound(_)));
}

#[semio_framework_async_macros::async_test]
async fn document_text_round_trip_graph_store() {
    let mut store = TrinityGraphStore::new(create_trinity_graph_envelope("test", mini_fixture())).await.expect("valid artifact store");
    dispatch_trinity_graph_mutations(&mut store, vec![rename_node("root".into(), "renamed".into())]).await.expect("apply");
    ::store::os_store::test_support::assert_document_text_round_trip(&store).await;
    ::store::os_store::test_support::assert_document_pack_round_trip(&store).await;
}

#[semio_framework_async_macros::async_test]
async fn dispatch_trinity_graph_mutations_noop_on_empty() {
    let mut store = TrinityGraphStore::new(create_trinity_graph_envelope("test", mini_fixture())).await.expect("valid artifact store");
    let generation_before = store.generation();
    dispatch_trinity_graph_mutations(&mut store, vec![]).await.expect("empty ops ok");
    assert_eq!(store.generation(), generation_before);
}

#[semio_framework_async_macros::async_test]
async fn graph_op_reposition_and_rename_undo_restore_prior_values() {
    let mut store = TrinityGraphStore::new(create_trinity_graph_envelope("test", mini_fixture())).await.expect("valid artifact store");
    dispatch_trinity_graph_mutations(&mut store, vec![move_node("root".into(), 50.0, 60.0)]).await.expect("reposition");
    assert_eq!(store.snapshot().unwrap().nodes().iter().find(|n| n.id == "root").unwrap().x, 50.0);
    store.dispatch(ArtifactCommand::Undo).await.expect("undo reposition");
    assert_eq!(store.snapshot().unwrap().nodes().iter().find(|n| n.id == "root").unwrap().x, 0.0);

    dispatch_trinity_graph_mutations(&mut store, vec![rename_node("root".into(), "renamed".into())]).await.expect("rename");
    store.dispatch(ArtifactCommand::Undo).await.expect("undo rename");
    assert_eq!(store.snapshot().unwrap().nodes().iter().find(|n| n.id == "root").unwrap().name, "core");
}

#[semio_framework_async_macros::async_test]
async fn graph_op_delete_edge_undo_recreates_edge() {
    let mut store = TrinityGraphStore::new(create_trinity_graph_envelope("test", mini_fixture())).await.expect("valid artifact store");
    dispatch_trinity_graph_mutations(&mut store, vec![delete_edge("e1".into())]).await.expect("delete edge");
    assert!(store.snapshot().unwrap().edges().is_empty());
    store.dispatch(ArtifactCommand::Undo).await.expect("undo delete edge");
    assert_eq!(store.snapshot().unwrap().edges().len(), 1);
}

#[semio_framework_async_macros::async_test]
async fn graph_op_delete_node_undo_restores_node_and_incident_edges() {
    let mut store = TrinityGraphStore::new(create_trinity_graph_envelope("test", mini_fixture())).await.expect("valid artifact store");
    dispatch_trinity_graph_mutations(&mut store, vec![delete_node("root".into())]).await.expect("delete node");
    let projection = store.snapshot().unwrap();
    assert_eq!(projection.nodes().len(), 1);
    assert!(projection.edges().is_empty());
    store.dispatch(ArtifactCommand::Undo).await.expect("undo delete node");
    let projection = store.snapshot().unwrap();
    assert_eq!(projection.nodes().len(), 2);
    assert_eq!(projection.edges().len(), 1);
}

#[semio_framework_async_macros::async_test]
async fn graph_op_set_and_clear_data_property_undo_round_trip() {
    let mut store = TrinityGraphStore::new(create_trinity_graph_envelope("test", mini_fixture())).await.expect("valid artifact store");
    dispatch_trinity_graph_mutations(&mut store, vec![change_data_property(EntityRef::Node("root".into()), "label".into(), PropertyValue::String("first".into()))]).await.expect("set");
    dispatch_trinity_graph_mutations(&mut store, vec![change_data_property(EntityRef::Node("root".into()), "label".into(), PropertyValue::String("second".into()))]).await.expect("set again");
    store.dispatch(ArtifactCommand::Undo).await.expect("undo second set");
    let value = store.snapshot().unwrap().nodes().iter().find(|n| n.id == "root").unwrap().properties.get("label").cloned();
    assert_eq!(value, Some(PropertyValue::String("first".into())));

    dispatch_trinity_graph_mutations(&mut store, vec![remove_data_property(EntityRef::Node("root".into()), "label".into())]).await.expect("clear");
    assert!(!store.snapshot().unwrap().nodes().iter().find(|n| n.id == "root").unwrap().properties.contains_key("label"));
    store.dispatch(ArtifactCommand::Undo).await.expect("undo clear");
    let value = store.snapshot().unwrap().nodes().iter().find(|n| n.id == "root").unwrap().properties.get("label").cloned();
    assert_eq!(value, Some(PropertyValue::String("first".into())));
}

#[semio_framework_async_macros::async_test]
async fn dispatch_registers_semantic_descriptors() {
    register_trinity_graph_mutation_descriptors(::semio_framework_os_kernel::StateClass::Artifact).expect("mutation descriptor registration");
    for kind in <TrinityGraphMutation as protocol::SemanticMutation<JackSnapshot>>::kinds() {
        assert!(protocol::is_approved_verb(kind.verb), "verb '{}' must be in APPROVED_VERBS", kind.verb);
    }
    assert_eq!(<TrinityGraphMutation as protocol::SemanticMutation<JackSnapshot>>::kinds().len(), 8);
}

//#region 🧪️OutcomeLaws
/// ⚖️ `📋️contract-freeze.md` §C2 laws, per verb family: `assert_missing_target_is_error`/
/// `assert_fatal_never_applies` below, `assert_outcome_policy_matrix` cases further down (delete,
/// rename, create node/edge).
#[semio_framework_async_macros::async_test]
async fn delete_missing_node_is_a_target_missing_error() {
    let base = mini_fixture();
    protocol::os_spr::testkit::assert_missing_target_is_error(&base, &TrinityGraphMutation::DeleteNode(DeleteNode { id: "does-not-exist".into() })).await;
}

#[semio_framework_async_macros::async_test]
async fn rename_missing_node_is_a_target_missing_error() {
    let base = mini_fixture();
    protocol::os_spr::testkit::assert_missing_target_is_error(&base, &TrinityGraphMutation::RenameNode(RenameNode { id: "does-not-exist".into(), new_name: "New".into() })).await;
}

#[semio_framework_async_macros::async_test]
async fn create_node_duplicate_id_never_applies() {
    let base = mini_fixture();
    let duplicate = TrinityGraphMutation::CreateNode(CreateNode { node: mini_node("root", 0.0, 0.0, vec![]) });
    protocol::os_spr::testkit::assert_fatal_never_applies(&duplicate.diff(&base)).await;
}

#[semio_framework_async_macros::async_test]
async fn create_edge_duplicate_id_never_applies() {
    let base = mini_fixture();
    let duplicate = TrinityGraphMutation::CreateEdge(CreateEdge { edge: Edge { id: "e1".into(), kind: "Connection".into(), source: "root@out-a".into(), target: "child@in-a".into(), properties: PropertyBag::new() } });
    protocol::os_spr::testkit::assert_fatal_never_applies(&duplicate.diff(&base)).await;
}

#[semio_framework_async_macros::async_test]
async fn delete_node_outcome_obeys_the_policy_matrix() {
    let base = mini_fixture();
    protocol::os_spr::testkit::assert_outcome_policy_matrix(&base, &TrinityGraphMutation::DeleteNode(DeleteNode { id: "child".into() })).await;
}

#[semio_framework_async_macros::async_test]
async fn rename_node_outcome_obeys_the_policy_matrix() {
    let base = mini_fixture();
    protocol::os_spr::testkit::assert_outcome_policy_matrix(&base, &TrinityGraphMutation::RenameNode(RenameNode { id: "child".into(), new_name: "New".into() })).await;
}

#[semio_framework_async_macros::async_test]
async fn create_node_outcome_obeys_the_policy_matrix() {
    let base = mini_fixture();
    protocol::os_spr::testkit::assert_outcome_policy_matrix(&base, &TrinityGraphMutation::CreateNode(CreateNode { node: mini_node("node-fresh", 10.0, 10.0, vec![]) })).await;
}

#[semio_framework_async_macros::async_test]
async fn create_edge_outcome_obeys_the_policy_matrix() {
    let base = mini_fixture();
    let edge = Edge { id: "e2".into(), kind: "Connection".into(), source: "root@out-a".into(), target: "child@in-a".into(), properties: PropertyBag::new() };
    protocol::os_spr::testkit::assert_outcome_policy_matrix(&base, &TrinityGraphMutation::CreateEdge(CreateEdge { edge })).await;
}
//#endregion 🧪️OutcomeLaws
