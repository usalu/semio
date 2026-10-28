//! 🧪 Terminal, diagnostic and population owners remain under the initiating original receipt until controlled close.
use super::*;
use semio_framework_plugin::app_testing::OriginalWorkFixture;

const INTEGER: ExpressExpression = ExpressExpression { kind: ExpressExpressionKind::Integer, offset: 0, text: Some("1"), operator: None, name: None, operands: &[], entries: &[] };
static BROKEN_SCALAR: ExpressExpression = ExpressExpression { kind: ExpressExpressionKind::Binary, operator: Some("+"), operands: &[INTEGER], ..INTEGER };
static UNSUPPORTED: ExpressExpression = ExpressExpression { kind: ExpressExpressionKind::Binary, operator: Some("UNSUPPORTED"), operands: &[INTEGER, INTEGER], ..INTEGER };
const EMPTY_PROGRAM: ExpressProgram = ExpressProgram { schema: "IFC4X3_ADD2", functions: &[], global_rules: &[], rules: &[], derived: &[], unique: &[], bounds: &[], inverse: &[] };
static INVALID_PROGRAM: ExpressProgram = ExpressProgram { schema: "INVALID", ..EMPTY_PROGRAM };
static REFUSAL_PROGRAM: ExpressProgram = ExpressProgram { rules: &[ExpressRule { owner: "IFCDIRECTION", label: "REFUSAL", expression: &UNSUPPORTED }], ..EMPTY_PROGRAM };

