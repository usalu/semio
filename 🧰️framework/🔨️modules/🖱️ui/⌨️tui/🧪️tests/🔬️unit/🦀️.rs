
use crate::tui::ansi::{AnsiParser, AnsiPatch, emit_runs, setup_sequence, teardown_sequence};
use crate::tui::cell::{Cell, CellBuffer, DiffRun, attr, diff};
use crate::tui::chrome::{ChromeState, FooterState, KeyHint, NavItem, NavbarState, WindowState, mount_window_layout, shell, window_chip_layout};
use crate::tui::event::{Event, Key, KeyEvent, MouseEvent, MouseKind};
use crate::tui::geometry::{Pos, Rect, Size};
use crate::tui::layout::{Constraint, Dimension, Direction, WindowLayout, WindowLayoutRoot, WindowLayoutStackNode, WindowLayoutWindowNode, create_default_layout, even_window_layout, solve, solve_window_layout};
use crate::tui::scene::{Node, NodeContent, Scene};
use crate::tui::text::{display_width, truncate_to};
use crate::tui::theme::{Role, Surface, Theme};
use crate::tui::widget::{Align, ChipState, DividerState, InputState, LabelState, ListState, LogScroll, LogState, SelectState, TableAlign, TableColumn, TableRow, TableState, TabsState, TerminalState, WidgetSignal, WidgetState, WizardState};
use ui_styling::appearance::AppearanceName;

fn row_text(buf: &CellBuffer, y: u16) -> String {
    (0..buf.size.width).filter_map(|x| buf.get(x, y)).map(|c| c.ch).filter(|&c| c != '\0').collect()
}

#[test]
fn layout_row_weights_fill_exactly() {
    let mut scene = Scene::new();
    let root = scene.root();
    scene.node_mut(root).set_constraint(Constraint { direction: Direction::Row, ..Default::default() });
    let a = scene.add(root, Node::new(NodeContent::Box));
    let b = scene.add(root, Node::new(NodeContent::Box));
    scene.node_mut(a).set_constraint(Constraint { width: Dimension::Weight(1), ..Default::default() });
    scene.node_mut(b).set_constraint(Constraint { width: Dimension::Weight(2), ..Default::default() });
    solve(&mut scene, Rect::new(0, 0, 30, 10));
    assert_eq!(scene.rect(a).width + scene.rect(b).width, 30);
    assert!(scene.rect(b).width > scene.rect(a).width);
}

#[test]
fn layout_auto_measures_text_width() {
    let mut scene = Scene::new();
    let root = scene.root();
    scene.node_mut(root).set_constraint(Constraint { direction: Direction::Row, ..Default::default() });
    let text = scene.add(root, Node::new(NodeContent::Text("hi".into())));
    solve(&mut scene, Rect::new(0, 0, 30, 10));
    assert_eq!(scene.rect(text).width, 2);
}

#[test]
fn layout_padding_and_gap() {
    let mut scene = Scene::new();
    let root = scene.root();
    scene.node_mut(root).set_constraint(Constraint { direction: Direction::Row, gap: 1, padding: [1, 1, 1, 1], ..Default::default() });
    let a = scene.add(root, Node::new(NodeContent::Box));
    let b = scene.add(root, Node::new(NodeContent::Box));
    scene.node_mut(a).set_constraint(Constraint { width: Dimension::Cells(2), ..Default::default() });
    scene.node_mut(b).set_constraint(Constraint { width: Dimension::Cells(2), ..Default::default() });
    solve(&mut scene, Rect::new(0, 0, 20, 10));
    assert_eq!(scene.rect(a).x, 1);
    assert_eq!(scene.rect(b).x, 4);
    assert_eq!(scene.rect(root).height, 10);
    assert_eq!(scene.rect(a).height, 8);
}

#[test]
fn window_layout_row_of_stacks_tiles_without_gaps() {
    let layout = create_default_layout(&["a".to_string(), "b".to_string()], "row", Some(&[1.0, 1.0]), None);
    let measures = solve_window_layout(&layout, Rect::new(0, 0, 100, 10));
    assert_eq!(measures.len(), 2);
    assert_eq!(measures[0].rect.width + measures[1].rect.width, 100);
    assert_eq!(measures[0].rect.x, 0);
    assert_eq!(measures[1].rect.x, measures[0].rect.width);
}

#[test]
fn window_layout_stack_exposes_tabs() {
    let layout = WindowLayout {
        root: WindowLayoutRoot::Stack(WindowLayoutStackNode {
            size: None,
            active_window_kind_id: Some("b".into()),
            children: vec![WindowLayoutWindowNode { window_kind_id: "a".into(), title: None, corner: None }, WindowLayoutWindowNode { window_kind_id: "b".into(), title: None, corner: None }],
        }),
        zoomed: None,
    };
    let measures = solve_window_layout(&layout, Rect::new(0, 0, 40, 10));
    assert_eq!(measures.len(), 1);
    assert_eq!(measures[0].window_kind_id, "b");
    assert_eq!(measures[0].stack_tabs, vec!["a", "b"]);
}

#[test]
fn window_chrome_recesses_tabs_into_the_top_corners_of_a_closed_shape() {
    let theme = Theme::new(AppearanceName::Dark);
    let rect = Rect::new(0, 0, 40, 5);
    let mut buf = CellBuffer::new(Size { width: 40, height: 5 }, Cell::blank([0, 0, 0], [0, 0, 0]));
    let mut w = WindowState::new("Puzzle 3D");
    w.focused = true;
    ChromeState::Window(w).paint(&theme, rect, &mut buf);

    // single top-left corner tab with inline actions; right edge stays flat at the body hairline
    let row0 = row_text(&buf, 0);
    let row1 = row_text(&buf, 1);
    let row2 = row_text(&buf, 2);
    assert!(row0.starts_with('\u{250c}'), "tab top-left corner: {row0:?}");
    assert!(row1.contains("Puzzle 3D"), "title in tab: {row1:?}");
    assert!(row1.contains('\u{2922}'), "maximize glyph: {row1:?}");
    assert!(row1.contains('\u{29C9}'), "new-window glyph: {row1:?}");
    assert!(row1.contains('\u{2715}'), "close glyph: {row1:?}");
    assert!(row2.contains('\u{2514}'), "tab bends into body: {row2:?}");
    assert_eq!(row2.chars().last(), Some('\u{2510}'), "flat top-right at body: {row2:?}");
    assert_eq!(row_text(&buf, 4).chars().next(), Some('\u{2514}'));
    assert_eq!(row_text(&buf, 4).chars().last(), Some('\u{2518}'));

    let title_x = (0..40).find(|&x| buf.get(x, 1).unwrap().ch == 'P').expect("title text rendered");
    assert_eq!(buf.get(title_x, 1).unwrap().fg, theme.role(Role::Accent));
}

#[test]
fn window_chrome_flattens_the_right_side_when_no_controls_tab_is_wanted() {
    let theme = Theme::new(AppearanceName::Dark);
    let rect = Rect::new(0, 0, 40, 6);
    let mut buf = CellBuffer::new(Size { width: 40, height: 6 }, Cell::blank([0, 0, 0], [0, 0, 0]));
    let mut w = WindowState::new("Log");
    w.closable = false;
    w.maximizable = false;
    ChromeState::Window(w).paint(&theme, rect, &mut buf);

    let row0 = row_text(&buf, 0);
    let row1 = row_text(&buf, 1);
    let row2 = row_text(&buf, 2);
    assert!(row0.starts_with('\u{250c}'), "{row0:?}");
    assert!(row1.contains("Log"), "{row1:?}");
    assert!(!row1.contains('\u{2922}') && !row1.contains('\u{2715}'), "no action glyphs: {row1:?}");
    assert!(row2.contains('\u{2514}'), "bend into body: {row2:?}");
    assert_eq!(row2.chars().last(), Some('\u{2510}'), "flat right corner: {row2:?}");
    assert!(!row_text(&buf, 0).contains('\u{2922}') && !row_text(&buf, 1).contains('\u{2922}'), "no controls tab was requested");
}

#[test]
fn window_chrome_hides_both_tabs_when_too_narrow_for_even_the_title() {
    let theme = Theme::new(AppearanceName::Dark);
    let rect = Rect::new(0, 0, 4, 5);
    let mut buf = CellBuffer::new(Size { width: 4, height: 5 }, Cell::blank([0, 0, 0], [0, 0, 0]));
    let w = WindowState::new("X");
    assert!(!window_chip_layout(&w, rect).has_tabs);
    let window = ChromeState::Window(w);
    window.paint(&theme, rect, &mut buf);
    // plain flat box when the raised corner chrome cannot fit
    assert_eq!(row_text(&buf, 0), "┌──┐");
    assert_eq!(window.window_control_at(rect, Pos { x: 2, y: 0 }), None);
    assert_eq!(window.window_control_at(rect, Pos { x: 2, y: 1 }), None);
}

#[test]
fn window_control_clicks_resolve_to_close_and_maximize_signals() {
    let theme = Theme::new(AppearanceName::Dark);
    let rect = Rect::new(0, 0, 40, 5);
    let mut buf = CellBuffer::new(Size { width: 40, height: 5 }, Cell::blank([0, 0, 0], [0, 0, 0]));
    let window = ChromeState::Window(WindowState::new("Plugins"));
    window.paint(&theme, rect, &mut buf);
    let maximize_x = (0..40).find(|&x| buf.get(x, 1).unwrap().ch == '\u{2922}').expect("maximize glyph rendered");
    let close_x = (0..40).find(|&x| buf.get(x, 1).unwrap().ch == '\u{2715}').expect("close glyph rendered");
    assert_eq!(window.window_control_at(rect, Pos { x: maximize_x, y: 1 }), Some(WidgetSignal::WindowMaximize));
    assert_eq!(window.window_control_at(rect, Pos { x: close_x, y: 1 }), Some(WidgetSignal::WindowClose));
    assert_eq!(window.window_control_at(rect, Pos { x: close_x, y: 0 }), None, "clicks on the tab's own top edge must not trigger a control");
    assert_eq!(window.window_control_at(rect, Pos { x: close_x, y: 2 }), None, "clicks below the tab row must not trigger a control");
}

