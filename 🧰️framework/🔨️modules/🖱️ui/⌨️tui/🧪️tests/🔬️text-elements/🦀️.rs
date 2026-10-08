use crate::tui::cell::{attr, Cell, CellBuffer};
use crate::tui::event::{mods, Key, KeyEvent, MouseButton, MouseEvent, MouseKind};
use crate::tui::geometry::{Pos, Rect, Size};
use crate::tui::text::{display_width, display_width_in, WidthMode};
use crate::tui::theme::{GlyphSet, Role, Status, Theme};
use crate::tui::widget::{CursorShape, InputState, ListState, LogState, ProgressState, ScrollableState, SelectState, TableAlign, TableColumn, TableRow, TableState, ToggleState, TreeItem, TreeState, WidgetSignal, WidgetState, WizardState};
use std::time::Instant;
use ui_styling::appearance::AppearanceName;

fn theme() -> Theme {
    Theme::new(AppearanceName::Dark)
}

fn key(key: Key) -> KeyEvent {
    KeyEvent { key, mods: 0 }
}

fn press(x: u16, y: u16, clicks: u8) -> MouseEvent {
    MouseEvent { kind: MouseKind::Down(MouseButton::Left), pos: Pos { x, y }, mods: 0, clicks }
}

fn wheel(x: u16, y: u16, dy: i16) -> MouseEvent {
    MouseEvent { kind: MouseKind::Scroll { dx: 0, dy }, pos: Pos { x, y }, mods: 0, clicks: 0 }
}

fn paint(widget: &WidgetState, rect: Rect, focused: bool) -> CellBuffer {
    let mut buf = CellBuffer::new(Size { width: rect.x + rect.width, height: rect.y + rect.height }, Cell::blank([0, 0, 0], [0, 0, 0]));
    widget.paint(&theme(), rect, &mut buf, focused);
    buf
}

fn rows_of(buf: &CellBuffer, rect: Rect) -> Vec<String> {
    (rect.y..rect.y + rect.height).map(|y| buf.row_text(y).chars().skip(usize::from(rect.x)).collect::<String>().trim_end_matches(['\u{2502}', '\u{2503}']).trim_end().to_string()).collect()
}

fn names(count: usize) -> Vec<String> {
    (0..count).map(|n| format!("item{n:03}")).collect()
}

//#region 📃️List
#[test]
fn list_selects_on_a_press_activates_on_a_double_press_and_follows_the_selection() {
    let rect = Rect::new(0, 0, 12, 5);
    let mut widget = WidgetState::List(ListState::new(names(30)));
    paint(&widget, rect, true);
    assert_eq!(widget.on_mouse(rect, &press(3, 2, 1)), Some(WidgetSignal::SelectionChanged(2)));
    assert_eq!(widget.on_mouse(rect, &press(3, 2, 2)), Some(WidgetSignal::Activated(2)));
    assert_eq!(widget.on_key(&key(Key::Enter)), Some(WidgetSignal::Activated(2)));
    for _ in 0..6 {
        widget.on_key(&key(Key::Down));
    }
    let after_down = paint(&widget, rect, true);
    assert_eq!(rows_of(&after_down, rect)[4], "  item008", "moving down past the view keeps the highlight on the last row");
    for _ in 0..3 {
        widget.on_key(&key(Key::Up));
    }
    let after_up = paint(&widget, rect, true);
    assert_eq!(rows_of(&after_up, rect), ["  item004", "  item005", "  item006", "  item007", "  item008"], "moving back up scrolls nothing");
    assert_eq!(after_up.get(0, 1).unwrap().bg, theme().role(Role::ActiveBase), "the highlight moved up inside the view");
}

#[test]
fn list_page_home_end_wheel_hover_and_scroll_bar() {
    let rect = Rect::new(0, 0, 12, 5);
    let mut widget = WidgetState::List(ListState::new(names(30)));
    paint(&widget, rect, true);
    assert_eq!(widget.on_key(&key(Key::PageDown)), Some(WidgetSignal::SelectionChanged(5)));
    assert_eq!(widget.on_key(&key(Key::PageUp)), Some(WidgetSignal::SelectionChanged(0)));
    assert_eq!(widget.on_key(&key(Key::End)), Some(WidgetSignal::SelectionChanged(29)));
    assert_eq!(widget.on_key(&key(Key::End)), None);
    assert_eq!(widget.on_key(&key(Key::Home)), Some(WidgetSignal::SelectionChanged(0)));
    assert_eq!(widget.on_mouse(rect, &wheel(2, 2, 2)), None);
    let scrolled = paint(&widget, rect, true);
    assert_eq!(rows_of(&scrolled, rect)[0], "  item006", "two wheel steps scroll six rows");
    assert_eq!(widget.on_key(&key(Key::Down)), Some(WidgetSignal::SelectionChanged(1)));
    assert_eq!(rows_of(&paint(&widget, rect, true), rect)[0], "  item001", "a key brings the selection back into view");
    assert!(widget.set_hover(rect, Some(Pos { x: 4, y: 3 })));
    assert!(!widget.set_hover(rect, Some(Pos { x: 5, y: 3 })));
    let hovered = paint(&widget, rect, true);
    assert_eq!(hovered.get(0, 3).unwrap().bg, theme().role(Role::HoverInteractive));
    assert!(widget.set_hover(rect, None));
    let bar = paint(&widget, rect, true);
    assert!((0..5).any(|y| bar.get(11, y).unwrap().ch == '\u{2503}'), "an overflowing list draws a scroll bar thumb in the last column");
}

