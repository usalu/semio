use super::*;
use crate::event::{MouseButton, MouseEvent, MouseKind};
use crate::geometry::{Pos, Rect, Size};
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
    let root = parse(include_str!("../🧫️fixtures/📜️rows/🔣️.json"), JsonMemberPolicy::Reject).expect("row filter fixture");
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

fn tree_fixture() -> Value {
    parse(include_str!("../../🧫️fixtures/🌳️tree-emphasis/🔣️.json"), JsonMemberPolicy::Reject).expect("tree emphasis fixture")
}

fn fixture_tree(root: &Value) -> crate::tui::tree::TreeState {
    let object = root.as_object().unwrap();
    crate::tui::tree::TreeState::new(object.get("items").and_then(Value::as_array).unwrap().iter().enumerate().map(|(id, item)| {
        let item = item.as_array().unwrap();
        crate::tui::tree::TreeItem::new(id.to_string(), item[0].as_str().unwrap(), item[1].as_u64().unwrap() as u16)
    }).collect())
}

fn fixture_color(theme: &Theme, name: &str) -> Rgb {
    match name {
        "foreground" => theme.role(Role::Foreground),
        "muted" => theme.role(Role::MutedForeground),
        "emphasized" => theme.role(Role::BorderEmphasized),
        "normal" => theme.role(Role::BorderNormal),
        "active" => theme.role(Role::ActiveForeground),
        _ => panic!("unknown color {name}"),
    }
}

fn oracle_cell(pixmap: &mut tiny_skia::Pixmap, x: u16, y: u16, color: Rgb) {
    let mut paint = tiny_skia::Paint::default();
    paint.set_color_rgba8(color[0], color[1], color[2], 255);
    paint.anti_alias = false;
    pixmap.fill_rect(tiny_skia::Rect::from_xywh(f32::from(x), f32::from(y), 1.0, 1.0).unwrap(), &paint, tiny_skia::Transform::identity(), None);
}

#[test]
fn tree_emphasis_fixture_matches_semantic_cells_and_independent_skia_color_planes() {
    let root = tree_fixture();
    assert_eq!(root.as_object().unwrap().get("emphasisAttribute").and_then(Value::as_str), Some("bold"));
    for appearance in [ui_styling::appearance::AppearanceName::Dark, ui_styling::appearance::AppearanceName::Light] {
        for glyphs in [crate::tui::theme::GlyphSet::Unicode, crate::tui::theme::GlyphSet::Ascii] {
            let mut theme = Theme::new(appearance);
            theme.set_glyphs(glyphs);
            for case in root.as_object().unwrap().get("cases").and_then(Value::as_array).unwrap() {
                let case = case.as_object().unwrap();
                let name = case.get("name").and_then(Value::as_str).unwrap();
                let mut tree = fixture_tree(&root);
                tree.filterable = false;
                for item in case.get("expanded").and_then(Value::as_array).unwrap() {
                    tree.set_expanded(item.as_u64().unwrap() as usize, true);
                }
                tree.set_query(case.get("query").and_then(Value::as_str).unwrap());
                tree.select_item(case.get("selected").and_then(Value::as_u64).unwrap() as usize);
                tree.view.set_hover(case.get("hover").and_then(Value::as_u64).and_then(|item| tree.position_of(item as usize)));
                let expected = case.get("rows").and_then(Value::as_array).unwrap();
                let rect = Rect::new(0, 0, 20, expected.len() as u16);
                let mut buf = CellBuffer::new(Size { width: rect.width, height: rect.height }, Cell::blank([0; 3], [0; 3]));
                crate::tui::tree::paint_tree(&tree, &theme, rect, &mut buf, true);
                let mut foreground = tiny_skia::Pixmap::new(u32::from(rect.width), u32::from(rect.height)).unwrap();
                let mut background = foreground.clone();
                assert_eq!(tree.count(), expected.len(), "{name}");
                for (y, row) in expected.iter().enumerate() {
                    let row = row.as_object().unwrap();
                    let item = row.get("item").and_then(Value::as_u64).unwrap() as usize;
                    assert_eq!(tree.row_item(y), Some(item), "{name}");
                    let fg = fixture_color(&theme, row.get("label").and_then(Value::as_str).unwrap());
                    let bg = match row.get("background").and_then(Value::as_str).unwrap() {
                        "active" => theme.role(Role::ActiveBase),
                        "hover" => theme.role(Role::HoverInteractive),
                        _ => theme.surface(crate::tui::theme::Surface::Window),
                    };
                    let guides = row.get("guides").and_then(Value::as_array).unwrap();
                    let indent = tree.item(item).unwrap().depth * 2;
                    let marker = if !tree.is_folder(item) { ' ' } else if tree.is_expanded(item) || !tree.query().is_empty() { theme.glyphs.glyph(Glyph::Expanded).chars().next().unwrap() } else { theme.glyphs.glyph(Glyph::Collapsed).chars().next().unwrap() };
                    assert_eq!(buf.get(indent, y as u16).unwrap().ch, marker, "{name} marker {item}");
                    if row.get("label").and_then(Value::as_str) == Some("emphasized") {
                        assert_ne!(buf.get(indent, y as u16).unwrap().attrs & crate::tui::cell::attr::BOLD, 0, "{name} emphasized marker {item}");
                        assert_ne!(buf.get(indent + 2, y as u16).unwrap().attrs & crate::tui::cell::attr::BOLD, 0, "{name} emphasized label {item}");
                    }
                    for x in 0..rect.width {
                        let color = if x < indent && x % 2 == 0 {
                            assert_eq!(buf.get(x, y as u16).unwrap().ch, theme.glyphs.glyph(Glyph::ScrollTrack).chars().next().unwrap(), "{name} guide {item}:{x}");
                            fixture_color(&theme, guides[usize::from(x / 2)].as_str().unwrap())
                        } else { fg };
                        oracle_cell(&mut foreground, x, y as u16, color);
                        oracle_cell(&mut background, x, y as u16, bg);
                    }
                }
                for y in 0..rect.height {
                    for x in 0..rect.width {
                        let cell = buf.get(x, y).unwrap();
                        let at = (usize::from(y) * usize::from(rect.width) + usize::from(x)) * 4;
                        assert_eq!(cell.fg, foreground.data()[at..at + 3], "{name} foreground {x}:{y}");
                        assert_eq!(cell.bg, background.data()[at..at + 3], "{name} background {x}:{y}");
                    }
                }
            }
        }
    }
}

