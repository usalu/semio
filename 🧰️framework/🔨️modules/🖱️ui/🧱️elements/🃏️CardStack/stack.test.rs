use super::*;
use semio_ui_draw_contract::resolve_shared_border_segments;
use semio_ui_draw_contract::SharedBorderSegment;
use serde_json::Value;

fn fixture() -> Value {
    serde_json::from_str(include_str!("../🗂️WindowChrome/compositor.cases.json")).unwrap()
}

fn stack_cases(cases: &Value) -> impl Iterator<Item = &Value> {
    cases["interlock"].as_array().unwrap().iter().filter(|case| {
        let windows = case["windows"].as_array().unwrap();
        windows.iter().all(|window| window["left"] == windows[0]["left"] && window["width"] == windows[0]["width"])
    })
}

fn edge(value: &Value) -> CardStackEdge {
    CardStackEdge { depth: value["depth"].as_f64().unwrap(), chips: value["chips"].as_array().unwrap().iter().map(|chip| CardBandSpan { left: chip["left"].as_f64().unwrap(), right: chip["right"].as_f64().unwrap() }).collect() }
}

fn card(window: &Value) -> CardStackCard {
    CardStackCard { height: window["height"].as_f64().unwrap(), top: edge(&window["topEdge"]), bottom: edge(&window["bottomEdge"]) }
}

#[test]
fn a_stack_of_the_interlock_vectors_places_each_card_on_its_neighbours_bottom_line() {
    let cases = fixture();
    let mut stacks = 0;
    for case in stack_cases(&cases) {
        let windows = case["windows"].as_array().unwrap();
        stacks += 1;
        let cards: Vec<_> = windows.iter().map(card).collect();
        let tops: Vec<f64> = layout_card_stack(&cards).iter().map(|placement| placement.top).collect();
        let expected: Vec<f64> = windows.iter().map(|window| window["top"].as_f64().unwrap()).collect();
        assert_eq!(tops, expected, "{}", case["name"]);
    }
    assert!(stacks >= 2);
}

#[test]
fn interlocked_outlines_paint_every_vector_line_once_and_straight() {
    let cases = fixture();
    for case in stack_cases(&cases) {
        let mut input = Vec::new();
        for (owner, window) in case["windows"].as_array().unwrap().iter().enumerate() {
            let placement = CardStackPlacement { top: window["top"].as_f64().unwrap(), height: window["height"].as_f64().unwrap(), top_depth: window["topEdge"]["depth"].as_f64().unwrap(), bottom_depth: window["bottomEdge"]["depth"].as_f64().unwrap() };
            for (from, to) in card_outline(window["left"].as_f64().unwrap(), window["width"].as_f64().unwrap(), placement, &card(window)) {
                input.push(SharedBorderSegment { id: input.len(), owner, from, to, state: SharedBorderState::Normal });
            }
        }
        let resolved = resolve_shared_border_segments(&input).unwrap();
        for line in case["lines"].as_array().unwrap() {
            let (at, low, high) = (line["at"].as_f64().unwrap(), line["from"].as_f64().unwrap(), line["to"].as_f64().unwrap());
            let horizontal = line["axis"] == "h";
            let mut spans: Vec<(f64, f64, usize)> = resolved
                .iter()
                .filter_map(|edge| {
                    let (from, to) = (edge.segment.from, edge.segment.to);
                    let (normal, along) = if horizontal { ((from[1], to[1]), (from[0], to[0])) } else { ((from[0], to[0]), (from[1], to[1])) };
                    ((normal.0 - at).abs() < 1e-6 && (normal.1 - at).abs() < 1e-6).then(|| (along.0.min(along.1).max(low), along.0.max(along.1).min(high), edge.contributors.len()))
                })
                .filter(|span| span.1 - span.0 > 1e-6)
                .collect();
            spans.sort_by(|a, b| a.0.total_cmp(&b.0));
            let mut reach = low;
            for (start, end, _) in &spans {
                assert!(*start >= reach - 1e-6, "{}: line {line} is painted twice over {start}", case["name"]);
                assert!((*start - reach).abs() < 1e-6, "{}: line {line} has a gap before {start}", case["name"]);
                reach = *end;
            }
            assert!((reach - high).abs() < 1e-6, "{}: line {line} stops at {reach}", case["name"]);
            assert!(spans.iter().any(|span| span.2 > 1), "{}: line {line} is a shared seam of two cards", case["name"]);
        }
    }
}

#[test]
fn painting_a_stack_draws_the_shared_band_lines_once() {
    let cases = fixture();
    let case = stack_cases(&cases)
        .find(|case| case["name"] == "footer chip of the upper card shares the band with the title chip of the lower card")
        .expect("shared vertical band fixture");
    let cards: Vec<_> = case["windows"].as_array().unwrap().iter().map(card).collect();
    let mut draw = DrawList::default();
    let color = Rgba { r: 0.2, g: 0.3, b: 0.4, a: 1.0 };
    paint_card_stack(&mut draw, &card_stack_outlines(0.0, 0.0, 400.0, &cards), 1.0, |_| (color, SharedBorderState::Normal));
    while !draw.finalize_shared_borders_step().unwrap() {}
    let painted: Vec<_> = draw.layers.iter().flat_map(|layer| &layer.ui_instances).filter(|item| item.rect[2] > 0.0 && item.rect[3] > 0.0).collect();
    let covered = |y: f32, low: f32, high: f32| painted.iter().filter(|item| item.rect[1] == y && item.rect[3] == 1.0).map(|item| (item.rect[0] + item.rect[2]).min(high) - item.rect[0].max(low)).filter(|width| *width > 0.0).sum::<f32>();
    assert!((covered(171.0, 0.0, 100.0) - 100.0).abs() < 0.001, "the lower card's cap chip line is one line on the band top");
    assert!((covered(199.0, 300.0, 400.0) - 100.0).abs() < 0.001, "the upper card's footer chip line is one line on the band bottom");
}
