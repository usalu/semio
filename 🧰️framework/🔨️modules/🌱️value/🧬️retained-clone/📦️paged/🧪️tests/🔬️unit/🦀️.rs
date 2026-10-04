use super::*;
use crate::{DslValue, FromValue, ToValue, retained_clone::RetainedCloneSource, retirement::owned_retirement};

fn grant(fixture: &serde_json::Value) -> RetainedCloneGrant {
    let value = &fixture["grant"];
    RetainedCloneGrant {
        maximum_items: value["maximumItems"].as_u64().expect("maximum items") as usize,
        maximum_copy_bytes: value["maximumCopyBytes"].as_u64().expect("maximum copy bytes") as usize,
        maximum_capacity_bytes: value["maximumCapacityBytes"].as_u64().expect("maximum capacity bytes") as usize,
        maximum_depth: value["maximumDepth"].as_u64().expect("maximum depth") as usize,
    }
}

fn close_cursor<T: RetainedClone>(cursor: &mut T::Cursor, maximum_items: usize, maximum_bytes: usize) -> usize {
    let _ = cursor.begin_close();
    let mut turns = 0usize;
    loop {
        turns += 1;
        assert!(turns < 20_000, "retained paged carrier cursor must close");
        match cursor.close_step(maximum_items, maximum_bytes).expect("retained paged carrier cursor close") {
            SnapshotRetirementStep::Pending { released_items, released_bytes } => {
                assert!(released_items <= maximum_items);
                assert!(released_bytes <= maximum_bytes);
            }
            SnapshotRetirementStep::Blocked => panic!("exact retained paged carrier close grant blocked"),
            SnapshotRetirementStep::Complete => break,
        }
    }
    assert!(cursor.terminal_is_empty());
    turns
}

fn retire_owner<T: RetireOwned>(value: T, maximum_items: usize, maximum_bytes: usize) {
    let mut retirement = owned_retirement(value);
    loop {
        match retirement.close_step(maximum_items, maximum_bytes).expect("retained paged carrier retirement") {
            SnapshotRetirementStep::Pending { released_items, released_bytes } => {
                assert!(released_items <= maximum_items);
                assert!(released_bytes <= maximum_bytes);
            }
            SnapshotRetirementStep::Blocked => panic!("exact retained paged carrier retirement grant blocked"),
            SnapshotRetirementStep::Complete => break,
        }
    }
    assert!(retirement.terminal_is_empty());
}

