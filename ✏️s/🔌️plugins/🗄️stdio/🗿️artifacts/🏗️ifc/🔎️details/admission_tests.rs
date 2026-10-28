//! 🧪️ Actual retained admission preserves cold byte limits and original cancellation/close owners.
use super::*;
use semio_framework_plugin::app_testing::OriginalWorkFixture;
use semio_framework_value::retained_clone::RetainedCloneClose;
fn grant() -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32768, maximum_capacity_bytes: 262144, maximum_release_bytes: 33554432, maximum_depth: 256 }
}
fn close(caller: &mut OriginalWorkFixture, admission: &mut EventAdmission, event: &mut Option<editing::SnapshotEditEvent>) {
    admission.begin_close();
    for _ in 0..1_000_000 {
        if admission.terminal_is_empty() {
            break;
        }
        caller
            .turn(grant(), |cx| {
                cx.with_original_retirement(|cx| {
                    let demand = admission.close_demands()?;
                    semio_framework_job::close_original_retained_step(cx, demand, |grant| admission.close_step(grant))?;
                    Ok(())
                })
            })
            .unwrap();
    }
    assert!(admission.terminal_is_empty());
    let mut retirement = RetainedCloneClose::default();
    for _ in 0..1_000_000 {
        if event.is_none() && retirement.is_empty() {
            return;
        }
        caller
            .turn(grant(), |cx| {
                cx.with_original_retirement(|cx| {
                    let demand = retirement.next_owner_demands::<editing::SnapshotEditEvent>(event.is_some(), &None, 4096)?;
                    semio_framework_job::close_original_retained_step(cx, demand, |grant| {
                        if !retirement.is_empty() { retirement.step_granted(grant) } else { retirement.begin_granted(event, grant)?.ok_or_else(|| invalid("admission original event owner absent")) }
                    })?;
                    Ok(())
                })
            })
            .unwrap();
    }
    panic!("original event admission close did not drain")
}
fn run(event: editing::SnapshotEditEvent, cancel_at: Option<usize>, refused: bool) -> (usize, usize) {
    let mut event = Some(event);
    let mut caller = OriginalWorkFixture::new();
    let mut admission = EventAdmission::default();
    let mut completed = false;
    let mut turns = 0;
    for turn in 1..1_000_000 {
        turns = turn;
        if cancel_at == Some(turn) {
            caller.cancel().cancel_now();
        }
        let demand = admission.demands().unwrap();
        assert!(demand.capacity_bytes <= grant().maximum_capacity_bytes);
        match caller.work_turn(grant(), 1, |cx| admission.advance(event.as_ref().unwrap(), cx)) {
            Ok(true) => {
                completed = true;
                break;
            }
            Ok(false) => {}
            Err(_) => {
                assert!(refused || cancel_at.is_some());
                break;
            }
        }
    }
    assert_eq!(completed, !refused && cancel_at.is_none());
    let bytes = admission.bytes;
    close(&mut caller, &mut admission, &mut event);
    caller.finish();
    (turns, bytes)
}
#[test]
fn neutral_event_admission_counts_exact_native_bytes_and_cancels_every_frontier() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️tests/admission.json")).unwrap();
    for (index, event) in fixture["events"].as_array().unwrap().iter().enumerate() {
        let parsed = semio_framework_pack_json::parse(&event.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let event = editing::SnapshotEditEvent::from_value(semio_framework_pack_json::to_dsl_value(&parsed)).unwrap();
        let expected = protocol::OpBinary::encode_op(&event).unwrap().len();
        assert_eq!(expected, fixture["exactBytes"][index].as_u64().unwrap() as usize);
        let (turns, bytes) = run(event.clone(), None, false);
        assert_eq!(bytes, expected);
        for turn in 1..=turns {
            run(event.clone(), Some(turn), false);
        }
    }
}
#[test]
fn native_admission_refuses_duplicate_nonfinite_node_depth_and_wire_overflow() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️tests/admission.json")).unwrap();
    let edit = |value| editing::SnapshotEditEvent::SetValue { path: "/entities/0/args/0".into(), value };
    let keys = fixture["refusals"]["duplicateObjectKeys"].as_array().unwrap();
    run(edit(DslValue::Object(keys.iter().map(|key| (key.as_str().unwrap().into(), DslValue::Null)).collect())), None, true);
    run(edit(DslValue::Number(semio_framework_value::Number::Float(f64::NAN))), None, true);
    let nodes = fixture["refusals"]["nodes"].as_u64().unwrap() as usize;
    run(edit(DslValue::Array(vec![DslValue::Null; nodes - 1])), None, true);
    let mut value = DslValue::Null;
    for _ in 0..fixture["refusals"]["depth"].as_u64().unwrap() {
        value = DslValue::Array(vec![value]);
    }
    run(edit(value), None, true);
    run(edit(DslValue::String("x".repeat(fixture["refusals"]["oversizedStringBytes"].as_u64().unwrap() as usize))), None, true);
}