#[test]
fn window_hit_resolves_tab_activation_and_new_tab_signals() {
    let rect = Rect::new(0, 0, 60, 8);
    let w = WindowState::new("Main").with_stack_tabs(vec!["tab1".into(), "tab2".into()], 0);
    let layout = window_chip_layout(&w, rect);
    let window = ChromeState::Window(w);
    let group = layout.groups.iter().find(|g| g.corner == crate::tui::layout::WindowStackCorner::TopLeft).expect("top-left group");
    let tab0 = group.tabs.first().expect("first tab geometry");
    let tab1 = group.tabs.get(1).expect("second tab geometry");
    assert_eq!(window.window_hit(rect, Pos { x: tab0.x + 2, y: rect.y + 1 }), Some(WidgetSignal::WindowTabActivated(0)));
    assert_eq!(window.window_hit(rect, Pos { x: tab1.x + 2, y: rect.y + 1 }), Some(WidgetSignal::WindowTabActivated(1)));
    let new_x = tab0.new_x.expect("new-window glyph geometry");
    assert_eq!(window.window_hit(rect, Pos { x: new_x, y: rect.y + 1 }), Some(WidgetSignal::WindowNewTab));
}

#[test]
fn shell_window_wizard_body_paints_options_after_remount() {
    let mut tui = crate::tui::engine::Tui::new(Size { width: 60, height: 16 }, Theme::new(AppearanceName::Dark));
    let navbar = NavbarState { left: vec![], center: vec![], right: vec![] };
    let footer = FooterState { hints: vec![], status: String::new() };
    let layout = create_default_layout(&["w1".into()], "row", None, Some(&["wizard".into()]));
    let mut built = shell(&mut tui.scene, navbar, footer, &layout);
    let (_id, chrome) = built.windows[0].clone();
    let widget = tui.scene.add(chrome, Node::new(NodeContent::Widget(WidgetState::Wizard(WizardState::new(vec!["dev".into(), "build".into(), "test".into()])))));
    tui.scene.node_mut(widget).set_constraint(Constraint { width: Dimension::Weight(1), height: Dimension::Weight(1), ..Default::default() });
    built.remount(&mut tui.scene, &layout);
    tui.set_focus(Some(widget));
    let _ = tui.render_full();
    let rect = tui.scene.rect(widget);
    assert!(rect.width > 4 && rect.height > 1, "wizard rect collapsed: {rect:?}");
    let mut buf = CellBuffer::new(Size { width: 60, height: 16 }, Cell::blank([0, 0, 0], [0, 0, 0]));
    match &tui.scene.node(widget).content {
        NodeContent::Widget(w) => w.paint(&Theme::new(AppearanceName::Dark), rect, &mut buf, true),
        _ => panic!("expected wizard widget"),
    }
    let body: String = (0..60).filter_map(|x| buf.get(x, rect.y).map(|c| c.ch)).collect();
    assert!(body.contains("dev") || body.contains("›"), "wizard options missing from paint: {body:?}");
}

#[test]
fn mount_window_layout_reparents_windows_and_preserves_widget_state() {
    let mut scene = Scene::new();
    let root = scene.root();
    let canvas = scene.add(root, Node::new(NodeContent::Chrome(ChromeState::Canvas)));
    let w1_chrome = scene.add(canvas, Node::new(NodeContent::Chrome(ChromeState::Window(WindowState::new("w1")))));
    let term = scene.add(w1_chrome, Node::new(NodeContent::Widget(WidgetState::Terminal(TerminalState::new(Size { width: 10, height: 5 }, 100)))));
    if let Some(WidgetState::Terminal(t)) = scene.node_mut(term).widget() {
        t.feed(b"hello");
    }
    let layout = even_window_layout(&["w1".into()]);
    let mut mount_root = None;
    mount_window_layout(&mut scene, canvas, &layout, &[("w1".into(), w1_chrome)], &mut mount_root);
    assert!(mount_root.is_some());
    assert_ne!(scene.node(w1_chrome).parent, Some(canvas), "window chrome must leave the canvas direct-child list");
    if let Some(WidgetState::Terminal(t)) = scene.node_mut(term).widget() {
        let has_h = (0..t.screen.size.height).any(|y| (0..t.screen.size.width).any(|x| t.screen.cell_at(x, y).map(|c| c.ch) == Some('h')));
        assert!(has_h);
    }
}

#[test]
fn wizard_keys_filter_navigate_back_and_activate() {
    let mut widget = WidgetState::Wizard(WizardState::new(vec!["build".into(), "dev".into(), "test".into()]));
    let down = KeyEvent { key: Key::Down, mods: 0 };
    assert_eq!(widget.on_key(&down), Some(WidgetSignal::SelectionChanged(1)));
    let e = KeyEvent { key: Key::Char('e'), mods: 0 };
    let v = KeyEvent { key: Key::Char('v'), mods: 0 };
    assert_eq!(widget.on_key(&e), None);
    assert_eq!(widget.on_key(&v), None);
    let WidgetState::Wizard(w) = &widget else { unreachable!() };
    assert_eq!(w.filter, "ev");
    let enter = KeyEvent { key: Key::Enter, mods: 0 };
    assert_eq!(widget.on_key(&enter), Some(WidgetSignal::Activated(1)));
    let mut empty = WidgetState::Wizard(WizardState::new(vec!["only".into()]));
    let backspace = KeyEvent { key: Key::Backspace, mods: 0 };
    assert_eq!(empty.on_key(&backspace), Some(WidgetSignal::NavigateBack));
}

#[test]
fn tui_dispatch_emits_window_close_signal_on_click() {
    let mut tui = crate::tui::engine::Tui::new(Size { width: 40, height: 12 }, Theme::new(AppearanceName::Dark));
    let navbar = NavbarState { left: vec![], center: vec![], right: vec![] };
    let footer = FooterState { hints: vec![], status: String::new() };
    let layout = even_window_layout(&["plugins".to_string()]);
    let built = shell(&mut tui.scene, navbar, footer, &layout);
    tui.render_full();
    let (_, window_id) = built.windows[0].clone();
    let rect = tui.scene.rect(window_id);
    let mut buf = CellBuffer::new(Size { width: rect.width, height: rect.height }, Cell::blank([0, 0, 0], [0, 0, 0]));
    ChromeState::Window(WindowState::new("plugins")).paint(&Theme::new(AppearanceName::Dark), Rect::new(0, 0, rect.width, rect.height), &mut buf);
    let close_x = (0..rect.width).find(|&x| buf.get(x, 1).unwrap().ch == '\u{2715}').expect("close glyph rendered");
    let signals = tui.dispatch(&Event::Mouse(MouseEvent { kind: MouseKind::Down(0), pos: Pos { x: rect.x + close_x, y: rect.y + 1 }, mods: 0 }));
    assert_eq!(signals, vec![(window_id, WidgetSignal::WindowClose)]);
}

fn sample_table() -> TableState {
    let columns = vec![TableColumn::new("Plugin / App", 0, TableAlign::Left), TableColumn::new("React", 6, TableAlign::Right)];
    let rows = vec![
        TableRow::parent("puzzle", vec!["puzzle".into(), "".into()]),
        TableRow::child("puzzle2d", vec!["puzzle2d".into(), "6012".into()], 1),
        TableRow::child("puzzle3d", vec!["puzzle3d".into(), "6013".into()], 1),
        TableRow::parent("draw", vec!["draw".into(), "".into()]),
        TableRow::child("draw", vec!["draw".into(), "6064".into()], 1),
    ];
    TableState::new(columns, rows)
}

#[test]
fn table_header_is_bold_muted_with_a_hairline_underline_and_row_separators() {
    let theme = Theme::new(AppearanceName::Dark);
    let table = sample_table();
    let rect = Rect::new(0, 0, 40, 12);
    let mut buf = CellBuffer::new(Size { width: 40, height: 12 }, Cell::blank([0, 0, 0], [0, 0, 0]));
    WidgetState::Table(table).paint(&theme, rect, &mut buf, false);

    assert_eq!(row_text(&buf, 0).trim_end(), "Plugin / App                       React");
    assert_eq!(buf.get(0, 0).unwrap().fg, theme.role(Role::MutedForeground));
    assert_eq!(buf.get(0, 0).unwrap().attrs & attr::BOLD, attr::BOLD, "header must be bold");
    assert_eq!(row_text(&buf, 1), "\u{2500}".repeat(40), "hairline underline missing below the header");
    // no vertical rules anywhere in the header/underline rows
    assert!(!row_text(&buf, 0).contains('\u{2502}'));

    assert_eq!(row_text(&buf, 2).trim_end(), "\u{25be} puzzle");
    assert_eq!(row_text(&buf, 3), "\u{2500}".repeat(40), "row separator missing after the first row");
    assert_eq!(row_text(&buf, 4).trim_end(), "    puzzle2d                        6012");
}

#[test]
fn table_visible_indices_skips_children_of_a_collapsed_parent() {
    let mut table = sample_table();
    assert_eq!(table.visible_indices(), vec![0, 1, 2, 3, 4]);
    table.rows[0].expanded = false;
    assert_eq!(table.visible_indices(), vec![0, 3, 4], "puzzle's children must be hidden while collapsed");
}

