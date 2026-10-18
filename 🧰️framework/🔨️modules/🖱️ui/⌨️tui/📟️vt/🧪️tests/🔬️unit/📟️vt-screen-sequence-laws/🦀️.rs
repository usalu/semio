// 📟️ Vt screen sequence laws: the child mode tracking, the query answers, the osc handling and the cursor, erase, scroll, sgr, utf8, alternate screen and reset sequences.

//#region 📟️Screen
#[test]
fn child_modes_are_tracked_from_decset() {
    let mut screen = VtScreen::new(size(10, 3), 10);
    screen.feed(b"\x1b[?1h\x1b[?2004h\x1b[?1004h\x1b[?1002h\x1b[?1006h\x1b=");
    let modes = screen.input_modes();
    assert!(modes.app_cursor && modes.bracketed_paste && modes.focus_reporting && modes.app_keypad);
    assert_eq!(modes.mouse, MouseReporting::Drag);
    assert_eq!(modes.mouse_encoding, MouseEncoding::Sgr);
    screen.feed(b"\x1b[?1l\x1b[?2004l\x1b[?1004l\x1b[?1002l\x1b[?1006l\x1b>");
    let modes = screen.input_modes();
    assert!(!modes.app_cursor && !modes.bracketed_paste && !modes.focus_reporting && !modes.app_keypad);
    assert_eq!(modes.mouse, MouseReporting::Off);
    assert_eq!(modes.mouse_encoding, MouseEncoding::X10);
    screen.feed(b"\x1b[?1049h");
    assert!(screen.input_modes().alt_screen);
}

#[test]
fn queries_are_answered_in_order() {
    let mut screen = VtScreen::new(size(20, 5), 10);
    screen.feed(b"\x1b[3;7H\x1b[c\x1b[5n\x1b[6n\x1b[>c\x1b[18t");
    assert_eq!(screen.take_replies(), b"\x1b[?62;22c\x1b[0n\x1b[3;7R\x1b[>0;0;0c\x1b[8;5;20t");
    assert!(screen.take_replies().is_empty());
}

#[test]
fn mode_reports_answer_decrqm() {
    let mut screen = VtScreen::new(size(20, 5), 10);
    screen.feed(b"\x1b[?2004h\x1b[?2004$p\x1b[?1$p\x1b[?9999$p");
    assert_eq!(screen.take_replies(), b"\x1b[?2004;1$y\x1b[?1;2$y\x1b[?9999;0$y");
}

#[test]
fn cursor_position_report_is_relative_in_origin_mode() {
    let mut screen = VtScreen::new(size(20, 10), 10);
    screen.feed(b"\x1b[3;6r\x1b[?6h\x1b[2;4H\x1b[6n");
    assert_eq!(screen.take_replies(), b"\x1b[2;4R");
    assert_eq!(screen.cursor, Pos { x: 3, y: 3 });
}

#[test]
fn osc_sets_title_cwd_and_answers_colour_queries() {
    let mut screen = VtScreen::new(size(20, 5), 10);
    screen.feed(b"\x1b]0;hello\x07\x1b]2;two\x1b\\\x1b]7;file:///tmp\x07");
    assert_eq!(screen.title.as_deref(), Some("two"));
    assert_eq!(screen.cwd.as_deref(), Some("file:///tmp"));
    screen.feed(b"\x1b]11;?\x07");
    assert_eq!(screen.take_replies(), b"\x1b]11;rgb:0000/0000/0000\x1b\\");
}

#[test]
fn osc_52_hands_the_clipboard_text_to_the_host() {
    let mut term = TerminalState::new(size(20, 5), 10);
    let signals = term.feed(b"\x1b]52;c;aGVsbG8gd29ybGQ=\x07");
    assert_eq!(signals, vec![WidgetSignal::Copy("hello world".into())]);
    assert!(term.feed(b"\x1b]52;c;?\x07").is_empty());
}

#[test]
fn replay_never_answers_old_queries() {
    let mut term = TerminalState::new(size(20, 5), 10);
    term.feed_replay(b"\x1b[c\x1b[6n\x1b]52;c;aGk=\x07");
    assert!(term.feed(b"x").is_empty());
    assert_eq!(term.feed(b"\x1b[6n"), vec![WidgetSignal::TerminalInput(b"\x1b[1;2R".to_vec())]);
}

