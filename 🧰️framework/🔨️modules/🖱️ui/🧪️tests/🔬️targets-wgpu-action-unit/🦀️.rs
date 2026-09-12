
use super::*;

fn publish(queue: &mut BoundedActionQueue, index: usize) {
    let mut reservation = queue.reserve("controller", "dispatch", 128).expect("reservation");
    let builder = reservation.builder();
    builder.begin_object(None).expect("root");
    builder.number(Some("index"), index as f64).expect("number");
    builder.string(Some("value"), "owned").expect("string");
    builder.end_container().expect("root end");
    reservation.publish().expect("publication");
}

#[test]
fn reservation_rejects_max_plus_one_before_allocating_action_storage() {
    let mut queue = BoundedActionQueue::default();
    assert!(matches!(queue.reserve("controller", "dispatch", ACTION_ITEM_BYTE_CAPACITY + 1), Err(BoundedActionFault::ByteCredits)));
    assert!(matches!(queue.reserve(&"x".repeat(ACTION_STRING_BYTE_CAPACITY + 1), "dispatch", ACTION_ITEM_BYTE_CAPACITY), Err(BoundedActionFault::StringCredits)));
    assert!(queue.is_empty());
}

#[test]
fn hostile_depth_poison_has_only_flat_inline_storage_to_release() {
    let mut queue = BoundedActionQueue::default();
    let mut reservation = queue.reserve("controller", "dispatch", ACTION_ITEM_BYTE_CAPACITY).expect("reservation");
    for _ in 0..ACTION_DEPTH_CAPACITY {
        reservation.builder().begin_array(None).expect("admitted depth");
    }
    assert_eq!(reservation.builder().begin_array(None), Err(BoundedActionFault::DepthCredits));
    assert_eq!(reservation.publish(), Err(BoundedActionFault::DepthCredits));
    assert!(queue.is_empty());
}

#[test]
fn joined_string_writes_directly_into_the_pre_admitted_flat_slab() {
    let mut queue = BoundedActionQueue::default();
    let credits = checked_action_string_bytes(&["controller", "dispatch", "id", "surface", "/", "object"]).unwrap();
    let mut reservation = queue.reserve("controller", "dispatch", credits).expect("reservation");
    reservation.builder().begin_object(None).unwrap();
    reservation.builder().string_joined(Some("id"), &["surface", "/", "object"]).unwrap();
    reservation.builder().end_container().unwrap();
    reservation.publish().unwrap();
    let descriptor = queue.pop_front().unwrap().into_descriptor().unwrap();
    assert_eq!(descriptor.args.as_ref().and_then(|args| args.get("id")).and_then(DslValue::as_str), Some("surface/object"));

    let mut oversized = queue.reserve("controller", "dispatch", ACTION_ITEM_BYTE_CAPACITY).unwrap();
    assert_eq!(oversized.builder().string_joined(None, &[&"x".repeat(ACTION_STRING_BYTE_CAPACITY), "x"]), Err(BoundedActionFault::StringCredits));
    assert_eq!(oversized.publish(), Err(BoundedActionFault::StringCredits));
}

#[test]
fn full_queue_fails_before_builder_and_retry_preserves_fifo() {
    let mut queue = BoundedActionQueue::default();
    for index in 0..ACTION_QUEUE_ITEM_CAPACITY {
        publish(&mut queue, index);
    }
    assert!(matches!(queue.reserve("controller", "retry", 128), Err(BoundedActionFault::ItemCredits)));
    let first = queue.pop_front().expect("first").into_descriptor().expect("descriptor");
    assert_eq!(first.args.as_ref().and_then(|args| args.get("index")).and_then(DslValue::as_f64), Some(0.0));
    publish(&mut queue, ACTION_QUEUE_ITEM_CAPACITY);
    let mut last = None;
    while let Some(action) = queue.pop_front() {
        last = Some(action.into_descriptor().expect("descriptor"));
    }
    assert_eq!(last.and_then(|action| action.args).as_ref().and_then(|args| args.get("index")).and_then(DslValue::as_f64), Some(ACTION_QUEUE_ITEM_CAPACITY as f64));
    assert_eq!(queue.bytes(), 0);
}

