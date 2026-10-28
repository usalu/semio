//! 🧪️ Retained continuation execution consumes the same static semantic cases under original work ownership.
use super::{ast::*, machine::*};
use semio_framework_plugin::app_testing::OriginalWorkFixture;
use semio_framework_value::retained_clone::RetainedCloneGrant;
include!("expression-cases.rs");
include!("statement-cases.rs");

fn grant() -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items: 100_000, maximum_copy_bytes: 1_048_576, maximum_capacity_bytes: 1_048_576, maximum_release_bytes: 1_048_576, maximum_depth: 256 }
}
fn close(machine: &mut RetainedMachine, caller: &mut OriginalWorkFixture) {
    machine.begin_close();
    for turn in 0..1_000_000 {
        if machine.terminal_is_empty() {
            return;
        }
        caller
            .turn(grant(), |cx| {
                cx.with_original_retirement(|cx| {
                    let before = cx.retained_progress();
                    let (answer, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| semio_framework_job::close_original_retained_step(cx, machine.close_demands(4096)?, |loan| machine.close_step(loan)));
                    let after = cx.retained_progress();
                    assert_eq!((heap.requested_bytes, heap.released_bytes), (after.retained_capacity_bytes - before.retained_capacity_bytes, after.released_bytes - before.released_bytes));
                    answer
                })
            })
            .unwrap();
        assert!(turn < 999_999);
    }
}
#[test]
fn retained_expression_continuations_preserve_every_shared_scalar_case() {
    for (source, expression, kind, expected) in EXPRESSION_CASES {
        let mut caller = OriginalWorkFixture::with_identity(semio_framework_job::OperationId(924), semio_framework_job::Generation(2));
        let mut machine = RetainedMachine::new(super::ifc4x3_express_program());
        assert!(caller.work_turn(grant(), 1, |cx| machine.start(expression, cx)).unwrap());
        for turn in 0..1_000_000 {
            let done = caller
                .work_turn(grant(), 1, |cx| {
                    let before = cx.retained_progress();
                    let (answer, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| machine.advance(&MachineEnvironment::empty(), cx));
                    let after = cx.retained_progress();
                    assert_eq!((heap.requested_bytes, heap.released_bytes), (after.retained_capacity_bytes - before.retained_capacity_bytes, after.released_bytes - before.released_bytes));
                    answer
                })
                .unwrap();
            if done {
                break;
            }
            assert!(turn < 999_999, "{source}");
        }
        let environment = MachineEnvironment::empty();
        let actual = machine.result_scalar(&environment).unwrap();
        assert_eq!(actual.kind(), *kind, "{source}");
        if *kind != "unknown" {
            assert_eq!(actual.text(), *expected, "{source}");
        }
        close(&mut machine, &mut caller);
        assert!(machine.terminal_is_empty());
        caller.finish();
    }
}
#[test]
fn retained_function_statement_loops_preserve_shared_scope_domains_and_results() {
    for (name, arguments, expected) in STATEMENT_CASES {
        let mut caller = OriginalWorkFixture::with_identity(semio_framework_job::OperationId(924), semio_framework_job::Generation(3));
        let mut machine = RetainedMachine::new(&STATEMENT_PROGRAM);
        assert!(caller.work_turn(grant(), 1, |cx| machine.start_call(name, arguments, cx)).unwrap());
        for turn in 0..1_000_000 {
            if caller.work_turn(grant(), 1, |cx| machine.advance(&MachineEnvironment::empty(), cx)).unwrap() {
                break;
            }
            assert!(turn < 999_999);
        }
        assert_eq!(machine.result_scalar(&MachineEnvironment::empty()).unwrap().integer(), Some(*expected), "{name}");
        close(&mut machine, &mut caller);
        caller.finish();
    }
}

#[test]
fn every_function_continuation_cancel_prefix_retires_the_same_original_machine() {
    let (name, arguments, _) = STATEMENT_CASES[0];
    let mut completed_turns = None;
    for prefix in 0..10_000 {
        let mut caller = OriginalWorkFixture::with_identity(semio_framework_job::OperationId(926), semio_framework_job::Generation(prefix as u64 + 1));
        let mut machine = RetainedMachine::new(&STATEMENT_PROGRAM);
        assert!(caller.work_turn(grant(), 1, |cx| machine.start_call(name, arguments, cx)).unwrap());
        let mut finished = false;
        for _ in 0..prefix {
            if caller.work_turn(grant(), 1, |cx| machine.advance(&MachineEnvironment::empty(), cx)).unwrap() {
                finished = true;
                break;
            }
        }
        if finished {
            completed_turns = Some(prefix);
        } else {
            caller.cancel().cancel_now();
            caller
                .work_turn(grant(), 1, |cx| {
                    let before = cx.retained_progress();
                    let (answer, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| machine.advance(&MachineEnvironment::empty(), cx));
                    assert!(answer.is_err());
                    assert_eq!(before, cx.retained_progress());
                    assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
                    Ok(())
                })
                .unwrap();
        }
        close(&mut machine, &mut caller);
        assert!(machine.terminal_is_empty());
        caller.finish();
        if completed_turns.is_some() {
            break;
        }
    }
    assert!(completed_turns.is_some());
}

#[test]
fn every_actual_growth_and_continuation_declines_all_insufficient_original_axes() {
    let (name, arguments, expected) = STATEMENT_CASES[4];
    let mut caller = OriginalWorkFixture::with_identity(semio_framework_job::OperationId(927), semio_framework_job::Generation(1));
    let mut machine = RetainedMachine::new(&STATEMENT_PROGRAM);
    assert!(caller.work_turn(grant(), 1, |cx| machine.start_call(name, arguments, cx)).unwrap());
    for turn in 0..10_000 {
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
                    let (answer, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| machine.advance(&MachineEnvironment::empty(), cx));
                    assert!(!answer?);
                    assert_eq!(before, cx.retained_progress());
                    assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
                    Ok(())
                })
                .unwrap();
        }
        if caller.work_turn(grant(), 1, |cx| machine.advance(&MachineEnvironment::empty(), cx)).unwrap() {
            break;
        }
        assert!(turn < 9999);
    }
    assert_eq!(machine.result_scalar(&MachineEnvironment::empty()).unwrap().integer(), Some(expected));
    close(&mut machine, &mut caller);
    caller.finish();
}

#[test]
fn immutable_program_indices_and_execution_contract_keep_native_execution_unclaimed() {
    let contract: serde_json::Value = serde_json::from_str(include_str!("machine-cases.json")).unwrap();
    assert_eq!(contract["execution"], false);
    assert!(!super::IFC4X3_NATIVE_EXPRESS_EXECUTION);
    assert!(!super::IFC4X3_RETAINED_EXPRESS_IMPLEMENTATION);
    let program = super::ifc4x3_express_program();
    for (name, count) in [
        ("where", program.rules.len()),
        ("derived", program.derived.len()),
        ("inverse", program.inverse.len()),
        ("bounds", program.bounds.len()),
        ("unique", program.unique.len()),
        ("functions", program.functions.len()),
        ("globals", program.global_rules.len()),
    ] {
        assert_eq!(contract["counts"][name].as_u64().unwrap() as usize, count);
    }
}