#[test]
fn list_marks_toggle_and_empty_list_is_safe() {
    let mut widget = WidgetState::List(ListState::new(names(3)));
    assert_eq!(widget.on_key(&key(Key::Char(' '))), Some(WidgetSignal::Toggled(true)));
    assert_eq!(widget.on_key(&key(Key::Char(' '))), Some(WidgetSignal::Toggled(false)));
    let mut empty = ListState::new(vec![]);
    empty.empty = "nothing here".to_string();
    let mut widget = WidgetState::List(empty);
    for k in [Key::Up, Key::Down, Key::PageUp, Key::PageDown, Key::Home, Key::End, Key::Enter, Key::Char(' ')] {
        assert_eq!(widget.on_key(&key(k)), None);
    }
    let rect = Rect::new(0, 0, 14, 3);
    assert_eq!(rows_of(&paint(&widget, rect, false), rect)[0], "nothing here");
    assert_eq!(widget.on_mouse(rect, &press(1, 1, 1)), None);
}

#[test]
fn list_rows_show_their_status_glyph_in_the_role_colour_and_the_spinner_ticks() {
    let rect = Rect::new(0, 0, 20, 4);
    let mut state = ListState::new(vec!["build ui".into(), "test ui".into(), "dev ui".into()]);
    state.set_status(0, Some(Status::Success));
    state.set_status(1, Some(Status::Running));
    state.set_status(2, Some(Status::Failure));
    let mut widget = WidgetState::List(state);
    let buf = paint(&widget, rect, false);
    assert_eq!(rows_of(&buf, rect)[..3], ["  \u{2713} build ui", "  \u{25d0} test ui", "  \u{2717} dev ui"]);
    assert_eq!(buf.get(2, 0).unwrap().fg, theme().role(Role::Success));
    assert_eq!(buf.get(2, 1).unwrap().fg, theme().role(Role::Accent));
    assert_eq!(buf.get(2, 2).unwrap().fg, theme().role(Role::Danger));
    assert!(!widget.tick(0), "the first frame is already drawn");
    assert!(!widget.tick(100), "within the same 125 ms frame nothing changes");
    assert!(widget.tick(130), "the next frame repaints the running row");
    assert_eq!(rows_of(&paint(&widget, rect, false), rect)[1], "  \u{25d3} test ui");
    let mut idle = WidgetState::List(ListState::new(names(2)));
    assert!(!idle.tick(10_000), "a list without running rows never asks for a repaint");
    assert_eq!(rows_of(&paint(&idle, rect, false), rect)[0], "  item000", "rows without any status keep the compact layout");
}
//#endregion 📃️List

//#region 🧙️Wizard
#[test]
fn wizard_signals_carry_option_identities_while_the_filter_narrows() {
    let options: Vec<String> = ["build ui", "dev ui", "test ui", "dev quiz"].iter().map(|s| s.to_string()).collect();
    let mut widget = WidgetState::Wizard(WizardState::new(options));
    for c in "dev".chars() {
        widget.on_key(&key(Key::Char(c)));
    }
    assert_eq!(widget.on_key(&key(Key::Down)), Some(WidgetSignal::SelectionChanged(3)), "the second visible row is option 3, not position 1");
    assert_eq!(widget.on_key(&key(Key::Enter)), Some(WidgetSignal::Activated(3)));
    assert_eq!(widget.on_key(&key(Key::Backspace)), Some(WidgetSignal::ValueChanged("de".into())));
    assert_eq!(widget.on_key(&key(Key::Esc)), Some(WidgetSignal::ValueChanged(String::new())));
    assert_eq!(widget.on_key(&key(Key::Esc)), Some(WidgetSignal::NavigateBack));
    assert_eq!(widget.on_key(&key(Key::Backspace)), None, "a held Backspace stops at the empty filter");
}

#[test]
fn wizard_clicks_select_double_clicks_activate_and_pastes_filter() {
    let rect = Rect::new(2, 1, 20, 6);
    let mut widget = WidgetState::Wizard(WizardState::new(names(40)));
    paint(&widget, rect, true);
    assert_eq!(widget.on_mouse(rect, &press(5, 3, 1)), Some(WidgetSignal::SelectionChanged(1)), "row 0 is the filter, row 1 of the list is item001");
    assert_eq!(widget.on_mouse(rect, &press(5, 3, 2)), Some(WidgetSignal::Activated(1)));
    assert_eq!(widget.on_mouse(rect, &press(5, 1, 1)), None, "the filter row is not a list row");
    assert_eq!(widget.on_paste("item03\n9"), Some(WidgetSignal::ValueChanged("item03 9".into())));
    let WidgetState::Wizard(w) = &widget else { unreachable!() };
    assert_eq!(w.list.count(), 1);
    assert_eq!(w.selected_option(), Some(39));
}