fn custody(event: &editing::SnapshotEditEvent) -> (usize, usize, usize) {
    let editing::SnapshotEditEvent::SetValue { path, value: DslValue::String(value) } = event else { panic!("custody fixture must retain actual text owners") };
    (event as *const _ as usize, path.as_ptr() as usize, value.as_ptr() as usize)
}
fn inline_frontier(admission: &EventAdmission) -> ([usize; 128], [usize; 129], [bool; 129], usize, usize, usize, usize, u8, bool) {
    let value = &admission.values;
    (value.path, value.next, value.objects, value.depth, value.nodes, value.other, value.offset, value.phase as u8, value.done)
}
#[test]
fn actual_admission_grant_axes_preserve_original_custody_and_denied_frontiers() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️tests/admission.json")).unwrap();
    for axis in fixture["grantLaws"]["axes"].as_array().unwrap().iter().map(|axis| axis.as_str().unwrap()).chain(std::iter::once("fuel")) {
        let mut event = Some(editing::SnapshotEditEvent::SetValue { path: "/entities/0/args/2/value".into(), value: DslValue::String("Original Grüße 😀".into()) });
        let original = custody(event.as_ref().unwrap());
        let mut admission = EventAdmission::default();
        let mut caller = OriginalWorkFixture::new();
        let expected = protocol::OpBinary::encode_op(event.as_ref().unwrap()).unwrap().len();
        let mut denied = 0;
        let mut inline = 0;
        let mut partial = false;
        for turn in 0..100_000 {
            let demand = admission.demands().unwrap();
            let mut loan = grant();
            let (short, fuel) = match axis {
                "items" => { loan.maximum_items = 0; (true, 1) }
                "copyBytes" => { loan.maximum_copy_bytes = demand.copy_bytes.saturating_sub(1); (demand.copy_bytes != 0, 1) }
                "capacityBytes" => { loan.maximum_capacity_bytes = demand.capacity_bytes.saturating_sub(1); (demand.capacity_bytes != 0, 1) }
                "releaseBytes" => { loan.maximum_release_bytes = demand.release_bytes.saturating_sub(1); (demand.release_bytes != 0, 1) }
                "depth" => { loan.maximum_depth = demand.depth.saturating_sub(1); (demand.depth != 0, 1) }
                "fuel" => (true, 0),
                _ => unreachable!(),
            };
            let state = inline_frontier(&admission);
            let wire = (admission.bytes, admission.chunk, admission.codec.complete, admission.codec.closed, admission.done, admission.closing);
            let (ready, heap, before, after, prior_fuel, next_fuel) = caller.work_turn(loan, fuel, |cx| {
                let before = cx.retained_progress();
                let prior_fuel = cx.fuel_remaining();
                let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| admission.advance(event.as_ref().unwrap(), cx));
                Ok((result?, heap, before, cx.retained_progress(), prior_fuel, cx.fuel_remaining()))
            }).unwrap();
            assert_eq!(custody(event.as_ref().unwrap()), original);
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
            assert!(!heap.overflowed);
            if short {
                denied += 1;
                assert!(!ready);
                assert_eq!(after, before);
                assert_eq!(next_fuel, prior_fuel);
                assert_eq!(inline_frontier(&admission), state);
                assert_eq!((admission.bytes, admission.chunk, admission.codec.complete, admission.codec.closed, admission.done, admission.closing), wire);
                assert_eq!(admission.demands().unwrap(), demand);
                if axis == "copyBytes" && demand.copy_bytes == 64 { partial = true; assert_eq!(loan.maximum_copy_bytes, fixture["grantLaws"]["partialCodecCopyBytes"].as_u64().unwrap() as usize); }
                if caller.work_turn(grant(), 1, |cx| admission.advance(event.as_ref().unwrap(), cx)).unwrap() { break; }
            } else {
                inline += 1;
                assert_eq!(after.retained_capacity_bytes, before.retained_capacity_bytes);
                assert_eq!(after.released_bytes, before.released_bytes);
                if ready { break; }
            }
            assert!(turn < 99_999);
        }
        assert!(admission.done && admission.codec.closed);
        assert_eq!(admission.bytes, expected);
        match axis {
            "capacityBytes" | "releaseBytes" => assert!(inline > 0 && denied == 0),
            "copyBytes" => assert!(denied > 0 && inline > 0 && partial),
            _ => assert!(denied > 0),
        }
        caller.cancel().cancel_now();
        assert!(caller.work_turn(grant(), 1, |cx| admission.advance(event.as_ref().unwrap(), cx)).is_err());
        assert_eq!(custody(event.as_ref().unwrap()), original);
        close(&mut caller, &mut admission, &mut event);
        caller.finish();
    }
}