#[test]
fn paged_octets_copy_and_retire_by_credited_bytes_under_single_item_grants() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/📦️owners/🔣️.json")).expect("paged owner fixture");
    let length = fixture["bytes"]["length"].as_u64().expect("octet length") as usize;
    let multiplier = fixture["bytes"]["multiplier"].as_u64().expect("octet multiplier") as usize;
    let increment = fixture["bytes"]["increment"].as_u64().expect("octet increment") as usize;
    let maximum_turns = fixture["expected"]["singleItemMaximumTurns"].as_u64().expect("single-item maximum turns") as usize;
    let expected: Vec<u8> = (0..length).map(|ordinal| ((ordinal * multiplier + increment) & 255) as u8).collect();
    let source = RetainedCloneSource::from_owner(PagedBytes::<8192>::try_from_slice(&expected).expect("paged octet source"));
    let grant = RetainedCloneGrant { maximum_items: 1, ..grant(&fixture) };
    let mut cursor = PagedBytes::<8192>::retained_clone_cursor();
    let mut turns = 0usize;
    loop {
        turns += 1;
        assert!(turns <= maximum_turns, "paged octet clone exceeded its neutral single-item turn bound");
        let step = cursor.advance(source.borrow(), grant).expect("paged octet single-item clone");
        let progress = step.progress();
        assert!(progress.fits(grant));
        if matches!(step, RetainedCloneStep::Complete(_)) {
            break;
        }
    }
    let copied = cursor.take().expect("paged octet single-item owner");
    assert_eq!(serde_json::to_value(copied.to_vec_owner()).expect("serde octet oracle"), serde_json::to_value(&expected).expect("serde expected octets"));
    assert!(close_cursor::<PagedBytes<8192>>(&mut cursor, grant.maximum_items, grant.maximum_capacity_bytes) <= maximum_turns);
    let mut retirement = owned_retirement(copied);
    let mut refusal = None;
    for turn in 1..=maximum_turns {
        match retirement.close_step(grant.maximum_items, grant.maximum_copy_bytes) {
            Ok(SnapshotRetirementStep::Complete) => panic!("paged octet retirement unexpectedly completed without paying its physical page-release demand"),
            Ok(_) => assert!(turn < maximum_turns, "paged octet retirement did not reach its physical release frontier"),
            Err(error) => {
                refusal = Some(error);
                break;
            }
        }
    }
    assert_eq!(refusal.expect("undersized page-release grant refusal").kind, ValueRefusalKind::WorkLimit);
    assert!(retirement.next_close_byte_demand() > grant.maximum_copy_bytes);
    for turn in 1..=maximum_turns {
        let demand = retirement.next_close_byte_demand().max(grant.maximum_copy_bytes);
        let step = retirement.close_step(grant.maximum_items, demand).expect("paged octet demand-aware retirement");
        if step == SnapshotRetirementStep::Complete {
            assert!(retirement.terminal_is_empty());
            drop(source);
            return;
        }
        assert!(turn < maximum_turns, "paged octet demand-aware retirement exceeded its neutral single-item turn bound");
    }
    unreachable!()
}