#[test]
fn wizard_keeps_its_selection_when_the_options_refresh_and_places_the_cursor_on_the_filter() {
    let rect = Rect::new(0, 0, 20, 5);
    let mut state = WizardState::new(vec!["a".into(), "b".into(), "c".into()]);
    state.steps.push(("dev".into(), "dev".into()));
    let mut widget = WidgetState::Wizard(state);
    widget.on_key(&key(Key::Down));
    widget.on_key(&key(Key::Down));
    let WidgetState::Wizard(w) = &mut widget else { unreachable!() };
    w.set_options(vec!["z".into(), "c".into(), "a".into()]);
    assert_eq!(w.selected_option(), Some(1), "the selected label moved to index 1");
    widget.on_key(&key(Key::Char('é')));
    let spec = widget.cursor(rect).expect("the filter row holds the cursor");
    assert_eq!((spec.pos, spec.shape), (Pos { x: 3, y: 1 }, CursorShape::Bar), "below the breadcrumb row, after the slash and the typed cell");
    let painted = rows_of(&paint(&widget, rect, true), rect);
    assert_eq!(painted[0], "dev");
    assert_eq!(painted[1], "/ é");
}

#[test]
fn wizard_filters_fifty_thousand_options_keystroke_by_keystroke() {
    let options: Vec<String> = (0..50_000).map(|n| format!("{} / project{} / target{}", ["dev", "build", "test", "check"][n % 4], n % 997, n % 31)).collect();
    let mut widget = WidgetState::Wizard(WizardState::new(options));
    let budget_ms = if cfg!(debug_assertions) { 200.0 } else { 5.0 };
    let rect = Rect::new(0, 0, 60, 20);
    for c in "dev project2 target1".chars() {
        let started = Instant::now();
        widget.on_key(&key(Key::Char(c)));
        paint(&widget, rect, true);
        let elapsed = started.elapsed().as_secs_f64() * 1000.0;
        assert!(elapsed < budget_ms, "typing {c:?} and repainting took {elapsed:.2} ms (budget {budget_ms} ms)");
    }
}
//#endregion 🧙️Wizard

//#region 📊️Table
fn table() -> TableState {
    let columns = vec![TableColumn::new("Name", 0, TableAlign::Left), TableColumn::new("N", 3, TableAlign::Right)];
    let rows = vec![
        TableRow::parent("a", vec!["alpha".into(), "1".into()]),
        TableRow::child("a1", vec!["one".into(), "2".into()], 1),
        TableRow::child("a2", vec!["two".into(), "3".into()], 1),
        TableRow::parent("b", vec!["beta".into(), "4".into()]),
        TableRow::child("b1", vec!["uno".into(), "5".into()], 1),
    ];
    TableState::new(columns, rows)
}

#[test]
fn table_survives_a_stale_selection_and_navigates_by_page_home_end() {
    let rect = Rect::new(0, 0, 24, 8);
    let mut stale = table();
    stale.selected = 99;
    let mut widget = WidgetState::Table(stale);
    for k in [Key::Down, Key::Up, Key::PageDown, Key::PageUp, Key::Home, Key::End, Key::Left, Key::Right, Key::Enter] {
        widget.on_key(&key(k));
    }
    paint(&widget, rect, true);
    let mut collapsed = table();
    collapsed.selected = 2;
    collapsed.rows[0].expanded = false;
    let mut widget = WidgetState::Table(collapsed);
    assert!(widget.on_key(&key(Key::Down)).is_some(), "a selection hidden under a collapsed parent restarts from the first visible row");
    let mut empty = WidgetState::Table(TableState::new(vec![], vec![]));
    for k in [Key::Down, Key::End, Key::Enter] {
        assert_eq!(empty.on_key(&key(k)), None);
    }
    let mut widget = WidgetState::Table(table());
    paint(&widget, rect, true);
    assert_eq!(widget.on_key(&key(Key::End)), Some(WidgetSignal::SelectionChanged(4)));
    assert_eq!(widget.on_key(&key(Key::Home)), Some(WidgetSignal::SelectionChanged(0)));
}