fn grant() -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items: 100_000, maximum_copy_bytes: 1_048_576, maximum_capacity_bytes: 1_048_576, maximum_release_bytes: 1_048_576, maximum_depth: 256 }
}
fn measured<'context, T>(cx: &mut StepContext<'context>, transition: impl FnOnce(&mut StepContext<'context>) -> Result<T, OriginalTurnFailure>) -> Result<T, OriginalTurnFailure> {
    let before = cx.retained_progress();
    let (answer, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| transition(cx));
    let after = cx.retained_progress();
    assert_eq!((heap.requested_bytes, heap.released_bytes), (after.retained_capacity_bytes - before.retained_capacity_bytes, after.released_bytes - before.released_bytes));
    answer
}
fn source() -> IfcSnapshot {
    IfcSnapshot {
        header: semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::IfcHeader { file_schema: vec![IfcValue::Aggregate(vec![IfcValue::String("IFC4X3_ADD2".into())])], ..Default::default() },
        entities: vec![IfcEntity { id: 1, name: "IFCDIRECTION".into(), args: vec![IfcValue::Aggregate(vec![IfcValue::Real(1.0), IfcValue::Real(0.0), IfcValue::Real(0.0)])], complex: vec![] }],
        ..Default::default()
    }
}
fn receive(source: &IfcSnapshot, mutations: &[IfcMutation], overlay: &mut PopulationOverlay, receiver: &mut PopulationReceiver, caller: &mut OriginalWorkFixture) {
    for turn in 0..10_000 {
        if caller.work_turn(grant(), 1, |cx| measured(cx, |cx| overlay.advance(source, mutations, cx))).unwrap() {
            break;
        }
        assert!(turn < 9999);
    }
    for turn in 0..10_000 {
        if caller.work_turn(grant(), 1, |cx| measured(cx, |cx| receiver.advance(overlay, source, mutations, cx))).unwrap() {
            return;
        }
        assert!(turn < 9999);
    }
}
fn close(machine: &mut RetainedMachine, receiver: &mut PopulationReceiver, overlay: &mut PopulationOverlay, caller: &mut OriginalWorkFixture) {
    machine.begin_close();
    for turn in 0..100_000 {
        if machine.terminal_is_empty() {
            break;
        }
        let demand = machine.close_demands(4096).unwrap();
        if demand.release_bytes > 0 {
            let denied = RetainedCloneGrant { maximum_release_bytes: demand.release_bytes - 1, ..grant() };
            caller.turn(denied, |cx| cx.with_original_retirement(|cx| measured(cx, |cx| semio_framework_job::close_original_retained_step(cx, demand, |_| panic!("underfunded retirement entered the machine"))))).unwrap();
            assert!(!machine.terminal_is_empty());
        }
        caller.turn(grant(), |cx| cx.with_original_retirement(|cx| measured(cx, |cx| semio_framework_job::close_original_retained_step(cx, demand, |loan| machine.close_step(loan))))).unwrap();
        assert!(turn < 99_999);
    }
    assert!(machine.terminal_is_empty());
    receiver.begin_close();
    while !receiver.terminal_is_empty() {
        caller.turn(grant(), |cx| cx.with_original_retirement(|cx| measured(cx, |cx| semio_framework_job::close_original_retained_step(cx, receiver.close_demands(4096)?, |loan| receiver.close_step(loan))))).unwrap();
    }
    overlay.begin_close();
    while !overlay.terminal_is_empty() {
        caller.turn(grant(), |cx| cx.with_original_retirement(|cx| measured(cx, |cx| semio_framework_job::close_original_retained_step(cx, overlay.close_demands(4096)?, |loan| overlay.close_step(loan))))).unwrap();
    }
    caller.finish();
}
#[test]
fn failed_population_initialization_is_terminal_and_keeps_its_single_borrowed_diagnostic() {
    let contract: serde_json::Value = serde_json::from_str(include_str!("machine-terminal.json")).unwrap();
    let mut caller = OriginalWorkFixture::with_identity(semio_framework_job::OperationId(928), semio_framework_job::Generation(1));
    let mut machine = RetainedMachine::new(&INVALID_PROGRAM);
    assert!(caller.work_turn(grant(), 1, |cx| measured(cx, |cx| machine.start_population(cx))).unwrap());
    let env = MachineEnvironment::empty();
    let mut done = false;
    for turn in 0..100 {
        if caller.work_turn(grant(), 1, |cx| measured(cx, |cx| machine.advance(&env, cx))).unwrap() {
            done = true;
            break;
        }
        assert!(turn < 99, "failed initialization kept restarting");
    }
    assert!(done);
    let report = machine.report().unwrap();
    assert!(!report.complete && !report.valid);
    assert_eq!(report.diagnostics.len(), contract["cases"][0]["diagnostics"].as_u64().unwrap() as usize);
    assert_eq!(report.diagnostics[0].code, contract["cases"][0]["code"].as_str());
    let owner = report.diagnostics.as_ptr();
    for _ in 0..3 {
        caller
            .work_turn(grant(), 1, |cx| {
                let before = cx.retained_progress();
                assert!(measured(cx, |cx| machine.advance(&env, cx))?);
                assert_eq!(before, cx.retained_progress());
                Ok(())
            })
            .unwrap();
        assert_eq!(machine.report().unwrap().diagnostics.as_ptr(), owner);
        assert_eq!(machine.report().unwrap().diagnostics.len(), 1);
    }
    assert!(machine.result_scalar(&env).is_err());
    close(&mut machine, &mut PopulationReceiver::default(), &mut PopulationOverlay::default(), &mut caller);
}
#[test]
fn failed_scalar_continuation_refuses_every_retry_without_consuming_or_exposing_partial_results() {
    let mut caller = OriginalWorkFixture::with_identity(semio_framework_job::OperationId(928), semio_framework_job::Generation(2));
    let mut machine = RetainedMachine::new(&EMPTY_PROGRAM);
    assert!(caller.work_turn(grant(), 1, |cx| measured(cx, |cx| machine.start(&BROKEN_SCALAR, cx))).unwrap());
    let env = MachineEnvironment::empty();
    let mut refused = false;
    for _ in 0..100 {
        if caller.work_turn(grant(), 1, |cx| measured(cx, |cx| machine.advance(&env, cx))).is_err() {
            refused = true;
            break;
        }
    }
    assert!(refused);
    for _ in 0..3 {
        caller
            .work_turn(grant(), 1, |cx| {
                let before = cx.retained_progress();
                assert!(measured(cx, |cx| machine.advance(&env, cx)).is_err());
                assert_eq!(before, cx.retained_progress());
                Ok(())
            })
            .unwrap();
        assert!(machine.result_scalar(&env).is_err());
        assert!(machine.report().is_none());
    }
    close(&mut machine, &mut PopulationReceiver::default(), &mut PopulationOverlay::default(), &mut caller);
}
#[test]
fn population_refusal_and_every_cancelled_prefix_keep_original_owners_and_all_five_loan_axes() {
    let source = source();
    let foreign = source.clone();
    let mut complete_prefix = None;
    for cut in 0..10_000 {
        let mut caller = OriginalWorkFixture::with_identity(semio_framework_job::OperationId(929), semio_framework_job::Generation(cut + 1));
        let mut overlay = PopulationOverlay::default();
        let mut receiver = PopulationReceiver::default();
        receive(&source, &[], &mut overlay, &mut receiver, &mut caller);
        let env = MachineEnvironment { source: Some(&source), overlay: Some(&overlay), receiver: Some(&receiver), mutations: &[] };
        let other = MachineEnvironment { source: Some(&foreign), ..env };
        let mut machine = RetainedMachine::new(&REFUSAL_PROGRAM);
        assert!(caller.work_turn(grant(), 1, |cx| measured(cx, |cx| machine.start_population(cx))).unwrap());
        let mut finished = false;
        for _ in 0..cut {
            let demand = machine.demands().unwrap();
            for axis in 0..5 {
                let mut loan = grant();
                match axis {
                    0 => loan.maximum_items = 0,
                    1 if demand.copy_bytes > 0 => loan.maximum_copy_bytes = demand.copy_bytes - 1,
                    2 if demand.capacity_bytes > 0 => loan.maximum_capacity_bytes = demand.capacity_bytes - 1,
                    3 if demand.release_bytes > 0 => loan.maximum_release_bytes = demand.release_bytes - 1,
                    4 if demand.depth > 0 => loan.maximum_depth = demand.depth - 1,
                    _ => continue,
                }
                caller
                    .work_turn(loan, 1, |cx| {
                        let before = cx.retained_progress();
                        assert!(!measured(cx, |cx| machine.advance(&env, cx))?);
                        assert_eq!(before, cx.retained_progress());
                        Ok(())
                    })
                    .unwrap();
            }
            caller
                .work_turn(grant(), 1, |cx| {
                    let before = cx.retained_progress();
                    assert!(measured(cx, |cx| machine.advance(&other, cx)).is_err());
                    assert_eq!(before, cx.retained_progress());
                    Ok(())
                })
                .unwrap();
            if caller.work_turn(grant(), 1, |cx| measured(cx, |cx| machine.advance(&env, cx))).unwrap() {
                finished = true;
                break;
            }
        }
        if finished {
            complete_prefix = Some(cut);
            let report = machine.report().unwrap();
            assert!(!report.complete && !report.valid);
            assert_eq!(report.diagnostics.len(), 1);
            assert_eq!(report.diagnostics[0].code, Some("express.unsupported-operator"));
            let owner = report.diagnostics.as_ptr();
            caller
                .work_turn(grant(), 1, |cx| {
                    let before = cx.retained_progress();
                    assert!(measured(cx, |cx| machine.advance(&env, cx))?);
                    assert_eq!(before, cx.retained_progress());
                    Ok(())
                })
                .unwrap();
            assert_eq!(machine.report().unwrap().diagnostics.as_ptr(), owner);
            assert!(machine.result_scalar(&env).is_err());
        } else {
            caller.cancel().cancel_now();
            caller
                .work_turn(grant(), 1, |cx| {
                    let before = cx.retained_progress();
                    assert!(measured(cx, |cx| machine.advance(&env, cx)).is_err());
                    assert_eq!(before, cx.retained_progress());
                    Ok(())
                })
                .unwrap();
            assert!(machine.report().is_none());
        }
        close(&mut machine, &mut receiver, &mut overlay, &mut caller);
        if finished {
            break;
        }
    }
    assert!(complete_prefix.is_some());
}

