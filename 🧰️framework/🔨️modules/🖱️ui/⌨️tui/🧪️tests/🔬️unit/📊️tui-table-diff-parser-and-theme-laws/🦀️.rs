/// 📊️ Tui table, diff, parser and theme laws: the table helpers, header, rows, selection and flex columns, the cell diff, the ansi parser round trips, the text width, the theme and the wasm host smoke.

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

/// 🧮 No vertical rules appear anywhere in the header and underline rows.
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

/// 🧭 Down skips the hidden puzzle2d and puzzle3d children and lands on "draw".
#[test]
fn table_on_key_navigates_visible_rows_toggles_and_activates_leaves() {
    let mut table = sample_table();
    table.rows[0].expanded = false;
    table.selected = 0;
    let mut widget = WidgetState::Table(table);
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
            assert_eq!(k.mods, semio_framework_ui_tui_core::event::mods::CTRL);
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
    use semio_framework_ui_tui_core::theme::{Role, Surface};
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
    let mut host = semio_framework_ui_tui_host::WasmHost::new(40, 10, true);
    host.feed(b"\r");
    let patch = host.render();
    assert!(!patch.is_empty());
}
