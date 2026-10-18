// ⌨️ Vt key, mouse, paste and focus encoding laws: the key and keypad encoders, the bracketed paste, the focus reports and the mouse encodings with their gating.

//#region ⌨️Keys
#[test]
fn keys_plain_and_application_cursor() {
    let normal = InputModes::default();
    let application = InputModes { app_cursor: true, ..normal };
    let cases: &[(Key, &[u8], &[u8])] = &[
        (Key::Up, b"\x1b[A", b"\x1bOA"),
        (Key::Down, b"\x1b[B", b"\x1bOB"),
        (Key::Right, b"\x1b[C", b"\x1bOC"),
        (Key::Left, b"\x1b[D", b"\x1bOD"),
        (Key::Home, b"\x1b[H", b"\x1bOH"),
        (Key::End, b"\x1b[F", b"\x1bOF"),
        (Key::Insert, b"\x1b[2~", b"\x1b[2~"),
        (Key::Delete, b"\x1b[3~", b"\x1b[3~"),
        (Key::PageUp, b"\x1b[5~", b"\x1b[5~"),
        (Key::PageDown, b"\x1b[6~", b"\x1b[6~"),
    ];
    for (k, plain, app) in cases {
        assert_eq!(bytes(key(*k, 0), &normal), *plain, "{k:?} normal");
        assert_eq!(bytes(key(*k, 0), &application), *app, "{k:?} application");
    }
}

#[test]
fn keys_modified_cursor_and_editing_keys_use_the_modifier_parameter() {
    let modes = InputModes { app_cursor: true, ..InputModes::default() };
    assert_eq!(bytes(key(Key::Up, mods::CTRL), &modes), b"\x1b[1;5A");
    assert_eq!(bytes(key(Key::Left, mods::SHIFT), &modes), b"\x1b[1;2D");
    assert_eq!(bytes(key(Key::Right, mods::ALT), &modes), b"\x1b[1;3C");
    assert_eq!(bytes(key(Key::Down, mods::SHIFT | mods::CTRL | mods::ALT), &modes), b"\x1b[1;8B");
    assert_eq!(bytes(key(Key::Home, mods::CTRL), &modes), b"\x1b[1;5H");
    assert_eq!(bytes(key(Key::End, mods::SHIFT), &modes), b"\x1b[1;2F");
    assert_eq!(bytes(key(Key::Delete, mods::CTRL), &modes), b"\x1b[3;5~");
    assert_eq!(bytes(key(Key::PageUp, mods::SHIFT), &modes), b"\x1b[5;2~");
    assert_eq!(bytes(key(Key::Insert, mods::ALT), &modes), b"\x1b[2;3~");
}

#[test]
fn keys_function_keys_f1_to_f12_and_modified() {
    let modes = InputModes::default();
    let plain: [&[u8]; 12] = [b"\x1bOP", b"\x1bOQ", b"\x1bOR", b"\x1bOS", b"\x1b[15~", b"\x1b[17~", b"\x1b[18~", b"\x1b[19~", b"\x1b[20~", b"\x1b[21~", b"\x1b[23~", b"\x1b[24~"];
    for (index, expected) in plain.iter().enumerate() {
        assert_eq!(bytes(key(Key::F(index as u8 + 1), 0), &modes), *expected, "F{}", index + 1);
    }
    assert_eq!(bytes(key(Key::F(1), mods::SHIFT), &modes), b"\x1b[1;2P");
    assert_eq!(bytes(key(Key::F(4), mods::CTRL), &modes), b"\x1b[1;5S");
    assert_eq!(bytes(key(Key::F(5), mods::CTRL), &modes), b"\x1b[15;5~");
    assert_eq!(bytes(key(Key::F(12), mods::SHIFT | mods::ALT), &modes), b"\x1b[24;4~");
    assert_eq!(bytes(key(Key::F(13), 0), &modes), b"\x1b[1;2P");
    assert_eq!(bytes(key(Key::F(25), 0), &modes), b"\x1b[1;5P");
    assert_eq!(bytes(key(Key::F(40), 0), &modes), b"\x1b[1;6S");
    assert_eq!(bytes(key(Key::F(17), mods::ALT), &modes), b"\x1b[15;4~");
    assert_eq!(encode_key(&key(Key::F(0), 0), &modes), None);
    assert_eq!(encode_key(&key(Key::F(64), 0), &modes), None);
}