#[test]
fn table_on_key_navigates_visible_rows_toggles_and_activates_leaves() {
    let mut table = sample_table();
    table.rows[0].expanded = false;
    table.selected = 0;
    let mut widget = WidgetState::Table(table);
    // Down should skip the hidden puzzle2d/puzzle3d children and land on "draw"
    let down = KeyEvent { key: Key::Down, mods: 0 };
    assert_eq!(widget.on_key(&down), Some(WidgetSignal::SelectionChanged(3)));
    let WidgetState::Table(table) = &mut widget else { unreachable!() };
    assert_eq!(table.selected, 3);
    table.selected = 0;

    let enter = KeyEvent { key: Key::Enter, mods: 0 };
    assert_eq!(widget.on_key(&enter), Some(WidgetSignal::SelectionChanged(0)));
    let WidgetState::Table(table) = &mut widget else { unreachable!() };
    assert!(table.rows[0].expanded, "Enter on a collapsed parent should expand it");
    table.selected = 1;

    assert_eq!(widget.on_key(&enter), Some(WidgetSignal::Activated(1)), "Enter on a leaf should activate it");
}

#[test]
fn table_selected_row_uses_active_base_only_when_focused() {
    let theme = Theme::new(AppearanceName::Dark);
    let mut table = sample_table();
    table.selected = 1;
    let rect = Rect::new(0, 0, 40, 12);

    let mut unfocused = CellBuffer::new(Size { width: 40, height: 12 }, Cell::blank([0, 0, 0], [0, 0, 0]));
    WidgetState::Table(TableState::new(std::mem::take(&mut table.columns), std::mem::take(&mut table.rows))).paint(&theme, rect, &mut unfocused, false);
    assert_ne!(unfocused.get(4, 4).unwrap().bg, theme.role(Role::ActiveBase), "unfocused selection must not fill with the accent");

    let mut table2 = sample_table();
    table2.selected = 1;
    let mut focused = CellBuffer::new(Size { width: 40, height: 12 }, Cell::blank([0, 0, 0], [0, 0, 0]));
    WidgetState::Table(table2).paint(&theme, rect, &mut focused, true);
    assert_eq!(focused.get(4, 4).unwrap().bg, theme.role(Role::ActiveBase));
    assert_eq!(focused.get(4, 4).unwrap().fg, theme.role(Role::ActiveForeground));
}

#[test]
fn table_flex_column_fills_remaining_width_and_right_aligns_numeric_column() {
    let theme = Theme::new(AppearanceName::Dark);
    let table = sample_table();
    let rect = Rect::new(0, 0, 20, 6);
    let mut buf = CellBuffer::new(Size { width: 20, height: 6 }, Cell::blank([0, 0, 0], [0, 0, 0]));
    WidgetState::Table(table).paint(&theme, rect, &mut buf, false);
    let header = row_text(&buf, 0);
    assert_eq!(header.len(), 20);
    assert!(header.trim_end().ends_with("React"), "the fixed-width numeric column should sit right-aligned at the row's end: {header:?}");
    let _ = theme.surface(Surface::Window);
}

#[test]
fn diff_emits_minimal_runs() {
    let blank = Cell::blank([0, 0, 0], [0, 0, 0]);
    let prev = CellBuffer::new(Size { width: 10, height: 2 }, blank);
    let mut next = prev.clone();
    next.put(3, 0, Cell { ch: 'x', ..blank });
    let runs = diff(&prev, &next);
    assert_eq!(runs.len(), 1);
    assert_eq!(runs[0], DiffRun { y: 0, x: 3, len: 1 });
}

#[test]
fn diff_merges_nearby_runs() {
    let blank = Cell::blank([0, 0, 0], [0, 0, 0]);
    let prev = CellBuffer::new(Size { width: 20, height: 1 }, blank);
    let mut next = prev.clone();
    next.put(0, 0, Cell { ch: 'a', ..blank });
    next.put(3, 0, Cell { ch: 'b', ..blank });
    let runs = diff(&prev, &next);
    assert_eq!(runs.len(), 1);
    assert_eq!(runs[0].len, 4);
}

#[test]
fn parser_roundtrip_arrow_keys_with_modifiers() {
    let mut parser = AnsiParser::new();
    let mut events = Vec::new();
    parser.feed(b"\x1b[1;5C", &mut events);
    assert_eq!(events.len(), 1);
    match &events[0] {
        Event::Key(k) => {
            assert_eq!(k.key, Key::Right);
            assert_eq!(k.mods, crate::tui::event::mods::CTRL);
        }
        _ => panic!("expected key event"),
    }
}

#[test]
fn parser_sgr_mouse_click() {
    let mut parser = AnsiParser::new();
    let mut events = Vec::new();
    parser.feed(b"\x1b[<0;10;5M", &mut events);
    assert_eq!(events.len(), 1);
    match &events[0] {
        Event::Mouse(m) => {
            assert_eq!(m.pos.x, 9);
            assert_eq!(m.pos.y, 4);
        }
        _ => panic!("expected mouse event"),
    }
}

#[test]
fn parser_bracketed_paste() {
    let mut parser = AnsiParser::new();
    let mut events = Vec::new();
    parser.feed(b"\x1b[200~hello\x1b[201~", &mut events);
    assert_eq!(events, vec![Event::Paste("hello".to_string())]);
}

#[test]
fn parser_lone_esc_flushes_on_timeout() {
    let mut parser = AnsiParser::new();
    let mut events = Vec::new();
    parser.feed(b"\x1b", &mut events);
    assert!(events.is_empty());
    parser.flush_escape(&mut events);
    assert_eq!(events, vec![Event::Key(KeyEvent { key: Key::Esc, mods: 0 })]);
}

#[test]
fn parser_split_utf8_across_feeds() {
    let mut parser = AnsiParser::new();
    let mut events = Vec::new();
    let bytes = "ü".as_bytes();
    parser.feed(&bytes[..1], &mut events);
    assert!(events.is_empty());
    parser.feed(&bytes[1..], &mut events);
    assert_eq!(events, vec![Event::Key(KeyEvent { key: Key::Char('ü'), mods: 0 })]);
}

#[test]
fn text_display_width_and_truncate() {
    assert_eq!(display_width("abc"), 3);
    let (s, w) = truncate_to("abcdef", 3);
    assert_eq!(s, "abc");
    assert_eq!(w, 3);
}

#[test]
fn theme_light_and_dark_differ() {
    use crate::tui::theme::{Role, Surface};
    let light = Theme::new(AppearanceName::Light);
    let dark = Theme::new(AppearanceName::Dark);
    assert_ne!(light.surface(Surface::Base), dark.surface(Surface::Base));
    for surface in [Surface::Base, Surface::Window, Surface::Pane, Surface::Panel, Surface::Dialog, Surface::Menu] {
        let _ = light.surface(surface);
        let _ = dark.surface(surface);
    }
    assert_ne!(light.surface(Surface::Base), light.surface(Surface::Menu));
    let _ = light.role(Role::HoverInteractive);
}

#[test]
fn wasm_host_feed_and_render_smoke() {
    let mut host = crate::tui::host::WasmHost::new(40, 10, true);
    host.feed(b"\r");
    let patch = host.render();
    assert!(!patch.is_empty());
}

//#region ???Geometry
#[test]
fn rect_contains_checks_boundaries() {
    let r = Rect::new(2, 2, 3, 3);
    assert!(r.contains(Pos { x: 2, y: 2 }));
    assert!(r.contains(Pos { x: 4, y: 4 }));
    assert!(!r.contains(Pos { x: 5, y: 4 }), "the right edge is exclusive");
    assert!(!r.contains(Pos { x: 1, y: 2 }), "left of the rect is outside");
}

#[test]
fn rect_intersect_returns_overlap_or_default_on_miss() {
    let a = Rect::new(0, 0, 10, 10);
    let b = Rect::new(5, 5, 10, 10);
    assert_eq!(a.intersect(b), Rect::new(5, 5, 5, 5));
    let miss = Rect::new(20, 20, 5, 5);
    assert_eq!(a.intersect(miss), Rect::default());
}

#[test]
fn rect_inset_shrinks_and_clamps_when_margin_exceeds_size() {
    let r = Rect::new(0, 0, 10, 10);
    assert_eq!(r.inset(2), Rect::new(2, 2, 6, 6));
    let small = Rect::new(0, 0, 3, 3);
    let shrunk = small.inset(5);
    assert_eq!(shrunk, Rect { x: 3, y: 3, width: 0, height: 0 }, "an oversized margin clamps size to zero but the origin still shifts by min(margin, size)");
}

#[test]
fn rect_split_top_and_split_bottom_partition_rect() {
    let r = Rect::new(1, 1, 10, 10);
    let (top, rest) = r.split_top(3);
    assert_eq!(top, Rect::new(1, 1, 10, 3));
    assert_eq!(rest, Rect::new(1, 4, 10, 7));
    let (rest2, bottom) = r.split_bottom(4);
    assert_eq!(bottom, Rect::new(1, 7, 10, 4));
    assert_eq!(rest2, Rect::new(1, 1, 10, 6));
    let (top2, rest3) = r.split_top(50);
    assert_eq!(top2.height, 10, "rows beyond the rect's height clamp to the full height");
    assert_eq!(rest3.height, 0);
}
//#endregion ???Geometry

//#region ???Text
#[test]
fn text_char_cells_zero_and_wide() {
    assert_eq!(crate::tui::text::char_cells('a'), 1);
    assert_eq!(crate::tui::text::char_cells('世'), 2);
    assert_eq!(crate::tui::text::char_cells('\u{200b}'), 0, "a zero-width space occupies no cells");
}

#[test]
fn text_cell_width_unicode_goldens() {
    assert_eq!(display_width("Semio"), 5, "ASCII is one cell per scalar");
    assert_eq!(display_width("e\u{0301}"), 1, "combining accents add no cell");
    assert_eq!(display_width("a\u{0000}\u{001b}b"), 2, "C0 controls add no cell");
    assert_eq!(display_width("界面"), 4, "CJK wide scalars occupy two cells");
    assert_eq!(display_width("🙂"), 2, "emoji presentation scalars occupy two cells");
    assert_eq!(crate::tui::text::char_cells('\u{200d}'), 0, "the zero-width joiner adds no cell");
    assert_eq!(display_width("👩\u{200d}💻"), 4, "the cell renderer retains one glyph per emoji scalar and skips the joiner");
}

