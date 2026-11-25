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
fn the_shared_vectors_pick_the_active_band() {
    for vector in fixture()["previews"].as_array().expect("previews") {
        let flag = |key: &str| vector[key].as_bool().unwrap();
        let band = tree_drop_preview(kind(vector["kind"].as_str().unwrap()), position(vector["position"].as_str().unwrap()), flag("accepts"), flag("source"), rect(&vector["frame"]));
        assert_eq!(band, vector["band"].as_array().map(|_| rect(&vector["band"])), "{vector}");
    }
}

#[test]
fn the_bands_are_the_shared_places_cases() {
    let cases: Value = serde_json::from_str(include_str!("../../../../🔨️modules/🫳️pointer-drag/places.cases.json")).expect("the shared drag cases parse");
    for case in cases["treeBands"].as_array().expect("tree bands") {
        let corners = |value: &Value| ["left", "top", "right", "bottom"].map(|key| value[key].as_f64().unwrap() as f32);
        let [left, top, right, bottom] = corners(&case["row"]);
        let [band_left, band_top, band_right, band_bottom] = corners(&case["band"]);
        let drop = position(case["position"].as_str().unwrap());
        let band = tree_drop_preview(TreeDragKind::Plain, drop, true, false, [left, top, right - left, bottom - top]).expect("a plain drag previews every position");
        assert_eq!(band, [band_left, band_top, band_right - band_left, band_bottom - band_top], "{case}");
        assert_eq!(tree_band(drop), tree_band(position(case["position"].as_str().unwrap())));
    }
}

#[test]
fn an_edge_band_is_a_quarter_row_and_the_edge_fraction_is_the_same_quarter() {
    let frame = [0.0, 100.0, 200.0, 24.0];
    for drop in [TreeDrop::Before, TreeDrop::After] {
        let band = tree_drop_preview(TreeDragKind::Plain, drop, true, false, frame).unwrap();
        assert_eq!(band[3], frame[3] * TREE_DROP_EDGE_FRACTION);
    }
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
