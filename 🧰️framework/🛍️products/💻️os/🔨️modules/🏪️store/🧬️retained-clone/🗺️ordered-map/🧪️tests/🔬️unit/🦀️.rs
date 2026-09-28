use super::*;
use crate::os_store::retained_clone::RetainedCloneSource;
use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Fixture {
    page_capacity: usize,
    entry_count: usize,
    key_prefix: String,
    value_prefix: String,
    long_key_byte_length: usize,
    comparison_grant: ComparisonGrant,
    progress_channels: ProgressChannels,
    repeated_growth: RepeatedGrowth,
    immutable_lookup: ImmutableLookup,
    operations: Vec<Operation>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct RepeatedGrowth {
    entry_count: usize,
    insertions: usize,
    key_prefix: String,
    value_prefix: String,
    expected_entry_count: usize,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ImmutableLookup {
    captured_target: String,
    external_after_capture: String,
    expected_ordinal: usize,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ComparisonGrant {
    maximum_items: usize,
    maximum_bytes: usize,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ProgressChannels {
    comparison_only: bool,
    capacity_only: bool,
    minimum_moved_items: usize,
}

#[derive(Deserialize)]
struct Operation {
    kind: String,
    key: String,
    value: Option<String>,
    expected: Expected,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Expected {
    found: bool,
    ordinal: usize,
    entry_count: usize,
}

fn fixture() -> Fixture {
    serde_json::from_str(include_str!("../../🧫️fixtures/📦️paging/🔣️.json")).expect("retained ordered-map fixture")
}

fn oracle(fixture: &Fixture) -> BTreeMap<String, String> {
    (0..fixture.entry_count).map(|ordinal| (format!("{}{:04}", fixture.key_prefix, ordinal), format!("{}{}", fixture.value_prefix, ordinal))).collect()
}

fn retained_map(source: &RetainedOrderedMap<String, String>) -> RetainedOrderedMap<String, String> {
    let source = RetainedCloneSource::from_owner(source.clone());
    let grant = RetainedCloneGrant { maximum_items: 2, maximum_copy_bytes: 7, maximum_capacity_bytes: 65_536, maximum_depth: 64 };
    let mut cursor = RetainedOrderedMap::<String, String>::retained_clone_cursor();
    let output = loop {
        let step = cursor.advance(source.borrow(), grant).expect("retained ordered-map clone");
        assert!(step.progress().fits(grant));
        if matches!(step, RetainedCloneStep::Complete(_)) {
            break cursor.take().expect("retained ordered-map output");
        }
    };
    cursor.begin_close();
    while !cursor.terminal_is_empty() {
        let step = cursor.close_step(1, 7).expect("retained ordered-map cursor close");
        if step == SnapshotRetirementStep::Complete {
            assert!(cursor.terminal_is_empty());
        }
    }
    output
}

fn lookup(map: &RetainedOrderedMap<String, String>, key: &String, fixture: &Fixture) -> RetainedOrderedMapLookup {
    let map = RetainedCloneSource::from_owner(map.clone());
    let key = RetainedCloneSource::from_owner(key.clone());
    let grant = BoundedOrdGrant { maximum_items: fixture.comparison_grant.maximum_items, maximum_bytes: fixture.comparison_grant.maximum_bytes };
    let mut cursor = RetainedOrderedMapLookupCursor::default();
    loop {
        let (result, progress) = cursor.advance(map.borrow(), key.borrow(), grant).expect("retained ordered-map lookup");
        assert!(progress.fits(grant));
        if let Some(result) = result {
            return result;
        }
    }
}

#[test]
fn fixed_page_clone_matches_btree_and_serde_oracles() {
    let fixture = fixture();
    assert_eq!(fixture.page_capacity, RETAINED_ORDERED_MAP_PAGE_CAPACITY);
    let oracle = oracle(&fixture);
    let source = RetainedOrderedMap::from_sorted_entries_for_test(oracle.clone().into_iter().collect()).expect("ordered source");
    let copied = retained_map(&source);
    assert_eq!(copied.iter().map(|(key, value)| (key.clone(), value.clone())).collect::<BTreeMap<_, _>>(), oracle);
    assert_eq!(serde_json::to_value(&copied).expect("retained ordered-map JSON"), serde_json::to_value(&oracle).expect("BTreeMap JSON"));
    assert_eq!(copied.page_count(), fixture.entry_count.div_ceil(RETAINED_ORDERED_MAP_PAGE_CAPACITY));
}

#[test]
fn bounded_lookup_insert_and_duplicate_refusal_match_btree_oracle() {
    let fixture = fixture();
    let mut oracle = oracle(&fixture);
    let source = RetainedOrderedMap::from_sorted_entries_for_test(oracle.clone().into_iter().collect()).expect("ordered source");
    let mut map = retained_map(&source);
    for operation in &fixture.operations {
        let before = lookup(&map, &operation.key, &fixture);
        match operation.kind.as_str() {
            "lookup" => {
                assert_eq!(matches!(before, RetainedOrderedMapLookup::Found(_)), operation.expected.found);
                assert_eq!(
                    match before {
                        RetainedOrderedMapLookup::Found(ordinal) | RetainedOrderedMapLookup::Missing(ordinal) => ordinal,
                    },
                    operation.expected.ordinal
                );
            }
            "insert" => {
                assert!(matches!(before, RetainedOrderedMapLookup::Missing(ordinal) if ordinal == operation.expected.ordinal));
                let value = operation.value.clone().expect("insert value");
                let mut cursor = RetainedOrderedMapInsertCursor::new(map, operation.key.clone(), value.clone());
                let grant = RetainedOrderedMapInsertGrant { comparison: BoundedOrdGrant { maximum_items: 1, maximum_bytes: 7 }, maximum_moved_items: 32, maximum_moved_bytes: 4_096, maximum_capacity_bytes: 65_536 };
                let ordinal = loop {
                    match cursor.advance(grant).expect("bounded insertion") {
                        RetainedOrderedMapInsertStep::Progress(progress) => assert!(progress.fits(grant)),
                        RetainedOrderedMapInsertStep::Complete { ordinal, progress } => {
                            assert!(progress.fits(grant));
                            break ordinal;
                        }
                    }
                };
                map = cursor.take().expect("bounded insertion output");
                assert_eq!(ordinal, operation.expected.ordinal);
                oracle.insert(operation.key.clone(), value);
                assert!(cursor.begin_close());
                assert!(cursor.terminal_is_empty());
            }
            "duplicate" => {
                assert!(matches!(before, RetainedOrderedMapLookup::Found(ordinal) if ordinal == operation.expected.ordinal));
                assert!(operation.expected.found);
                let before_json = serde_json::to_value(&map).expect("map before duplicate");
                let mut cursor = RetainedOrderedMapInsertCursor::new(map, operation.key.clone(), operation.value.clone().expect("duplicate value"));
                let grant = RetainedOrderedMapInsertGrant { comparison: BoundedOrdGrant { maximum_items: 1, maximum_bytes: 7 }, maximum_moved_items: 1, maximum_moved_bytes: 7, maximum_capacity_bytes: 65_536 };
                let error = loop {
                    match cursor.advance(grant) {
                        Ok(RetainedOrderedMapInsertStep::Progress(progress)) => assert!(progress.fits(grant)),
                        Ok(RetainedOrderedMapInsertStep::Complete { .. }) => panic!("duplicate insertion completed"),
                        Err(error) => break error,
                    }
                };
                map = cursor.take_refused_workspace().expect("duplicate refusal workspace");
                assert!(error.contains("duplicate"));
                assert_eq!(serde_json::to_value(&map).expect("map after duplicate"), before_json);
                assert!(cursor.begin_close());
                while !cursor.terminal_is_empty() {
                    let step = cursor.close_step(1, 7).expect("duplicate cursor close");
                    if step == SnapshotRetirementStep::Complete {
                        assert!(cursor.terminal_is_empty());
                    }
                }
            }
            other => panic!("unexpected fixture operation {other}"),
        }
        assert_eq!(map.len(), operation.expected.entry_count);
        assert_eq!(map.iter().map(|(key, value)| (key.clone(), value.clone())).collect::<BTreeMap<_, _>>(), oracle);
    }
}

#[test]
fn long_string_comparison_is_byte_paged_and_shape_pinned() {
    let fixture = fixture();
    let left = "k".repeat(fixture.long_key_byte_length);
    let right = format!("{left}z");
    let grant = BoundedOrdGrant { maximum_items: fixture.comparison_grant.maximum_items, maximum_bytes: fixture.comparison_grant.maximum_bytes };
    let mut cursor = String::bounded_ord_cursor();
    let left_source = RetainedCloneSource::from_owner(left.clone());
    let right_source = RetainedCloneSource::from_owner(right.clone());
    let mut turns = 0usize;
    loop {
        turns += 1;
        match cursor.compare(left_source.borrow(), right_source.borrow(), grant).expect("bounded string comparison") {
            BoundedOrdStep::Progress(progress) => assert!(progress.fits(grant)),
            BoundedOrdStep::Complete { ordering, progress } => {
                assert!(progress.fits(grant));
                assert_eq!(ordering, Ordering::Less);
                break;
            }
        }
    }
    assert!(turns > fixture.long_key_byte_length / fixture.comparison_grant.maximum_bytes);

    let changed = RetainedCloneSource::from_owner(left.clone());
    let replacement = RetainedCloneSource::from_owner(format!("{}x", &left[..left.len() - 1]));
    let right = RetainedCloneSource::from_owner(right);
    let mut cursor = String::bounded_ord_cursor();
    cursor.compare(changed.borrow(), right.borrow(), grant).expect("comparison initialization");
    assert!(cursor.compare(replacement.borrow(), right.borrow(), grant).expect_err("changed comparison source must fail").contains("projected path changed"));
}

#[test]
fn immutable_lookup_target_and_repeated_directory_growth_match_btree_oracle() {
    let fixture = fixture();
    let source = RetainedOrderedMap::from_sorted_entries_for_test(oracle(&fixture).into_iter().collect()).expect("lookup source");
    let source = RetainedCloneSource::from_owner(source);
    let mut external_target = fixture.immutable_lookup.captured_target.clone();
    let target = RetainedCloneSource::from_owner(external_target.clone());
    let grant = BoundedOrdGrant { maximum_items: 1, maximum_bytes: 7 };
    let mut lookup = RetainedOrderedMapLookupCursor::default();
    lookup.advance(source.borrow(), target.borrow(), grant).expect("lookup initialization");
    external_target = fixture.immutable_lookup.external_after_capture.clone();
    let result = loop {
        if let (Some(result), progress) = lookup.advance(source.borrow(), target.borrow(), grant).expect("captured lookup") {
            assert!(progress.fits(grant));
            break result;
        }
    };
    assert_eq!(result, RetainedOrderedMapLookup::Missing(fixture.immutable_lookup.expected_ordinal));
    assert_ne!(external_target, fixture.immutable_lookup.captured_target);

    let growth = &fixture.repeated_growth;
    let mut oracle = (0..growth.entry_count).map(|ordinal| (format!("{}{:04}", growth.key_prefix, ordinal * 2), format!("{}{}", growth.value_prefix, ordinal))).collect::<BTreeMap<_, _>>();
    let source = RetainedOrderedMap::from_sorted_entries_for_test(oracle.clone().into_iter().collect()).expect("growth source");
    let mut map = retained_map(&source);
    let grant = RetainedOrderedMapInsertGrant { comparison: BoundedOrdGrant { maximum_items: 1, maximum_bytes: 7 }, maximum_moved_items: 32, maximum_moved_bytes: 4_096, maximum_capacity_bytes: 65_536 };
    for ordinal in 0..growth.insertions {
        let key = format!("{}{:04}", growth.key_prefix, ordinal * 2 + 1);
        let value = format!("{}insert-{ordinal}", growth.value_prefix);
        let mut cursor = RetainedOrderedMapInsertCursor::new(map, key.clone(), value.clone());
        loop {
            match cursor.advance(grant).expect("repeated map growth") {
                RetainedOrderedMapInsertStep::Progress(progress) => assert!(progress.fits(grant)),
                RetainedOrderedMapInsertStep::Complete { progress, .. } => {
                    assert!(progress.fits(grant));
                    break;
                }
            }
        }
        map = cursor.take().expect("grown map output");
        oracle.insert(key, value);
        assert!(cursor.begin_close());
        while !cursor.terminal_is_empty() {
            let step = cursor.close_step(1, 7).expect("grown insertion close");
            if step == SnapshotRetirementStep::Complete {
                assert!(cursor.terminal_is_empty());
            }
        }
    }
    assert_eq!(map.len(), growth.expected_entry_count);
    assert_eq!(map.iter().map(|(key, value)| (key.clone(), value.clone())).collect::<BTreeMap<_, _>>(), oracle);
}

#[test]
fn insertion_reports_comparison_movement_and_capacity_independently() {
    let fixture = fixture();
    let prefix = "k".repeat(fixture.long_key_byte_length);
    let entries = (0..RETAINED_ORDERED_MAP_PAGE_CAPACITY).map(|ordinal| (format!("{prefix}-{:04}", ordinal * 2), format!("value-{ordinal}"))).collect::<Vec<_>>();
    let oracle = entries.iter().cloned().collect::<BTreeMap<_, _>>();
    let map = RetainedOrderedMap::from_sorted_entries_for_test(entries).expect("long-key insertion source");
    let key = format!("{prefix}-0001");
    let value = "inserted-β".to_string();
    let grant = RetainedOrderedMapInsertGrant {
        comparison: BoundedOrdGrant { maximum_items: 1, maximum_bytes: fixture.comparison_grant.maximum_bytes },
        maximum_moved_items: RETAINED_ORDERED_MAP_PAGE_CAPACITY + 1,
        maximum_moved_bytes: (RETAINED_ORDERED_MAP_PAGE_CAPACITY + 1) * std::mem::size_of::<(String, String)>(),
        maximum_capacity_bytes: 65_536,
    };
    let mut cursor = RetainedOrderedMapInsertCursor::new(map, key.clone(), value.clone());
    let mut compared_bytes = 0usize;
    let mut moved_bytes = 0usize;
    let mut retained_capacity_bytes = 0usize;
    loop {
        let step = cursor.advance(grant).expect("long-key bounded insertion");
        let (progress, complete) = match step {
            RetainedOrderedMapInsertStep::Progress(progress) => (progress, false),
            RetainedOrderedMapInsertStep::Complete { progress, .. } => (progress, true),
        };
        assert!(progress.fits(grant));
        assert!(!fixture.progress_channels.comparison_only || progress.comparison.compared_bytes == 0 || (progress.moved_items == 0 && progress.moved_bytes == 0 && progress.retained_capacity_bytes == 0));
        assert!(!fixture.progress_channels.capacity_only || progress.retained_capacity_bytes == 0 || (progress.comparison == BoundedOrdProgress::default() && progress.moved_items == 0 && progress.moved_bytes == 0));
        compared_bytes += progress.comparison.compared_bytes;
        moved_bytes += progress.moved_bytes;
        retained_capacity_bytes += progress.retained_capacity_bytes;
        if complete {
            break;
        }
    }
    assert!(compared_bytes >= fixture.long_key_byte_length);
    assert!(moved_bytes >= fixture.progress_channels.minimum_moved_items * std::mem::size_of::<(String, String)>());
    assert!(retained_capacity_bytes >= RETAINED_ORDERED_MAP_PAGE_CAPACITY * std::mem::size_of::<(String, String)>());
    let map = cursor.take().expect("long-key insertion output");
    let mut oracle = oracle;
    oracle.insert(key, value);
    assert_eq!(map.iter().map(|(key, value)| (key.clone(), value.clone())).collect::<BTreeMap<_, _>>(), oracle);
    assert!(cursor.begin_close());
    assert!(cursor.terminal_is_empty());
}

#[test]
fn cancelled_partial_insertion_retires_candidate_and_shifted_workspace() {
    let fixture = fixture();
    let source = RetainedOrderedMap::from_sorted_entries_for_test(oracle(&fixture).into_iter().collect()).expect("ordered source");
    let workspace = retained_map(&source);
    let original_pages = workspace.pages.iter().map(Vec::len).collect::<Vec<_>>();
    let mut cursor = RetainedOrderedMapInsertCursor::new(workspace, "key-0000a".to_string(), "cancelled-β".to_string());
    let grant = RetainedOrderedMapInsertGrant { comparison: BoundedOrdGrant { maximum_items: 1, maximum_bytes: 7 }, maximum_moved_items: 32, maximum_moved_bytes: 4_096, maximum_capacity_bytes: 65_536 };
    let mut turns = 0usize;
    while cursor.map.as_ref().expect("insertion workspace").pages.iter().map(Vec::len).eq(original_pages.iter().copied()) {
        assert!(matches!(cursor.advance(grant).expect("partial insertion"), RetainedOrderedMapInsertStep::Progress(_)));
        turns += 1;
        assert!(turns < 1_000);
    }
    assert_eq!(cursor.map.as_ref().expect("shifted workspace").len(), fixture.entry_count);

    assert!(cursor.begin_close());
    while !cursor.terminal_is_empty() {
        let step = cursor.close_step(1, 7).expect("cancelled insertion cursor close");
        if step == SnapshotRetirementStep::Complete {
            assert!(cursor.terminal_is_empty());
        }
    }
}
