#[path="../../../../♻️retirement/🧪️tests/⏱️work/🦀️.rs"] mod close_work;
use crate::retained_clone::FixtureSourceBirth;
use super::*;
use crate::{DslValue, FromValue, ToValue, ErasedSnapshotRetirement, retained_clone::RetainedCloneSource, retirement::controlled::ControlledRetirement};
use semio_framework_value::RetainedCloneRef;

#[test]
fn paged_native_ordered_list_edits_move_one_slot_and_preserve_cancelled_owners() {
    use semio_framework_value_paged_list::list::PagedListEditCursor;
    let law: serde_json::Value = serde_json::from_str(crate::text("📦️paged/🧪️tests/🧫️fixtures/🎮️native-owner/🔣️.json")).unwrap();
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
}

#[test]
fn paged_native_utf8_append_has_real_turns_and_exact_partial_cancellation() {
    use semio_framework_value_testing_allocator::observe_retirement_allocations;
    let law: serde_json::Value = serde_json::from_str(crate::text("📦️paged/🧪️tests/🧫️fixtures/🎮️native-owner/🔣️.json")).unwrap();
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
            let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:budget,maximum_capacity_bytes:budget,maximum_release_bytes:0,maximum_depth:64};
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
            let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:budget.max(cursor.next_close_copy_byte_demand().unwrap()),maximum_capacity_bytes:cursor.next_close_capacity_byte_demand(budget).unwrap(),maximum_release_bytes:cursor.next_close_release_byte_demand().unwrap(),maximum_depth:cursor.next_depth_demand().unwrap()};
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
            let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:budget.max(owner.next_copy_byte_demand().unwrap()),maximum_capacity_bytes:owner.next_capacity_byte_demand(budget).unwrap(),maximum_release_bytes:owner.next_release_byte_demand().unwrap(),maximum_depth:owner.next_depth_demand().unwrap()};
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
}

#[test]
fn paged_native_object_keys_clone_without_contiguous_key_capacity() {
    let law: serde_json::Value = serde_json::from_str(crate::text("📦️paged/🧪️tests/🧫️fixtures/🎮️native-owner/🔣️.json")).unwrap();
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
        let grant=funded_grant(&cursor,source.borrow(),4096,grant);
        let step = cursor.advance(source.borrow(), 4096, grant).unwrap();
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
}

