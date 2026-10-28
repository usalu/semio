//! 🧪️ Original declaration metadata retains source identity and refuses unfunded transitions.
use super::super::super::DetailsTestProfile;
use super::*;
use semio_framework_plugin::app_testing::OriginalWorkFixture;
fn grant() -> semio_framework_value::retained_clone::RetainedCloneGrant {
    semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 1_048_576, maximum_capacity_bytes: 1_048_576, maximum_release_bytes: 1_048_576, maximum_depth: 256 }
}
#[test]
fn original_domain_metadata_denies_each_axis_then_resumes_without_heap_and_rejects_substitution() {
    let contract: serde_json::Value = serde_json::from_str(include_str!("🧪️tests/domain-metadata.json")).unwrap();
    for row in contract["names"].as_array().unwrap() {
        let mut bytes = [0; 256];
        assert_eq!(canonical_name(row["source"].as_str().unwrap(), &mut bytes), row["canonical"].as_str());
    }
    assert!(canonical_name(&"a".repeat(257), &mut [0; 256]).is_none());
    for row in contract["membership"].as_array().unwrap() {
        let mut membership = super::super::domain_cursor::TypeMembership::default();
        let catalog = DetailsTestProfile::catalog();
        let expected = catalog.types.get_key_value(row["expected"].as_str().unwrap()).map(|(name, _)| name.as_str()).or_else(|| catalog.entities.get_key_value(row["expected"].as_str().unwrap()).map(|(name, _)| name.as_str())).unwrap();
        let actual = catalog.types.get_key_value(row["actual"].as_str().unwrap()).map(|(name, _)| name.as_str()).or_else(|| catalog.entities.get_key_value(row["actual"].as_str().unwrap()).map(|(name, _)| name.as_str())).unwrap();
        membership.start(expected, actual);
        for turn in 0..10000 {
            let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| membership.advance(catalog));
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
            if let Some(result) = result.unwrap() {
                assert_eq!(result, row["admitted"].as_bool().unwrap());
                break;
            }
            assert!(turn < 9999);
        }
    }
    let original: serde_json::Value = serde_json::from_str(include_str!("🧪️tests/constituent-value.json")).unwrap();
    let parsed = semio_framework_pack_json::parse(&original["snapshot"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let snapshot = IfcSnapshot::from_value(semio_framework_pack_json::to_dsl_value(&parsed)).unwrap();
    let event = editing::SnapshotEditEvent::SetValue { path: "/entities/0/complex/0/args/0/value/1/value".into(), value: DslValue::Number(semio_framework_value::Number::Int(9)) };
    let mut caller = OriginalWorkFixture::new();
    for row in contract["addresses"].as_array().unwrap() {
        let event = editing::SnapshotEditEvent::SetValue { path: row["path"].as_str().unwrap().into(), value: DslValue::Null };
        let mut address = pointer::PathAdmission::default();
        for turn in 0..10000 {
            if caller.work_turn(grant(), 1, |cx| address.advance(&event, &snapshot, |_, _| None, cx)).unwrap().is_some() {
                break;
            }
            assert!(turn < 9999);
        }
        let native = address.native_address(&event).unwrap();
        assert_eq!(native.position, row["position"].as_u64().unwrap() as usize);
        assert_eq!(native.part, Some(row["part"].as_u64().unwrap() as usize));
        assert_eq!(native.argument, row["argument"].as_u64().unwrap() as usize);
        assert_eq!(&row["path"].as_str().unwrap()[native.prefix..], row["suffix"].as_str().unwrap());
        address.begin_close();
        while !address.terminal_is_empty() {
            let demand = address.close_demands();
            caller.turn(grant(), |cx| cx.with_original_retirement(|cx| semio_framework_job::close_original_retained_step(cx, demand, |grant| address.close_step(grant)))).unwrap();
        }
    }
    let mut address = pointer::PathAdmission::default();
    for turn in 0..10000 {
        if caller.work_turn(grant(), 1, |cx| address.advance(&event, &snapshot, |_, _| None, cx)).unwrap().is_some() {
            break;
        }
        assert!(turn < 9999);
    }
    let mut complex = snapshot.entities[0].complex.clone();
    complex[0].args[0] = IfcValue::Aggregate(vec![IfcValue::Real(0.0), IfcValue::Real(9.0)]);
    let mutations = vec![IfcMutation::SetEntityComplex(semio_s_artifact_stdio_ifc_4_mutations::standards::v4::subsets::any::schema::mutations::set_entity_complex::SetEntityComplex { id: 1, complex })];
    let mut work = DomainWork::default();
    for turn in 0..10000 {
        let demand = work.demands();
        for axis in 0..4 {
            let mut denied = grant();
            let mut fuel = 1;
            match axis {
                0 => denied.maximum_items = 0,
                1 => denied.maximum_copy_bytes = demand.copy_bytes - 1,
                2 => denied.maximum_depth = 0,
                _ => fuel = 0,
            }
            let before = work.metadata();
            caller
                .work_turn(denied, fuel, |cx| {
                    let receipt = cx.retained_progress();
                    assert!(!work.advance::<DetailsTestProfile>(&event, &snapshot, &mutations, &address, None, cx)?);
                    assert_eq!(receipt, cx.retained_progress());
                    Ok(())
                })
                .unwrap();
            assert_eq!(before, work.metadata());
        }
        let done = caller
            .work_turn(grant(), 1, |cx| {
                let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| work.advance::<DetailsTestProfile>(&event, &snapshot, &mutations, &address, None, cx));
                assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
                result
            })
            .unwrap();
        if turn == 0 {
            let other = snapshot.clone();
            assert!(caller.work_turn(grant(), 1, |cx| work.advance::<DetailsTestProfile>(&event, &other, &mutations, &address, None, cx)).is_err());
            let other = mutations.clone();
            assert!(caller.work_turn(grant(), 1, |cx| work.advance::<DetailsTestProfile>(&event, &snapshot, &other, &address, None, cx)).is_err());
            let other = event.clone();
            assert!(caller.work_turn(grant(), 1, |cx| work.advance::<DetailsTestProfile>(&other, &snapshot, &mutations, &address, None, cx)).is_err());
            let other = pointer::PathAdmission::default();
            assert!(caller.work_turn(grant(), 1, |cx| work.advance::<DetailsTestProfile>(&event, &snapshot, &mutations, &other, None, cx)).is_err());
        }
        if done {
            break;
        }
        assert!(turn < 9999);
    }
    caller.cancel().cancel_now();
    let before = work.metadata();
    assert!(caller.work_turn(grant(), 1, |cx| work.advance::<DetailsTestProfile>(&event, &snapshot, &mutations, &address, None, cx)).is_err());
    assert_eq!(before, work.metadata());
    work.begin_close();
    let demand = work.close_demands();
    let before = work.metadata();
    let denied = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_copy_bytes: demand.copy_bytes - 1, ..grant() };
    caller.turn(denied, |cx| cx.with_original_retirement(|cx| semio_framework_job::close_original_retained_step(cx, demand, |grant| work.close_step(grant)))).unwrap();
    assert_eq!(before, work.metadata());
    caller.turn(grant(), |cx| cx.with_original_retirement(|cx| semio_framework_job::close_original_retained_step(cx, demand, |grant| work.close_step(grant)))).unwrap();
    assert!(work.terminal_is_empty());
    address.begin_close();
    while !address.terminal_is_empty() {
        let demand = address.close_demands();
        caller.turn(grant(), |cx| cx.with_original_retirement(|cx| semio_framework_job::close_original_retained_step(cx, demand, |grant| address.close_step(grant)))).unwrap();
    }
    caller.finish();
}