#[test]
fn truncate_to_stops_before_splitting_a_wide_char() {
    let (s, w) = truncate_to("a世b", 2);
    assert_eq!(s, "a", "the wide char doesn't fit in the remaining 1 cell, so it's dropped whole");
    assert_eq!(w, 1);
    let (s2, w2) = truncate_to("世", 2);
    assert_eq!(s2, "世");
    assert_eq!(w2, 2);
}
//#endregion ???Text

//#region ???Cell
#[test]
fn cell_buffer_get_returns_none_out_of_bounds() {
    let buf = CellBuffer::new(Size { width: 3, height: 3 }, Cell::blank([0, 0, 0], [0, 0, 0]));
    assert!(buf.get(3, 0).is_none());
    assert!(buf.get(0, 3).is_none());
    assert!(buf.get(2, 2).is_some());
}

#[test]
fn cell_buffer_put_pairs_and_orphans_wide_char_continuations() {
    let blank = Cell::blank([0, 0, 0], [0, 0, 0]);
    let mut buf = CellBuffer::new(Size { width: 5, height: 1 }, blank);
    buf.put(1, 0, Cell { ch: '?', width: 2, ..blank });
    assert_eq!(buf.get(1, 0).unwrap().width, 2);
    assert_eq!(buf.get(2, 0).unwrap().width, 0, "the wide char auto-populates its continuation cell");

    buf.put(2, 0, Cell { ch: '\0', width: 0, ..blank });
    assert_eq!(buf.get(2, 0).unwrap().width, 0, "a width-0 write next to a wide lead stays paired");

    buf.put(0, 0, Cell { ch: '\0', width: 0, ..blank });
    assert_eq!(buf.get(0, 0).unwrap().width, 0, "column 0 has no left neighbor to check, so width is left untouched");
}

#[test]
fn cell_buffer_put_clamps_wide_char_at_right_edge() {
    let blank = Cell::blank([0, 0, 0], [0, 0, 0]);
    let mut buf = CellBuffer::new(Size { width: 3, height: 1 }, blank);
    buf.put(2, 0, Cell { ch: '?', width: 2, ..blank });
    assert_eq!(buf.get(2, 0).unwrap().width, 1, "a wide char at the last column clamps to width 1 instead of overflowing");
}

#[test]
fn cell_buffer_put_str_counts_clipped_cells_but_only_draws_inside_clip() {
    let blank = Cell::blank([0, 0, 0], [0, 0, 0]);
    let mut buf = CellBuffer::new(Size { width: 10, height: 1 }, blank);
    let clip = Rect::new(2, 0, 4, 1);
    let written = buf.put_str(Pos { x: 0, y: 0 }, "abcdefgh", [1, 1, 1], [2, 2, 2], 0, clip);
    assert_eq!(written, 6, "cells advance past the clip's left edge even though nothing is drawn there");
    assert_eq!(buf.get(0, 0).unwrap().ch, ' ', "cells left of the clip stay untouched");
    assert_eq!(buf.get(2, 0).unwrap().ch, 'c');
    assert_eq!(buf.get(5, 0).unwrap().ch, 'f');
    assert_eq!(buf.get(6, 0).unwrap().ch, ' ', "cells right of the clip stay untouched");

    let out_of_row = buf.put_str(Pos { x: 2, y: 1 }, "zzz", [0, 0, 0], [0, 0, 0], 0, clip);
    assert_eq!(out_of_row, 0, "a row outside the clip's y-range writes nothing");
}

#[test]
fn cell_buffer_fill_rect_clips_to_buffer_bounds() {
    let blank = Cell::blank([0, 0, 0], [0, 0, 0]);
    let mut buf = CellBuffer::new(Size { width: 4, height: 4 }, blank);
    buf.fill_rect(Rect::new(2, 2, 10, 10), Cell { ch: '#', ..blank });
    assert_eq!(buf.get(2, 2).unwrap().ch, '#');
    assert_eq!(buf.get(3, 3).unwrap().ch, '#');
}

#[test]
fn cell_buffer_hline_and_vline_paint_expected_cells() {
    let blank = Cell::blank([0, 0, 0], [0, 0, 0]);
    let mut buf = CellBuffer::new(Size { width: 5, height: 5 }, blank);
    buf.hline(Pos { x: 1, y: 1 }, 3, '-', [1, 1, 1], [2, 2, 2]);
    assert_eq!(buf.get(1, 1).unwrap().ch, '-');
    assert_eq!(buf.get(3, 1).unwrap().ch, '-');
    assert_eq!(buf.get(0, 1).unwrap().ch, ' ');
    assert_eq!(buf.get(4, 1).unwrap().ch, ' ');

    buf.vline(Pos { x: 2, y: 0 }, 3, '|', [1, 1, 1], [2, 2, 2]);
    assert_eq!(buf.get(2, 0).unwrap().ch, '|');
    assert_eq!(buf.get(2, 2).unwrap().ch, '|');
    assert_eq!(buf.get(2, 3).unwrap().ch, ' ');
}

#[test]
fn diff_full_redraw_when_sizes_differ() {
    let blank = Cell::blank([0, 0, 0], [0, 0, 0]);
    let a = CellBuffer::new(Size { width: 5, height: 5 }, blank);
    let b = CellBuffer::new(Size { width: 6, height: 5 }, blank);
    let runs = diff(&a, &b);
    assert_eq!(runs, vec![DiffRun { y: 0, x: 0, len: 30 }]);
}
//#endregion ???Cell

//#region ???Ansi
#[test]
fn emit_runs_writes_cursor_move_and_truecolor_sgr() {
    let blank = Cell::blank([0, 0, 0], [0, 0, 0]);
    let mut buf = CellBuffer::new(Size { width: 5, height: 1 }, blank);
    buf.put(2, 0, Cell { ch: 'Q', fg: [10, 20, 30], bg: [40, 50, 60], attrs: attr::BOLD, width: 1 });
    let runs = vec![DiffRun { y: 0, x: 2, len: 1 }];
    let mut patch = AnsiPatch::default();
    emit_runs(&buf, &runs, &mut patch);
    assert!(patch.0.contains("\x1b[1;3H"), "cursor moves to the 1-indexed row/col");
    assert!(patch.0.contains(";38;2;10;20;30"), "foreground truecolor sgr");
    assert!(patch.0.contains(";48;2;40;50;60"), "background truecolor sgr");
    assert!(patch.0.contains(";1"), "bold attribute sgr");
    assert!(patch.0.ends_with('Q'));
}

#[test]
fn ansi_setup_and_teardown_sequences_contain_expected_escapes() {
    assert!(setup_sequence().contains("\x1b[?1049h"), "enters the alternate screen");
    assert!(setup_sequence().contains("\x1b[?25l"), "hides the cursor");
    assert!(teardown_sequence().contains("\x1b[?1049l"), "restores the primary screen");
    assert!(teardown_sequence().contains("\x1b[?25h"), "shows the cursor");
}

#[test]
fn parser_ctrl_char_maps_to_char_with_ctrl_mod() {
    let mut parser = AnsiParser::new();
    let mut events = Vec::new();
    parser.feed(&[0x01, 0x03], &mut events);
    assert_eq!(events, vec![Event::Key(KeyEvent { key: Key::Char('a'), mods: crate::tui::event::mods::CTRL }), Event::Key(KeyEvent { key: Key::Char('c'), mods: crate::tui::event::mods::CTRL })]);
}

#[test]
fn parser_alt_prefixed_char_sets_alt_mod() {
    let mut parser = AnsiParser::new();
    let mut events = Vec::new();
    parser.feed(b"\x1bj", &mut events);
    assert_eq!(events, vec![Event::Key(KeyEvent { key: Key::Char('j'), mods: crate::tui::event::mods::ALT })]);
}

#[test]
fn parser_backtab_and_focus_gained_lost() {
    let mut parser = AnsiParser::new();
    let mut events = Vec::new();
    parser.feed(b"\x1b[Z\x1b[I\x1b[O", &mut events);
    assert_eq!(events, vec![Event::Key(KeyEvent { key: Key::BackTab, mods: 0 }), Event::FocusGained, Event::FocusLost]);
}

#[test]
fn parser_tilde_navigation_and_function_keys() {
    let mut parser = AnsiParser::new();
    let mut events = Vec::new();
    parser.feed(b"\x1b[3~\x1b[15~", &mut events);
    assert_eq!(events, vec![Event::Key(KeyEvent { key: Key::Delete, mods: 0 }), Event::Key(KeyEvent { key: Key::F(5), mods: 0 })]);
}

#[test]
fn parser_ss3_function_keys() {
    let mut parser = AnsiParser::new();
    let mut events = Vec::new();
    parser.feed(b"\x1bOP\x1bOQ", &mut events);
    assert_eq!(events, vec![Event::Key(KeyEvent { key: Key::F(1), mods: 0 }), Event::Key(KeyEvent { key: Key::F(2), mods: 0 })]);
}

#[test]
fn parser_mouse_scroll_drag_and_release_kinds() {
    let mut parser = AnsiParser::new();
    let mut events = Vec::new();
    parser.feed(b"\x1b[<64;5;5M\x1b[<97;5;5M\x1b[<32;5;5M\x1b[<0;5;5m", &mut events);
    let kinds: Vec<MouseKind> = events
        .iter()
        .map(|e| match e {
            Event::Mouse(m) => m.kind,
            _ => panic!("expected mouse event"),
        })
        .collect();
    assert_eq!(kinds, vec![MouseKind::ScrollUp, MouseKind::ScrollDown, MouseKind::Drag(0), MouseKind::Up(0)]);
}

