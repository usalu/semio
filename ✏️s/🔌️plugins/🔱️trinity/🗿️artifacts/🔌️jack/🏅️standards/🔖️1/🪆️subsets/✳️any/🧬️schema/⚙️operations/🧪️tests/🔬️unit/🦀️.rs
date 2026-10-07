use crate::standards::v1::subsets::any::io::binary::mutations::new_trinity_graph_store;
use crate::JackWorkingScene;
use super::*;
use crate::standards::v1::subsets::any::schema::mutations::{register_trinity_graph_mutation_descriptors,set_query,SetQuery};

use crate::{Camera, Edge, Manifest, Node, Port, PortDirection};
use store::ArtifactCommand;

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

fn mini_graph() -> Graph {
    Graph::from_snapshot(mini_fixture()).expect("mini graph")
}

fn check(effect: GraphEffect) -> Result<(), crate::TrinityRamError> {
    validate_graph_effect(&effect, &mini_graph())
}

#[semio_framework_async_macros::async_test]
async fn effect_rejects_port_kind_not_declared_on_the_node_kind() {
    let mut graph = mini_graph();
    graph.manifest = Manifest {
        node_kinds: vec![crate::NodeKindDef { name: "Piece".into(), properties: vec![], port_kinds: vec!["Connector".into()] }],
        edge_kinds: vec![crate::EdgeKindDef { name: "Connection".into(), properties: vec![] }],
        port_kinds: vec![
            crate::PortKindDef { name: "Connector".into(), direction: PortDirection::Out, properties: vec![] },
            crate::PortKindDef { name: "Other".into(), direction: PortDirection::In, properties: vec![] },
        ],
    };
    let effect = GraphEffect::CreateNode(mini_node("new", 0.0, 0.0, vec![Port { id: "p".into(), kind: "Other".into(), direction: PortDirection::In, properties: PropertyBag::new() }]));
    assert!(matches!(validate_graph_effect(&effect, &graph), Err(crate::TrinityRamError::PortKindNotDeclaredOnMutation { .. })));
}

#[semio_framework_async_macros::async_test]
async fn effect_create_edge_rejects_invalid_port_keys_and_missing_endpoints() {
    let edge = |source: String, target: String| GraphEffect::CreateEdge(Edge { id: "e2".into(), kind: "Connection".into(), source, target, properties: PropertyBag::new() });
    assert!(matches!(check(edge("noAt".into(), crate::port_key("child", "in-a"))), Err(crate::TrinityRamError::InvalidSourcePortKey(_))));
    assert!(matches!(check(edge(crate::port_key("root", "out-a"), "noAt".into())), Err(crate::TrinityRamError::InvalidTargetPortKey(_))));
    assert!(matches!(check(edge(crate::port_key("ghost", "out"), crate::port_key("child", "in-a"))), Err(crate::TrinityRamError::SourceNodeNotFound(_))));
    assert!(matches!(check(edge(crate::port_key("root", "out-a"), crate::port_key("ghost", "in"))), Err(crate::TrinityRamError::TargetNodeNotFound(_))));
}

#[semio_framework_async_macros::async_test]
async fn effect_rejects_duplicate_node_and_edge_ids() {
    assert!(matches!(check(GraphEffect::CreateNode(mini_node("root", 0.0, 0.0, vec![]))), Err(crate::TrinityRamError::NodeAlreadyExists(_))));
    assert!(matches!(check(GraphEffect::CreateEdge(Edge { id: "e1".into(), kind: "Connection".into(), source: crate::port_key("root", "out-a"), target: crate::port_key("child", "in-a"), properties: PropertyBag::new() })), Err(crate::TrinityRamError::EdgeAlreadyExists(_))));
}

#[semio_framework_async_macros::async_test]
async fn effect_rejects_missing_entities_on_delete_rename_move_and_property_edits() {
    assert!(matches!(check(GraphEffect::DeleteNode("ghost".into())), Err(crate::TrinityRamError::NodeNotFound(_))));
    assert!(matches!(check(GraphEffect::DeleteEdge("ghost".into())), Err(crate::TrinityRamError::EdgeNotFound(_))));
    assert!(matches!(check(GraphEffect::RenameNode { id: "ghost".into(), name: "x".into() }), Err(crate::TrinityRamError::NodeNotFound(_))));
    assert!(matches!(check(GraphEffect::MoveNode { id: "ghost".into(), x: 0.0, y: 0.0 }), Err(crate::TrinityRamError::NodeNotFound(_))));
    assert!(matches!(check(GraphEffect::RemoveProperty { entity: EntityRef::Node("ghost".into()), key: "label".into() }), Err(crate::TrinityRamError::NodeNotFound(_))));
    assert!(matches!(check(GraphEffect::RemoveProperty { entity: EntityRef::Edge("ghost".into()), key: "u".into() }), Err(crate::TrinityRamError::EdgeNotFound(_))));
}

