mod ansi_unit {
    use super::*;
    use crate::tui::event::{mods, Event, Key, KeyEvent, KeypadKey};
    use crate::tui::geometry::Pos;
    use crate::tui::widget::{CursorShape, CursorSpec};

    fn decode(bytes: &[u8]) -> Vec<Event> {
        let mut parser = AnsiParser::new();
        let mut events = Vec::new();
        parser.feed(bytes, &mut events);
        events
    }

    fn key(key: Key, mods: u8) -> Event {
        Event::Key(KeyEvent { key, mods })
    }

    fn mode_changes(sequence: &str) -> Vec<(u16, char)> {
        sequence
            .split("[?")
            .skip(1)
            .filter_map(|part| {
                let digits: String = part.chars().take_while(char::is_ascii_digit).collect();
                let action = part[digits.len()..].chars().next()?;
                let code = digits.parse::<u16>().ok()?;
                matches!(action, 'h' | 'l').then_some((code, action))
            })
            .collect()
    }

    #[test]
    fn setup_and_teardown_come_from_one_mode_table_in_reverse_order() {
        let setup: Vec<(u16, char)> = mode_changes(setup_sequence()).into_iter().filter(|(code, _)| *code != 1).collect();
        let teardown = mode_changes(teardown_sequence());
        let session: Vec<(u16, char)> = teardown.iter().copied().filter(|(code, _)| *code != 2026).collect();
        let inverse: Vec<(u16, char)> = setup.iter().rev().map(|(code, action)| (*code, if *action == 'h' { 'l' } else { 'h' })).collect();
        assert_eq!(session, inverse);
        for code in [1049, 25, 1000, 1003, 1006, 1004, 2004] {
            assert!(setup.iter().any(|(mode, _)| *mode == code), "setup owns mode {code}");
        }
        assert_eq!(teardown.first(), Some(&(2026, 'l')), "teardown closes a synchronized frame first");
    }

    #[test]
    fn setup_enables_any_motion_sgr_mouse_focus_and_paste_and_resets_cursor_keys() {
        let setup = setup_sequence();
        for needle in ["\x1b[?1003h", "\x1b[?1006h", "\x1b[?1004h", "\x1b[?2004h", "\x1b[?1l\x1b>"] {
            assert!(setup.contains(needle), "{needle:?}");
        }
        assert!(!setup.contains("\x1b[?1002h"), "button-event tracking is superseded by any-motion tracking");
        assert!(teardown_sequence().contains("\x1b[0 q"), "teardown resets the cursor style");
    }

    fn spec(x: u16, y: u16, shape: CursorShape, blink: bool) -> CursorSpec {
        CursorSpec { pos: Pos { x, y }, shape, blink }
    }

    #[test]
    fn cursor_frame_places_shapes_and_shows_the_cursor_once() {
        let mut cursor = CursorEmitter::default();
        let first = cursor.frame("", Some(spec(4, 2, CursorShape::Bar, false)), false);
        assert_eq!(first, "\x1b[6 q\x1b[3;5H\x1b[?25h");
        assert_eq!(cursor.frame("", Some(spec(4, 2, CursorShape::Bar, false)), false), "", "an unchanged cursor writes nothing");
        assert_eq!(cursor.frame("", Some(spec(5, 2, CursorShape::Bar, false)), false), "\x1b[3;6H");
        assert_eq!(cursor.frame("", Some(spec(5, 2, CursorShape::Block, true)), false), "\x1b[1 q");
        assert_eq!(cursor.frame("", Some(spec(5, 2, CursorShape::Underline, true)), false), "\x1b[3 q");
        assert_eq!(cursor.frame("", None, false), "\x1b[?25l");
        assert_eq!(cursor.frame("", None, false), "", "a hidden cursor stays hidden without writes");
    }

    #[test]
    fn cursor_frame_hides_while_painting_and_restores_the_position_afterwards() {
        let mut cursor = CursorEmitter::default();
        cursor.frame("", Some(spec(1, 1, CursorShape::Block, false)), false);
        let frame = cursor.frame("PATCH", Some(spec(1, 1, CursorShape::Block, false)), false);
        assert_eq!(frame, "\x1b[?25lPATCH\x1b[2;2H\x1b[?25h");
        let hidden = cursor.frame("PATCH", None, false);
        assert_eq!(hidden, "\x1b[?25lPATCH", "a visible cursor hides before painting and stays hidden");
        assert_eq!(cursor.frame("PATCH", None, false), "PATCH", "an already hidden cursor needs no further hide");
    }

    #[test]
    fn cursor_frame_wraps_a_synchronized_update_only_when_something_is_written() {
        let mut cursor = CursorEmitter::default();
        let frame = cursor.frame("PATCH", None, true);
        assert_eq!(frame, format!("{SYNC_BEGIN}PATCH{SYNC_END}"));
        assert_eq!(cursor.frame("", None, true), "");
    }

    #[test]
    fn cursor_reset_forgets_the_terminal_state() {
        let mut cursor = CursorEmitter::default();
        let target = spec(0, 0, CursorShape::Block, true);
        cursor.frame("", Some(target), false);
        cursor.reset();
        assert_eq!(cursor.frame("", Some(target), false), "\x1b[1 q\x1b[1;1H\x1b[?25h");
    }

