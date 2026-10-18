// 📜️ Vt history, reflow and selection text laws: the history eviction, the viewport anchoring, the reflow, the selection text and the search.

//#region 📜️History
fn numbered(count: usize) -> Vec<u8> {
    (0..count).flat_map(|n| format!("l{n}\r\n").into_bytes()).collect()
}

#[test]
fn rows_leave_the_screen_into_history_with_stable_ids() {
    let mut screen = fed(8, 3, &numbered(6));
    assert_eq!(history_text(&screen), vec!["l0", "l1", "l2", "l3"]);
    assert_eq!(screen.screen_top(), 4);
    assert_eq!(rows(&screen), vec!["l4", "l5", ""]);
    screen.feed(b"l6\r\n");
    assert_eq!(screen.row_at(0).unwrap().cells[1].ch, '0');
    assert_eq!(screen.screen_top(), 5);
}

#[test]
fn viewport_stays_put_while_output_arrives_when_scrolled_up() {
    let mut screen = fed(8, 3, &numbered(10));
    screen.scroll_view(3);
    let top = screen.view_top();
    let before: Vec<String> = (0..3).map(|y| screen.row_at(top + y).map(|row| row.cells[0..3].iter().map(|c| c.ch).collect::<String>().trim().to_string()).unwrap()).collect();
    screen.feed(&numbered(4));
    assert_eq!(screen.view_top(), top);
    let after: Vec<String> = (0..3).map(|y| screen.row_at(screen.view_top() + y).map(|row| row.cells[0..3].iter().map(|c| c.ch).collect::<String>().trim().to_string()).unwrap()).collect();
    assert_eq!(before, after);
    assert_eq!(screen.view_offset(), 7);
    assert!(!screen.following());
    screen.scroll_view_to_bottom();
    assert!(screen.following());
}

#[test]
fn viewport_follows_output_at_the_bottom_and_clamps_at_the_ends() {
    let mut screen = fed(8, 3, &numbered(10));
    assert!(screen.following());
    screen.scroll_view(1000);
    assert_eq!(screen.view_top(), screen.first_row());
    screen.scroll_view(-1000);
    assert!(screen.following());
}

#[test]
fn eviction_moves_the_anchor_to_the_oldest_row() {
    let mut screen = VtScreen::new(size(8, 2), 3);
    screen.feed(&numbered(6));
    screen.scroll_view(2);
    let first = screen.first_row();
    screen.feed(&numbered(10));
    assert!(screen.first_row() > first);
    assert!(screen.view_top() >= screen.first_row());
}

#[test]
fn erase_display_three_clears_history_but_keeps_ids_monotonic() {
    let mut screen = fed(8, 3, &numbered(8));
    let top = screen.screen_top();
    screen.feed(b"\x1b[3J");
    assert_eq!(screen.scrollback_len(), 0);
    assert_eq!(screen.screen_top(), top);
}

#[test]
fn reflow_joins_soft_wrapped_rows_and_splits_them_again() {
    let mut screen = fed(10, 4, b"abcdefghijklmno");
    assert_eq!(rows(&screen)[..2], ["abcdefghij", "klmno"]);
    screen.resize(size(5, 4));
    assert_eq!(rows(&screen), vec!["abcde", "fghij", "klmno", ""]);
    assert_eq!(screen.cursor, Pos { x: 4, y: 2 });
    screen.resize(size(10, 4));
    assert_eq!(rows(&screen), vec!["abcdefghij", "klmno", "", ""]);
    assert_eq!(screen.cursor, Pos { x: 5, y: 1 });
}

#[test]
fn reflow_keeps_hard_line_breaks() {
    let mut screen = fed(10, 4, b"abc\r\ndef");
    screen.resize(size(30, 4));
    assert_eq!(rows(&screen), vec!["abc", "def", "", ""]);
    screen.resize(size(2, 6));
    assert_eq!(rows(&screen)[..4], ["ab", "c", "de", "f"]);
}

