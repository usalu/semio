use super::*;
use crate::{DslValue, FromValue, ToValue, retained_clone::RetainedCloneSource, retirement::controlled::ControlledRetirement};

#[test]
fn paged_native_ordered_list_edits_move_one_slot_and_preserve_cancelled_owners() {
    use crate::list::PagedListEditCursor;
    let law: serde_json::Value = serde_json::from_str(include_str!("../../../../📦️paged/🧫️fixtures/🎮️native-owner/🔣️.json")).unwrap();
    assert!(std::mem::size_of::<PagedListEditCursor>() <= law["bodyBytes"].as_u64().unwrap() as usize);
    for row in law["listEdits"].as_array().unwrap() {
        let original: Vec<u64> = serde_json::from_value(row["initial"].clone()).unwrap();
        let insertion = row["kind"] == "insert";
        for cancel_at in 0..=original.len() + 3 {
            let mut list: PagedList<u64, {usize::MAX}> = original.iter().copied().collect();
            let mut item = insertion.then(|| row["value"].as_u64().unwrap());
            let index = row["index"].as_u64().unwrap() as usize;
            let mut cursor = if insertion { PagedListEditCursor::insert(index, list.len()) } else { PagedListEditCursor::remove(index, list.len()) };
            let before = serde_json::to_value(&list).unwrap();
            let zero = cursor.step(&mut list, &mut item, 0, 4096).unwrap();
            assert_eq!(zero.moved_items, 0);
            assert_eq!(serde_json::to_value(&list).unwrap(), before);
            for _ in 0..cancel_at {
                let step = cursor.step(&mut list, &mut item, 1, 4096).unwrap();
                assert!(step.moved_items <= 1);
                assert!(step.progress.allocated_bytes + step.progress.placed_bytes + step.progress.released_allocation_bytes <= 4096);
                if step.complete { break; }
            }
            if cursor.is_finished() {
                assert_eq!(serde_json::to_value(&list).unwrap(), row["expected"]);
                if !insertion { assert_eq!(item, row["removed"].as_u64()); }
            }
            let mut retained: Vec<_> = list.iter().copied().chain(item).collect();
            let mut expected = original.clone();
            if insertion { expected.push(row["value"].as_u64().unwrap()); }
            retained.sort_unstable();
            expected.sort_unstable();
            assert_eq!(retained, expected);
            retire_owner(list, 1, 4096);
        }
    }
    eprintln!("[DEBUG] native paged insert/remove admitted one ordered slot and conserved all cancelled owners");
}