#[test]
fn canonical_completed_population_pins_the_original_mutation_owner_and_never_redebits_terminal_results() {
    use semio_s_artifact_stdio_ifc_4_mutations::standards::v4::subsets::any::schema::mutations::set_entity_arg::SetEntityArg;
    let source = source();
    let mutations = vec![IfcMutation::SetEntityArg(SetEntityArg { id: 1, index: 0, value: IfcValue::Aggregate(vec![IfcValue::Real(0.0), IfcValue::Real(1.0), IfcValue::Real(0.0)]) })];
    let foreign = mutations.clone();
    let mut caller = OriginalWorkFixture::with_identity(semio_framework_job::OperationId(930), semio_framework_job::Generation(1));
    let mut overlay = PopulationOverlay::default();
    let mut receiver = PopulationReceiver::default();
    receive(&source, &mutations, &mut overlay, &mut receiver, &mut caller);
    let env = MachineEnvironment { source: Some(&source), overlay: Some(&overlay), receiver: Some(&receiver), mutations: &mutations };
    let other = MachineEnvironment { mutations: &foreign, ..env };
    let mut machine = RetainedMachine::default();
    assert!(caller.work_turn(grant(), 1, |cx| measured(cx, |cx| machine.start_population(cx))).unwrap());
    let mut finished = false;
    for turn in 0..100_000 {
        assert!(machine.report().is_none());
        caller
            .work_turn(grant(), 1, |cx| {
                let before = cx.retained_progress();
                assert!(measured(cx, |cx| machine.advance(&other, cx)).is_err());
                assert_eq!(before, cx.retained_progress());
                Ok(())
            })
            .unwrap();
        if caller.work_turn(grant(), 1, |cx| measured(cx, |cx| machine.advance(&env, cx))).unwrap() {
            finished = true;
            break;
        }
        assert!(turn < 99_999);
    }
    assert!(finished);
    let report = machine.report().unwrap();
    assert!(report.complete && report.valid);
    assert!(report.diagnostics.is_empty());
    assert!(report.results.iter().any(|row| row.owner == Some("IFCDIRECTION") && row.rule == Some("MAGNITUDEGREATERZERO")));
    let records = report.results.as_ptr();
    let work = report.work;
    for _ in 0..3 {
        caller
            .work_turn(grant(), 1, |cx| {
                let before = cx.retained_progress();
                assert!(measured(cx, |cx| machine.advance(&env, cx))?);
                assert_eq!(before, cx.retained_progress());
                Ok(())
            })
            .unwrap();
        assert_eq!(machine.report().unwrap().results.as_ptr(), records);
        assert_eq!(machine.report().unwrap().work, work);
    }
    caller.cancel().cancel_now();
    caller
        .work_turn(grant(), 1, |cx| {
            let before = cx.retained_progress();
            assert!(measured(cx, |cx| machine.advance(&env, cx)).is_err());
            assert_eq!(before, cx.retained_progress());
            Ok(())
        })
        .unwrap();
    close(&mut machine, &mut receiver, &mut overlay, &mut caller);
}