#[test]
fn table_rows_take_two_lines_for_the_pointer_and_a_double_press_toggles_or_activates() {
    let rect = Rect::new(0, 0, 24, 9);
    let mut widget = WidgetState::Table(table());
    paint(&widget, rect, true);
    assert_eq!(widget.on_mouse(rect, &press(3, 4, 1)), Some(WidgetSignal::SelectionChanged(1)), "row 1 is drawn on line 4, below the header and its hairline");
    assert_eq!(widget.on_mouse(rect, &press(3, 5, 1)), None, "the hairline between rows belongs to no row");
    assert_eq!(widget.on_mouse(rect, &press(3, 4, 2)), Some(WidgetSignal::Activated(1)));
    assert_eq!(widget.on_mouse(rect, &press(3, 2, 2)), Some(WidgetSignal::SelectionChanged(0)), "a double press on a parent row toggles it");
    let WidgetState::Table(t) = &widget else { unreachable!() };
    assert!(!t.rows[0].expanded);
    assert!(widget.set_hover(rect, Some(Pos { x: 2, y: 4 })));
    let right = MouseEvent { kind: MouseKind::Down(MouseButton::Right), pos: Pos { x: 3, y: 2 }, mods: 0, clicks: 1 };
    assert_eq!(widget.on_mouse(rect, &right), Some(WidgetSignal::ContextMenu { pos: Pos { x: 3, y: 2 }, item: Some(0) }));
}
//#endregion 📊️Table

//#region 🪵️Log
#[test]
fn log_scrolls_with_the_wheel_and_every_navigation_key() {
    let mut log = LogState::new(100);
    for n in 0..40 {
        log.push(&format!("line{n}"));
    }
    let rect = Rect::new(0, 0, 12, 6);
    let mut widget = WidgetState::Log(log);
    assert_eq!(rows_of(&paint(&widget, rect, false), rect)[5], "line39");
    widget.on_mouse(rect, &wheel(2, 2, -2));
    assert_eq!(rows_of(&paint(&widget, rect, false), rect)[5], "line33", "two wheel steps up move six lines");
    widget.on_key(&key(Key::Home));
    assert_eq!(rows_of(&paint(&widget, rect, false), rect)[0], "line0", "Home shows the first page, not one line");
    widget.on_key(&key(Key::PageDown));
    assert_eq!(rows_of(&paint(&widget, rect, false), rect)[0], "line6");
    widget.on_key(&key(Key::End));
    assert_eq!(rows_of(&paint(&widget, rect, false), rect)[5], "line39");
    widget.on_key(&key(Key::PageUp));
    assert_eq!(rows_of(&paint(&widget, rect, false), rect)[5], "line33", "the first PageUp from the tail moves a whole page");
}
//#endregion 🪵️Log

//#region ✏️Input
fn typed(value: &str, cursor: usize) -> WidgetState {
    WidgetState::Input(InputState { value: value.to_string(), cursor, placeholder: String::new() })
}

fn input(widget: &WidgetState) -> (&str, usize) {
    match widget {
        WidgetState::Input(i) => (&i.value, i.cursor),
        _ => unreachable!(),
    }
}

#[test]
fn input_moves_and_deletes_whole_grapheme_clusters_and_never_panics() {
    let text = "a🧰\u{fe0f}e\u{301}ü";
    let mut widget = typed(text, text.len());
    for expected in ["a🧰\u{fe0f}e\u{301}", "a🧰\u{fe0f}", "a", ""] {
        widget.on_key(&key(Key::Left));
        let (value, cursor) = input(&widget);
        assert_eq!(&value[..cursor], expected);
    }
    widget.on_key(&key(Key::Right));
    widget.on_key(&key(Key::Right));
    assert_eq!(widget.on_key(&key(Key::Backspace)), Some(WidgetSignal::ValueChanged("aé".replace('é', "e\u{301}ü"))));
    let mut widget = typed("ü", 1);
    widget.on_key(&key(Key::Char('x')));
    assert_eq!(input(&widget), ("xü", 1), "a cursor that sat inside a character is repaired to the boundary before it instead of panicking");
    assert_eq!(widget.on_key(&key(Key::Delete)), Some(WidgetSignal::ValueChanged("x".into())), "Delete removes the whole character behind the caret");
    assert_eq!(widget.on_key(&key(Key::Delete)), None);
    widget.on_key(&key(Key::Home));
    assert_eq!(widget.on_key(&key(Key::Delete)), Some(WidgetSignal::ValueChanged(String::new())));
}

#[test]
fn input_ignores_control_and_alt_characters_and_edits_words_and_lines() {
    let mut widget = typed("", 0);
    assert_eq!(widget.on_key(&KeyEvent { key: Key::Char('a'), mods: mods::CTRL }), None, "Ctrl+A moves to the start, it does not type an a");
    assert_eq!(widget.on_key(&KeyEvent { key: Key::Char('z'), mods: mods::ALT }), None);
    assert_eq!(input(&widget), ("", 0));
    let mut widget = typed("one two three", 13);
    widget.on_key(&KeyEvent { key: Key::Char('w'), mods: mods::CTRL });
    assert_eq!(input(&widget), ("one two ", 8));
    widget.on_key(&KeyEvent { key: Key::Left, mods: mods::CTRL });
    assert_eq!(input(&widget).1, 4);
    widget.on_key(&KeyEvent { key: Key::Char('k'), mods: mods::CTRL });
    assert_eq!(input(&widget), ("one ", 4));
    widget.on_key(&KeyEvent { key: Key::Char('u'), mods: mods::CTRL });
    assert_eq!(input(&widget), ("", 0));
    assert_eq!(widget.on_paste("a\tb\nc\u{7}"), Some(WidgetSignal::ValueChanged("a b c".into())));
}