    const TRUE: &str = "\x1b[0;1;38;2;255;0;0;48;2;0;0;0mred \u{2500}\u{1f600}\x1b[0;4;38;2;128;128;128;48;2;30;144;255m!";

    #[test]
    fn truecolor_patches_pass_through_untouched() {
        assert!(matches!(quantize_patch(TRUE, ColorDepth::TrueColor), Cow::Borrowed(_)));
    }

    #[test]
    fn ansi256_patches_use_palette_indices_and_keep_text_and_attributes() {
        assert_eq!(quantize_patch(TRUE, ColorDepth::Ansi256), "\x1b[0;1;38;5;196;48;5;16mred \u{2500}\u{1f600}\x1b[0;4;38;5;244;48;5;75m!");
    }

    #[test]
    fn ansi16_patches_use_the_sixteen_system_colours() {
        assert_eq!(quantize_patch(TRUE, ColorDepth::Ansi16), "\x1b[0;1;91;40mred \u{2500}\u{1f600}\x1b[0;4;37;106m!");
    }

    #[test]
    fn monochrome_patches_keep_only_attributes() {
        assert_eq!(quantize_patch(TRUE, ColorDepth::Monochrome), "\x1b[0;1mred \u{2500}\u{1f600}\x1b[0;4m!");
        assert_eq!(quantize_patch("\x1b[31;42;7mx", ColorDepth::Monochrome), "\x1b[7mx");
    }

    #[test]
    fn quantization_leaves_cursor_sequences_and_fragments_alone() {
        let patch = "\x1b[3;4H\x1b[?25h\x1b[38;2;1;2";
        assert_eq!(quantize_patch(patch, ColorDepth::Ansi256), patch);
        assert_eq!(quantize_patch("tail\x1b[", ColorDepth::Ansi16), "tail\x1b[");
    }

    #[test]
    fn indexed_colours_are_reduced_for_sixteen_colour_terminals() {
        assert_eq!(quantize_patch("\x1b[38;5;196m", ColorDepth::Ansi16), "\x1b[91m");
        assert_eq!(quantize_patch("\x1b[48;5;3m", ColorDepth::Ansi16), "\x1b[43m");
        assert_eq!(quantize_patch("\x1b[48;5;12m", ColorDepth::Ansi16), "\x1b[104m");
        assert_eq!(quantize_patch("\x1b[38;5;200m", ColorDepth::Ansi256), "\x1b[38;5;200m");
    }

    #[test]
    fn invalid_utf8_never_swallows_the_next_byte() {
        assert_eq!(decode(&[0xc3, b'a']), vec![key(Key::Char('a'), 0)]);
        assert_eq!(decode(&[0xe2, 0x82, b'b']), vec![key(Key::Char('b'), 0)]);
        assert_eq!(decode(&[0x80, 0xbf, b'c']), vec![key(Key::Char('c'), 0)]);
        assert_eq!(decode(&[0xc0, 0x80, b'd']), vec![key(Key::Char('d'), 0)]);
        assert_eq!(decode(&[0xff, b'e']), vec![key(Key::Char('e'), 0)]);
    }

    #[test]
    fn oversized_parameter_lists_and_numbers_do_not_break_the_decoder() {
        let many: String = (0..40).map(|n| n.to_string()).collect::<Vec<_>>().join(";");
        assert_eq!(decode(format!("\x1b[{many}A").as_bytes()), vec![key(Key::Up, 0)]);
        assert_eq!(decode(b"\x1b[99999999999999A"), vec![key(Key::Up, 0)]);
    }

    #[test]
    fn an_escape_or_cancel_inside_a_sequence_restarts_decoding() {
        assert_eq!(decode(b"\x1b[1\x1b[A"), vec![key(Key::Up, 0)]);
        assert_eq!(decode(b"\x1b[1\x18x"), vec![key(Key::Char('x'), 0)]);
        assert_eq!(decode("\u{1b}[1é".as_bytes()), vec![key(Key::Char('é'), 0)]);
    }

    #[test]
    fn linux_console_function_keys_decode() {
        assert_eq!(decode(b"\x1b[[A\x1b[[E"), vec![key(Key::F(1), 0), key(Key::F(5), 0)]);
    }

    #[test]
    fn keypad_application_mode_keys_decode() {
        assert_eq!(
            decode(b"\x1bOM\x1bOp\x1bOy\x1bOk\x1b[57399u\x1b[57414;5u"),
            vec![key(Key::Keypad(KeypadKey::Enter), 0), key(Key::Keypad(KeypadKey::Digit(0)), 0), key(Key::Keypad(KeypadKey::Digit(9)), 0), key(Key::Keypad(KeypadKey::Plus), 0), key(Key::Keypad(KeypadKey::Digit(0)), 0), key(Key::Keypad(KeypadKey::Enter), mods::CTRL)]
        );
    }

