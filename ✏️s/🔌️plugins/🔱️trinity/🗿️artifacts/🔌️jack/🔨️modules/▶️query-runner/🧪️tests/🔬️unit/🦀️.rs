use super::{run_json, run_json_controlled, Graph};
use semio_framework_value::NativeEncodeControl;

#[test]
fn refused_response_leaves_the_graph_unchanged() {
    let fixture = include_str!("../../../../🐚️shell/🧪️tests/🧫️fixtures/🔱️mini-tower/🔣️.json");
    let mut graph = Graph::load_scene_json(fixture).unwrap();
    let original = graph.clone();
    let query = "CREATE (n:Piece)";
    let mut progress = |_| true;
    let mut too_small = NativeEncodeControl::new(1, &mut progress);
    assert!(run_json_controlled(&mut graph, query, &mut too_small).is_err());
    assert_eq!(graph, original);

    let mut cancel = |_| false;
    let mut cancelled = NativeEncodeControl::new(4096, &mut cancel);
    assert!(run_json_controlled(&mut graph, query, &mut cancelled).is_err());
    assert_eq!(graph, original);

    run_json(&mut graph, query).unwrap();
    assert_eq!(graph.nodes.len(), original.nodes.len() + 1);
}
