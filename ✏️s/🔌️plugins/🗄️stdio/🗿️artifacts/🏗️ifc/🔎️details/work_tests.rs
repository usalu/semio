//! 🧪️ Real retained IFC work branches preserve captured graph, original control and owned close.
use super::super::super::{DetailsTestEditor, DetailsTestProfile, details_test_command};
use super::*;
use protocol::ColdMutation;
use semio_framework_job::StepContext;
use semio_framework_plugin::app_testing::OriginalWorkFixture;
use semio_framework_plugin::retained_command::ArtifactCommandSnapshotRead;
use semio_framework_plugin::{AppOperationContext, HistoryView, NoConfig};
type Work = IfcDetailsWork<DetailsTestEditor, DetailsTestProfile>;
#[path="reference_tests.rs"]mod reference_tests;
#[path="constituent_value_tests.rs"]mod constituent_value_tests;
#[path="entity_removal_tests.rs"]mod entity_removal_tests;
#[path="argument_insertion_tests.rs"]mod argument_insertion_tests;
#[path="constituent_move_tests.rs"]mod constituent_move_tests;
#[path="constituent_roster_removal_tests.rs"]mod constituent_roster_removal_tests;
#[path="constituent_zero_slot_type_tests.rs"]mod constituent_zero_slot_type_tests;
#[path="primary_zero_slot_type_tests.rs"]mod primary_zero_slot_type_tests;
#[path="zero_slot_constituent_insertion_tests.rs"]mod zero_slot_constituent_insertion_tests;
#[path="normalization_identity_tests.rs"]mod normalization_identity_tests;
#[path="equal_arity_type_tests.rs"]mod equal_arity_type_tests;
#[path="primary_equal_arity_type_tests.rs"]mod primary_equal_arity_type_tests;
#[path="constituent_optional_shape_tests.rs"]mod constituent_optional_shape_tests;
#[path="primary_optional_shape_tests.rs"]mod primary_optional_shape_tests;
#[path="constituent_remap_tests.rs"]mod constituent_remap_tests;
#[path="primary_remap_tests.rs"]mod primary_remap_tests;
#[path="type_input_tests.rs"]mod type_input_tests;
#[path="candidate_rule_tests.rs"]mod candidate_rule_tests;
#[path="type_number_binary_input_tests.rs"]mod type_number_binary_input_tests;
#[path="source_loan_tests.rs"]mod source_loan_tests;
#[path="type_guidance_request_tests.rs"]mod type_guidance_request_tests;
#[path="type_guidance_candidate_tests.rs"]mod type_guidance_candidate_tests;
#[path="type_number_binary_guidance_tests.rs"]mod type_number_binary_guidance_tests;
#[path="type_list_input_tests.rs"]mod type_list_input_tests;
#[path="type_select_input_tests.rs"]mod type_select_input_tests;
#[path="type_reference_select_input_tests.rs"]mod type_reference_select_input_tests;
#[path="type_mixed_select_input_tests.rs"]mod type_mixed_select_input_tests;
#[path="type_set_select_input_tests.rs"]mod type_set_select_input_tests;
#[path="type_measure_select_input_tests.rs"]mod type_measure_select_input_tests;
fn grant() -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items: 1_000_000, maximum_copy_bytes: 1_048_576, maximum_capacity_bytes: 1_048_576, maximum_release_bytes: 1_048_576, maximum_depth: 256 }
}
fn turn<T>(caller: &mut OriginalWorkFixture, fuel: u64, cancelled: bool, run: impl FnOnce(&mut StepContext<'_>) -> Result<T, semio_framework_job::OriginalTurnFailure>) -> Result<T, semio_framework_job::OriginalTurnFailure> {
    if cancelled {
        caller.cancel().cancel_now();
    }
    caller.work_turn(grant(), fuel, run)
}
fn wall(count: usize) -> Arc<IfcSnapshot> {
    let mut args = vec![IfcValue::Unset; count];
    args[0] = IfcValue::String("0123456789012345678901".into());
    if count > 2 {
        args[2] = IfcValue::String("Wall".into());
    }
    Arc::new(IfcSnapshot { entities: vec![IfcEntity { id: 1, name: "IFCWALL".into(), args, complex: Vec::new() }], ..Default::default() })
}
fn inputs<T>(
    snapshot: &Arc<IfcSnapshot>,
    event: editing::SnapshotEditEvent,
    caller: &mut OriginalWorkFixture,
    work: &mut Work,
    run: impl FnOnce(&ArtifactCommandInputs<'_, EditorApp<DetailsTestEditor>>, &mut OriginalWorkFixture, &mut Work) -> T,
) -> T {
    let mut command = details_test_command(event);
    assert!(turn(caller, 1_000_000, false, |cx| work.receive_original_command(&mut command, cx)).unwrap());
    let config = NoConfig {};
    let history = HistoryView::empty();
    let interaction = Default::default();
    let hover = Default::default();
    let operation = AppOperationContext { app_instance_id: 17, parent_document_id: "ifc-work-test".into(), operation_id: 716, generation: 3, canonical_base_revision: [0; 32], retained: grant(), authoring_seed: "ifc-work-test".into() };
    let input = ArtifactCommandInputs {
        command: &command,
        snapshot: snapshot.as_ref(),
        snapshot_owner: Some(snapshot),
        snapshot_read: ArtifactCommandSnapshotRead::unavailable(&operation),
        config: &config,
        history: &history,
        interaction: &interaction,
        hover: &hover,
        context: None,
        operation: &operation,
    };
    run(&input, caller, work)
}
fn prepare(caller: &mut OriginalWorkFixture, work: &mut Work, input: &ArtifactCommandInputs<'_, EditorApp<DetailsTestEditor>>) {
    for _ in 0..2048 {
        if work.entity.is_some() || work.value.is_some() {
            return;
        }
        let step = turn(caller, 1_000_000, false, |cx| work.step(input, cx)).unwrap();
        assert!(matches!(step, ArtifactCommandWorkStep::Pending { .. } | ArtifactCommandWorkStep::Progress { .. }));
    }
    panic!("retained IFC source preparation did not finish")
}
fn close(caller: &mut OriginalWorkFixture, work: &mut Work) {
    work.begin_close();
    for _ in 0..8192 {
        if work.terminal_is_empty() {
            return;
        }
        let step = caller.close(work, grant());
        assert!(step.is_ok());
    }
    panic!("retained IFC owned close did not drain")
}
fn finish(caller: &mut OriginalWorkFixture, work: &mut Work, input: &ArtifactCommandInputs<'_, EditorApp<DetailsTestEditor>>) -> ArtifactCommandWorkStep<EditorApp<DetailsTestEditor>> {
    for _ in 0..2048 {
        let step = turn(caller, 1_000_000, false, |cx| work.step(input, cx)).unwrap();
        if matches!(step, ArtifactCommandWorkStep::Complete(_)) {
            return step;
        }
    }
    panic!("retained IFC publication did not finish")
}
#[test]
fn original_wire_hook_builds_actual_editor_commands_before_owned_event_receiving() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("🧪️tests/command-wire.json")).unwrap();
    for row in fixture["valid"].as_array().unwrap(){
        let parsed=semio_framework_pack_json::parse(&row["EditSnapshot"]["event"].to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let event=editing::SnapshotEditEvent::from_value(semio_framework_pack_json::to_dsl_value(&parsed)).unwrap();
        let wire=protocol::OpBinary::encode_op(&details_test_command(event.clone())).unwrap();
        let original=wire.as_ptr();let mut caller=OriginalWorkFixture::new();let mut work=Work::new(event.action_id());let mut received=None;
        for _ in 0..100_000{if let Some(command)=turn(&mut caller,1,false,|cx|work.receive_original_wire(&wire,cx)).unwrap(){received=Some(command);break;}assert_eq!(wire.as_ptr(),original);}
        let mut command=received.expect("retained original wire should produce the actual editor command");
        assert_eq!(<DetailsTestEditor as editing::SnapshotEditingEditor>::snapshot_edit_event(&command),Some(&event));
        assert!(work.retains_original_wire());
        assert!(turn(&mut caller,1,false,|cx|work.receive_original_command(&mut command,cx)).unwrap());
        assert_eq!(work.original_event.as_ref(),Some(&event));
        caller.cancel().cancel_now();close(&mut caller,&mut work);assert!(!work.retains_original_wire());caller.finish();
    }
}
#[test]
fn original_event_admission_runs_before_capture_and_cancels_every_worker_frontier() {
    let snapshot = wall(9);
    let event = editing::SnapshotEditEvent::SetValue { path: "/entities/0/args/2/value".into(), value: DslValue::String("Admitted wall".into()) };
    let mut frontiers = 0;
    for cancel_at in std::iter::once(None).chain((1..=2048).map(Some)) {
        if cancel_at.is_some_and(|turn| turn > frontiers) { break; }
        let mut caller = OriginalWorkFixture::with_identity(semio_framework_job::OperationId(716), semio_framework_job::Generation(3));
        let mut work = Work::new(editing::SET_SNAPSHOT_VALUE_ACTION_ID);
        inputs(&snapshot, event.clone(), &mut caller, &mut work, |input, caller, work| {
            for turn_index in 1..=2048 {
                if cancel_at == Some(turn_index) { caller.cancel().cancel_now(); }
                let result = turn(caller, 1, false, |cx| work.step(input, cx));
                assert!(work.source.is_none() && work.pending.is_none());
                assert_eq!(snapshot.entities[0].args[2], IfcValue::String("Wall".into()));
                if cancel_at == Some(turn_index) { assert!(result.is_err()); break; }
                assert!(matches!(result.unwrap(), ArtifactCommandWorkStep::Progress { .. }));
                if work.admitted == Some(true) { frontiers = turn_index; break; }
                assert!(turn_index < 2048);
            }
            close(caller, work);
            assert!(work.terminal_is_empty());
        });
        caller.finish();
    }
    assert!(frontiers > 1);
}
#[test]
fn custom_retained_type_switch_emits_one_reversible_primitive_batch() {
    let snapshot = wall(9);
    let original = snapshot.as_ref().clone();
    let event = editing::SnapshotEditEvent::SetValue { path: "/entities/0/name".into(), value: DslValue::String("IFCDOOR".into()) };
    let mut caller = OriginalWorkFixture::with_identity(semio_framework_job::OperationId(716), semio_framework_job::Generation(3));
    let mut work = Work::new(editing::SET_SNAPSHOT_VALUE_ACTION_ID);
    inputs(&snapshot, event, &mut caller, &mut work, |input, caller, work| {
        prepare(caller, work, input);
        let step = finish(caller, work, input);
        let ArtifactCommandWorkStep::Complete(emit) = step else { panic!("prepared IFC variant did not complete") };
        assert!(emit.artifact_mutations.len() >= 2);
        assert_eq!(snapshot.as_ref(), &original);
        let mut next = original.clone();
        let mut inverses = Vec::new();
        for mutation in emit.artifact_mutations {
            inverses.splice(0..0, mutation.inverse(&next).unwrap());
            next = protocol::apply_diff_cold(mutation.diff(&next).diff(), &next).unwrap();
        }
        assert_eq!(next.entities[0].name, "IFCDOOR");
        for inverse in inverses {
            next = protocol::apply_diff_cold(inverse.diff(&next).diff(), &next).unwrap();
        }
        assert_eq!(next, original);
        close(caller, work);
        assert!(work.terminal_is_empty());
    });
    caller.finish();
}
#[test]
fn final_original_zero_fuel_and_cancel_never_emit_or_change_the_graph() {
    for (fuel, cancelled) in [(0, false), (1, false), (1_000_000, true)] {
        let snapshot = wall(9);
        let original = snapshot.as_ref().clone();
        let event = editing::SnapshotEditEvent::SetValue { path: "/entities/0/name".into(), value: DslValue::String("IFCDOOR".into()) };
        let mut caller = OriginalWorkFixture::with_identity(semio_framework_job::OperationId(716), semio_framework_job::Generation(3));
        let mut work = Work::new(editing::SET_SNAPSHOT_VALUE_ACTION_ID);
        inputs(&snapshot, event, &mut caller, &mut work, |input, caller, work| {
            prepare(caller, work, input);
            let result = turn(caller, fuel, cancelled, |cx| work.step(input, cx));
            assert!(!matches!(result, Ok(ArtifactCommandWorkStep::Complete(_))));
            assert_eq!(snapshot.as_ref(), &original);
            close(caller, work);
            assert!(work.terminal_is_empty());
        });
        caller.finish();
    }
}
#[test]
fn overcapacity_malformed_tuple_refuses_without_publication_and_closes_its_owners() {
    let snapshot = wall(1000);
    let original = snapshot.as_ref().clone();
    let event = editing::SnapshotEditEvent::SetValue { path: "/entities/0/name".into(), value: DslValue::String("IFCCARTESIANPOINT".into()) };
    let mut caller = OriginalWorkFixture::with_identity(semio_framework_job::OperationId(716), semio_framework_job::Generation(3));
    let mut work = Work::new(editing::SET_SNAPSHOT_VALUE_ACTION_ID);
    inputs(&snapshot, event, &mut caller, &mut work, |input, caller, work| {
        prepare(caller, work, input);
        assert!(turn(caller, 1_000_000, false, |cx| work.step(input, cx)).is_err());
        assert_eq!(snapshot.as_ref(), &original);
        close(caller, work);
        assert!(work.terminal_is_empty());
    });
    caller.finish();
}

#[test]
fn pending_single_argument_rerun_uses_the_retained_mutation_after_value_transfer() {
    let snapshot = wall(9);
    let original = snapshot.as_ref().clone();
    let event = editing::SnapshotEditEvent::SetValue { path: "/entities/0/args/2/value".into(), value: DslValue::String("Edited wall".into()) };
    let mut caller = OriginalWorkFixture::with_identity(semio_framework_job::OperationId(716), semio_framework_job::Generation(3));
    let mut work = Work::new(editing::SET_SNAPSHOT_VALUE_ACTION_ID);
    inputs(&snapshot, event, &mut caller, &mut work, |input, caller, work| {
        prepare(caller, work, input);
        assert!(work.value.is_some());
        let step = turn(caller, 2, false, |cx| work.step(input, cx)).unwrap();
        assert!(matches!(step, ArtifactCommandWorkStep::Pending { .. } | ArtifactCommandWorkStep::Progress { .. }));
        assert!(!work.complete);
        let step = finish(caller, work, input);
        let ArtifactCommandWorkStep::Complete(emit) = step else { panic!("pending IFC mutation did not resume") };
        assert_eq!(emit.artifact_mutations.len(), 1);
        assert!(matches!(&emit.artifact_mutations[0],IfcMutation::SetEntityArg(edit)if edit.id==1&&edit.index==2&&edit.value==IfcValue::String("Edited wall".into())));
        assert_eq!(snapshot.as_ref(), &original);
        close(caller, work);
        assert!(work.terminal_is_empty());
    });
    caller.finish();
    assert_eq!(Arc::strong_count(&snapshot), 1);
}

#[test]
fn declared_reference_inputs_and_whole_wrappers_share_target_and_set_guards() {
    let unit = |id, unit_type: &str, name: &str| IfcEntity { id, name: "IFCSIUNIT".into(), args: vec![IfcValue::Derived, IfcValue::Enum(unit_type.into()), IfcValue::Unset, IfcValue::Enum(name.into())], complex: Vec::new() };
    let snapshot = IfcSnapshot {
        entities: vec![
            IfcEntity { id: 1, name: "IFCUNITASSIGNMENT".into(), args: vec![IfcValue::Aggregate(vec![IfcValue::Reference(2), IfcValue::Reference(3)])], complex: Vec::new() },
            unit(2, "LENGTHUNIT", "METRE"),
            unit(3, "AREAUNIT", "SQUARE_METRE"),
            unit(4, "TIMEUNIT", "SECOND"),
        ],
        ..Default::default()
    };
    for (suffix, value, admitted) in [
        ("/reference", DslValue::String("3".into()), false),
        ("/value", DslValue::String("3".into()), false),
        ("/reference", DslValue::String("2".into()), true),
        ("/value", DslValue::String("4".into()), true),
        ("/value", DslValue::uint(2), false),
        ("", IfcValue::Reference(3).to_value(), false),
        ("", IfcValue::Reference(4).to_value(), true),
        ("", IfcValue::Reference(0).to_value(), false),
        ("", IfcValue::Reference(u64::MAX).to_value(), false),
        ("", IfcValue::Integer(2).to_value(), false),
    ] {
        let event = editing::SnapshotEditEvent::SetValue { path: format!("/entities/0/args/0/value/0{suffix}"), value };
        let result = normalized_reference_edit_controlled::<DetailsTestProfile>(&event, &snapshot, true, &mut || Ok(()));
        assert_eq!(result.is_ok(), admitted, "{suffix}");
        if admitted {
            assert!(result.unwrap().is_some());
        }
    }
}

#[test]
fn unfunded_contiguous_existing_body_stays_borrowed_and_cancel_refuses_before_publication() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️tests/source-owners.json")).unwrap();
    let short: RetainedCloneGrant = serde_json::from_value(fixture["shortLoan"].clone()).unwrap();
    let case = &fixture["deniedExistingBody"];
    let bytes = case["textBytes"].as_u64().unwrap() as usize;
    let mut snapshot = wall(9).as_ref().clone();
    snapshot.entities[0].args[3] = IfcValue::Aggregate(vec![IfcValue::String("x".repeat(bytes))]);
    let snapshot = Arc::new(snapshot);
    let original = match &snapshot.entities[0].args[3] {
        IfcValue::Aggregate(values) => values[0].as_str().unwrap().as_ptr(),
        _ => unreachable!(),
    };
    let event = editing::SnapshotEditEvent::SetValue { path: case["path"].as_str().unwrap().into(), value: DslValue::String("changed".into()) };
    let mut caller = OriginalWorkFixture::with_identity(semio_framework_job::OperationId(716), semio_framework_job::Generation(3));
    let mut work = Work::new(editing::SET_SNAPSHOT_VALUE_ACTION_ID);
    inputs(&snapshot, event, &mut caller, &mut work, |input, caller, work| {
        let mut denied = false;
        for _ in 0..case["maximumTurns"].as_u64().unwrap() {
            let demand = work.work_demands(input, 4096).unwrap();
            if demand.capacity_bytes == bytes {
                assert!(demand.capacity_bytes > short.maximum_capacity_bytes);
                caller
                    .work_turn(short, 10, |cx| {
                        let before = cx.retained_progress();
                        let fuel = cx.fuel_remaining();
                        assert!(matches!(work.step(input, cx)?, ArtifactCommandWorkStep::Progress { .. } | ArtifactCommandWorkStep::Pending { .. }));
                        assert_eq!(cx.retained_progress(), before);
                        assert_eq!(cx.fuel_remaining(), fuel);
                        Ok(())
                    })
                    .unwrap();
                denied = true;
                break;
            }
            assert!(!matches!(caller.work_turn(short, 100, |cx| work.step(input, cx)).unwrap(), ArtifactCommandWorkStep::Complete(_)));
        }
        assert!(denied);
        assert!(work.pending.is_none() && work.value.is_none());
        assert_eq!(work.publication.turns(), 0);
        caller.cancel().cancel_now();
        assert!(caller.work_turn(short, 100, |cx| work.step(input, cx)).is_err());
        work.begin_close();
        for _ in 0..10_000 {
            if work.terminal_is_empty() {
                break;
            }
            caller.close(work, short).unwrap();
        }
        assert!(work.terminal_is_empty());
    });
    caller.finish();
    let IfcValue::Aggregate(values) = &snapshot.entities[0].args[3] else { unreachable!() };
    assert_eq!(values[0].as_str().unwrap().as_ptr(), original);
    assert_eq!(values[0].as_str().unwrap().len(), bytes);
}