fn funded_grant<T:RetainedClone>(cursor:&impl RetainedCloneCursor<T>,source:RetainedCloneRef<'_,T>,body:usize,grant:RetainedCloneGrant)->RetainedCloneGrant {
    let demand=cursor.advance_demands(source,body).unwrap();
    RetainedCloneGrant {maximum_copy_bytes:grant.maximum_copy_bytes.max(demand.copy_bytes),maximum_capacity_bytes:grant.maximum_capacity_bytes.max(demand.capacity_bytes),maximum_release_bytes:grant.maximum_release_bytes.max(demand.release_bytes),maximum_depth:grant.maximum_depth.max(demand.depth),..grant}
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
    let fixture: serde_json::Value = serde_json::from_str(crate::text("🧬️retained-clone/📦️paged/🧪️tests/🧫️fixtures/📦️owners/🔣️.json")).expect("paged owner fixture");
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
        let step = cursor.advance(source.borrow(), fixture["grant"]["maximumCopyBytes"].as_u64().unwrap() as usize, grant).expect("paged octet single-item clone");
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
        let (step, heap) = semio_framework_value_testing_allocator::observe_retirement_allocations(|| retirement.step(turn).unwrap());
        assert!(step.progress().fits(turn));
        assert_eq!(heap, (step.progress().retained_capacity_bytes, step.progress().released_bytes));
    }
    let demand = retirement.next_release_byte_demand().unwrap();
    assert!(demand > release_grant.maximum_release_bytes);
    for _ in 0..3 {
        let (step, heap) = semio_framework_value_testing_allocator::observe_retirement_allocations(|| retirement.step(release_grant).expect("paged octet whole physical grant remains pending"));
        assert_eq!(step, RetainedCloneStep::Progress(Default::default()));
        assert_eq!(heap, (0, 0));
        assert_eq!(retirement.next_release_byte_demand().unwrap(), demand);
        assert!(!retirement.terminal_is_empty());
    }
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
    let fixture: serde_json::Value = serde_json::from_str(crate::text("🧬️retained-clone/📦️paged/🧪️tests/🧫️fixtures/📦️owners/🔣️.json")).expect("paged owner fixture");
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
        if matches!(bytes_cursor.advance(bytes_source.borrow(), fixture["grant"]["maximumCopyBytes"].as_u64().unwrap() as usize, grant).expect("paged octet copy"), RetainedCloneStep::Complete(_)) {
            break;
        }
    }
    let bytes_copy = bytes_cursor.take().expect("paged octet copy owner");
    assert_eq!(bytes_copy.to_vec_owner(), expected_bytes);
    assert_eq!(bytes_copy.to_value(), DslValue::Bytes(expected_bytes.clone()));
    assert!(byte_turns > 1);
    assert!(close_cursor::<PagedBytes<8192>>(&mut bytes_cursor, grant.maximum_items, grant.maximum_capacity_bytes) >= 1);
    let mut cancelled_bytes_cursor = PagedBytes::<8192>::retained_clone_cursor();
    assert!(matches!(cancelled_bytes_cursor.advance(bytes_source.borrow(), fixture["grant"]["maximumCopyBytes"].as_u64().unwrap() as usize, grant).expect("partial paged octet copy"), RetainedCloneStep::Progress(_)));
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
        let grant=funded_grant(&text_cursor,text_source.borrow(),fixture["grant"]["maximumCopyBytes"].as_u64().unwrap() as usize,grant);
        if matches!(text_cursor.advance(text_source.borrow(), fixture["grant"]["maximumCopyBytes"].as_u64().unwrap() as usize, grant).expect("paged text copy"), RetainedCloneStep::Complete(_)) {
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
    assert!(matches!(cancelled_text_cursor.advance(text_source.borrow(), fixture["grant"]["maximumCopyBytes"].as_u64().unwrap() as usize, grant).expect("partial paged text copy"), RetainedCloneStep::Progress(_)));
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
        let grant=funded_grant(&map_cursor,map_source.borrow(),fixture["grant"]["maximumCopyBytes"].as_u64().unwrap() as usize,grant);
        if matches!(map_cursor.advance(map_source.borrow(), fixture["grant"]["maximumCopyBytes"].as_u64().unwrap() as usize, grant).expect("paged map copy"), RetainedCloneStep::Complete(_)) {
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
    assert!(matches!(cancelled_map_cursor.advance(map_source.borrow(), fixture["grant"]["maximumCopyBytes"].as_u64().unwrap() as usize, grant).expect("partial paged map copy"), RetainedCloneStep::Progress(_)));
    assert!(close_cursor::<PagedMap<String, 128>>(&mut cancelled_map_cursor, 1, grant.maximum_capacity_bytes) > 1);
    retire_owner(map_copy, grant.maximum_items, grant.maximum_capacity_bytes);
    drop(map_source);
}
#[test]
fn paged_native_combined_grants_and_identifier_order_follow_neutral_law() {
    use super::super::ordered_map::{BoundedOrd, BoundedOrdCursor, BoundedOrdGrant, BoundedOrdStep};
    fn close_grant<const N:usize>(cursor:&PagedUtf8BoundedOrdCursor<N>,items:usize)->RetainedCloneGrant{let d=cursor.close_demands(4096).unwrap();assert_eq!(cursor.close_demands(d.copy_bytes).unwrap(),d,"comparator close quote must remain selected by its exact grant");RetainedCloneGrant{maximum_items:items,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}
    fn close_comparator<const N:usize>(cursor:&mut PagedUtf8BoundedOrdCursor<N>){let mut work=close_work::Work::new();while !cursor.terminal_is_empty(){let d=cursor.close_demands(4096).unwrap();let grant=close_grant(cursor,1);let(step,heap)=semio_framework_value_testing_allocator::observe_retirement_allocations(||cursor.close_step(grant).unwrap());let p=step.progress();assert!(p.fits(grant));assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes));work.observe(p==Default::default()&&!cursor.terminal_is_empty()&&cursor.close_demands(4096).unwrap()==d);}}
    let physical=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:65536,maximum_capacity_bytes:65536,maximum_release_bytes:65536,maximum_depth:64};
    let law: serde_json::Value = serde_json::from_str(crate::text("📦️paged/🧪️tests/🧫️fixtures/🎮️native-owner/🔣️.json")).unwrap();
    let prefix = law["prefix"].as_str().unwrap().repeat(law["prefixRepeat"].as_u64().unwrap() as usize);
    let bytes = law["bodyBytes"].as_u64().unwrap() as usize;
    assert!(std::mem::size_of::<PagedUtf8BoundedOrdCursor<{usize::MAX}>>() <= bytes);
    for row in law["emptyComparisons"].as_array().unwrap() {
        let left = RetainedCloneSource::from_owner(PagedUtf8::<{usize::MAX}>::try_from_str(row["left"].as_str().unwrap()).unwrap());
        let right = RetainedCloneSource::from_owner(PagedUtf8::<{usize::MAX}>::try_from_str(row["right"].as_str().unwrap()).unwrap());
        let mut cursor = PagedUtf8::<{usize::MAX}>::bounded_ord_cursor();
        for _ in 0..2 {assert!(matches!(cursor.compare(left.borrow(), right.borrow(), BoundedOrdGrant {maximum_items:1,maximum_bytes:0},physical).unwrap(), BoundedOrdStep::Authority(progress) if progress.fits(physical)));}
        assert!(matches!(cursor.compare(left.borrow(), right.borrow(), BoundedOrdGrant {maximum_items:1,maximum_bytes:0},physical).unwrap(), BoundedOrdStep::Progress(_)));
        let BoundedOrdStep::Complete {ordering,progress} = cursor.compare(left.borrow(), right.borrow(), BoundedOrdGrant {maximum_items:1,maximum_bytes:0},physical).unwrap() else { panic!("empty identifier comparison requires no payload bytes"); };
        assert_eq!(progress.compared_bytes, 0);
        assert_eq!(match ordering {std::cmp::Ordering::Less=>-1,std::cmp::Ordering::Equal=>0,std::cmp::Ordering::Greater=>1},row["ordering"].as_i64().unwrap());
        cursor.begin_close();
        close_comparator(&mut cursor);
        assert!(cursor.terminal_is_empty());
    }
    for row in law["comparisons"].as_array().unwrap() {
        let left = format!("{prefix}{}", row["left"].as_str().unwrap());
        let right = format!("{prefix}{}", row["right"].as_str().unwrap());
        let left = RetainedCloneSource::from_owner(PagedUtf8::<{usize::MAX}>::try_from_str(&left).unwrap());
        let right = RetainedCloneSource::from_owner(PagedUtf8::<{usize::MAX}>::try_from_str(&right).unwrap());
        let mut comparator = PagedUtf8::<{usize::MAX}>::bounded_ord_cursor();
        let mut completed = false;
        for _ in 0..100000 {
            let step = comparator.compare(left.borrow(), right.borrow(), BoundedOrdGrant { maximum_items: 1, maximum_bytes: 7 },physical).unwrap();
            match step {
                BoundedOrdStep::Authority(progress) => assert!(progress.fits(physical)),
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
        assert_eq!(usize::from(comparator.left.is_some())+usize::from(comparator.right.is_some()),close_law["bindings"].as_u64().unwrap()as usize);
        for _ in 0..100000{if comparator.terminal_is_empty(){break;}let grant=close_grant(&comparator,close_law["maximumItems"].as_u64().unwrap()as usize);let(step,heap)=semio_framework_value_testing_allocator::observe_retirement_allocations(||comparator.close_step(grant).unwrap());let progress=step.progress();assert!(progress.fits(grant));assert_eq!(heap,(progress.retained_capacity_bytes,progress.released_bytes));assert_eq!(left.borrow().get().to_string_owner(),format!("{prefix}{}",row["left"].as_str().unwrap()));}
        assert!(comparator.terminal_is_empty());
        assert!(comparator.compare(left.borrow(), right.borrow(), BoundedOrdGrant { maximum_items: 1, maximum_bytes: 7 },physical).is_err());
        let mut cursor = PagedUtf8::<{usize::MAX}>::retained_clone_cursor();
        for _ in 0..100000 {
            let d=cursor.advance_demands(left.borrow(),bytes).unwrap();assert!(d.copy_bytes.checked_add(d.capacity_bytes).and_then(|n|n.checked_add(d.release_bytes)).unwrap()<=bytes);let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};
            let step = cursor.advance(left.borrow(), bytes, grant).unwrap();
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
            for _ in 0..at.as_u64().unwrap() { cancelled.compare(left.borrow(), right.borrow(), BoundedOrdGrant { maximum_items: 1, maximum_bytes: 7 },physical).unwrap(); }
            cancelled.begin_close();
            close_comparator(&mut cancelled);
            assert!(cancelled.terminal_is_empty());
            let mut cursor = PagedUtf8::<{usize::MAX}>::retained_clone_cursor();
            for _ in 0..at.as_u64().unwrap() {
                let d=cursor.advance_demands(left.borrow(),bytes).unwrap();assert!(d.copy_bytes.checked_add(d.capacity_bytes).and_then(|n|n.checked_add(d.release_bytes)).unwrap()<=bytes);let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};
                cursor.advance(left.borrow(), bytes, grant).unwrap();
            }
            close_cursor::<PagedUtf8<{usize::MAX}>>(&mut cursor, 1, bytes);
        }
    }
}

