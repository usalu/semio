
use super::*;

trait JackChildOwnerOracle {
    fn expected() -> pack::JsonValue;
}

struct SerdeJsonJackChildOwnerOracle;

impl JackChildOwnerOracle for SerdeJsonJackChildOwnerOracle {
    fn expected() -> pack::JsonValue {
        pack::parse_json(include_str!("../../🧫️fixtures/🧫️child-owner-isolation/🔣️.json")).expect("language-neutral Jack child-owner fixture")
    }
}

#[semio_framework_async_macros::async_test]
async fn working_scene_belongs_to_the_exact_content_child() {
    let owned = jack_content_child_with_owner(Vec::new(), Vec::new());
    let wire = pack::json_to_string(&pack::json_from_dsl_value(&dsl::to_dsl_value(&owned).expect("Jack child wire identity"))).into_bytes();
    let reconstructed: JackContentChild = dsl::from_dsl_value(pack::json_to_dsl_value(&pack::parse_json_bytes(&wire).expect("Jack child wire roundtrip"))).expect("Jack child wire roundtrip");
    let observed = pack::json!({
        "ownedHasScene": owned.local_owner::<JackWorkingScene>().is_some(),
        "wireIdentityMatches": owned == reconstructed,
        "wireHasScene": reconstructed.local_owner::<JackWorkingScene>().is_some(),
    });

    assert_eq!(observed, SerdeJsonJackChildOwnerOracle::expected());
}
use crate::standards::v1::subsets::any::schema::mutations::{create_edge, create_node};
use crate::standards::v1::subsets::any::schema::mutations::text::{dispatch_trinity_graph_mutations, validate_trinity_graph_operation};
use store::ArtifactCommand;

fn mini_fixture() -> JackSnapshot {
    JackSnapshot::with_content(
        JackSnapshot::SCHEMA.into(),
        "mini".into(),
        Some("nakagin".into()),
        Manifest::nakagin_default(),
        Camera::default(),
        vec![
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
                    let mut pos = BTreeMap::new();
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
        ],
        vec![Edge {
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
        }],
        Some("root".into()),
    )
}

#[semio_framework_async_macros::async_test]
async fn manifest_nakagin_has_piece_and_connection() {
    let m = Manifest::nakagin_default();
    assert!(m.node_kind("Piece").is_some());
    assert!(m.edge_kind("Connection").is_some());
}

#[semio_framework_async_macros::async_test]
async fn fixture_loads_manifest_id_only() {
    let json = r#"{"schema":"trinity.graph","name":"mini","manifestId":"nakagin","camera":{"x":0,"y":0,"zoom":1},"nodes":[],"edges":[]}"#;
    let graph = Graph::load_json(json).unwrap();
    assert!(graph.manifest.node_kind("Piece").is_some());
}

#[semio_framework_async_macros::async_test]
async fn fixture_round_trip() {
    let fixture = mini_fixture();
    let json = fixture.to_json().unwrap();
    let back = JackSnapshot::from_json(&json).unwrap();
    assert_eq!(back.nodes().len(), 2);
    assert_eq!(back.edges().len(), 1);
}

#[semio_framework_async_macros::async_test]
async fn remove_node_cascades_edges() {
    let mut g = Graph::from_fixture(mini_fixture()).unwrap();
    assert!(g.remove_node("root"));
    assert!(g.edges.is_empty());
    assert!(g.nodes.contains_key("child"));
}

#[semio_framework_async_macros::async_test]
async fn graph_op_create_node_and_undo() {
    let fixture = mini_fixture();
    let mut store = TrinityGraphStore::new(create_trinity_graph_envelope("test", fixture)).await.expect("valid artifact store");
    dispatch_trinity_graph_mutations(&mut store, vec![create_node(Node { id: "new".into(), kind: "Piece".into(), name: "new-piece".into(), x: 200.0, y: 40.0, width: 80.0, height: 40.0, properties: PropertyBag::new(), ports: vec![] })])
        .await
        .expect("create");
    assert_eq!(store.snapshot().expect("projection").nodes().len(), 3);
    store.dispatch(ArtifactCommand::Undo).await.expect("undo");
    assert_eq!(store.snapshot().expect("projection").nodes().len(), 2);
}

#[semio_framework_async_macros::async_test]
async fn graph_op_dispatch_validates_create_edge_batch_incrementally() {
    let fixture = mini_fixture();
    let mut nodes = fixture.nodes();
    while nodes.len() < 9 {
        nodes.push(Node { id: format!("pad-{}", nodes.len()), kind: "Piece".into(), name: format!("pad-{}", nodes.len()), x: 0.0, y: 0.0, width: 80.0, height: 40.0, properties: PropertyBag::new(), ports: vec![] });
    }
    let fixture = JackSnapshot::with_content(fixture.schema.clone(), fixture.name.clone(), fixture.manifest_id.clone(), fixture.manifest.clone(), fixture.camera.clone(), nodes, fixture.edges(), fixture.root_node_id.clone());
    let mut store = TrinityGraphStore::new(create_trinity_graph_envelope("test", fixture)).await.expect("valid artifact store");
    dispatch_trinity_graph_mutations(
        &mut store,
        vec![
            create_node(Node {
                id: "x-9".into(),
                kind: "Piece".into(),
                name: "x".into(),
                x: 1080.0,
                y: 0.0,
                width: 80.0,
                height: 40.0,
                properties: PropertyBag::new(),
                ports: vec![Port { id: "out".into(), kind: "Connector".into(), direction: PortDirection::Out, properties: PropertyBag::new() }],
            }),
            create_node(Node {
                id: "y-10".into(),
                kind: "Piece".into(),
                name: "y".into(),
                x: 1200.0,
                y: 80.0,
                width: 80.0,
                height: 40.0,
                properties: PropertyBag::new(),
                ports: vec![Port { id: "in".into(), kind: "Connector".into(), direction: PortDirection::In, properties: PropertyBag::new() }],
            }),
            create_edge(Edge { id: "e-batch".into(), kind: "Connection".into(), source: port_key("x-9", "out"), target: port_key("y-10", "in"), properties: PropertyBag::new() }),
        ],
    )
    .await
    .expect("batch create edge");
    let projection = store.snapshot().expect("projection");
    assert_eq!(projection.nodes().len(), 11);
    assert_eq!(projection.edges().len(), 2);
}

