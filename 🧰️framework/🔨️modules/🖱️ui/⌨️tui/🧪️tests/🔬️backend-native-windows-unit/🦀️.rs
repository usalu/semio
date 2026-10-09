mod tests {
    use super::*;

    #[test]
    fn mode_rollback_retries_after_the_first_restore_failure() {
        let mut modes = ConsoleModeOwnership { stdin_changed: false, stdout_changed: true, ..Default::default() };
        assert!(!modes.restore_with(|_, _| false, std::ptr::null_mut(), 1, std::ptr::null_mut(), 2));
        assert!(modes.stdout_changed);
        assert!(modes.restore_with(|_, _| true, std::ptr::null_mut(), 1, std::ptr::null_mut(), 2));
        assert!(modes.is_empty());
    }

    #[test]
    fn mode_rollback_cleans_both_modes_after_a_setup_write_failure() {
        let mut modes = ConsoleModeOwnership { stdin_changed: true, stdout_changed: true, ..Default::default() };
        assert!(modes.restore_with(|_, _| true, std::ptr::null_mut(), 1, std::ptr::null_mut(), 2));
        assert!(modes.is_empty());
    }

    #[test]
    fn code_page_restoration_retries_only_the_remaining_owned_page() {
        let mut modes = ConsoleModeOwnership { input_cp_changed: true, output_cp_changed: true, ..Default::default() };
        modes.restore_code_pages_with(|page| { assert_eq!(page, 437); false }, |page| { assert_eq!(page, 850); true }, 437, 850);
        assert!(modes.input_cp_changed); assert!(!modes.output_cp_changed);
        modes.restore_code_pages_with(|page| { assert_eq!(page, 437); true }, |_| panic!("already restored"), 437, 850);
        assert!(modes.is_empty());
    }

    #[test]
    fn ansi_teardown_failure_remains_owned_for_a_later_windows_cleanup() {
        let mut ansi_setup_owned = true;
        assert!(!release_owned_terminal_cleanup(&mut ansi_setup_owned, false));
        assert!(ansi_setup_owned);
        assert!(release_owned_terminal_cleanup(&mut ansi_setup_owned, true));
        assert!(!ansi_setup_owned);
    }
}

mod windows_input_tests {
    use super::*;
    use crate::tui::component::windows_abi::{INPUT_RECORD_0, COORD, KEY_EVENT_RECORD, WINDOW_BUFFER_SIZE_RECORD};

    fn key_record(unit: u16, down: bool, repeat: u16) -> INPUT_RECORD {
        let key = KEY_EVENT_RECORD { bKeyDown: i32::from(down), wRepeatCount: repeat, wVirtualKeyCode: 0, wVirtualScanCode: 0, uChar: unit, dwControlKeyState: 0 };
        INPUT_RECORD { EventType: KEY_EVENT, Event: INPUT_RECORD_0 { KeyEvent: key } }
    }

    fn resize_record() -> INPUT_RECORD {
        let size = WINDOW_BUFFER_SIZE_RECORD { dwSize: COORD { X: 100, Y: 30 } };
        INPUT_RECORD { EventType: WINDOW_BUFFER_SIZE_EVENT, Event: INPUT_RECORD_0 { WindowBufferSizeEvent: size } }
    }

    #[test]
    fn a_set_wake_event_ends_a_pending_wait_immediately() {
        let input = WakeEvent::new().unwrap();
        let wake = Arc::new(WakeEvent::new().unwrap());
        let sender = Arc::clone(&wake);
        let started = Instant::now();
        let thread = std::thread::spawn(move || {
            std::thread::sleep(Duration::from_millis(40));
            sender.set();
        });
        let outcome = wait_signalled(input.0.as_raw(), wake.0.as_raw(), Some(Duration::from_secs(10))).unwrap();
        thread.join().unwrap();
        assert_eq!(outcome, WaitOutcome::Wake);
        assert!(started.elapsed() < Duration::from_secs(5), "the wake did not wait for the timeout: {:?}", started.elapsed());
    }

