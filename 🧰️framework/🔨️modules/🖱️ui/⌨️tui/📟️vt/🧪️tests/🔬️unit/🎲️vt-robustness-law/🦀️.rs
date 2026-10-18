// 🎲️ Vt robustness law: arbitrary output, resizes and queries never panic.

//#region 🎲️Robustness
#[test]
fn arbitrary_output_resizes_and_queries_never_panic() {
    let mut seed = 0x2545_f491_4f6c_dd1d_u64;
    let mut next = move || {
        seed ^= seed << 13;
        seed ^= seed >> 7;
        seed ^= seed << 17;
        seed
    };
    let tokens: &[&[u8]] = &[
        b"\x1b[", b"\x1b]", b"\x1bP", b"\x1b_", b"\x1b(", b"\x1b#", b"\x1b", b";", b":", b"?", b">", b"<", b"=", b"0", b"1", b"2", b"3", b"9", b"99999", b"2147483647", b"m", b"H", b"J", b"K", b"r", b"h", b"l", b"@", b"P", b"L", b"M", b"S", b"T", b"X",
        b"b", b"d", b"G", b"I", b"Z", b"s", b"u", b"c", b"n", b"t", b"q", b"p", b"$", b"!", b" ", b"\r\n", b"\n", b"\r", "好".as_bytes(), "é".as_bytes(), "😀".as_bytes(), b"\xff", b"\xe2\x82", b"\x07", b"\x08", b"\t", b"\x0e", b"\x0f", b"\x18", b"\x1a", b"abc",
        b"\x1b\\", b"\x1b7", b"\x1b8", b"\x1bM", b"\x1bD", b"\x1bE", b"\x1bc", b"\x1b[?1049h", b"\x1b[?1049l", b"\x1b[?6h", b"\x1b[?7l", b"\x1b[4h", b"\x1b[2J", b"\x1b[3J",
    ];
    let mut term = TerminalState::new(size(17, 6), 40);
    let theme = Theme::new(AppearanceName::Dark);
    let mut buf = CellBuffer::new(Size { width: 60, height: 30 }, Cell::blank([0, 0, 0], [0, 0, 0]));
    for round in 0..6_000 {
        let mut chunk = Vec::new();
        for _ in 0..(next() % 24) {
            chunk.extend_from_slice(tokens[(next() % tokens.len() as u64) as usize]);
        }
        term.feed(&chunk);
        match next() % 9 {
            0 => term.resize(size(1 + (next() % 40) as u16, 1 + (next() % 20) as u16)),
            1 => term.screen.scroll_view((next() % 50) as i64 - 25),
            2 => {
                term.begin_search();
                term.on_paste("a");
            }
            3 => term.end_search(),
            4 => {
                let area = Rect::new(1, 1, term.screen.size.width.min(50), term.screen.size.height.min(25));
                let kinds = [MouseKind::Down(MouseButton::Left), MouseKind::Drag(MouseButton::Left), MouseKind::Up(MouseButton::Left), MouseKind::Scroll { dx: 0, dy: 1 }, MouseKind::Move];
                let kind = kinds[(next() % 5) as usize];
                let mut event = pointer(kind, (next() % 60) as u16, (next() % 30) as u16, (next() % 8) as u8);
                event.clicks = 1 + (next() % 3) as u8;
                term.on_mouse(area, &event);
            }
            5 => {
                let area = Rect::new(0, 0, term.screen.size.width.min(60), term.screen.size.height.min(30));
                term.paint(&theme, area, &mut buf);
                term.cursor(area);
            }
            6 => {
                term.select_all();
                term.selected_text();
            }
            _ => {}
        }
        assert!(term.screen.cursor.x < term.screen.size.width && term.screen.cursor.y < term.screen.size.height, "round {round}: cursor escaped the screen");
        assert!(term.screen.view_top() >= term.screen.first_row() && term.screen.view_top() <= term.screen.screen_top());
    }
}
//#endregion 🎲️Robustness
