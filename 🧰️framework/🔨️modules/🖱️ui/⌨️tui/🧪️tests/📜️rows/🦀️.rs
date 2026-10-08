use super::*;
use crate::tui::event::{MouseButton, MouseEvent, MouseKind};
use crate::tui::geometry::{Pos, Rect, Size};
use semio_framework_pack_json::{parse, JsonMemberPolicy, Value};
use std::time::Instant;

fn labels(count: usize) -> Vec<String> {
    const VERBS: [&str; 8] = ["dev", "build", "test", "check", "serve", "bench", "lint", "format"];
    const OWNERS: [&str; 6] = ["🧰\u{fe0f}framework", "🔨\u{fe0f}modules", "🖱\u{fe0f}ui", "⌨\u{fe0f}tui", "🛍\u{fe0f}products", "🦑\u{fe0f}repo"];
    (0..count).map(|n| format!("{} / {} / project{} / target{}", VERBS[n % VERBS.len()], OWNERS[(n / 8) % OWNERS.len()], n % 997, n % 31)).collect()
}

fn naive(labels: &[String], query: &str) -> Vec<u32> {
    let lowered = query.to_lowercase();
    let tokens: Vec<&str> = lowered.split_whitespace().collect();
    labels.iter().enumerate().filter(|(_, label)| tokens.iter().all(|token| label.to_lowercase().contains(token))).map(|(index, _)| index as u32).collect()
}

fn click(x: u16, y: u16, clicks: u8, button: MouseButton) -> MouseEvent {
    MouseEvent { kind: MouseKind::Down(button), pos: Pos { x, y }, mods: 0, clicks }
}

//#region 🔎️Filter
#[test]
fn filter_matches_every_token_case_insensitively_and_unicode_aware() {
    let mut index = FilterIndex::new(["Dev Puzzle3D", "build ui-tui", "Überblick", "test puzzle3d"]);
    assert_eq!(index.matches("puzzle3d"), [0, 3]);
    assert_eq!(index.matches("PUZZLE dev"), [0]);
    assert_eq!(index.matches("überblick"), [2]);
    assert_eq!(index.matches("ÜBER"), [2]);
    assert_eq!(index.matches("  "), [0, 1, 2, 3], "an empty query lets every row through");
    assert_eq!(index.matches("nothing"), [] as [u32; 0]);
}

#[test]
fn shared_fixture_queries_select_the_listed_rows() {
    let root = parse(include_str!("../../🧫️fixtures/📜️rows/🔣️.json"), JsonMemberPolicy::Reject).expect("row filter fixture");
    let object = root.as_object().expect("fixture object");
    let labels: Vec<String> = object.get("labels").and_then(Value::as_array).expect("labels").iter().map(|label| label.as_str().expect("label").to_string()).collect();
    let mut index = FilterIndex::new(&labels);
    let mut model = ListModel::new(labels);
    let queries = object.get("queries").and_then(Value::as_array).expect("queries");
    assert!(queries.len() >= 10);
    for entry in queries {
        let entry = entry.as_object().expect("query entry");
        let query = entry.get("query").and_then(Value::as_str).expect("query");
        let rows: Vec<u32> = entry.get("rows").and_then(Value::as_array).expect("rows").iter().map(|row| row.as_u64().expect("row") as u32).collect();
        assert_eq!(index.matches(query), rows, "index for {query:?}");
        model.set_query(query);
        let visible: Vec<u32> = (0..model.count()).filter_map(|position| model.option_at(position)).map(|option| option as u32).collect();
        assert_eq!(visible, rows, "list model for {query:?}");
    }
}

