mod input_decoding_fixture {
    use super::*;
    use crate::tui::event::{mods, Event, Key, KeypadKey, MouseButton, MouseKind};
    use semio_framework_pack_json::{parse, JsonMemberPolicy, Value};

    const FIXTURE: &str = include_str!("../../🧫️fixtures/⌨️input-decoding/🔣️.json");

    fn mods_suffix(bits: u8) -> String {
        [(mods::CTRL, "+ctrl"), (mods::ALT, "+alt"), (mods::SHIFT, "+shift")].iter().filter(|(bit, _)| bits & bit != 0).map(|(_, name)| *name).collect()
    }

    fn key_name(key: Key) -> String {
        match key {
            Key::Char(' ') => "Space".into(),
            Key::Char('+') => "Plus".into(),
            Key::Char(c) => c.to_string(),
            Key::F(n) => format!("F{n}"),
            Key::Keypad(KeypadKey::Digit(n)) => format!("Keypad{n}"),
            Key::Keypad(other) => format!("Keypad{other:?}"),
            other => format!("{other:?}"),
        }
    }

    fn button_name(button: MouseButton) -> &'static str {
        match button {
            MouseButton::Left => "left",
            MouseButton::Middle => "middle",
            MouseButton::Right => "right",
        }
    }

    fn project(event: &Event) -> String {
        match event {
            Event::Key(key) => format!("key:{}{}", key_name(key.key), mods_suffix(key.mods)),
            Event::Paste(text) => format!("paste:{text}"),
            Event::FocusGained => "focus:gained".into(),
            Event::FocusLost => "focus:lost".into(),
            Event::Mouse(mouse) => {
                let head = match mouse.kind {
                    MouseKind::Down(button) => format!("mouse:down:{}", button_name(button)),
                    MouseKind::Up(button) => format!("mouse:up:{}", button_name(button)),
                    MouseKind::Drag(button) => format!("mouse:drag:{}", button_name(button)),
                    MouseKind::Move => "mouse:move".into(),
                    MouseKind::Scroll { dx, dy } => format!("mouse:scroll:{dx},{dy}"),
                };
                format!("{head}@{},{}#{}{}", mouse.pos.x, mouse.pos.y, mouse.clicks, mods_suffix(mouse.mods))
            }
            other => panic!("the fixture grammar has no projection for {other:?}"),
        }
    }

    fn strings(value: &Value) -> Vec<String> {
        value.as_array().expect("event list").iter().map(|item| item.as_str().expect("event text").to_string()).collect()
    }

    fn corpus() -> Value {
        parse(FIXTURE, JsonMemberPolicy::Reject).expect("closed input decoding corpus")
    }

    #[test]
    fn every_decoding_vector_projects_to_the_shared_events() {
        let corpus = corpus();
        let rows = corpus["decoding"].as_array().expect("decoding rows");
        assert!(rows.len() >= 50, "the corpus keeps its breadth");
        for row in rows {
            let mut parser = AnsiParser::new();
            let mut events = Vec::new();
            parser.feed(row["input"].as_str().expect("input").as_bytes(), &mut events);
            if row["expire"].as_bool() == Some(true) {
                parser.expire(&mut events);
            }
            let id = row["id"].as_str().expect("id");
            assert_eq!(events.iter().map(project).collect::<Vec<_>>(), strings(&row["events"]), "{id}");
            assert!(parser.pending_timeout().is_none(), "{id} leaves no partial input behind");
        }
        assert!(!rows.is_empty(), "the shared decoding fixture lists vectors");
    }

    #[test]
    fn every_decoding_vector_survives_being_fed_one_byte_at_a_time() {
        let corpus = corpus();
        for row in corpus["decoding"].as_array().expect("decoding rows") {
            let mut parser = AnsiParser::new();
            let mut events = Vec::new();
            for byte in row["input"].as_str().expect("input").as_bytes() {
                parser.feed(std::slice::from_ref(byte), &mut events);
            }
            if row["expire"].as_bool() == Some(true) {
                parser.expire(&mut events);
            }
            assert_eq!(events.iter().map(project).collect::<Vec<_>>(), strings(&row["events"]), "{}", row["id"].as_str().expect("id"));
        }
    }

    #[test]
    fn every_click_vector_counts_presses_on_the_shared_clock() {
        let corpus = corpus();
        for row in corpus["clicks"].as_array().expect("click rows") {
            let mut parser = AnsiParser::new();
            let mut counter = ClickCounter::default();
            let mut projected = Vec::new();
            for chunk in row["chunks"].as_array().expect("chunks") {
                let mut events = Vec::new();
                parser.feed(chunk["input"].as_str().expect("input").as_bytes(), &mut events);
                counter.stamp_all(&mut events, chunk["at"].as_u64().expect("timestamp"));
                projected.extend(events.iter().map(project));
            }
            assert_eq!(projected, strings(&row["events"]), "{}", row["id"].as_str().expect("id"));
        }
    }

    #[test]
    fn every_capability_vector_detects_the_shared_depth_repertoire_and_sync_support() {
        use crate::tui::backend::{detect_capabilities, ColorDepth, UnicodeLevel};
        let corpus = corpus();
        let rows = corpus["capabilities"].as_array().expect("capability rows");
        assert!(rows.len() >= 25, "the capability table keeps its breadth");
        for row in rows {
            let id = row["id"].as_str().expect("id");
            let pairs: Vec<(String, String)> = row["env"].as_object().expect("env").iter().map(|(name, value)| (name.to_string(), value.as_str().expect("value").to_string())).collect();
            let found = detect_capabilities(&|name| pairs.iter().find(|(key, _)| key == name).map(|(_, value)| value.clone()), row["windows"].as_bool().expect("windows"));
            let color = match row["color"].as_str().expect("color") {
                "TrueColor" => ColorDepth::TrueColor,
                "Ansi256" => ColorDepth::Ansi256,
                "Ansi16" => ColorDepth::Ansi16,
                "Monochrome" => ColorDepth::Monochrome,
                other => panic!("unknown colour depth {other}"),
            };
            let unicode = if row["unicode"].as_str().expect("unicode") == "Full" { UnicodeLevel::Full } else { UnicodeLevel::Ascii };
            assert_eq!((found.color, found.unicode, found.synchronized_output), (color, unicode, row["synchronized_output"].as_bool().expect("sync")), "{id}");
        }
    }

    #[test]
    fn every_quantization_vector_matches_the_shared_palette_indices() {
        let corpus = corpus();
        for row in corpus["quantization"].as_array().expect("quantization rows") {
            let rgb: Vec<u8> = row["rgb"].as_array().expect("rgb").iter().map(|channel| channel.as_u64().expect("channel") as u8).collect();
            assert_eq!(u64::from(rgb_to_ansi256(rgb[0], rgb[1], rgb[2])), row["ansi256"].as_u64().expect("ansi256"), "{rgb:?}");
            assert_eq!(u64::from(rgb_to_ansi16(rgb[0], rgb[1], rgb[2])), row["ansi16"].as_u64().expect("ansi16"), "{rgb:?}");
        }
    }
}