#[test]
fn whole_replacement_borrows_the_large_old_body_under_the_short_original_loan() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️tests/source-owners.json")).unwrap();
    let case = &fixture["borrowedWholeReplacement"];
    let short: RetainedCloneGrant = serde_json::from_value(fixture["shortLoan"].clone()).unwrap();
    let mut snapshot = wall(9).as_ref().clone();
    snapshot.entities[0].args[3] = IfcValue::String("x".repeat(case["textBytes"].as_u64().unwrap() as usize));
    let snapshot = Arc::new(snapshot);
    let original = snapshot.entities[0].args[3].as_str().unwrap().as_ptr();
    let event = editing::SnapshotEditEvent::SetValue { path: case["path"].as_str().unwrap().into(), value: DslValue::String(case["replacement"].as_str().unwrap().into()) };
    let mut caller = OriginalWorkFixture::with_identity(semio_framework_job::OperationId(716), semio_framework_job::Generation(3));
    let mut work = Work::new(editing::SET_SNAPSHOT_VALUE_ACTION_ID);
    inputs(&snapshot, event, &mut caller, &mut work, |input, caller, work| {
        let mut complete = None;
        for _ in 0..2_000_000 {
            let demand = work.work_demands(input, 4096).unwrap();
            assert!(demand.capacity_bytes <= short.maximum_capacity_bytes);
            if let ArtifactCommandWorkStep::Complete(emit) = caller.work_turn(short, 128, |cx| work.step(input, cx)).unwrap() {
                complete = Some(emit);
                break;
            }
        }
        let emit = complete.expect("borrowed original inverse must not require a contiguous old-body clone");
        assert!(work.value_cursor.terminal_is_empty());
        assert_eq!(snapshot.entities[0].args[3].as_str().unwrap().as_ptr(), original);
        assert!(matches!(emit.artifact_mutations.as_slice(),[IfcMutation::SetEntityArg(value)]if value.index==3&&value.value==IfcValue::String(case["replacement"].as_str().unwrap().into())));
        work.begin_close();
        for _ in 0..65536 {
            if work.terminal_is_empty() {
                break;
            }
            caller.close(work, short).unwrap();
        }
        assert!(work.terminal_is_empty());
    });
    caller.finish();
}

