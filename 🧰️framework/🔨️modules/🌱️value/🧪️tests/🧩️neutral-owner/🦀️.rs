//! 🧩️ Portable value and retained-owner laws run with product packages physically absent.
use semio_framework_value::{DslValue, FromValue, ToValue, RetainedClone, RetireOwned};
use semio_framework_value::retained_clone::{RetainedClone as RetainedCloneTrait, RetainedCloneBorrowAuthority, RetainedCloneCursor, RetainedCloneGrant, RetainedCloneSource, RetainedCloneStep};
use serde::{Deserialize, Serialize};

fn admit_source<T: semio_framework_value::retirement::RetireOwned + Sync>(owner: T) -> RetainedCloneSource<T> {
    let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: RetainedCloneSource::<T>::constructor_copy_bytes(), maximum_capacity_bytes: RetainedCloneSource::<T>::owned_constructor_capacity_bytes::<()>(), maximum_depth: 1, ..Default::default() };
    RetainedCloneSource::admit_owned(owner, (), grant).map_err(|(error, _, _)| error).expect("original source admission").0
}

fn drain_source<T: semio_framework_value::retirement::RetireOwned + Sync>(mut source: RetainedCloneSource<T>) {
    for _ in 0..100000 {
        if source.terminal_is_empty() { return; }
        let copy = source.next_close_copy_byte_demand().unwrap();
        let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: copy, maximum_capacity_bytes: source.next_close_capacity_byte_demand(copy).unwrap(), maximum_release_bytes: source.next_close_release_byte_demand().unwrap(), maximum_depth: source.next_close_depth_demand().unwrap() };
        assert!(source.close_step(grant).unwrap().progress().fits(grant));
    }
    panic!("original source did not close");
}

fn admit_authority<A: semio_framework_value::retirement::RetireOwned + Sync>(owner: A) -> RetainedCloneBorrowAuthority<A> {
    let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: RetainedCloneBorrowAuthority::<A>::constructor_copy_bytes(), maximum_capacity_bytes: RetainedCloneBorrowAuthority::<A>::constructor_capacity_bytes(), maximum_depth: 1, ..Default::default() };
    RetainedCloneBorrowAuthority::admit(owner, grant).map_err(|(error, _)| error).expect("borrowed authority admission").0
}

fn drain_authority<A: semio_framework_value::retirement::RetireOwned + Sync>(mut authority: RetainedCloneBorrowAuthority<A>) {
    for _ in 0..100000 {
        if authority.terminal_is_empty() { return; }
        let copy = authority.next_close_copy_byte_demand().unwrap();
        let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: copy, maximum_capacity_bytes: authority.next_close_capacity_byte_demand(copy).unwrap(), maximum_release_bytes: authority.next_close_release_byte_demand().unwrap(), maximum_depth: authority.next_close_depth_demand().unwrap() };
        assert!(authority.close_step(grant).unwrap().progress().fits(grant));
    }
    panic!("borrowed authority did not close");
}

fn close_turn<T: RetainedCloneTrait>(cursor: &mut impl RetainedCloneCursor<T>, work: usize) -> RetainedCloneStep {
    let release = cursor.next_close_release_byte_demand().unwrap();
    let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: work, maximum_capacity_bytes: cursor.next_close_capacity_byte_demand(work).unwrap(), maximum_release_bytes: release, maximum_depth: 64 };
    let step = cursor.close_step(grant).unwrap();
    assert!(step.progress().fits(grant));
    step
}