#[test]
fn parser_control_keys_enter_tab_backspace() {
    let mut parser = AnsiParser::new();
    let mut events = Vec::new();
    parser.feed(&[0x0d, 0x0a, 0x09, 0x7f, 0x08], &mut events);
    assert_eq!(
        events,
        vec![
            Event::Key(KeyEvent { key: Key::Enter, mods: 0 }),
            Event::Key(KeyEvent { key: Key::Enter, mods: 0 }),
            Event::Key(KeyEvent { key: Key::Tab, mods: 0 }),
            Event::Key(KeyEvent { key: Key::Backspace, mods: 0 }),
            Event::Key(KeyEvent { key: Key::Backspace, mods: 0 }),
        ]
    );
}
//#endregion ???Ansi

//#region ???Vt
fn vt_screen(w: u16, h: u16) -> crate::tui::vt::VtScreen {
    crate::tui::vt::VtScreen::new(Size { width: w, height: h }, 100)
}

fn vt_row(screen: &crate::tui::vt::VtScreen, y: u16) -> String {
    (0..screen.size.width).filter_map(|x| screen.cell_at(x, y)).map(|c| c.ch).filter(|&c| c != '\0').collect()
}

#[test]
fn vt_cursor_motion_cup_and_cuu() {
    let mut s = vt_screen(10, 5);
    s.feed(b"\x1b[3;4H");
    assert_eq!(s.cursor, Pos { x: 3, y: 2 });
    s.feed(b"\x1b[2A");
    assert_eq!(s.cursor, Pos { x: 3, y: 0 });
    s.feed(b"\x1b[1B\x1b[2C\x1b[1D");
    assert_eq!(s.cursor, Pos { x: 4, y: 1 });
}

#[test]
fn vt_wrap_at_edge() {
    let mut s = vt_screen(4, 3);
    s.feed(b"abcdX");
    assert_eq!(&vt_row(&s, 0)[..4], "abcd");
    assert_eq!(s.cell_at(0, 1).map(|c| c.ch), Some('X'));
}

#[test]
fn vt_scroll_region_decstbm_newline_scrolls_inside() {
    let mut s = vt_screen(5, 5);
    s.feed(b"[1;1HAAAAA[2;1HBBBBB[3;1HCCCCC[4;1HDDDDD[5;1HEEEEE");
    s.feed(b"[2;4r");
    assert_eq!((s.scroll_top, s.scroll_bottom), (1, 3));
    s.feed(
        b"[4;1H
",
    );
    assert_eq!(&vt_row(&s, 0)[..5], "AAAAA");
    assert_eq!(&vt_row(&s, 1)[..5], "CCCCC");
    assert_eq!(&vt_row(&s, 2)[..5], "DDDDD");
    assert_eq!(vt_row(&s, 3).trim(), "");
    assert_eq!(&vt_row(&s, 4)[..5], "EEEEE");
}

#[test]
fn vt_sgr_truecolor_sets_cell_fg_bg() {
    let mut s = vt_screen(5, 2);
    s.feed(b"\x1b[38;2;10;20;30;48;2;40;50;60mZ");
    let cell = s.cell_at(0, 0).expect("cell");
    assert_eq!(cell.ch, 'Z');
    assert_eq!(cell.fg, [10, 20, 30]);
    assert_eq!(cell.bg, [40, 50, 60]);
}

#[test]
fn vt_alt_screen_1049_preserves_primary() {
    let mut s = vt_screen(5, 3);
    s.feed(b"HELLO");
    assert_eq!(&vt_row(&s, 0)[..5], "HELLO");
    s.feed(b"\x1b[?1049h");
    assert!(s.alt_active);
    s.feed(b"ALT");
    assert_eq!(&vt_row(&s, 0)[..3], "ALT");
    s.feed(b"\x1b[?1049l");
    assert!(!s.alt_active);
    assert_eq!(&vt_row(&s, 0)[..5], "HELLO");
}

#[test]
fn vt_resize_clamps_cursor() {
    let mut s = vt_screen(10, 10);
    s.feed(b"\x1b[8;8H");
    assert_eq!(s.cursor, Pos { x: 7, y: 7 });
    s.resize(Size { width: 4, height: 3 });
    assert_eq!(s.size, Size { width: 4, height: 3 });
    assert_eq!(s.cursor, Pos { x: 3, y: 2 });
}
//#endregion ???Vt

//#region ???Scene
#[test]
fn scene_node_mut_setters_update_content_and_visibility() {
    let mut scene = Scene::new();
    let root = scene.root();
    let id = scene.add(root, Node::new(NodeContent::Box));
    scene.node_mut(id).set_text("hi");
    match &scene.node(id).content {
        NodeContent::Text(s) => assert_eq!(s, "hi"),
        _ => panic!("expected text content"),
    }
    scene.node_mut(id).set_visible(false);
    assert!(!scene.node(id).visible);
}

#[test]
fn scene_widget_and_chrome_return_none_for_mismatched_content() {
    let mut scene = Scene::new();
    let root = scene.root();
    let id = scene.add(root, Node::new(NodeContent::Box));
    assert!(scene.node_mut(id).widget().is_none());
    assert!(scene.node_mut(id).chrome().is_none());
}

#[test]
fn scene_hit_finds_deepest_visible_node_topmost_child_wins() {
    let mut scene = Scene::new();
    let root = scene.root();
    scene.node_mut(root).set_constraint(Constraint { direction: Direction::Stack, ..Default::default() });
    let a = scene.add(root, Node::new(NodeContent::Box));
    let b = scene.add(root, Node::new(NodeContent::Box));
    solve(&mut scene, Rect::new(0, 0, 10, 10));
    assert_eq!(scene.hit(Pos { x: 5, y: 5 }), Some(b), "the last-added, topmost child wins on overlap");
    scene.node_mut(b).set_visible(false);
    assert_eq!(scene.hit(Pos { x: 5, y: 5 }), Some(a), "an invisible node is skipped by hit-testing");
    assert_eq!(scene.hit(Pos { x: 20, y: 20 }), None, "outside every rect");
}

#[test]
fn scene_remove_detaches_subtree_from_parent() {
    let mut scene = Scene::new();
    let root = scene.root();
    let a = scene.add(root, Node::new(NodeContent::Box));
    scene.add(a, Node::new(NodeContent::Box));
    scene.remove(a);
    assert!(!scene.node(root).children().contains(&a));
}

#[test]
#[should_panic(expected = "stale NodeId")]
fn scene_node_panics_after_its_id_is_removed() {
    let mut scene = Scene::new();
    let root = scene.root();
    let a = scene.add(root, Node::new(NodeContent::Box));
    scene.remove(a);
    scene.node(a);
}
//#endregion ???Scene

//#region ???Layout
#[test]
fn layout_column_direction_stacks_children_vertically() {
    let mut scene = Scene::new();
    let root = scene.root();
    scene.node_mut(root).set_constraint(Constraint { direction: Direction::Column, ..Default::default() });
    let a = scene.add(root, Node::new(NodeContent::Box));
    let b = scene.add(root, Node::new(NodeContent::Box));
    scene.node_mut(a).set_constraint(Constraint { height: Dimension::Cells(3), ..Default::default() });
    scene.node_mut(b).set_constraint(Constraint { height: Dimension::Weight(1), ..Default::default() });
    solve(&mut scene, Rect::new(0, 0, 10, 10));
    assert_eq!(scene.rect(a), Rect::new(0, 0, 10, 3));
    assert_eq!(scene.rect(b), Rect::new(0, 3, 10, 7));
}

#[test]
fn layout_distribute_weight_remainder_goes_to_earliest_child_on_ties() {
    let mut scene = Scene::new();
    let root = scene.root();
    scene.node_mut(root).set_constraint(Constraint { direction: Direction::Row, ..Default::default() });
    let a = scene.add(root, Node::new(NodeContent::Box));
    let b = scene.add(root, Node::new(NodeContent::Box));
    let c = scene.add(root, Node::new(NodeContent::Box));
    for id in [a, b, c] {
        scene.node_mut(id).set_constraint(Constraint { width: Dimension::Weight(1), ..Default::default() });
    }
    solve(&mut scene, Rect::new(0, 0, 10, 5));
    let widths = [scene.rect(a).width, scene.rect(b).width, scene.rect(c).width];
    assert_eq!(widths.iter().sum::<u16>(), 10);
    assert_eq!(widths[0], 4, "equal fractional remainders are broken by original order");
}

#[test]
fn window_layout_split_resize_move_zoom_and_tabs() {
    use crate::tui::layout::{WindowLayout, WindowLayoutRoot, WindowLayoutStackNode, WindowLayoutWindowNode, activate_stack_tab, cycle_stack_tab, move_window_to_stack, resize_window, solve_window_layout, split_window, zoom_window};
    let mut layout =
        WindowLayout { root: WindowLayoutRoot::Stack(WindowLayoutStackNode { size: None, active_window_kind_id: Some("a".into()), children: vec![WindowLayoutWindowNode { window_kind_id: "a".into(), title: None, corner: None }] }), zoomed: None };
    assert!(split_window(&mut layout, "a", "row", "b", None));
    let measures = solve_window_layout(&layout, Rect::new(0, 0, 100, 10));
    assert_eq!(measures.len(), 2);
    assert!(resize_window(&mut layout, "a", 0.5));
    assert!(move_window_to_stack(&mut layout, "b", "a"));
    let measures = solve_window_layout(&layout, Rect::new(0, 0, 100, 10));
    assert_eq!(measures.len(), 1);
    assert_eq!(measures[0].stack_tabs, vec!["a", "b"]);
    assert!(activate_stack_tab(&mut layout, "b"));
    assert!(cycle_stack_tab(&mut layout, "b", 1));
    zoom_window(&mut layout, Some("a"));
    let zoomed = solve_window_layout(&layout, Rect::new(0, 0, 80, 12));
    assert_eq!(zoomed.len(), 1);
    assert_eq!(zoomed[0].window_kind_id, "a");
    assert_eq!(zoomed[0].rect, Rect::new(0, 0, 80, 12));
    zoom_window(&mut layout, None);
}

//#endregion ???Layout

