use super::*;

fn close(cursor: &mut RetainedDictionaryDelta, grant: usize) -> usize {
    let expected = cursor.retained_scratch_bytes();
    let mut retired = cursor.close_bytes(0);
    assert_eq!(retired, 0);
    while !cursor.terminal_is_empty() {
        let count = cursor.close_bytes(grant);
        assert!(count <= grant);
        retired += count;
    }
    assert_eq!(retired, expected);
    assert!(cursor.utf8.iter().all(|byte| *byte == 0));
    retired
}

fn verify_payload_fixture(accepted: bool) -> usize {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixture/🔣️.json")).unwrap();
    let mut selected = 0;
    for row in fixture["cases"].as_array().unwrap().iter().filter(|row| row["error"].is_null() == accepted) {
        selected += 1;
        let name = row["id"].as_str().unwrap();
        let hex = row["hex"].as_str().unwrap();
        let mut bytes: Vec<u8> = (0..hex.len()).step_by(2).map(|index| u8::from_str_radix(&hex[index..index + 2], 16).unwrap()).collect();
        if let Some(repeat) = row.get("repeat") {
            bytes.resize(bytes.len() + repeat["count"].as_u64().unwrap() as usize, repeat["byte"].as_u64().unwrap() as u8);
        }
        for grant in fixture["grants"].as_array().unwrap().iter().map(|value| value.as_u64().unwrap() as usize) {
            let mut cursor = RetainedDictionaryDelta::new(0, bytes.len() as u64).unwrap();
            let mut zero = 0;
            assert!(!cursor.push(bytes[0], &mut zero).unwrap());
            assert_eq!(cursor.next_offset(), 0);
            let mut error = None;
            let mut entries = 0;
            let mut events = Vec::new();
            'feed: while (cursor.next_offset() as usize) < bytes.len() {
                let mut fuel = grant;
                while fuel > 0 && (cursor.next_offset() as usize) < bytes.len() {
                    let before = cursor.next_offset();
                    let before_fuel = fuel;
                    let pending = cursor.has_event();
                    if let Err(found) = cursor.push(bytes[before as usize], &mut fuel) {
                        if pending {
                            assert_eq!(cursor.next_offset(), before, "{name}");
                            assert_eq!(fuel, before_fuel, "{name}");
                        }
                        error = Some(found);
                        break 'feed;
                    }
                    assert_eq!(cursor.next_offset(), before + 1, "{name}");
                    assert_eq!(fuel + 1, before_fuel, "{name}");
                    let event = match cursor.event.as_ref() {
                        Some(DictionaryDeltaEvent::Begin { base, count }) => {
                            events.push(serde_json::json!(["begin", base, count]));
                            Some("begin")
                        }
                        Some(DictionaryDeltaEvent::Entry { offset, length }) => {
                            entries += 1;
                            events.push(serde_json::json!(["entry", offset, length]));
                            Some("entry")
                        }
                        None => None,
                    };
                    if row["cancelAt"].as_u64() == Some(cursor.next_offset()) {
                        cursor.cancel();
                        error = cursor.finish().err();
                        break 'feed;
                    }
                    if event != row["hold"].as_str() {
                        cursor.take_event().unwrap();
                        assert!(!cursor.has_event());
                    }
                }
            }
            let error = error.or_else(|| cursor.finish().err());
            if let Some(expected) = error {
                let before = cursor.next_offset();
                let mut fuel = grant;
                assert_eq!(cursor.push(0, &mut fuel), Err(expected), "{name}");
                assert_eq!(cursor.next_offset(), before);
                assert_eq!(fuel, grant);
            }
            let position = cursor.next_offset();
            let scratch = cursor.retained_scratch_bytes();
            let retired = close(&mut cursor, grant);
            let actual = error.map(|error| match error {
                DictionaryDeltaError::Malformed => "malformed",
                DictionaryDeltaError::Capacity => "capacity",
                DictionaryDeltaError::State => "state",
                DictionaryDeltaError::Cancelled => "cancelled",
            });
            let mut expected_events = Vec::new();
            for event in fixture["events"][name].as_array().unwrap() {
                if event[0] == "begin" {
                    expected_events.push(event.clone());
                } else {
                    for index in 0..event[3].as_u64().unwrap() {
                        expected_events.push(serde_json::json!(["entry", event[1].as_u64().unwrap() + index * event[4].as_u64().unwrap(), event[2]]));
                    }
                }
            }
            assert_eq!(events, expected_events, "{name}");
            assert_eq!(actual, row["error"].as_str(), "{name}");
            assert_eq!(position, row["offset"].as_u64().unwrap(), "{name}");
            assert_eq!(entries, row["entries"].as_u64().unwrap(), "{name}");
            assert_eq!(scratch as u64, row["scratchBytes"].as_u64().unwrap(), "{name}");
            assert_eq!(retired as u64, row["scratchBytes"].as_u64().unwrap(), "{name}");
        }
    }
    selected
}

