use super::*;
use semio_framework_value::retirement::controlled::ControlledRetirement;

fn grant(d: RetirementDemand) -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: d.copy_bytes, maximum_capacity_bytes: d.capacity_bytes, maximum_release_bytes: d.release_bytes, maximum_depth: d.depth }
}

fn observe<T>(action: impl FnOnce() -> T) -> (T, usize, usize) {
    semio_framework_trace_memory_testing::observe_backing(action)
}

fn run(case: &serde_json::Value, cut: Option<usize>) -> usize {
    let input = case["input"].as_array().unwrap().iter().map(|value| value.as_str().unwrap().to_owned()).collect::<Vec<_>>();
    let expected = case["expected"].as_array().unwrap().iter().map(|value| value.as_str().unwrap().to_owned()).collect::<Vec<_>>();
    let required = input.len().checked_add(usize::try_from(case["additional"].as_u64().unwrap()).unwrap()).unwrap();
    let mut buffer = OwnedRowsBuffer::new(input, required).unwrap_or_else(|_| panic!("valid original row capacity"));
    let mut turns = 0;
    while !buffer.ready() && cut.is_none_or(|cut| turns < cut) {
        let demand = buffer.demand(4096, 64).unwrap();
        let work = buffer.work_demand(4096, 64).unwrap();
        let selected = grant(demand);
        for denied in [
            Some(RetainedCloneGrant::default()),
            Some(RetainedCloneGrant { maximum_items: 0, ..selected }),
            (demand.copy_bytes > 0).then_some(RetainedCloneGrant { maximum_copy_bytes: demand.copy_bytes - 1, ..selected }),
            (demand.capacity_bytes > 0).then_some(RetainedCloneGrant { maximum_capacity_bytes: demand.capacity_bytes - 1, ..selected }),
            (demand.release_bytes > 0).then_some(RetainedCloneGrant { maximum_release_bytes: demand.release_bytes - 1, ..selected }),
            Some(RetainedCloneGrant { maximum_depth: 0, ..selected }),
        ]
        .into_iter()
        .flatten()
        {
            let (turn, born, released) = observe(|| buffer.advance(4096, 64, denied));
            assert_eq!((born, released), (0, 0));
            assert_eq!(turn.work, 0);
            assert_eq!(turn.result.unwrap().progress(), Default::default());
        }
        let (turn, born, released) = observe(|| buffer.advance(4096, 0, selected));
        assert_eq!((born, released), (0, 0));
        assert_eq!(turn.work, 0);
        assert_eq!(turn.result.unwrap().progress(), Default::default());
        let (turn, born, released) = observe(|| buffer.advance(4096, 64, selected));
        assert!(turn.work <= work);
        let progress = turn.result.unwrap().progress();
        assert!(progress.fits(selected));
        assert_eq!((born, released), (progress.retained_capacity_bytes, progress.released_bytes));
        turns += 1;
        assert!(turns < 4096);
    }
    if cut.is_none() {
        let selected = grant(buffer.take_demand().unwrap());
        let (turn, born, released) = observe(|| buffer.take(selected));
        assert_eq!((born, released), (0, 0));
        let (output, progress) = turn.result.unwrap().unwrap();
        assert!(progress.fits(selected));
        assert_eq!(output, expected);
        assert!(output.capacity() >= required);
        let mut retirement = ControlledRetirement::new(output).unwrap();
        while !retirement.terminal_is_empty() {
            let selected = grant(retirement.next_demand(4096).unwrap());
            let step = retirement.step(selected).unwrap();
            assert!(step.progress().fits(selected));
        }
    }
    buffer.begin_close();
    for _ in 0..4096 {
        if buffer.terminal_is_empty() {
            return turns;
        }
        let selected = grant(buffer.close_demand(4096).unwrap());
        let work = buffer.close_work_demand(4096).unwrap();
        let (denied, born, released) = observe(|| buffer.close_step(RetainedCloneGrant::default()));
        assert_eq!((born, released), (0, 0));
        assert_eq!(denied.work, 0);
        assert_eq!(denied.result.unwrap().progress(), Default::default());
        let (turn, born, released) = observe(|| buffer.close_step(selected));
        assert!(turn.work <= work);
        let progress = turn.result.unwrap().progress();
        assert!(progress.fits(selected));
        assert_eq!((born, released), (progress.retained_capacity_bytes, progress.released_bytes));
    }
    panic!("owned row buffer closure did not settle");
}