fn handwritten_quote(demand:crate::RetirementDemand,body:usize)->RetainedCloneGrant{
    RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:body.max(demand.copy_bytes),maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth}
}

fn handwritten_denials(grant:RetainedCloneGrant,demand:crate::RetirementDemand)->impl Iterator<Item=RetainedCloneGrant>{
    [Some(RetainedCloneGrant{maximum_items:0,..grant}),
    (demand.copy_bytes>0).then_some(RetainedCloneGrant{maximum_copy_bytes:demand.copy_bytes.saturating_sub(1),..grant}),
    (demand.capacity_bytes>0).then_some(RetainedCloneGrant{maximum_capacity_bytes:demand.capacity_bytes.saturating_sub(1),..grant}),
    (demand.release_bytes>0).then_some(RetainedCloneGrant{maximum_release_bytes:demand.release_bytes.saturating_sub(1),..grant}),
    (demand.depth>0).then_some(RetainedCloneGrant{maximum_depth:demand.depth.saturating_sub(1),..grant})].into_iter().flatten()
}

fn handwritten_frontier<T:RetainedClone+serde::Serialize>(row:&serde_json::Value,body:usize,maximum:usize,cancel:Option<usize>,build:&impl Fn(&serde_json::Value)->T)->usize{
    use semio_framework_value_testing_allocator::observe_retirement_allocations as observe;
    let(value,heap)=observe(||build(&row["value"]));
    let(mut born,mut freed)=heap;
    let expected=serde_json::to_value(&value).unwrap();
    let demand=RetainedCloneSource::<T>::owned_constructor_demand::<()>();
    let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:RetainedCloneSource::<T>::constructor_copy_bytes(),maximum_capacity_bytes:demand.capacity_bytes,maximum_depth:demand.depth,..Default::default()};
    let(result,heap)=observe(||RetainedCloneSource::admit_owned(value,(),grant));
    let(mut source,receipt)=result.unwrap_or_else(|_|panic!("handwritten source birth"));
    assert!(receipt.fits(grant));assert_eq!(heap,(receipt.retained_capacity_bytes,receipt.released_bytes));born+=heap.0;freed+=heap.1;
    let root=source.borrow().get() as *const T as usize;
    let(mut cursor,heap)=observe(T::retained_clone_cursor);assert_eq!(heap,(0,0));
    let(mut turns,mut output)=(0,None);
    for _ in 0..maximum{
        if cancel==Some(turns){break;}
        let(demand,heap)=observe(||cursor.advance_demands(source.borrow(),body).unwrap());assert_eq!(heap,(0,0));
        let funded=handwritten_quote(demand,body);
        for denied in handwritten_denials(funded,demand){
            let(result,heap)=observe(||cursor.advance(source.borrow(), body, denied));assert_eq!(heap,(0,0));assert_eq!(result.unwrap().progress(),Default::default());
            assert_eq!(cursor.advance_demands(source.borrow(),body).unwrap(),demand);assert_eq!(source.borrow().get() as *const T as usize,root);
        }
        let(step,heap)=observe(||cursor.advance(source.borrow(), body, funded).unwrap());
        let receipt=step.progress();assert!(receipt.fits(funded));assert_eq!(heap,(receipt.retained_capacity_bytes,receipt.released_bytes));born+=heap.0;freed+=heap.1;turns+=1;
        assert_ne!(receipt,Default::default(),"{}",row["name"]);
        if matches!(step,RetainedCloneStep::Complete(_)){output=cursor.take();break;}
    }
    assert!(output.is_some()||cancel==Some(turns),"{}",row["name"]);
    if let Some(value)=output.as_ref(){assert_eq!(serde_json::to_value(value).unwrap(),expected);}
    assert_eq!(serde_json::to_value(source.borrow().get()).unwrap(),expected);
    cursor.begin_close();
    for _ in 0..maximum{
        if cursor.terminal_is_empty(){break;}
        let(demand,heap)=observe(||super::super::retained_clone_close_demands::<T>(&cursor,body).unwrap());assert_eq!(heap,(0,0));
        let funded=handwritten_quote(demand,0);
        for denied in handwritten_denials(funded,demand){
            let(result,heap)=observe(||cursor.close_step(denied));assert_eq!(heap,(0,0));assert_eq!(result.unwrap().progress(),Default::default());
            assert_eq!(super::super::retained_clone_close_demands::<T>(&cursor,body).unwrap(),demand);
        }
        assert_eq!(super::super::retained_clone_close_demands::<T>(&cursor,funded.maximum_copy_bytes).unwrap(),demand);
        let(step,heap)=observe(||cursor.close_step(funded).unwrap());let receipt=step.progress();assert!(receipt.fits(funded));assert_eq!(heap,(receipt.retained_capacity_bytes,receipt.released_bytes));born+=heap.0;freed+=heap.1;
    }
    assert!(cursor.terminal_is_empty());assert_eq!(observe(||drop(cursor)).1,(0,0));
    for _ in 0..maximum{
        if source.terminal_is_empty(){break;}
        let demand=source.next_close_demand(body).unwrap();
        let funded=handwritten_quote(demand,0);
        assert_eq!(source.next_close_demand(funded.maximum_copy_bytes).unwrap(),demand);
        let(step,heap)=observe(||source.close_step(funded).unwrap());let receipt=step.progress();assert!(receipt.fits(funded));assert_eq!(heap,(receipt.retained_capacity_bytes,receipt.released_bytes));born+=heap.0;freed+=heap.1;
    }
    assert!(source.terminal_is_empty());assert_eq!(observe(||drop(source)).1,(0,0));
    if let Some(value)=output{
        let(mut owner,heap)=observe(||ControlledRetirement::new(value).unwrap_or_else(|_|panic!("handwritten output retirement")));assert_eq!(heap,(0,0));
        for _ in 0..maximum{
            if owner.terminal_is_empty(){break;}
            let demand=owner.selected_demand(body).unwrap();
            let funded=handwritten_quote(demand,0);
            assert_eq!(owner.selected_demand(funded.maximum_copy_bytes).unwrap(),demand);
            let(step,heap)=observe(||owner.step(funded).unwrap());let receipt=step.progress();assert!(receipt.fits(funded));assert_eq!(heap,(receipt.retained_capacity_bytes,receipt.released_bytes));born+=heap.0;freed+=heap.1;
        }
        assert!(owner.terminal_is_empty());assert_eq!(observe(||drop(owner)).1,(0,0));
    }
    assert_eq!(born,freed,"{}",row["name"]);turns
}

