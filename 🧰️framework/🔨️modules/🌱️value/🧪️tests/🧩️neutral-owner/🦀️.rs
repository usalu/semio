//! 🧩️ Portable value and retained-owner laws run with product packages physically absent.
use semio_framework_value::{DslValue, FromValue, ToValue, RetainedClone, RetireOwned, SnapshotRetirementStep};
use semio_framework_value::retained_clone::{RetainedClone as RetainedCloneTrait, RetainedCloneBorrowAuthority, RetainedCloneCursor, RetainedCloneGrant, RetainedCloneSource, RetainedCloneStep};
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

#[derive(Debug, PartialEq, Serialize, Deserialize, FromValue, ToValue, RetainedClone, RetireOwned, semio_framework_dsl_record_derive::DslRecord)]
struct PagedRecord {
    label: semio_framework_value::paged::PagedUtf8<{usize::MAX}>,
    count: u64,
    choice: Option<semio_framework_value::paged::PagedUtf8<{usize::MAX}>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, FromValue, ToValue, RetainedClone, RetireOwned, semio_framework_dsl_record_derive::DslRecord)]
struct PagedDocument {
    title: semio_framework_value::paged::PagedUtf8<{usize::MAX}>,
    #[dsl(table)]
    rows: semio_framework_value::list::PagedList<PagedRecord, {usize::MAX}>,
    objects: semio_framework_value::paged::PagedMap<PagedRecord, {usize::MAX}>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, RetainedClone, RetireOwned)]
enum PagedTree {
    Branch(PagedBranch),
    Leaf,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, RetainedClone, RetireOwned)]
struct PagedBranch {
    label: semio_framework_value::paged::PagedUtf8<{usize::MAX}>,
    children: semio_framework_value::list::PagedList<PagedTree, {usize::MAX}>,
}

#[test]
fn paged_native_recursive_clone_uses_admitted_lazy_cursor_owners() {
    let law: serde_json::Value = serde_json::from_str(include_str!("../../📦️paged/🧫️fixtures/🎮️native-owner/🔣️.json")).unwrap();
    let depth = law["treeDepth"].as_u64().unwrap() as usize;
    assert!(std::mem::size_of::<<PagedTree as RetainedCloneTrait>::Cursor>() <= law["maximumCursorBytes"].as_u64().unwrap() as usize);
    let mut tree = PagedTree::Leaf;
    for _ in 0..depth { tree = PagedTree::Branch(PagedBranch { label: "Grundstück🧬".into(), children: [tree].into_iter().collect() }); }
    let oracle = serde_json::to_value(&tree).unwrap();
    let source = RetainedCloneSource::from_authority(Arc::new(tree), ());
    let mut cursor = PagedTree::retained_clone_cursor();
    for turn in 0..100000 {
        let grant = match turn % 3 { 0 => RetainedCloneGrant::one_capacity_turn(4096, 64), 1 => RetainedCloneGrant::one_payload_turn(4096, 64), _ => RetainedCloneGrant::one_release_turn(4096, 64) };
        let step = cursor.advance(source.borrow(), grant).unwrap();
        let progress = step.progress();
        assert!(progress.copied_items <= 1 && progress.copied_bytes + progress.retained_capacity_bytes + progress.released_bytes <= 4096);
        if matches!(step, RetainedCloneStep::Complete(_)) { break; }
    }
    let output = cursor.take().unwrap();
    assert_eq!(serde_json::to_value(&output).unwrap(), oracle);
    cursor.begin_close();
    while cursor.close_step(1, 4096).unwrap() != SnapshotRetirementStep::Complete {}
    assert!(cursor.terminal_is_empty());
    let mut owner = semio_framework_value::retirement::owned_retirement(output);
    while owner.close_step(1, 4096).unwrap() != SnapshotRetirementStep::Complete {}
    assert!(owner.terminal_is_empty());
    eprintln!("[DEBUG] paged native recursive tree depth={depth} lazy cursor grant4096 and terminal closure verified");
}

