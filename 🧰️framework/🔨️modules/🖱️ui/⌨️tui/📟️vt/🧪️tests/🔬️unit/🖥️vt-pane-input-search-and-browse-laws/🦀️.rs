// 🖥️ Vt pane input, search and browse laws: the pane helpers, the key passthrough, the viewport snap, the search, the browse mode, the paste and the focus events.

//#region 🖥️Pane
fn pane(width: u16, height: u16) -> TerminalState {
    TerminalState::new(size(width, height), 100)
}

fn input(term: &mut TerminalState, event: KeyEvent) -> Option<Vec<u8>> {
    match term.on_key(&event) {
        Some(WidgetSignal::TerminalInput(bytes)) => Some(bytes),
        _ => None,
    }
}

fn rect(term: &TerminalState) -> Rect {
    Rect::new(2, 1, term.screen.size.width, term.screen.size.height)
}

#[test]
fn passthrough_sends_every_key_to_the_child() {
    let mut term = pane(20, 5);
    for k in [Key::PageUp, Key::PageDown, Key::Home, Key::End, Key::Char('/'), Key::Char('p'), Key::Esc, Key::Up] {
        assert!(input(&mut term, key(k, 0)).is_some(), "{k:?} reaches the child");
    }
    assert_eq!(input(&mut term, key(Key::Char('p'), mods::CTRL)), Some(vec![0x10]));
    assert!(term.search().is_none());
}

#[test]
fn keys_follow_the_childs_application_cursor_mode() {
    let mut term = pane(20, 5);
    assert_eq!(input(&mut term, key(Key::Up, 0)), Some(b"\x1b[A".to_vec()));
    term.feed(b"\x1b[?1h");
    assert_eq!(input(&mut term, key(Key::Up, 0)), Some(b"\x1bOA".to_vec()));
}

#[test]
fn typing_snaps_the_viewport_back_to_the_bottom() {
    let mut term = pane(8, 3);
    term.feed(&numbered(10));
    term.screen.scroll_view(4);
    assert!(!term.screen.following());
    input(&mut term, key(Key::Char('x'), 0));
    assert!(term.screen.following());
}

#[test]
fn search_is_opened_only_by_command_and_escape_always_leaves() {
    let mut term = pane(20, 4);
    term.feed(b"alpha\r\nbeta\r\nalpha two\r\n");
    assert_eq!(input(&mut term, key(Key::Char('/'), 0)), Some(b"/".to_vec()));
    term.begin_search();
    assert_eq!(term.on_key(&key(Key::Char('a'), 0)), Some(WidgetSignal::ValueChanged("a".into())));
    assert_eq!(term.on_key(&key(Key::Char('l'), 0)), Some(WidgetSignal::ValueChanged("al".into())));
    assert_eq!(term.search().unwrap().matches.len(), 2);
    assert_eq!(term.search().unwrap().current, 1);
    assert_eq!(term.on_key(&key(Key::Enter, 0)), Some(WidgetSignal::SelectionChanged(0)));
    assert_eq!(term.on_key(&key(Key::Enter, mods::SHIFT)), Some(WidgetSignal::SelectionChanged(1)));
    assert_eq!(term.on_key(&key(Key::Backspace, 0)), Some(WidgetSignal::ValueChanged("a".into())));
    assert_eq!(term.on_key(&key(Key::Esc, 0)), Some(WidgetSignal::ValueChanged(String::new())));
    assert!(term.search().is_none());
    assert_eq!(input(&mut term, key(Key::Esc, 0)), Some(vec![0x1b]));
}

#[test]
fn search_reveals_matches_in_history_and_survives_new_output() {
    let mut term = pane(12, 3);
    term.feed(b"needle\r\n");
    term.feed(&numbered(10));
    term.begin_search();
    term.on_paste("needle");
    let hit = term.search().unwrap().matches[0];
    assert!(term.screen.view_top() <= hit.row && hit.row < term.screen.view_top() + 3);
    let top = term.screen.view_top();
    term.feed(&numbered(3));
    assert_eq!(term.screen.view_top(), top);
    assert_eq!(term.search().unwrap().matches.len(), 1);
}

#[test]
fn browse_mode_scrolls_searches_copies_and_returns() {
    let mut term = pane(12, 3);
    term.feed(&numbered(12));
    term.set_passthrough(false);
    assert_eq!(term.on_key(&key(Key::Char('x'), 0)), None);
    assert!(term.screen.following());
    term.on_key(&key(Key::PageUp, 0));
    assert_eq!(term.screen.view_offset(), 3);
    term.on_key(&key(Key::Char('k'), 0));
    assert_eq!(term.screen.view_offset(), 4);
    term.on_key(&key(Key::Home, 0));
    assert_eq!(term.screen.view_top(), term.screen.first_row());
    term.on_key(&key(Key::End, 0));
    assert!(term.screen.following());
    assert_eq!(term.on_key(&key(Key::Esc, 0)), Some(WidgetSignal::Toggled(true)));
    assert!(term.passthrough);
}

#[test]
fn paste_reaches_the_child_bracketed_once_and_snaps_to_the_bottom() {
    let mut term = pane(12, 3);
    term.feed(&numbered(10));
    term.screen.scroll_view(3);
    assert_eq!(term.on_paste("a\nb"), Some(WidgetSignal::TerminalInput(b"a\rb".to_vec())));
    assert!(term.screen.following());
    term.feed(b"\x1b[?2004h");
    assert_eq!(term.on_paste("a\nb"), Some(WidgetSignal::TerminalInput(b"\x1b[200~a\nb\x1b[201~".to_vec())));
    term.set_passthrough(false);
    assert_eq!(term.on_paste("ignored"), None);
}

#[test]
fn focus_events_follow_the_childs_request() {
    let mut term = pane(12, 3);
    assert_eq!(term.on_focus(true), None);
    term.feed(b"\x1b[?1004h");
    assert_eq!(term.on_focus(true), Some(WidgetSignal::TerminalInput(b"\x1b[I".to_vec())));
    assert_eq!(term.on_focus(false), Some(WidgetSignal::TerminalInput(b"\x1b[O".to_vec())));
}

#[test]
fn cursor_follows_the_child_and_hides_when_scrolled_away() {
    let mut term = pane(10, 4);
    term.feed(b"ab\r\ncd");
    let area = rect(&term);
    let spec = term.cursor(area).expect("cursor");
    assert_eq!((spec.pos, spec.shape, spec.blink), (Pos { x: 4, y: 2 }, CursorShape::Block, true));
    term.feed(b"\x1b[5 q");
    assert_eq!(term.cursor(area).unwrap().shape, CursorShape::Bar);
    term.feed(b"\x1b[?25l");
    assert_eq!(term.cursor(area), None);
    term.feed(b"\x1b[?25h");
    term.feed(&numbered(10));
    term.screen.scroll_view(6);
    assert_eq!(term.cursor(area), None);
    term.begin_search();
    let spec = term.cursor(area).unwrap();
    assert_eq!((spec.pos.y, spec.shape), (area.y + area.height - 1, CursorShape::Bar));
}
//#endregion 🖥️Pane