#[test]
fn filter_agrees_with_a_naive_scan_for_pseudo_random_queries() {
    let all = labels(3000);
    let mut index = FilterIndex::new(&all);
    let words = ["dev", "puz", "project1", "target3", "ui", "tui", "🧰", "framework", "/", "x", "Build", "ZZ", "e", "t3", "repo"];
    let mut state = 0x9e37_79b9_u32;
    let mut next = move || {
        state ^= state << 13;
        state ^= state >> 17;
        state ^= state << 5;
        state
    };
    for _ in 0..600 {
        let count = 1 + next() % 3;
        let query = (0..count).map(|_| words[(next() % words.len() as u32) as usize]).collect::<Vec<_>>().join(" ");
        assert_eq!(index.matches(&query), naive(&all, &query), "query {query:?}");
    }
}

#[test]
fn an_extending_query_scans_only_the_previous_answer() {
    let all = labels(50_000);
    let mut index = FilterIndex::new(&all);
    let mut previous = all.len() as u64;
    for query in ["d", "de", "dev", "dev ", "dev /", "dev / 🖱", "dev / 🖱\u{fe0f}ui", "dev / 🖱\u{fe0f}ui project4"] {
        let before = index.scanned();
        let rows = index.matches(query).len() as u64;
        let cost = index.scanned() - before;
        assert!(cost <= previous, "{query:?} scanned {cost} rows although the previous answer had {previous}");
        previous = rows.max(1);
    }
    let before = index.scanned();
    index.matches("dev / 🖱\u{fe0f}ui project4");
    assert_eq!(index.scanned(), before, "the same query is answered from the history");
}

#[test]
fn backspace_is_answered_from_the_history_and_stays_correct_beyond_it() {
    let all = labels(50_000);
    let mut index = FilterIndex::new(&all);
    let long = "dev / 🧰\u{fe0f}framework / project1 / target1".to_string();
    let prefixes: Vec<String> = (1..=long.chars().count()).map(|n| long.chars().take(n).collect()).collect();
    for query in &prefixes {
        assert_eq!(index.matches(query), naive(&all, query), "typing {query:?}");
    }
    let before = index.scanned();
    for query in prefixes.iter().rev().skip(1).take(10) {
        assert_eq!(index.matches(query), naive(&all, query), "backspacing to {query:?}");
    }
    assert_eq!(index.scanned(), before, "the last ten queries are all in the history window");
    for query in prefixes.iter().rev().skip(11) {
        assert_eq!(index.matches(query), naive(&all, query), "backspacing past the window to {query:?}");
    }
}

#[test]
fn keystrokes_stay_fast_at_fifty_thousand_rows() {
    let all = labels(50_000);
    let mut index = FilterIndex::new(&all);
    let budget_ms = if cfg!(debug_assertions) { 60.0 } else { 5.0 };
    let mut worst = 0.0f64;
    let mut typed = String::new();
    for c in "dev project2 target1".chars() {
        typed.push(c);
        let started = Instant::now();
        let rows = index.matches(&typed).len();
        let elapsed = started.elapsed().as_secs_f64() * 1000.0;
        worst = worst.max(elapsed);
        assert!(rows <= 50_000);
        assert!(elapsed < budget_ms, "keystroke {typed:?} took {elapsed:.2} ms for {rows} rows (budget {budget_ms} ms)");
    }
    assert!(index.scanned() < 50_000 * 4, "twenty keystrokes scanned {} rows in total", index.scanned());
    assert!(worst < budget_ms);
}
//#endregion 🔎️Filter

//#region 🪟️Viewport
#[test]
fn the_viewport_follows_the_selection_and_keeps_its_offset_when_moving_back() {
    let mut rows = Rows::new();
    assert_eq!(rows.window(10, 100), 0..10);
    for _ in 0..12 {
        rows.step(1, 100);
    }
    assert_eq!(rows.window(10, 100), 3..13, "stepping past the bottom edge scrolls by the overshoot only");
    for _ in 0..4 {
        rows.step(-1, 100);
    }
    assert_eq!(rows.selected(), 8);
    assert_eq!(rows.window(10, 100), 3..13, "stepping back up moves the highlight inside the view; the offset stays");
    rows.select(1, 100);
    assert_eq!(rows.window(10, 100), 1..11, "selecting above the view scrolls up to it");
}