#[test]
fn keys_alt_prefixes_escape_and_control_maps_to_c0() {
    let modes = InputModes::default();
    assert_eq!(bytes(key(Key::Char('x'), mods::ALT), &modes), b"\x1bx");
    assert_eq!(bytes(key(Key::Char('é'), mods::ALT), &modes), "\u{1b}é".as_bytes());
    assert_eq!(bytes(key(Key::Char('a'), mods::CTRL), &modes), b"\x01");
    assert_eq!(bytes(key(Key::Char('Z'), mods::CTRL), &modes), b"\x1a");
    assert_eq!(bytes(key(Key::Char('a'), mods::CTRL | mods::ALT), &modes), b"\x1b\x01");
    assert_eq!(bytes(key(Key::Char(' '), mods::CTRL), &modes), b"\x00");
    assert_eq!(bytes(key(Key::Char('['), mods::CTRL), &modes), b"\x1b");
    assert_eq!(bytes(key(Key::Char('\\'), mods::CTRL), &modes), b"\x1c");
    assert_eq!(bytes(key(Key::Char(']'), mods::CTRL), &modes), b"\x1d");
    assert_eq!(bytes(key(Key::Char('/'), mods::CTRL), &modes), b"\x1f");
    assert_eq!(bytes(key(Key::Char('?'), mods::CTRL), &modes), b"\x7f");
    assert_eq!(bytes(key(Key::Char('€'), 0), &modes), "€".as_bytes());
    assert_eq!(bytes(key(Key::Char('A'), mods::SHIFT), &modes), b"A");
}

#[test]
fn keys_enter_tab_backspace_escape() {
    let modes = InputModes::default();
    assert_eq!(bytes(key(Key::Enter, 0), &modes), b"\r");
    assert_eq!(bytes(key(Key::Enter, 0), &InputModes { newline: true, ..modes }), b"\r\n");
    assert_eq!(bytes(key(Key::Enter, mods::ALT), &modes), b"\x1b\r");
    assert_eq!(bytes(key(Key::Tab, 0), &modes), b"\t");
    assert_eq!(bytes(key(Key::Tab, mods::SHIFT), &modes), b"\x1b[Z");
    assert_eq!(bytes(key(Key::BackTab, 0), &modes), b"\x1b[Z");
    assert_eq!(bytes(key(Key::Backspace, 0), &modes), b"\x7f");
    assert_eq!(bytes(key(Key::Backspace, mods::CTRL), &modes), b"\x08");
    assert_eq!(bytes(key(Key::Backspace, mods::ALT), &modes), b"\x1b\x7f");
    assert_eq!(bytes(key(Key::Esc, 0), &modes), b"\x1b");
    assert_eq!(bytes(key(Key::Esc, mods::ALT), &modes), b"\x1b\x1b");
}

#[test]
fn keypad_follows_application_keypad_mode() {
    let normal = InputModes::default();
    let application = InputModes { app_keypad: true, ..normal };
    assert_eq!(encode_keypad(KeypadKey::Digit(5), 0, &normal), b"5");
    assert_eq!(encode_keypad(KeypadKey::Digit(5), 0, &application), b"\x1bOu");
    assert_eq!(encode_keypad(KeypadKey::Enter, 0, &application), b"\x1bOM");
    assert_eq!(encode_keypad(KeypadKey::Plus, 0, &application), b"\x1bOk");
    assert_eq!(encode_keypad(KeypadKey::Decimal, 0, &normal), b".");
    assert_eq!(encode_keypad(KeypadKey::Enter, mods::ALT, &normal), b"\x1b\r");
}
//#endregion ⌨️Keys

//#region 📋️Paste And Focus
#[test]
fn paste_is_bracketed_exactly_once_and_cannot_close_the_bracket_early() {
    let bracketed = InputModes { bracketed_paste: true, ..InputModes::default() };
    assert_eq!(encode_paste("a\nb", &bracketed), b"\x1b[200~a\nb\x1b[201~");
    let hostile = encode_paste("x\x1b[201~rm -rf /", &bracketed);
    let text = String::from_utf8(hostile).unwrap();
    assert_eq!(text.matches("\u{1b}[201~").count(), 1);
    assert_eq!(text.matches("\u{1b}[200~").count(), 1);
    assert!(text.ends_with("\u{1b}[201~"));
    assert_eq!(encode_paste("a\r\nb\rc", &bracketed), b"\x1b[200~a\nb\nc\x1b[201~");
}

#[test]
fn paste_without_bracketing_sends_carriage_returns() {
    let modes = InputModes::default();
    assert_eq!(encode_paste("one\ntwo\r\nthree", &modes), b"one\rtwo\rthree");
    assert_eq!(encode_paste("tab\there", &modes), b"tab\there");
    assert_eq!(encode_paste("é€😀", &modes), "é€😀".as_bytes());
}

#[test]
fn focus_reports_only_when_the_child_asked() {
    let off = InputModes::default();
    let on = InputModes { focus_reporting: true, ..off };
    assert_eq!(encode_focus(true, &off), None);
    assert_eq!(encode_focus(true, &on).as_deref(), Some(&b"\x1b[I"[..]));
    assert_eq!(encode_focus(false, &on).as_deref(), Some(&b"\x1b[O"[..]));
}
//#endregion 📋️Paste And Focus

