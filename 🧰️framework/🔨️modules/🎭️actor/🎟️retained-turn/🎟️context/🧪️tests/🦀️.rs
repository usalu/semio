//! 🧪️ Original actor input and authorities retain one source budget across bounded calls.
use super::*;
use semio_framework_value::RetainedCloneGrant;

fn fixture_clock() -> Option<u64> { Some(1) }
fn declared_work(law: &serde_json::Value) -> OriginalActorTurnWork {
    serde_json::from_value(law["work"].clone()).unwrap()
}
fn declared_grant(law: &serde_json::Value, key: &str) -> RetainedCloneGrant {
    let row = &law[key];
    RetainedCloneGrant {
        maximum_items: row[0].as_u64().unwrap() as usize,
        maximum_copy_bytes: row[1].as_u64().unwrap() as usize,
        maximum_capacity_bytes: row[2].as_u64().unwrap() as usize,
        maximum_release_bytes: row[3].as_u64().unwrap() as usize,
        maximum_depth: row[4].as_u64().unwrap() as usize,
    }
}
fn loan(policy: RetainedCloneGrant, total: RetainedCloneGrant) -> RetainedCloneGrant {
    RetainedCloneGrant {
        maximum_items: policy.maximum_items.min(total.maximum_items),
        maximum_copy_bytes: policy.maximum_copy_bytes.min(total.maximum_copy_bytes),
        maximum_capacity_bytes: policy.maximum_capacity_bytes.min(total.maximum_capacity_bytes),
        maximum_release_bytes: policy.maximum_release_bytes.min(total.maximum_release_bytes),
        maximum_depth: policy.maximum_depth.min(total.maximum_depth),
    }
}
fn next_input<T>(owner: &OriginalActorContext<T>, epoch: u64) -> RetainedTurnInput {
    let (operation, generation) = owner.identity.unwrap();
    RetainedTurnInput {
        operation: operation.0, generation: generation.0, epoch,
        work: owner.remaining_work().unwrap(), grant: owner.remaining_grant().unwrap(),
    }
}
fn observe<T>(body: impl FnOnce() -> T) -> (T, (usize, usize)) {
    let (value, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(body);
    (value, (heap.requested_bytes, heap.released_bytes))
}

#[test]
fn original_actor_retirement_binding_preserves_private_source_on_refusal() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🏛️boundary/🔣️.json")).unwrap();
    let law = &fixture["binding"];
    let mut retained = RetainedTurnInput {
        operation: law["operation"].as_u64().unwrap(),
        generation: law["generation"].as_u64().unwrap(),
        epoch: law["firstEpoch"].as_u64().unwrap(),
        work: declared_work(law), grant: declared_grant(law, "totalGrant"),
    };
    let policy = declared_grant(law, "callPolicy");
    let mut slot = OriginalActorContextSlot::empty();
    let (refused, heap) = observe(|| slot.begin_retirement_input(retained));
    assert_eq!(heap, (0, 0));
    assert_eq!(refused.unwrap_err().kind, ValueRefusalKind::InvariantViolated);
    assert!(slot.terminal_is_empty());
    let (admission, heap) = observe(|| slot.admit_step(retained, policy, fixture_clock));
    assert_eq!(heap, (0, 0));
    assert_eq!(admission.receipt_spent, RetainedCloneProgress { copied_items: 1, ..Default::default() });
    assert_eq!(admission.work_units_spent, 1);
    assert!(!admission.result.unwrap().ready);
    assert_eq!(slot.cancellation_is_cancelled(), None);
    retained = next_input(slot.context.as_ref().unwrap(), retained.epoch + 1);
    for row in law["cases"].as_array().unwrap() {
        let supplied = RetainedTurnInput {
            operation: row["operation"].as_u64().unwrap(),
            generation: row["generation"].as_u64().unwrap(),
            epoch: row["epoch"].as_u64().unwrap(), ..retained
        };
        let (result, heap) = observe(|| slot.begin_retirement_input(supplied));
        assert_eq!(heap, (0, 0));
        assert_eq!(result.is_ok(), row["accepted"].as_bool().unwrap());
        if let Err(error) = result { assert_eq!(error.kind, ValueRefusalKind::InvariantViolated); }
    }
    let mut metadata = 0;
    retained.epoch = 22;
    while !slot.terminal_is_empty() {
        if slot.context.as_ref().is_some_and(|original| original.terminal_is_empty()) {
            for row in law["closedContextCases"].as_array().unwrap() {
                let mut altered = retained;
                let increment = row["increment"].as_u64().unwrap();
                match row["axis"].as_str().unwrap() {
                    "fuel" => altered.work.fuel += increment,
                    "items" => altered.grant.maximum_items += increment as usize,
                    _ => unreachable!(),
                }
                let (call, heap) = observe(|| slot.close_step(altered, loan(policy, altered.grant)));
                assert_eq!(heap, (0, 0));
                assert_eq!(call.result.is_ok(), row["accepted"].as_bool().unwrap());
                assert_eq!(call.receipt_spent, Default::default());
                assert_eq!(call.work_units_spent, 0);
                assert!(!slot.terminal_is_empty());
            }
        }
        let (call, heap) = observe(|| slot.close_step(retained, loan(policy, retained.grant)));
        assert_eq!(heap, (0, 0));
        metadata += call.receipt_spent.copied_items;
        let returned = call.receipt().unwrap();
        assert!(call.result.is_ok(), "{:?}", call.result);
        retained = RetainedTurnInput {
            epoch: retained.epoch + 1, grant: returned.remaining,
            work: OriginalActorTurnWork { fuel: returned.remaining_fuel, ..retained.work }, ..retained
        };
        assert!(retained.epoch < 26);
    }
    assert_eq!(metadata, 3);
    assert_eq!(retained.grant.maximum_items, 60);
    assert_eq!(retained.work.fuel, 60);
    assert_eq!(observe(|| drop(slot)).1, (0, 0));
    eprintln!("[DEBUG] Actual actor private epochs20/21 accepted; foreign/repeated/zero denied heap0; total64-paid4=60; three separate close frontiers; terminalDrop0");
}

