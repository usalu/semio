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
    let law: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🪜️stepper-presentation/🔣️.json")).unwrap();
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
    let law: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🪜️stepper-presentation/🔣️.json")).unwrap();
    for case in law["numbers"].as_array().unwrap() {
        assert_eq!(ui_contract::format_ui_number(case["value"].as_f64().unwrap()), case["text"].as_str().unwrap());
    }
    for invalid in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        assert_eq!(ui_contract::format_ui_number(invalid), "")
    }
}

#[test]
fn slider_cells_range_and_thumb_match_the_shared_react_geometry() {
    let law: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🎚️slider-presentation/🔣️.json")).unwrap();
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
    let law: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🎚️slider-presentation/🔣️.json")).unwrap();
    let unit = &law["unit"];
    let value = unit["value"].as_f64().unwrap();
    let gap = unit["gap"].as_f64().unwrap() as f32;
    let suffix_width = 40.0;
    assert_eq!(slider_unit_label(&ui_contract::format_ui_number(value), unit["unit"].as_str()), Some(unit["externalReadout"].as_str().unwrap().to_string()));
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

#[test]
fn ring_value_and_angle_match_the_shared_react_vectors() {
    let law: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/⭕️ring-angle/🔣️.json")).unwrap();
    let number = |value: &serde_json::Value| value.as_f64().unwrap();
    for frame in law["frames"].as_array().unwrap() {
        let bounds = Rect::new(number(&frame[0]) as f32, number(&frame[1]) as f32, number(&frame[2]) as f32, number(&frame[3]) as f32);
        let (cx, cy, scale) = ring_center(bounds);
        assert_eq!(scale, 1.0);
        for knob in law["knobs"].as_array().unwrap() {
            let (x, y) = ring_point(bounds, number(&knob["t"]));
            assert!((f64::from(x - cx) - number(&knob["offset"][0])).abs() < 1e-3 && (f64::from(y - cy) - number(&knob["offset"][1])).abs() < 1e-3, "{} at {frame}", knob["id"]);
        }
        for drag in law["drags"].as_array().unwrap() {
            let t = ring_t_at(bounds, cx + number(&drag["offset"][0]) as f32, cy + number(&drag["offset"][1]) as f32);
            assert!((t - number(&drag["t"])).abs() < 1e-6, "{} at {frame}: {t}", drag["id"]);
        }
    }
}

#[test]
fn ring_knob_and_drag_round_trip_and_stay_in_the_unit_interval() {
    let bounds = Rect::new(3.0, 4.0, 100.0, 100.0);
    for step in 0..64 {
        let t = f64::from(step) / 64.0;
        let (x, y) = ring_point(bounds, t);
        let back = ring_t_at(bounds, x, y);
        assert!((back - t).abs() < 1e-4 || (back - t).abs() > 1.0 - 1e-4, "{t} -> {back}");
        assert!((0.0..1.0).contains(&back));
    }
}

/// 📜️ The scrollbar shows only when content overflows, hugs the inline end, and its thumb is the visible share of the content riding the track as the offset moves.
#[test]
fn a_scrollbar_exists_only_for_overflow_and_its_thumb_tracks_the_offset() {
    let viewport = Rect::new(10.0, 20.0, 200.0, 100.0);
    assert!(scrollbar(viewport, 100.0, 0.0, true, false).is_none(), "content that fits has no scrollbar");
    let top = scrollbar(viewport, 400.0, 0.0, true, false).expect("overflow shows a scrollbar");
    assert_eq!((top.track.x, top.track.w), (210.0 - SCROLLBAR_SIZE_PX, SCROLLBAR_SIZE_PX));
    assert_eq!(top.thumb.h, 25.0, "a quarter of the content is visible");
    assert_eq!(top.thumb.y, 20.0);
    let bottom = scrollbar(viewport, 400.0, 300.0, true, false).unwrap();
    assert_eq!(bottom.thumb.y + bottom.thumb.h, 120.0, "scrolled to the end the thumb rests on the track end");
    assert_eq!(scrollbar(viewport, 400.0, 0.0, true, true).unwrap().track.x, 10.0, "RTL puts the track on the left");
    let huge = scrollbar(viewport, 100_000.0, 0.0, true, false).unwrap();
    assert_eq!(huge.thumb.h, SCROLLBAR_THUMB_MIN_PX, "a thumb never shrinks below the grabbable minimum");
}

#[test]
fn dragging_the_thumb_scrolls_in_proportion_and_a_track_press_pages_towards_it() {
    let bar = scrollbar(Rect::new(0.0, 0.0, 200.0, 100.0), 400.0, 0.0, true, false).unwrap();
    assert_eq!(bar.offset_for_thumb_drag(0.0, 75.0), 300.0, "the whole thumb travel is the whole content travel");
    assert_eq!(bar.offset_for_thumb_drag(0.0, 1_000.0), 300.0, "clamped at the end");
    assert_eq!(bar.offset_for_thumb_drag(100.0, -1_000.0), 0.0, "clamped at the start");
    assert_eq!(bar.offset_for_track_press(0.0, 90.0), 100.0, "a press below the thumb pages down one viewport");
    let scrolled = scrollbar(Rect::new(0.0, 0.0, 200.0, 100.0), 400.0, 150.0, true, false).unwrap();
    assert_eq!(scrolled.offset_for_track_press(150.0, 2.0), 50.0, "a press above it pages up");
}

#[test]
fn a_horizontal_scrollbar_runs_along_the_block_end_and_scrolls_on_the_x_axis() {
    let viewport = Rect::new(0.0, 0.0, 100.0, 200.0);
    assert!(scrollbar(viewport, 100.0, 0.0, false, false).is_none());
    let bar = scrollbar(viewport, 400.0, 0.0, false, false).unwrap();
    assert_eq!((bar.track.y, bar.track.h, bar.track.w), (200.0 - SCROLLBAR_SIZE_PX, SCROLLBAR_SIZE_PX, 100.0));
    assert_eq!(bar.thumb.w, 25.0);
    assert_eq!(bar.offset_for_thumb_drag(0.0, 75.0), 300.0);
    assert_eq!(bar.offset_for_track_press(0.0, 90.0), 100.0);
}

#[test]
fn ring_press_starts_only_on_the_orb() {
    let bounds = Rect::new(10.0, 20.0, 100.0, 100.0);
    let (x, y) = ring_point(bounds, 0.25);
    assert!(ring_orb_hit(bounds, 0.25, x, y));
    assert!(ring_orb_hit(bounds, 0.25, x + 6.9, y));
    assert!(!ring_orb_hit(bounds, 0.25, x + 7.5, y));
    assert!(!ring_orb_hit(bounds, 0.0, x, y), "the bare ring away from the orb is not a handle");
}

#[test]
fn icon_selector_cells_stack_mode_editor_preview_and_the_two_buttons() {
    for rows_of_text in [4u8, 5] {
        let bounds = Rect::new(4.0, 6.0, 200.0, icon_select_height(rows_of_text));
        let rows = icon_select_rows(bounds, rows_of_text);
        assert_eq!(rows.mode.y, bounds.y);
        assert_eq!(rows.editor.y, rows.mode.y + rows.mode.h);
        assert_eq!(rows.preview.y, rows.editor.y + rows.editor.h);
        assert_eq!(rows.import.y, rows.preview.y + rows.preview.h);
        assert!((rows.import.y + rows.import.h - (bounds.y + bounds.h)).abs() < 0.01);
        assert_eq!((rows.import.x, rows.import.w, rows.clear.x, rows.clear.w), (bounds.x, 100.0, bounds.x + 100.0, 100.0));
    }
}

#[test]
fn the_editor_grows_from_four_rows_to_five_for_data_and_vector() {
    use semio_ui_declaration::ui::IconSelectorMode;
    assert_eq!(IconSelectorMode::ALL.map(IconSelectorMode::editor_rows), [4, 4, 4, 5, 4, 4, 5]);
    assert_eq!(icon_select_editor_height(4), TEXTAREA_MIN_HEIGHT);
    assert_eq!(icon_select_editor_height(5), TEXTAREA_MIN_HEIGHT * 1.25);
    assert!(icon_select_height(5) > icon_select_height(4));
}
