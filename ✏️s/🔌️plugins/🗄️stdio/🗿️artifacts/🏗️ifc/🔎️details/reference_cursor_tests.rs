//! 🧪️ Actual reference metadata and original input custody obey denied loans and cancellation.
use super::super::super::DetailsTestProfile;
use super::*;
use semio_framework_plugin::app_testing::OriginalWorkFixture;
fn grant() -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 65_536, maximum_depth: 256, ..Default::default() }
}
fn dsl(value: &serde_json::Value) -> DslValue {
    let parsed = semio_framework_pack_json::parse(&value.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    semio_framework_pack_json::to_dsl_value(&parsed)
}
fn fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("🧪️tests/reference-reduction.json")).unwrap()
}
fn admit(caller: &mut OriginalWorkFixture, address: &mut pointer::PathAdmission, event: &editing::SnapshotEditEvent, snapshot: &IfcSnapshot) {
    for turn in 0..100_000 {
        if let Some(accepted) = caller.work_turn(grant(), 1, |cx| address.advance(event, snapshot, |_, _| None, cx)).unwrap() {
            assert!(accepted);
            return;
        }
        assert!(turn < 99_999);
    }
}
fn close(caller: &mut OriginalWorkFixture, cursor: &mut ReferenceReduction, address: &mut pointer::PathAdmission) {
    cursor.begin_close();
    address.begin_close();
    while !cursor.terminal_is_empty() {
        caller
            .turn(grant(), |cx| {
                cx.with_original_retirement(|cx| {
                    semio_framework_job::close_original_retained_step(cx, cursor.close_demands(), |grant| cursor.close_step(grant))?;
                    Ok(())
                })
            })
            .unwrap();
    }
    while !address.terminal_is_empty() {
        caller
            .turn(grant(), |cx| {
                cx.with_original_retirement(|cx| {
                    semio_framework_job::close_original_retained_step(cx, address.close_demands(), |grant| address.close_step(grant))?;
                    Ok(())
                })
            })
            .unwrap();
    }
}
#[test]
fn original_reference_every_prefix_cancels_before_request_and_retains_original_input() {
    let fixture = fixture();
    let snapshot = IfcSnapshot::from_value(dsl(&fixture["snapshot"])).unwrap();
    for name in ["Append Unique Reference", "Set Exact Maximum Identity", "Set Constituent Reference", "Keep Reference Inside Legal Aggregate Alias"] {
        let case = fixture["cases"].as_array().unwrap().iter().find(|case| case["name"] == name).unwrap();
        let event = editing::SnapshotEditEvent::SetValue { path: case["path"].as_str().unwrap().into(), value: dsl(&case["value"]) };
        let original = protocol::OpBinary::encode_op(&event).unwrap();
        let mut turns = 0;
        for cancel_at in std::iter::once(None).chain((1..100_000).map(Some)) {
            if cancel_at.is_some_and(|turn| turn > turns) {
                break;
            }
            let mut caller = OriginalWorkFixture::new();
            let mut address = pointer::PathAdmission::default();
            admit(&mut caller, &mut address, &event, &snapshot);
            let mut cursor = ReferenceReduction::default();
            for turn in 1..100_000 {
                if cancel_at == Some(turn) {
                    caller.cancel().cancel_now();
                }
                let result = caller.work_turn(grant(), 1, |cx| cursor.advance::<DetailsTestProfile>(&event, &snapshot, &address, cx));
                assert_eq!(protocol::OpBinary::encode_op(&event).unwrap(), original);
                if cancel_at == Some(turn) {
                    assert!(result.is_err());
                    break;
                }
                if result.unwrap() {
                    assert!(cursor.request.is_some());
                    turns = turn;
                    break;
                }
                assert!(turn < 99_999);
            }
            caller.cancel().cancel_now();
            close(&mut caller, &mut cursor, &mut address);
            caller.finish();
        }
        assert!(turns > 20);
    }
}
#[test]
fn original_reference_grant_axes_pin_rebinding_and_close_are_source_owned() {
    let fixture = fixture();
    let snapshot = IfcSnapshot::from_value(dsl(&fixture["snapshot"])).unwrap();
    let event = editing::SnapshotEditEvent::SetValue { path: "/entities/0/args/0/appendReference".into(), value: DslValue::String("4".into()) };
    for axis in ["items", "copyBytes", "capacityBytes", "releaseBytes", "depth", "fuel"] {
        let mut caller = OriginalWorkFixture::new();
        let mut address = pointer::PathAdmission::default();
        admit(&mut caller, &mut address, &event, &snapshot);
        let mut cursor = ReferenceReduction::default();
        for turn in 0..100_000 {
            let demand = cursor.demands();
            let mut loan = grant();
            let mut fuel = 1;
            let short = match axis {
                "items" => {
                    loan.maximum_items = 0;
                    true
                }
                "copyBytes" => {
                    loan.maximum_copy_bytes = demand.copy_bytes - 1;
                    true
                }
                "capacityBytes" | "releaseBytes" => false,
                "depth" => {
                    loan.maximum_depth = 0;
                    true
                }
                "fuel" => {
                    fuel = 0;
                    true
                }
                _ => unreachable!(),
            };
            let state = format!("{cursor:?}");
            let (done, before, after, heap) = caller
                .work_turn(loan, fuel, |cx| {
                    let before = cx.retained_progress();
                    let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| cursor.advance::<DetailsTestProfile>(&event, &snapshot, &address, cx));
                    Ok((result?, before, cx.retained_progress(), heap))
                })
                .unwrap();
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
            assert!(!heap.overflowed);
            if short {
                assert!(!done);
                assert_eq!(before, after);
                assert_eq!(format!("{cursor:?}"), state);
                assert_eq!(cursor.demands(), demand);
            }
            if done || short && caller.work_turn(grant(), 1, |cx| cursor.advance::<DetailsTestProfile>(&event, &snapshot, &address, cx)).unwrap() {
                break;
            }
            assert!(turn < 99_999);
        }
        assert_eq!(cursor.request.unwrap().id, Some(4));
        let copied = event.clone();
        assert!(caller.work_turn(grant(), 1, |cx| cursor.advance::<DetailsTestProfile>(&copied, &snapshot, &address, cx)).is_err());
        let copied = snapshot.clone();
        assert!(caller.work_turn(grant(), 1, |cx| cursor.advance::<DetailsTestProfile>(&event, &copied, &address, cx)).is_err());
        caller.cancel().cancel_now();
        cursor.begin_close();
        let original = cursor.binding;
        let demand = cursor.close_demands();
        let denied = RetainedCloneGrant { maximum_copy_bytes: demand.copy_bytes - 1, ..grant() };
        caller
            .turn(denied, |cx| {
                cx.with_original_retirement(|cx| {
                    let before = cx.retained_progress();
                    let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| semio_framework_job::close_original_retained_step(cx, demand, |grant| cursor.close_step(grant)));
                    result?;
                    assert_eq!(before, cx.retained_progress());
                    assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
                    Ok(())
                })
            })
            .unwrap();
        assert_eq!(cursor.binding, original);
        close(&mut caller, &mut cursor, &mut address);
        caller.finish();
    }
}
