use crate::JackWorkingScene;
use super::*;
use semio_s_artifact_stdio_semio::standards::v1::subsets::graph::schema::mutations::SemioGraphMutation;
use semio_s_artifact_stdio_semio::standards::v1::subsets::graph::schema::snapshot::SemioGraphSnapshot;

trait JackChildOwnerOracle {
    fn expected() -> semio_framework_pack_json::Value;
}

struct SerdeJsonJackChildOwnerOracle;

impl JackChildOwnerOracle for SerdeJsonJackChildOwnerOracle {
    fn expected() -> semio_framework_pack_json::Value {
        semio_framework_pack_json::parse(include_str!("../../🧫️fixtures/🧫️child-owner-isolation/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("language-neutral Jack child-owner fixture")
    }
}

#[semio_framework_async_macros::async_test]
async fn working_scene_belongs_to_the_exact_content_child() {
    let owned = jack_content_child_with_owner(Vec::new(), Vec::new());
    let wire = semio_framework_pack_json::to_string(&semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(&owned))).into_bytes();
    let reconstructed: JackContentChild = semio_framework_value::FromValue::from_value(semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::parse_bytes(&wire, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("Jack child wire roundtrip"))).expect("Jack child wire roundtrip");
    let observed = semio_framework_pack_json::json!({
        "ownedHasScene": owned.local_owner::<crate::JackContentOwner>().is_some(),
        "wireIdentityMatches": owned == reconstructed,
        "wireHasScene": reconstructed.local_owner::<crate::JackContentOwner>().is_some(),
    });

    assert_eq!(observed, SerdeJsonJackChildOwnerOracle::expected());
}

#[semio_framework_async_macros::async_test]
async fn jack_child_restore_projection_accepts_the_exact_owned_content() {
    let snapshot = JackSnapshot::default();
    let projection = store::ChildRestoreProjection::from_snapshot(&snapshot).expect("canonical Jack content child");
    assert_eq!(projection.len(), 1);
    assert!(projection.admits_member("content", &snapshot.content.target));
    assert_eq!(snapshot.content.child_id, snapshot.content.target.artifact_id);
}

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
                properties: {
                    let mut p = PropertyBag::new();
                    let mut pos = PropertyBag::new();
                    pos.insert("x".into(), PropertyValue::Number(0.0));
                    pos.insert("y".into(), PropertyValue::Number(0.0));
                    pos.insert("z".into(), PropertyValue::Number(0.0));
                    p.insert("position".into(), PropertyValue::Object(pos));
                    p
                },
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

#[semio_framework_async_macros::async_test]
async fn manifest_nakagin_has_piece_and_connection() {
    let m = Manifest::nakagin_default();
    assert!(m.node_kind("Piece").is_some());
    assert!(m.edge_kind("Connection").is_some());
}

#[semio_framework_async_macros::async_test]
async fn fixture_loads_manifest_id_only() {
    let json = r#"{"schema":"trinity.graph","name":"mini","manifestId":"nakagin","manifest":{"nodeKinds":[],"edgeKinds":[],"portKinds":[]},"camera":{"x":0,"y":0,"zoom":1},"content":{"childId":"mini-content","target":{"artifactId":"mini-content","dialect":{"artifactKind":"s.stdio.semio","standard":"v1","subset":"graph"}}},"query":""}"#;
    let mut snapshot = JackSnapshot::from_json(json).unwrap();
    snapshot.resolve_manifest().unwrap();
    assert!(snapshot.manifest.node_kind("Piece").is_some());
}

#[semio_framework_async_macros::async_test]
async fn fixture_round_trip() {
    let fixture = mini_fixture();
    let json = fixture.to_json().unwrap();
    let mut back = JackSnapshot::from_json(&json).unwrap();
    assert_eq!(back.content, fixture.content, "the JSON parent carries the exact content coordinate");
    assert!(back.nodes().is_err(), "the JSON parent carries no content owner: its child is a separate member");
    crate::materialize_jack_snapshot(&mut back.content, crate::jack_content_for_handle(&fixture.content).expect("fixture content owner").snapshot().clone());
    assert_eq!(back.nodes().expect("valid retained Jack child").len(), 2);
    assert_eq!(back.edges().expect("valid retained Jack child").len(), 1);
}

#[semio_framework_async_macros::async_test]
async fn remove_node_cascades_edges() {
    let mut g = Graph::from_snapshot(mini_fixture()).unwrap();
    assert!(g.remove_node("root"));
    assert!(g.edges.is_empty());
    assert!(g.nodes.contains_key("child"));
}

#[semio_framework_async_macros::async_test]
async fn graph_effects_create_a_node_and_its_content_leaf_undoes_it() {
    let fixture = mini_fixture();
    let mut graph = Graph::from_snapshot(fixture.clone()).unwrap();
    let effect = GraphEffect::CreateNode(Node { id: "new".into(), kind: "Piece".into(), name: "new-piece".into(), x: 200.0, y: 40.0, width: 80.0, height: 40.0, properties: PropertyBag::new(), ports: vec![] });
    apply_graph_effects(&mut graph, std::slice::from_ref(&effect)).expect("create");
    assert_eq!(graph.nodes.len(), 3);
    let base = jack_content_for_handle(&fixture.content).expect("retained content").snapshot().clone();
    let leaves = graph_leaves(&base, &[effect]);
    assert_eq!(leaves.len(), 1);
    let mut child = base.clone();
    let inverse = <SemioGraphMutation as protocol::Mutation<SemioGraphSnapshot>>::inverse(&leaves[0], &child).expect("inverse");
    child = protocol::apply_diff(<SemioGraphMutation as protocol::Mutation<SemioGraphSnapshot>>::diff(&leaves[0], &child).diff(), &child).expect("leaf applies");
    assert_eq!(child.nodes.len(), 3);
    for step in inverse.iter().rev() {
        child = protocol::apply_diff(<SemioGraphMutation as protocol::Mutation<SemioGraphSnapshot>>::diff(step, &child).diff(), &child).expect("inverse step applies");
    }
    assert_eq!(child, base);
}

#[semio_framework_async_macros::async_test]
async fn graph_effects_validate_a_create_edge_batch_incrementally() {
    let mut graph = Graph::from_snapshot(mini_fixture()).unwrap();
    apply_graph_effects(
        &mut graph,
        &[
            GraphEffect::CreateNode(Node { id: "x-9".into(), kind: "Piece".into(), name: "x".into(), x: 1080.0, y: 0.0, width: 80.0, height: 40.0, properties: PropertyBag::new(), ports: vec![Port { id: "out".into(), kind: "Connector".into(), direction: PortDirection::Out, properties: PropertyBag::new() }] }),
            GraphEffect::CreateNode(Node { id: "y-10".into(), kind: "Piece".into(), name: "y".into(), x: 1200.0, y: 80.0, width: 80.0, height: 40.0, properties: PropertyBag::new(), ports: vec![Port { id: "in".into(), kind: "Connector".into(), direction: PortDirection::In, properties: PropertyBag::new() }] }),
            GraphEffect::CreateEdge(Edge { id: "e-batch".into(), kind: "Connection".into(), source: port_key("x-9", "out"), target: port_key("y-10", "in"), properties: PropertyBag::new() }),
        ],
    )
    .expect("batch create edge");
    assert_eq!(graph.nodes.len(), 4);
    assert_eq!(graph.edges.len(), 2);
}

#[semio_framework_async_macros::async_test]
async fn graph_effects_reject_an_unknown_node_kind() {
    let graph = Graph::from_snapshot(mini_fixture()).unwrap();
    let err = validate_graph_effect(&GraphEffect::CreateNode(Node { id: "new".into(), kind: "Piece2".into(), name: "x".into(), x: 0.0, y: 0.0, width: 80.0, height: 40.0, properties: PropertyBag::new(), ports: vec![] }), &graph).expect_err("unknown kind");
    assert!(err.to_string().contains("unknown node kind"));
}

#[semio_framework_async_macros::async_test]
async fn from_json_rejects_wrong_schema() {
    let json = r#"{"schema":"bogus","name":"x","manifest":{"nodeKinds":[],"edgeKinds":[],"portKinds":[]},"camera":{"x":0,"y":0,"zoom":1},"content":{"childId":"x-content","target":{"artifactId":"x-content","dialect":{"artifactKind":"s.stdio.semio","standard":"v1","subset":"graph"}}},"query":""}"#;
    let err = JackSnapshot::from_json(json).expect_err("schema mismatch");
    assert!(err.to_string().contains("expected schema trinity.graph"));
}

#[semio_framework_async_macros::async_test]
async fn resolve_manifest_errors_when_missing_and_empty() {
    let mut fixture = JackSnapshot::with_content(JackSnapshot::SCHEMA.into(), "x".into(), None, Manifest::default(), Camera::default(), JackWorkingScene { nodes: vec![], edges: vec![] }, None);
    let err = fixture.resolve_manifest().expect_err("missing manifest");
    assert!(matches!(err, TrinityRamError::ManifestMissing));
}

#[semio_framework_async_macros::async_test]
async fn resolve_manifest_errors_on_unknown_id() {
    let mut fixture = JackSnapshot::with_content(JackSnapshot::SCHEMA.into(), "x".into(), Some("nope".into()), Manifest::default(), Camera::default(), JackWorkingScene { nodes: vec![], edges: vec![] }, None);
    let err = fixture.resolve_manifest().expect_err("unknown manifest id");
    assert!(err.to_string().contains("unknown manifest id nope"));
}

#[semio_framework_async_macros::async_test]
async fn graph_from_host_snapshot_rejects_port_kind_not_declared_on_node_kind() {
    let fixture = mini_fixture();
    let mut nodes = fixture.nodes().expect("valid retained Jack child");
    nodes[0].ports.push(Port { id: "bad".into(), kind: "core circular bottom".into(), direction: PortDirection::Out, properties: PropertyBag::new() });
    let fixture = JackSnapshot::with_content(fixture.schema.clone(), fixture.name.clone(), fixture.manifest_id.clone(), fixture.manifest.clone(), fixture.camera.clone(), JackWorkingScene { nodes: nodes, edges: fixture.edges().expect("valid retained Jack child") }, fixture.root_node_id.clone());
    let err = Graph::from_snapshot(fixture).expect_err("undeclared port kind");
    assert!(matches!(err, TrinityRamError::PortKindNotDeclaredOnFixture { .. }));
    assert!(err.to_string().contains("root"));
}

#[semio_framework_async_macros::async_test]
async fn graph_accessors_and_mutators() {
    let mut g = Graph::from_snapshot(mini_fixture()).unwrap();
    assert!(g.node("root").is_some());
    assert!(g.node("ghost").is_none());
    assert!(g.edge("e1").is_some());
    g.node_mut("root").unwrap().name = "renamed".into();
    assert_eq!(g.node("root").unwrap().name, "renamed");

    g.add_node(Node { id: "extra".into(), kind: "Piece".into(), name: "extra".into(), x: 0.0, y: 0.0, width: 10.0, height: 10.0, properties: PropertyBag::new(), ports: vec![] });
    assert!(g.node("extra").is_some());

    g.add_edge(Edge { id: "e2".into(), kind: "Connection".into(), source: "root@out-a".into(), target: "extra@in-a".into(), properties: PropertyBag::new() });
    assert!(g.edge("e2").is_some());
    assert!(g.remove_edge("e2"));
    assert!(!g.remove_edge("e2"));
}

#[semio_framework_async_macros::async_test]
async fn graph_remove_node_clears_root_node_id() {
    let mut g = Graph::from_snapshot(mini_fixture()).unwrap();
    assert!(g.remove_node("root"));
    assert!(g.edges.is_empty());
    assert!(g.nodes.contains_key("child"));
    assert!(g.root_node_id.is_none());
    assert!(!g.remove_node("root"));
}

#[semio_framework_async_macros::async_test]
async fn graph_set_property_success_and_errors() {
    let mut g = Graph::from_snapshot(mini_fixture()).unwrap();
    g.set_property(EntityRef::Node("root".into()), "label", PropertyValue::String("hi".into())).expect("set node prop");
    assert_eq!(g.node("root").unwrap().properties.get("label"), Some(&PropertyValue::String("hi".into())));
    let err = g.set_property(EntityRef::Node("ghost".into()), "label", PropertyValue::Null).expect_err("missing node");
    assert!(matches!(err, TrinityRamError::NodeNotFound(_)));

    g.set_property(EntityRef::Edge("e1".into()), "gap", PropertyValue::Number(1.0)).expect("set edge prop");
    assert_eq!(g.edge("e1").unwrap().properties.get("gap"), Some(&PropertyValue::Number(1.0)));
    let err = g.set_property(EntityRef::Edge("ghost".into()), "gap", PropertyValue::Null).expect_err("missing edge");
    assert!(matches!(err, TrinityRamError::EdgeNotFound(_)));
}

#[semio_framework_async_macros::async_test]
async fn graph_to_host_snapshot_and_fixture_json() {
    let g = Graph::from_snapshot(mini_fixture()).unwrap();
    let fixture = g.to_snapshot();
    assert_eq!(fixture.nodes().expect("valid retained Jack child").len(), 2);
    assert_eq!(fixture.manifest_id.as_deref(), Some("nakagin"));
    let json = g.host_snapshot_json().expect("fixture json");
    assert!(json.contains("\"schema\""));
}

#[semio_framework_async_macros::async_test]
async fn subgraph_snapshot_filters_entities_and_keeps_root_when_included() {
    let g = Graph::from_snapshot(mini_fixture()).unwrap();
    let node_ids: BTreeSet<String> = ["root".to_string()].into_iter().collect();
    let sub = g.subgraph_snapshot(&node_ids, &BTreeSet::new());
    assert_eq!(sub.nodes().expect("valid retained Jack child").len(), 1);
    assert!(sub.edges().expect("valid retained Jack child").is_empty());
    assert_eq!(sub.root_node_id.as_deref(), Some("root"));
    assert!(sub.name.contains("subgraph"));
}

#[semio_framework_async_macros::async_test]
async fn subgraph_snapshot_drops_root_when_not_included() {
    let g = Graph::from_snapshot(mini_fixture()).unwrap();
    let node_ids: BTreeSet<String> = ["child".to_string()].into_iter().collect();
    let sub = g.subgraph_snapshot(&node_ids, &BTreeSet::new());
    assert!(sub.root_node_id.is_none());
}

#[semio_framework_async_macros::async_test]
async fn port_key_helpers_handle_malformed_keys() {
    assert_eq!(parse_port_key("node@port"), Some(("node", "port")));
    assert_eq!(parse_port_key("noport"), None);
    assert_eq!(parse_port_key("@port"), None);
    assert_eq!(parse_port_key("node@"), None);
    assert_eq!(port_node_id("node@port"), Some("node"));
    assert_eq!(port_port_id("node@port"), Some("port"));
    assert_eq!(port_key("a", "b"), "a@b");
}

/// 🧬️ The projection names exactly the snapshot's declared child slots — what the live envelope load checks
/// before a decoded document may replace the store.
#[test]
fn the_child_restore_projection_names_every_declared_child_slot() {
    let snapshot = crate::empty_trinity_graph_snapshot();
    let projection = crate::jack_child_restore_projection(&snapshot).expect("the loaded-parent child projection");
    assert_eq!(projection.len(), <crate::JackSnapshot as semio_framework_schema_composition::ArtifactCompositionFields>::child_slots().len());
}