#[test]
fn detached_claim_reserves_exact_aggregate_credits_and_rejects_stale_epoch() {
    let mut queue = BoundedActionQueue::default();
    assert_eq!(queue.claim(ACTION_ITEM_BYTE_CAPACITY + 1), Err(BoundedActionFault::ByteCredits));
    let claim = queue.claim(ACTION_ITEM_BYTE_CAPACITY).expect("exact detached claim");
    assert_eq!(queue.claimed_items(), 1);
    assert_eq!(queue.claimed_bytes(), ACTION_ITEM_BYTE_CAPACITY);
    assert!(queue.reserve("controller", "dispatch", ACTION_QUEUE_BYTE_CAPACITY).is_err());
    queue.release_claim(claim).expect("claim release");
    let replacement = queue.claim(128).expect("reused slot with new epoch");
    assert_eq!(queue.release_claim(claim), Err(BoundedActionFault::Structure));

    let mut reservation = queue.reserve_claimed(replacement, "controller", "dispatch").expect("claimed builder");
    reservation.builder().begin_object(None).unwrap();
    reservation.builder().string(Some("value"), "owned").unwrap();
    reservation.builder().end_container().unwrap();
    reservation.publish().expect("claimed publication");
    assert_eq!(queue.claimed_items(), 0);
    assert_eq!(queue.len(), 1);
    assert_eq!(queue.pop_front().unwrap().into_descriptor().unwrap().action, "dispatch");
    assert!(queue.is_empty());
}

#[test]
fn detached_claim_close_releases_one_fixed_owner_per_grant() {
    let mut queue = BoundedActionQueue::default();
    let _first = queue.claim(128).unwrap();
    let _second = queue.claim(256).unwrap();
    assert!(!queue.close_claim_step());
    assert_eq!(queue.claimed_items(), 1);
    assert!(!queue.close_claim_step());
    assert!(queue.close_claim_step());
    assert!(queue.is_empty());
}

#[test]
fn claimed_draft_builds_incrementally_and_publishes_only_complete_owner() {
    let mut queue = BoundedActionQueue::default();
    let claim = queue.claim(256).expect("draft claim");
    let mut draft = queue.draft_claimed(claim, "controller", "dispatch").expect("detached draft");
    draft.builder().begin_object(None).unwrap();
    draft.builder().begin_array(Some("values")).unwrap();
    draft.builder().number(None, 1.0).unwrap();
    draft.builder().number(None, 2.0).unwrap();
    draft.builder().end_container().unwrap();
    draft.builder().end_container().unwrap();
    let prepared = draft.finish().expect("complete flat owner");
    assert_eq!(queue.len(), 0);
    assert_eq!(queue.bytes(), 0);
    assert!(!queue.is_empty());
    assert_eq!(queue.claimed_items(), 1);
    queue.publish_prepared_claimed(prepared).expect("claimed publication");
    assert_eq!(queue.claimed_items(), 0);
    let action = queue.pop_front().expect("published owner").into_descriptor().expect("descriptor");
    assert!(matches!(action.args.as_ref().and_then(|args| args.get("values")), Some(DslValue::Array(values)) if values.len() == 2));

    let stale = queue.claim(64).expect("stale claim");
    let draft = queue.draft_claimed(stale, "controller", "stale").expect("stale draft");
    queue.release_claim(stale).expect("cancel claim");
    let prepared = draft.finish().expect("shallow stale owner");
    assert_eq!(queue.publish_prepared_claimed(prepared), Err(BoundedActionFault::Structure));
    assert!(queue.is_empty());
}

#[test]
fn detached_claim_batch_reserves_and_publishes_all_pages_atomically_in_fifo_order() {
    let mut queue = BoundedActionQueue::default();
    let claims = queue.claim_batch(&[128, 128, 128]).expect("three exact page claims");
    assert_eq!(claims.len(), 3);
    assert_eq!(queue.claimed_items(), 3);
    let mut batch = PreparedClaimedActionBatch::new(claims);
    for index in 0..3 {
        let claim = batch.claim(index).expect("page claim");
        let mut draft = queue.draft_claimed(claim, "controller", ["first", "second", "third"][index]).expect("page draft");
        draft.builder().begin_object(None).unwrap();
        draft.builder().number(Some("index"), index as f64).unwrap();
        draft.builder().end_container().unwrap();
        batch.push(draft.finish().unwrap()).expect("ordered page");
    }
    assert_eq!(queue.len(), 0);
    assert_eq!(queue.bytes(), 0);
    assert!(!queue.is_empty());
    queue.publish_prepared_claimed_batch(batch).expect("atomic page publication");
    assert_eq!(queue.claimed_items(), 0);
    assert_eq!(queue.pop_front().unwrap().into_descriptor().unwrap().action, "first");
    assert_eq!(queue.pop_front().unwrap().into_descriptor().unwrap().action, "second");
    assert_eq!(queue.pop_front().unwrap().into_descriptor().unwrap().action, "third");
}