#[test]
fn paged_native_utf8_append_has_real_turns_and_exact_partial_cancellation() {
    use crate::value::observe_retirement_allocations;
    let law: serde_json::Value = serde_json::from_str(include_str!("../../../../📦️paged/🧫️fixtures/🎮️native-owner/🔣️.json")).unwrap();
    let close = &law["appendClose"];
    let budget = close["maximumBytes"].as_u64().unwrap() as usize;
    let prefix = close["destinationPrefix"].as_str().unwrap();
    let text = law["prefix"].as_str().unwrap().repeat(law["prefixRepeat"].as_u64().unwrap() as usize);
    for cancel_at in close["cancelAt"].as_array().unwrap().iter().map(|value| value.as_u64().unwrap() as usize).chain([usize::MAX]) {
        let mut destination = PagedUtf8::<{usize::MAX}>::try_from_str(prefix).unwrap();
        let (mut cursor, birth) = observe_retirement_allocations(crate::paged::PagedUtf8AppendCursor::default);
        assert_eq!(birth, (0, 0));
        let (zero, allocation) = observe_retirement_allocations(|| cursor.advance("zero grant must not bind this source", &mut destination, RetainedCloneGrant::default()).unwrap());
        assert_eq!(zero.progress(), Default::default());
        assert_eq!(allocation, (0, 0));
        for turn in 0..20000 {
            if turn == cancel_at { break; }
            let grant = match turn % 3 { 0 => RetainedCloneGrant::one_capacity_turn(budget, 64), 1 => RetainedCloneGrant::one_payload_turn(budget, 64), _ => RetainedCloneGrant::one_release_turn(budget, 64) };
            let (step, allocation) = observe_retirement_allocations(|| cursor.advance(&text, &mut destination, grant).unwrap());
            let progress = step.progress();
            assert!(progress.fits(grant));
            assert!(allocation.0 <= progress.retained_capacity_bytes && allocation.1 <= progress.released_bytes, "append: {allocation:?} {progress:?}");
            if matches!(step, RetainedCloneStep::Complete(_)) { break; }
        }
        if cancel_at == usize::MAX { assert_eq!(serde_json::to_value(&destination).unwrap(), format!("{prefix}{text}")); }
        assert!(destination.to_string_owner().starts_with(prefix));
        let (_, allocation) = observe_retirement_allocations(|| cursor.begin_close());
        assert_eq!(allocation, (0, 0));
        let (zero, allocation) = observe_retirement_allocations(|| cursor.close_step(RetainedCloneGrant::default()).unwrap());
        assert_eq!(zero.progress(), Default::default());
        assert_eq!(allocation, (0, 0));
        let mut closed = false;
        for turn in 0..20000 {
            let grant = match turn % 3 { 0 => RetainedCloneGrant::one_capacity_turn(budget, 64), 1 => RetainedCloneGrant::one_payload_turn(budget, 64), _ => RetainedCloneGrant::one_release_turn(budget, 64) };
            let (step, allocation) = observe_retirement_allocations(|| cursor.close_step(grant).unwrap());
            let progress = step.progress();
            assert!(progress.fits(grant));
            assert!(allocation.0 <= progress.retained_capacity_bytes && allocation.1 <= progress.released_bytes, "append close: {allocation:?} {progress:?}");
            if matches!(step, RetainedCloneStep::Complete(_)) { closed = true; break; }
        }
        assert!(closed && cursor.terminal_is_empty());
        let (_, allocation) = observe_retirement_allocations(|| drop(cursor));
        assert_eq!(allocation, (0, 0));
        let (owner, allocation) = observe_retirement_allocations(|| crate::retirement::controlled::ControlledRetirement::new(destination));
        assert_eq!(allocation, (0, 0));
        let mut owner = owner.unwrap();
        let mut closed = false;
        for turn in 0..20000 {
            let grant = match turn % 3 { 0 => RetainedCloneGrant::one_capacity_turn(budget, 64), 1 => RetainedCloneGrant::one_payload_turn(budget, 64), _ => RetainedCloneGrant::one_release_turn(budget, 64) };
            let (step, allocation) = observe_retirement_allocations(|| owner.step(grant).unwrap());
            let progress = step.progress();
            assert!(progress.fits(grant));
            assert!(allocation.0 <= progress.retained_capacity_bytes && allocation.1 <= progress.released_bytes, "appended destination: {allocation:?} {progress:?}");
            if matches!(step, RetainedCloneStep::Complete(_)) { closed = true; break; }
        }
        assert!(closed && owner.terminal_is_empty());
        let (_, allocation) = observe_retirement_allocations(|| drop(owner));
        assert_eq!(allocation, (0, 0));
    }
    eprintln!("[DEBUG] native UTF8 append constructor/zero grants remained allocation-free; every actual partial-close birth/release and native destination retirement admitted within4096");
}