#[test]
fn input_scrolls_to_keep_the_caret_visible_and_a_click_places_it() {
    let rect = Rect::new(3, 1, 8, 1);
    let widget = typed("0123456789abcdef", 16);
    let buf = paint(&widget, rect, true);
    assert_eq!(rows_of(&buf, rect)[0], "9abcdef", "the view ends at the caret");
    let spec = widget.cursor(rect).unwrap();
    assert_eq!(spec.pos, Pos { x: 10, y: 1 });
    assert_eq!(buf.get(10, 1).unwrap().attrs & attr::REVERSE, 0, "no painted caret: the terminal cursor shows the position");
    let mut widget = typed("0123456789abcdef", 16);
    widget.on_mouse(rect, &press(5, 1, 1));
    assert_eq!(input(&widget).1, 11, "column 2 of the view is character 9 + 2");
    let wide = typed("a世b", 1);
    let buf = paint(&wide, Rect::new(0, 0, 6, 1), true);
    assert_eq!(rows_of(&buf, Rect::new(0, 0, 6, 1))[0], "a世b");
    assert_eq!(display_width("a世"), 3);
}
//#endregion ✏️Input

//#region 🔽️Select
#[test]
fn select_cycles_by_key_wheel_and_click_and_jumps_with_home_end() {
    let rect = Rect::new(0, 0, 24, 1);
    let mut widget = WidgetState::Select(SelectState { label: "L".into(), options: vec!["a".into(), "b".into(), "c".into()], index: 0 });
    assert_eq!(widget.on_key(&key(Key::End)), Some(WidgetSignal::SelectionChanged(2)));
    assert_eq!(widget.on_key(&key(Key::End)), None);
    assert_eq!(widget.on_key(&key(Key::Home)), Some(WidgetSignal::SelectionChanged(0)));
    assert_eq!(widget.on_key(&key(Key::PageDown)), Some(WidgetSignal::SelectionChanged(1)));
    assert_eq!(widget.on_mouse(rect, &press(2, 0, 1)), Some(WidgetSignal::SelectionChanged(2)));
    assert_eq!(widget.on_mouse(rect, &wheel(2, 0, -1)), Some(WidgetSignal::SelectionChanged(1)));
}
//#endregion 🔽️Select

//#region 🌳️Tree
fn forest() -> TreeState {
    let items = vec![
        TreeItem::new("dev", "dev", 0),
        TreeItem::new("dev/puzzle", "puzzle3d", 1),
        TreeItem::new("dev/puzzle/react", "react", 2),
        TreeItem::new("dev/puzzle/wgpu", "wgpu", 2),
        TreeItem::new("dev/quiz", "quiz", 1),
        TreeItem::new("test", "test", 0),
        TreeItem::new("test/unit", "unit", 1),
    ];
    TreeState::new(items)
}

fn visible(widget: &WidgetState) -> Vec<String> {
    let WidgetState::Tree(t) = widget else { unreachable!() };
    (0..t.count()).filter_map(|row| t.row_item(row)).filter_map(|item| t.item(item)).map(|item| item.id.clone()).collect()
}

#[test]
fn tree_expands_collapses_and_walks_parents_and_children() {
    let mut widget = WidgetState::Tree(forest());
    assert_eq!(visible(&widget), ["dev", "test"]);
    assert_eq!(widget.on_key(&key(Key::Right)), Some(WidgetSignal::Toggled(true)));
    assert_eq!(visible(&widget), ["dev", "dev/puzzle", "dev/quiz", "test"]);
    assert_eq!(widget.on_key(&key(Key::Right)), Some(WidgetSignal::SelectionChanged(1)), "Right on an open folder enters it");
    assert_eq!(widget.on_key(&key(Key::Right)), Some(WidgetSignal::Toggled(true)));
    assert_eq!(widget.on_key(&key(Key::Down)), Some(WidgetSignal::SelectionChanged(2)));
    assert_eq!(widget.on_key(&key(Key::Left)), Some(WidgetSignal::SelectionChanged(1)), "Left on a leaf goes to its parent");
    assert_eq!(widget.on_key(&key(Key::Left)), Some(WidgetSignal::Toggled(false)));
    assert_eq!(widget.on_key(&key(Key::Left)), Some(WidgetSignal::SelectionChanged(0)));
    assert_eq!(widget.on_key(&key(Key::Enter)), Some(WidgetSignal::Toggled(false)), "Enter on a folder toggles it");
    assert_eq!(widget.on_key(&key(Key::End)), Some(WidgetSignal::SelectionChanged(5)));
    assert_eq!(widget.on_key(&key(Key::Home)), Some(WidgetSignal::SelectionChanged(0)));
    let WidgetState::Tree(t) = &mut widget else { unreachable!() };
    t.select_item(6);
    assert_eq!(widget.on_key(&key(Key::Enter)), Some(WidgetSignal::Activated(6)), "Enter on a leaf activates its item");
}