fn retire<T: semio_framework_value::retirement::RetireOwned>(value: T, work: usize) {
    let mut owner = semio_framework_value::retirement::controlled::ControlledRetirement::new(value).map_err(|(error, _)| error).unwrap();
    for _ in 0..100000 {
        let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: work, maximum_capacity_bytes: owner.next_capacity_byte_demand(work).unwrap(), maximum_release_bytes: owner.next_release_byte_demand().unwrap(), maximum_depth: owner.next_depth_demand().unwrap() };
        let step = owner.step(grant).unwrap();
        assert!(step.progress().fits(grant));
        if matches!(step, RetainedCloneStep::Complete(_)) { break; }
    }
    assert!(owner.terminal_is_empty());
}

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
    let source = admit_source(tree);
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
    while !matches!(close_turn(&mut cursor, 4096), RetainedCloneStep::Complete(_)) {}
    assert!(cursor.terminal_is_empty());
    retire(output, 4096);
    drain_source(source);
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
    let authority = admit_authority(source);
    let mut cursor = PagedDocument::retained_clone_cursor();
    for turn in 0..10000 {
        let grant = match turn % 3 { 0 => RetainedCloneGrant::one_capacity_turn(4096, 64), 1 => RetainedCloneGrant::one_payload_turn(4096, 64), _ => RetainedCloneGrant::one_release_turn(4096, 64) };
        let step = cursor.advance(authority.borrow(0, |source| source), grant).unwrap();
        let progress = step.progress();
        assert!(progress.copied_items <= 1 && progress.copied_bytes + progress.retained_capacity_bytes + progress.released_bytes <= 4096);
        if matches!(step, RetainedCloneStep::Complete(_)) { break; }
    }
    let cloned = cursor.take().unwrap();
    assert_eq!(serde_json::to_value(&cloned).unwrap(), input);
    cursor.begin_close();
    while !matches!(close_turn(&mut cursor, 4096), RetainedCloneStep::Complete(_)) {}
    assert!(cursor.terminal_is_empty());
    for value in [copied, cloned] {
        retire(value, 4096);
    }
    drain_authority(authority);
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
            let source = admit_source(actual);
            let turn = &corpus["grant"];
            let grant = RetainedCloneGrant { maximum_items: turn["maximumItems"].as_u64().unwrap() as usize, maximum_copy_bytes: turn["maximumCopyBytes"].as_u64().unwrap() as usize + RetainedCloneBorrowAuthority::<()>::constructor_copy_bytes(), maximum_capacity_bytes: turn["maximumCapacityBytes"].as_u64().unwrap() as usize, maximum_depth: turn["maximumDepth"].as_u64().unwrap() as usize, maximum_release_bytes: turn["maximumReleaseBytes"].as_u64().unwrap() as usize };
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
                if matches!(close_turn(&mut cursor, 1024), RetainedCloneStep::Complete(_)) { break; }
            }
            assert!(cursor.terminal_is_empty());
            retire(copied, 1024);
            drain_source(source);
        }
    }
}

#[test]
fn borrowed_record_clone_matches_neutral_serde_and_exact_cancellation() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧩️neutral-owner/🔣️.json")).unwrap();
    let turn = &corpus["grant"];
    let grant = RetainedCloneGrant { maximum_items: turn["maximumItems"].as_u64().unwrap() as usize, maximum_copy_bytes: turn["maximumCopyBytes"].as_u64().unwrap() as usize + RetainedCloneBorrowAuthority::<()>::constructor_copy_bytes(), maximum_capacity_bytes: turn["maximumCapacityBytes"].as_u64().unwrap() as usize, maximum_depth: turn["maximumDepth"].as_u64().unwrap() as usize, maximum_release_bytes: turn["maximumReleaseBytes"].as_u64().unwrap() as usize };
    for row in corpus["vectors"].as_array().unwrap().iter().filter(|row| row["accepted"] == true) {
        let authority = admit_authority(serde_json::from_value::<Record>(row["input"].clone()).unwrap());
        let mut cursor = Record::retained_clone_cursor();
        loop { if matches!(cursor.advance(authority.borrow(0, |source| source), grant).unwrap(), RetainedCloneStep::Complete(_)) { break; } }
        let copied = cursor.take().unwrap();
        assert_eq!(serde_json::to_value(&copied).unwrap(), row["input"]);
        assert_eq!(serde_json::to_value(authority.borrow(0, |source| source).get()).unwrap() == row["input"], corpus["borrowAuthority"]["sourceUnchanged"].as_bool().unwrap());
        cursor.begin_close();
        while !matches!(close_turn(&mut cursor, 1024), RetainedCloneStep::Complete(_)) {}
        assert!(cursor.terminal_is_empty());
        retire(copied, 1024);
        for cancelled_at in corpus["borrowAuthority"]["cancelAt"].as_array().unwrap() {
            let mut cancelled = Record::retained_clone_cursor();
            for _ in 0..cancelled_at.as_u64().unwrap() { cancelled.advance(authority.borrow(0, |source| source), grant).unwrap(); }
            cancelled.begin_close();
            while !matches!(close_turn(&mut cancelled, 1024), RetainedCloneStep::Complete(_)) {}
            assert!(cancelled.terminal_is_empty());
            assert_eq!(serde_json::to_value(authority.borrow(0, |source| source).get()).unwrap(), row["input"]);
        }
        let changed = admit_authority(serde_json::from_value::<Record>(row["input"].clone()).unwrap());
        let mut cursor = Record::retained_clone_cursor();
        cursor.advance(authority.borrow(0, |source| source), grant).unwrap();
        assert_eq!(cursor.advance(changed.borrow(0, |source| source), grant).is_ok(), corpus["borrowAuthority"]["changedSourceAccepted"].as_bool().unwrap());
        cursor.begin_close();
        while !matches!(close_turn(&mut cursor, 1024), RetainedCloneStep::Complete(_)) {}
        assert!(cursor.terminal_is_empty());
        drain_authority(changed);
        drain_authority(authority);
    }
    eprintln!("[DEBUG] borrowed record neutral serde/source-address/cancel/terminal laws verified");
}
