//! 🧪 Both actual aggregate operands determine result kind, uniqueness and retained matching custody.
use super::*;
use semio_framework_plugin::app_testing::OriginalWorkFixture;

const EMPTY: ExpressExpression = ExpressExpression { kind: ExpressExpressionKind::Unknown, offset: 0, text: None, operator: None, name: None, operands: &[], entries: &[] };
const ONE: ExpressExpression = ExpressExpression { kind: ExpressExpressionKind::Integer, text: Some("1"), ..EMPTY };
macro_rules! text {
    ($text:literal) => {
        ExpressExpression { kind: ExpressExpressionKind::String, text: Some($text), ..EMPTY }
    };
}
macro_rules! call {
    ($name:literal $(,$operand:expr)*) => { ExpressExpression { kind: ExpressExpressionKind::Call, operands: &[ExpressExpression { kind: ExpressExpressionKind::Identifier, name: Some($name), ..EMPTY }, $($operand),*], ..EMPTY } };
}
macro_rules! binary {
    ($left:expr,$operator:literal,$right:expr) => {
        ExpressExpression { kind: ExpressExpressionKind::Binary, operator: Some($operator), operands: &[$left, $right], ..EMPTY }
    };
}
macro_rules! items {
    ($($value:expr),*) => { ExpressExpression { kind: ExpressExpressionKind::Aggregate, entries: &[$(ExpressAggregateEntry { value: &$value, repeat: None }),*], ..EMPTY } };
}
macro_rules! function {
    ($name:literal,$result:literal,$value:expr) => {
        ExpressFunction {
            name: $name,
            parameters: &[],
            result_type: $result,
            locals: &[],
            body: &[ExpressStatement { kind: ExpressStatementKind::Return, offset: 0, name: None, expressions: &[$value], body: &[], otherwise: &[], cases: &[], while_condition: None, until_condition: None }],
        }
    };
}
static PROGRAM: ExpressProgram = ExpressProgram {
    schema: "IFC4X3_ADD2",
    functions: &[
        function!("BAGVALUES", "BAG OF STRING", items!(text!("x"), text!("x"), text!("y"))),
        function!("OTHERBAG", "BAG OF STRING", items!(text!("x"), text!("x"), text!("z"))),
        function!("SETVALUES", "SET OF STRING", items!(text!("x"), text!("z"))),
        function!("OTHERSET", "SET OF STRING", items!(text!("x"), text!("y"))),
    ],
    global_rules: &[],
    rules: &[],
    derived: &[],
    unique: &[],
    bounds: &[],
    inverse: &[],
};
static CASES: &[(&str, ExpressExpression)] = &[
    ("scalar-set-union", binary!(text!("INTEGER"), "+", call!("TYPEOF", ONE))),
    ("set-scalar-union", binary!(call!("TYPEOF", ONE), "+", text!("INTEGER"))),
    ("bag-set-intersection", binary!(call!("BAGVALUES"), "*", call!("SETVALUES"))),
    ("set-bag-intersection", binary!(call!("SETVALUES"), "*", call!("BAGVALUES"))),
    ("mixed-intersection-nested-union", binary!(binary!(call!("BAGVALUES"), "*", call!("SETVALUES")), "+", items!(text!("x")))),
    ("bag-bag-intersection", binary!(call!("BAGVALUES"), "*", call!("OTHERBAG"))),
    ("bag-intersection-nested-union", binary!(binary!(call!("BAGVALUES"), "*", call!("OTHERBAG")), "+", items!(text!("x")))),
    ("set-set-intersection", binary!(call!("SETVALUES"), "*", call!("OTHERSET"))),
    ("set-bag-union", binary!(call!("SETVALUES"), "+", call!("BAGVALUES"))),
    ("bag-set-union", binary!(call!("BAGVALUES"), "+", call!("SETVALUES"))),
];
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
fn close(machine: &mut RetainedMachine, caller: &mut OriginalWorkFixture) {
    machine.begin_close();
    for turn in 0..100_000 {
        if machine.terminal_is_empty() {
            break;
        }
        let demand = machine.close_demands(4096).unwrap();
        if demand.release_bytes > 0 {
            let denied = RetainedCloneGrant { maximum_release_bytes: demand.release_bytes - 1, ..grant() };
            caller.turn(denied, |cx| cx.with_original_retirement(|cx| measured(cx, |cx| semio_framework_job::close_original_retained_step(cx, demand, |_| panic!("denied aggregate retirement advanced"))))).unwrap();
            assert!(!machine.terminal_is_empty());
        }
        caller.turn(grant(), |cx| cx.with_original_retirement(|cx| measured(cx, |cx| semio_framework_job::close_original_retained_step(cx, demand, |loan| machine.close_step(loan))))).unwrap();
        assert!(turn < 99_999);
    }
    assert!(machine.terminal_is_empty());
    caller.finish();
}
#[test]
fn retained_aggregate_result_kind_and_values_match_every_neutral_operand_case_under_all_five_axes() {
    let contract: serde_json::Value = serde_json::from_str(include_str!("aggregate-kind-cases.json")).unwrap();
    assert_eq!(contract["nativeExecution"], false);
    assert_eq!(CASES.len(), contract["cases"].as_array().unwrap().len());
    for (index, (name, expression)) in CASES.iter().enumerate() {
        let expected = &contract["cases"][index];
        assert_eq!(expected["id"], *name);
        let mut caller = OriginalWorkFixture::with_identity(semio_framework_job::OperationId(931), semio_framework_job::Generation(index as u64 + 1));
        let mut machine = RetainedMachine::new(&PROGRAM);
        assert!(caller.work_turn(grant(), 1, |cx| measured(cx, |cx| machine.start(expression, cx))).unwrap());
        let env = MachineEnvironment::empty();
        let mut done = false;
        for turn in 0..100_000 {
            let demand = machine.demands().unwrap();
            for axis in 0..5 {
                let mut denied = grant();
                match axis {
                    0 => denied.maximum_items = 0,
                    1 if demand.copy_bytes > 0 => denied.maximum_copy_bytes = demand.copy_bytes - 1,
                    2 if demand.capacity_bytes > 0 => denied.maximum_capacity_bytes = demand.capacity_bytes - 1,
                    3 if demand.release_bytes > 0 => denied.maximum_release_bytes = demand.release_bytes - 1,
                    4 if demand.depth > 0 => denied.maximum_depth = demand.depth - 1,
                    _ => continue,
                }
                caller
                    .work_turn(denied, 1, |cx| {
                        let before = cx.retained_progress();
                        assert!(!measured(cx, |cx| machine.advance(&env, cx))?);
                        assert_eq!(before, cx.retained_progress());
                        Ok(())
                    })
                    .unwrap();
            }
            if caller.work_turn(grant(), 1, |cx| measured(cx, |cx| machine.advance(&env, cx))).unwrap() {
                done = true;
                break;
            }
            assert!(turn < 99_999);
        }
        assert!(done);
        let handle = *machine.store().results.last().unwrap();
        let Raw::Aggregate { kind, cursor, .. } = machine.raw(handle, &env).unwrap() else { panic!("aggregate result required") };
        assert_eq!(
            expected["aggregate"],
            match kind {
                AggregateKind::Set => "SET",
                AggregateKind::Bag => "BAG",
                _ => panic!("wrong aggregate category"),
            },
            "{name}"
        );
        let mut next = cursor.next;
        let mut actual = Vec::new();
        while let Some(index) = next {
            let edge = machine.store().edges[index];
            next = edge.next;
            let Raw::String(value) = machine.raw(edge.value, &env).unwrap() else { panic!("string member required") };
            actual.push(machine.text(value, &env).unwrap().to_string());
        }
        actual.sort();
        assert_eq!(serde_json::to_value(actual).unwrap(), expected["values"], "{name}");
        close(&mut machine, &mut caller);
    }
}
#[test]
fn both_corrected_aggregate_paths_keep_every_original_cancelled_prefix_for_controlled_close() {
    for index in [0, 4] {
        let mut completed = false;
        for prefix in 0..10_000 {
            let mut caller = OriginalWorkFixture::with_identity(semio_framework_job::OperationId(932), semio_framework_job::Generation(prefix + 1));
            let mut machine = RetainedMachine::new(&PROGRAM);
            assert!(caller.work_turn(grant(), 1, |cx| measured(cx, |cx| machine.start(&CASES[index].1, cx))).unwrap());
            for _ in 0..prefix {
                if caller.work_turn(grant(), 1, |cx| measured(cx, |cx| machine.advance(&MachineEnvironment::empty(), cx))).unwrap() {
                    completed = true;
                    break;
                }
            }
            if !completed {
                caller.cancel().cancel_now();
                caller
                    .work_turn(grant(), 1, |cx| {
                        let before = cx.retained_progress();
                        assert!(measured(cx, |cx| machine.advance(&MachineEnvironment::empty(), cx)).is_err());
                        assert_eq!(before, cx.retained_progress());
                        Ok(())
                    })
                    .unwrap();
            }
            close(&mut machine, &mut caller);
            if completed {
                break;
            }
        }
        assert!(completed);
    }
}
