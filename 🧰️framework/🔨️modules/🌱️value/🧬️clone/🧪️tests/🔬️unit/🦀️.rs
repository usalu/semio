use super::*;
use std::sync::Arc;

fn fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap()
}

fn limits() -> DslValueCloneLimits {
    DslValueCloneLimits { maximum_depth: 64, maximum_retained_bytes: 1_048_576 }
}

fn grant(size: usize) -> DslValueCloneGrant {
    DslValueCloneGrant { maximum_items: size, maximum_bytes: size }
}

fn observe_step(step: DslValueCloneStep, previous: DslValueCloneCheckpoint, grant: DslValueCloneGrant) -> (DslValueCloneCheckpoint, bool) {
    match step {
        DslValueCloneStep::Blocked(checkpoint) => {
            assert_eq!(checkpoint, previous);
            (checkpoint, false)
        }
        DslValueCloneStep::Progress { receipt, checkpoint } | DslValueCloneStep::Complete { receipt, checkpoint } => {
            assert!(receipt.structural_items <= grant.maximum_items.min(1));
            assert!(receipt.copied_bytes <= grant.maximum_bytes.min(DSL_VALUE_CLONE_CHUNK_BYTES));
            assert_eq!(checkpoint.completed_items - previous.completed_items, receipt.structural_items);
            assert_eq!(checkpoint.copied_bytes - previous.copied_bytes, receipt.copied_bytes);
            assert!(receipt.structural_items > 0 || receipt.copied_bytes > 0);
            let complete = matches!(step, DslValueCloneStep::Complete { .. });
            (checkpoint, complete)
        }
    }
}

fn close_and_return<R: DslValueSource>(cursor: &mut DslValueCloneCursor<R>) -> R {
    assert!(!cursor.terminal_is_empty());
    assert!(matches!(cursor.close_step(DslValueCloneGrant { maximum_items: 0, maximum_bytes: usize::MAX }), DslValueCloneCloseStep::Blocked(_)));
    loop {
        match cursor.close_step(DslValueCloneGrant { maximum_items: 1, maximum_bytes: 0 }) {
            DslValueCloneCloseStep::Blocked(_) => panic!("one structural item must admit bounded close progress"),
            DslValueCloneCloseStep::Progress { receipt, .. } => {
                assert_eq!(receipt.structural_items, 1);
                assert_eq!(receipt.copied_bytes, 0);
            }
            DslValueCloneCloseStep::Returned { source, receipt, .. } => {
                assert_eq!(receipt.structural_items, 1);
                assert_eq!(receipt.copied_bytes, 0);
                assert!(cursor.terminal_is_empty());
                assert!(matches!(cursor.close_step(DslValueCloneGrant { maximum_items: 1, maximum_bytes: 0 }), DslValueCloneCloseStep::Complete(_)));
                return source;
            }
            DslValueCloneCloseStep::Complete(_) => panic!("source must be returned exactly once"),
        }
    }
}

fn clone_value(value: &DslValue, size: usize) -> (DslValue, DslValueCloneCheckpoint) {
    let root = Arc::new(value.clone());
    let mut cursor = DslValueCloneCursor::new(root.clone(), limits()).unwrap();
    let mut previous = DslValueCloneCheckpoint::default();
    for _ in 0..100_000 {
        assert!(matches!(cursor.close_step(DslValueCloneGrant { maximum_items: 0, maximum_bytes: size }), DslValueCloneCloseStep::Blocked(checkpoint) if checkpoint == previous));
        let (checkpoint, complete) = observe_step(cursor.advance(grant(size)).unwrap(), previous, grant(size));
        assert!(checkpoint.retained_bytes <= limits().maximum_retained_bytes);
        previous = checkpoint;
        if complete {
            let result = cursor.take_value().unwrap();
            let returned = close_and_return(&mut cursor);
            assert!(Arc::ptr_eq(&root, &returned));
            assert!(cursor.terminal_is_empty());
            return (result, checkpoint);
        }
    }
    panic!("value clone failed to complete within the fixture bound");
}

#[test]
fn shared_value_clone_matches_neutral_vectors_and_serde_json() {
    let vectors = fixture();
    for expected in vectors["values"].as_array().unwrap() {
        let source = DslValue::from(expected);
        let (actual, _) = clone_value(&source, DSL_VALUE_CLONE_CHUNK_BYTES);
        assert_eq!(actual, source);
        assert_eq!(serde_json::to_value(&actual).unwrap(), *expected);
    }
    let unit = vectors["largeString"]["unit"].as_str().unwrap();
    let count = vectors["largeString"]["repeat"].as_u64().unwrap() as usize;
    let source = DslValue::String(unit.repeat(count));
    let (actual, checkpoint) = clone_value(&source, DSL_VALUE_CLONE_CHUNK_BYTES);
    assert_eq!(actual, source);
    assert_eq!(checkpoint.copied_bytes, unit.len() * count);
    assert_eq!(checkpoint.completed_items, 3);
    let source = DslValue::Object(vec![("z".into(), DslValue::uint(u64::MAX)), ("a".into(), DslValue::float(1.0)), ("z".into(), DslValue::int(i64::MIN))]);
    assert_eq!(clone_value(&source, DSL_VALUE_CLONE_CHUNK_BYTES).0, source);
    println!("[DEBUG] shared value clone: 9 neutral values, large UTF-8 chunks, exact numeric variants and duplicate-key order agree");
}

