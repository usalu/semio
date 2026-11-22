use semio_ui_contract::window_layout_ops::*;
use semio_ui_contract::WindowLayoutNode;
use serde_json::{from_value, json, to_value, Value};

fn corpus() -> Value {
    serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).expect("window layout corpus")
}

fn node(value: &Value) -> WindowLayoutNode {
    from_value(value.clone()).expect("layout node")
}

#[test]
fn corpus_operations_match_expected_layouts() {
    let corpus = corpus();
    for case in corpus["operations"].as_array().expect("operations") {
        let op: WindowLayoutOp = from_value(case["op"].clone()).expect("operation");
        let actual = to_value(apply_window_layout_op(&node(&case["layout"]), &op)).expect("serialize");
        assert_eq!(actual, case["expect"], "{}", case["id"]);
    }
}

#[test]
fn corpus_queries_match_expected_results() {
    let corpus = corpus();
    for case in corpus["queries"].as_array().expect("queries") {
        let layout = node(&case["layout"]);
        let path = || from_value::<Vec<usize>>(case["path"].clone()).expect("path");
        let actual = match case["query"].as_str().expect("query") {
            "stackPathOf" => json!(stack_path_of(&layout, case["windowId"].as_str().expect("window id"))),
            "collectWindowIds" => json!(collect_window_ids(&layout)),
            "readAt" => json!(read_at(&layout, &path())),
            other => panic!("unknown query {other}"),
        };
        assert_eq!(actual, case["expect"], "{}", case["id"]);
    }
}

#[test]
fn corpus_builders_match_expected_layouts() {
    let corpus = corpus();
    for case in corpus["builders"].as_array().expect("builders") {
        let ids: Vec<String> = from_value(case["windowIds"].clone()).expect("ids");
        assert_eq!(to_value(even_layout(&ids)).expect("serialize"), case["expect"], "{}", case["id"]);
    }
}

#[test]
fn corpus_layouts_roundtrip_through_the_contract() {
    let corpus = corpus();
    for case in corpus["operations"].as_array().expect("operations") {
        assert_eq!(to_value(node(&case["layout"])).expect("serialize"), case["layout"], "{}", case["id"]);
    }
}

#[test]
fn builders_produce_schema_valid_layouts_and_the_validator_rejects_violations() {
    use semio_ui_contract::Axis;
    let ids: Vec<String> = ["a", "b", "c"].iter().map(|id| id.to_string()).collect();
    for count in 0..=3 {
        let slice = &ids[..count];
        assert_eq!(validate_window_layout(&even_layout(slice)), Ok(()), "even {count}");
        assert_eq!(validate_window_layout(&create_default_layout(slice, Axis::Horizontal, None, None).root), Ok(()), "default {count}");
        assert_eq!(validate_window_layout(&create_stack_layout(slice, None).root), Ok(()), "stack {count}");
    }
    let window = || create_window_layout("a", None, None, None);
    let stack = |size: Option<f64>| WindowLayoutNode::Stack { size, active_window_id: None, children: vec![window()] };
    let one_child = WindowLayoutNode::Split { axis: Axis::Horizontal, size: None, children: vec![stack(None)] };
    assert!(validate_window_layout(&one_child).is_err(), "a split needs two children");
    assert!(validate_window_layout(&window()).is_err(), "a window is never a root");
    assert!(validate_window_layout(&stack(Some(0.0))).is_err() && validate_window_layout(&stack(Some(40.0))).is_err(), "sizes are fractions in (0, 1]");
    let window_in_split = WindowLayoutNode::Split { axis: Axis::Vertical, size: None, children: vec![stack(None), window()] };
    assert!(validate_window_layout(&window_in_split).is_err(), "split children are stacks or splits");
    let nested = WindowLayoutNode::Split { axis: Axis::Vertical, size: Some(0.5), children: vec![stack(Some(0.5)), stack(Some(0.5))] };
    assert_eq!(validate_window_layout(&nested), Ok(()));
}
