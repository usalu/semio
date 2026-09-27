use super::*;

#[test]
fn vertical_layout_distributes_children() {
    let theme = Theme::default();
    let bounds = Rect::new(0.0, 0.0, 100.0, 100.0);
    let rects = layout_vertical(bounds, 4.0, 8.0, &[20.0, 30.0]);
    assert_eq!(rects.len(), 2);
    assert!(rects[0].h > 20.0);
    assert!(rects[1].y > rects[0].y);
    let _ = theme;
}

#[test]
fn number_stepper_square_buttons_match_the_shared_react_geometry() {
    let law: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🪜️stepper-presentation/🔣️.json")).unwrap();
    for case in law["geometry"].as_array().unwrap() {
        let rect = |value: &serde_json::Value| Rect::new(value[0].as_f64().unwrap() as f32, value[1].as_f64().unwrap() as f32, value[2].as_f64().unwrap() as f32, value[3].as_f64().unwrap() as f32);
        assert_eq!(
            number_stepper_segments(rect(&case["bounds"]), if case["inline"] == "rtl" { ui_contract::FlowInline::Rtl } else { ui_contract::FlowInline::Ltr }, case["border"].as_f64().unwrap() as f32),
            [rect(&case["segments"][0]), rect(&case["segments"][1]), rect(&case["segments"][2])],
            "{}",
            case["id"]
        );
    }
}

#[test]
fn number_stepper_display_matches_reacts_twelve_significant_digits() {
    let law: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🪜️stepper-presentation/🔣️.json")).unwrap();
    for case in law["numbers"].as_array().unwrap() {
        assert_eq!(ui_contract::format_ui_number(case["value"].as_f64().unwrap()), case["text"].as_str().unwrap());
    }
    for invalid in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        assert_eq!(ui_contract::format_ui_number(invalid), "")
    }
}

#[test]
fn slider_cells_range_and_thumb_match_the_shared_react_geometry() {
    let law: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🎚️slider-presentation/🔣️.json")).unwrap();
    let rect = |value: &serde_json::Value| Rect::new(value[0].as_f64().unwrap() as f32, value[1].as_f64().unwrap() as f32, value[2].as_f64().unwrap() as f32, value[3].as_f64().unwrap() as f32);
    let close_rect = |left: Rect, right: Rect| [left.x - right.x, left.y - right.y, left.w - right.w, left.h - right.h].into_iter().all(|delta| delta.abs() < 0.001);
    for case in law["cases"].as_array().unwrap() {
        let presentation =
            slider_presentation(rect(&case["bounds"]), case["value"].as_f64().unwrap(), case["min"].as_f64().unwrap(), case["max"].as_f64().unwrap(), if case["inline"] == "rtl" { ui_contract::FlowInline::Rtl } else { ui_contract::FlowInline::Ltr });
        assert!(close_rect(presentation.track_cell, rect(&case["trackCell"])), "{} track", case["id"]);
        assert!(close_rect(presentation.value_cell, rect(&case["valueCell"])), "{} value", case["id"]);
        assert!(close_rect(presentation.rail, rect(&case["rail"])), "{} rail", case["id"]);
        assert!(close_rect(presentation.range, rect(&case["range"])), "{} range", case["id"]);
        assert!(close_rect(presentation.thumb, rect(&case["thumb"])), "{} thumb", case["id"]);
        assert_eq!(ui_contract::format_ui_number(case["value"].as_f64().unwrap()), case["formatted"].as_str().unwrap());
    }
}

#[test]
fn slider_unit_is_a_shrink_to_content_sibling_outside_the_inner_track_and_readout() {
    let law: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🎚️slider-presentation/🔣️.json")).unwrap();
    let unit = &law["unit"];
    let value = unit["value"].as_f64().unwrap();
    let gap = unit["gap"].as_f64().unwrap() as f32;
    let suffix_width = 40.0;
    assert_eq!(slider_unit_label(value, unit["unit"].as_str()), Some(unit["externalReadout"].as_str().unwrap().to_string()));
    for placement in unit["placements"].as_array().unwrap() {
        let bounds = Rect::new(placement["bounds"][0].as_f64().unwrap() as f32, placement["bounds"][1].as_f64().unwrap() as f32, placement["bounds"][2].as_f64().unwrap() as f32, placement["bounds"][3].as_f64().unwrap() as f32);
        let inline = if placement["inline"] == "rtl" { ui_contract::FlowInline::Rtl } else { ui_contract::FlowInline::Ltr };
        let presentation = slider_control_presentation(bounds, value, 0.0, 10.0, Some(suffix_width), gap, inline);
        let unit_cell = presentation.unit_cell.expect("authored unit sibling");
        assert!((unit_cell.w - suffix_width).abs() < 0.001, "{} unit width", placement["id"]);
        assert!((presentation.slider_bounds.w + gap + unit_cell.w - bounds.w).abs() < 0.001, "{} wrapper width", placement["id"]);
        assert!(presentation.slider.track_cell.x >= presentation.slider_bounds.x);
        assert!(presentation.slider.value_cell.x + presentation.slider.value_cell.w <= presentation.slider_bounds.x + presentation.slider_bounds.w + 0.001);
        if inline.is_rtl() {
            assert!((unit_cell.x + unit_cell.w + gap - presentation.slider_bounds.x).abs() < 0.001, "RTL puts the authored second sibling on the physical left");
        } else {
            assert!((presentation.slider_bounds.x + presentation.slider_bounds.w + gap - unit_cell.x).abs() < 0.001, "LTR puts the authored second sibling on the physical right");
        }
    }
}
