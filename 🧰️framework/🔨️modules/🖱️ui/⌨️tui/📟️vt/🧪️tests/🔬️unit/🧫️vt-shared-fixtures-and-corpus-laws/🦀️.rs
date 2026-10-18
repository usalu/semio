/// 🧫️ Vt shared fixtures and corpus laws: the size, key, row, history and byte helpers, the shared corpus loaders and the pyte screen corpus law.

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
    assert!(names_capability(include_str!("../../../../../🧪️tests/⌨️tui-terminal-streams/🥒️.feature"), "tui-terminal-screen"));
    let corpus = corpus(include_str!("../../../../../🧪️tests/🧫️fixtures/⌨️tui-terminal-streams/🔣️.json"));
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
    assert!(names_capability(include_str!("../../../../../🧪️tests/⌨️tui-terminal-keys/🥒️.feature"), "tui-terminal-keys"));
    let corpus = corpus(include_str!("../../../../../🧪️tests/🧫️fixtures/⌨️tui-terminal-keys/🔣️.json"));
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
    assert!(names_capability(include_str!("../../../../../🧪️tests/⌨️tui-terminal-mouse/🥒️.feature"), "tui-terminal-mouse"));
    let corpus = corpus(include_str!("../../../../../🧪️tests/🧫️fixtures/⌨️tui-terminal-mouse/🔣️.json"));
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
