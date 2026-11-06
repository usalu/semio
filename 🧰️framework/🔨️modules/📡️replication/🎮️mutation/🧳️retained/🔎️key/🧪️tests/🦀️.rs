use super::*;
use semio_framework_test_fixtures::fixture_text;
use semio_framework_value::{retained_clone::RetainedCloneSource, retirement::RetireOwned};

fn grant(demand: RetirementDemand) -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth }
}

fn observe<T>(operation: impl FnOnce() -> T) -> (T, (usize, usize)) {
    let (result, born, released) = crate::test_allocation::observe_backing(operation);
    (result, (born, released))
}

fn source<T: RetireOwned + Sync>(value: T) -> RetainedCloneSource<T> {
    let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 1 << 20, maximum_capacity_bytes: 1 << 20, maximum_release_bytes: 1 << 20, maximum_depth: 4096 };
    let mut admission = RetainedCloneSource::admission_owned(value, ());
    let mut output = None;
    for _ in 0..64 {
        if admission.terminal_is_empty() {
            return output.expect("funded original source");
        }
        let mut actual = None;
        let outcome = if admission.phase() == "ready" { admission.take_output(&mut output, grant) } else { admission.advance(grant, &mut actual) };
        assert!(outcome.result.unwrap().progress().fits(grant));
    }
    panic!("original source admission did not settle");
}

fn close_source<T: RetireOwned + Sync>(source: &mut RetainedCloneSource<T>) {
    for _ in 0..4096 {
        if source.terminal_is_empty() {
            return;
        }
        let quote = grant(RetirementDemand {
            copy_bytes: source.next_close_copy_byte_demand().unwrap(),
            capacity_bytes: source.next_close_capacity_byte_demand(4096).unwrap(),
            release_bytes: source.next_close_release_byte_demand().unwrap(),
            depth: source.next_close_depth_demand().unwrap(),
        });
        assert!(source.close_step(quote).result.unwrap().progress().fits(quote));
    }
    panic!("original source closure did not settle");
}

fn close(lookup: &mut RetainedKeyLookup) {
    lookup.begin_close();
    for _ in 0..4096 {
        if lookup.terminal_is_empty() {
            return;
        }
        let quote = grant(lookup.close_demand(4096).unwrap());
        let work = lookup.close_work_demand(4096).unwrap();
        let (denied, heap) = observe(|| lookup.close_step(RetainedCloneGrant::default()));
        assert_eq!(heap, (0, 0));
        assert_eq!(denied.work, 0);
        assert_eq!(denied.result.unwrap().progress(), Default::default());
        let (outcome, heap) = observe(|| lookup.close_step(quote));
        assert!(outcome.work <= work);
        let step = outcome.result.unwrap();
        assert!(step.progress().fits(quote));
        assert_eq!(heap, (step.progress().retained_capacity_bytes, step.progress().released_bytes));
    }
    panic!("lookup closure did not settle");
}

fn keys(case: &serde_json::Value) -> Vec<String> {
    case["keys"].as_array().unwrap().iter().map(|key| key.as_str().unwrap().to_owned()).collect()
}
fn state(lookup: &RetainedKeyLookup) -> (bool, bool, usize, usize, Option<usize>, Option<RetainedKeyMatch>) {
    (lookup.rows.is_some(), lookup.key.is_some(), lookup.index, lookup.offset, lookup.first, lookup.result)
}
fn indices(result: RetainedKeyMatch) -> Vec<usize> {
    match result {
        RetainedKeyMatch::Missing => vec![],
        RetainedKeyMatch::Unique(index) => vec![index],
        RetainedKeyMatch::Duplicate(first, second) => vec![first, second],
    }
}

