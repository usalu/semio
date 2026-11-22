//! 📊️ LAW: the table model answers the shared vectors `🧫️fixtures/📊️table-column-model/🔣️.json` exactly as the TypeScript table rules do.

use semio_ui_wgpu_layout::table::*;
use serde_json::Value;

fn law() -> Value {
    serde_json::from_str(include_str!("../🧫️fixtures/📊️table-column-model/🔣️.json")).expect("📊️ the table-column-model fixture parses")
}

fn f(value: &Value, key: &str) -> f32 {
    value[key].as_f64().unwrap_or_else(|| panic!("number {key}")) as f32
}

fn u(value: &Value, key: &str) -> usize {
    value[key].as_u64().unwrap_or_else(|| panic!("integer {key}")) as usize
}

fn s<'a>(value: &'a Value, key: &str) -> Option<&'a str> {
    value[key].as_str()
}

fn close(a: f32, b: f32) -> bool {
    (a - b).abs() <= 1e-3
}

#[test]
fn templates_match_the_react_grid_template() {
    for case in law()["templates"].as_array().unwrap() {
        assert_eq!(table_column_template(u(case, "columns"), u(case, "actions"), u(case, "leading"), u(case, "trailing")), case["template"].as_str().unwrap());
    }
}

#[test]
fn tracks_offsets_and_scroll_range_are_fixed_width() {
    let law = law();
    let (medium, single) = (f(&law["metrics"], "sizeMediumPx"), f(&law["metrics"], "spacingSinglePx"));
    for case in law["tracks"].as_array().unwrap() {
        let mut widths = vec![TableTrack::FIXED.width(); u(case, "columns")];
        if u(case, "actions") > 0 {
            widths.push(table_actions_track_px(u(case, "actions"), medium, single));
        }
        let expected: Vec<f32> = case["widths"].as_array().unwrap().iter().map(|v| v.as_f64().unwrap() as f32).collect();
        assert!(widths.iter().zip(&expected).all(|(a, b)| close(*a, *b)) && widths.len() == expected.len(), "{}", case["name"]);
        let (starts, content) = table_track_offsets(&widths);
        assert!(starts.iter().zip(case["starts"].as_array().unwrap()).all(|(a, b)| close(*a, b.as_f64().unwrap() as f32)), "{}", case["name"]);
        assert!(close(content, f(case, "content")), "{}", case["name"]);
        assert!(close(table_max_scroll_left(content, f(case, "viewport")), f(case, "maxScrollLeft")), "{}", case["name"]);
    }
}

#[test]
fn min_max_tracks_resolve_to_the_larger_bound() {
    assert_eq!(TableTrack { min: 100.0, max: 80.0 }.width(), 100.0);
    assert_eq!(TableTrack { min: 100.0, max: 240.0 }.width(), 240.0);
}

#[test]
fn scroll_left_brings_a_column_into_view() {
    for case in law()["scrollLeftForColumn"].as_array().unwrap() {
        assert!(close(table_scroll_left_for_column(u(case, "index"), f(case, "columnPx"), f(case, "scrollLeft"), f(case, "width")), f(case, "expected")), "{case}");
    }
}

#[test]
fn the_surface_table_shares_its_width_equally() {
    for case in law()["fillTrack"].as_array().unwrap() {
        assert!(close(table_fill_track_px(f(case, "width"), u(case, "columns")), f(case, "expected")), "{case}");
    }
}

#[test]
fn scroll_top_brings_a_row_into_view() {
    for case in law()["scrollTopForRow"].as_array().unwrap() {
        assert!(close(table_scroll_top_for_row(u(case, "index"), f(case, "rowPx"), f(case, "scrollTop"), f(case, "height")), f(case, "expected")), "{case}");
    }
}

#[test]
fn row_and_column_keys_follow_the_logical_extent() {
    let law = law();
    for case in law["nextRow"].as_array().unwrap() {
        assert_eq!(table_next_row(s(case, "key").unwrap(), u(case, "current"), u(case, "total"), u(case, "page")), case["expected"].as_u64().map(|v| v as usize), "{case}");
    }
    for case in law["nextColumn"].as_array().unwrap() {
        assert_eq!(table_next_column(s(case, "key").unwrap(), u(case, "current"), u(case, "total")), case["expected"].as_u64().map(|v| v as usize), "{case}");
    }
}

#[test]
fn keys_emit_focus_range_and_toggle_selection() {
    for case in law()["keys"].as_array().unwrap() {
        let outcome = table_key(s(case, "key").unwrap(), case["shift"].as_bool().unwrap(), case["ctrl"].as_bool().unwrap(), u(case, "current"), u(case, "total"), u(case, "page"));
        assert_eq!(outcome, TableKeyOutcome { focus: case["focus"].as_u64().map(|v| v as usize), select: s(case, "select").map(|m| match m { "range" => "range", "invertive" => "invertive", "replace" => "replace", other => panic!("{other}") }), activate: case["activate"].as_bool().unwrap(), escape: case["escape"].as_bool().unwrap() }, "{case}");
    }
    for case in law()["merge"].as_array().unwrap() {
        assert_eq!(table_merge_for_modifiers(case["shift"].as_bool().unwrap(), case["ctrlOrMeta"].as_bool().unwrap(), case["alt"].as_bool().unwrap()), s(case, "expected").unwrap());
    }
}

#[test]
fn sort_cycles_and_announces_aria_sort() {
    for case in law()["sort"].as_array().unwrap() {
        let (column, sort_column, direction) = (s(case, "columnId").unwrap(), s(case, "sortColumn"), s(case, "sortDirection"));
        assert_eq!(table_sort_next(column, sort_column, direction), s(case, "next").unwrap());
        assert_eq!(table_aria_sort(column, sort_column, direction), s(case, "aria"));
    }
}

#[test]
fn empty_label_is_react_key_in_en_and_de() {
    let law = law();
    assert_eq!(TABLE_EMPTY_LABEL_KEY, s(&law["emptyLabel"], "key").unwrap());
    assert_eq!(table_empty_label("en"), s(&law["emptyLabel"], "en"));
    assert_eq!(table_empty_label("de"), s(&law["emptyLabel"], "de"));
    assert_eq!(table_empty_label("fr"), None);
}

#[test]
fn avatar_initials_are_two_upper_case_word_starts() {
    for case in law()["avatarInitials"].as_array().unwrap() {
        assert_eq!(table_avatar_initials(s(case, "name").unwrap()), s(case, "expected").unwrap());
    }
}
