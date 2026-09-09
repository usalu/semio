use super::*;
use std::sync::Arc;

fn fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap()
}

fn limits() -> DslValueCloneLimits {
    DslValueCloneLimits { maximum_depth: 64, maximum_retained_bytes: 1_048_576 }
}

fn clone_value(value: &DslValue) -> (DslValue, DslValueCloneCheckpoint) {
    let root = Arc::new(value.clone());
    let mut cursor = DslValueCloneCursor::new(root.clone(), limits()).unwrap();
    let mut previous = DslValueCloneCheckpoint::default();
    for _ in 0..100_000 {
        let step = cursor.advance().unwrap();
        let checkpoint = cursor.checkpoint();
        assert!(checkpoint.completed_items - previous.completed_items <= 1);
        assert!(checkpoint.copied_bytes - previous.copied_bytes <= 256);
        assert!(checkpoint.retained_bytes <= limits().maximum_retained_bytes);
        previous = checkpoint;
        if matches!(step, DslValueCloneStep::Complete(_)) {
            let result = cursor.take_value().unwrap();
            assert!(Arc::ptr_eq(&root, &cursor.take_source().unwrap()));
            assert!(cursor.terminal_is_empty());
            return (result, checkpoint);
        }
        assert!(cursor.take_source().is_none());
    }
    panic!("value clone failed to complete within the fixture bound");
}

#[test]
fn shared_value_clone_matches_neutral_vectors_and_serde_json() {
    let vectors = fixture();
    for expected in vectors["values"].as_array().unwrap() {
        let source = DslValue::from(expected);
        let (actual, _) = clone_value(&source);
        assert_eq!(actual, source);
        assert_eq!(serde_json::to_value(&actual).unwrap(), *expected);
    }
    let unit = vectors["largeString"]["unit"].as_str().unwrap();
    let count = vectors["largeString"]["repeat"].as_u64().unwrap() as usize;
    let source = DslValue::String(unit.repeat(count));
    let (actual, checkpoint) = clone_value(&source);
    assert_eq!(actual, source);
    assert_eq!(checkpoint.copied_bytes, unit.len() * count);
    assert!(checkpoint.completed_items > 8);
    let source = DslValue::Object(vec![("z".into(), DslValue::uint(u64::MAX)), ("a".into(), DslValue::float(1.0)), ("z".into(), DslValue::int(i64::MIN))]);
    assert_eq!(clone_value(&source).0, source);
    println!("[DEBUG] shared value clone: 9 neutral values, large UTF-8 chunks, exact numeric variants and duplicate-key order agree");
}

#[test]
fn shared_value_clone_cancellation_returns_the_exact_source_after_bounded_cleanup() {
    let vectors = fixture();
    let root = Arc::new(DslValue::from(&vectors["values"][8]));
    for stop in vectors["cancellationAfterSteps"].as_array().unwrap() {
        let mut cursor = DslValueCloneCursor::new(root.clone(), limits()).unwrap();
        for _ in 0..stop.as_u64().unwrap() {
            if matches!(cursor.advance().unwrap(), DslValueCloneStep::Complete(_)) { break; }
        }
        cursor.cancel();
        assert!(cursor.take_value().is_none());
        assert!(cursor.take_source().is_none());
        for _ in 0..100_000 {
            if cursor.close_step() { break; }
        }
        assert!(Arc::ptr_eq(&root, &cursor.take_source().unwrap()));
        assert!(cursor.terminal_is_empty());
        assert_eq!(Arc::strong_count(&root), 1);
    }
    println!("[DEBUG] shared value clone: all 10 cancellation checkpoints retire partial output and return the exact source");
}

#[test]
fn shared_value_clone_rejects_capacity_and_depth_without_losing_the_source() {
    for vector in fixture()["limits"].as_array().unwrap() {
        let root = Arc::new(DslValue::from(&vector["value"]));
        let limits = DslValueCloneLimits { maximum_depth: vector["maximumDepth"].as_u64().unwrap() as usize, maximum_retained_bytes: vector["maximumRetainedBytes"].as_u64().unwrap() as usize };
        let mut cursor = DslValueCloneCursor::new(root.clone(), limits).unwrap();
        let mut error = None;
        for _ in 0..1000 {
            if let Err(reason) = cursor.advance() { error = Some(reason); break; }
        }
        assert_eq!(error, vector["error"].as_str());
        cursor.cancel();
        while !cursor.close_step() {}
        assert!(Arc::ptr_eq(&root, &cursor.take_source().unwrap()));
        assert!(cursor.terminal_is_empty());
    }
    println!("[DEBUG] shared value clone: capacity and depth rejection preserve exact source ownership");
}