#[test]
fn detached_claim_batch_rejects_capacity_plus_one_before_ownership_and_closes_one_claim_at_a_time() {
    let mut queue = BoundedActionQueue::default();
    assert!(matches!(queue.claim_batch(&[1; ACTION_CLAIM_BATCH_CAPACITY + 1]), Err(BoundedActionFault::ItemCredits)));
    assert_eq!(queue.claimed_items(), 0);
    let claims = queue.claim_batch(&[32; ACTION_CLAIM_BATCH_CAPACITY]).expect("max fixed batch");
    let mut batch = PreparedClaimedActionBatch::new(claims);
    let mut turns = 0;
    while let Some(claim) = batch.take_last_claim() {
        queue.release_claim(claim).expect("cursorized batch claim release");
        turns += 1;
    }
    assert_eq!(turns, ACTION_CLAIM_BATCH_CAPACITY);
    assert_eq!(queue.claimed_items(), 0);
    assert!(queue.is_empty());
}

#[test]
fn batch_reservation_is_atomic_and_preserves_order() {
    let mut queue = BoundedActionQueue::default();
    let mut batch = queue.reserve_batch(2, 256).expect("batch");
    batch.action("controller", "first", 128, |_| Ok(())).expect("first");
    batch.action("controller", "second", 128, |_| Ok(())).expect("second");
    batch.publish().expect("publish");
    assert_eq!(queue.pop_front().expect("first").into_descriptor().expect("first descriptor").action, "first");
    assert_eq!(queue.pop_front().expect("second").into_descriptor().expect("second descriptor").action, "second");
}

#[test]
fn incomplete_or_over_credit_batch_publishes_nothing() {
    let mut queue = BoundedActionQueue::default();
    let mut batch = queue.reserve_batch(2, 128).expect("batch");
    batch.action("controller", "first", 64, |_| Ok(())).expect("first");
    assert_eq!(batch.publish(), Err(BoundedActionFault::ItemCredits));
    assert!(queue.is_empty());
    assert!(matches!(queue.reserve_batch(ACTION_BATCH_ITEM_CAPACITY + 1, 1), Err(BoundedActionFault::ItemCredits)));
}

#[test]
fn semantic_commit_runs_only_after_the_flat_owner_is_complete() {
    let mut queue = BoundedActionQueue::default();
    let mut committed = false;
    let mut reservation = queue.reserve("controller", "dispatch", 32).unwrap();
    reservation.builder().begin_object(None).unwrap();
    assert_eq!(reservation.publish_with(|| committed = true), Err(BoundedActionFault::Structure));
    assert!(!committed);
    assert!(queue.is_empty());

    let mut reservation = queue.reserve("controller", "dispatch", 32).unwrap();
    reservation.builder().begin_object(None).unwrap();
    reservation.builder().end_container().unwrap();
    reservation.publish_with(|| committed = true).unwrap();
    assert!(committed);
    assert_eq!(queue.len(), 1);
}

#[test]
fn rejected_revision_commit_publishes_no_flat_owner() {
    let mut queue = BoundedActionQueue::default();
    let mut batch = queue.reserve_batch(1, 32).unwrap();
    batch.action("controller", "dispatch", 32, |_| Ok(())).unwrap();
    assert_eq!(batch.publish_with_checked(|| false), Err(BoundedActionFault::Structure));
    assert!(queue.is_empty());

    let mut batch = queue.reserve_batch(2, 64).unwrap();
    batch.action("controller", "dispatch", 32, |_| Ok(())).unwrap();
    assert_eq!(batch.publish_partial_with_checked(|| false), Err(BoundedActionFault::Structure));
    assert!(queue.is_empty());
}

#[test]
fn close_releases_one_flat_inline_owner_per_grant() {
    let mut queue = BoundedActionQueue::default();
    publish(&mut queue, 1);
    publish(&mut queue, 2);
    drop(queue.pop_back().expect("one owner"));
    assert_eq!(queue.len(), 1);
    drop(queue.pop_back().expect("one owner"));
    assert!(queue.is_empty());
}

#[test]
fn production_boundary_has_no_forget_or_background_drop_escape() {
    const SOURCE: &str = include_str!("../../🎯️targets/🧊️wgpu/🎬️action/🦀️.rs");
    assert!(!SOURCE.contains(concat!("mem::", "forget")));
    assert!(!SOURCE.contains(concat!("thread::", "spawn")));
    assert!(!SOURCE.contains(concat!("impl Clone", " for BoundedAction")));
    assert!(!SOURCE.contains(concat!("#[derive(Clone, Debug)]", "\npub struct BoundedAction")));
}
