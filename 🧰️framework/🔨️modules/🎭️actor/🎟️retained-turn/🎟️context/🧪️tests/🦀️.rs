//! 🧪️ Original actor input and authorities retain one source budget across bounded calls.
use super::*;
use semio_framework_value::RetainedCloneGrant;
#[test]
fn actor_observer_checked_stock_matches_portable_oracle_without_hiding_overredemption(){let law:serde_json::Value=serde_json::from_str(include_str!("reserved-observer.json")).unwrap();for row in law["cases"].as_array().unwrap(){let actual=RetainedCloneProgress{copied_items:row["aggregate"].as_u64().unwrap()as usize,..Default::default()};let reserved=RetainedCloneProgress{copied_items:row["reserved"].as_u64().unwrap()as usize,..Default::default()};let fresh=fresh_progress(actual,reserved);let work=row["aggregateWork"].as_u64().unwrap().checked_sub(row["reservedWork"].as_u64().unwrap());let valid=fresh.as_ref().is_ok_and(|fresh|fresh.copied_items<=row["grant"].as_u64().unwrap()as usize)&&work.is_some_and(|work|work<=row["fuel"].as_u64().unwrap());assert_eq!(valid,row["valid"].as_bool().unwrap());if valid{assert_eq!(fresh.unwrap().copied_items,row["fresh"].as_u64().unwrap()as usize);assert_eq!(work.unwrap(),row["freshWork"].as_u64().unwrap());}}}
#[test]
fn actor_reserved_redemption_preserves_aggregate_receipt_and_repeated_fresh_borrow(){
 let stock=RetainedCloneGrant{maximum_items:100000,maximum_copy_bytes:1<<24,maximum_capacity_bytes:1<<24,maximum_release_bytes:1<<24,maximum_depth:128};let mut input=RetainedTurnInput{operation:981,generation:1,epoch:1,grant:stock,work:OriginalActorTurnWork{clock_binding:981,absolute_deadline_us:1000000,fuel:1000000}};let mut slot=OriginalActorContextSlot::empty();
 loop{let call=slot.admit_step(input,StepBudget::new(input.work.fuel,input.work.absolute_deadline_us,stock),fixture_clock);let ready=call.result.unwrap().ready;input=next_input(slot.context.as_ref().unwrap(),input.epoch+1);if ready{break}}
 let mut cell=0u64;let address=&mut cell as*mut u64 as usize;let mut place=semio_framework_job::OriginalCallerReturnPlace::empty();let mut preissued=false;let mut observed=false;
 for _ in 0..64{let mut receipt=Default::default();let call=slot.dispatch_step(input,StepBudget::new(input.work.fuel,input.work.absolute_deadline_us,input.grant),&mut receipt,fixture_clock,|cx|->Result<(),OriginalTurnFailure>{if !place.is_admitted(){place.admit_sized(cx,address,8)?;}else if !preissued{preissued=place.preissue_capture(cx,address,8)?;if preissued{cell=7;}}else{let before=cx.reserved_return_progress();let empty=RetainedCloneGrant{maximum_items:0,maximum_copy_bytes:0,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:128};cx.with_retained_loan(empty,|cx|cx.with_original_work_loan(0,cx.deadline_us(),|cx|{place.settle_return(cx)?;Ok(())}))?;let after=cx.reserved_return_progress();assert!(after.copied_items>before.copied_items);assert!(after.copied_bytes>before.copied_bytes);}Ok(())});assert!(call.result.is_ok());let context=slot.context.as_ref().unwrap();let fresh_work=context.fresh_work().unwrap();assert_eq!(u128::from(call.work_units_spent),fresh_work+context.reserved_work);if context.reserved_work!=0{observed=true;assert!(context.reserved_spent.copied_items!=0);let before=context.fresh_spent;let mut repeated=Default::default();{let remaining=context.remaining_work().unwrap();let grant=context.remaining_grant().unwrap();let mut cx=slot.context.as_mut().unwrap().borrow_context(input,StepBudget::new(remaining.fuel,remaining.absolute_deadline_us,grant),&mut repeated,fixture_clock).unwrap();assert_eq!(cx.reserved_return_progress(),Default::default());cx.consume_fuel(0);}assert_eq!(slot.context.as_ref().unwrap().fresh_spent,before);assert_eq!(repeated,Default::default());}input=next_input(slot.context.as_ref().unwrap(),input.epoch+1);if preissued&&place.terminal_is_empty(){break}}
 assert!(observed);assert_eq!(cell,7);assert!(place.terminal_is_empty());while !slot.terminal_is_empty(){let call=slot.close_step(input,semio_framework_job::StepBudget::new(input.work.fuel,input.work.absolute_deadline_us,input.grant));assert!(call.result.is_ok());if let Some(context)=slot.context.as_ref(){input=next_input(context,input.epoch+1);}}drop(slot);
}

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
    let (admission, heap) = observe(|| slot.admit_step(retained,StepBudget::new(retained.work.fuel,retained.work.absolute_deadline_us,policy), fixture_clock));
    assert_eq!(heap, (0, 0));
    assert_eq!(admission.receipt_spent, RetainedCloneProgress { copied_items: 1,copied_bytes:std::mem::size_of::<Option<OriginalActorContext<()>>>(), ..Default::default() });
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
        let (call, heap) = observe(|| slot.close_step(retained,semio_framework_job::StepBudget::new(retained.work.fuel,retained.work.absolute_deadline_us,loan(policy, retained.grant))));
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
        let (admission, heap) = observe(|| owner.admit_step(retained,StepBudget::new(retained.work.fuel,retained.work.absolute_deadline_us,loan(policy,retained.grant)), fixture_clock).unwrap());
        births += heap.0;
        assert_eq!(heap, (admission.progress.retained_capacity_bytes, 0));
        assert_eq!(admission.ready, epoch == 4);
    }
    assert_eq!(owner.take_input().unwrap(), Some(()));
    let recipient_identity = &owner.fresh_spent as *const _ as usize;
    let source = owner.source.as_ref().unwrap() as *const _;
    let mut last = 4;
    for epoch in fixture["epochs"].as_array().unwrap() {
        let epoch = epoch.as_u64().unwrap();
        let retained = next_input(&owner, epoch);
        owner.begin_dispatch(retained).unwrap();
        let before = owner.fresh_spent;
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
        assert_eq!(owner.fresh_spent, before);
        last = epoch;
    }
    let mut released = 0;
    while !owner.terminal_is_empty() {
        last += 1;
        let retained = next_input(&owner, last);
        let (step, heap) = observe(|| owner.close_step(retained,semio_framework_job::StepBudget::new(retained.work.fuel,retained.work.absolute_deadline_us,loan(policy, retained.grant))).unwrap());
        assert_eq!(heap, (0, step.progress().released_bytes));
        released += heap.1;
    }
    assert_eq!(released, births);
    assert_eq!(observe(|| drop(owner)).1, (0, 0));
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
        let (actual, heap) = observe(|| owner.admit_step(retained,StepBudget::new(retained.work.fuel,retained.work.absolute_deadline_us,loan(policy,retained.grant)), fixture_clock).unwrap());
        let expected = &fixture["phases"][index];
        assert_eq!(actual.ready, expected["ready"].as_bool().unwrap());
        assert_eq!(actual.progress.copied_items, expected["items"].as_u64().unwrap() as usize);
        assert_eq!(actual.progress.copied_bytes,if index==5{std::mem::size_of::<Option<usize>>()}else{0});
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
    let recipient_identity = &owner.fresh_spent as *const _ as usize;
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
    assert_eq!(owner.remaining_grant().unwrap().maximum_copy_bytes,retained.grant.maximum_copy_bytes-41);
    assert_eq!(owner.remaining_work().unwrap().fuel,retained.work.fuel-1);
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
        let (step, heap) = observe(|| owner.close_step(retained,semio_framework_job::StepBudget::new(retained.work.fuel,retained.work.absolute_deadline_us,loan(body, retained.grant))).unwrap());
        assert_eq!(step.progress().copied_items, 1);
        assert_eq!(step.progress().copied_bytes, 0);
        assert_eq!(heap, (0, step.progress().released_bytes));
        released += step.progress().released_bytes;
        steps += 1;
        assert!(steps <= 16);
    }
    assert_eq!(released, births);
    assert_eq!(observe(|| drop(owner)).1, (0, 0));
}