#[test]
fn shared_value_clone_grants_bound_utf8_progress_and_cancellation_returns_the_exact_source() {
    let vectors = fixture();
    let unit = vectors["largeString"]["unit"].as_str().unwrap();
    let count = vectors["largeString"]["repeat"].as_u64().unwrap() as usize;
    let value = DslValue::String(unit.repeat(count));
    let root = Arc::new(value.clone());
    let mut split = DslValueCloneCursor::new(root.clone(), limits()).unwrap();
    let item = DslValueCloneGrant { maximum_items: 1, maximum_bytes: 0 };
    let bytes = DslValueCloneGrant { maximum_items: 0, maximum_bytes: 1 };
    let (allocated, _) = observe_step(split.advance(item).unwrap(), DslValueCloneCheckpoint::default(), item);
    assert!(allocated.retained_bytes >= unit.len() * count);
    assert!(matches!(split.advance(item).unwrap(), DslValueCloneStep::Blocked(checkpoint) if checkpoint == allocated));
    let (copied, _) = observe_step(split.advance(bytes).unwrap(), allocated, bytes);
    assert_eq!(copied.copied_bytes, 1);
    assert_eq!(copied.retained_bytes, allocated.retained_bytes);
    split.cancel();
    let returned = close_and_return(&mut split);
    assert!(Arc::ptr_eq(&root, &returned));
    for size in vectors["grantSizes"].as_array().unwrap().iter().map(|value| value.as_u64().unwrap() as usize) {
        let root = Arc::new(value.clone());
        let mut blocked = DslValueCloneCursor::new(root.clone(), limits()).unwrap();
        assert_eq!(blocked.checkpoint(), DslValueCloneCheckpoint::default());
        assert!(matches!(blocked.advance(grant(0)).unwrap(), DslValueCloneStep::Blocked(checkpoint) if checkpoint == DslValueCloneCheckpoint::default()));
        assert_eq!(blocked.checkpoint(), DslValueCloneCheckpoint::default());
        blocked.cancel();
        let returned = close_and_return(&mut blocked);
        assert!(Arc::ptr_eq(&root, &returned));
        drop(returned);
        assert_eq!(Arc::strong_count(&root), 1);
        if size == 0 { continue; }
        let (actual, checkpoint) = clone_value(&value, size);
        assert_eq!(actual, value);
        assert_eq!(serde_json::to_value(&actual).unwrap(), serde_json::to_value(&value).unwrap());
        assert_eq!(checkpoint.copied_bytes, unit.len() * count);
        for stop in vectors["cancellationAfterSteps"].as_array().unwrap() {
            let root = Arc::new(value.clone());
            let mut cursor = DslValueCloneCursor::new(root.clone(), limits()).unwrap();
            let mut previous = cursor.checkpoint();
            for _ in 0..stop.as_u64().unwrap() {
                let (checkpoint, complete) = observe_step(cursor.advance(grant(size)).unwrap(), previous, grant(size));
                previous = checkpoint;
                if complete { break; }
            }
            cursor.cancel();
            assert!(cursor.take_value().is_none());
            let returned = close_and_return(&mut cursor);
            assert!(Arc::ptr_eq(&root, &returned));
            assert!(cursor.terminal_is_empty());
            drop(returned);
            assert_eq!(Arc::strong_count(&root), 1);
        }
    }
    println!("[DEBUG] shared value clone: 0/1/7/256 item-byte grants bound UTF-8 progress and every cancellation matrix position returns the exact source");
}

#[cfg(debug_assertions)]
#[test]
fn shared_value_clone_drop_rejects_live_recursive_ownership() {
    let root = Arc::new(DslValue::Array(vec![DslValue::String("owned".into())]));
    let cursor = DslValueCloneCursor::new(root.clone(), limits()).unwrap();
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| drop(cursor)));
    assert!(result.is_err());
    assert_eq!(Arc::strong_count(&root), 2);
    println!("[DEBUG] shared value clone: live cursor Drop rejects recursive owner destruction and explicit close remains mandatory");
}

#[test]
fn shared_value_clone_rejects_capacity_and_depth_without_losing_the_source() {
    for vector in fixture()["limits"].as_array().unwrap() {
        let root = Arc::new(DslValue::from(&vector["value"]));
        let limits = DslValueCloneLimits { maximum_depth: vector["maximumDepth"].as_u64().unwrap() as usize, maximum_retained_bytes: vector["maximumRetainedBytes"].as_u64().unwrap() as usize };
        let mut cursor = DslValueCloneCursor::new(root.clone(), limits).unwrap();
        let mut error = None;
        for _ in 0..1000 {
            if let Err(reason) = cursor.advance(grant(DSL_VALUE_CLONE_CHUNK_BYTES)) { error = Some(reason); break; }
        }
        assert_eq!(error, vector["error"].as_str());
        cursor.cancel();
        let returned = close_and_return(&mut cursor);
        assert!(Arc::ptr_eq(&root, &returned));
        assert!(cursor.terminal_is_empty());
    }
    println!("[DEBUG] shared value clone: capacity and depth rejection preserve exact source ownership");
}

#[test]
fn shared_value_clone_reservation_rejects_allocator_overcapacity_before_payload_admission() {
    for case in fixture()["reservationCases"].as_array().unwrap() {
        let requested = case["requested"].as_u64().unwrap() as usize;
        let allocated = case["allocated"].as_u64().unwrap() as usize;
        let budget = case["budget"].as_u64().unwrap() as usize;
        let mut called = false;
        let result = reserve_vec_with::<u8>(requested, budget, |value, actual_request| {
            called = true;
            assert_eq!(actual_request, requested);
            assert!(value.is_empty());
            value.try_reserve_exact(allocated)
        });
        assert_eq!(called, requested <= budget);
        assert_eq!(result.is_ok(), case["accepted"].as_bool().unwrap());
        if let Ok(value) = result { assert!(value.is_empty()); assert!(value.capacity() <= budget); }
    }
    println!("[DEBUG] shared value reservation rejects an injected overcapacity allocator before admitting payload storage");
}