#[test]
fn original_actor_epoch_is_exact_and_cannot_be_rebound_by_a_child() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🧫️fixtures/🎟️epoch.json")).unwrap();
    let first = RetainedTurnInput {
        operation: fixture["operation"].as_u64().unwrap(),
        generation: fixture["generation"].as_u64().unwrap(), epoch: 1,
        work: declared_work(&fixture), grant: declared_grant(&fixture, "totalGrant"),
    };
    let policy = declared_grant(&fixture, "callPolicy");
    let mut owner = OriginalActorContext::new((), first).unwrap();
    let mut births = 0;
    for epoch in 1..=4 {
        let retained = next_input(&owner, epoch);
        let (admission, heap) = observe(|| owner.admit_step(retained, loan(policy, retained.grant), fixture_clock).unwrap());
        births += heap.0;
        assert_eq!(heap, (admission.progress.retained_capacity_bytes, 0));
        assert_eq!(admission.ready, epoch == 4);
    }
    assert_eq!(owner.take_input().unwrap(), Some(()));
    let recipient_identity = &owner.physical_spent as *const _ as usize;
    let source = owner.source.as_ref().unwrap() as *const _;
    let mut last = 4;
    for epoch in fixture["epochs"].as_array().unwrap() {
        let epoch = epoch.as_u64().unwrap();
        let retained = next_input(&owner, epoch);
        owner.begin_dispatch(retained).unwrap();
        let before = owner.physical_spent;
        let mut external = RetainedCloneProgress::default();
        let (_, heap) = observe(|| {
            let mut cx = owner.borrow_context(retained, StepBudget::new(1, retained.work.absolute_deadline_us, loan(policy, retained.grant)), &mut external, fixture_clock).unwrap();
            assert_eq!(cx.original_source_epoch(), Some(epoch));
            assert_eq!(cx.original_recipient_identity(), recipient_identity);
            assert_eq!(cx.original_source_parent().unwrap() as *const _, source);
            assert_eq!(cx.retained_progress(), before);
            assert!(cx.bind_original_source_epoch(fixture["refusedEpoch"].as_u64().unwrap()).is_err());
            assert!(cx.bind_original_source_epoch(epoch + 1).is_err());
            cx.bind_original_source_epoch(epoch).unwrap();
            assert_eq!(cx.original_source_epoch(), Some(epoch));
        });
        assert_eq!(heap, (fixture["heapBytes"].as_u64().unwrap() as usize, 0));
        assert_eq!(external, RetainedCloneProgress::default());
        assert_eq!(owner.physical_spent, before);
        last = epoch;
    }
    let mut released = 0;
    while !owner.terminal_is_empty() {
        last += 1;
        let retained = next_input(&owner, last);
        let (step, heap) = observe(|| owner.close_step(retained, loan(policy, retained.grant)).unwrap());
        assert_eq!(heap, (0, step.progress().released_bytes));
        released += heap.1;
    }
    assert_eq!(released, births);
    assert_eq!(observe(|| drop(owner)).1, (0, 0));
    eprintln!("[DEBUG] Actual actor epochs7/9 childRebind refused heap0; persistent source recipient stable; external zero delta; rootsReleased={released}");
}