#[test]
fn tree_filter_matches_paths_keeps_ancestors_and_restores_the_selection() {
    let mut widget = WidgetState::Tree(forest());
    for c in "dev wgpu".chars() {
        widget.on_key(&key(Key::Char(c)));
    }
    assert_eq!(visible(&widget), ["dev", "dev/puzzle", "dev/puzzle/wgpu"], "the match shows with every ancestor and nothing else");
    let WidgetState::Tree(t) = &widget else { unreachable!() };
    assert_eq!(t.selected_item(), Some(3), "the selection lands on the first real match");
    assert_eq!(widget.on_key(&key(Key::Enter)), Some(WidgetSignal::Activated(3)));
    assert_eq!(widget.on_key(&key(Key::Esc)), Some(WidgetSignal::ValueChanged(String::new())));
    assert_eq!(visible(&widget), ["dev", "dev/puzzle", "dev/puzzle/react", "dev/puzzle/wgpu", "dev/quiz", "test"], "clearing the filter keeps the found item revealed and selected");
    let WidgetState::Tree(t) = &widget else { unreachable!() };
    assert_eq!(t.selected_item(), Some(3));
    let mut widget = WidgetState::Tree(forest());
    widget.on_key(&key(Key::Char('u')));
    assert_eq!(visible(&widget), ["dev", "dev/puzzle", "dev/puzzle/react", "dev/puzzle/wgpu", "dev/quiz", "test", "test/unit"], "puzzle3d, quiz and unit contain a u; their folders stay as ancestors");
}

#[test]
fn tree_pointer_toggles_on_the_marker_selects_on_the_label_and_activates_on_double_press() {
    let rect = Rect::new(0, 0, 24, 8);
    let mut widget = WidgetState::Tree(forest());
    paint(&widget, rect, true);
    assert_eq!(widget.on_mouse(rect, &press(0, 1, 1)), Some(WidgetSignal::Toggled(true)), "row 0 is the filter row; the marker of dev is at x 0 on line 1");
    assert_eq!(widget.on_mouse(rect, &press(8, 3, 1)), Some(WidgetSignal::SelectionChanged(4)));
    assert_eq!(widget.on_mouse(rect, &press(8, 3, 2)), Some(WidgetSignal::Activated(4)));
    assert!(widget.set_hover(rect, Some(Pos { x: 5, y: 2 })));
    let buf = paint(&widget, rect, true);
    assert_eq!(rows_of(&buf, rect)[1], "▾ dev");
    assert_eq!(rows_of(&buf, rect)[2], "  ▸ puzzle3d");
    let right = MouseEvent { kind: MouseKind::Down(MouseButton::Right), pos: Pos { x: 8, y: 2 }, mods: 0, clicks: 1 };
    assert_eq!(widget.on_mouse(rect, &right), Some(WidgetSignal::ContextMenu { pos: Pos { x: 8, y: 2 }, item: Some(1) }));
    assert_eq!(widget.cursor(rect).map(|spec| spec.pos), Some(Pos { x: 2, y: 0 }));
}

#[test]
fn tree_filters_fifty_thousand_nodes_quickly() {
    let mut items = Vec::with_capacity(50_000);
    for project in 0..500 {
        items.push(TreeItem::new(format!("p{project}"), format!("project{project}"), 0));
        for target in 0..99 {
            items.push(TreeItem::new(format!("p{project}/t{target}"), format!("target{target}"), 1));
        }
    }
    let mut widget = WidgetState::Tree(TreeState::new(items));
    let budget_ms = if cfg!(debug_assertions) { 80.0 } else { 8.0 };
    let rect = Rect::new(0, 0, 40, 20);
    for c in "project42 target7".chars() {
        let started = Instant::now();
        widget.on_key(&key(Key::Char(c)));
        paint(&widget, rect, true);
        let elapsed = started.elapsed().as_secs_f64() * 1000.0;
        assert!(elapsed < budget_ms, "typing {c:?} took {elapsed:.2} ms (budget {budget_ms} ms)");
    }
    let shown = visible(&widget);
    assert_eq!(shown.len(), 11 * 12, "projects 42 and 420..429 each keep their eleven t7 targets and their own row");
    assert_eq!(&shown[..3], ["p42", "p42/t7", "p42/t70"]);
}
//#endregion 🌳️Tree