    #[test]
    fn an_unsignalled_wait_times_out_and_a_signalled_input_wins() {
        let input = WakeEvent::new().unwrap();
        let wake = WakeEvent::new().unwrap();
        let started = Instant::now();
        assert_eq!(wait_signalled(input.0.as_raw(), wake.0.as_raw(), Some(Duration::from_millis(30))).unwrap(), WaitOutcome::Timeout);
        assert!(started.elapsed() < Duration::from_secs(5), "the wait is bounded by its timeout: {:?}", started.elapsed());
        input.set();
        wake.set();
        assert_eq!(wait_signalled(input.0.as_raw(), wake.0.as_raw(), Some(Duration::from_secs(1))).unwrap(), WaitOutcome::Input);
        assert_eq!(wait_signalled(input.0.as_raw(), wake.0.as_raw(), Some(Duration::from_secs(1))).unwrap(), WaitOutcome::Wake, "the unconsumed wake is still pending");
    }

    #[test]
    fn utf16_units_reassemble_into_utf8_including_surrogate_pairs() {
        let mut decoder = Utf16Decoder::default();
        let mut bytes = Vec::new();
        for unit in "aä€".encode_utf16() {
            decoder.push(unit, &mut bytes);
        }
        assert_eq!(bytes, "aä€".as_bytes());
        bytes.clear();
        let pair: Vec<u16> = "😀".encode_utf16().collect();
        decoder.push(pair[0], &mut bytes);
        assert!(bytes.is_empty(), "a high surrogate waits for its partner");
        decoder.push(pair[1], &mut bytes);
        assert_eq!(bytes, "😀".as_bytes());
        bytes.clear();
        decoder.push(0xdc00, &mut bytes);
        decoder.push(0xd83d, &mut bytes);
        decoder.push(u16::from(b'x'), &mut bytes);
        assert_eq!(bytes, b"x", "lone surrogates are dropped, the next character survives");
    }

    #[test]
    fn console_records_become_the_vt_byte_stream_and_a_resize_flag() {
        let mut decoder = Utf16Decoder::default();
        let mut bytes = Vec::new();
        let mut records: Vec<INPUT_RECORD> = "\x1b[A".encode_utf16().map(|unit| key_record(unit, true, 1)).collect();
        records.push(key_record(u16::from(b'a'), false, 1));
        records.push(key_record(0, true, 1));
        records.push(key_record(u16::from(b'z'), true, 3));
        assert!(!records_to_bytes(&records, &mut decoder, &mut bytes));
        assert_eq!(bytes, b"\x1b[Azzz");
        assert!(records_to_bytes(&[resize_record()], &mut decoder, &mut bytes));
        let mut events = Vec::new();
        let mut parser = crate::tui::ansi::AnsiParser::new();
        parser.feed(&bytes, &mut events);
        assert_eq!(events.len(), 4);
    }

    #[test]
    fn native_control_space_delivers_the_nul_key_once() {
        let fixture = semio_framework_pack_json::parse(include_str!("../../🧫️fixtures/⌨️input-decoding/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        for vector in fixture["consoleControls"].as_array().unwrap() {
            let records: Vec<_> = vector["records"].as_array().unwrap().iter().map(|value| {
                let mut record = key_record(value["unit"].as_u64().unwrap() as u16, value["down"].as_bool().unwrap(), 1);
                let mut key = unsafe { record.Event.KeyEvent };
                key.wVirtualKeyCode = value["key"].as_u64().unwrap() as u16;
                key.dwControlKeyState = value["control"].as_u64().unwrap() as u32;
                record.Event.KeyEvent = key;
                record
            }).collect();
            let mut bytes = Vec::new();
            assert!(!records_to_bytes(&records, &mut Utf16Decoder::default(), &mut bytes));
            assert_eq!(bytes, vector["text"].as_str().unwrap().as_bytes(), "{}", vector["id"].as_str().unwrap());
        }
    }
    #[test]
    fn committed_alt_code_text_is_delivered_once() {
        let fixture = semio_framework_pack_json::parse(include_str!("../../🧫️fixtures/⌨️input-decoding/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        for vector in fixture["consoleCommits"].as_array().unwrap() {
            let records: Vec<_> = vector["records"].as_array().unwrap().iter().map(|value| {
                let mut record = key_record(value["unit"].as_u64().unwrap() as u16, value["down"].as_bool().unwrap(), 1);
                let mut key = unsafe { record.Event.KeyEvent };
                key.wVirtualKeyCode = value["key"].as_u64().unwrap() as u16;
                record.Event.KeyEvent = key;
                record
            }).collect();
            let mut bytes = Vec::new();
            assert!(!records_to_bytes(&records, &mut Utf16Decoder::default(), &mut bytes));
            assert_eq!(bytes, vector["text"].as_str().unwrap().as_bytes());
        }
    }
}
