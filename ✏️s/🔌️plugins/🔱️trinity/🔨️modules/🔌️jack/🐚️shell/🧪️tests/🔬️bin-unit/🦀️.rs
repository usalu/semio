use semio_s_artifact_trinity_jack::JackWorkingScene;
use super::*;
use semio_s_artifact_trinity_jack::{Camera, JackSnapshot, Manifest, Node, Port, PortDirection, PropertyBag};

fn mini_json() -> String {
    let fixture = JackSnapshot::with_content(JackSnapshot::SCHEMA.into(), "mini".into(), Some("nakagin".into()), Manifest::nakagin_default(), Camera::default(), JackWorkingScene { nodes: vec![Node {
            id: "root".into(),
            kind: "Piece".into(),
            name: "core".into(),
            x: 0.0,
            y: 0.0,
            width: 80.0,
            height: 40.0,
            properties: PropertyBag::new(),
            ports: vec![Port { id: "out".into(), kind: "Connector".into(), direction: PortDirection::Out, properties: PropertyBag::new() }],
        }], edges: vec![] }, Some("root".into()));
    fixture.to_json().unwrap()
}

#[semio_framework_async_macros::async_test]
async fn shell_loads_fixture() {
    let mut graph = Graph::load_json(&mini_json()).unwrap();
    let result = run(&mut graph, "MATCH (a:Piece) RETURN a.name").unwrap();
    assert_eq!(result.rows.len(), 1);
}