#[test]
fn tree_rebuild_clears_hover_identity_before_positions_can_point_to_siblings() {
    let root = tree_fixture();
    let mut tree = fixture_tree(&root);
    tree.expand_to_depth(3);
    tree.view.set_hover(tree.position_of(2));
    tree.set_expanded(1, false);
    assert_eq!(tree.view.hover(), None);
    tree.view.set_hover(tree.position_of(4));
    tree.set_query("OtherLeaf");
    assert_eq!(tree.view.hover(), None);
}

#[test]
fn tree_clips_hover_guides_and_retains_keyboard_and_unfocused_selection() {
    let root = tree_fixture();
    let mut tree = fixture_tree(&root);
    tree.filterable = false;
    tree.expand_to_depth(3);
    tree.select_item(2);
    tree.view.set_hover(tree.position_of(2));
    let theme = Theme::new(ui_styling::appearance::AppearanceName::Dark);
    let sentinel = Cell::blank([11, 22, 33], [44, 55, 66]);
    let mut buf = CellBuffer::new(Size { width: 8, height: 5 }, sentinel);
    let rect = Rect::new(2, 1, 3, 2);
    crate::tui::tree::paint_tree(&tree, &theme, rect, &mut buf, false);
    assert_eq!(tree.view.top(), 1);
    assert_eq!(buf.get(2, 2).unwrap().fg, theme.role(Role::BorderEmphasized));
    for y in 0..buf.size.height {
        for x in 0..buf.size.width {
            if !rect.contains(Pos { x, y }) { assert_eq!(*buf.get(x, y).unwrap(), sentinel); }
        }
    }
    let wide = Rect::new(0, 0, 20, 8);
    let mut buf = CellBuffer::new(Size { width: 20, height: 8 }, sentinel);
    crate::tui::tree::paint_tree(&tree, &theme, wide, &mut buf, false);
    let label = buf.get(6, 2).unwrap();
    assert_eq!((label.fg, label.bg, label.attrs), (theme.role(Role::Foreground), theme.surface(crate::tui::theme::Surface::Window), crate::tui::cell::attr::BOLD));
    assert_eq!(crate::tui::tree::tree_on_key(&mut tree, &crate::tui::event::KeyEvent { key: Key::Left, mods: 0 }), Some(crate::tui::widget::WidgetSignal::SelectionChanged(1)));
    assert_eq!(crate::tui::tree::tree_on_key(&mut tree, &crate::tui::event::KeyEvent { key: Key::Right, mods: 0 }), Some(crate::tui::widget::WidgetSignal::SelectionChanged(2)));
}

