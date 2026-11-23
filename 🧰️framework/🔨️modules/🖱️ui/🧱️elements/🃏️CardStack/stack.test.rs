use super::*;
use semio_ui_theme::{Level, Theme};
use semio_ui_draw_contract::SharedBorderState;
use serde_json::Value;

const EPSILON: f64 = 1e-9;

fn cases() -> Value {
    serde_json::from_str(include_str!("stack.cases.json")).unwrap()
}

fn rect(value: &Value) -> [f64; 4] {
    let values = value.as_array().unwrap();
    [values[0].as_f64().unwrap(), values[1].as_f64().unwrap(), values[2].as_f64().unwrap(), values[3].as_f64().unwrap()]
}

fn cards(case: &Value) -> Vec<CardStackCard> {
    case["cards"].as_array().unwrap().iter().map(|card| CardStackCard { height: card["height"].as_f64().unwrap(), row: card["row"].as_f64().unwrap(), top_chip: card["topChip"].as_f64().unwrap(), bottom_chip: card["bottomChip"].as_f64().unwrap() }).collect()
}

fn frames(case: &Value) -> Vec<CardStackFrame> {
    card_stack_frames(case["left"].as_f64().unwrap(), case["top"].as_f64().unwrap(), case["width"].as_f64().unwrap(), &cards(case))
}

fn assert_rect(actual: [f64; 4], expected: [f64; 4], name: &str) {
    assert!(actual.iter().zip(expected).all(|(a, b)| (a - b).abs() < EPSILON), "{name}: {actual:?} != {expected:?}");
}

#[test]
fn frames_and_chips_match_the_language_neutral_cases() {
    let data = cases();
    for case in data["cases"].as_array().unwrap() {
        let name = case["name"].as_str().unwrap();
        let expected = case["expected"]["frames"].as_array().unwrap();
        let actual = frames(case);
        assert_eq!(actual.len(), expected.len(), "{name}");
        for (frame, expected) in actual.iter().zip(expected) {
            assert_rect(frame.rect, rect(&expected["rect"]), name);
            let chips = expected["chips"].as_array().unwrap();
            assert_eq!(frame.chips.len(), chips.len(), "{name}");
            for (chip, expected) in frame.chips.iter().zip(chips) {
                assert_rect(chip.rect, rect(&expected["rect"]), name);
                assert_eq!(if chip.dock == CardStackDock::Top { "top" } else { "bottom" }, expected["dock"].as_str().unwrap(), "{name}");
                let inner: Vec<bool> = expected["innerEdges"].as_array().unwrap().iter().map(|edge| edge.as_bool().unwrap()).collect();
                assert_eq!(chip.inner_edges.to_vec(), inner, "{name}");
            }
        }
    }
}

#[test]
fn chips_stay_inside_their_frame_and_frames_tile_without_gap_or_overlap() {
    let data = cases();
    for case in data["cases"].as_array().unwrap() {
        let name = case["name"].as_str().unwrap();
        let frames = frames(case);
        let (left, top, width) = (case["left"].as_f64().unwrap(), case["top"].as_f64().unwrap(), case["width"].as_f64().unwrap());
        let total: f64 = frames.iter().map(|frame| frame.rect[3]).sum();
        let (columns, rows) = (width as usize, total as usize);
        let mut covered = vec![0u8; columns * rows];
        for frame in &frames {
            for chip in &frame.chips {
                assert!(chip.rect[0] >= frame.rect[0] - EPSILON && chip.rect[1] >= frame.rect[1] - EPSILON && chip.rect[0] + chip.rect[2] <= frame.rect[0] + frame.rect[2] + EPSILON && chip.rect[1] + chip.rect[3] <= frame.rect[1] + frame.rect[3] + EPSILON, "{name}: chip leaves its frame");
            }
            for y in (frame.rect[1] - top) as usize..(frame.rect[1] - top + frame.rect[3]) as usize {
                for x in (frame.rect[0] - left) as usize..(frame.rect[0] - left + frame.rect[2]) as usize {
                    covered[y * columns + x] += 1;
                }
            }
        }
        assert!(covered.iter().all(|count| *count == 1), "{name}: every pixel of the stack belongs to exactly one frame");
    }
}

