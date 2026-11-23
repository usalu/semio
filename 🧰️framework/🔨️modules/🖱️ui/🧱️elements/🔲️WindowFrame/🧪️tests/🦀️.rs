//! 🧪️ `frame.cases.json` against the Rust geometry, with `kurbo` as the independent rectangle oracle.

use super::*;
use serde_json::Value;

fn num(value: &Value) -> f32 {
    value.as_f64().unwrap() as f32
}

fn edge(value: &Value) -> WindowFrameEdge {
    WindowFrameEdge::new(num(&value["depth"]), value["chips"].as_array().unwrap().iter().map(|chip| WindowFrameChip::new(num(&chip["left"]), num(&chip["right"]))).collect())
}

fn cases() -> Vec<Value> {
    serde_json::from_str::<Vec<Value>>(include_str!("../frame.cases.json")).unwrap()
}

fn frame_of(case: &Value) -> WindowFrame {
    let m = &case["metrics"];
    WindowFrame::new(Rect::new(0.0, 0.0, num(&m["width"]), num(&m["height"])), edge(&m["top"]), edge(&m["bottom"]))
}

fn oracle(rect: Rect) -> kurbo::Rect {
    kurbo::Rect::new(rect.x as f64, rect.y as f64, (rect.x + rect.w) as f64, (rect.y + rect.h) as f64)
}

#[test]
fn frame_cases_hold() {
    for case in cases() {
        let name = case["name"].as_str().unwrap();
        let expected = &case["expected"];
        let frame = frame_of(&case);
        let f = &expected["frame"];
        assert_eq!(frame.content_clip_rect(), Rect::new(num(&f["x"]), num(&f["y"]), num(&f["width"]), num(&f["height"])), "{name}: frame");
        let regions = frame.chip_regions();
        let wanted = expected["chipRegions"].as_array().unwrap();
        assert_eq!(regions.len(), wanted.len(), "{name}: chip count");
        for (region, want) in regions.iter().zip(wanted) {
            assert_eq!(region.rect, Rect::new(num(&want["x"]), num(&want["y"]), num(&want["width"]), num(&want["height"])), "{name}: chip rect");
            assert_eq!(region.dock, if want["dock"] == "top" { WindowFrameDock::Top } else { WindowFrameDock::Bottom }, "{name}: chip dock");
        }
        let c = &expected["safeClearances"];
        assert_eq!(frame.safe_clearances(), WindowFrameSafeClearances { top: num(&c["top"]), right: num(&c["right"]), bottom: num(&c["bottom"]), left: num(&c["left"]) }, "{name}: clearances");
        assert_eq!(frame.border_path(WINDOW_FRAME_PATH_INSET), expected["borderPath"].as_str().unwrap(), "{name}: border path");
        for probe in expected["contains"].as_array().unwrap() {
            assert_eq!(frame.contains(num(&probe["x"]), num(&probe["y"])), probe["inside"].as_bool().unwrap(), "{name}: contains {probe}");
        }
    }
}

#[test]
fn chips_lie_inside_the_frame_and_never_overlap_by_the_independent_oracle() {
    for case in cases() {
        let name = case["name"].as_str().unwrap();
        let frame = frame_of(&case);
        let outline = oracle(frame.bounds);
        let regions = frame.chip_regions();
        for region in &regions {
            let chip = oracle(region.rect);
            assert_eq!(outline.union(chip), outline, "{name}: chip leaves the frame");
        }
        for (i, a) in regions.iter().enumerate() {
            for b in &regions[i + 1..] {
                assert_eq!(oracle(a.rect).intersect(oracle(b.rect)).area(), 0.0, "{name}: chips overlap");
            }
        }
    }
}

#[test]
fn every_point_of_the_frame_is_content_or_a_chip_with_no_cutouts() {
    for case in cases() {
        let name = case["name"].as_str().unwrap();
        let frame = frame_of(&case);
        let b = frame.bounds;
        let (w, h) = (b.w, b.h);
        for iy in 0..=8 {
            for ix in 0..=8 {
                let (x, y) = (w * ix as f32 / 8.0, h * iy as f32 / 8.0);
                assert!(frame.contains(x, y), "{name}: ({x},{y}) is a cutout");
            }
        }
        for region in frame.chip_regions() {
            let (cx, cy) = (region.rect.x + region.rect.w / 2.0, region.rect.y + region.rect.h / 2.0);
            assert_eq!(frame.chip_at(cx, cy).map(|hit| hit.rect), Some(region.rect), "{name}: chip hit");
        }
    }
}

#[test]
fn clearance_rect_is_the_frame_minus_the_chip_rows() {
    let frame = WindowFrame::from_measured_top(Rect::new(10.0, 20.0, 300.0, 200.0), 80.0, 60.0, 32.0);
    assert_eq!(frame.content_clip_rect(), Rect::new(10.0, 20.0, 300.0, 200.0));
    assert_eq!(frame.clearance_rect(), Rect::new(10.0, 52.0, 300.0, 168.0));
    assert!(frame.contains(150.0, 30.0));
    assert_eq!(frame.chip_at(150.0, 30.0), None);
    assert_eq!(frame.chip_regions().iter().map(|r| r.rect).collect::<Vec<_>>(), vec![Rect::new(10.0, 20.0, 80.0, 32.0), Rect::new(250.0, 20.0, 60.0, 32.0)]);
}
