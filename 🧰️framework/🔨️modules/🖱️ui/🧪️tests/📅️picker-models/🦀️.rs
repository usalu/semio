//! 📅️ LAW: the table row menu, date picker and colour picker models answer the shared vectors `🧫️fixtures/📅️picker-models/🔣️.json` exactly as the browser's own `Date`, `Intl` and `<input>` do.

use semio_ui_wgpu_layout::colour_picker::*;
use semio_ui_wgpu_layout::date_picker::*;
use semio_ui_wgpu_layout::table::*;
use serde_json::Value;

fn law() -> Value {
    serde_json::from_str(include_str!("../🧫️fixtures/📅️picker-models/🔣️.json")).expect("📅️ the picker-models fixture parses")
}

fn quad(value: &Value) -> (f32, f32, f32, f32) {
    let n = |index: usize| value[index].as_f64().unwrap() as f32;
    (n(0), n(1), n(2), n(3))
}

fn close(a: (f32, f32, f32, f32), b: (f32, f32, f32, f32)) -> bool {
    (a.0 - b.0).abs() < 1e-3 && (a.1 - b.1).abs() < 1e-3 && (a.2 - b.2).abs() < 1e-3 && (a.3 - b.3).abs() < 1e-3
}

fn date(text: &str) -> CivilDate {
    CivilDate::parse_iso(text).unwrap_or_else(|| panic!("date {text}"))
}

#[test]
fn row_menu_trigger_anchor_and_height_follow_react() {
    let law = law();
    for case in law["rowMenu"]["trigger"].as_array().unwrap() {
        assert!(close(table_row_menu_trigger(quad(&case["row"]), case["spacingSingle"].as_f64().unwrap() as f32, case["rtl"].as_bool().unwrap()), quad(&case["expected"])), "{case}");
    }
    for case in law["rowMenu"]["anchor"].as_array().unwrap() {
        let (x, y) = table_row_menu_anchor(quad(&case["row"]));
        let expected = &case["expected"];
        assert!((x - expected[0].as_f64().unwrap() as f32).abs() < 1e-3 && (y - expected[1].as_f64().unwrap() as f32).abs() < 1e-3, "{case}");
    }
    for case in law["rowMenu"]["height"].as_array().unwrap() {
        assert_eq!(table_row_height(case["sizeMedium"].as_f64().unwrap() as f32), case["expected"].as_f64().unwrap() as f32);
    }
}

#[test]
fn iso_values_epoch_days_and_weekdays_match_date() {
    for case in law()["date"]["iso"].as_array().unwrap() {
        let text = case["text"].as_str().unwrap();
        if case["valid"].as_bool() == Some(false) {
            assert!(CivilDate::parse_iso(text).is_none(), "{text}");
            continue;
        }
        let parsed = date(text);
        assert_eq!(parsed.iso(), text);
        assert_eq!(parsed.epoch_day(), case["epochDay"].as_i64().unwrap());
        assert_eq!(CivilDate::from_epoch_day(parsed.epoch_day()), Some(parsed));
        assert_eq!(parsed.weekday_from_monday(), case["weekdayFromMonday"].as_u64().unwrap() as u8);
    }
}

#[test]
fn locale_names_and_display_match_intl() {
    for locale in law()["date"]["locales"].as_array().unwrap() {
        let ours = date_locale(locale["id"].as_str().unwrap()).unwrap();
        assert_eq!(ours.week_starts_monday, locale["weekStartsMonday"].as_bool().unwrap());
        for (index, month) in locale["months"].as_array().unwrap().iter().enumerate() {
            assert_eq!(ours.months[index], month.as_str().unwrap());
        }
        for case in locale["format"].as_array().unwrap() {
            assert_eq!(ours.format(date(case["iso"].as_str().unwrap())), case["text"].as_str().unwrap());
        }
    }
    assert!(date_locale("fr").is_none());
}

#[test]
fn month_grids_start_on_the_locale_week_and_span_six_weeks() {
    for case in law()["date"]["grids"].as_array().unwrap() {
        let grid = month_grid(case["year"].as_i64().unwrap() as i32, case["month"].as_u64().unwrap() as u8, case["weekStartsMonday"].as_bool().unwrap()).unwrap();
        assert_eq!(grid[0].iso(), case["first"].as_str().unwrap());
        assert_eq!(grid[41].iso(), case["last"].as_str().unwrap());
    }
}

#[test]
fn date_keys_move_pick_and_close_like_the_native_grid() {
    for case in law()["date"]["keys"].as_array().unwrap() {
        let got = date_key(case["key"].as_str().unwrap(), case["shift"].as_bool().unwrap(), date(case["focused"].as_str().unwrap()), case["mondayFirst"].as_bool().unwrap());
        let expected = &case["expected"];
        match (got, expected) {
            (None, Value::Null) => {}
            (Some(DateKey::Close), Value::String(text)) if text == "close" => {}
            (Some(DateKey::Focus(moved)), Value::Object(map)) => assert_eq!(moved.iso(), map["focus"].as_str().unwrap(), "{case}"),
            (Some(DateKey::Select(picked)), Value::Object(map)) => assert_eq!(picked.iso(), map["select"].as_str().unwrap(), "{case}"),
            other => panic!("{case}: {other:?}"),
        }
    }
}