#[semio_framework_async_macros::async_test]
async fn effect_set_property_checks_the_manifest() {
    let mut graph = mini_graph();
    graph.nodes.get_mut("root").expect("root").kind = "Ghost".into();
    let set = |key: &str, value: PropertyValue| GraphEffect::SetProperty { entity: EntityRef::Node("root".into()), key: key.into(), value };
    assert!(matches!(validate_graph_effect(&set("label", PropertyValue::String("x".into())), &graph), Err(crate::TrinityRamError::UnknownEntityKind { .. })));
    assert!(matches!(check(set("bogus", PropertyValue::Null)), Err(crate::TrinityRamError::UnknownPropertyAtPath { .. })));
    assert!(matches!(check(set("label", PropertyValue::Number(1.0))), Err(crate::TrinityRamError::PropertyTypeMismatch { .. })));
}

#[semio_framework_async_macros::async_test]
async fn apply_graph_effects_applies_a_valid_sequence_and_refuses_an_invalid_one() {
    let mut graph = mini_graph();
    apply_graph_effects(&mut graph, &[GraphEffect::RenameNode { id: "root".into(), name: "renamed".into() }, GraphEffect::MoveNode { id: "root".into(), x: 50.0, y: 60.0 }]).expect("rename and move apply");
    assert_eq!(graph.node("root").map(|node| (node.name.as_str(), node.x, node.y)), Some(("renamed", 50.0, 60.0)));
    assert!(matches!(apply_graph_effects(&mut graph, &[GraphEffect::DeleteNode("ghost".into())]), Err(crate::TrinityRamError::NodeNotFound(_))));
}

#[semio_framework_async_macros::async_test]
async fn set_query_validation_bounds_the_query() {
    let fixture = mini_fixture();
    validate_trinity_graph_operation(&set_query("MATCH (a:Piece) RETURN a".into()), &fixture).expect("bounded query");
    let err = validate_trinity_graph_operation(&set_query("x".repeat(crate::JACK_QUERY_MAXIMUM_BYTES + 1)), &fixture).expect_err("oversized query");
    assert!(matches!(err, crate::TrinityRamError::QueryTooLarge { .. }));
}

#[semio_framework_async_macros::async_test]
async fn document_text_round_trip_graph_store() {
    let mut store = new_trinity_graph_store(create_trinity_graph_envelope("test", mini_fixture()), protocol::ActorId(protocol::LOCAL_ACTOR_ID.into())).await.expect("valid artifact store");
    dispatch_trinity_graph_mutations(&mut store, vec![set_query("MATCH (a:Piece) RETURN a".into())]).await.expect("apply");
    ::store::os_store::test_support::assert_document_text_round_trip(&store).await;
    ::store::os_store::test_support::assert_document_pack_round_trip(&store).await;
}

#[semio_framework_async_macros::async_test]
async fn dispatch_trinity_graph_mutations_noop_on_empty() {
    let mut store = new_trinity_graph_store(create_trinity_graph_envelope("test", mini_fixture()), protocol::ActorId(protocol::LOCAL_ACTOR_ID.into())).await.expect("valid artifact store");
    let generation_before = store.generation();
    dispatch_trinity_graph_mutations(&mut store, vec![]).await.expect("empty ops ok");
    assert_eq!(store.generation(), generation_before);
}

#[semio_framework_async_macros::async_test]
async fn set_query_undo_restores_the_prior_query() {
    let mut store = new_trinity_graph_store(create_trinity_graph_envelope("test", mini_fixture()), protocol::ActorId(protocol::LOCAL_ACTOR_ID.into())).await.expect("valid artifact store");
    let before = store.snapshot().unwrap().query.clone();
    dispatch_trinity_graph_mutations(&mut store, vec![set_query("MATCH (a:Piece) RETURN a".into())]).await.expect("set query");
    assert_eq!(store.snapshot().unwrap().query, "MATCH (a:Piece) RETURN a");
    store.dispatch(ArtifactCommand::Undo).await.expect("undo set query");
    assert_eq!(store.snapshot().unwrap().query, before);
}

#[semio_framework_async_macros::async_test]
async fn dispatch_registers_semantic_descriptors() {
    register_trinity_graph_mutation_descriptors(::semio_framework_schema_state::StateClass::Artifact).expect("mutation descriptor registration");
    for kind in <TrinityGraphMutation as protocol::SemanticMutation<JackSnapshot>>::kinds() {
        assert!(protocol::is_approved_verb(kind.verb), "verb '{}' must be in APPROVED_VERBS", kind.verb);
    }
    assert_eq!(<TrinityGraphMutation as protocol::SemanticMutation<JackSnapshot>>::kinds().len(), 1);
}

//#region 🧪️OutcomeLaws
/// ⚖️ `📋️contract-freeze.md` §C2 laws for the one parent-lane verb.
#[semio_framework_async_macros::async_test]
async fn set_query_outcome_obeys_the_policy_matrix() {
    let base = mini_fixture();
    protocol::os_spr::protocol_laws::assert_outcome_policy_matrix(&base, &TrinityGraphMutation::SetQuery(SetQuery { value: "MATCH (a:Piece) RETURN a".into() })).await;
}
//#endregion 🧪️OutcomeLaws