#[test]
fn paged_native_object_keys_clone_without_contiguous_key_capacity() {
    let law: serde_json::Value = serde_json::from_str(include_str!("../../../../📦️paged/🧫️fixtures/🎮️native-owner/🔣️.json")).unwrap();
    let key = law["prefix"].as_str().unwrap().repeat(law["prefixRepeat"].as_u64().unwrap() as usize);
    let expected = serde_json::Value::Object(law["objectEntries"].as_array().unwrap().iter().map(|row| (format!("{key}{}", row["suffix"].as_str().unwrap()), row["value"].clone())).collect());
    let original: PagedMap<u64, {usize::MAX}> = serde_json::from_value(expected.clone()).unwrap();
    let mut edited = original.clone();
    let replacement = &law["objectReplacement"];
    let replacement_key = PagedUtf8::<{usize::MAX}>::try_from_str(&format!("{key}{}", replacement["suffix"].as_str().unwrap())).unwrap();
    assert_eq!(edited.insert(replacement_key.clone(), replacement["value"].as_u64().unwrap()), Some(replacement["previous"].as_u64().unwrap()));
    assert_eq!(edited.get(&replacement_key), Some(&replacement["value"].as_u64().unwrap()));
    let last_key = format!("{key}{}", law["lexicalTailSuffix"].as_str().unwrap());
    assert!(edited.last_key_value().unwrap().0.eq_str(&last_key));
    let (removed_key, _) = edited.pop_last().unwrap();
    assert!(removed_key.eq_str(&last_key));
    assert!(edited.get(&removed_key).is_none());
    retire_owner(removed_key, 1, 4096);
    retire_owner(replacement_key, 1, 4096);
    retire_owner(edited, 1, 4096);

    let source = RetainedCloneSource::from_owner(original);
    let mut cursor = PagedMap::<u64, {usize::MAX}>::retained_clone_cursor();
    for turn in 0..20000 {
        let grant = match turn % 3 { 0 => RetainedCloneGrant::one_capacity_turn(4096, 64), 1 => RetainedCloneGrant::one_payload_turn(4096, 64), _ => RetainedCloneGrant::one_release_turn(4096, 64) };
        let step = cursor.advance(source.borrow(), grant).unwrap();
        let progress = step.progress();
        assert!(progress.copied_items <= 1 && progress.copied_bytes + progress.retained_capacity_bytes + progress.released_bytes <= 4096);
        if matches!(step, RetainedCloneStep::Complete(_)) { break; }
    }
    let copied = cursor.take().unwrap();
    assert_eq!(copied.get(&key), Some(&7));
    assert_eq!(serde_json::to_value(&copied).unwrap(), expected);
    assert_eq!(serde_json::Value::from(copied.to_value()), expected);
    close_cursor::<PagedMap<u64, {usize::MAX}>>(&mut cursor, 1, 4096);
    retire_owner(copied, 1, 4096);
    eprintln!("[DEBUG] native paged object long keys match Serde and exact 4096-byte turns");
}

fn grant(fixture: &serde_json::Value) -> RetainedCloneGrant {
    let value = &fixture["grant"];
    RetainedCloneGrant {
        maximum_items: value["maximumItems"].as_u64().expect("maximum items") as usize,
        maximum_copy_bytes: value["maximumCopyBytes"].as_u64().expect("maximum copy bytes") as usize,
        maximum_capacity_bytes: value["maximumCapacityBytes"].as_u64().expect("maximum capacity bytes") as usize,
        maximum_depth: value["maximumDepth"].as_u64().expect("maximum depth") as usize, maximum_release_bytes: value["maximumReleaseBytes"].as_u64().expect("maximum capacity bytes") as usize }
}

fn close_cursor<T: RetainedClone>(cursor: &mut T::Cursor, maximum_items: usize, maximum_bytes: usize) -> usize {
    let _ = cursor.begin_close();
    let mut turns = 0usize;
    loop {
        turns += 1;
        assert!(turns < 20_000, "retained paged carrier cursor must close");
        let grant = RetainedCloneGrant { maximum_items, maximum_copy_bytes: maximum_bytes, maximum_capacity_bytes: cursor.next_close_capacity_byte_demand(maximum_bytes).unwrap(), maximum_release_bytes: cursor.next_close_release_byte_demand().unwrap(), maximum_depth: 64 };
        let step = cursor.close_step(grant).expect("retained paged carrier cursor close");
        assert!(step.progress().fits(grant));
        if matches!(step, RetainedCloneStep::Complete(_)) { break; }
    }
    assert!(cursor.terminal_is_empty());
    turns
}

