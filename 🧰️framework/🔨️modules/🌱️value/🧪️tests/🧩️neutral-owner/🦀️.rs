//! 🧩️ Portable value and retained-owner laws run with product packages physically absent.
use semio_framework_value::{DslValue, FromValue, ToValue, RetainedClone, RetireOwned, SnapshotRetirementStep};
use semio_framework_value::retained_clone::{RetainedClone as RetainedCloneTrait, RetainedCloneCursor, RetainedCloneGrant, RetainedCloneSource, RetainedCloneStep};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, PartialEq, Serialize, Deserialize, FromValue, ToValue, RetainedClone, RetireOwned)]
#[serde(deny_unknown_fields)]
#[value(deny_unknown_fields)]
struct Record {
    label: String,
    count: u64,
    choice: Option<String>,
}

#[test]
fn neutral_record_vectors_match_independent_serde() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧩️neutral-owner/🔣️.json")).unwrap();
    for vector in corpus["vectors"].as_array().unwrap() {
        let input = vector["input"].clone();
        let oracle = serde_json::from_value::<Record>(input.clone());
        let actual = Record::from_value(DslValue::from(&input));
        assert_eq!(actual.is_ok(), vector["accepted"].as_bool().unwrap(), "{}", vector["id"]);
        assert_eq!(actual.is_ok(), oracle.is_ok(), "{}", vector["id"]);
        if let (Ok(actual), Ok(oracle)) = (actual, oracle) {
            assert_eq!(actual, oracle);
            assert_eq!(serde_json::Value::from(actual.to_value()), serde_json::to_value(&oracle).unwrap());
            let source = RetainedCloneSource::from_authority(Arc::new(actual), ());
            let turn = &corpus["grant"];
            let grant = RetainedCloneGrant { maximum_items: turn["maximumItems"].as_u64().unwrap() as usize, maximum_copy_bytes: turn["maximumCopyBytes"].as_u64().unwrap() as usize, maximum_capacity_bytes: turn["maximumCapacityBytes"].as_u64().unwrap() as usize, maximum_depth: turn["maximumDepth"].as_u64().unwrap() as usize };
            let mut cursor = Record::retained_clone_cursor();
            let mut turns = 0;
            loop {
                turns += 1;
                assert!(turns < 10000);
                if matches!(cursor.advance(source.borrow(), grant).unwrap(), RetainedCloneStep::Complete(_)) { break; }
            }
            assert!(turns > 1);
            let copied = cursor.take().unwrap();
            assert_eq!(serde_json::to_value(&copied).unwrap(), serde_json::to_value(&oracle).unwrap());
            assert!(cursor.begin_close());
            for _ in 0..10000 {
                if cursor.close_step(1, 1024).unwrap() == SnapshotRetirementStep::Complete { break; }
            }
            assert!(cursor.terminal_is_empty());
            let mut retirement = semio_framework_value::retirement::owned_retirement(copied);
            for _ in 0..10000 {
                if retirement.close_step(1, 1024).unwrap() == SnapshotRetirementStep::Complete { break; }
            }
            assert!(retirement.terminal_is_empty());
        }
    }
}
