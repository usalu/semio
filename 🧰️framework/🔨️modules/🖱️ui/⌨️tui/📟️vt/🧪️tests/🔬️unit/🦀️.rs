use super::encode::{encode_focus, encode_key, encode_keypad, encode_mouse, encode_paste, encode_wheel_as_arrows, InputModes, KeypadKey, MouseEncoding, MouseReporting};
use super::pane::{SelectionMode, TerminalState};
use super::{CellPoint, Palette, VtScreen};
use crate::tui::cell::{attr, Cell, CellBuffer};
use crate::tui::event::{mods, Key, KeyEvent, MouseButton, MouseEvent, MouseKind};
use crate::tui::geometry::{Pos, Rect, Size};
use crate::tui::theme::Theme;
use crate::tui::widget::{CursorShape, WidgetSignal};
use ui_styling::appearance::AppearanceName;

fn size(width: u16, height: u16) -> Size {
    Size { width, height }
}

fn key(key: Key, modifiers: u8) -> KeyEvent {
    KeyEvent { key, mods: modifiers }
}

fn row_text(screen: &VtScreen, y: u16) -> String {
    (0..screen.size.width).filter_map(|x| screen.cell_at(x, y)).filter(|cell| cell.width != 0).map(|cell| cell.ch).collect::<String>().trim_end().to_string()
}

fn rows(screen: &VtScreen) -> Vec<String> {
    (0..screen.size.height).map(|y| row_text(screen, y)).collect()
}

fn history_text(screen: &VtScreen) -> Vec<String> {
    (screen.first_row()..screen.screen_top()).filter_map(|abs| screen.row_at(abs)).map(|row| row.cells.iter().filter(|cell| cell.width != 0).map(|cell| cell.ch).collect::<String>().trim_end().to_string()).collect()
}

fn fed(width: u16, height: u16, bytes: &[u8]) -> VtScreen {
    let mut screen = VtScreen::new(size(width, height), 100);
    screen.feed(bytes);
    screen
}

fn bytes(event: KeyEvent, modes: &InputModes) -> Vec<u8> {
    encode_key(&event, modes).expect("key has a spelling")
}

fn pointer(kind: MouseKind, x: u16, y: u16, modifiers: u8) -> MouseEvent {
    MouseEvent { kind, pos: Pos { x, y }, mods: modifiers, clicks: 1 }
}

fn modes_with(mouse: MouseReporting, encoding: MouseEncoding) -> InputModes {
    InputModes { mouse, mouse_encoding: encoding, ..InputModes::default() }
}

//#region 🧫️Shared Fixtures
use semio_framework_pack_json::{parse, JsonMemberPolicy, Value};

fn corpus(text: &str) -> Value {
    parse(text, JsonMemberPolicy::Reject).expect("fixture parses")
}

fn strings(value: &Value) -> Vec<&str> {
    value.as_array().expect("array").iter().map(|item| item.as_str().expect("string")).collect()
}

fn fixture_modifiers(names: &Value) -> u8 {
    strings(names).iter().fold(0, |bits, name| {
        bits | match *name {
            "shift" => mods::SHIFT,
            "alt" => mods::ALT,
            "ctrl" => mods::CTRL,
            other => panic!("unknown modifier {other}"),
        }
    })
}

/// 🥒️ Whether the first tag of `feature` names `capability`; the tests bind their feature file with `include_str!`, so renaming it breaks the build.
fn names_capability(feature: &str, capability: &str) -> bool {
    feature.lines().next().is_some_and(|line| line == format!("@capability-{capability}"))
}

#[test]
fn shared_streams_reproduce_every_screen_pyte_shows() {
    assert!(names_capability(include_str!("../../../../🧪️tests/⌨️tui-terminal-streams/🥒️.feature"), "tui-terminal-screen"));
    let corpus = corpus(include_str!("../../../../🧫️fixtures/⌨️tui-terminal-streams/🔣️.json"));
    let cases = corpus["cases"].as_array().expect("cases");
    assert!(cases.len() >= 30);
    for case in cases {
        let id = case["id"].as_str().expect("id");
        let mut screen = VtScreen::new(size(case["cols"].as_u64().expect("cols") as u16, case["rows"].as_u64().expect("rows") as u16), 100);
        screen.feed(case["stream"].as_str().expect("stream").as_bytes());
        assert_eq!(rows(&screen), strings(&case["screen"]), "{id}: screen");
        if let Some(cursor) = case["cursor"].as_array() {
            assert_eq!(screen.cursor, Pos { x: cursor[0].as_u64().expect("x") as u16, y: cursor[1].as_u64().expect("y") as u16 }, "{id}: cursor");
        }
    }
    assert!(!cases.is_empty(), "the shared terminal stream fixture lists cases");
}