#[test]
fn original_actor_context_preserves_input_and_paid_authorities_across_admission_loans_and_close() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
    let admission = declared_grant(&fixture, "admissionGrant");
    let body = declared_grant(&fixture, "bodyGrant");
    let mut input = Vec::<u8>::with_capacity(fixture["originalInputCapacity"].as_u64().unwrap() as usize);
    input.extend_from_slice(&[83; 41]);
    let pointer = input.as_ptr();
    let capacity = input.capacity();
    assert_eq!(capacity, 97);
    let first = RetainedTurnInput {
        operation: fixture["operation"].as_u64().unwrap(),
        generation: fixture["generation"].as_u64().unwrap(), epoch: 1,
        work: declared_work(&fixture), grant: declared_grant(&fixture, "totalGrant"),
    };
    let (mut owner, heap) = observe(|| OriginalActorContext::new(input, first).unwrap());
    assert_eq!(heap, (0, 0));
    let phases = [
        RetainedCloneGrant { maximum_items: 0, ..admission }, admission,
        RetainedCloneGrant { maximum_capacity_bytes: 0, ..admission }, admission,
        RetainedCloneGrant { maximum_capacity_bytes: 0, ..admission }, admission, body,
    ];
    let mut births = 0;
    for (index, policy) in phases.into_iter().enumerate() {
        let retained = next_input(&owner, index as u64 + 1);
        let (actual, heap) = observe(|| owner.admit_step(retained, loan(policy, retained.grant), fixture_clock).unwrap());
        let expected = &fixture["phases"][index];
        assert_eq!(actual.ready, expected["ready"].as_bool().unwrap());
        assert_eq!(actual.progress.copied_items, expected["items"].as_u64().unwrap() as usize);
        assert_eq!(actual.progress.copied_bytes, 0);
        assert_eq!(heap, (actual.progress.retained_capacity_bytes, 0));
        assert_eq!(owner.input().unwrap().as_ptr(), pointer);
        assert_eq!(owner.input().unwrap().capacity(), capacity);
        births += actual.progress.retained_capacity_bytes;
        if index < 6 {
            let (result, heap) = observe(|| owner.take_input());
            assert!(result.is_err());
            assert_eq!(heap, (0, 0));
        }
    }
    let parent = owner.source.as_ref().unwrap() as *const _;
    let recipient_identity = &owner.physical_spent as *const _ as usize;
    let (mut original, heap) = observe(|| owner.take_input().unwrap().unwrap());
    assert_eq!(heap, (0, 0));
    assert_eq!(original.as_ptr(), pointer);
    let retained = next_input(&owner, 8);
    owner.begin_dispatch(retained).unwrap();
    let mut external = RetainedCloneProgress::default();
    let mut target = [0; 41];
    let (_, heap) = observe(|| {
        let mut cx = owner.borrow_context(retained, StepBudget::new(1, retained.work.absolute_deadline_us, loan(body, retained.grant)), &mut external, fixture_clock).unwrap();
        assert_eq!(cx.original_source_parent().unwrap() as *const _, parent);
        assert_eq!(cx.original_recipient_identity(), recipient_identity);
        cx.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes: 41, ..Default::default() }).unwrap();
        cx.consume_fuel(1);
        target.copy_from_slice(&original);
    });
    assert_eq!(heap, (0, 0));
    assert_eq!(target, original.as_slice());
    assert_eq!(external, RetainedCloneProgress { copied_items: 1, copied_bytes: 41, ..Default::default() });
    assert_eq!(owner.remaining_grant().unwrap().maximum_copy_bytes, 4055);
    assert_eq!(owner.remaining_work().unwrap().fuel, 59);
    assert_eq!(retained.return_original(external, 1).unwrap().remaining, owner.remaining_grant().unwrap());
    let retained = next_input(&owner, 9);
    owner.begin_dispatch(retained).unwrap();
    let mut release = RetainedCloneProgress::default();
    let (_, heap) = observe(|| {
        let mut cx = owner.borrow_context(retained, StepBudget::new(1, retained.work.absolute_deadline_us, loan(body, retained.grant)), &mut release, fixture_clock).unwrap();
        cx.consume_retained(RetainedCloneProgress { copied_items: 1, released_bytes: capacity, ..Default::default() }).unwrap();
        cx.consume_fuel(1);
        drop(std::mem::take(&mut original));
    });
    assert_eq!(heap, (0, capacity));
    assert_eq!(release.released_bytes, capacity);
    assert_eq!(retained.return_original(release, 1).unwrap().remaining, owner.remaining_grant().unwrap());
    let mut released = 0;
    let mut steps = 0;
    while !owner.terminal_is_empty() {
        let retained = next_input(&owner, 10 + steps);
        let (step, heap) = observe(|| owner.close_step(retained, loan(body, retained.grant)).unwrap());
        assert_eq!(step.progress().copied_items, 1);
        assert_eq!(step.progress().copied_bytes, 0);
        assert_eq!(heap, (0, step.progress().released_bytes));
        released += step.progress().released_bytes;
        steps += 1;
        assert!(steps <= 16);
    }
    assert_eq!(released, births);
    assert_eq!(observe(|| drop(owner)).1, (0, 0));
    eprintln!("[DEBUG] Actual actor71/3 source64 paid admission4 copy41 release97 through same persistent recipient={recipient_identity}; independent external deltas; rootsReleased={released}; terminalDrop0");
}
