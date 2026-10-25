use super::*;
use semio_framework_graph_manifest::{PortDirection, PropertyBag};
use semio_s_artifact_trinity_jack::standards::v1::subsets::any::schema::ast::JackQueryGraphSnapshot;
use semio_s_artifact_trinity_jack::standards::v1::subsets::any::schema::graph::{Camera, Node, Port};
use semio_s_artifact_trinity_jack::standards::v1::subsets::any::schema::manifest::Manifest;
use semio_s_artifact_trinity_jack::standards::v1::subsets::any::schema::snapshot::JackSnapshot;

fn mini_json() -> String {
    JackQueryGraphSnapshot {
        schema: JackSnapshot::SCHEMA.into(),
        name: "mini".into(),
        manifest_id: Some("nakagin".into()),
        manifest: Manifest::nakagin_default(),
        camera: Camera::default(),
        nodes: vec![Node {
            id: "root".into(),
            kind: "Piece".into(),
            name: "core".into(),
            x: 0.0,
            y: 0.0,
            width: 80.0,
            height: 40.0,
            properties: PropertyBag::new(),
            ports: vec![Port { id: "out".into(), kind: "Connector".into(), direction: PortDirection::Out, properties: PropertyBag::new() }],
        }],
        edges: vec![],
        root_node_id: Some("root".into()),
        query: String::new(),
    }
    .to_json()
    .unwrap()
}

#[semio_framework_async_macros::async_test]
async fn shell_loads_fixture() {
    let mut graph = Graph::load_scene_json(&mini_json()).unwrap();
    let result = run(&mut graph, "MATCH (a:Piece) RETURN a.name").unwrap();
    assert_eq!(result.rows.len(), 1);
}

#[semio_framework_async_macros::async_test]
async fn shell_without_a_snapshot_path_states_its_usage_in_both_languages() {
    let text = TrinityJackShellError::Usage.to_string();
    assert!(text.contains("usage:") && text.contains("Aufruf:") && text.contains("<snapshot.json>"));
}

const EXAMPLE_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../🧪️tests/🧫️fixtures/🔱️mini-tower/🔣️.json");

#[semio_framework_async_macros::async_test]
async fn shell_example_fixture_is_the_mini_snapshot_and_answers_a_query() {
    let expected = mini_json();
    if !std::path::Path::new(EXAMPLE_PATH).exists() {
        std::fs::create_dir_all(std::path::Path::new(EXAMPLE_PATH).parent().unwrap()).unwrap();
        std::fs::write(EXAMPLE_PATH, &expected).unwrap();
    }
    let stored = std::fs::read_to_string(EXAMPLE_PATH).unwrap();
    assert_eq!(stored, expected);
    let mut graph = Graph::load_scene_json(&stored).unwrap();
    let result = run(&mut graph, "MATCH (a:Piece) RETURN a.name").unwrap();
    assert_eq!(result.rows.len(), 1);
}

#[semio_framework_async_macros::async_test]
async fn the_run_launcher_defaults_to_the_example_snapshot() {
    let launcher = include_str!("../🔬️rust-package-commands/🟦️.ts");
    let relative = EXAMPLE_PATH.split_once("/../../").unwrap().1;
    assert!(launcher.contains(&format!("\"../../{relative}\"")), "{relative}");
}