//#region 🐁️Mouse
#[test]
fn mouse_sgr_press_release_drag_move_and_wheel() {
    let modes = modes_with(MouseReporting::Motion, MouseEncoding::Sgr);
    let at = Pos { x: 4, y: 2 };
    let report = |kind, modifiers| encode_mouse(&pointer(kind, 0, 0, modifiers), at, &modes);
    assert_eq!(report(MouseKind::Down(MouseButton::Left), 0), b"\x1b[<0;5;3M");
    assert_eq!(report(MouseKind::Up(MouseButton::Left), 0), b"\x1b[<0;5;3m");
    assert_eq!(report(MouseKind::Down(MouseButton::Right), 0), b"\x1b[<2;5;3M");
    assert_eq!(report(MouseKind::Down(MouseButton::Middle), mods::CTRL), b"\x1b[<17;5;3M");
    assert_eq!(report(MouseKind::Drag(MouseButton::Left), 0), b"\x1b[<32;5;3M");
    assert_eq!(report(MouseKind::Move, 0), b"\x1b[<35;5;3M");
    assert_eq!(report(MouseKind::Scroll { dx: 0, dy: -1 }, 0), b"\x1b[<64;5;3M");
    assert_eq!(report(MouseKind::Scroll { dx: 0, dy: 2 }, mods::SHIFT), b"\x1b[<69;5;3M\x1b[<69;5;3M");
    assert_eq!(report(MouseKind::Scroll { dx: 1, dy: 0 }, 0), b"\x1b[<67;5;3M");
}

#[test]
fn mouse_modes_gate_which_events_are_reported() {
    let at = Pos { x: 0, y: 0 };
    let drag = pointer(MouseKind::Drag(MouseButton::Left), 0, 0, 0);
    let motion = pointer(MouseKind::Move, 0, 0, 0);
    let down = pointer(MouseKind::Down(MouseButton::Left), 0, 0, 0);
    let up = pointer(MouseKind::Up(MouseButton::Left), 0, 0, 0);
    let wheel = pointer(MouseKind::Scroll { dx: 0, dy: 1 }, 0, 0, 0);
    let off = modes_with(MouseReporting::Off, MouseEncoding::Sgr);
    let click = modes_with(MouseReporting::Click, MouseEncoding::Sgr);
    let press = modes_with(MouseReporting::Press, MouseEncoding::Sgr);
    let dragging = modes_with(MouseReporting::Drag, MouseEncoding::Sgr);
    assert!(encode_mouse(&down, at, &off).is_empty());
    assert!(!encode_mouse(&down, at, &click).is_empty());
    assert!(encode_mouse(&up, at, &click).is_empty());
    assert!(encode_mouse(&wheel, at, &click).is_empty());
    assert!(!encode_mouse(&up, at, &press).is_empty());
    assert!(encode_mouse(&drag, at, &press).is_empty());
    assert!(!encode_mouse(&drag, at, &dragging).is_empty());
    assert!(encode_mouse(&motion, at, &dragging).is_empty());
    let click_with_modifier = pointer(MouseKind::Down(MouseButton::Left), 0, 0, mods::CTRL);
    assert_eq!(encode_mouse(&click_with_modifier, at, &click), b"\x1b[<0;1;1M");
}

#[test]
fn mouse_legacy_encodings() {
    let down = pointer(MouseKind::Down(MouseButton::Left), 0, 0, 0);
    let up = pointer(MouseKind::Up(MouseButton::Left), 0, 0, 0);
    let x10 = modes_with(MouseReporting::Press, MouseEncoding::X10);
    assert_eq!(encode_mouse(&down, Pos { x: 4, y: 2 }, &x10), vec![0x1b, b'[', b'M', 32, 32 + 5, 32 + 3]);
    assert_eq!(encode_mouse(&up, Pos { x: 4, y: 2 }, &x10), vec![0x1b, b'[', b'M', 35, 37, 35]);
    assert!(encode_mouse(&down, Pos { x: 223, y: 0 }, &x10).is_empty());
    assert!(!encode_mouse(&down, Pos { x: 222, y: 0 }, &x10).is_empty());
    let utf8 = modes_with(MouseReporting::Press, MouseEncoding::Utf8);
    let wide = encode_mouse(&down, Pos { x: 299, y: 0 }, &utf8);
    assert_eq!(&wide[..4], b"\x1b[M ");
    assert_eq!(String::from_utf8(wide[4..].to_vec()).unwrap(), format!("{}{}", char::from_u32(300 + 32).unwrap(), '!'));
    let urxvt = modes_with(MouseReporting::Press, MouseEncoding::Urxvt);
    assert_eq!(encode_mouse(&down, Pos { x: 4, y: 2 }, &urxvt), b"\x1b[32;5;3M");
}

#[test]
fn wheel_on_the_alternate_screen_becomes_arrow_keys() {
    let plain = InputModes::default();
    assert_eq!(encode_wheel_as_arrows(-1, &plain), b"\x1b[A\x1b[A\x1b[A");
    let application = InputModes { app_cursor: true, ..plain };
    assert_eq!(encode_wheel_as_arrows(1, &application), b"\x1bOB\x1bOB\x1bOB");
}
//#endregion 🐁️Mouse