#[test]
fn tree_hover_clears_over_filter_empty_space_and_scrollbar_and_updates_between_branches() {
    let root = tree_fixture();
    let mut tree = fixture_tree(&root);
    tree.expand_to_depth(3);
    let theme = Theme::new(ui_styling::appearance::AppearanceName::Dark);
    let rect = Rect::new(0, 0, 20, 9);
    let mut buf = CellBuffer::new(Size { width: 20, height: 9 }, Cell::blank([0; 3], [0; 3]));
    crate::tui::tree::paint_tree(&tree, &theme, rect, &mut buf, true);
    assert!(crate::tui::tree::tree_set_hover(&mut tree, rect, Some(Pos { x: 6, y: 3 })));
    assert_eq!(tree.view.hover(), Some(2));
    assert!(crate::tui::tree::tree_set_hover(&mut tree, rect, Some(Pos { x: 6, y: 8 })));
    crate::tui::tree::paint_tree(&tree, &theme, rect, &mut buf, true);
    assert_eq!(buf.get(4, 2).unwrap().fg, theme.role(Role::Foreground));
    assert_eq!(buf.get(2, 7).unwrap().fg, theme.role(Role::BorderEmphasized));
    assert!(crate::tui::tree::tree_set_hover(&mut tree, rect, Some(Pos { x: 0, y: 0 })));
    assert_eq!(tree.view.hover(), None);
    tree.set_expanded(0, false);
    assert!(!crate::tui::tree::tree_set_hover(&mut tree, rect, Some(Pos { x: 6, y: 8 })));
    assert_eq!(tree.view.hover(), None);
    let short = Rect::new(0, 0, 20, 2);
    crate::tui::tree::paint_tree(&tree, &theme, short, &mut buf, true);
    tree.view.set_hover(Some(0));
    assert!(crate::tui::tree::tree_set_hover(&mut tree, short, Some(Pos { x: 19, y: 1 })));
    assert_eq!(tree.view.hover(), None);
}

#[test]
fn tree_filtered_ancestor_selection_keeps_its_foreground_without_hover() {
    let root = tree_fixture();
    let mut tree = fixture_tree(&root);
    tree.filterable = false;
    tree.set_query("OtherLeaf");
    tree.select_item(6);
    let theme = Theme::new(ui_styling::appearance::AppearanceName::Dark);
    let rect = Rect::new(0, 0, 16, 2);
    let mut buf = CellBuffer::new(Size { width: 16, height: 2 }, Cell::blank([0; 3], [0; 3]));
    crate::tui::tree::paint_tree(&tree, &theme, rect, &mut buf, true);
    let label = buf.get(2, 0).unwrap();
    assert_eq!((label.fg, label.bg), (theme.role(Role::ActiveForeground), theme.role(Role::ActiveBase)));
    assert_eq!(tree.selected_item(), Some(6));
}

#[test]
fn tree_hover_path_labels_and_chevrons_are_bold_while_selected_and_unrelated_rows_keep_their_attributes() {
    let root = tree_fixture();
    for appearance in [ui_styling::appearance::AppearanceName::Dark, ui_styling::appearance::AppearanceName::Light] {
        let theme = Theme::new(appearance);
        for (selected, focused) in [(1, true), (1, false), (6, true), (6, false)] {
            let mut tree = fixture_tree(&root);
            tree.filterable = false;
            tree.expand_to_depth(3);
            tree.select_item(selected);
            tree.view.set_hover(tree.position_of(2));
            let rect = Rect::new(0, 0, 20, 8);
            let mut buf = CellBuffer::new(Size { width: 20, height: 8 }, Cell::blank([0; 3], [0; 3]));
            crate::tui::tree::paint_tree(&tree, &theme, rect, &mut buf, focused);
            for item in 0..tree.len() {
                let path = [0, 1, 2].contains(&item);
                let expected = if item == selected {
                    if focused { 0 } else { crate::tui::cell::attr::BOLD }
                } else if path { crate::tui::cell::attr::BOLD } else { 0 };
                let marker = tree.item(item).unwrap().depth * 2;
                assert_eq!(buf.get(marker, item as u16).unwrap().attrs, expected, "{appearance:?} marker {item} selected={selected} focused={focused}");
                assert_eq!(buf.get(marker + 2, item as u16).unwrap().attrs, expected, "{appearance:?} label {item} selected={selected} focused={focused}");
            }
        }
    }
}