#[test]
fn page_home_and_end_navigate_by_the_painted_page() {
    let mut rows = Rows::new();
    rows.window(10, 100);
    assert_eq!(rows.navigate(Key::PageDown, 100), Some(true));
    assert_eq!(rows.selected(), 10);
    assert_eq!(rows.navigate(Key::PageDown, 100), Some(true));
    assert_eq!(rows.window(10, 100), 11..21);
    assert_eq!(rows.navigate(Key::PageUp, 100), Some(true));
    assert_eq!(rows.selected(), 10);
    assert_eq!(rows.navigate(Key::End, 100), Some(true));
    assert_eq!(rows.window(10, 100), 90..100);
    assert_eq!(rows.navigate(Key::End, 100), Some(false), "End at the end changes nothing");
    assert_eq!(rows.navigate(Key::Home, 100), Some(true));
    assert_eq!(rows.window(10, 100), 0..10);
    assert_eq!(rows.navigate(Key::Char('x'), 100), None);
    assert_eq!(rows.navigate(Key::Down, 0), Some(false), "an empty list has nowhere to go");
}

#[test]
fn the_wheel_scrolls_without_moving_the_selection_until_the_selection_moves_again() {
    let mut rows = Rows::new();
    rows.window(10, 100);
    rows.scroll(5, 100);
    assert_eq!(rows.window(10, 100), 15..25);
    assert_eq!(rows.selected(), 0, "the selection did not follow the wheel");
    assert_eq!(rows.window(10, 100), 15..25, "painting again does not snap the view back");
    rows.step(1, 100);
    assert_eq!(rows.window(10, 100), 1..11, "a keyboard move brings the selection back into view");
    rows.scroll(-50, 100);
    assert_eq!(rows.top(), 0);
    rows.scroll(500, 100);
    assert_eq!(rows.window(10, 100), 90..100);
}

#[test]
fn pointer_selects_on_press_activates_on_double_press_and_scrolls_with_the_wheel() {
    let area = Rect::new(2, 3, 20, 5);
    let mut rows = Rows::new();
    rows.window(5, 40);
    assert_eq!(rows.pointer(area, &click(4, 5, 1, MouseButton::Left), 40), Pointer::Selected(2));
    assert_eq!(rows.selected(), 2);
    assert_eq!(rows.pointer(area, &click(4, 5, 2, MouseButton::Left), 40), Pointer::Activated(2));
    assert_eq!(rows.pointer(area, &click(0, 5, 1, MouseButton::Left), 40), Pointer::Ignored, "outside the rows");
    assert_eq!(rows.pointer(area, &click(4, 8, 1, MouseButton::Left), 40), Pointer::Ignored, "below the last visible row");
    let right = rows.pointer(area, &click(5, 4, 1, MouseButton::Right), 40);
    assert_eq!(right, Pointer::Context { position: Pos { x: 5, y: 4 }, row: Some(1) });
    let wheel = MouseEvent { kind: MouseKind::Scroll { dx: 0, dy: 1 }, pos: Pos { x: 4, y: 4 }, mods: 0, clicks: 0 };
    assert_eq!(rows.pointer(area, &wheel, 40), Pointer::Scrolled);
    rows.window(5, 40);
    assert_eq!(rows.top(), 3);
    assert_eq!(rows.pointer(area, &click(4, 3, 1, MouseButton::Left), 40), Pointer::Selected(3), "rows map through the scrolled offset");
    assert_eq!(rows.pointer(area, &click(4, 3, 1, MouseButton::Left), 2), Pointer::Ignored, "only two rows exist now");
}

#[test]
fn hover_follows_the_pointer_over_rows_and_clears_outside() {
    let area = Rect::new(0, 0, 10, 4);
    let mut rows = Rows::new();
    rows.window(4, 3);
    assert!(rows.hover_at(area, Some(Pos { x: 2, y: 1 }), 3));
    assert_eq!(rows.hover(), Some(1));
    assert!(!rows.hover_at(area, Some(Pos { x: 5, y: 1 }), 3), "same row, nothing to repaint");
    assert!(rows.hover_at(area, Some(Pos { x: 5, y: 3 }), 3));
    assert_eq!(rows.hover(), None, "row 3 does not exist in a three row list");
    assert!(rows.hover_at(area, Some(Pos { x: 1, y: 0 }), 3));
    assert!(rows.hover_at(area, None, 3));
    assert_eq!(rows.hover(), None);
}
//#endregion 🪟️Viewport

