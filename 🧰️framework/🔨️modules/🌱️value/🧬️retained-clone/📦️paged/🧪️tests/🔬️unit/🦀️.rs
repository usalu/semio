#[path="../../../../♻️retirement/🧪️tests/⏱️work/🦀️.rs"] mod close_work;
use crate::retained_clone::FixtureSourceBirth;
use super::*;
use crate::{DslValue, FromValue, ToValue, ErasedSnapshotRetirement, retained_clone::RetainedCloneSource, retirement::controlled::ControlledRetirement};
use semio_framework_value::RetainedCloneRef;
type Failure=semio_framework_value_retirement::OriginalRetirementFailure;
use crate::retained_clone::FixtureSource;
use semio_framework_value_clone_source_testing::FixtureCloseStep;
use semio_framework_value_testing_effect_bank::{EffectBank,Wallet};
fn grant_of(d:crate::RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}
/// 🚗️ Drives one poll-shaped close turn under a real original action; the harness asserts the physical heap equals the recorded progress.
fn stepped(grant:RetainedCloneGrant,call:impl FnMut(&mut semio_framework_value::RetainedAllocationEffect<'_>)->semio_framework_value::OriginalRetirementPoll)->(RetainedCloneStep,(usize,usize)){let(outcome,heap)=crate::effect::driven(grant,call);(outcome.result.unwrap(),heap)}
/// 🧵️ Drives one reply-shaped call under a real original action; the harness asserts the physical heap equals the recorded progress.
fn driven_reply(grant:RetainedCloneGrant,call:impl for<'r> FnMut(&mut semio_framework_value::RetainedAllocationEffect<'_>,crate::retained_clone::OriginalEffectReply<'r,RetainedCloneStep,Failure>,bool)->Result<crate::retained_clone::OriginalEffectReplyComplete<'r,RetainedCloneStep,Failure>,semio_framework_value::ValueError>)->semio_framework_value::RetainedWorkOutcome<RetainedCloneStep,Failure>{
    semio_framework_value_clone_source_testing::drive::warm();
    semio_framework_value_clone_source_testing::drive::drive_reply(grant,crate::effect::CPU,|result|match result{Ok(step)=>step.progress(),Err(error)=>error.retained_progress()},call)
}
/// 🎟️ Advances one cursor turn under the physical heap check.
fn advanced<T:RetainedClone>(cursor:&mut T::Cursor,source:RetainedCloneRef<'_,T>,body:usize,grant:RetainedCloneGrant)->semio_framework_value::RetainedWorkOutcome<RetainedCloneStep,Failure>{driven_reply(grant,|effect,reply,recover|if recover{cursor.recover_advance_into(source,body,crate::effect::CPU,effect,reply)}else{cursor.advance_into(source,body,crate::effect::CPU,effect,reply)})}
/// ♻️ Closes one cursor turn under the physical heap check.
fn closed<T:RetainedClone>(cursor:&mut T::Cursor,body:usize,grant:RetainedCloneGrant)->semio_framework_value::RetainedWorkOutcome<RetainedCloneStep,Failure>{driven_reply(grant,|effect,reply,recover|if recover{cursor.recover_close_into(body,effect,reply)}else{cursor.close_into(body,effect,reply)})}
/// 📤️ Transfers the ready owner of one cursor under the physical heap check.
fn take_driven<T:RetainedClone>(cursor:&mut T::Cursor)->Option<T>{
    let demand=cursor.take_demand().unwrap();let mut output=None;
    semio_framework_value_clone_source_testing::drive::warm();
    semio_framework_value_clone_source_testing::drive::drive_reply::<RetainedCloneStep,semio_framework_value::ValueError>(grant_of(demand),crate::effect::CPU,|result|match result{Ok(step)=>step.progress(),Err(error)=>error.retained_progress()},|effect,reply,recover|if recover{cursor.recover_take(&mut output,effect,reply)}else{cursor.take(&mut output,effect,reply)}).result.unwrap();
    output
}
/// 🚦️ Starts one cursor close under the physical heap check.
fn begun_cursor<T:RetainedClone>(cursor:&mut T::Cursor)->semio_framework_value::RetainedWorkOutcome<RetainedCloneStep,Failure>{let demand=cursor.begin_close_demand().unwrap();driven_reply(grant_of(demand),|effect,reply,recover|if recover{cursor.recover_begin_close_into(effect,reply)}else{cursor.begin_close_into(effect,reply)})}
fn bounded_cursor()-><PagedUtf8<{usize::MAX}> as semio_framework_value::retained_clone::ordered_map::BoundedOrd>::Cursor{use semio_framework_value::retained_clone::ordered_map::BoundedOrd;type K=PagedUtf8<{usize::MAX}>;let mut output=None;crate::effect::turn::<Failure>(grant_of(K::bounded_ord_birth_demand()),crate::effect::CPU,&mut |effect,reply|K::admit_bounded_ord_cursor_into(&mut output,effect,reply)).result.unwrap();output.unwrap()}
fn take_value<T:semio_framework_value::retained_clone::RetainedClone,C:semio_framework_value::retained_clone::RetainedCloneCursor<T>>(cursor:&mut C)->Option<T>{let demand=cursor.take_demand().unwrap();crate::effect::take(cursor,grant_of(demand)).result.unwrap().map(|(value,_)|value)}

/// 🧗️ Runs one ordered list edit turn under the quoted grant, or none when the item grant is zero.
fn edit_turn<const N:usize>(cursor:&mut semio_framework_value_paged_list::list::PagedListEditCursor,list:&mut PagedList<u64,N>,item:&mut Option<u64>,items:usize)->semio_framework_value_paged_list::list::PagedListEditStep{
    use semio_framework_value_paged_list::list::PagedListEditOutcome;
    let demand=cursor.step_demand(list,item).unwrap();let work=cursor.step_work_demand(list,item).unwrap();
    let grant=RetainedCloneGrant{maximum_items:items,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};
    let mut actual=None;
    match crate::effect::with_effect(grant,work,&mut actual,|effect|cursor.step(list,item,effect)).0{
        semio_framework_value::RetainedWorkPoll::Complete(turn)=>match turn.result.unwrap(){PagedListEditOutcome::Ready(step)=>step,PagedListEditOutcome::Denied=>Default::default()},
        semio_framework_value::RetainedWorkPoll::Pending{..}=>panic!("synchronous list edit remained pending"),
    }
}

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
            let zero = edit_turn(&mut cursor, &mut list, &mut item, 0);
            assert_eq!(zero.moved_items, 0);
            assert_eq!(serde_json::to_value(&list).unwrap(), before);
            for _ in 0..cancel_at {
                let step = edit_turn(&mut cursor, &mut list, &mut item, 1);
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
        let (zero, allocation) = observe_retirement_allocations(|| cursor.advance("zero grant must not bind this source",&mut destination,0,RetainedCloneGrant::default()).result.unwrap());
        assert_eq!(zero.progress(), Default::default());
        assert_eq!(allocation, (0, 0));
        for turn in 0..20000 {
            if turn == cancel_at { break; }
            let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:budget,maximum_capacity_bytes:budget,maximum_release_bytes:0,maximum_depth:64};
            let (step, allocation) = observe_retirement_allocations(|| cursor.advance(&text,&mut destination,grant.maximum_copy_bytes,grant).result.unwrap());
            let progress = step.progress();
            assert!(progress.fits(grant));
            assert!(allocation.0 <= progress.retained_capacity_bytes && allocation.1 <= progress.released_bytes, "append: {allocation:?} {progress:?}");
            if matches!(step, RetainedCloneStep::Complete(_)) { break; }
        }
        if cancel_at == usize::MAX { assert_eq!(serde_json::to_value(&destination).unwrap(), format!("{prefix}{text}")); }
        assert!(destination.to_string_owner().starts_with(prefix));
        let (_, allocation) = observe_retirement_allocations(|| cursor.begin_close());
        assert_eq!(allocation, (0, 0));
        let (zero, allocation) = stepped(RetainedCloneGrant::default(), |effect| cursor.close_step(0, effect));
        assert_eq!(zero.progress(), Default::default());
        assert_eq!(allocation, (0, 0));
        let mut closed = false;
        for turn in 0..20000 {
            let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:budget.max(cursor.next_close_copy_byte_demand().unwrap()),maximum_capacity_bytes:cursor.next_close_capacity_byte_demand(budget).unwrap(),maximum_release_bytes:cursor.next_close_release_byte_demand().unwrap(),maximum_depth:cursor.next_depth_demand().unwrap()};
            let (step, allocation) = stepped(grant, |effect| cursor.close_step(grant.maximum_copy_bytes, effect));
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
            let (step, allocation) = stepped(grant, |effect| owner.step(grant.maximum_copy_bytes, effect));
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
        let step = crate::effect::advance(&mut cursor, source.borrow(), 4096, crate::effect::CPU, grant).result.unwrap();
        let progress = step.progress();
        assert!(progress.copied_items <= 1 && progress.copied_bytes <= crate::retained_clone::RETAINED_CLONE_BULK_PAGE_BYTES && progress.retained_capacity_bytes + progress.released_bytes <= 4096,"turn {turn} grant {grant:?} progress {progress:?}");
        if matches!(step, RetainedCloneStep::Complete(_)) { break; }
    }
    let copied = take_value(&mut cursor).unwrap();
    assert_eq!(copied.get(&key), Some(&7));
    assert_eq!(serde_json::to_value(&copied).unwrap(), expected);
    assert_eq!(serde_json::Value::from(copied.to_value()), expected);
    close_cursor::<PagedMap<u64, {usize::MAX}>>(&mut cursor, 1, 4096);
    retire_owner(copied, 1, 4096);
}

fn funded_grant<T:RetainedClone>(cursor:&impl RetainedCloneCursor<T>,source:RetainedCloneRef<'_,T>,body:usize,grant:RetainedCloneGrant)->RetainedCloneGrant {
    let demand=cursor.advance_demands(source,body,crate::effect::CPU).unwrap();
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
    crate::effect::begin_close(cursor);
    let mut turns = 0usize;
    while !cursor.terminal_is_empty() {
        turns += 1;
        assert!(turns < 20_000, "retained paged carrier cursor must close");
        let grant = RetainedCloneGrant { maximum_items, maximum_copy_bytes: maximum_bytes, maximum_capacity_bytes: cursor.next_close_capacity_byte_demand(maximum_bytes).unwrap(), maximum_release_bytes: cursor.next_close_release_byte_demand().unwrap(), maximum_depth: 64 };
        let step = crate::effect::close_selected(cursor, maximum_bytes, grant).result.expect("retained paged carrier cursor close");
        assert!(step.progress().fits(grant));
    }
    turns
}

fn retire_owner<T: RetireOwned>(value: T, maximum_items: usize, maximum_bytes: usize) {
    let mut retirement = ControlledRetirement::new(value).map_err(|(error, _)| error).unwrap();
    while !retirement.terminal_is_empty() {
        let grant = RetainedCloneGrant { maximum_items, maximum_copy_bytes: maximum_bytes, maximum_capacity_bytes: retirement.next_capacity_byte_demand(maximum_bytes).unwrap(), maximum_release_bytes: retirement.next_release_byte_demand().unwrap(), maximum_depth: retirement.next_depth_demand().unwrap() };
        let (step, _) = stepped(grant, |effect| retirement.step(maximum_bytes, effect));
        assert!(step.progress().fits(grant));
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
        let funded = funded_grant(&cursor, source.borrow(), fixture["grant"]["maximumCopyBytes"].as_u64().unwrap() as usize, grant);
        let step = crate::effect::advance(&mut cursor, source.borrow(), fixture["grant"]["maximumCopyBytes"].as_u64().unwrap() as usize, crate::effect::CPU, funded).result.expect("paged octet single-item clone");
        let progress = step.progress();
        assert!(progress.fits(funded));
        if matches!(step, RetainedCloneStep::Complete(_)) {
            break;
        }
    }
    let copied = take_value(&mut cursor).expect("paged octet single-item owner");
    assert_eq!(serde_json::to_value(copied.to_vec_owner()).expect("serde octet oracle"), serde_json::to_value(&expected).expect("serde expected octets"));
    assert!(close_cursor::<PagedBytes<8192>>(&mut cursor, grant.maximum_items, grant.maximum_capacity_bytes) <= maximum_turns);
    let mut retirement = ControlledRetirement::new(copied).map_err(|(error, _)| error).unwrap();
    let release_grant = RetainedCloneGrant { maximum_release_bytes: 64, ..grant };
    let body = grant.maximum_copy_bytes;
    let quoted = |retirement: &ControlledRetirement<PagedBytes<8192>>, release: Option<usize>| { let d = retirement.selected_demand(body).unwrap(); RetainedCloneGrant { maximum_copy_bytes: d.copy_bytes.max(body), maximum_capacity_bytes: d.capacity_bytes, maximum_release_bytes: release.unwrap_or(d.release_bytes), maximum_depth: d.depth, ..grant } };
    let mut preparation_turns = 0;
    while retirement.next_release_byte_demand().unwrap() <= release_grant.maximum_release_bytes {
        preparation_turns += 1;
        assert!(preparation_turns < maximum_turns);
        let turn = quoted(&retirement, None);
        let (step, heap) = stepped(turn, |effect| retirement.step(body, effect));
        assert!(step.progress().fits(turn));
        assert_eq!(heap, (step.progress().retained_capacity_bytes, step.progress().released_bytes));
    }
    let demand = retirement.next_release_byte_demand().unwrap();
    assert!(demand > release_grant.maximum_release_bytes);
    for _ in 0..3 {
        let denied = quoted(&retirement, Some(release_grant.maximum_release_bytes));
        let (step, heap) = stepped(denied, |effect| retirement.step(body, effect));
        assert_eq!(step, RetainedCloneStep::Progress(Default::default()));
        assert_eq!(heap, (0, 0));
        assert_eq!(retirement.next_release_byte_demand().unwrap(), demand);
        assert!(!retirement.terminal_is_empty());
    }
    for turn in preparation_turns + 1..=maximum_turns {
        let funded = quoted(&retirement, None);
        let (step, _) = stepped(funded, |effect| retirement.step(body, effect));
        assert!(step.progress().fits(funded));
        if retirement.terminal_is_empty() {
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
        let funded = funded_grant(&bytes_cursor, bytes_source.borrow(), fixture["grant"]["maximumCopyBytes"].as_u64().unwrap() as usize, grant);
        if matches!(crate::effect::advance(&mut bytes_cursor, bytes_source.borrow(), fixture["grant"]["maximumCopyBytes"].as_u64().unwrap() as usize, crate::effect::CPU, funded).result.expect("paged octet copy"), RetainedCloneStep::Complete(_)) {
            break;
        }
    }
    let bytes_copy = take_value(&mut bytes_cursor).expect("paged octet copy owner");
    assert_eq!(bytes_copy.to_vec_owner(), expected_bytes);
    assert_eq!(bytes_copy.to_value(), DslValue::Bytes(expected_bytes.clone()));
    assert!(byte_turns > 1);
    assert!(close_cursor::<PagedBytes<8192>>(&mut bytes_cursor, grant.maximum_items, grant.maximum_capacity_bytes) >= 1);
    let mut cancelled_bytes_cursor = PagedBytes::<8192>::retained_clone_cursor();
    let funded = funded_grant(&cancelled_bytes_cursor, bytes_source.borrow(), fixture["grant"]["maximumCopyBytes"].as_u64().unwrap() as usize, grant);
    assert!(matches!(crate::effect::advance(&mut cancelled_bytes_cursor, bytes_source.borrow(), fixture["grant"]["maximumCopyBytes"].as_u64().unwrap() as usize, crate::effect::CPU, funded).result.expect("partial paged octet copy"), RetainedCloneStep::Progress(_)));
    assert!(close_cursor::<PagedBytes<8192>>(&mut cancelled_bytes_cursor, 1, grant.maximum_capacity_bytes) >= 1);
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
        if matches!(crate::effect::advance(&mut text_cursor, text_source.borrow(), fixture["grant"]["maximumCopyBytes"].as_u64().unwrap() as usize, crate::effect::CPU, grant).result.expect("paged text copy"), RetainedCloneStep::Complete(_)) {
            break;
        }
    }
    let text_copy = take_value(&mut text_cursor).expect("paged text copy owner");
    assert_eq!(text_copy.to_string_owner(), expected_text);
    assert_eq!(serde_json::to_value(text_copy.to_string_owner()).expect("text serde oracle"), serde_json::Value::String(expected_text));
    let text_close_turns = close_cursor::<PagedUtf8<8192>>(&mut text_cursor, grant.maximum_items, grant.maximum_capacity_bytes);
    assert!(text_turns > 1);
    assert!(text_close_turns >= 1);
    let mut cancelled_text_cursor = PagedUtf8::<8192>::retained_clone_cursor();
    assert!(matches!(crate::effect::advance(&mut cancelled_text_cursor, text_source.borrow(), fixture["grant"]["maximumCopyBytes"].as_u64().unwrap() as usize, crate::effect::CPU, grant).result.expect("partial paged text copy"), RetainedCloneStep::Progress(_)));
    assert!(close_cursor::<PagedUtf8<8192>>(&mut cancelled_text_cursor, 1, grant.maximum_capacity_bytes) >= 1);
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
        if matches!(crate::effect::advance(&mut map_cursor, map_source.borrow(), fixture["grant"]["maximumCopyBytes"].as_u64().unwrap() as usize, crate::effect::CPU, grant).result.expect("paged map copy"), RetainedCloneStep::Complete(_)) {
            break;
        }
    }
    let map_copy = take_value(&mut map_cursor).expect("paged map copy owner");
    assert_eq!(map_copy.to_value(), expected_map);
    assert_eq!(map_copy.keys().map(PagedUtf8::to_string_owner).collect::<Vec<_>>(), entries.iter().map(|entry| entry.0.clone()).collect::<Vec<_>>());
    let map_close_turns = close_cursor::<PagedMap<String, 128>>(&mut map_cursor, grant.maximum_items, grant.maximum_capacity_bytes);
    assert!(map_turns > 1);
    assert!(map_close_turns >= 1);
    let mut cancelled_map_cursor = PagedMap::<String, 128>::retained_clone_cursor();
    assert!(matches!(crate::effect::advance(&mut cancelled_map_cursor, map_source.borrow(), fixture["grant"]["maximumCopyBytes"].as_u64().unwrap() as usize, crate::effect::CPU, grant).result.expect("partial paged map copy"), RetainedCloneStep::Progress(_)));
    assert!(close_cursor::<PagedMap<String, 128>>(&mut cancelled_map_cursor, 1, grant.maximum_capacity_bytes) >= 1);
    retire_owner(map_copy, grant.maximum_items, grant.maximum_capacity_bytes);
    drop(map_source);
}
#[test]
fn paged_native_combined_grants_and_identifier_order_follow_neutral_law() {
    use super::super::ordered_map::{BoundedOrd, BoundedOrdCursor, BoundedOrdGrant};
    const BODY: usize = 4096;
    fn compared<const N:usize>(cursor:&mut PagedUtf8BoundedOrdCursor<N>,left:RetainedCloneRef<'_,PagedUtf8<N>>,right:RetainedCloneRef<'_,PagedUtf8<N>>,grant:BoundedOrdGrant,physical:RetainedCloneGrant)->RetainedCloneStep{
        driven_reply(physical,|effect,reply,recover|if recover{cursor.recover_compare_into(left,right,grant,BODY,effect,reply)}else{cursor.compare_into(left,right,grant,BODY,effect,reply)}).result.unwrap()
    }
    fn begun<const N:usize>(cursor:&mut PagedUtf8BoundedOrdCursor<N>)->RetainedCloneStep{
        let demand=cursor.begin_close_demand();
        driven_reply(grant_of(demand),|effect,reply,recover|if recover{cursor.recover_begin_close_into(effect,reply)}else{cursor.begin_close_into(effect,reply)}).result.unwrap()
    }
    fn close_grant<const N:usize>(cursor:&PagedUtf8BoundedOrdCursor<N>,items:usize)->RetainedCloneGrant{let d=cursor.close_demands(4096).unwrap();assert_eq!(cursor.close_demands(d.copy_bytes).unwrap(),d,"comparator close quote must remain selected by its exact grant");RetainedCloneGrant{maximum_items:items,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}
    fn close_turn<const N:usize>(cursor:&mut PagedUtf8BoundedOrdCursor<N>,grant:RetainedCloneGrant)->semio_framework_value::RetainedWorkOutcome<RetainedCloneStep,Failure>{
        driven_reply(grant,|effect,reply,recover|if recover{cursor.recover_close_into(4096,effect,reply)}else{cursor.close_into(4096,effect,reply)})
    }
    fn close_comparator<const N:usize>(cursor:&mut PagedUtf8BoundedOrdCursor<N>){let mut work=close_work::Work::new();while !cursor.terminal_is_empty(){let d=cursor.close_demands(4096).unwrap();let grant=close_grant(cursor,1);let step=close_turn(cursor,grant).result.unwrap();let p=step.progress();assert!(p.fits(grant));work.observe(p==Default::default()&&!cursor.terminal_is_empty()&&cursor.close_demands(4096).unwrap()==d);}}
    let physical=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:65536,maximum_capacity_bytes:65536,maximum_release_bytes:65536,maximum_depth:64};
    let law: serde_json::Value = serde_json::from_str(crate::text("📦️paged/🧪️tests/🧫️fixtures/🎮️native-owner/🔣️.json")).unwrap();
    let prefix = law["prefix"].as_str().unwrap().repeat(law["prefixRepeat"].as_u64().unwrap() as usize);
    let bytes = law["bodyBytes"].as_u64().unwrap() as usize;
    assert!(std::mem::size_of::<PagedUtf8BoundedOrdCursor<{usize::MAX}>>() <= bytes);
    for row in law["emptyComparisons"].as_array().unwrap() {
        let left = RetainedCloneSource::from_owner(PagedUtf8::<{usize::MAX}>::try_from_str(row["left"].as_str().unwrap()).unwrap());
        let right = RetainedCloneSource::from_owner(PagedUtf8::<{usize::MAX}>::try_from_str(row["right"].as_str().unwrap()).unwrap());
        let mut cursor = bounded_cursor();
        let empty = BoundedOrdGrant {maximum_items:1,maximum_bytes:0};
        for _ in 0..2 {let step = compared(&mut cursor, left.borrow(), right.borrow(), empty, physical);assert!(step.progress().fits(physical));assert!(cursor.ordering().is_none());}
        let step = compared(&mut cursor, left.borrow(), right.borrow(), empty, physical);
        assert!(matches!(step, RetainedCloneStep::Progress(_)) && cursor.ordering().is_none());
        let step = compared(&mut cursor, left.borrow(), right.borrow(), empty, physical);
        assert!(matches!(step, RetainedCloneStep::Complete(_)), "empty identifier comparison requires no payload bytes");
        assert_eq!(cursor.comparison_progress().compared_bytes, 0);
        let ordering = cursor.ordering().unwrap();
        assert_eq!(match ordering {std::cmp::Ordering::Less=>-1,std::cmp::Ordering::Equal=>0,std::cmp::Ordering::Greater=>1},row["ordering"].as_i64().unwrap());
        begun(&mut cursor);
        close_comparator(&mut cursor);
        assert!(cursor.terminal_is_empty());
    }
    for row in law["comparisons"].as_array().unwrap() {
        let left = format!("{prefix}{}", row["left"].as_str().unwrap());
        let right = format!("{prefix}{}", row["right"].as_str().unwrap());
        let left = RetainedCloneSource::from_owner(PagedUtf8::<{usize::MAX}>::try_from_str(&left).unwrap());
        let right = RetainedCloneSource::from_owner(PagedUtf8::<{usize::MAX}>::try_from_str(&right).unwrap());
        let mut comparator = bounded_cursor();
        let seven = BoundedOrdGrant { maximum_items: 1, maximum_bytes: 7 };
        let mut completed = false;
        for _ in 0..100000 {
            let step = compared(&mut comparator, left.borrow(), right.borrow(), seven, physical);
            let progress = comparator.comparison_progress();
            assert!(step.progress().fits(physical));
            assert!(progress.compared_items <= 1 && progress.compared_bytes <= 7);
            if matches!(step, RetainedCloneStep::Complete(_)) {
                let ordering = comparator.ordering().unwrap();
                assert_eq!(match ordering { std::cmp::Ordering::Less => -1, std::cmp::Ordering::Equal => 0, std::cmp::Ordering::Greater => 1 }, row["ordering"].as_i64().unwrap());
                completed = true;
                break;
            }
        }
        assert!(completed);
        let close_law = &law["comparatorClose"];
        let first = begun(&mut comparator);
        assert!(matches!(first, RetainedCloneStep::Progress(progress) if progress.copied_bytes == std::mem::size_of::<bool>()));
        assert!(comparator.close_started());
        let second = begun(&mut comparator);
        assert_eq!(second, RetainedCloneStep::Complete(Default::default()));
        assert_eq!(close_turn(&mut comparator, Default::default()).result.unwrap(), RetainedCloneStep::Progress(Default::default()));
        assert_eq!(usize::from(comparator.left.is_some())+usize::from(comparator.right.is_some()),close_law["bindings"].as_u64().unwrap()as usize);
        for _ in 0..100000{if comparator.terminal_is_empty(){break;}let grant=close_grant(&comparator,close_law["maximumItems"].as_u64().unwrap()as usize);let step=close_turn(&mut comparator,grant).result.unwrap();let progress=step.progress();assert!(progress.fits(grant));assert_eq!(left.borrow().get().to_string_owner(),format!("{prefix}{}",row["left"].as_str().unwrap()));}
        assert!(comparator.terminal_is_empty());
        assert!(driven_reply(physical, |effect, reply, recover| if recover { comparator.recover_compare_into(left.borrow(), right.borrow(), seven, BODY, effect, reply) } else { comparator.compare_into(left.borrow(), right.borrow(), seven, BODY, effect, reply) }).result.is_err());
        let mut cursor = PagedUtf8::<{usize::MAX}>::retained_clone_cursor();
        for _ in 0..100000 {
            let d=cursor.advance_demands(left.borrow(),bytes,crate::effect::CPU).unwrap();assert!(d.copy_bytes.checked_add(d.capacity_bytes).and_then(|n|n.checked_add(d.release_bytes)).unwrap()<=bytes);let grant=grant_of(d);
            let step = crate::effect::advance(&mut cursor, left.borrow(), bytes, crate::effect::CPU, grant).result.unwrap();
            let progress = step.progress();
            assert!(progress.copied_items <= 1 && progress.copied_bytes + progress.retained_capacity_bytes + progress.released_bytes <= bytes);
            if matches!(step, RetainedCloneStep::Complete(_)) { break; }
        }
        let output = take_value(&mut cursor).unwrap();
        assert_eq!(serde_json::to_value(&output).unwrap(), serde_json::to_value(left.borrow().get()).unwrap());
        close_cursor::<PagedUtf8<{usize::MAX}>>(&mut cursor, 1, bytes);
        retire_owner(output, 1, bytes);
        for at in law["cancelAt"].as_array().unwrap() {
            let mut cancelled = bounded_cursor();
            for _ in 0..at.as_u64().unwrap() { compared(&mut cancelled, left.borrow(), right.borrow(), seven, physical); }
            begun(&mut cancelled);
            close_comparator(&mut cancelled);
            assert!(cancelled.terminal_is_empty());
            let mut cursor = PagedUtf8::<{usize::MAX}>::retained_clone_cursor();
            for _ in 0..at.as_u64().unwrap() {
                let d=cursor.advance_demands(left.borrow(),bytes,crate::effect::CPU).unwrap();assert!(d.copy_bytes.checked_add(d.capacity_bytes).and_then(|n|n.checked_add(d.release_bytes)).unwrap()<=bytes);let grant=grant_of(d);
                crate::effect::advance(&mut cursor, left.borrow(), bytes, crate::effect::CPU, grant).result.unwrap();
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
    let mut bank=EffectBank::new(Wallet::new(crate::effect::STOCK,crate::effect::CPU));
    let(source,receipt)=match semio_framework_value_clone_source_testing::admit_owned(&mut bank,value,(),crate::effect::STOCK).result{Ok(admitted)=>admitted,Err(_)=>panic!("handwritten source birth")};
    assert!(receipt.retained_capacity_bytes<=demand.capacity_bytes);born+=receipt.retained_capacity_bytes;freed+=receipt.released_bytes;
    let mut source=FixtureSource::from_admitted(source,bank).unwrap_or_else(|_|panic!("handwritten source bank is busy"));
    let root=source.borrow().get() as *const T as usize;
    let(mut cursor,heap)=observe(T::retained_clone_cursor);assert_eq!(heap,(0,0));
    let(mut turns,mut output)=(0,None);
    for _ in 0..maximum{
        if cancel==Some(turns){break;}
        let(demand,heap)=observe(||cursor.advance_demands(source.borrow(),body,crate::effect::CPU).unwrap());assert_eq!(heap,(0,0));
        let funded=handwritten_quote(demand,body);
        for denied in handwritten_denials(funded,demand){
            let outcome=advanced::<T>(&mut cursor,source.borrow(),body,denied);assert_eq!(crate::effect::heap_of(&outcome),(0,0));assert_eq!(outcome.result.unwrap().progress(),Default::default());
            assert_eq!(cursor.advance_demands(source.borrow(),body,crate::effect::CPU).unwrap(),demand);assert_eq!(source.borrow().get() as *const T as usize,root);
        }
        let step=advanced::<T>(&mut cursor,source.borrow(),body,funded).result.unwrap();
        let receipt=step.progress();assert!(receipt.fits(funded));born+=receipt.retained_capacity_bytes;freed+=receipt.released_bytes;turns+=1;
        if matches!(step,RetainedCloneStep::Complete(_)){output=take_driven::<T>(&mut cursor);break;}
        assert_ne!(receipt,Default::default(),"{}",row["name"]);
    }
    assert!(output.is_some()||cancel==Some(turns),"{}",row["name"]);
    if let Some(value)=output.as_ref(){assert_eq!(serde_json::to_value(value).unwrap(),expected);}
    assert_eq!(serde_json::to_value(source.borrow().get()).unwrap(),expected);
    begun_cursor::<T>(&mut cursor).result.unwrap();
    for _ in 0..maximum{
        if cursor.terminal_is_empty(){break;}
        let(demand,heap)=observe(||super::super::retained_clone_close_demands::<T>(&cursor,body).unwrap());assert_eq!(heap,(0,0));
        let funded=handwritten_quote(demand,0);
        for denied in handwritten_denials(funded,demand){if semio_framework_value::retained_clone::retained_clone_grant_admits(denied,super::super::retained_clone_close_demands::<T>(&cursor,denied.maximum_copy_bytes).unwrap()){continue}
            let outcome=closed::<T>(&mut cursor,body,denied);assert_eq!(crate::effect::heap_of(&outcome),(0,0));assert_eq!(outcome.result.unwrap().progress(),Default::default());
            assert_eq!(super::super::retained_clone_close_demands::<T>(&cursor,body).unwrap(),demand);
        }
        assert_eq!(super::super::retained_clone_close_demands::<T>(&cursor,funded.maximum_copy_bytes).unwrap(),demand);
        let step=closed::<T>(&mut cursor,funded.maximum_copy_bytes,funded).result.unwrap();let receipt=step.progress();assert!(receipt.fits(funded));born+=receipt.retained_capacity_bytes;freed+=receipt.released_bytes;
    }
    assert!(cursor.terminal_is_empty());assert_eq!(observe(||drop(cursor)).1,(0,0));
    for _ in 0..maximum{
        if source.terminal_is_empty(){break;}
        let demand=source.next_close_demand(body).unwrap();
        let funded=handwritten_quote(demand,0);
        assert_eq!(source.next_close_demand(funded.maximum_copy_bytes).unwrap(),demand);
        match source.close_with(funded.maximum_copy_bytes,funded).unwrap(){FixtureCloseStep::Progress(receipt)=>{assert!(receipt.fits(funded));born+=receipt.retained_capacity_bytes;freed+=receipt.released_bytes;},FixtureCloseStep::Complete|FixtureCloseStep::Pending=>{},other=>panic!("handwritten source close refused: {other:?}")}
    }
    assert!(source.terminal_is_empty());let(bank,heap)=observe(||source.into_bank().ok().expect("terminal handwritten source"));assert_eq!(heap,(0,0));drop(bank);
    if let Some(value)=output{
        let(mut owner,heap)=observe(||ControlledRetirement::new(value).unwrap_or_else(|_|panic!("handwritten output retirement")));assert_eq!(heap,(0,0));
        for _ in 0..maximum{
            if owner.terminal_is_empty(){break;}
            let demand=owner.selected_demand(body).unwrap();
            let funded=handwritten_quote(demand,0);
            assert_eq!(owner.selected_demand(funded.maximum_copy_bytes).unwrap(),demand);
            let(step,_)=stepped(funded,|effect|owner.step(funded.maximum_copy_bytes,effect));let receipt=step.progress();assert!(receipt.fits(funded));born+=receipt.retained_capacity_bytes;freed+=receipt.released_bytes;
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