//#region 📜️Scrollable
#[test]
fn scrollable_scrolls_both_axes_by_key_wheel_and_scroll_bar() {
    let lines: Vec<String> = (0..50).map(|n| format!("{n:02} {}", "x".repeat(40))).collect();
    let rect = Rect::new(0, 0, 20, 6);
    let mut widget = WidgetState::Scrollable(ScrollableState::new(lines));
    assert_eq!(rows_of(&paint(&widget, rect, true), rect)[0].chars().take(5).collect::<String>(), "00 xx");
    widget.on_key(&key(Key::PageDown));
    assert_eq!(rows_of(&paint(&widget, rect, true), rect)[0].chars().take(2).collect::<String>(), "06");
    widget.on_key(&key(Key::End));
    assert_eq!(rows_of(&paint(&widget, rect, true), rect)[5].chars().take(2).collect::<String>(), "49");
    widget.on_key(&key(Key::Home));
    widget.on_mouse(rect, &wheel(3, 3, 2));
    assert_eq!(rows_of(&paint(&widget, rect, true), rect)[0].chars().take(2).collect::<String>(), "06");
    widget.on_key(&key(Key::Right));
    let shifted = rows_of(&paint(&widget, rect, true), rect)[0].clone();
    assert!(shifted.starts_with("xx"), "{shifted:?}: four columns scrolled out of the line number");
    widget.on_key(&key(Key::Home));
    widget.on_mouse(rect, &MouseEvent { kind: MouseKind::Down(MouseButton::Left), pos: Pos { x: 19, y: 5 }, mods: 0, clicks: 1 });
    assert_eq!(rows_of(&paint(&widget, rect, true), rect)[5].chars().take(2).collect::<String>(), "49", "pressing the bottom of the scroll bar jumps to the end");
}

#[test]
fn scrollable_keeps_wide_clusters_whole_at_the_left_edge() {
    let mut state = ScrollableState::new(vec!["a世界b".into(); 2]);
    state.left = 2;
    let widget = WidgetState::Scrollable(state);
    let rect = Rect::new(0, 0, 4, 2);
    let buf = paint(&widget, rect, false);
    assert_eq!(rows_of(&buf, rect)[0], " 界b", "the offset lands inside the second cell of 世, which shows as one blank");
    let mut state = ScrollableState::new(vec!["a世界b".into(); 2]);
    state.left = 1;
    let rect = Rect::new(0, 0, 5, 2);
    let buf = paint(&WidgetState::Scrollable(state), rect, false);
    assert_eq!(rows_of(&buf, rect)[0], "世界b", "a left offset inside a wide glyph starts at the next whole cluster");
}
//#endregion 📜️Scrollable

//#region 📶️Progress
#[test]
fn progress_draws_a_determinate_bar_with_percentage_and_a_bouncing_indeterminate_marker() {
    let rect = Rect::new(0, 0, 40, 1);
    let widget = WidgetState::Progress(ProgressState::determinate("build", 0.5));
    let buf = paint(&widget, rect, false);
    let row = rows_of(&buf, rect)[0].clone();
    assert!(row.starts_with("build "), "{row:?}");
    assert!(row.ends_with(" 50%"), "{row:?}");
    let filled = row.chars().filter(|&c| c == '\u{2588}').count();
    let track = row.chars().filter(|&c| c == '\u{2591}').count();
    assert!(filled.abs_diff(track) <= 1, "half the bar is full: {filled} full, {track} empty");
    assert!(!widget.interactive());

    let mut waiting = WidgetState::Progress(ProgressState::indeterminate("sync"));
    assert!(!waiting.tick(0), "the first frame is already drawn");
    let before = rows_of(&paint(&waiting, rect, false), rect)[0].clone();
    assert!(!waiting.tick(50), "within the same frame nothing changes");
    assert!(waiting.tick(900));
    let after = rows_of(&paint(&waiting, rect, false), rect)[0].clone();
    assert_ne!(before, after, "the marker moved");
    let mut done = WidgetState::Progress(ProgressState::determinate("x", 1.0));
    assert!(!done.tick(10_000), "a determinate bar has nothing to animate");

    let mut ascii = theme();
    ascii.set_glyphs(GlyphSet::Ascii);
    let mut buf = CellBuffer::new(Size { width: 40, height: 1 }, Cell::blank([0, 0, 0], [0, 0, 0]));
    widget.paint(&ascii, rect, &mut buf, false);
    let row = rows_of(&buf, rect)[0].clone();
    assert!(row.contains('#') && row.contains('-') && row.is_ascii(), "{row:?}");
}
//#endregion 📶️Progress