#[path="🛬️decoder/🦀️.rs"]
mod original_decoder_tests;

#[test]
fn actor_source_treasury_binds_initial_total_while_birth_and_dispatch_stay_scoped(){
 let stock=RetainedCloneGrant{maximum_items:100000,maximum_copy_bytes:1<<24,maximum_capacity_bytes:1<<24,maximum_release_bytes:1<<24,maximum_depth:128};let policy=RetainedCloneGrant{maximum_items:50000,maximum_copy_bytes:1<<23,maximum_capacity_bytes:1<<23,maximum_release_bytes:1<<23,maximum_depth:64};let mut input=RetainedTurnInput{operation:982,generation:1,epoch:1,grant:stock,work:OriginalActorTurnWork{clock_binding:982,absolute_deadline_us:1000000,fuel:1000000}};let mut slot=OriginalActorContextSlot::empty();
 for turn in 0..32{let call=slot.admit_step(input,StepBudget::new(input.work.fuel,input.work.absolute_deadline_us,loan(policy,input.grant)),fixture_clock);let ready=call.result.unwrap().ready;assert!(call.receipt_spent.fits(policy));input=next_input(slot.context.as_ref().unwrap(),input.epoch+1);if ready{break}assert!(turn<31)}assert!(slot.is_ready());let before=slot.context.as_ref().unwrap().fresh_spent;let mut receipt=Default::default();let call=slot.dispatch_step(input,StepBudget::new(input.work.fuel,input.work.absolute_deadline_us,loan(policy,input.grant)),&mut receipt,fixture_clock,|cx|->Result<(),OriginalTurnFailure>{assert!(cx.retained_grant().maximum_capacity_bytes<=policy.maximum_capacity_bytes);cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;cx.consume_fuel(1);Ok(())});assert!(call.result.is_ok());assert_eq!(call.receipt_spent.copied_items,1);assert_eq!(slot.context.as_ref().unwrap().fresh_spent.copied_items,before.copied_items+1);input=next_input(slot.context.as_ref().unwrap(),input.epoch+1);
 for turn in 0..10000{if slot.terminal_is_empty(){break}let call=slot.close_step(input,semio_framework_job::StepBudget::new(input.work.fuel,input.work.absolute_deadline_us,input.grant));assert!(call.result.is_ok());if let Some(context)=slot.context.as_ref(){input=next_input(context,input.epoch+1)}assert!(turn<9999)}assert!(slot.terminal_is_empty());
}