#[test]
fn paged_text_and_map_copy_match_serde_oracles_and_close_terminal_empty() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/📦️owners/🔣️.json")).expect("paged owner fixture");
    let grant = grant(&fixture);
    let byte_length = fixture["bytes"]["length"].as_u64().expect("octet length") as usize;
    let multiplier = fixture["bytes"]["multiplier"].as_u64().expect("octet multiplier") as usize;
    let increment = fixture["bytes"]["increment"].as_u64().expect("octet increment") as usize;
    let expected_bytes: Vec<u8> = (0..byte_length).map(|ordinal| ((ordinal * multiplier + increment) & 255) as u8).collect();
    let bytes = PagedBytes::<8192>::try_from_slice(&expected_bytes).expect("paged octet fixture");
    let bytes_source = RetainedCloneSource::from_owner(bytes);
    let mut bytes_cursor = PagedBytes::<8192>::retained_clone_cursor();
    let mut byte_turns = 0usize;
    loop {
        byte_turns += 1;
        assert!(byte_turns < 20_000, "paged octet copy must complete");
        if matches!(bytes_cursor.advance(bytes_source.borrow(), grant).expect("paged octet copy"), RetainedCloneStep::Complete(_)) {
            break;
        }
    }
    let bytes_copy = bytes_cursor.take().expect("paged octet copy owner");
    assert_eq!(bytes_copy.to_vec_owner(), expected_bytes);
    assert_eq!(bytes_copy.to_value(), DslValue::Bytes(expected_bytes.clone()));
    assert!(byte_turns > 1);
    assert!(close_cursor::<PagedBytes<8192>>(&mut bytes_cursor, grant.maximum_items, grant.maximum_capacity_bytes) >= 1);
    let mut cancelled_bytes_cursor = PagedBytes::<8192>::retained_clone_cursor();
    assert!(matches!(cancelled_bytes_cursor.advance(bytes_source.borrow(), grant).expect("partial paged octet copy"), RetainedCloneStep::Progress(_)));
    assert!(close_cursor::<PagedBytes<8192>>(&mut cancelled_bytes_cursor, 1, grant.maximum_capacity_bytes) > 1);
    retire_owner(bytes_copy, grant.maximum_items, grant.maximum_capacity_bytes);
    drop(bytes_source);

    let segment = fixture["text"]["segment"].as_str().expect("text segment");
    let repetitions = fixture["text"]["repetitions"].as_u64().expect("text repetitions") as usize;
    let expected_text = segment.repeat(repetitions);
    let text = PagedUtf8::<8192>::try_from_str(&expected_text).expect("paged text fixture");
    let text_source = RetainedCloneSource::from_owner(text);
    let mut text_cursor = PagedUtf8::<8192>::retained_clone_cursor();
    let mut text_turns = 0usize;
    loop {
        text_turns += 1;
        assert!(text_turns < 20_000, "paged text copy must complete");
        if matches!(text_cursor.advance(text_source.borrow(), grant).expect("paged text copy"), RetainedCloneStep::Complete(_)) {
            break;
        }
    }
    let text_copy = text_cursor.take().expect("paged text copy owner");
    assert_eq!(text_copy.to_string_owner(), expected_text);
    assert_eq!(serde_json::to_value(text_copy.to_string_owner()).expect("text serde oracle"), serde_json::Value::String(expected_text));
    let text_close_turns = close_cursor::<PagedUtf8<8192>>(&mut text_cursor, grant.maximum_items, grant.maximum_capacity_bytes);
    assert!(text_turns > 1);
    assert!(text_close_turns >= 1);
    let mut cancelled_text_cursor = PagedUtf8::<8192>::retained_clone_cursor();
    assert!(matches!(cancelled_text_cursor.advance(text_source.borrow(), grant).expect("partial paged text copy"), RetainedCloneStep::Progress(_)));
    assert!(close_cursor::<PagedUtf8<8192>>(&mut cancelled_text_cursor, 1, grant.maximum_capacity_bytes) > 1);
    retire_owner(text_copy, grant.maximum_items, grant.maximum_capacity_bytes);
    drop(text_source);

    let entry_count = fixture["map"]["entryCount"].as_u64().expect("map entry count") as usize;
    let key_prefix = fixture["map"]["keyPrefix"].as_str().expect("map key prefix");
    let value_prefix = fixture["map"]["valuePrefix"].as_str().expect("map value prefix");
    let entries: Vec<_> = (0..entry_count).map(|ordinal| (format!("{key_prefix}{ordinal:03}"), DslValue::String(format!("{value_prefix}{ordinal}")))).collect();
    let expected_map = DslValue::Object(entries.clone());
    let map = PagedMap::<String, 128>::from_value(expected_map.clone()).expect("paged map fixture");
    let map_source = RetainedCloneSource::from_owner(map);
    let mut map_cursor = PagedMap::<String, 128>::retained_clone_cursor();
    let mut map_turns = 0usize;
    loop {
        map_turns += 1;
        assert!(map_turns < 20_000, "paged map copy must complete");
        if matches!(map_cursor.advance(map_source.borrow(), grant).expect("paged map copy"), RetainedCloneStep::Complete(_)) {
            break;
        }
    }
    let map_copy = map_cursor.take().expect("paged map copy owner");
    assert_eq!(map_copy.to_value(), expected_map);
    assert_eq!(map_copy.keys().cloned().collect::<Vec<_>>(), entries.iter().map(|entry| entry.0.clone()).collect::<Vec<_>>());
    let map_close_turns = close_cursor::<PagedMap<String, 128>>(&mut map_cursor, grant.maximum_items, grant.maximum_capacity_bytes);
    assert!(map_turns > 1);
    assert!(map_close_turns >= 1);
    let mut cancelled_map_cursor = PagedMap::<String, 128>::retained_clone_cursor();
    assert!(matches!(cancelled_map_cursor.advance(map_source.borrow(), grant).expect("partial paged map copy"), RetainedCloneStep::Progress(_)));
    assert!(close_cursor::<PagedMap<String, 128>>(&mut cancelled_map_cursor, 1, grant.maximum_capacity_bytes) > 1);
    retire_owner(map_copy, grant.maximum_items, grant.maximum_capacity_bytes);
    drop(map_source);
}