fn retire_owner<T: RetireOwned>(value: T, maximum_items: usize, maximum_bytes: usize) {
    let mut retirement = ControlledRetirement::new(value).map_err(|(error, _)| error).unwrap();
    loop {
        let grant = RetainedCloneGrant { maximum_items, maximum_copy_bytes: maximum_bytes, maximum_capacity_bytes: retirement.next_capacity_byte_demand(maximum_bytes).unwrap(), maximum_release_bytes: retirement.next_release_byte_demand().unwrap(), maximum_depth: retirement.next_depth_demand().unwrap() };
        let step = retirement.step(grant).expect("retained paged carrier retirement");
        assert!(step.progress().fits(grant));
        if matches!(step, RetainedCloneStep::Complete(_)) { break; }
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
    let mut retirement = ControlledRetirement::new(copied).map_err(|(error, _)| error).unwrap();
    let release_grant = RetainedCloneGrant { maximum_release_bytes: 64, ..grant };
    let mut preparation_turns = 0;
    while retirement.next_release_byte_demand().unwrap() <= release_grant.maximum_release_bytes {
        preparation_turns += 1;
        assert!(preparation_turns < maximum_turns);
        let turn = RetainedCloneGrant { maximum_capacity_bytes: retirement.next_capacity_byte_demand(grant.maximum_copy_bytes).unwrap(), maximum_release_bytes: retirement.next_release_byte_demand().unwrap(), maximum_depth: retirement.next_depth_demand().unwrap(), ..grant };
        let (step, heap) = crate::value::observe_retirement_allocations(|| retirement.step(turn).unwrap());
        assert!(step.progress().fits(turn));
        assert_eq!(heap, (step.progress().retained_capacity_bytes, step.progress().released_bytes));
    }
    let demand = retirement.next_release_byte_demand().unwrap();
    assert!(demand > release_grant.maximum_release_bytes);
    for _ in 0..3 {
        let (step, heap) = crate::value::observe_retirement_allocations(|| retirement.step(release_grant).expect("paged octet whole physical grant remains pending"));
        assert_eq!(step, RetainedCloneStep::Progress(Default::default()));
        assert_eq!(heap, (0, 0));
        assert_eq!(retirement.next_release_byte_demand().unwrap(), demand);
        assert!(!retirement.terminal_is_empty());
    }
    eprintln!("[DEBUG] paged octet physical frontier demand={demand}; original64 denied with exact owner retained and heap0/0");
    for turn in preparation_turns + 1..=maximum_turns {
        let funded = RetainedCloneGrant { maximum_capacity_bytes: retirement.next_capacity_byte_demand(grant.maximum_copy_bytes).unwrap(), maximum_release_bytes: retirement.next_release_byte_demand().unwrap(), maximum_depth: retirement.next_depth_demand().unwrap(), ..grant };
        let step = retirement.step(funded).expect("paged octet demand-aware retirement");
        assert!(step.progress().fits(funded));
        if matches!(step, RetainedCloneStep::Complete(_)) {
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
    assert_eq!(map_copy.keys().map(PagedUtf8::to_string_owner).collect::<Vec<_>>(), entries.iter().map(|entry| entry.0.clone()).collect::<Vec<_>>());
    let map_close_turns = close_cursor::<PagedMap<String, 128>>(&mut map_cursor, grant.maximum_items, grant.maximum_capacity_bytes);
    assert!(map_turns > 1);
    assert!(map_close_turns >= 1);
    let mut cancelled_map_cursor = PagedMap::<String, 128>::retained_clone_cursor();
    assert!(matches!(cancelled_map_cursor.advance(map_source.borrow(), grant).expect("partial paged map copy"), RetainedCloneStep::Progress(_)));
    assert!(close_cursor::<PagedMap<String, 128>>(&mut cancelled_map_cursor, 1, grant.maximum_capacity_bytes) > 1);
    retire_owner(map_copy, grant.maximum_items, grant.maximum_capacity_bytes);
    drop(map_source);
}
#[test]
fn paged_native_combined_grants_and_identifier_order_follow_neutral_law() {
    use super::super::ordered_map::{BoundedOrd, BoundedOrdCursor, BoundedOrdGrant, BoundedOrdStep};
    let law: serde_json::Value = serde_json::from_str(include_str!("../../../../📦️paged/🧫️fixtures/🎮️native-owner/🔣️.json")).unwrap();
    let prefix = law["prefix"].as_str().unwrap().repeat(law["prefixRepeat"].as_u64().unwrap() as usize);
    let bytes = law["bodyBytes"].as_u64().unwrap() as usize;
    assert!(std::mem::size_of::<PagedUtf8BoundedOrdCursor<{usize::MAX}>>() <= bytes);
    for row in law["emptyComparisons"].as_array().unwrap() {
        let left = RetainedCloneSource::fixture_from_authority(std::sync::Arc::new(PagedUtf8::<{usize::MAX}>::try_from_str(row["left"].as_str().unwrap()).unwrap()), ());
        let right = RetainedCloneSource::fixture_from_authority(std::sync::Arc::new(PagedUtf8::<{usize::MAX}>::try_from_str(row["right"].as_str().unwrap()).unwrap()), ());
        let mut cursor = PagedUtf8::<{usize::MAX}>::bounded_ord_cursor();
        assert!(matches!(cursor.compare(left.borrow(), right.borrow(), BoundedOrdGrant {maximum_items:1,maximum_bytes:0}).unwrap(), BoundedOrdStep::Progress(_)));
        let BoundedOrdStep::Complete {ordering,progress} = cursor.compare(left.borrow(), right.borrow(), BoundedOrdGrant {maximum_items:1,maximum_bytes:0}).unwrap() else { panic!("empty identifier comparison requires no payload bytes"); };
        assert_eq!(progress.compared_bytes, 0);
        assert_eq!(match ordering {std::cmp::Ordering::Less=>-1,std::cmp::Ordering::Equal=>0,std::cmp::Ordering::Greater=>1},row["ordering"].as_i64().unwrap());
        cursor.begin_close();
        while !matches!(cursor.close_step(RetainedCloneGrant{maximum_items:1,maximum_release_bytes:cursor.next_close_release_byte_demand().unwrap(),maximum_capacity_bytes:cursor.next_close_capacity_byte_demand(0).unwrap(),maximum_depth:1,..Default::default()}).unwrap(),RetainedCloneStep::Complete(_)) {}
        assert!(cursor.terminal_is_empty());
    }
    for row in law["comparisons"].as_array().unwrap() {
        let left = format!("{prefix}{}", row["left"].as_str().unwrap());
        let right = format!("{prefix}{}", row["right"].as_str().unwrap());
        let left = RetainedCloneSource::fixture_from_authority(std::sync::Arc::new(PagedUtf8::<{usize::MAX}>::try_from_str(&left).unwrap()), ());
        let right = RetainedCloneSource::fixture_from_authority(std::sync::Arc::new(PagedUtf8::<{usize::MAX}>::try_from_str(&right).unwrap()), ());
        let mut comparator = PagedUtf8::<{usize::MAX}>::bounded_ord_cursor();
        let mut completed = false;
        for _ in 0..100000 {
            let step = comparator.compare(left.borrow(), right.borrow(), BoundedOrdGrant { maximum_items: 1, maximum_bytes: 7 }).unwrap();
            match step {
                BoundedOrdStep::Progress(progress) => assert!(progress.compared_items <= 1 && progress.compared_bytes <= 7),
                BoundedOrdStep::Complete { ordering, progress } => {
                    assert!(progress.compared_items <= 1 && progress.compared_bytes <= 7);
                    assert_eq!(match ordering { std::cmp::Ordering::Less => -1, std::cmp::Ordering::Equal => 0, std::cmp::Ordering::Greater => 1 }, row["ordering"].as_i64().unwrap());
                    completed = true;
                    break;
                }
            }
        }
        assert!(completed);
        let close_law = &law["comparatorClose"];
        assert!(comparator.begin_close());
        assert!(!comparator.begin_close());
        assert_eq!(comparator.close_step(Default::default()).unwrap(), RetainedCloneStep::Progress(Default::default()));
        let mut released = 0;
        loop {
            match comparator.close_step(RetainedCloneGrant{maximum_items:close_law["maximumItems"].as_u64().unwrap() as usize,maximum_copy_bytes:close_law["maximumBytes"].as_u64().unwrap() as usize,maximum_release_bytes:comparator.next_close_release_byte_demand().unwrap(),maximum_capacity_bytes:comparator.next_close_capacity_byte_demand(0).unwrap(),maximum_depth:1,..Default::default()}).unwrap() {
                RetainedCloneStep::Progress(crate::retained_clone::RetainedCloneProgress{copied_items:released_items,released_bytes,..}) => { assert_eq!(released_items, 1); assert_eq!(released_bytes, 0); released += released_items; }
                RetainedCloneStep::Complete(progress) => {assert_eq!(progress,Default::default());break;},
            }
        }
        assert_eq!(released, close_law["bindings"].as_u64().unwrap() as usize);
        assert!(comparator.terminal_is_empty());
        assert!(comparator.compare(left.borrow(), right.borrow(), BoundedOrdGrant { maximum_items: 1, maximum_bytes: 7 }).is_err());
        let mut cursor = PagedUtf8::<{usize::MAX}>::retained_clone_cursor();
        for turn in 0..100000 {
            let grant = match turn % 3 { 0 => RetainedCloneGrant::one_capacity_turn(bytes, 64), 1 => RetainedCloneGrant::one_payload_turn(bytes, 64), _ => RetainedCloneGrant::one_release_turn(bytes, 64) };
            let step = cursor.advance(left.borrow(), grant).unwrap();
            let progress = step.progress();
            assert!(progress.copied_items <= 1 && progress.copied_bytes + progress.retained_capacity_bytes + progress.released_bytes <= bytes);
            if matches!(step, RetainedCloneStep::Complete(_)) { break; }
        }
        let output = cursor.take().unwrap();
        assert_eq!(serde_json::to_value(&output).unwrap(), serde_json::to_value(left.borrow().get()).unwrap());
        close_cursor::<PagedUtf8<{usize::MAX}>>(&mut cursor, 1, bytes);
        retire_owner(output, 1, bytes);
        for at in law["cancelAt"].as_array().unwrap() {
            let mut cancelled = PagedUtf8::<{usize::MAX}>::bounded_ord_cursor();
            for _ in 0..at.as_u64().unwrap() { cancelled.compare(left.borrow(), right.borrow(), BoundedOrdGrant { maximum_items: 1, maximum_bytes: 7 }).unwrap(); }
            cancelled.begin_close();
            while !matches!(cancelled.close_step(RetainedCloneGrant{maximum_items:1,maximum_release_bytes:cancelled.next_close_release_byte_demand().unwrap(),maximum_capacity_bytes:cancelled.next_close_capacity_byte_demand(0).unwrap(),maximum_depth:1,..Default::default()}).unwrap(),RetainedCloneStep::Complete(_)) {}
            assert!(cancelled.terminal_is_empty());
            let mut cursor = PagedUtf8::<{usize::MAX}>::retained_clone_cursor();
            for turn in 0..at.as_u64().unwrap() {
                let grant = match turn % 3 { 0 => RetainedCloneGrant::one_capacity_turn(bytes, 64), 1 => RetainedCloneGrant::one_payload_turn(bytes, 64), _ => RetainedCloneGrant::one_release_turn(bytes, 64) };
                cursor.advance(left.borrow(), grant).unwrap();
            }
            close_cursor::<PagedUtf8<{usize::MAX}>>(&mut cursor, 1, bytes);
        }
    }
    eprintln!("[DEBUG] paged native long identifiers, alternating total4096 clone, cancellation and serde law verified");
}