//#region 📃️Listing
#[test]
fn the_list_model_keeps_option_identity_while_the_filter_changes() {
    let mut list = ListModel::new(vec!["build".into(), "dev".into(), "test".into(), "develop".into()]);
    assert_eq!((list.count(), list.selected_option()), (4, Some(0)));
    list.rows.select(2, 4);
    assert_eq!(list.selected_option(), Some(2));
    assert!(list.set_query("dev"));
    assert_eq!((list.count(), list.option_at(0), list.option_at(1)), (2, Some(1), Some(3)));
    assert_eq!(list.selected_option(), Some(1), "a new filter selects its first match");
    assert_eq!(list.position_of(3), Some(1));
    assert_eq!(list.position_of(2), None, "test is filtered out");
    list.push_query('e');
    assert_eq!(list.query(), "deve");
    assert_eq!(list.count(), 1);
    assert!(list.pop_query());
    assert!(list.pop_query() && list.pop_query() && list.pop_query());
    assert!(!list.pop_query(), "nothing left to drop");
    assert_eq!(list.count(), 4);
    assert!(!list.set_query(""), "the query was already empty");
}

#[test]
fn replacing_the_options_keeps_the_selected_label_and_the_query() {
    let mut list = ListModel::new(vec!["a".into(), "b".into(), "c".into()]);
    list.rows.select(2, 3);
    list.set_labels(vec!["z".into(), "c".into(), "a".into(), "b".into()]);
    assert_eq!(list.selected_option(), Some(1), "the selection followed its label to the new index");
    list.set_query("a");
    list.set_labels(vec!["a".into(), "ab".into(), "b".into()]);
    assert_eq!(list.count(), 2, "the query survives a refresh");
    list.set_labels(vec!["x".into()]);
    assert_eq!((list.count(), list.selected_option()), (0, None));
}

#[test]
fn the_list_model_filters_fifty_thousand_rows_with_a_stable_selection() {
    let mut list = ListModel::new(labels(50_000));
    for c in "build".chars() {
        list.push_query(c);
    }
    assert_eq!(list.count(), 6250);
    assert_eq!(list.selected_option(), Some(1), "the first row labelled build is option 1");
    assert!(list.scanned() < 50_000 * 2, "{} rows scanned for five keystrokes", list.scanned());
}
//#endregion 📃️Listing

//#region ↕️Scroll Bar
#[test]
fn the_scroll_bar_draws_only_when_rows_overflow_and_the_thumb_tracks_the_offset() {
    let theme = Theme::new(ui_styling::appearance::AppearanceName::Dark);
    let blank = Cell::blank([0, 0, 0], [0, 0, 0]);
    let mut buf = CellBuffer::new(Size { width: 1, height: 10 }, blank);
    let track = Rect::new(0, 0, 1, 10);
    assert!(!paint_scroll_bar(&mut buf, &theme, track, 0, 10, 10), "everything fits");
    assert!(paint_scroll_bar(&mut buf, &theme, track, 0, 10, 40));
    let thumb_rows = |buf: &CellBuffer| (0..10u16).filter(|&y| buf.get(0, y).unwrap().ch == '\u{2503}').collect::<Vec<_>>();
    assert_eq!(thumb_rows(&buf), vec![0, 1], "a quarter of the rows is visible, so the thumb is a quarter of the track");
    paint_scroll_bar(&mut buf, &theme, track, 30, 10, 40);
    assert_eq!(thumb_rows(&buf), vec![8, 9], "at the end the thumb sits at the bottom");
}
//#endregion ↕️Scroll Bar