#[test]
fn sequences_cursor_addressing_family() {
    let screen = fed(10, 5, b"\x1b[3;4H*\x1b[2G+\x1b[5d-\x1b[2E>\x1b[1F<");
    assert_eq!(rows(&screen), vec!["", "", " + *", "<", "> -"]);
}

#[test]
fn charset_designators_do_not_leak_and_line_drawing_maps() {
    let screen = fed(12, 2, b"\x1b(Bplain\x1b(0lqk\x1b(B!");
    assert_eq!(row_text(&screen, 0), "plain┌─┐!");
    let shifted = fed(12, 2, b"\x1b)0\x0elqk\x0fabc");
    assert_eq!(row_text(&shifted, 0), "┌─┐abc");
}

#[test]
fn index_next_line_reverse_index_and_tab_stops() {
    let screen = fed(20, 4, b"A\x1bDB\x1bEC\x1bMD");
    assert_eq!(rows(&screen), vec!["A", " D", "C", ""]);
    let tabs = fed(30, 1, b"a\tb\x1bH\x1b[3gc\x1b[1G\x1bH\tz");
    let text = row_text(&tabs, 0);
    assert!(text.starts_with("a       bc"), "{text:?}");
    assert!(text.ends_with('z') && text.chars().count() == 30, "{text:?}");
}

#[test]
fn repeat_erase_insert_delete_cells() {
    let screen = fed(10, 3, b"ab\x1b[3bX\r\n12345\x1b[2D\x1b[2@\r\nabcdef\x1b[3G\x1b[2P");
    assert_eq!(row_text(&screen, 0), "abbbbX");
    assert_eq!(row_text(&screen, 1), "123  45");
    assert_eq!(row_text(&screen, 2), "abef");
    let erased = fed(10, 1, b"abcdef\x1b[3G\x1b[2X");
    assert_eq!(row_text(&erased, 0), "ab  ef");
}

#[test]
fn insert_mode_shifts_text_right() {
    let screen = fed(10, 1, b"abc\x1b[1G\x1b[4hXY\x1b[4l");
    assert_eq!(row_text(&screen, 0), "XYabc");
}

#[test]
fn scroll_regions_scroll_and_reverse_scroll() {
    let screen = fed(5, 5, b"\x1b[1;1HAAAAA\x1b[2;1HBBBBB\x1b[3;1HCCCCC\x1b[4;1HDDDDD\x1b[5;1HEEEEE\x1b[2;4r\x1b[2S");
    assert_eq!(rows(&screen), vec!["AAAAA", "DDDDD", "", "", "EEEEE"]);
    let reverse = fed(5, 5, b"\x1b[1;1HAAAAA\x1b[2;1HBBBBB\x1b[3;1HCCCCC\x1b[4;1HDDDDD\x1b[5;1HEEEEE\x1b[2;4r\x1b[1T");
    assert_eq!(rows(&reverse), vec!["AAAAA", "", "BBBBB", "CCCCC", "EEEEE"]);
    let ri = fed(5, 5, b"\x1b[1;1HAAAAA\x1b[2;1HBBBBB\x1b[2;4r\x1b[2;1H\x1bM");
    assert_eq!(rows(&ri), vec!["AAAAA", "", "BBBBB", "", ""]);
}

#[test]
fn erase_uses_the_current_background_colour() {
    let screen = fed(6, 2, b"\x1b[44mab\x1b[K");
    let cell = screen.cell_at(5, 0).copied().unwrap();
    assert_eq!(cell.bg, [0, 0, 238]);
    assert_eq!(cell.ch, ' ');
    let reset = fed(6, 2, b"\x1b[44mab\x1b[0m\x1b[2;1H\x1b[K");
    assert_eq!(reset.cell_at(5, 1).unwrap().bg, [0, 0, 0]);
}