#[test]
fn the_date_popup_fits_its_cells_inside_the_panel() {
    let layout = date_popup_layout(semio_ui_theme::geometry::Rect::new(10.0, 10.0, 120.0, 22.4), 600.0, 22.4, 3.2, false);
    assert_eq!(layout.panel.y, 32.4);
    assert!(layout.cells.iter().all(|cell| cell.x >= layout.panel.x && cell.x + cell.w <= layout.panel.x + layout.panel.w + 1e-3 && cell.y + cell.h <= layout.panel.y + layout.panel.h + 1e-3));
    let flipped = date_popup_layout(semio_ui_theme::geometry::Rect::new(10.0, 560.0, 120.0, 22.4), 600.0, 22.4, 3.2, false);
    assert!(flipped.panel.y + flipped.panel.h <= 560.0 + 1e-3);
    let mirrored = date_popup_layout(semio_ui_theme::geometry::Rect::new(10.0, 10.0, 120.0, 22.4), 600.0, 22.4, 3.2, true);
    assert!(mirrored.cells[0].x > layout.cells[0].x);
}

#[test]
fn colour_values_normalise_like_the_dom_colour_input() {
    for case in law()["colour"]["values"].as_array().unwrap() {
        assert_eq!(colour_value(case["text"].as_str().unwrap()).as_deref(), case["expected"].as_str(), "{case}");
    }
    for case in law()["colour"]["channels"].as_array().unwrap() {
        let expected: Vec<u8> = case["expected"].as_array().unwrap().iter().map(|value| value.as_u64().unwrap() as u8).collect();
        assert_eq!(colour_channels(case["text"].as_str().unwrap()).unwrap().to_vec(), expected);
    }
}

#[test]
fn colour_fields_make_a_value_or_keep_the_draft() {
    for case in law()["colour"]["fields"].as_array().unwrap() {
        let field = match case["field"].as_str().unwrap() {
            "hex" => ColourField::Hex,
            "red" => ColourField::Red,
            "green" => ColourField::Green,
            _ => ColourField::Blue,
        };
        assert_eq!(colour_apply_field(case["current"].as_str().unwrap(), field, case["draft"].as_str().unwrap()).as_deref(), case["expected"].as_str(), "{case}");
    }
    for case in law()["colour"]["swatchKeys"].as_array().unwrap() {
        assert_eq!(swatch_key(case["key"].as_str().unwrap(), case["focused"].as_u64().unwrap() as usize), case["expected"].as_u64().map(|value| value as usize), "{case}");
    }
}

#[test]
fn the_colour_popup_holds_every_swatch_inside_the_panel() {
    let layout = colour_popup_layout(semio_ui_theme::geometry::Rect::new(10.0, 10.0, 120.0, 22.4), 600.0, 22.4, 3.2, false);
    assert!(layout.swatches.iter().chain(layout.channels.iter()).chain([&layout.hex, &layout.preview]).all(|rect| rect.x >= layout.panel.x - 1e-3 && rect.x + rect.w <= layout.panel.x + layout.panel.w + 1e-3 && rect.y + rect.h <= layout.panel.y + layout.panel.h + 1e-3));
}

#[test]
fn a_file_input_wraps_its_change_action_and_answers_with_the_picked_name() {
    use semio_framework_value::DslValue;
    use semio_ui_declaration::layout::ActionDescriptor;
    use semio_ui_declaration::ui::{UiIconSelectNode, UiInputNode};
    let change = ActionDescriptor { controller_id: "app".into(), action: "setAttachment".into(), args: Some(DslValue::Object(vec![("slot".into(), DslValue::String("a".into()))])) };
    let input = UiInputNode { input_kind: "file".into(), accept: Some(".png,.jpg".into()), on_change: change, ..Default::default() };
    let pick = input.pick_file_action().expect("a file input has a pick action");
    assert_eq!((pick.controller_id.as_str(), pick.action.as_str()), ("framework", UiInputNode::PICK_FILE_ACTION));
    assert_eq!(pick.args.as_ref().and_then(|args| args.get("accept")).and_then(DslValue::as_str), Some(".png,.jpg"));
    let answer = UiIconSelectNode::imported_value_action(&pick, "photo.png").expect("the pick wraps the change action");
    assert_eq!((answer.controller_id.as_str(), answer.action.as_str()), ("app", "setAttachment"));
    assert_eq!(answer.args.as_ref().and_then(|args| args.get("value")).and_then(DslValue::as_str), Some("photo.png"));
    assert_eq!(answer.args.as_ref().and_then(|args| args.get("slot")).and_then(DslValue::as_str), Some("a"));
    assert!(UiInputNode { input_kind: "text".into(), ..Default::default() }.pick_file_action().is_none());
}