#[test]
fn original_event_retirement_denies_actual_nonzero_release_without_losing_payload() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️tests/admission.json")).unwrap();
    let length = fixture["grantLaws"]["originalOwnerStringBytes"].as_u64().unwrap() as usize;
    let mut event = Some(editing::SnapshotEditEvent::SetValue { path: "/entities/0/args/2/value".into(), value: DslValue::String("x".repeat(length)) });
    let mut caller = OriginalWorkFixture::new();
    let mut admission = EventAdmission::default();
    caller.cancel().cancel_now();
    assert!(caller.work_turn(grant(), 1, |cx| admission.advance(event.as_ref().unwrap(), cx)).is_err());
    admission.begin_close();
    while !admission.terminal_is_empty() {
        caller.turn(grant(), |cx| cx.with_original_retirement(|cx| { semio_framework_job::close_original_retained_step(cx, admission.close_demands()?, |grant| admission.close_step(grant))?; Ok(()) })).unwrap();
    }
    let mut retirement = RetainedCloneClose::default();
    let birth=retirement.next_owner_demands::<editing::SnapshotEditEvent>(true,&None,4096).unwrap();
    assert!(birth.capacity_bytes>0);
    let original=custody(event.as_ref().unwrap());
    let denied_birth=RetainedCloneGrant{maximum_capacity_bytes:birth.capacity_bytes-1,..grant()};
    let (heap,before,after)=caller.turn(denied_birth,|cx|cx.with_original_retirement(|cx|{
        let before=cx.retained_progress();
        let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||semio_framework_job::close_original_retained_step(cx,birth,|g|retirement.begin_granted(&mut event,g)?.ok_or_else(||invalid("original event owner absent"))));
        result?;Ok((heap,before,cx.retained_progress()))
    })).unwrap();
    assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(before,after);assert_eq!(custody(event.as_ref().unwrap()),original);assert!(retirement.is_empty());
    let mut denied = false;
    for _ in 0..100_000 {
        let demand = retirement.next_owner_demands::<editing::SnapshotEditEvent>(event.is_some(), &None, 4096).unwrap();
        if demand.release_bytes != 0 {
            let loan = RetainedCloneGrant { maximum_release_bytes: demand.release_bytes - 1, ..grant() };
            let original = event.as_ref().map(custody);
            let (heap, before, after) = caller.turn(loan, |cx| cx.with_original_retirement(|cx| {
                let before = cx.retained_progress();
                let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| semio_framework_job::close_original_retained_step(cx, demand, |grant| {
                    if retirement.is_empty() { retirement.begin_granted(&mut event, grant)?.ok_or_else(|| invalid("event owner absent")) } else { retirement.step_granted(grant) }
                }));
                result?;
                Ok((heap, before, cx.retained_progress()))
            })).unwrap();
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
            assert!(!heap.overflowed);
            assert_eq!(after, before);
            assert_eq!(event.as_ref().map(custody), original);
            assert_eq!(retirement.next_owner_demands::<editing::SnapshotEditEvent>(event.is_some(), &None, 4096).unwrap(), demand);
            denied = true;
            break;
        }
        caller.turn(grant(), |cx| cx.with_original_retirement(|cx| {
            semio_framework_job::close_original_retained_step(cx, demand, |grant| {
                if retirement.is_empty() { retirement.begin_granted(&mut event, grant)?.ok_or_else(|| invalid("event owner absent")) } else { retirement.step_granted(grant) }
            })?;
            Ok(())
        })).unwrap();
    }
    assert!(denied);
    for _ in 0..100_000 {
        if retirement.is_empty() && event.is_none() { caller.finish(); return; }
        caller.turn(grant(), |cx| cx.with_original_retirement(|cx| {
            let demand = retirement.next_owner_demands::<editing::SnapshotEditEvent>(event.is_some(), &None, 4096)?;
            semio_framework_job::close_original_retained_step(cx, demand, |grant| if retirement.is_empty() { retirement.begin_granted(&mut event, grant)?.ok_or_else(|| invalid("event owner absent")) } else { retirement.step_granted(grant) })?;
            Ok(())
        })).unwrap();
    }
    panic!("cancelled original event owner did not retire")
}