//#region ???Widget
#[test]
fn list_on_key_boundaries_toggle_and_activate() {
    let mut widget = WidgetState::List(ListState::new(vec!["a".to_string(), "b".to_string()]));
    let up = KeyEvent { key: Key::Up, mods: 0 };
    assert_eq!(widget.on_key(&up), None, "already at the top");
    let down = KeyEvent { key: Key::Down, mods: 0 };
    assert_eq!(widget.on_key(&down), Some(WidgetSignal::SelectionChanged(1)));
    assert_eq!(widget.on_key(&down), None, "already at the bottom");
    let space = KeyEvent { key: Key::Char(' '), mods: 0 };
    assert_eq!(widget.on_key(&space), Some(WidgetSignal::Toggled(true)));
    assert_eq!(widget.on_key(&space), Some(WidgetSignal::Toggled(false)));
    let enter = KeyEvent { key: Key::Enter, mods: 0 };
    assert_eq!(widget.on_key(&enter), Some(WidgetSignal::Activated(1)));

    let mut empty_widget = WidgetState::List(ListState::new(vec![]));
    assert_eq!(empty_widget.on_key(&space), None, "toggling with no items is a no-op");
}

#[test]
fn select_on_key_wraps_and_ignores_empty_options() {
    let mut widget = WidgetState::Select(SelectState { label: "L".to_string(), options: vec!["x".to_string(), "y".to_string()], index: 0 });
    let left = KeyEvent { key: Key::Left, mods: 0 };
    assert_eq!(widget.on_key(&left), Some(WidgetSignal::SelectionChanged(1)), "Left from index 0 wraps to the last option");
    let right = KeyEvent { key: Key::Right, mods: 0 };
    assert_eq!(widget.on_key(&right), Some(WidgetSignal::SelectionChanged(0)));

    let mut empty_widget = WidgetState::Select(SelectState { label: "L".to_string(), options: vec![], index: 0 });
    assert_eq!(empty_widget.on_key(&right), None);
}

#[test]
fn tabs_on_key_wraps_and_ignores_empty_tabs() {
    let mut widget = WidgetState::Tabs(TabsState { tabs: vec!["one".to_string(), "two".to_string()], active: 0 });
    let left = KeyEvent { key: Key::Left, mods: 0 };
    assert_eq!(widget.on_key(&left), Some(WidgetSignal::TabChanged(1)), "Left from index 0 wraps to the last tab");
    let right = KeyEvent { key: Key::Right, mods: 0 };
    assert_eq!(widget.on_key(&right), Some(WidgetSignal::TabChanged(0)));

    let mut empty_widget = WidgetState::Tabs(TabsState { tabs: vec![], active: 0 });
    assert_eq!(empty_widget.on_key(&right), None);
}

#[test]
fn input_on_key_inserts_utf8_and_respects_cursor_bounds() {
    let mut widget = WidgetState::Input(InputState { value: String::new(), cursor: 0, placeholder: "ph".to_string() });
    let type_u = KeyEvent { key: Key::Char('ü'), mods: 0 };
    assert_eq!(widget.on_key(&type_u), Some(WidgetSignal::ValueChanged("ü".to_string())));
    let cursor_after_insert = match &widget {
        WidgetState::Input(i) => i.cursor,
        _ => unreachable!(),
    };
    assert_eq!(cursor_after_insert, 'ü'.len_utf8());

    let left = KeyEvent { key: Key::Left, mods: 0 };
    assert_eq!(widget.on_key(&left), None);
    let cursor_after_left = match &widget {
        WidgetState::Input(i) => i.cursor,
        _ => unreachable!(),
    };
    assert_eq!(cursor_after_left, 1, "Left steps by one byte, not a full UTF-8 char boundary");
    assert_eq!(widget.on_key(&left), None);
    let cursor_after_second_left = match &widget {
        WidgetState::Input(i) => i.cursor,
        _ => unreachable!(),
    };
    assert_eq!(cursor_after_second_left, 0);
    assert_eq!(widget.on_key(&left), None, "already at cursor 0");

    let backspace = KeyEvent { key: Key::Backspace, mods: 0 };
    assert_eq!(widget.on_key(&backspace), None, "nothing before the cursor to delete");

    let right = KeyEvent { key: Key::Right, mods: 0 };
    assert_eq!(widget.on_key(&right), None);
    assert_eq!(widget.on_key(&right), None);
    assert_eq!(widget.on_key(&right), None, "already at the end");

    assert_eq!(widget.on_key(&backspace), Some(WidgetSignal::ValueChanged(String::new())), "backspace at the end removes the multi-byte char");
}

#[test]
fn log_state_push_evicts_oldest_beyond_capacity() {
    let mut log = LogState::new(3);
    log.push("a");
    log.push("b");
    log.push("c");
    log.push("d");
    assert_eq!(log.lines().len(), 3);
    assert_eq!(log.lines().front(), Some(&"b".to_string()), "the oldest line is evicted once capacity is exceeded");
    log.clear();
    assert!(log.lines().is_empty());
    assert_eq!(log.scroll, LogScroll::Follow, "clearing resets scroll to Follow");
}

#[test]
fn log_on_key_page_and_home_end_scroll_states() {
    let mut log = LogState::new(20);
    for i in 0..20 {
        log.push(&format!("l{i}"));
    }
    let mut widget = WidgetState::Log(log);

    let page_down = KeyEvent { key: Key::PageDown, mods: 0 };
    widget.on_key(&page_down);
    assert_eq!(log_scroll(&widget), LogScroll::Follow, "PageDown while already following is a no-op");

    let home = KeyEvent { key: Key::Home, mods: 0 };
    widget.on_key(&home);
    assert_eq!(log_scroll(&widget), LogScroll::At(0));

    widget.on_key(&page_down);
    assert_eq!(log_scroll(&widget), LogScroll::At(10), "PageDown short of the end scrolls forward by 10");

    widget.on_key(&page_down);
    assert_eq!(log_scroll(&widget), LogScroll::Follow, "PageDown past the end resumes following");

    let page_up = KeyEvent { key: Key::PageUp, mods: 0 };
    widget.on_key(&page_up);
    assert_eq!(log_scroll(&widget), LogScroll::At(19), "PageUp from Follow jumps to the last line");

    widget.on_key(&page_up);
    assert_eq!(log_scroll(&widget), LogScroll::At(9), "a second PageUp scrolls back 10 more lines");

    let end = KeyEvent { key: Key::End, mods: 0 };
    widget.on_key(&end);
    assert_eq!(log_scroll(&widget), LogScroll::Follow);
}

fn log_scroll(widget: &WidgetState) -> LogScroll {
    match widget {
        WidgetState::Log(l) => l.scroll,
        _ => unreachable!(),
    }
}

#[test]
fn table_on_key_left_right_expand_collapse_and_boundaries() {
    let mut table = sample_table();
    table.selected = 0;
    let mut widget = WidgetState::Table(table);
    let up = KeyEvent { key: Key::Up, mods: 0 };
    assert_eq!(widget.on_key(&up), None, "already at the first visible row");

    let left = KeyEvent { key: Key::Left, mods: 0 };
    assert_eq!(widget.on_key(&left), Some(WidgetSignal::SelectionChanged(0)), "Left on an expanded parent collapses it");
    assert_eq!(widget.on_key(&left), None, "Left on an already-collapsed parent is a no-op");

    let right = KeyEvent { key: Key::Right, mods: 0 };
    assert_eq!(widget.on_key(&right), Some(WidgetSignal::SelectionChanged(0)));
    assert_eq!(widget.on_key(&right), None, "Right on an already-expanded parent is a no-op");

    let WidgetState::Table(t) = &mut widget else { unreachable!() };
    t.selected = 1;
    assert_eq!(widget.on_key(&left), None, "Left on a leaf row is a no-op");
    assert_eq!(widget.on_key(&right), None, "Right on a leaf row is a no-op");

    let mut empty_widget = WidgetState::Table(TableState::new(vec![], vec![]));
    assert_eq!(empty_widget.on_key(&up), None, "no rows at all means every key is a no-op");
}

#[test]
fn widget_preferred_size_for_label_select_chip_and_divider() {
    let label = WidgetState::Label(LabelState { text: "hello".to_string(), align: Align::Left, role: Role::Foreground });
    assert_eq!(label.preferred_size(), Size { width: 5, height: 1 });

    let select = WidgetState::Select(SelectState { label: "L".to_string(), options: vec!["opt".to_string()], index: 0 });
    let expected = display_width("L \u{2039} opt \u{203a}");
    assert_eq!(select.preferred_size(), Size { width: expected, height: 1 });

    let chip = WidgetState::Chip(ChipState { label: "tag".to_string(), on: true });
    assert_eq!(chip.preferred_size(), Size { width: 5, height: 1 });

    let divider = WidgetState::Divider(DividerState::default());
    assert_eq!(divider.preferred_size(), Size { width: 1, height: 1 });

    let list = WidgetState::List(ListState::new(vec!["x".to_string()]));
    assert_eq!(list.preferred_size(), Size { width: 0, height: 0 }, "list has no measured preferred size");
}

#[test]
fn paint_list_highlights_selected_row_only_when_focused() {
    let theme = Theme::new(AppearanceName::Dark);
    let list = ListState { items: vec!["a".to_string(), "b".to_string()], selected: 1, offset: 0, marks: vec![false, true] };
    let rect = Rect::new(0, 0, 10, 2);
    let mut buf = CellBuffer::new(Size { width: 10, height: 2 }, Cell::blank([0, 0, 0], [0, 0, 0]));
    WidgetState::List(list).paint(&theme, rect, &mut buf, true);
    assert_eq!(buf.get(0, 1).unwrap().bg, theme.role(Role::ActiveBase));
    assert_ne!(buf.get(0, 0).unwrap().bg, theme.role(Role::ActiveBase));
    assert!(row_text(&buf, 1).starts_with('\u{2713}'), "a marked row shows a check prefix");
    assert!(row_text(&buf, 0).starts_with(' '), "an unmarked row has a blank prefix");
}