#[test]
fn bootstrap_header_continues_exact_ready_epoch_without_rebegin_or_losing_birth_debt(){let mut input=RetainedTurnInput{operation:983,generation:1,epoch:1,grant:RetainedCloneGrant{maximum_items:100000,maximum_copy_bytes:1<<24,maximum_capacity_bytes:1<<24,maximum_release_bytes:1<<24,maximum_depth:128},work:OriginalActorTurnWork{clock_binding:983,absolute_deadline_us:1000000,fuel:1000000}};let mut slot=OriginalActorContextSlot::empty();assert_eq!(slot.bootstrap_phase(),0);for _ in 0..16{let call=slot.admit_step(input,StepBudget::new(input.work.fuel,input.work.absolute_deadline_us,input.grant),fixture_clock);assert!(call.result.is_ok());let posted=slot.record_bootstrap_effects(input,RetainedCloneProgress{copied_items:1,copied_bytes:8,..Default::default()},1);posted.0.unwrap();posted.1.unwrap();if slot.is_ready(){break}input=next_input(slot.context.as_ref().unwrap(),input.epoch+1);}assert!(slot.is_ready());assert_eq!(slot.bootstrap_phase(),4);let before=slot.context.as_ref().unwrap().fresh_spent;let mut physical=Default::default();let mut policy=input.grant;policy.maximum_items=128;let call=slot.dispatch_pending_step(input,StepBudget::new(1000,input.work.absolute_deadline_us,policy),&mut physical,fixture_clock,|cx|->Result<(),OriginalTurnFailure>{cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:1,..Default::default()})?;cx.consume_fuel(1);Ok(())});assert!(call.result.is_ok());assert_eq!(call.receipt_spent.copied_items,1);assert!(call.work_units_spent>=3);assert_eq!(slot.context.as_ref().unwrap().fresh_spent.copied_items,before.copied_items+1);let call=slot.dispatch_step(input,StepBudget::new(1,input.work.absolute_deadline_us,policy),&mut Default::default(),fixture_clock,|_|Ok::<_,OriginalTurnFailure>(()));assert!(call.result.is_err());input=next_input(slot.context.as_ref().unwrap(),input.epoch+1);for turn in 0..10000{if slot.terminal_is_empty(){break}assert!(slot.close_step(input,semio_framework_job::StepBudget::new(input.work.fuel,input.work.absolute_deadline_us,input.grant)).result.is_ok());if let Some(c)=slot.context.as_ref(){input=next_input(c,input.epoch+1)}assert!(turn<9999)}assert!(slot.terminal_is_empty());}