#[test]
fn handwritten_retained_clone_demands_close_every_original_prefix_with_funded_owner_movement(){
    let law:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔮️demands/🔣️.json")).unwrap();
    let schema:serde_json::Value=serde_json::from_str(include_str!("../../🧬️schema/🔮️demands/🔣️.json")).unwrap();
    let mut oracle=std::process::Command::new("bun").args(["-e",r#"import Ajv from "ajv";const {schema,law}=JSON.parse(await Bun.stdin.text());const validate=new Ajv({strict:true}).compile(schema);if(!validate(law)||validate({...law,deniedAxes:[]}))throw Error(JSON.stringify(validate.errors));console.log(JSON.stringify(law.cases.map(row=>{const value=structuredClone(row.value);return row.kind==="bytes"?Array.from(Uint8Array.from(value)):row.kind==="text"?new TextDecoder().decode(new TextEncoder().encode(value)):value;})));"#]).stdin(std::process::Stdio::piped()).stdout(std::process::Stdio::piped()).spawn().unwrap();
    use std::io::Write;
    oracle.stdin.take().unwrap().write_all(serde_json::json!({"schema":schema,"law":law}).to_string().as_bytes()).unwrap();
    let output=oracle.wait_with_output().unwrap();assert!(output.status.success());
    let independent:Vec<serde_json::Value>=serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(independent,law["cases"].as_array().unwrap().iter().map(|row|row["value"].clone()).collect::<Vec<_>>());
    let body=law["maximumPayloadCopyBytes"].as_u64().unwrap() as usize;
    let maximum=law["maximumTurns"].as_u64().unwrap() as usize;
    for row in law["cases"].as_array().unwrap(){
        macro_rules! run {($type:ty,$build:expr)=>{{let build=$build;let total=handwritten_frontier::<$type>(row,body,maximum,None,&build);for cancel in 0..=total{handwritten_frontier::<$type>(row,body,maximum,Some(cancel),&build);}}}}
        match row["kind"].as_str().unwrap(){
            "bytes"=>run!(PagedBytes<64>,|value:&serde_json::Value|PagedBytes::try_from_slice(&serde_json::from_value::<Vec<u8>>(value.clone()).unwrap()).unwrap()),
            "text"=>run!(PagedUtf8<64>,|value:&serde_json::Value|serde_json::from_value::<PagedUtf8<64>>(value.clone()).unwrap()),
            "map"=>run!(PagedMap<u64,8>,|value:&serde_json::Value|serde_json::from_value::<PagedMap<u64,8>>(value.clone()).unwrap()),
            "list"=>run!(PagedList<String,8>,|value:&serde_json::Value|serde_json::from_value::<PagedList<String,8>>(value.clone()).unwrap()),
            _=>unreachable!(),
        }
    }
}