#[test]
fn wrong_reference_wrapper_refuses_before_receiving_a_large_payload() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️tests/source-owners.json")).unwrap();
    let case = &fixture["rejectedReferenceWrapper"];
    let snapshot = IfcSnapshot {
        entities: vec![
            IfcEntity { id: 1, name: "IFCUNITASSIGNMENT".into(), args: vec![IfcValue::Aggregate(vec![IfcValue::Reference(2)])], complex: Vec::new() },
            IfcEntity { id: 2, name: "IFCSIUNIT".into(), args: vec![IfcValue::Derived, IfcValue::Enum("LENGTHUNIT".into()), IfcValue::Unset, IfcValue::Enum("METRE".into())], complex: Vec::new() },
        ],
        ..Default::default()
    };
    let value = DslValue::Object(vec![("kind".into(), DslValue::String(case["kind"].as_str().unwrap().into())), ("value".into(), DslValue::Array(vec![DslValue::String("x".repeat(case["textBytes"].as_u64().unwrap() as usize))]))]);
    let event = editing::SnapshotEditEvent::SetValue { path: case["path"].as_str().unwrap().into(), value };
    let mut ticks = 0;
    assert!(
        normalized_reference_edit_controlled::<DetailsTestProfile>(&event, &snapshot, true, &mut || {
            ticks += 1;
            Ok(())
        })
        .is_err()
    );
    assert_eq!(ticks, case["maximumTicks"].as_u64().unwrap() as usize);
}