#[test]
fn a_frame_contains_exactly_its_rectangle() {
    let frame = &card_stack_frames(10.0, 20.0, 200.0, &[CardStackCard { height: 80.0, row: 28.0, top_chip: 90.0, bottom_chip: 0.0 }])[0];
    assert!(card_frame_contains(frame, 10.0, 20.0));
    assert!(card_frame_contains(frame, 210.0, 100.0));
    assert!(card_frame_contains(frame, 150.0, 24.0), "the gap beside a chip is content, not a cutout");
    assert!(!card_frame_contains(frame, 9.9, 50.0));
    assert!(!card_frame_contains(frame, 100.0, 100.1));
}

fn painted(draw: &DrawList) -> Vec<[f32; 4]> {
    draw.layers.iter().flat_map(|layer| &layer.ui_instances).filter(|item| item.rect[2] > 0.0 && item.rect[3] > 0.0).map(|item| item.rect).collect()
}

fn coverage(painted: &[[f32; 4]], y: f32, low: f32, high: f32) -> f32 {
    painted.iter().filter(|rect| rect[1] == y && rect[3] == 1.0).map(|rect| (rect[0] + rect[2]).min(high) - rect[0].max(low)).filter(|width| *width > 0.0).sum()
}

#[test]
fn the_seam_between_abutting_cards_is_one_painted_line() {
    let data = cases();
    let case = data["cases"].as_array().unwrap().iter().find(|case| case["name"].as_str().unwrap().starts_with("cards abut")).unwrap();
    let mut draw = DrawList::default();
    let color = Rgba { r: 0.2, g: 0.3, b: 0.4, a: 1.0 };
    let style = Theme::default().glass(Level::Window);
    paint_card_stack(&mut draw, &frames(case), 1.0, style, |_| (color, SharedBorderState::Normal));
    while !draw.finalize_shared_borders_step().unwrap() {}
    let lines = painted(&draw);
    assert!((coverage(&lines, 99.0, 10.0, 210.0) + coverage(&lines, 100.0, 10.0, 210.0) - 200.0).abs() < 0.001, "the seam at y=100 is painted once");
    assert!((coverage(&lines, 159.0, 10.0, 210.0) + coverage(&lines, 160.0, 10.0, 210.0) - 200.0).abs() < 0.001, "the seam at y=160 is painted once");
}

#[test]
fn chip_glass_sits_inside_its_frame_above_the_frame_lines() {
    let data = cases();
    let case = data["cases"].as_array().unwrap().iter().find(|case| case["name"].as_str().unwrap().starts_with("title and footer")).unwrap();
    let frames = frames(case);
    let mut draw = DrawList::default();
    let color = Rgba { r: 0.2, g: 0.3, b: 0.4, a: 1.0 };
    let style = Theme::default().glass(Level::Window);
    paint_card_stack(&mut draw, &frames, 1.0, style, |_| (color, SharedBorderState::Normal));
    assert_eq!(draw.glass_regions.len(), 2);
    let frame = frames[0].rect;
    for (region, chip) in draw.glass_regions.iter().zip(&frames[0].chips) {
        let expected = [chip.rect[0] as f32, chip.rect[1] as f32, chip.rect[2] as f32, chip.rect[3] as f32];
        assert_eq!(region.rect, expected);
        assert!(region.rect[0] as f64 >= frame[0] && (region.rect[0] + region.rect[2]) as f64 <= frame[0] + frame[2] && region.rect[1] as f64 >= frame[1] && (region.rect[1] + region.rect[3]) as f64 <= frame[1] + frame[3]);
        assert!(region.alpha < 1.0 && region.blur_px > 0.0, "the content stays visible through the chip");
    }
    assert!(draw.layers.iter().all(|layer| layer.clip.is_none()), "no clip region and no mask for a frame");
}
