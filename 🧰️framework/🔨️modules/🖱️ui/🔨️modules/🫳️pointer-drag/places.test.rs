use super::*;
use kurbo::{Point, Rect};
use serde_json::Value;

fn cases() -> Value {
    serde_json::from_str(include_str!("places.cases.json")).expect("the shared drag cases parse")
}

fn number(value: &Value) -> f32 {
    value.as_f64().expect("a number") as f32
}

fn rect(value: &Value) -> DropRect {
    DropRect { left: number(&value["left"]), top: number(&value["top"]), right: number(&value["right"]), bottom: number(&value["bottom"]) }
}

fn point(value: &Value) -> [f32; 2] {
    [number(&value["x"]), number(&value["y"])]
}

fn band(name: &str) -> TreeBand {
    match name {
        "before" => TreeBand::Before,
        "after" => TreeBand::After,
        "inside" => TreeBand::Inside,
        other => panic!("unknown band {other}"),
    }
}

fn kurbo_rect(rect: DropRect) -> Rect {
    Rect::new(rect.left as f64, rect.top as f64, rect.right as f64, rect.bottom as f64)
}

fn encloses(outer: DropRect, inner: DropRect) -> bool {
    kurbo_rect(outer).contains_rect(kurbo_rect(inner))
}

#[test]
fn the_tints_are_the_shared_percentages() {
    let tint = &cases()["tint"];
    assert_eq!((DROP_TINT_AVAILABLE_PERCENT, DROP_TINT_ACTIVE_PERCENT), (number(&tint["available"]), number(&tint["active"])));
    assert_eq!((DropPlaceState::Available.tint_percent(), DropPlaceState::Active.tint_percent()), (10.0, 32.0));
    assert!(DropPlaceState::Available.tint_alpha() < DropPlaceState::Active.tint_alpha());
}

#[test]
fn the_shared_cases_pick_the_place_under_the_pointer() {
    for case in cases()["places"].as_array().expect("places") {
        let owned: Vec<(String, DropRect)> = case["places"].as_array().unwrap().iter().map(|place| (place["id"].as_str().unwrap().to_string(), rect(&place["rect"]))).collect();
        let places: Vec<DropPlace> = owned.iter().map(|(id, rect)| DropPlace { id, rect: *rect }).collect();
        let [x, y] = point(&case["point"]);
        assert_eq!(drop_place_at(&places, x, y), case["active"].as_str(), "{case}");
    }
}

#[test]
fn the_hit_rule_agrees_with_kurbo_inside_the_places() {
    let places = [("outer", DropRect { left: 0.0, top: 0.0, right: 100.0, bottom: 100.0 }), ("inner", DropRect { left: 20.0, top: 20.0, right: 60.0, bottom: 60.0 }), ("side", DropRect { left: 50.0, top: 10.0, right: 90.0, bottom: 90.0 })];
    let list: Vec<DropPlace> = places.iter().map(|(id, rect)| DropPlace { id, rect: *rect }).collect();
    for x in (1..100).step_by(7) {
        for y in (1..100).step_by(5) {
            let (px, py) = (x as f32 + 0.5, y as f32 + 0.5);
            let oracle = places.iter().filter(|(_, rect)| kurbo_rect(*rect).contains(Point::new(px as f64, py as f64))).min_by(|(_, a), (_, b)| kurbo_rect(*a).area().total_cmp(&kurbo_rect(*b).area())).map(|(id, _)| *id);
            assert_eq!(drop_place_at(&list, px, py), oracle, "({px}, {py})");
        }
    }
}

#[test]
fn the_shared_cases_place_the_ghost_and_start_the_drag() {
    let cases = cases();
    assert_eq!(DRAG_GHOST_OFFSET, point(&cases["offset"]));
    for case in cases["ghost"].as_array().expect("ghost") {
        let [x, y] = point(&case["point"]);
        assert_eq!(drag_ghost_position(x, y), point(&case["position"]), "{case}");
    }
    for case in cases["threshold"].as_array().expect("threshold") {
        assert_eq!(drag_starts(point(&case["from"]), point(&case["to"])), case["begins"].as_bool().unwrap(), "{case}");
    }
    assert_eq!(DRAG_START_THRESHOLD_PX, 4.0);
    for case in cases["keys"].as_array().expect("keys") {
        assert_eq!(drag_cancel_key(case["key"].as_str().unwrap()), case["cancels"].as_bool().unwrap(), "{case}");
    }
}

#[test]
fn the_shared_cases_cut_the_insert_slot() {
    for case in cases()["insertSlots"].as_array().expect("insert slots") {
        let items: Vec<DropRect> = case["items"].as_array().unwrap().iter().map(rect).collect();
        let row = rect(&case["row"]);
        let slot = insert_slot(row, &items, case["index"].as_i64().unwrap());
        assert_eq!(slot, rect(&case["slot"]), "{case}");
        assert!(encloses(row, slot), "{case}");
    }
}

#[test]
fn the_shared_cases_cut_the_tree_band() {
    for case in cases()["treeBands"].as_array().expect("tree bands") {
        let row = rect(&case["row"]);
        let cut = tree_drop_band(row, band(case["position"].as_str().unwrap()));
        assert_eq!(cut, rect(&case["band"]), "{case}");
        assert!(encloses(row, cut), "{case}");
    }
}

#[test]
fn frames_and_edges_are_the_same_rectangle() {
    let rect = DropRect::from_frame([10.0, 20.0, 30.0, 40.0]);
    assert_eq!(rect, DropRect { left: 10.0, top: 20.0, right: 40.0, bottom: 60.0 });
    assert_eq!(rect.frame(), [10.0, 20.0, 30.0, 40.0]);
}