fn run(case: &serde_json::Value, cut: Option<usize>) -> usize {
    let mut rows = source(keys(case));
    let mut key = source(case["needle"].as_str().unwrap().to_owned());
    let mut lookup = RetainedKeyLookup::new();
    let mut turns = 0;
    while lookup.result().is_none() && cut.is_none_or(|cut| turns < cut) {
        let (demand, heap) = observe(|| lookup.demand(rows.borrow(), key.borrow(), 1, 2).unwrap());
        assert_eq!(heap, (0, 0));
        let work = lookup.advance_work_demand(rows.borrow(), key.borrow(), 1, 2).unwrap();
        assert!((1..=2).contains(&work));
        let selected = grant(demand);
        if work == 2 {
            let before = state(&lookup);
            assert_eq!(lookup.advance_work_demand(rows.borrow(), key.borrow(), 1, 1).unwrap(), 0);
            let (outcome, heap) = observe(|| lookup.advance(rows.borrow(), key.borrow(), 1, 1, selected));
            assert_eq!(heap, (0, 0));
            assert_eq!(outcome.work, 0);
            assert_eq!(outcome.result.unwrap().progress(), Default::default());
            assert_eq!(state(&lookup), before);
        }
        for denied in [
            Some(RetainedCloneGrant::default()),
            Some(RetainedCloneGrant { maximum_items: 0, ..selected }),
            (demand.copy_bytes > 0).then_some(RetainedCloneGrant { maximum_copy_bytes: demand.copy_bytes - 1, ..selected }),
            Some(RetainedCloneGrant { maximum_depth: 0, ..selected }),
        ]
        .into_iter()
        .flatten()
        {
            let before = state(&lookup);
            let (outcome, heap) = observe(|| lookup.advance(rows.borrow(), key.borrow(), 1, 2, denied));
            assert_eq!(heap, (0, 0));
            assert_eq!(outcome.work, 0);
            assert_eq!(outcome.result.unwrap().progress(), Default::default());
            assert_eq!(state(&lookup), before);
        }
        let before = state(&lookup);
        let (outcome, heap) = observe(|| lookup.advance(rows.borrow(), key.borrow(), 1, 0, selected));
        assert_eq!(heap, (0, 0));
        assert_eq!(outcome.work, 0);
        assert_eq!(outcome.result.unwrap().progress(), Default::default());
        assert_eq!(state(&lookup), before);
        let (outcome, heap) = observe(|| lookup.advance(rows.borrow(), key.borrow(), 1, 2, selected));
        assert_eq!(heap, (0, 0));
        assert_eq!(outcome.work, work);
        assert!(outcome.result.unwrap().progress().fits(selected));
        turns += 1;
        assert!(turns < 4096);
    }
    if cut.is_none() {
        let expected: Vec<usize> = case["expectedIndices"].as_array().unwrap().iter().map(|value| usize::try_from(value.as_u64().unwrap()).unwrap()).collect();
        assert_eq!(indices(lookup.result().unwrap()), expected, "{}", case["id"]);
    }
    close(&mut lookup);
    assert_eq!(observe(|| drop(lookup)).1, (0, 0));
    close_source(&mut key);
    close_source(&mut rows);
    turns
}

/// 🧫️ Portable Unicode, duplicate and empty keys resolve under one-byte turns with explicitly funded stored-key projections without row or key cloning.
#[test]
fn retained_key_lookup_matches_neutral_queries_without_key_or_row_clone() {
    let corpus: serde_json::Value = serde_json::from_str(fixture_text!("../../🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    for case in corpus["cases"].as_array().unwrap() {
        run(case, None);
    }
}

/// 🛑️ Every successful lookup frontier can return all actual aliases with accurate physical and work receipts.
#[test]
fn retained_key_lookup_cancellation_settles_all_original_aliases() {
    let corpus: serde_json::Value = serde_json::from_str(fixture_text!("../../🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    for case in corpus["cases"].as_array().unwrap() {
        let turns = run(case, None);
        for cut in 0..=turns {
            run(case, Some(cut));
        }
    }
}

/// 🪪️ Foreign sources fail before work even after the query completed.
#[test]
fn retained_key_lookup_rejects_source_substitution() {
    for finish in [false, true] {
        let mut rows = source(vec!["same".to_owned()]);
        let mut key = source("same".to_owned());
        let mut foreign_rows = source(vec!["same".to_owned()]);
        let mut foreign_key = source("same".to_owned());
        let mut lookup = RetainedKeyLookup::new();
        let mut turns = 0;
        while lookup.result().is_none() && (finish || turns < 3) {
            let selected = grant(lookup.demand(rows.borrow(), key.borrow(), 1, 2).unwrap());
            lookup.advance(rows.borrow(), key.borrow(), 1, 2, selected).result.unwrap();
            turns += 1;
            assert!(turns < 100);
        }
        let before = state(&lookup);
        for (rows, key) in [(foreign_rows.borrow(), key.borrow()), (rows.borrow(), foreign_key.borrow())] {
            let (outcome, heap) = observe(|| lookup.advance(rows, key, 1, 2, grant(scalar(4096))));
            assert_eq!(heap, (0, 0));
            assert_eq!(outcome.work, 0);
            assert!(outcome.result.is_err());
            assert_eq!(state(&lookup), before);
        }
        close(&mut lookup);
        close_source(&mut foreign_key);
        close_source(&mut foreign_rows);
        close_source(&mut key);
        close_source(&mut rows);
    }
}
