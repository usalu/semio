// 🖱️ Vt pane pointer, selection and paint laws: the wheel, the child mouse reports, the selection and clicks, the scrollbar, the theme colours and the painted marks.

//#region 🖥️Pane
#[test]
fn wheel_scrolls_the_viewport_unless_the_child_tracks_the_pointer() {
    let mut term = pane(10, 3);
    term.feed(&numbered(12));
    let area = rect(&term);
    let wheel_up = pointer(MouseKind::Scroll { dx: 0, dy: -1 }, 3, 2, 0);
    assert_eq!(term.on_mouse(area, &wheel_up), None);
    assert_eq!(term.screen.view_offset(), 3);
    term.feed(b"\x1b[?1000h\x1b[?1006h");
    assert_eq!(term.on_mouse(area, &wheel_up), Some(WidgetSignal::TerminalInput(b"\x1b[<64;2;2M".to_vec())));
    let shift_wheel = pointer(MouseKind::Scroll { dx: 0, dy: -1 }, 3, 2, mods::SHIFT);
    let before = term.screen.view_offset();
    assert_eq!(term.on_mouse(area, &shift_wheel), None);
    assert_eq!(term.screen.view_offset(), before + 3);
}

#[test]
fn wheel_on_the_alternate_screen_sends_arrow_keys() {
    let mut term = pane(10, 3);
    term.feed(b"\x1b[?1049h");
    let area = rect(&term);
    let down = pointer(MouseKind::Scroll { dx: 0, dy: 1 }, 3, 2, 0);
    assert_eq!(term.on_mouse(area, &down), Some(WidgetSignal::TerminalInput(b"\x1b[B\x1b[B\x1b[B".to_vec())));
    term.feed(b"\x1b[?1007l");
    assert_eq!(term.on_mouse(area, &down), None);
}

#[test]
fn child_mouse_reports_use_pane_local_coordinates() {
    let mut term = pane(10, 5);
    term.feed(b"\x1b[?1002h\x1b[?1006h");
    let area = rect(&term);
    let press = pointer(MouseKind::Down(MouseButton::Left), area.x + 3, area.y + 1, 0);
    assert_eq!(term.on_mouse(area, &press), Some(WidgetSignal::TerminalInput(b"\x1b[<0;4;2M".to_vec())));
    let drag = pointer(MouseKind::Drag(MouseButton::Left), area.x + 4, area.y + 1, 0);
    assert_eq!(term.on_mouse(area, &drag), Some(WidgetSignal::TerminalInput(b"\x1b[<32;5;2M".to_vec())));
    let release = pointer(MouseKind::Up(MouseButton::Left), area.x + 4, area.y + 1, 0);
    assert_eq!(term.on_mouse(area, &release), Some(WidgetSignal::TerminalInput(b"\x1b[<0;5;2m".to_vec())));
    assert!(term.selection.is_none());
}

#[test]
fn shift_drag_selects_even_while_the_child_tracks_the_pointer() {
    let mut term = pane(10, 3);
    term.feed(b"hello world\x1b[?1000h");
    let area = rect(&term);
    let down = pointer(MouseKind::Down(MouseButton::Left), area.x, area.y, mods::SHIFT);
    assert_eq!(term.on_mouse(area, &down), None);
    let drag = pointer(MouseKind::Drag(MouseButton::Left), area.x + 4, area.y, mods::SHIFT);
    term.on_mouse(area, &drag);
    let up = pointer(MouseKind::Up(MouseButton::Left), area.x + 4, area.y, mods::SHIFT);
    assert_eq!(term.on_mouse(area, &up), Some(WidgetSignal::Copy("hello".into())));
}

#[test]
fn drag_selects_copies_on_release_and_a_bare_click_selects_nothing() {
    let mut term = pane(12, 3);
    term.feed(b"hello world\r\nsecond");
    let area = rect(&term);
    let down = pointer(MouseKind::Down(MouseButton::Left), area.x + 6, area.y, 0);
    assert_eq!(term.on_mouse(area, &down), None);
    let drag = pointer(MouseKind::Drag(MouseButton::Left), area.x + 2, area.y + 1, 0);
    term.on_mouse(area, &drag);
    let up = pointer(MouseKind::Up(MouseButton::Left), area.x + 2, area.y + 1, 0);
    assert_eq!(term.on_mouse(area, &up), Some(WidgetSignal::Copy("world\nsec".into())));
    let click = pointer(MouseKind::Down(MouseButton::Left), area.x + 1, area.y, 0);
    term.on_mouse(area, &click);
    let release = pointer(MouseKind::Up(MouseButton::Left), area.x + 1, area.y, 0);
    assert_eq!(term.on_mouse(area, &release), None);
    assert!(term.selection.is_none());
}