#[test]
fn retained_declared_values_refuse_enum_and_derived_raw_edits_without_emission() {
    for derived in [false, true] {
        let snapshot = if derived {
            Arc::new(IfcSnapshot {
                entities: vec![IfcEntity { id: 1, name: "IFCSIUNIT".into(), args: vec![IfcValue::Derived, IfcValue::Enum("LENGTHUNIT".into()), IfcValue::Unset, IfcValue::Enum("METRE".into())], complex: Vec::new() }],
                ..Default::default()
            })
        } else {
            let mut graph = wall(9).as_ref().clone();
            graph.entities[0].args[8] = IfcValue::Enum("STANDARD".into());
            Arc::new(graph)
        };
        let original = snapshot.as_ref().clone();
        let event = if derived {
            editing::SnapshotEditEvent::SetValue { path: "/entities/0/args/0".into(), value: IfcValue::Integer(1).to_value() }
        } else {
            editing::SnapshotEditEvent::SetValue { path: "/entities/0/args/8/value".into(), value: DslValue::String("BANANA".into()) }
        };
        let mut caller = OriginalWorkFixture::with_identity(semio_framework_job::OperationId(716), semio_framework_job::Generation(3));
        let mut work = Work::new(editing::SET_SNAPSHOT_VALUE_ACTION_ID);
        inputs(&snapshot, event, &mut caller, &mut work, |input, caller, work| {
            let mut refused = false;
            for _ in 0..4096 {
                match turn(caller, 1_000_000, false, |cx| work.step(input, cx)) {
                    Err(_) => {
                        refused = true;
                        break;
                    }
                    Ok(ArtifactCommandWorkStep::Complete(_)) => panic!("invalid declared value published"),
                    _ => {}
                }
            }
            assert!(refused);
            assert_eq!(snapshot.as_ref(), &original);
            close(caller, work);
            assert!(work.terminal_is_empty());
        });
        caller.finish();
    }
}
#[test]
fn retained_declared_value_edit_uses_original_callback_during_final_admission() {
    let mut observed = 0;
    for cancel in [false, true] {
        let snapshot = wall(9);
        let original = snapshot.as_ref().clone();
        let event = editing::SnapshotEditEvent::SetValue { path: "/entities/0/args/2/value".into(), value: DslValue::String("Edited under original cancellation".into()) };
        let mut caller = OriginalWorkFixture::with_identity(semio_framework_job::OperationId(716), semio_framework_job::Generation(3));
        let mut work = Work::new(editing::SET_SNAPSHOT_VALUE_ACTION_ID);
        if cancel {
            work.cancel_at_reduction_tick(observed);
        }
        inputs(&snapshot, event, &mut caller, &mut work, |input, caller, work| {
            let mut completed = false;
            let mut refused = false;
            for _ in 0..4096 {
                match turn(caller, 1_000_000, false, |cx| work.step(input, cx)) {
                    Err(_) => {
                        refused = true;
                        break;
                    }
                    Ok(ArtifactCommandWorkStep::Complete(_)) => {
                        completed = true;
                        break;
                    }
                    _ => {}
                }
            }
            if cancel {
                assert!(refused && !completed);
                assert!(caller.cancel().is_cancelled());
            } else {
                assert!(completed);
                observed = work.reduction_ticks();
                assert!(observed > 32);
            }
            assert_eq!(snapshot.as_ref(), &original);
            close(caller, work);
            assert!(work.terminal_is_empty());
        });
        caller.finish();
    }
}