/// 🧺️ Real owned row capacity grows with exact backing receipts and preserves every portable row through all cancellation frontiers.
#[test]
fn owned_rows_buffer_preserves_neutral_order_and_funded_cancellation() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧪️tests/🧫️owned-rows-buffer/🔣️.json")).unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let turns = run(case, None);
        for cut in 0..=turns {
            run(case, Some(cut));
        }
    }
}

/// 🛑️ Capacity overflow returns the exact original row allocation without taking any child ownership.
#[test]
fn owned_rows_buffer_overflow_returns_its_original_input() {
    let input = vec!["original🧱".to_owned()];
    let pointer = input.as_ptr();
    let (result, born, released) = observe(|| OwnedRowsBuffer::new(input, usize::MAX));
    assert_eq!((born, released), (0, 0));
    let (_, input) = match result {
        Err(failure) => failure,
        Ok(_) => panic!("overflow must refuse before taking custody"),
    };
    assert_eq!(input.as_ptr(), pointer);
    assert_eq!(input, ["original🧱"]);
    let mut owner = ControlledRetirement::new(input).unwrap();
    while !owner.terminal_is_empty() {
        let selected = grant(owner.next_demand(4096).unwrap());
        assert!(owner.step(selected).unwrap().progress().fits(selected));
    }
}

/// 📏️ An unaddressable byte layout refuses before moving or allocating the exact original row vector.
#[test]
fn owned_rows_buffer_unaddressable_layout_preserves_original_input() {
    let input = vec![7u8];
    let pointer = input.as_ptr();
    let (result, born, released) = observe(|| OwnedRowsBuffer::new(input, (isize::MAX as usize) + 1));
    assert_eq!((born, released), (0, 0));
    let (error, input) = match result { Err(failure) => failure, Ok(_) => panic!("unaddressable layout must refuse") };
    assert_eq!(error.kind, ValueRefusalKind::OwnershipLimit);
    assert_eq!(input.as_ptr(), pointer);
    assert_eq!(input, [7]);
    let mut owner = ControlledRetirement::new(input).unwrap();
    while !owner.terminal_is_empty() { let selected = grant(owner.next_demand(4096).unwrap()); assert!(owner.step(selected).unwrap().progress().fits(selected)); }
}

/// 🪵️ Already sufficient original capacity transfers its header without reallocating or copying row values.
#[test]
fn owned_rows_buffer_preserves_sufficient_original_backing() {
    let mut input = Vec::with_capacity(8);
    input.push("a".to_owned());
    input.push("b".to_owned());
    let pointer = input.as_ptr();
    let mut owner = OwnedRowsBuffer::new(input, 4).unwrap_or_else(|_| panic!("sufficient original capacity"));
    let selected = grant(owner.demand(64, 1).unwrap());
    assert_eq!(selected.maximum_capacity_bytes, 0);
    let (turn, born, released) = observe(|| owner.advance(64, 1, selected));
    assert_eq!((born, released), (0, 0));
    assert_eq!(turn.work, 1);
    assert!(matches!(turn.result.unwrap(), RetainedCloneStep::Complete(_)));
    let selected = grant(owner.take_demand().unwrap());
    let (output, _) = owner.take(selected).result.unwrap().unwrap();
    assert_eq!(output.as_ptr(), pointer);
    assert_eq!(output, ["a", "b"]);
    owner.begin_close();
    assert!(owner.terminal_is_empty());
    let mut output = ControlledRetirement::new(output).unwrap();
    while !output.terminal_is_empty() {
        let selected = grant(output.next_demand(4096).unwrap());
        assert!(output.step(selected).unwrap().progress().fits(selected));
    }
}