#[test]
fn paint_select_renders_label_and_current_option() {
    let theme = Theme::new(AppearanceName::Dark);
    let select = SelectState { label: "Mode".to_string(), options: vec!["A".to_string(), "B".to_string()], index: 1 };
    let rect = Rect::new(0, 0, 20, 1);
    let mut buf = CellBuffer::new(Size { width: 20, height: 1 }, Cell::blank([0, 0, 0], [0, 0, 0]));
    WidgetState::Select(select).paint(&theme, rect, &mut buf, true);
    assert_eq!(row_text(&buf, 0).trim_end(), "Mode: \u{2039} B \u{203a}");
    assert_eq!(buf.get(0, 0).unwrap().fg, theme.role(Role::Accent), "a focused select uses the accent color");
}

#[test]
fn paint_tabs_bolds_the_active_tab() {
    let theme = Theme::new(AppearanceName::Dark);
    let tabs = TabsState { tabs: vec!["One".to_string(), "Two".to_string()], active: 1 };
    let rect = Rect::new(0, 0, 20, 1);
    let mut buf = CellBuffer::new(Size { width: 20, height: 1 }, Cell::blank([0, 0, 0], [0, 0, 0]));
    WidgetState::Tabs(tabs).paint(&theme, rect, &mut buf, false);
    let t_x = (0..20).find(|&x| buf.get(x, 0).unwrap().ch == 'T').expect("active tab rendered");
    assert_eq!(buf.get(t_x, 0).unwrap().attrs & attr::BOLD, attr::BOLD);
    let o_x = (0..20).find(|&x| buf.get(x, 0).unwrap().ch == 'O').expect("inactive tab rendered");
    assert_eq!(buf.get(o_x, 0).unwrap().attrs & attr::BOLD, 0, "inactive tab is not bold");
}

#[test]
fn terminal_widget_scroll_search_and_passthrough() {
    let mut term = TerminalState::new(Size { width: 8, height: 3 }, 50);
    term.feed(b"line1\nline2\nline3\nline4\n");
    let mut widget = WidgetState::Terminal(term);
    let page = KeyEvent { key: Key::PageUp, mods: 0 };
    assert_eq!(widget.on_key(&page), None);
    let slash = KeyEvent { key: Key::Char('/'), mods: 0 };
    assert_eq!(widget.on_key(&slash), None);
    let a = KeyEvent { key: Key::Char('a'), mods: 0 };
    assert_eq!(widget.on_key(&a), Some(WidgetSignal::ValueChanged("a".into())));
    let esc = KeyEvent { key: Key::Esc, mods: 0 };
    assert_eq!(widget.on_key(&esc), None);
    let x = KeyEvent { key: Key::Char('x'), mods: 0 };
    assert_eq!(widget.on_key(&x), Some(WidgetSignal::TerminalPassthrough));
    let mut buf = CellBuffer::new(Size { width: 8, height: 3 }, Cell::blank([0, 0, 0], [0, 0, 0]));
    widget.paint(&Theme::new(AppearanceName::Dark), Rect::new(0, 0, 8, 3), &mut buf, true);
}

#[test]
fn window_stack_tabs_paint_on_body_top() {
    let theme = Theme::new(AppearanceName::Dark);
    let rect = Rect::new(0, 0, 50, 6);
    let mut buf = CellBuffer::new(Size { width: 50, height: 6 }, Cell::blank([0, 0, 0], [0, 0, 0]));
    let w = WindowState::new("Main").with_stack_tabs(vec!["a".into(), "b".into()], 1);
    ChromeState::Window(w).paint(&theme, rect, &mut buf);
    let row: String = (0..50).filter_map(|x| buf.get(x, 1).map(|c| c.ch)).collect();
    assert!(row.contains('a') && row.contains('b'), "corner stack tabs missing: {row:?}");
    assert!(row.contains('\u{2922}') && row.contains('\u{2715}'), "inline action glyphs missing: {row:?}");
}

#[test]
fn window_stack_tabs_respect_bottom_corners() {
    use crate::tui::chrome::WindowStackTabState;
    use crate::tui::layout::WindowStackCorner;
    let theme = Theme::new(AppearanceName::Dark);
    let rect = Rect::new(0, 0, 50, 8);
    let mut buf = CellBuffer::new(Size { width: 50, height: 8 }, Cell::blank([0, 0, 0], [0, 0, 0]));
    let w = WindowState::new("Main").with_stack_tab_states(vec![WindowStackTabState::new("top", WindowStackCorner::TopLeft), WindowStackTabState::new("bot", WindowStackCorner::BottomRight)], 1);
    ChromeState::Window(w).paint(&theme, rect, &mut buf);
    let top = row_text(&buf, 1);
    let bottom = row_text(&buf, 6);
    assert!(top.contains("top"), "top tab missing: {top:?}");
    assert!(bottom.contains("bot"), "bottom tab missing: {bottom:?}");
}

#[test]
fn paint_log_shows_the_tail_when_following() {
    let theme = Theme::new(AppearanceName::Dark);
    let mut log = LogState::new(10);
    for i in 0..5 {
        log.push(&format!("line{i}"));
    }
    let rect = Rect::new(0, 0, 10, 3);
    let mut buf = CellBuffer::new(Size { width: 10, height: 3 }, Cell::blank([0, 0, 0], [0, 0, 0]));
    WidgetState::Log(log).paint(&theme, rect, &mut buf, false);
    assert_eq!(row_text(&buf, 0).trim_end(), "line2");
    assert_eq!(row_text(&buf, 1).trim_end(), "line3");
    assert_eq!(row_text(&buf, 2).trim_end(), "line4");
}

#[test]
fn paint_input_shows_placeholder_and_draws_cursor_when_focused() {
    let theme = Theme::new(AppearanceName::Dark);
    let rect = Rect::new(0, 0, 20, 1);
    let empty_input = InputState { value: String::new(), cursor: 0, placeholder: "type here".to_string() };
    let mut buf = CellBuffer::new(Size { width: 20, height: 1 }, Cell::blank([0, 0, 0], [0, 0, 0]));
    WidgetState::Input(empty_input).paint(&theme, rect, &mut buf, false);
    assert_eq!(row_text(&buf, 0).trim_end(), "type here");
    assert_eq!(buf.get(0, 0).unwrap().fg, theme.role(Role::MutedForeground));

    let filled = InputState { value: "hi".to_string(), cursor: 2, placeholder: "ph".to_string() };
    let mut buf2 = CellBuffer::new(Size { width: 20, height: 1 }, Cell::blank([0, 0, 0], [0, 0, 0]));
    WidgetState::Input(filled).paint(&theme, rect, &mut buf2, true);
    assert_eq!(buf2.get(2, 0).unwrap().ch, '\u{2588}', "a focused input draws a cursor block at the caret position");
}

#[test]
fn paint_divider_centers_its_label() {
    let theme = Theme::new(AppearanceName::Dark);
    let divider = DividerState { label: Some("Hi".to_string()) };
    let rect = Rect::new(0, 0, 10, 1);
    let mut buf = CellBuffer::new(Size { width: 10, height: 1 }, Cell::blank([0, 0, 0], [0, 0, 0]));
    WidgetState::Divider(divider).paint(&theme, rect, &mut buf, false);
    let row = row_text(&buf, 0);
    assert!(row.contains("Hi"));
    assert_eq!(row.chars().next().unwrap(), '\u{2500}', "the hairline still draws under the label");
}

#[test]
fn paint_chip_reflects_on_and_off_colors() {
    let theme = Theme::new(AppearanceName::Dark);
    let rect = Rect::new(0, 0, 10, 1);
    let on = ChipState { label: "x".to_string(), on: true };
    let mut buf_on = CellBuffer::new(Size { width: 10, height: 1 }, Cell::blank([0, 0, 0], [0, 0, 0]));
    WidgetState::Chip(on).paint(&theme, rect, &mut buf_on, false);
    assert_eq!(buf_on.get(0, 0).unwrap().bg, theme.role(Role::Accent));

    let off = ChipState { label: "x".to_string(), on: false };
    let mut buf_off = CellBuffer::new(Size { width: 10, height: 1 }, Cell::blank([0, 0, 0], [0, 0, 0]));
    WidgetState::Chip(off).paint(&theme, rect, &mut buf_off, false);
    assert_eq!(buf_off.get(0, 0).unwrap().bg, theme.surface(Surface::Panel));
}
//#endregion ???Widget

//#region ???Chrome
#[test]
fn paint_navbar_places_left_center_and_right_items() {
    let theme = Theme::new(AppearanceName::Dark);
    let rect = Rect::new(0, 0, 30, 2);
    let mut buf = CellBuffer::new(Size { width: 30, height: 2 }, Cell::blank([0, 0, 0], [0, 0, 0]));
    let navbar = NavbarState {
        left: vec![NavItem { id: "l".to_string(), label: "L".to_string(), active: false }],
        center: vec![NavItem { id: "c".to_string(), label: "C".to_string(), active: false }],
        right: vec![NavItem { id: "r".to_string(), label: "R".to_string(), active: true }],
    };
    ChromeState::Navbar(navbar).paint(&theme, rect, &mut buf);
    let row = row_text(&buf, 0);
    assert!(row.trim_start().starts_with('L'));
    assert!(row.contains('C'));
    assert!(row.trim_end().ends_with('R'));
    assert_eq!(row_text(&buf, 1), "\u{2500}".repeat(30), "hairline drawn below the navbar");
    let right_x = (0..30).find(|&x| buf.get(x, 0).unwrap().ch == 'R').expect("right item rendered");
    assert_eq!(buf.get(right_x, 0).unwrap().fg, theme.role(Role::Accent), "the active item uses the accent color");
}