#[test]
fn reflow_pushes_rows_into_history_and_back() {
    let mut screen = fed(10, 3, b"one\r\ntwo\r\nthree\r\nfour");
    assert_eq!(history_text(&screen), vec!["one"]);
    screen.resize(size(10, 2));
    assert_eq!(history_text(&screen), vec!["one", "two"]);
    assert_eq!(rows(&screen), vec!["three", "four"]);
    screen.resize(size(10, 5));
    assert_eq!(history_text(&screen), Vec::<String>::new());
    assert_eq!(rows(&screen)[..4], ["one", "two", "three", "four"]);
    assert_eq!(screen.cursor, Pos { x: 4, y: 3 });
}

#[test]
fn reflow_keeps_wide_characters_whole() {
    let mut screen = fed(6, 3, "ab好好好".as_bytes());
    screen.resize(size(5, 3));
    assert_eq!(rows(&screen)[..2], ["ab好", "好好"]);
    screen.resize(size(3, 4));
    assert_eq!(rows(&screen)[..3], ["ab", "好", "好"]);
}

#[test]
fn reflow_maps_the_scrolled_viewport_to_the_same_text() {
    let mut screen = fed(10, 3, &numbered(20));
    screen.scroll_view(8);
    let top_text = row_text_at(&screen, screen.view_top());
    screen.resize(size(6, 3));
    assert!(!screen.following());
    assert_eq!(row_text_at(&screen, screen.view_top()), top_text);
}

fn row_text_at(screen: &VtScreen, abs: u64) -> String {
    screen.row_at(abs).map(|row| row.cells.iter().map(|cell| cell.ch).collect::<String>().trim_end().to_string()).unwrap_or_default()
}

#[test]
fn alternate_screen_is_cropped_not_reflowed() {
    let mut screen = fed(6, 3, b"\x1b[?1049habcdef\r\nxyz");
    screen.resize(size(4, 2));
    assert_eq!(rows(&screen), vec!["abcd", "xyz"]);
    screen.feed(b"\x1b[?1049l");
    assert!(!screen.alt_active);
}

#[test]
fn text_between_joins_wrapped_rows_and_trims_blank_tails() {
    let screen = fed(5, 4, b"abcdefgh\r\nxy");
    let top = screen.screen_top();
    let text = screen.text_between(CellPoint { row: top, col: 1 }, CellPoint { row: top + 2, col: 4 });
    assert_eq!(text, "bcdefgh\nxy");
    let reversed = screen.text_between(CellPoint { row: top + 2, col: 4 }, CellPoint { row: top, col: 1 });
    assert_eq!(reversed, text);
}

#[test]
fn word_line_and_url_spans() {
    let screen = fed(30, 3, b"see https://a.example/x?y=1, now\r\nsecond line");
    let top = screen.screen_top();
    let at = |col| CellPoint { row: top, col };
    assert_eq!(screen.word_span(at(5)), (at(4), at(26)));
    assert_eq!(screen.url_at(at(10)).as_deref(), Some("https://a.example/x?y=1"));
    assert_eq!(screen.url_at(at(1)), None);
    let wrapped = fed(5, 3, b"abcdefgh");
    let top = wrapped.screen_top();
    assert_eq!(wrapped.line_span(CellPoint { row: top + 1, col: 1 }), (CellPoint { row: top, col: 0 }, CellPoint { row: top + 1, col: 4 }));
}

#[test]
fn find_is_case_insensitive_and_spans_history() {
    let screen = fed(12, 3, b"Alpha\r\nbeta\r\nALPHA beta\r\nalpha");
    let hits = screen.find("alpha");
    assert_eq!(hits.len(), 3);
    assert_eq!(hits[0].row, 0);
    assert!(hits.windows(2).all(|pair| pair[0].row <= pair[1].row));
    assert_eq!(hits[1].cells, 5);
    assert!(screen.find("").is_empty());
}
//#endregion 📜️History