#[test]
fn real_retained_domain_validation_resumes_at_one_original_fuel_per_turn() {
    let snapshot = wall(9);
    let original = snapshot.as_ref().clone();
    let text = "Unicode 😀 declaration validation across many original turns";
    let event = editing::SnapshotEditEvent::SetValue { path: "/entities/0/args/2/value".into(), value: DslValue::String(text.into()) };
    let mut caller = OriginalWorkFixture::with_identity(semio_framework_job::OperationId(716), semio_framework_job::Generation(3));
    let mut work = Work::new(editing::SET_SNAPSHOT_VALUE_ACTION_ID);
    inputs(&snapshot, event, &mut caller, &mut work, |input, caller, work| {
        for _ in 0..4096 {
            if work.pending.is_some() {
                break;
            }
            let step = turn(caller, 1_000_000, false, |cx| work.step(input, cx)).unwrap();
            assert!(!matches!(step, ArtifactCommandWorkStep::Complete(_)));
        }
        assert!(work.pending.is_some());
        let mut turns = 0;
        while !work.domain_validated {
            turns += 1;
            assert!(turns < 1024);
            let step = turn(caller, 1, false, |cx| work.step(input, cx)).unwrap();
            assert!(!matches!(step, ArtifactCommandWorkStep::Complete(_)));
        }
        assert!(turns > text.chars().count());
        assert!(turns < text.chars().count() + 64);
        assert_eq!(snapshot.as_ref(), &original);
        let ArtifactCommandWorkStep::Complete(emit) = finish(caller, work, input) else { panic!("resumed domain did not publish") };
        assert_eq!(emit.artifact_mutations.len(), 1);
        let mutation = &emit.artifact_mutations[0];
        let inverse = mutation.inverse(&original).unwrap();
        let mut next = protocol::apply_diff_cold(mutation.diff(&original).diff(), &original).unwrap();
        for mutation in inverse {
            next = protocol::apply_diff_cold(mutation.diff(&next).diff(), &next).unwrap();
        }
        assert_eq!(next, original);
        close(caller, work);
        assert!(work.terminal_is_empty());
    });
    caller.finish();
}

#[path="type_value_select_input_tests.rs"]
mod type_value_select_input_tests;