    #[test]
    fn keypad_keys_fold_to_the_text_a_text_widget_expects() {
        let typed: Vec<Key> = decode(b"\x1bOM\x1bOp\x1bOy\x1bOk\x1bOo")
            .into_iter()
            .map(|event| match event {
                Event::Key(event) => event.text_equivalent().key,
                other => panic!("unexpected {other:?}"),
            })
            .collect();
        assert_eq!(typed, vec![Key::Enter, Key::Char('0'), Key::Char('9'), Key::Char('+'), Key::Char('/')]);
    }

    fn paste_of(bytes: &[u8]) -> String {
        let mut parser = AnsiParser::new();
        let mut events = Vec::new();
        parser.feed(b"\x1b[200~", &mut events);
        parser.feed(bytes, &mut events);
        parser.feed(b"\x1b[201~", &mut events);
        match events.as_slice() {
            [Event::Paste(text)] => text.clone(),
            other => panic!("expected exactly one paste, got {other:?}"),
        }
    }

    #[test]
    fn paste_decodes_utf8_once_and_keeps_split_characters_whole() {
        assert_eq!(paste_of("ä€😀".as_bytes()), "ä€😀");
        let mut parser = AnsiParser::new();
        let mut events = Vec::new();
        let bytes = "x😀y".as_bytes();
        parser.feed(b"\x1b[200~", &mut events);
        parser.feed(&bytes[..3], &mut events);
        parser.feed(&bytes[3..], &mut events);
        parser.feed(b"\x1b[201~", &mut events);
        assert_eq!(events, vec![Event::Paste("x😀y".into())]);
    }

    #[test]
    fn paste_keeps_partial_terminator_lookalikes() {
        assert_eq!(paste_of(b"a\x1b[20b\x1b[2\x1b\x1b[x"), "a\x1b[20b\x1b[2\x1b\x1b[x");
    }

    #[test]
    fn paste_is_capped_at_the_limit_on_a_character_boundary() {
        let text = "ä".repeat(PASTE_LIMIT);
        let pasted = paste_of(text.as_bytes());
        assert!(pasted.len() <= PASTE_LIMIT);
        assert!(pasted.len() >= PASTE_LIMIT - 1);
        assert!(pasted.chars().all(|c| c == 'ä'));
        let ascii = paste_of(&vec![b'a'; PASTE_LIMIT + 4096]);
        assert_eq!(ascii.len(), PASTE_LIMIT);
    }

    #[test]
    fn a_paste_without_terminator_is_delivered_on_expiry_and_input_resumes() {
        let mut parser = AnsiParser::new();
        let mut events = Vec::new();
        parser.feed(b"\x1b[200~partial", &mut events);
        assert!(parser.in_paste());
        assert_eq!(parser.pending_timeout(), Some(PASTE_TIMEOUT));
        parser.expire(&mut events);
        assert_eq!(events, vec![Event::Paste("partial".into())]);
        assert!(!parser.in_paste());
        parser.feed(b"k", &mut events);
        assert_eq!(events.last(), Some(&key(Key::Char('k'), 0)));
    }

    #[test]
    fn partial_input_reports_its_timeout_and_expires_cleanly() {
        let mut parser = AnsiParser::new();
        let mut events = Vec::new();
        assert_eq!(parser.pending_timeout(), None);
        parser.feed(b"\x1b", &mut events);
        assert_eq!(parser.pending_timeout(), Some(ESCAPE_TIMEOUT));
        parser.feed(b"[1;", &mut events);
        assert_eq!(parser.pending_timeout(), Some(SEQUENCE_TIMEOUT));
        parser.expire(&mut events);
        assert!(events.is_empty(), "a half-received control sequence is dropped, never typed");
        assert_eq!(parser.pending_timeout(), None);
        parser.feed(b"\x1b]0;title", &mut events);
        assert_eq!(parser.pending_timeout(), Some(SEQUENCE_TIMEOUT));
        parser.expire(&mut events);
        parser.feed(b"x", &mut events);
        assert_eq!(events, vec![key(Key::Char('x'), 0)]);
        parser.feed(&[0xe2, 0x82], &mut events);
        assert_eq!(parser.pending_timeout(), Some(SEQUENCE_TIMEOUT));
        parser.expire(&mut events);
        parser.feed(b"y", &mut events);
        assert_eq!(events.last(), Some(&key(Key::Char('y'), 0)));
    }

    #[test]
    fn flush_escape_resolves_only_a_lone_escape() {
        let mut parser = AnsiParser::new();
        let mut events = Vec::new();
        parser.feed(b"\x1b[", &mut events);
        parser.flush_escape(&mut events);
        assert!(events.is_empty());
        let mut lone = AnsiParser::new();
        lone.feed(b"\x1b", &mut events);
        lone.flush_escape(&mut events);
        assert_eq!(events, vec![key(Key::Esc, 0)]);
    }

    #[test]
    fn click_counter_stamps_only_mouse_events() {
        let mut counter = ClickCounter::default();
        let mut events = vec![Event::FocusGained];
        counter.stamp_all(&mut events, 0);
        assert_eq!(events, vec![Event::FocusGained]);
    }
}