#[test]fn actor_source_birth_respects_actual_scoped_work_before_moving_its_parents(){let mut input=RetainedTurnInput{operation:984,generation:1,epoch:1,grant:RetainedCloneGrant{maximum_items:100000,maximum_copy_bytes:1<<24,maximum_capacity_bytes:1<<24,maximum_release_bytes:1<<24,maximum_depth:128},work:OriginalActorTurnWork{clock_binding:984,absolute_deadline_us:1000000,fuel:1000000}};let mut owner=OriginalActorContext::new((),input).unwrap();for _ in 0..2{let(admission,heap)=observe(||owner.admit_step(input,StepBudget::new(input.work.fuel,input.work.absolute_deadline_us,input.grant),fixture_clock).unwrap());assert!(!admission.ready);assert_eq!(heap,(admission.progress.retained_capacity_bytes,0));input=next_input(&owner,input.epoch+1)}assert!(owner.cancel.is_some()&&owner.payload.is_some()&&owner.source.is_none());let required=OriginalCallerSourceParent::admission_work_demand()+1;for fuel in[0,required-1]{let before=(owner.cancel.as_ref().unwrap()as*const _,owner.payload.as_ref().unwrap()as*const _);let(admission,heap)=observe(||owner.admit_step(input,StepBudget::new(fuel,input.work.absolute_deadline_us,input.grant),fixture_clock).unwrap());assert!(!admission.ready);assert_eq!(admission.progress,Default::default());assert_eq!(heap,(0,0));assert_eq!(before,(owner.cancel.as_ref().unwrap()as*const _,owner.payload.as_ref().unwrap()as*const _));assert!(owner.source.is_none()&&owner.admission_actual.is_none());}let(admission,heap)=observe(||owner.admit_step(input,StepBudget::new(required,input.work.absolute_deadline_us,input.grant),fixture_clock).unwrap());assert!(!admission.ready);assert_eq!(heap,(admission.progress.retained_capacity_bytes,0));assert_eq!(owner.work_units_spent,u128::from(required));assert!(owner.admission_actual.is_none());input=next_input(&owner,input.epoch+1);assert!(owner.admit_step(input,StepBudget::new(1,input.work.absolute_deadline_us,input.grant),fixture_clock).unwrap().ready);owner.take_input().unwrap().unwrap();input=next_input(&owner,input.epoch+1);for turn in 0..100000{if owner.terminal_is_empty(){break}let step=owner.close_step(input,semio_framework_job::StepBudget::new(input.work.fuel,input.work.absolute_deadline_us,input.grant)).unwrap();assert!(step.progress().fits(input.grant));input=next_input(&owner,input.epoch+1);assert!(turn<99999)}assert!(owner.terminal_is_empty());}

#[test]fn closure_error_graph_retains_whole_blocked_and_receipt_causes_until_paid_source_retirement(){use semio_framework_async::CancelTokenRetirementBlocked;for index in 0..5{let(mut donor,seed)=observe(||{let message=||ValueError::new(ValueRefusalKind::InvalidValue,"complete original closure cause 源");Some(match index{0=>OriginalActorContextError::Refused(message()),1=>OriginalActorContextError::Cancellation(CancelTokenRetirementError::Blocked(CancelTokenRetirementBlocked::RegisteredWaiter)),2=>OriginalActorContextError::Cancellation(CancelTokenRetirementError::Refused(message())),3=>OriginalActorContextError::Original(OriginalTurnFailure::from(message())),_=>OriginalActorContextError::CancellationReceipt{blocked:CancelTokenRetirementBlocked::WeakAlias,receipt:OriginalTurnFailure::from(message())}})});let address=donor.as_ref().unwrap()as*const _;let stock=RetainedCloneGrant{maximum_items:100000,maximum_copy_bytes:1<<24,maximum_capacity_bytes:1<<24,maximum_release_bytes:1<<24,maximum_depth:128};let mut caller=semio_framework_job::OriginalJobFixtureCaller::admit(OperationId(985+index),Generation(1),stock,fixture_clock,1000000,1000000);let mut owner=ManuallyDrop::new(None);let denied=RetainedCloneGrant{maximum_items:0,..stock};let(result,heap)=observe(||caller.turn(denied,|cx|cx.with_original_retirement(|cx|close_retained_source_slot(&mut donor,&mut owner,cx))));assert!(!result.unwrap());assert_eq!(heap,(0,0));assert_eq!(donor.as_ref().unwrap()as*const _,address);let mut born=seed.0;let mut freed=0;for turn in 0..100000{let(result,heap)=observe(||caller.turn(stock,|cx|cx.with_original_retirement(|cx|close_retained_source_slot(&mut donor,&mut owner,cx))));born+=heap.0;freed+=heap.1;if result.unwrap(){break}assert!(turn<99999)}assert!(donor.is_none()&&owner.is_none());assert_eq!(born,freed);caller.retire(stock);}}