#[semio_framework_async_macros::async_test]
async fn graph_op_rejects_unknown_node_kind() {
    let fixture = mini_fixture();
    let err =
        validate_trinity_graph_operation(&create_node(Node { id: "new".into(), kind: "Piece2".into(), name: "x".into(), x: 0.0, y: 0.0, width: 80.0, height: 40.0, properties: PropertyBag::new(), ports: vec![] }), &fixture).expect_err("unknown kind");
    assert!(err.to_string().contains("unknown node kind"));
}

#[semio_framework_async_macros::async_test]
async fn from_json_rejects_wrong_schema() {
    let json = r#"{"schema":"bogus","name":"x","camera":{"x":0,"y":0,"zoom":1},"nodes":[],"edges":[]}"#;
    let err = JackSnapshot::from_json(json).expect_err("schema mismatch");
    assert!(err.to_string().contains("expected schema trinity.graph"));
}

#[semio_framework_async_macros::async_test]
async fn resolve_manifest_errors_when_missing_and_empty() {
    let mut fixture = JackSnapshot::with_content(JackSnapshot::SCHEMA.into(), "x".into(), None, Manifest::default(), Camera::default(), vec![], vec![], None);
    let err = fixture.resolve_manifest().expect_err("missing manifest");
    assert!(matches!(err, TrinityRamError::ManifestMissing));
}

#[semio_framework_async_macros::async_test]
async fn resolve_manifest_errors_on_unknown_id() {
    let mut fixture = JackSnapshot::with_content(JackSnapshot::SCHEMA.into(), "x".into(), Some("nope".into()), Manifest::default(), Camera::default(), vec![], vec![], None);
    let err = fixture.resolve_manifest().expect_err("unknown manifest id");
    assert!(err.to_string().contains("unknown manifest id nope"));
}

#[semio_framework_async_macros::async_test]
async fn graph_from_fixture_rejects_port_kind_not_declared_on_node_kind() {
    let fixture = mini_fixture();
    let mut nodes = fixture.nodes();
    nodes[0].ports.push(Port { id: "bad".into(), kind: "core circular bottom".into(), direction: PortDirection::Out, properties: PropertyBag::new() });
    let fixture = JackSnapshot::with_content(fixture.schema.clone(), fixture.name.clone(), fixture.manifest_id.clone(), fixture.manifest.clone(), fixture.camera.clone(), nodes, fixture.edges(), fixture.root_node_id.clone());
    let err = Graph::from_fixture(fixture).expect_err("undeclared port kind");
    assert!(matches!(err, TrinityRamError::PortKindNotDeclaredOnFixture { .. }));
    assert!(err.to_string().contains("root"));
}

#[semio_framework_async_macros::async_test]
async fn graph_accessors_and_mutators() {
    let mut g = Graph::from_fixture(mini_fixture()).unwrap();
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
    let mut g = Graph::from_fixture(mini_fixture()).unwrap();
    assert!(g.remove_node("root"));
    assert!(g.edges.is_empty());
    assert!(g.nodes.contains_key("child"));
    assert!(g.root_node_id.is_none());
    assert!(!g.remove_node("root"));
}

#[semio_framework_async_macros::async_test]
async fn graph_set_property_success_and_errors() {
    let mut g = Graph::from_fixture(mini_fixture()).unwrap();
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
async fn graph_to_fixture_and_fixture_json() {
    let g = Graph::from_fixture(mini_fixture()).unwrap();
    let fixture = g.to_fixture();
    assert_eq!(fixture.nodes().len(), 2);
    assert_eq!(fixture.manifest_id.as_deref(), Some("nakagin"));
    let json = g.fixture_json().expect("fixture json");
    assert!(json.contains("\"schema\""));
}

#[semio_framework_async_macros::async_test]
async fn subgraph_fixture_filters_entities_and_keeps_root_when_included() {
    let g = Graph::from_fixture(mini_fixture()).unwrap();
    let node_ids: BTreeSet<String> = ["root".to_string()].into_iter().collect();
    let sub = g.subgraph_fixture(&node_ids, &BTreeSet::new());
    assert_eq!(sub.nodes().len(), 1);
    assert!(sub.edges().is_empty());
    assert_eq!(sub.root_node_id.as_deref(), Some("root"));
    assert!(sub.name.contains("subgraph"));
}

#[semio_framework_async_macros::async_test]
async fn subgraph_fixture_drops_root_when_not_included() {
    let g = Graph::from_fixture(mini_fixture()).unwrap();
    let node_ids: BTreeSet<String> = ["child".to_string()].into_iter().collect();
    let sub = g.subgraph_fixture(&node_ids, &BTreeSet::new());
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
