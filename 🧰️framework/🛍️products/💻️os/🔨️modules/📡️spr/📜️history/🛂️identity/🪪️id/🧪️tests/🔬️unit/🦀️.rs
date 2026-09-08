use super::*;

fn hex(value: &str) -> Vec<u8> {
    value.as_bytes().chunks_exact(2).map(|pair| u8::from_str_radix(std::str::from_utf8(pair).unwrap(), 16).unwrap()).collect()
}

fn feed(cursor: &mut RetainedHistoryIdV1, wire: &[u8], dictionary: Option<&str>, resolved_index: Option<u32>, grant: usize) -> Result<(), HistoryIdDiagnostic> {
    let mut offset = 0;
    let mut dictionary_offset = 0;
    for _ in 0..4096 {
        let mut fuel = grant;
        while fuel > 0 {
            if let Some(index) = cursor.lookup() {
                let dictionary = dictionary.ok_or(HistoryIdDiagnostic::Malformed)?;
                assert!(cursor.begin_dictionary(resolved_index.unwrap_or(index), dictionary.len(), &mut fuel)?);
            } else if cursor.dictionary_pending() {
                let dictionary = dictionary.ok_or(HistoryIdDiagnostic::Malformed)?;
                assert!(cursor.push_dictionary(dictionary.as_bytes()[dictionary_offset], &mut fuel)?);
                dictionary_offset += 1;
            } else if offset < wire.len() {
                assert!(cursor.push_wire(wire[offset], &mut fuel)?);
                offset += 1;
            } else {
                return cursor.finish().map(|_| ());
            }
        }
    }
    panic!("bounded tagged-id cursor did not finish");
}

#[test]
fn retained_history_id_cursor_matches_neutral_bytes_and_refuses_unowned_resolution() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixture/🔣️.json")).unwrap();
    for row in fixture["completion"].as_array().unwrap() {
        let source = fixture["cases"].as_array().unwrap().iter().find(|case| case["id"] == row["case"]).unwrap();
        let wire = hex(source["wire"].as_str().unwrap());
        let wire_bytes = row["wireBytes"].as_u64().unwrap() as usize;
        let mut cursor = RetainedHistoryIdV1::new();
        let mut fuel = 4096;
        let resolve = |cursor: &mut RetainedHistoryIdV1, fuel: &mut usize| {
            if let (Some(index), Some(bytes)) = (cursor.lookup(), row["dictionaryBytes"].as_u64()) {
                let dictionary = source["dictionary"].as_str().unwrap().as_bytes();
                cursor.begin_dictionary(index, dictionary.len(), fuel).unwrap();
                for byte in &dictionary[..bytes as usize] {
                    cursor.push_dictionary(*byte, fuel).unwrap();
                }
            }
        };
        for byte in &wire[..wire_bytes] {
            resolve(&mut cursor, &mut fuel);
            cursor.push_wire(*byte, &mut fuel).unwrap();
        }
        resolve(&mut cursor, &mut fuel);
        if row["cancelled"].as_bool().unwrap() {
            cursor.cancel();
        }
        let before = (cursor.output, cursor.length, cursor.stage, cursor.value, cursor.digits, cursor.remaining, cursor.error, fuel);
        assert_eq!(cursor.is_complete(), row["complete"].as_bool().unwrap(), "{}", row["id"]);
        assert_eq!(cursor.is_complete(), row["complete"].as_bool().unwrap());
        assert_eq!(before, (cursor.output, cursor.length, cursor.stage, cursor.value, cursor.digits, cursor.remaining, cursor.error, fuel));
        if cursor.is_complete() {
            assert_eq!(cursor.finish().unwrap(), source["expected"].as_str().unwrap());
        }
        let retained = cursor.length;
        let mut retired = 0;
        while cursor.length > 0 {
            retired += cursor.close_bytes(1);
        }
        assert_eq!(retired, retained);
        assert!(cursor.output.iter().all(|byte| *byte == 0));
    }
    for row in fixture["cases"].as_array().unwrap() {
        let wire = hex(row["wire"].as_str().unwrap());
        for grant in [1, 7, 4096] {
            let mut cursor = RetainedHistoryIdV1::new();
            let mut zero = 0;
            assert!(!cursor.push_wire(wire[0], &mut zero).unwrap());
            assert_eq!(cursor.stage, Stage::Tag);
            let outcome = feed(&mut cursor, &wire, row["dictionary"].as_str(), row["resolvedIndex"].as_u64().map(|value| value as u32), grant);
            let error = outcome.err().map(|error| match error {
                HistoryIdDiagnostic::Identity => "identity",
                HistoryIdDiagnostic::Malformed => "malformed",
                HistoryIdDiagnostic::Capacity => "capacity",
                HistoryIdDiagnostic::State => "state",
                _ => panic!("unexpected neutral diagnostic"),
            });
            assert_eq!(error, row["error"].as_str(), "{}", row["id"]);
            if error.is_none() {
                assert_eq!(cursor.finish().unwrap(), row["expected"].as_str().unwrap());
            }
            let retained = cursor.length;
            let mut retired = 0;
            while cursor.length > 0 {
                let released = cursor.close_bytes(grant);
                assert!(released <= grant);
                retired += released;
            }
            assert_eq!(retired, retained);
            assert!(cursor.output.iter().all(|byte| *byte == 0));
        }
    }
    for (wire, index, bytes, expected) in [(&[1, 0][..], 1, 1, HistoryIdDiagnostic::State), (&[2, 0][..], 0, 220, HistoryIdDiagnostic::Capacity)] {
        let mut cursor = RetainedHistoryIdV1::new();
        let mut fuel = 64;
        for byte in wire {
            cursor.push_wire(*byte, &mut fuel).unwrap();
        }
        assert_eq!(cursor.begin_dictionary(index, bytes, &mut fuel), Err(expected));
        assert_eq!(cursor.finish(), Err(expected));
    }
    let wire = hex("000c4772c3bcc39f652df09f9880");
    for end in 0..=wire.len() {
        let mut cursor = RetainedHistoryIdV1::new();
        let mut fuel = 256;
        for byte in &wire[..end] {
            cursor.push_wire(*byte, &mut fuel).unwrap();
        }
        cursor.cancel();
        assert_eq!(cursor.finish(), Err(HistoryIdDiagnostic::Cancelled));
        assert_eq!(cursor.push_wire(1, &mut fuel), Err(HistoryIdDiagnostic::Cancelled));
        while cursor.length > 0 {
            assert!(cursor.close_bytes(1) <= 1);
        }
    }
    let dictionary = "Grüße-😀".as_bytes();
    for end in 0..=dictionary.len() {
        let mut cursor = RetainedHistoryIdV1::new();
        let mut fuel = 256;
        cursor.push_wire(1, &mut fuel).unwrap();
        cursor.push_wire(0, &mut fuel).unwrap();
        cursor.begin_dictionary(0, dictionary.len(), &mut fuel).unwrap();
        for byte in &dictionary[..end] {
            cursor.push_dictionary(*byte, &mut fuel).unwrap();
        }
        cursor.cancel();
        assert_eq!(cursor.finish(), Err(HistoryIdDiagnostic::Cancelled));
        assert_eq!(cursor.push_dictionary(1, &mut fuel), Err(HistoryIdDiagnostic::Cancelled));
        while cursor.length > 0 {
            assert!(cursor.close_bytes(1) <= 1);
        }
    }
    eprintln!("[DEBUG] retained semantic ID: 20 tagged wires x 3 grants, exact UTF-8/UUID, foreign dictionary denial, prefix capacity, every wire/dictionary boundary cancellation; no authority or typed snapshot publication");
}