#[test]
fn double_click_selects_a_word_and_triple_click_a_line() {
    let mut term = pane(20, 3);
    term.feed(b"foo bar baz\r\nnext");
    let area = rect(&term);
    let mut press = pointer(MouseKind::Down(MouseButton::Left), area.x + 5, area.y, 0);
    press.clicks = 2;
    term.on_mouse(area, &press);
    let release = pointer(MouseKind::Up(MouseButton::Left), area.x + 5, area.y, 0);
    assert_eq!(term.on_mouse(area, &release), Some(WidgetSignal::Copy("bar".into())));
    assert_eq!(term.selection.unwrap().mode, SelectionMode::Word);
    press.clicks = 3;
    term.on_mouse(area, &press);
    assert_eq!(term.on_mouse(area, &release), Some(WidgetSignal::Copy("foo bar baz".into())));
}

#[test]
fn selection_stays_on_its_text_while_output_scrolls() {
    let mut term = pane(12, 3);
    term.feed(b"keep me\r\nline\r\n");
    let area = rect(&term);
    term.on_mouse(area, &pointer(MouseKind::Down(MouseButton::Left), area.x, area.y, 0));
    term.on_mouse(area, &pointer(MouseKind::Drag(MouseButton::Left), area.x + 6, area.y, 0));
    term.feed(&numbered(2));
    let selection = term.selection.unwrap();
    assert_eq!(term.screen.text_between(selection.start, selection.end), "keep me");
    assert_eq!(term.copy_selection(), Some(WidgetSignal::Copy("keep me".into())));
    assert!(term.selection.is_none());
}

#[test]
fn control_click_opens_links_and_right_click_asks_for_a_menu() {
    let mut term = pane(30, 3);
    term.feed(b"go https://example.com/a now");
    let area = rect(&term);
    let link = pointer(MouseKind::Down(MouseButton::Left), area.x + 8, area.y, mods::CTRL);
    assert_eq!(term.on_mouse(area, &link), Some(WidgetSignal::OpenUrl("https://example.com/a".into())));
    let right = pointer(MouseKind::Down(MouseButton::Right), area.x + 1, area.y + 1, 0);
    assert_eq!(term.on_mouse(area, &right), Some(WidgetSignal::ContextMenu { pos: Pos { x: area.x + 1, y: area.y + 1 }, item: None }));
}

#[test]
fn scrollbar_data_tracks_the_viewport() {
    let mut term = pane(10, 4);
    assert_eq!(term.scrollbar(4), None);
    term.feed(&numbered(36));
    let bottom = term.scrollbar(4).unwrap();
    assert_eq!((bottom.thumb_len, bottom.thumb_start), (1, 3));
    term.screen.scroll_view_to_top();
    let top = term.scrollbar(4).unwrap();
    assert_eq!(top.thumb_start, 0);
    term.screen.feed(b"\x1b[?1049h");
    assert_eq!(term.scrollbar(4), None);
}

#[test]
fn scrollbar_click_jumps_to_the_proportional_position() {
    let mut term = pane(10, 4);
    term.feed(&numbered(36));
    term.screen.scroll_view(1);
    let area = rect(&term);
    let press = pointer(MouseKind::Down(MouseButton::Left), area.x + area.width - 1, area.y, 0);
    term.on_mouse(area, &press);
    assert_eq!(term.screen.view_top(), term.screen.first_row());
    term.on_mouse(area, &pointer(MouseKind::Up(MouseButton::Left), area.x + area.width - 1, area.y, 0));
}

fn painted(term: &TerminalState, theme: &Theme, area: Rect) -> CellBuffer {
    let mut buf = CellBuffer::new(Size { width: area.x + area.width + 2, height: area.y + area.height + 1 }, Cell::blank([9, 9, 9], [8, 8, 8]));
    term.paint(theme, area, &mut buf);
    buf
}