//#region 🔀️Toggle
#[test]
fn toggle_flips_by_space_enter_and_click_and_reports_hover() {
    let rect = Rect::new(0, 0, 20, 1);
    let mut widget = WidgetState::Toggle(ToggleState::new("Reduce motion", false));
    assert_eq!(widget.on_key(&key(Key::Char(' '))), Some(WidgetSignal::Toggled(true)));
    assert_eq!(widget.on_key(&key(Key::Enter)), Some(WidgetSignal::Toggled(false)));
    assert_eq!(widget.on_key(&key(Key::Right)), Some(WidgetSignal::Toggled(true)));
    assert_eq!(widget.on_key(&key(Key::Right)), None);
    assert_eq!(widget.on_mouse(rect, &press(4, 0, 1)), Some(WidgetSignal::Toggled(false)));
    assert_eq!(widget.on_mouse(rect, &press(4, 3, 1)), None);
    assert!(widget.set_hover(rect, Some(Pos { x: 1, y: 0 })));
    assert!(!widget.set_hover(rect, Some(Pos { x: 2, y: 0 })));
    let buf = paint(&widget, rect, false);
    assert_eq!(buf.get(0, 0).unwrap().bg, theme().role(Role::HoverInteractive));
    assert_eq!(rows_of(&buf, rect)[0], "\u{25cb} Reduce motion");
    assert_eq!(widget.preferred_size(), Size { width: 15, height: 1 });
}
//#endregion 🔀️Toggle

//#region 🎚️Width Modes
fn paint_in(widget: &WidgetState, rect: Rect, mode: WidthMode) -> CellBuffer {
    let mut buf = CellBuffer::new(Size { width: rect.x + rect.width, height: rect.y + rect.height }, Cell::blank([0, 0, 0], [0, 0, 0]));
    buf.set_width_mode(mode);
    widget.paint(&theme(), rect, &mut buf, true);
    buf
}

const PLATFORM: &str = "\u{1f5a5}\u{fe0f}platform";
const REPO_PATH: &str = "\u{1f9f0}\u{fe0f}framework/\u{1f528}\u{fe0f}modules/\u{1f5a5}\u{fe0f}platform/\u{2328}\u{fe0f}tui";

#[test]
fn the_input_cursor_column_and_click_follow_the_active_width_mode() {
    let rect = Rect::new(0, 0, 20, 1);
    for (mode, column) in [(WidthMode::Cluster, 10), (WidthMode::Scalar, 9)] {
        let widget = typed(PLATFORM, PLATFORM.len());
        let buf = paint_in(&widget, rect, mode);
        let spec = widget.cursor(rect).unwrap();
        assert_eq!(spec.pos.x, column, "{mode:?}");
        assert_eq!(display_width_in(&rows_of(&buf, rect)[0], mode), column, "{mode:?}: the caret sits right behind the last painted cell");
        let mut clicked = typed(PLATFORM, 0);
        clicked.on_mouse(rect, &press(column - 1, 0, 1));
        assert_eq!(input(&clicked).1, PLATFORM.len() - 1, "{mode:?}: a press on the last cell puts the caret before the m");
    }
}

#[test]
fn lists_trees_and_wizards_elide_repo_paths_to_the_cells_the_active_mode_paints() {
    let width = 14u16;
    let rect = Rect::new(0, 0, width, 4);
    for mode in [WidthMode::Cluster, WidthMode::Scalar] {
        let items = vec![REPO_PATH.to_string(), PLATFORM.to_string(), format!("{PLATFORM}-{PLATFORM}")];
        let list = WidgetState::List(ListState::new(items.clone()));
        let wizard = WidgetState::Wizard(WizardState::new(items.clone()));
        let tree = WidgetState::Tree(TreeState::new(items.iter().map(|label| TreeItem::new(label.clone(), label.clone(), 0)).collect()));
        for (name, widget) in [("list", list), ("wizard", wizard), ("tree", tree)] {
            let buf = paint_in(&widget, rect, mode);
            for row in rows_of(&buf, rect) {
                assert!(display_width_in(&row, mode) <= width, "{name} in {mode:?} paints {row:?} wider than {width} cells");
            }
            for y in 0..rect.height {
                for x in 0..width {
                    let cell = buf.get(x, y).unwrap();
                    assert!(u16::from(cell.width) <= width - x || cell.width == 0, "{name} {mode:?}: a wide glyph crosses the right edge at {x},{y}");
                }
            }
        }
    }
}

#[test]
fn the_tree_and_wizard_filter_cursor_columns_follow_the_active_mode() {
    let rect = Rect::new(0, 0, 30, 4);
    for (mode, column) in [(WidthMode::Cluster, 2 + 10), (WidthMode::Scalar, 2 + 9)] {
        let mut tree = WidgetState::Tree(TreeState::new(vec![TreeItem::new("a", PLATFORM, 0)]));
        paint_in(&tree, rect, mode);
        tree.on_paste(PLATFORM);
        assert_eq!(tree.cursor(rect).unwrap().pos.x, column, "tree {mode:?}");
        let mut wizard = WidgetState::Wizard(WizardState::new(vec![PLATFORM.to_string()]));
        paint_in(&wizard, rect, mode);
        wizard.on_paste(PLATFORM);
        assert_eq!(wizard.cursor(rect).unwrap().pos.x, column, "wizard {mode:?}");
    }
}
//#endregion 🎚️Width Modes