#[test]
fn sgr_forms_colon_semicolon_palette_and_attributes() {
    let screen = fed(10, 1, b"\x1b[38:2::10:20:30mA\x1b[38:2:1:2:3mB\x1b[38;5;196mC\x1b[48:5:21mD\x1b[1;3;4;7mE\x1b[22;23;24;27mF\x1b[91mG\x1b[39;49mH");
    let at = |x| *screen.cell_at(x, 0).unwrap();
    assert_eq!(at(0).fg, [10, 20, 30]);
    assert_eq!(at(1).fg, [1, 2, 3]);
    assert_eq!(at(2).fg, [255, 0, 0]);
    assert_eq!(at(3).bg, [0, 0, 255]);
    assert_eq!(at(4).attrs & (attr::BOLD | attr::ITALIC | attr::UNDERLINE | attr::REVERSE), attr::BOLD | attr::ITALIC | attr::UNDERLINE | attr::REVERSE);
    assert_eq!(at(5).attrs & 0x1f, 0);
    assert_eq!(at(6).fg, [255, 0, 0]);
    assert_eq!(at(7).fg, [192, 192, 192]);
    let under = fed(4, 1, b"\x1b[4:3mA\x1b[4:0mB");
    assert_eq!(under.cell_at(0, 0).unwrap().attrs & attr::UNDERLINE, attr::UNDERLINE);
    assert_eq!(under.cell_at(1, 0).unwrap().attrs & attr::UNDERLINE, 0);
}

#[test]
fn utf8_wide_and_broken_sequences() {
    let screen = fed(8, 2, "a好b".as_bytes());
    assert_eq!(row_text(&screen, 0), "a好b");
    assert_eq!(screen.cursor, Pos { x: 4, y: 0 });
    let split = {
        let mut screen = VtScreen::new(size(8, 1), 10);
        screen.feed(&"€".as_bytes()[..2]);
        screen.feed(&"€".as_bytes()[2..]);
        screen
    };
    assert_eq!(row_text(&split, 0), "€");
    let broken = fed(8, 1, b"a\xe2\x82b\xffc");
    assert_eq!(row_text(&broken, 0), "a\u{fffd}b\u{fffd}c");
}

#[test]
fn wide_character_wraps_whole_and_overwriting_half_blanks_the_pair() {
    let screen = fed(5, 2, "abcd好".as_bytes());
    assert_eq!(row_text(&screen, 0), "abcd");
    assert_eq!(row_text(&screen, 1), "好");
    let overwrite = fed(6, 1, "好b\x1b[1GX".as_bytes());
    assert_eq!(row_text(&overwrite, 0), "X b");
}

#[test]
fn alternate_screen_variants_and_cursor_restore() {
    let screen = fed(6, 2, b"main\x1b[?1049hALT\x1b[?1049l");
    assert_eq!(row_text(&screen, 0), "main");
    assert!(!screen.alt_active);
    assert_eq!(screen.cursor, Pos { x: 4, y: 0 });
    let plain = fed(6, 2, b"main\x1b[?47hALT");
    assert!(plain.alt_active);
    assert_eq!(row_text(&plain, 0), "    AL");
}

#[test]
fn cursor_style_and_visibility_are_reported() {
    let screen = fed(6, 2, b"\x1b[6 q\x1b[?25l");
    assert_eq!(screen.modes.cursor_shape, CursorShape::Bar);
    assert!(!screen.modes.cursor_blink);
    assert!(!screen.modes.cursor_visible);
    let blink = fed(6, 2, b"\x1b[3 q");
    assert_eq!((blink.modes.cursor_shape, blink.modes.cursor_blink), (CursorShape::Underline, true));
}

#[test]
fn soft_and_hard_reset() {
    let mut screen = fed(6, 2, b"\x1b[?1h\x1b[?7l\x1b[5;1r\x1b[!p");
    assert!(!screen.input.app_cursor && screen.modes.wrap);
    screen.feed(b"keep\r\n\r\nmore\x1bc");
    assert_eq!(rows(&screen), vec!["", ""]);
}

#[test]
fn control_strings_are_swallowed_and_aborted_cleanly() {
    let screen = fed(10, 1, b"a\x1bP1$rpayload\x1b\\b\x1b_apc\x07c\x1b[1;2:?xd\x1b[31\x18e");
    assert_eq!(row_text(&screen, 0), "abcde");
}

#[test]
fn bell_is_counted() {
    let screen = fed(5, 1, b"\x07\x07");
    assert_eq!(screen.bells, 2);
}
//#endregion 📟️Screen
