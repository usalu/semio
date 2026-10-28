//! 🧩️ Zero-slot type changes consume the actual original variant/name payload without default generation.
use super::*;
fn contract() -> serde_json::Value {
    serde_json::from_str(include_str!("🧪️tests/constituent-zero-slot-type.json")).unwrap()
}
fn event(row: &serde_json::Value) -> editing::SnapshotEditEvent {
    let value = if let Some(name) = row["value"].as_str() { DslValue::String(name.into()) } else { DslValue::Object(vec![("$ifcType".into(), DslValue::String(row["value"]["$ifcType"].as_str().unwrap().into()))]) };
    editing::SnapshotEditEvent::SetValue { path: row["path"].as_str().unwrap().into(), value }
}
struct AlternateProfile;
impl IfcDetailsProfile for AlternateProfile {
    type CandidateRules = ();
    const MAXIMUM_ATTRIBUTES: usize = 0;
    fn catalog() -> &'static IfcCatalog {
        panic!("substitute profile must refuse before catalog access")
    }
    fn normalization_attribute(_: &str, _: usize, _: bool) -> Result<Option<IfcNormalizationAttribute>, ()> {
        Err(())
    }
    fn direct_attribute_count(_: &str) -> Option<usize> {
        None
    }
}
#[test]
fn original_zero_slot_constituent_type_rejects_substitute_profile_before_metadata_or_payload_changes() {
    let row = &contract()["cases"][0];
    let snapshot = super::constituent_roster_removal_tests::source(row);
    let mut caller = OriginalWorkFixture::new();
    let mut work = Work::new(editing::SET_SNAPSHOT_VALUE_ACTION_ID);
    inputs(&snapshot, event(row), &mut caller, &mut work, |input, caller, work| {
        for index in 0..10000 {
            if work.constituent_roster.phase() == 7 {
                break;
            }
            turn(caller, 1, false, |cx| work.step(input, cx)).unwrap();
            assert!(index < 9999);
        }
        let metadata = work.constituent_roster.metadata();
        let command = work.original_event.as_ref().unwrap().to_value();
        caller
            .work_turn(grant(), 1, |cx| {
                let receipt = cx.retained_progress();
                assert!(work.constituent_roster.advance::<AlternateProfile>(work.original_event.as_mut().unwrap(), &snapshot, &work.address, work.source.as_ref(), &mut work.parts_cursor, &mut work.parts, &mut work.pending, &mut work.value_cursor, &mut work.value, cx).is_err());
                assert_eq!(receipt, cx.retained_progress());
                Ok(())
            })
            .unwrap();
        assert_eq!(metadata, work.constituent_roster.metadata());
        assert_eq!(command, work.original_event.as_ref().unwrap().to_value());
        assert!(work.parts.is_none() && work.pending.is_none());
        close(caller, work);
        assert!(work.terminal_is_empty());
    });
    caller.finish();
}
#[test]
fn original_zero_slot_constituent_types_match_neutral_diff_inverse_and_denied_axes() {
    for row in contract()["cases"].as_array().unwrap() {
        let snapshot = super::constituent_roster_removal_tests::source(row);
        let mut caller = OriginalWorkFixture::new();
        let mut work = Work::new(editing::SET_SNAPSHOT_VALUE_ACTION_ID);
        inputs(&snapshot, event(row), &mut caller, &mut work, |input, caller, work| {
            let mut result = None;
            for _ in 0..100000 {
                let bridge = work.prepared && work.reference.complete() && work.enumeration.complete() && work.pending.is_none() && work.constituent_roster.phase() != 2;
                if bridge {
                    let demand = work.work_demands(input, 4096).unwrap();
                    for axis in 0..6 {
                        let mut denied = grant();
                        let mut fuel = 1;
                        match axis {
                            0 => denied.maximum_items = 0,
                            1 if demand.copy_bytes > 0 => denied.maximum_copy_bytes = demand.copy_bytes - 1,
                            2 if demand.capacity_bytes > 0 => denied.maximum_capacity_bytes = demand.capacity_bytes - 1,
                            3 if demand.release_bytes > 0 => denied.maximum_release_bytes = demand.release_bytes - 1,
                            4 => denied.maximum_depth = 0,
                            5 => fuel = 0,
                            _ => continue,
                        }
                        let state = work.constituent_roster.metadata();
                        let original = work.original_event.as_ref().unwrap().to_value();
                        caller
                            .work_turn(denied, fuel, |cx| {
                                let before = cx.retained_progress();
                                let step = work.step(input, cx)?;
                                assert!(!matches!(step, ArtifactCommandWorkStep::Complete(_)));
                                assert_eq!(before, cx.retained_progress());
                                Ok(())
                            })
                            .unwrap();
                        assert_eq!(state, work.constituent_roster.metadata());
                        assert_eq!(original, work.original_event.as_ref().unwrap().to_value());
                    }
                }
                let step = caller.work_turn(RetainedCloneGrant { maximum_items: 1, ..grant() }, 1, |cx| {
                    let before = cx.retained_progress();
                    let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| work.step(input, cx));
                    if bridge {
                        let after = cx.retained_progress();
                        assert_eq!((heap.requested_bytes, heap.released_bytes), (after.retained_capacity_bytes - before.retained_capacity_bytes, after.released_bytes - before.released_bytes));
                    }
                    result
                });
                match step {
                    Ok(ArtifactCommandWorkStep::Complete(emit)) => {
                        result = Some(Ok(emit));
                        break;
                    }
                    Err(error) => {
                        result = Some(Err(error));
                        break;
                    }
                    _ => {}
                }
                assert_eq!(work.reduction_ticks(), 0);
                assert!(work.entity.is_none() && work.semantic_emit.is_none());
            }
            let result = result.unwrap();
            assert_eq!(result.is_ok(), row["admitted"].as_bool().unwrap(), "{}", row["id"]);
            if let Ok(emit) = result {
                assert_eq!(emit.artifact_mutations.len(), 1);
                let mutation = &emit.artifact_mutations[0];
                let IfcMutation::SetEntityComplex(change) = mutation else { panic!("central complex primitive required") };
                let mut expected = snapshot.entities[0].complex.clone();
                expected[1].name = row["name"].as_str().unwrap().into();
                assert!(ifc_complex_identical(&change.complex, &expected));
                let editing::SnapshotEditEvent::SetValue { value, .. } = work.original_event.as_ref().unwrap() else { panic!("original command owner required") };
                let old_name = value.as_str().or_else(|| requested_type(value)).unwrap();
                assert_eq!(old_name, snapshot.entities[0].complex[1].name);
                let mut next = protocol::apply_diff_cold(mutation.diff(&snapshot).diff(), &snapshot).unwrap();
                assert!(ifc_complex_identical(&next.entities[0].complex, &expected));
                for inverse in mutation.inverse(&snapshot).unwrap() {
                    next = protocol::apply_diff_cold(inverse.diff(&next).diff(), &next).unwrap();
                }
                assert!(ifc_complex_identical(&next.entities[0].complex, &snapshot.entities[0].complex));
            }
            close(caller, work);
            assert!(work.terminal_is_empty());
        });
        caller.finish();
    }
}
#[test]
fn original_zero_slot_constituent_types_leave_template_generation_on_explicit_frontier() {
    for kind in contract()["fallback"].as_array().unwrap() {
        let mut row = serde_json::json!({"path":"/entities/0/complex/1/name","value":"IFCCURVE"});
        let mut snapshot = super::constituent_roster_removal_tests::source(&row);
        let entity = &mut Arc::get_mut(&mut snapshot).unwrap().entities[0];
        match kind.as_str().unwrap() {
            "new-own-attributes" => row["value"] = serde_json::json!("IFCCARTESIANPOINT"),
            "old-own-attributes" => entity.complex[1].name = "IFCCARTESIANPOINT".into(),
            "source-surplus-arguments" => entity.complex[1].args.push(IfcValue::Unset),
            "unknown-new-type" => row["value"] = serde_json::json!("IFCUNKNOWN"),
            "derived-survivor" => entity.complex[0].name = "IFCSIUNIT".into(),
            "global-id-root" => entity.name = "IFCROOT".into(),
            "derived-direct-value" => entity.complex[2].args[0] = IfcValue::Derived,
            "lowercase-survivor-name" => entity.complex[0].name = "IfcGeometricRepresentationItem".into(),
            _ => panic!("unknown fallback"),
        };
        let mut caller = OriginalWorkFixture::new();
        let mut work = Work::new(editing::SET_SNAPSHOT_VALUE_ACTION_ID);
        inputs(&snapshot, event(&row), &mut caller, &mut work, |input, caller, work| {
            for turn_index in 0..10000 {
                if work.constituent_roster.fallback() {
                    break;
                }
                turn(caller, 1, false, |cx| work.step(input, cx)).unwrap();
                assert!(work.entity.is_none() && work.pending.is_none());
                assert!(turn_index < 9999);
            }
            assert!(work.constituent_roster.fallback());
            assert!(work.parts.is_none());
            close(caller, work);
            assert!(work.terminal_is_empty());
        });
        caller.finish();
    }
}
#[test]
fn original_zero_slot_constituent_type_cancels_and_closes_every_transfer_prefix() {
    for row in contract()["cases"].as_array().unwrap().iter().filter(|row| matches!(row["id"].as_str(), Some("provider-variant" | "canonical-name"))) {
        let snapshot = super::constituent_roster_removal_tests::source(row);
        let mut total = 0;
        let mut caller = OriginalWorkFixture::new();
        let mut work = Work::new(editing::SET_SNAPSHOT_VALUE_ACTION_ID);
        inputs(&snapshot, event(row), &mut caller, &mut work, |input, caller, work| {
            while work.pending.is_none() {
                turn(caller, 1, false, |cx| work.step(input, cx)).unwrap();
                total += 1;
                assert!(total < 100000);
            }
            close(caller, work);
        });
        caller.finish();
        for prefix in 0..=total {
            let mut caller = OriginalWorkFixture::new();
            let mut work = Work::new(editing::SET_SNAPSHOT_VALUE_ACTION_ID);
            inputs(&snapshot, event(row), &mut caller, &mut work, |input, caller, work| {
                for _ in 0..prefix {
                    turn(caller, 1, false, |cx| work.step(input, cx)).unwrap();
                }
                assert!(turn(caller, 1, true, |cx| work.step(input, cx)).is_err());
                close(caller, work);
                assert!(work.terminal_is_empty());
            });
            caller.finish();
        }
    }
}