#[test]
fn retained_dictionary_delta_matches_neutral_text_ranges_without_publication() {
    assert_eq!(verify_payload_fixture(true), 11);
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixture/🔣️.json")).unwrap();
    for (base, name) in [(0, "dictionary"), (7, "secondDictionary")] {
        let entries = fixture[name].as_array().unwrap();
        let mut bytes = vec![1, base, entries.len() as u8];
        for entry in entries {
            let value = entry.as_str().unwrap().as_bytes();
            bytes.push(value.len() as u8);
            bytes.extend_from_slice(value);
        }
        for grant in [1, 7, 4096] {
            let mut cursor = RetainedDictionaryDelta::new(32, 32 + bytes.len() as u64).unwrap();
            let mut ranges = Vec::new();
            let mut offset = 0;
            let mut zero = 0;
            assert!(!cursor.push(bytes[0], &mut zero).unwrap());
            assert_eq!(cursor.next_offset(), 32);
            while offset < bytes.len() {
                let mut fuel = grant;
                while fuel > 0 && offset < bytes.len() {
                    assert!(cursor.push(bytes[offset], &mut fuel).unwrap());
                    offset += 1;
                    match cursor.take_event().unwrap() {
                        Some(DictionaryDeltaEvent::Begin { base: actual, count }) => {
                            assert_eq!(actual, u64::from(base));
                            assert_eq!(count, entries.len() as u64);
                        }
                        Some(DictionaryDeltaEvent::Entry { offset, length }) => ranges.push((offset, length)),
                        None => {}
                    }
                }
            }
            cursor.finish().unwrap();
            assert_eq!(ranges.len(), entries.len());
            for ((offset, length), entry) in ranges.into_iter().zip(entries) {
                assert_eq!(&bytes[(offset - 32) as usize..(offset + length - 32) as usize], entry.as_str().unwrap().as_bytes());
            }
            assert_eq!(close(&mut cursor, grant), 0);
        }
    }
    println!("[DEBUG] dictionary payload cursor: 11 neutral accepted wires x3 grants; exact ranges, UTF8, canonical LEB, no publication");
}

#[test]
fn retained_dictionary_delta_rejects_tail_and_preserves_partial_utf8_until_close() {
    assert_eq!(verify_payload_fixture(false), 23);
    for bytes in [&[1, 0, 1, 2, 195, 40][..], &[1, 0, 1, 1, 195], &[1, 0, 1, 128, 0], &[1, 0, 0, 0]] {
        let mut cursor = RetainedDictionaryDelta::new(0, bytes.len() as u64).unwrap();
        let mut error = None;
        for byte in bytes {
            let mut fuel = 1;
            if let Err(found) = cursor.push(*byte, &mut fuel) {
                error = Some(found);
                break;
            }
            cursor.take_event().unwrap();
        }
        assert_eq!(error.or_else(|| cursor.finish().err()), Some(DictionaryDeltaError::Malformed));
        close(&mut cursor, 1);
    }
    let bytes = [1, 0, 1, 4, 240, 159, 152, 128];
    for boundary in 0..=bytes.len() {
        let mut cursor = RetainedDictionaryDelta::new(0, bytes.len() as u64).unwrap();
        for byte in &bytes[..boundary] {
            let mut fuel = 1;
            cursor.push(*byte, &mut fuel).unwrap();
            cursor.take_event().unwrap();
        }
        cursor.cancel();
        assert_eq!(cursor.finish(), Err(DictionaryDeltaError::Cancelled));
        close(&mut cursor, 1);
    }
    for bytes in [&[1, 0, 1, 0][..], &[1, 0, 1, 2, 194, 133]] {
        let mut cursor = RetainedDictionaryDelta::new(0, bytes.len() as u64).unwrap();
        for byte in bytes {
            let mut fuel = 1;
            cursor.push(*byte, &mut fuel).unwrap();
            cursor.take_event().unwrap();
        }
        cursor.finish().unwrap();
        close(&mut cursor, 1);
    }
    println!("[DEBUG] dictionary payload cursor: 23 neutral denied wires x3 grants; sticky errors/event fences and exact scratch0..4 retirement; every four-byte scalar cancellation boundary");
}