#[test]
fn the_open_popups_mint_the_cells_the_models_describe() {
    use semio_framework_value::DslValue;
    use semio_ui_declaration::layout::ActionDescriptor;
    use semio_ui_declaration::ui::{UiInputNode, UiNode};
    use semio_ui_locale::Locale;
    use semio_ui_theme::geometry::Rect;
    use semio_ui_theme::Theme;
    use semio_ui_wgpu_paint::picker_cells::*;
    let change = ActionDescriptor { controller_id: "app".into(), action: "set".into(), args: None };
    let theme = Theme::default();
    let anchor = Rect::new(10.0, 10.0, 120.0, theme.control_height);
    let field = UiInputNode { id: "d".into(), input_kind: "date".into(), value: "2026-09-17".into(), on_change: change.clone(), ..Default::default() };
    assert_eq!(picker_cell_count("date"), 53);
    let cells: Vec<(UiNode, Rect)> = (0..picker_cell_count("date")).map(|index| picker_cell(&field, None, None, anchor, 600.0, &theme, Some(Locale::De), index).map(|cell| (cell.node, cell.rect)).expect("cell")).collect();
    assert!(picker_cell(&field, None, None, anchor, 600.0, &theme, Some(Locale::De), 53).is_none());
    let label = |index: usize| match &cells[index].0 {
        UiNode::Button(button) => button.label.as_str().to_string(),
        _ => String::new(),
    };
    assert_eq!(label(DATE_CELL_TITLE), "September 2026");
    assert_eq!(label(53 - 7), "Mo");
    let seventeenth = cells[DATE_CELL_FIRST_DAY..DATE_CELL_FIRST_DAY + 42].iter().find(|(node, _)| matches!(node, UiNode::Button(button) if button.label.as_str() == "17" && button.presence.selected)).expect("the field's day is marked");
    let UiNode::Button(button) = &seventeenth.0 else { unreachable!() };
    assert_eq!(button.action.args.as_ref().and_then(|args| args.get("value")).and_then(DslValue::as_str), Some("2026-09-17"));
    let keyed = picker_cell(&field, Some(date("2026-09-18").epoch_day()), None, anchor, 600.0, &theme, Some(Locale::De), DATE_CELL_FIRST_DAY + 18).expect("day");
    assert!(matches!(&keyed.node, UiNode::Button(button) if button.label.as_str() == "18") && keyed.focused, "the keyboard-focused day holds the popup focus");
    assert!(!picker_cell(&field, None, None, anchor, 600.0, &theme, Some(Locale::De), DATE_CELL_FIRST_DAY + 18).expect("day").focused, "no keyboard day, no ring");
    assert!(cells.iter().skip(1).all(|(_, rect)| rect.x >= cells[0].1.x - 1e-3 && rect.y + rect.h <= cells[0].1.y + cells[0].1.h + 1e-3));
    let colour = UiInputNode { id: "c".into(), input_kind: "color".into(), value: "#ff8000".into(), on_change: change, ..Default::default() };
    assert_eq!(picker_cell_count("color"), 22);
    let swatch = picker_cell(&colour, None, Some(0), anchor, 600.0, &theme, Some(Locale::En), 2).map(|cell| (cell.node, cell.rect, cell.focused)).expect("swatch");
    assert!(swatch.2, "the highlighted swatch holds the popup focus");
    assert!(matches!(&swatch.0, UiNode::Input(input) if input.input_kind == "colorSwatch" && input.value == "#000000"));
    let channel = picker_cell(&colour, None, None, anchor, 600.0, &theme, Some(Locale::En), 19 + 1).map(|cell| (cell.node, cell.rect)).expect("green");
    assert!(matches!(&channel.0, UiNode::Input(input) if input.input_kind == "colourChannel" && input.value == "128" && input.commit.as_deref() == Some("colour:g:#ff8000")));
}

#[test]
fn typed_dates_read_iso_and_the_field_formats_and_no_clock_invents_a_day() {
    assert_eq!(parse_typed("2026-09-17").map(CivilDate::iso).as_deref(), Some("2026-09-17"));
    assert_eq!(parse_typed("09/17/2026").map(CivilDate::iso).as_deref(), Some("2026-09-17"));
    assert_eq!(parse_typed("17.09.2026").map(CivilDate::iso).as_deref(), Some("2026-09-17"));
    for refused in ["", "17.09.26", "13/01/2026", "31.02.2026", "1/2/3/4", "ab.cd.efgh"] {
        assert!(parse_typed(refused).is_none(), "{refused}");
    }
    set_today_epoch_day(date("2026-09-17").epoch_day());
    assert_eq!(today_epoch_day(), Some(date("2026-09-17").epoch_day()));
    assert_eq!(focus_date("", None), Some(date("2026-09-17")));
    assert_eq!(focus_date("2026-01-02", None), Some(date("2026-01-02")));
}