fn fixture_key(name: &str) -> Key {
    match name {
        "Up" => Key::Up,
        "Down" => Key::Down,
        "Left" => Key::Left,
        "Right" => Key::Right,
        "Home" => Key::Home,
        "End" => Key::End,
        "Insert" => Key::Insert,
        "Delete" => Key::Delete,
        "PageUp" => Key::PageUp,
        "PageDown" => Key::PageDown,
        "Backspace" => Key::Backspace,
        "BackTab" => Key::BackTab,
        function if function.starts_with('F') && function[1..].chars().all(|c| c.is_ascii_digit()) => Key::F(function[1..].parse().expect("function key number")),
        other => panic!("unknown key {other}"),
    }
}

fn fixture_keypad(name: &str) -> KeypadKey {
    match name {
        "Plus" => KeypadKey::Plus,
        "Minus" => KeypadKey::Minus,
        "Multiply" => KeypadKey::Multiply,
        "Divide" => KeypadKey::Divide,
        "Decimal" => KeypadKey::Decimal,
        "Enter" => KeypadKey::Enter,
        "Separator" => KeypadKey::Separator,
        digit => KeypadKey::Digit(digit.trim_start_matches("Digit").parse().expect("digit")),
    }
}

#[test]
fn shared_keys_encode_to_the_bytes_terminfo_promises() {
    assert!(names_capability(include_str!("../../../../🧪️tests/⌨️tui-terminal-keys/🥒️.feature"), "tui-terminal-keys"));
    let corpus = corpus(include_str!("../../../../🧫️fixtures/⌨️tui-terminal-keys/🔣️.json"));
    let cases = corpus["cases"].as_array().expect("cases");
    assert!(cases.len() >= 150);
    for case in cases {
        let capability = case["capability"].as_str().expect("capability");
        let modes_named = strings(&case["modes"]);
        let modes = InputModes { app_cursor: modes_named.contains(&"app_cursor"), app_keypad: modes_named.contains(&"app_keypad"), focus_reporting: modes_named.contains(&"focus_reporting"), ..InputModes::default() };
        let modifiers = fixture_modifiers(&case["mods"]);
        let name = case["key"].as_str().expect("key");
        let actual = match name {
            "FocusGained" => encode_focus(true, &modes).expect("focus in"),
            "FocusLost" => encode_focus(false, &modes).expect("focus out"),
            keypad if keypad.starts_with("Keypad") => encode_keypad(fixture_keypad(&keypad["Keypad".len()..]), modifiers, &modes),
            other => encode_key(&key(fixture_key(other), modifiers), &modes).unwrap_or_else(|| panic!("{capability}: no spelling")),
        };
        assert_eq!(actual, case["bytes"].as_str().expect("bytes").as_bytes(), "{capability} ({name})");
    }
    assert!(!cases.is_empty(), "the shared terminfo fixture lists capabilities");
}

#[test]
fn shared_pointer_events_encode_to_the_reports_prompt_toolkit_decodes() {
    assert!(names_capability(include_str!("../../../../🧪️tests/⌨️tui-terminal-mouse/🥒️.feature"), "tui-terminal-mouse"));
    let corpus = corpus(include_str!("../../../../🧫️fixtures/⌨️tui-terminal-mouse/🔣️.json"));
    let cases = corpus["cases"].as_array().expect("cases");
    assert!(cases.len() >= 300);
    for case in cases {
        let reporting = match case["reporting"].as_str().expect("reporting") {
            "off" => MouseReporting::Off,
            "click" => MouseReporting::Click,
            "press" => MouseReporting::Press,
            "drag" => MouseReporting::Drag,
            "motion" => MouseReporting::Motion,
            other => panic!("unknown reporting {other}"),
        };
        let encoding = match case["encoding"].as_str().expect("encoding") {
            "sgr" => MouseEncoding::Sgr,
            "x10" => MouseEncoding::X10,
            "urxvt" => MouseEncoding::Urxvt,
            "utf8" => MouseEncoding::Utf8,
            other => panic!("unknown encoding {other}"),
        };
        let button = match case["button"].as_str() {
            Some("left") => MouseButton::Left,
            Some("middle") => MouseButton::Middle,
            _ => MouseButton::Right,
        };
        let kind = match case["kind"].as_str().expect("kind") {
            "down" => MouseKind::Down(button),
            "up" => MouseKind::Up(button),
            "drag" => MouseKind::Drag(button),
            "move" => MouseKind::Move,
            "wheel_up" => MouseKind::Scroll { dx: 0, dy: -1 },
            "wheel_down" => MouseKind::Scroll { dx: 0, dy: 1 },
            other => panic!("unknown kind {other}"),
        };
        let local = Pos { x: case["x"].as_u64().expect("x") as u16, y: case["y"].as_u64().expect("y") as u16 };
        let event = pointer(kind, 0, 0, fixture_modifiers(&case["mods"]));
        let actual = encode_mouse(&event, local, &modes_with(reporting, encoding));
        assert_eq!(actual, case["bytes"].as_str().expect("bytes").as_bytes(), "{case:?}");
    }
    assert!(!cases.is_empty(), "the shared pointer fixture lists reports");
}
//#endregion 🧫️Shared Fixtures

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