#[test]
fn paged_native_record_derives_preserve_neutral_serde_and_dsl_shapes() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../📦️paged/🧫️fixtures/🎮️native-owner/🔣️.json")).unwrap();
    let title = corpus["prefix"].as_str().unwrap().repeat(corpus["prefixRepeat"].as_u64().unwrap() as usize);
    let input = serde_json::json!({"title":title,"rows":[{"label":"Grundstück🧬","count":7,"choice":null},{"label":"Aé","count":9,"choice":"Änderung"}],"objects":{title.clone():{"label":"Grundstück🧬","count":7,"choice":null}}});
    let source: PagedDocument = serde_json::from_value(input.clone()).unwrap();
    assert_eq!(serde_json::to_value(&source).unwrap(), input);
    assert_eq!(serde_json::Value::from(source.to_value()), input);
    let record = source.__dsl_to_record();
    let copied = PagedDocument::__dsl_from_record(&record).unwrap();
    assert_eq!(copied, source);
    assert_eq!(serde_json::to_value(&copied).unwrap(), input);
    let authority = RetainedCloneBorrowAuthority::new(());
    let mut cursor = PagedDocument::retained_clone_cursor();
    for turn in 0..10000 {
        let grant = match turn % 3 { 0 => RetainedCloneGrant::one_capacity_turn(4096, 64), 1 => RetainedCloneGrant::one_payload_turn(4096, 64), _ => RetainedCloneGrant::one_release_turn(4096, 64) };
        let step = cursor.advance(authority.borrow(&source), grant).unwrap();
        let progress = step.progress();
        assert!(progress.copied_items <= 1 && progress.copied_bytes + progress.retained_capacity_bytes + progress.released_bytes <= 4096);
        if matches!(step, RetainedCloneStep::Complete(_)) { break; }
    }
    let cloned = cursor.take().unwrap();
    assert_eq!(serde_json::to_value(&cloned).unwrap(), input);
    cursor.begin_close();
    while cursor.close_step(1, 4096).unwrap() != SnapshotRetirementStep::Complete {}
    assert!(cursor.terminal_is_empty());
    for value in [source, copied, cloned] {
        let mut owner = semio_framework_value::retirement::owned_retirement(value);
        while owner.close_step(1, 4096).unwrap() != SnapshotRetirementStep::Complete {}
        assert!(owner.terminal_is_empty());
    }
    eprintln!("[DEBUG] paged native record derive serde/DSL/clone/terminal law verified");
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
            let grant = RetainedCloneGrant { maximum_items: turn["maximumItems"].as_u64().unwrap() as usize, maximum_copy_bytes: turn["maximumCopyBytes"].as_u64().unwrap() as usize, maximum_capacity_bytes: turn["maximumCapacityBytes"].as_u64().unwrap() as usize, maximum_depth: turn["maximumDepth"].as_u64().unwrap() as usize, maximum_release_bytes: turn["maximumReleaseBytes"].as_u64().unwrap() as usize };
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

#[test]
fn borrowed_record_clone_matches_neutral_serde_and_exact_cancellation() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧩️neutral-owner/🔣️.json")).unwrap();
    let turn = &corpus["grant"];
    let grant = RetainedCloneGrant { maximum_items: turn["maximumItems"].as_u64().unwrap() as usize, maximum_copy_bytes: turn["maximumCopyBytes"].as_u64().unwrap() as usize, maximum_capacity_bytes: turn["maximumCapacityBytes"].as_u64().unwrap() as usize, maximum_depth: turn["maximumDepth"].as_u64().unwrap() as usize, maximum_release_bytes: turn["maximumReleaseBytes"].as_u64().unwrap() as usize };
    for row in corpus["vectors"].as_array().unwrap().iter().filter(|row| row["accepted"] == true) {
        let source: Record = serde_json::from_value(row["input"].clone()).unwrap();
        let authority = RetainedCloneBorrowAuthority::new((7u64, [9u8; 32]));
        let mut cursor = Record::retained_clone_cursor();
        loop { if matches!(cursor.advance(authority.borrow(&source), grant).unwrap(), RetainedCloneStep::Complete(_)) { break; } }
        let copied = cursor.take().unwrap();
        assert_eq!(serde_json::to_value(&copied).unwrap(), row["input"]);
        assert_eq!(serde_json::to_value(&source).unwrap() == row["input"], corpus["borrowAuthority"]["sourceUnchanged"].as_bool().unwrap());
        cursor.begin_close();
        while cursor.close_step(1, 1024).unwrap() != SnapshotRetirementStep::Complete {}
        assert!(cursor.terminal_is_empty());
        let mut retirement = semio_framework_value::retirement::owned_retirement(copied);
        while retirement.close_step(1, 1024).unwrap() != SnapshotRetirementStep::Complete {}
        for cancelled_at in corpus["borrowAuthority"]["cancelAt"].as_array().unwrap() {
            let mut cancelled = Record::retained_clone_cursor();
            for _ in 0..cancelled_at.as_u64().unwrap() { cancelled.advance(authority.borrow(&source), grant).unwrap(); }
            cancelled.begin_close();
            while cancelled.close_step(1, 1024).unwrap() != SnapshotRetirementStep::Complete {}
            assert!(cancelled.terminal_is_empty());
            assert_eq!(serde_json::to_value(&source).unwrap(), row["input"]);
        }
        let changed: Record = serde_json::from_value(row["input"].clone()).unwrap();
        let mut cursor = Record::retained_clone_cursor();
        cursor.advance(authority.borrow(&source), grant).unwrap();
        assert_eq!(cursor.advance(authority.borrow(&changed), grant).is_ok(), corpus["borrowAuthority"]["changedSourceAccepted"].as_bool().unwrap());
        cursor.begin_close();
        while cursor.close_step(1, 1024).unwrap() != SnapshotRetirementStep::Complete {}
        assert!(cursor.terminal_is_empty());
    }
    eprintln!("[DEBUG] borrowed record neutral serde/source-address/cancel/terminal laws verified");
}