#[test]
fn default_colours_and_palette_come_from_the_theme() {
    let dark = Theme::new(AppearanceName::Dark);
    let light = Theme::new(AppearanceName::Light);
    let mut term = pane(8, 2);
    term.feed(b"plain \x1b[31mred\x1b[0m");
    let area = rect(&term);
    for theme in [&dark, &light] {
        let palette = Palette::from_theme(theme);
        let buf = painted(&term, theme, area);
        let plain = buf.get(area.x, area.y).unwrap();
        assert_eq!((plain.fg, plain.bg), (palette.fg, palette.bg));
        let red = buf.get(area.x + 6, area.y).unwrap();
        assert_eq!(red.fg, palette.ansi[1]);
        assert_eq!(red.bg, palette.bg);
        let outside = buf.get(area.x + area.width, area.y).unwrap();
        assert_eq!(outside.bg, [8, 8, 8]);
    }
    assert_ne!(Palette::from_theme(&dark).bg, Palette::from_theme(&light).bg);
}

#[test]
fn reverse_video_mode_swaps_the_default_colours() {
    let theme = Theme::new(AppearanceName::Dark);
    let palette = Palette::from_theme(&theme);
    let mut term = pane(6, 2);
    term.feed(b"x\x1b[?5h");
    let area = rect(&term);
    let buf = painted(&term, &theme, area);
    let cell = buf.get(area.x, area.y).unwrap();
    assert_eq!((cell.fg, cell.bg), (palette.bg, palette.fg));
}

#[test]
fn pending_wrap_cursor_is_the_last_column_and_survives_a_resize() {
    let mut screen = fed(5, 3, b"abcde");
    assert_eq!(screen.cursor, Pos { x: 4, y: 0 });
    screen.resize(size(8, 3));
    assert_eq!(screen.cursor, Pos { x: 5, y: 0 });
    assert_eq!(row_text(&screen, 0), "abcde");
}

#[test]
fn save_and_restore_cursor_with_csi_s_and_u() {
    let screen = fed(10, 3, b"ab\x1b[s\x1b[3;5Hz\x1b[uc");
    assert_eq!(rows(&screen), vec!["abc", "", "    z"]);
}

#[test]
fn paint_marks_selection_matches_scroll_cue_and_search_row() {
    let theme = Theme::new(AppearanceName::Dark);
    let mut term = pane(14, 4);
    term.feed(&numbered(10));
    term.feed(b"find me");
    let area = rect(&term);
    let palette = Palette::from_theme(&theme);
    term.begin_search();
    term.on_paste("find");
    let buf = painted(&term, &theme, area);
    let last = area.y + area.height - 1;
    let row: String = (0..area.width).filter_map(|x| buf.get(area.x + x, last)).map(|c| c.ch).collect();
    assert!(row.starts_with("/find"), "search row: {row:?}");
    assert!(row.trim_end().ends_with("1/1"), "search count: {row:?}");
    term.end_search();
    term.screen.scroll_view(3);
    let buf = painted(&term, &theme, area);
    let corner: String = (0..area.width).filter_map(|x| buf.get(area.x + x, last)).map(|c| c.ch).collect();
    assert!(corner.contains('↓'), "scrolled cue: {corner:?}");
    let bar = buf.get(area.x + area.width - 1, area.y).unwrap();
    assert!(bar.ch == '┃' || bar.ch == '│');
    term.select_all();
    let buf = painted(&term, &theme, area);
    let cell = buf.get(area.x + 1, area.y + 1).unwrap();
    assert_eq!(cell.fg, palette.bg);
}

#[test]
fn fit_resizes_only_when_the_pane_size_changed() {
    let mut term = pane(10, 4);
    assert!(!term.fit(Rect::new(3, 3, 10, 4)));
    assert!(term.fit(Rect::new(3, 3, 12, 5)));
    assert_eq!(term.screen.size, size(12, 5));
    assert!(term.selection.is_none());
}

#[test]
fn escape_is_wanted_while_something_can_be_undone() {
    let mut term = pane(10, 4);
    assert!(!term.wants_escape());
    term.begin_search();
    assert!(term.wants_escape());
    term.end_search();
    term.set_passthrough(false);
    assert!(term.wants_escape());
}
//#endregion 🖥️Pane