#[test]
fn paint_footer_renders_hints_then_status() {
    let theme = Theme::new(AppearanceName::Dark);
    let rect = Rect::new(0, 0, 30, 2);
    let mut buf = CellBuffer::new(Size { width: 30, height: 2 }, Cell::blank([0, 0, 0], [0, 0, 0]));
    let footer = FooterState { hints: vec![KeyHint { key: "q".to_string(), label: "quit".to_string() }], status: "OK".to_string() };
    ChromeState::Footer(footer).paint(&theme, rect, &mut buf);
    assert_eq!(row_text(&buf, 0), "\u{2500}".repeat(30), "hairline drawn above the footer");
    let row1 = row_text(&buf, 1);
    assert!(row1.contains('q'));
    assert!(row1.contains("quit"));
    assert!(row1.trim_end().ends_with("OK"));
}
//#endregion ???Chrome

//#region ???Engine

#[test]
fn engine_mouse_move_does_not_steal_focus() {
    use crate::tui::engine::Tui;
    let mut tui = Tui::new(Size { width: 20, height: 10 }, Theme::new(AppearanceName::Dark));
    let root = tui.scene.root();
    let a = tui.scene.add(root, Node::new(NodeContent::Widget(WidgetState::Label(LabelState { text: "a".into(), align: Align::Left, role: Role::Foreground }))));
    let b = tui.scene.add(root, Node::new(NodeContent::Widget(WidgetState::Label(LabelState { text: "b".into(), align: Align::Left, role: Role::Foreground }))));
    tui.scene.node_mut(root).set_constraint(Constraint { direction: Direction::Row, ..Default::default() });
    tui.scene.node_mut(a).set_constraint(Constraint { width: Dimension::Cells(10), ..Default::default() });
    tui.scene.node_mut(b).set_constraint(Constraint { width: Dimension::Cells(10), ..Default::default() });
    let _ = tui.render_full();
    tui.set_focus(Some(a));
    tui.dispatch(&Event::Mouse(MouseEvent { kind: MouseKind::Move, pos: Pos { x: 15, y: 0 }, mods: 0 }));
    assert_eq!(tui.focus(), Some(a), "move must not change focus");
    tui.dispatch(&Event::Mouse(MouseEvent { kind: MouseKind::Down(0), pos: Pos { x: 15, y: 0 }, mods: 0 }));
    assert_eq!(tui.focus(), Some(b), "click focuses the hit widget");
}

#[test]
fn tui_focus_next_and_prev_cycle_through_focusables() {
    let mut tui = crate::tui::engine::Tui::new(Size { width: 20, height: 5 }, Theme::new(AppearanceName::Dark));
    let root = tui.scene.root();
    let a = tui.scene.add(root, Node::new(NodeContent::Widget(WidgetState::Label(LabelState { text: "a".to_string(), align: Align::Left, role: Role::Foreground }))));
    let b = tui.scene.add(root, Node::new(NodeContent::Widget(WidgetState::Label(LabelState { text: "b".to_string(), align: Align::Left, role: Role::Foreground }))));
    assert_eq!(tui.focus(), None);
    tui.focus_next();
    assert_eq!(tui.focus(), Some(a));
    tui.focus_next();
    assert_eq!(tui.focus(), Some(b));
    tui.focus_next();
    assert_eq!(tui.focus(), Some(a), "focus wraps back to the first focusable");
    tui.focus_prev();
    assert_eq!(tui.focus(), Some(b), "focus wraps backward past the first focusable");
}

#[test]
fn tui_dispatch_tab_and_backtab_move_focus() {
    let mut tui = crate::tui::engine::Tui::new(Size { width: 20, height: 5 }, Theme::new(AppearanceName::Dark));
    let root = tui.scene.root();
    let a = tui.scene.add(root, Node::new(NodeContent::Widget(WidgetState::Label(LabelState { text: "a".to_string(), align: Align::Left, role: Role::Foreground }))));
    let b = tui.scene.add(root, Node::new(NodeContent::Widget(WidgetState::Label(LabelState { text: "b".to_string(), align: Align::Left, role: Role::Foreground }))));
    let tab_ev = Event::Key(KeyEvent { key: Key::Tab, mods: 0 });
    assert_eq!(tui.dispatch(&tab_ev), vec![]);
    assert_eq!(tui.focus(), Some(a));
    let backtab_ev = Event::Key(KeyEvent { key: Key::BackTab, mods: 0 });
    tui.dispatch(&backtab_ev);
    assert_eq!(tui.focus(), Some(b));
}

#[test]
fn tui_render_skips_repaint_when_nothing_dirty() {
    let mut tui = crate::tui::engine::Tui::new(Size { width: 10, height: 3 }, Theme::new(AppearanceName::Dark));
    let first = tui.render_full();
    assert!(!first.0.is_empty());
    let second = tui.render();
    assert!(second.0.is_empty(), "no dirty state and no forced redraw emits nothing");
}

#[test]
fn tui_set_appearance_and_resize_force_full_redraw() {
    let mut tui = crate::tui::engine::Tui::new(Size { width: 10, height: 3 }, Theme::new(AppearanceName::Dark));
    tui.render_full();
    tui.set_appearance(AppearanceName::Light);
    let patch = tui.render();
    assert!(!patch.0.is_empty(), "set_appearance forces a full repaint even with no dirty nodes");
    tui.render();
    tui.resize(Size { width: 12, height: 4 });
    let patch2 = tui.render();
    assert!(!patch2.0.is_empty(), "resize forces a full repaint too");
}
//#endregion ???Engine

//#region ???WasmHost
#[test]
fn wasm_host_setup_and_teardown_match_ansi_sequences() {
    let host = crate::tui::host::WasmHost::new(10, 5, true);
    assert_eq!(host.setup(), setup_sequence());
    assert_eq!(host.teardown(), teardown_sequence());
}

#[test]
fn wasm_host_resize_updates_engine_size() {
    let mut host = crate::tui::host::WasmHost::new(10, 5, true);
    host.render();
    host.resize(20, 8);
    let patch = host.render();
    assert!(!patch.is_empty(), "resizing triggers a full repaint on the next render");
}
//#endregion ???WasmHost

//#region ???Clipboard
#[test]
fn osc52_copy_sequence_is_base64_payload() {
    let seq = crate::tui::backend::osc52_copy_sequence("hi");
    assert_eq!(seq.as_bytes()[0], 0x1b);
    assert!(seq.contains("]52;c;aGk="), "got {seq:?}");
    assert_eq!(*seq.as_bytes().last().unwrap(), 0x07);
    let mut mem = crate::tui::backend::MemoryClipboard::default();
    crate::tui::backend::Clipboard::enqueue_copy(&mut mem, "abc".to_string());
    assert!(matches!(crate::tui::backend::Clipboard::poll(&mut mem), Some(crate::tui::backend::ClipboardResult::Copied)));
    crate::tui::backend::Clipboard::enqueue_paste(&mut mem);
    assert!(matches!(
        crate::tui::backend::Clipboard::poll(&mut mem),
        Some(crate::tui::backend::ClipboardResult::Pasted(text)) if text == "abc"
    ));
}
//#endregion ???Clipboard

//#region ???Pty
#[cfg(all(unix, feature = "tui-terminal"))]
#[test]
fn pty_spawn_echo_hello() {
    use crate::tui::pty::{Pty, PtySize};
    use std::time::{Duration, Instant};

    let mut pty = Pty::spawn("/bin/echo", &["hello"], &[], None, PtySize { cols: 80, rows: 24 }).expect("spawn echo");
    let mut out = Vec::new();
    let mut buf = [0u8; 1024];
    let deadline = Instant::now() + Duration::from_secs(2);
    loop {
        match pty.try_read(&mut buf) {
            Ok(0) => {
                let _ = pty.try_wait();
                if Instant::now() >= deadline {
                    break;
                }
                std::thread::sleep(Duration::from_millis(10));
            }
            Ok(n) => {
                out.extend_from_slice(&buf[..n]);
                if String::from_utf8_lossy(&out).contains("hello") {
                    break;
                }
            }
            Err(e) => panic!("try_read failed: {}", e.message),
        }
    }
    let text = String::from_utf8_lossy(&out);
    assert!(text.contains("hello"), "PTY output missing hello: {text:?}");
}

#[cfg(all(unix, feature = "tui-terminal"))]
#[test]
fn pty_kill_terminates_child_process_group() {
    use crate::tui::pty::{Pty, PtySize};
    use std::process as system_process;
    use std::time::{Duration, Instant};

    let mut pty = Pty::spawn("bash", &["-c", "sleep 300 & sleep 300; wait"], &[], None, PtySize { cols: 80, rows: 24 }).expect("spawn bash sleep group");
    let pid = pty.pid();
    let deadline = Instant::now() + Duration::from_secs(2);
    let mut saw_child = false;
    while Instant::now() < deadline {
        let out = system_process::Command::new("pgrep").args(["-P", &pid.to_string()]).output().expect("pgrep");
        if !out.stdout.is_empty() {
            saw_child = true;
            break;
        }
        std::thread::sleep(Duration::from_millis(50));
    }
    assert!(saw_child, "expected child processes under pty leader pid={pid}");
    pty.kill().expect("kill pty group");
    std::thread::sleep(Duration::from_millis(200));
    let out = system_process::Command::new("pgrep").args(["-P", &pid.to_string()]).output().expect("pgrep after kill");
    assert!(out.stdout.is_empty(), "child processes still alive after kill: {:?}", String::from_utf8_lossy(&out.stdout));
    let leader = system_process::Command::new("ps").args(["-p", &pid.to_string()]).output().expect("ps leader");
    assert!(leader.status.success() == false || leader.stdout.len() < 2, "pty leader still running");
}

#[cfg(all(unix, feature = "tui-terminal"))]
#[test]
fn pty_resize_ok() {
    use crate::tui::pty::{Pty, PtySize};

    let mut pty = Pty::spawn("/bin/sleep", &["1"], &[], None, PtySize { cols: 80, rows: 24 }).expect("spawn sleep");
    pty.resize(PtySize { cols: 100, rows: 40 }).expect("resize");
    pty.kill().expect("kill");
}
//#endregion ???Pty
