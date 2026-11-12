use super::*;
use semio_framework_test_fixtures::fixture_text;
use semio_framework_value::{retained_clone::RetainedCloneSource, retirement::RetireOwned, RetainedWorkPoll};
use semio_framework_value_clone_source_testing::admit_owned;
use semio_framework_value_testing_effect_bank::{EffectBank, Wallet};

fn grant(demand: RetirementDemand) -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth }
}

fn observe<T>(operation: impl FnOnce() -> T) -> (T, (usize, usize)) {
    let (result, born, released) = crate::test_allocation::observe_backing(operation);
    (result, (born, released))
}

thread_local! {
    static BANK: std::cell::RefCell<EffectBank> = std::cell::RefCell::new(EffectBank::new(Wallet::new(RetainedCloneGrant { maximum_items: usize::MAX, maximum_copy_bytes: usize::MAX, maximum_capacity_bytes: usize::MAX, maximum_release_bytes: usize::MAX, maximum_depth: usize::MAX }, u64::MAX)));
}

/// 🪙️ Admits one owned fixture under the thread's finite Source stock with the whole remaining stock as its grant.
fn source<T: RetireOwned + Sync>(value: T) -> RetainedCloneSource<T> {
    BANK.with(|bank| {
        let mut bank = bank.borrow_mut();
        let stock = bank.wallet().remaining();
        let outcome = admit_owned(&mut bank, value, (), stock);
        let admitted = match outcome.result {
            Ok(admitted) => admitted,
            Err(_) => panic!("funded original source admission"),
        };
        assert!(admitted.1.fits(stock));
        admitted.0
    })
}

/// ♻️ Closes one admitted source under real original actions, each granted exactly its own quote and work.
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
        let work = source.next_close_work_demand(4096).unwrap();
        let poll = BANK.with(|bank| bank.borrow_mut().run(quote, work, |effect| source.close_step(4096, effect)).0);
        if let RetainedWorkPoll::Complete(turn) = poll {
            assert!(turn.result.unwrap().progress().fits(quote));
        }
    }
    panic!("original source closure did not settle");
}

/// ▶️ Runs one lookup advance under a real original action granting the given grant and the selected work bound.
fn advance<R: RetainedKeyedRow>(lookup: &mut RetainedKeyLookup, rows: RetainedCloneRef<'_, Vec<R>>, key: RetainedCloneRef<'_, String>, body: usize, work: u64, grant: RetainedCloneGrant) -> semio_framework_value::RetainedWorkOutcome<semio_framework_value::RetainedCloneStep, semio_framework_value::ValueError> {
    BANK.with(|bank| bank.borrow_mut().run_outcome(grant, work, |effect| lookup.advance(rows, key, body, work, effect)))
}

/// ♻️ Runs one lookup close turn under a real original action granting the given grant and the quoted work.
fn close_step(lookup: &mut RetainedKeyLookup, grant: RetainedCloneGrant) -> semio_framework_value::RetainedWorkOutcome<semio_framework_value::RetainedCloneStep, semio_framework_value::OriginalRetirementFailure> {
    let work = lookup.close_work_demand(4096).unwrap();
    match BANK.with(|bank| bank.borrow_mut().run(grant, work, |effect| lookup.close_step(effect)).0) {
        RetainedWorkPoll::Complete(turn) => turn,
        RetainedWorkPoll::Pending { .. } => panic!("lookup closure stayed pending"),
    }
}

fn close(lookup: &mut RetainedKeyLookup) {
    lookup.begin_close();
    for _ in 0..4096 {
        if lookup.terminal_is_empty() {
            return;
        }
        let quote = grant(lookup.close_demand(4096).unwrap());
        let work = lookup.close_work_demand(4096).unwrap();
        let (denied, heap) = observe(|| close_step(lookup, RetainedCloneGrant::default()));
        assert_eq!(heap, (0, 0));
        assert_eq!(denied.work, 0);
        assert_eq!(denied.result.unwrap().progress(), Default::default());
        let (outcome, heap) = observe(|| close_step(lookup, quote));
        assert!(outcome.work <= u128::from(work));
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
            let (outcome, heap) = observe(|| advance(&mut lookup, rows.borrow(), key.borrow(), 1, 1, selected));
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
            let (outcome, heap) = observe(|| advance(&mut lookup, rows.borrow(), key.borrow(), 1, 2, denied));
            assert_eq!(heap, (0, 0));
            assert_eq!(outcome.work, 0);
            assert_eq!(outcome.result.unwrap().progress(), Default::default());
            assert_eq!(state(&lookup), before);
        }
        let before = state(&lookup);
        let (outcome, heap) = observe(|| advance(&mut lookup, rows.borrow(), key.borrow(), 1, 0, selected));
        assert_eq!(heap, (0, 0));
        assert_eq!(outcome.work, 0);
        assert_eq!(outcome.result.unwrap().progress(), Default::default());
        assert_eq!(state(&lookup), before);
        let (outcome, heap) = observe(|| advance(&mut lookup, rows.borrow(), key.borrow(), 1, 2, selected));
        assert_eq!(heap, (0, 0));
        assert_eq!(outcome.work, u128::from(work));
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
            advance(&mut lookup, rows.borrow(), key.borrow(), 1, 2, selected).result.unwrap();
            turns += 1;
            assert!(turns < 100);
        }
        let before = state(&lookup);
        for (rows, key) in [(foreign_rows.borrow(), key.borrow()), (rows.borrow(), foreign_key.borrow())] {
            let (outcome, heap) = observe(|| advance(&mut lookup, rows, key, 1, 2, grant(scalar(4096))));
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
