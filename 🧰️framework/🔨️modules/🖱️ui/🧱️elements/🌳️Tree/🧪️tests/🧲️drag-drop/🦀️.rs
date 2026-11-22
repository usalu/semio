use super::*;
use serde_json::Value;

fn fixture() -> Value {
    serde_json::from_str(include_str!("../🧫️fixtures/🧲️drag-drop/🔣️.json")).expect("the neutral tree-drop vectors parse")
}

fn position(name: &str) -> TreeDrop {
    match name {
        "before" => TreeDrop::Before,
        "after" => TreeDrop::After,
        "inside" => TreeDrop::Inside,
        other => panic!("unknown position {other}"),
    }
}

fn kind(name: &str) -> TreeDragKind {
    match name {
        "rowReorder" => TreeDragKind::RowReorder,
        "plain" => TreeDragKind::Plain,
        "other" => TreeDragKind::Other,
        other => panic!("unknown kind {other}"),
    }
}

fn rect(value: &Value) -> [f32; 4] {
    let numbers = value.as_array().expect("a rect");
    [0, 1, 2, 3].map(|index| numbers[index].as_f64().expect("a number") as f32)
}

#[test]
fn the_shared_vectors_resolve_the_drop_position() {
    for vector in fixture()["positions"].as_array().expect("positions") {
        let (top, height, y) = (vector["top"].as_f64().unwrap() as f32, vector["height"].as_f64().unwrap() as f32, vector["y"].as_f64().unwrap() as f32);
        assert_eq!(tree_drop_position(top, height, y), position(vector["position"].as_str().unwrap()), "{vector}");
    }
}

#[test]
fn the_shared_vectors_classify_the_drag_payload() {
    for vector in fixture()["kinds"].as_array().expect("kinds") {
        let flag = |key: &str| vector[key].as_bool().unwrap();
        assert_eq!(tree_drag_kind(flag("rowReorder"), flag("defaultMime"), flag("foreign")), kind(vector["kind"].as_str().unwrap()), "{vector}");
    }
}

#[test]
fn the_shared_vectors_pick_the_preview_shape() {
    for vector in fixture()["previews"].as_array().expect("previews") {
        let flag = |key: &str| vector[key].as_bool().unwrap();
        let shape = tree_drop_preview(kind(vector["kind"].as_str().unwrap()), position(vector["position"].as_str().unwrap()), flag("accepts"), flag("source"), rect(&vector["frame"]));
        match (vector["shape"].as_str(), shape) {
            (None, None) => {}
            (Some("line"), Some(TreeDropShape::Line(line))) => assert_eq!(line, rect(&vector["rect"]), "{vector}"),
            (Some("box"), Some(TreeDropShape::Box(frame))) => assert_eq!(frame, rect(&vector["rect"]), "{vector}"),
            (want, got) => panic!("{vector}: wanted {want:?}, got {got:?}"),
        }
    }
}

#[test]
fn the_nest_box_border_and_alphas_are_react_tokens() {
    let law = &fixture()["box"];
    assert_eq!(tree_drop_box_border(rect(&law["frame"])).to_vec(), law["border"].as_array().unwrap().iter().map(rect).collect::<Vec<_>>());
    assert_eq!(TREE_DROP_BOX_BORDER_ALPHA as f64, (law["borderAlpha"].as_f64().unwrap() as f32) as f64);
    assert_eq!(TREE_DROP_BOX_FILL_ALPHA, law["fillAlpha"].as_f64().unwrap() as f32);
    assert_eq!(TREE_DROP_BOX_BORDER, law["borderWidth"].as_f64().unwrap() as f32);
}

#[test]
fn only_edge_drops_have_a_wire_spelling() {
    assert_eq!((TreeDrop::Before.wire(), TreeDrop::After.wire(), TreeDrop::Inside.wire()), (Some("before"), Some("after"), None));
}

#[test]
fn the_shared_vectors_decide_the_palette_drag() {
    let law = &fixture()["palette"];
    for vector in law["payloads"].as_array().expect("payloads") {
        assert_eq!(tree_palette_payload(vector["payload"].as_str()), vector["starts"].as_bool().unwrap(), "{vector}");
    }
    for vector in law["travel"].as_array().expect("travel") {
        assert_eq!(tree_palette_drag_begins(vector["dx"].as_f64().unwrap() as f32, vector["dy"].as_f64().unwrap() as f32), vector["begins"].as_bool().unwrap(), "{vector}");
    }
    assert_eq!(TREE_CATALOGUE_DRAG_MIME, law["mime"].as_str().unwrap());
}
