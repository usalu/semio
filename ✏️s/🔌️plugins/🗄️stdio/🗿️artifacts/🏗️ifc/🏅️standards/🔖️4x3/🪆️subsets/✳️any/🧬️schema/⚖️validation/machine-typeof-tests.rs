//! 🧪 Released SELECT closure preserves exact runtime seeds and original retained custody.
use super::*;
use semio_framework_plugin::app_testing::OriginalWorkFixture;
const EMPTY: ExpressExpression = ExpressExpression { kind: ExpressExpressionKind::Unknown, offset: 0, text: None, operator: None, name: None, operands: &[], entries: &[] };
const ZERO: ExpressExpression = ExpressExpression { kind: ExpressExpressionKind::Real, text: Some("0."), ..EMPTY };
const ONE: ExpressExpression = ExpressExpression { kind: ExpressExpressionKind::Real, text: Some("1."), ..EMPTY };
macro_rules! text {
    ($value:literal) => {
        ExpressExpression { kind: ExpressExpressionKind::String, text: Some($value), ..EMPTY }
    };
}
macro_rules! integer {
    ($value:literal) => {
        ExpressExpression { kind: ExpressExpressionKind::Integer, text: Some($value), ..EMPTY }
    };
}
macro_rules! call { ($name:literal $(,$operand:expr)*) => { ExpressExpression { kind: ExpressExpressionKind::Call, operands: &[ExpressExpression { kind: ExpressExpressionKind::Identifier, name: Some($name), ..EMPTY }, $($operand),*], ..EMPTY } }; }
macro_rules! items { ($($value:expr),*) => { ExpressExpression { kind: ExpressExpressionKind::Aggregate, entries: &[$(ExpressAggregateEntry { value: &$value, repeat: None }),*], ..EMPTY } }; }
macro_rules! contains {
    ($name:literal,$value:expr) => {
        ExpressExpression { kind: ExpressExpressionKind::Binary, operator: Some("IN"), operands: &[text!($name), call!("TYPEOF", $value)], ..EMPTY }
    };
}
const COLOUR: ExpressExpression = call!("IFCCOLOURRGB", EMPTY, ONE, ZERO, ZERO);
const IDS: &[&str] =
    &["entity-select", "transitive-select", "axis-select", "defined-select", "nested-defined-select", "untyped-string", "untyped-real", "measure-select", "unrelated-select", "unknown", "no-inferred-subtype", "aggregate-alias", "qualified-seed"];
static EXPRESSIONS: &[ExpressExpression] = &[
    contains!("IFC4X3_ADD2.IFCCOLOUR", COLOUR),
    contains!("IFC4X3_ADD2.IFCFILLSTYLESELECT", COLOUR),
    contains!("IFC4X3_ADD2.IFCAXIS2PLACEMENT", call!("IFCAXIS2PLACEMENT3D", call!("IFCCARTESIANPOINT", items!(ZERO, ZERO, ZERO)), EMPTY, EMPTY)),
    contains!("IFC4X3_ADD2.IFCSIMPLEVALUE", call!("IFCLABEL", text!("value"))),
    contains!("IFC4X3_ADD2.IFCVALUE", call!("IFCLABEL", text!("value"))),
    contains!("IFC4X3_ADD2.IFCSIMPLEVALUE", text!("value")),
    contains!("IFC4X3_ADD2.IFCMEASUREVALUE", ONE),
    contains!("IFC4X3_ADD2.IFCMEASUREVALUE", call!("IFCLENGTHMEASURE", ONE)),
    contains!("IFC4X3_ADD2.IFCAXIS2PLACEMENT", COLOUR),
    contains!("IFC4X3_ADD2.IFCCOLOUR", EMPTY),
    contains!("IFC4X3_ADD2.IFCPOSITIVELENGTHMEASURE", call!("IFCLENGTHMEASURE", ONE)),
    contains!("LIST", call!("IFCCOMPOUNDPLANEANGLEMEASURE", items!(integer!("1"), integer!("2"), integer!("3")))),
    contains!("IFC4X3_ADD2.IFCCOLOUR", ExpressExpression { kind: ExpressExpressionKind::Qualify, name: Some("IFCPRESENTATIONITEM"), operands: &[COLOUR], ..EMPTY }),
];
fn grant() -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items: 100_000, maximum_copy_bytes: 1_048_576, maximum_capacity_bytes: 1_048_576, maximum_release_bytes: 1_048_576, maximum_depth: 256 }
}
fn measured<'context, T>(cx: &mut StepContext<'context>, work: impl FnOnce(&mut StepContext<'context>) -> Result<T, OriginalTurnFailure>) -> Result<T, OriginalTurnFailure> {
    let before = cx.retained_progress();
    let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| work(cx));
    let after = cx.retained_progress();
    assert_eq!((heap.requested_bytes, heap.released_bytes), (after.retained_capacity_bytes - before.retained_capacity_bytes, after.released_bytes - before.released_bytes));
    result
}
fn advance(machine: &mut RetainedMachine, env: &MachineEnvironment<'_>, caller: &mut OriginalWorkFixture) -> bool {
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
                assert!(!measured(cx, |cx| machine.advance(env, cx))?);
                assert_eq!(before, cx.retained_progress());
                Ok(())
            })
            .unwrap();
    }
    caller.work_turn(grant(), 1, |cx| measured(cx, |cx| machine.advance(env, cx))).unwrap()
}
fn close(machine: &mut RetainedMachine, receiver: &mut PopulationReceiver, overlay: &mut PopulationOverlay, caller: &mut OriginalWorkFixture) {
    machine.begin_close();
    for turn in 0..1_000_000 {
        if machine.terminal_is_empty() {
            break;
        }
        let demand = machine.close_demands(4096).unwrap();
        if demand.release_bytes > 0 {
            caller
                .turn(RetainedCloneGrant { maximum_release_bytes: demand.release_bytes - 1, ..grant() }, |cx| {
                    cx.with_original_retirement(|cx| measured(cx, |cx| semio_framework_job::close_original_retained_step(cx, demand, |_| panic!("denied SELECT close advanced"))))
                })
                .unwrap();
        }
        caller.turn(grant(), |cx| cx.with_original_retirement(|cx| measured(cx, |cx| semio_framework_job::close_original_retained_step(cx, demand, |loan| machine.close_step(loan))))).unwrap();
        assert!(turn < 999_999);
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
fn native_typeof_closes_only_actual_runtime_seeds_over_released_selects_under_every_funding_axis() {
    let contract: serde_json::Value = serde_json::from_str(include_str!("typeof-select-cases.json")).unwrap();
    assert_eq!(EXPRESSIONS.len(), contract["expressions"].as_array().unwrap().len());
    for (index, expression) in EXPRESSIONS.iter().enumerate() {
        assert_eq!(contract["expressions"][index]["id"], IDS[index]);
        let mut caller = OriginalWorkFixture::with_identity(semio_framework_job::OperationId(933), semio_framework_job::Generation(index as u64 + 1));
        let mut machine = RetainedMachine::default();
        assert!(caller.work_turn(grant(), 1, |cx| measured(cx, |cx| machine.start(expression, cx))).unwrap());
        let env = MachineEnvironment::empty();
        let mut done = false;
        for turn in 0..1_000_000 {
            if advance(&mut machine, &env, &mut caller) {
                done = true;
                break;
            }
            assert!(turn < 999_999);
        }
        assert!(done);
        assert_eq!(machine.result_scalar(&env).unwrap().text(), contract["expressions"][index]["expected"].as_bool().unwrap().to_string(), "{}", contract["expressions"][index]["id"]);
        close(&mut machine, &mut PopulationReceiver::default(), &mut PopulationOverlay::default(), &mut caller);
    }
}
fn source(count: usize) -> IfcSnapshot {
    IfcSnapshot {
        header: semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::IfcHeader { file_schema: vec![IfcValue::Aggregate(vec![IfcValue::String("IFC4X3_ADD2".into())])], ..Default::default() },
        entities: vec![
            IfcEntity { id: 1, name: "IFCCOLOURRGB".into(), args: vec![IfcValue::Unset, IfcValue::Real(1.), IfcValue::Real(0.), IfcValue::Real(0.)], complex: vec![] },
            IfcEntity { id: 2, name: "IFCCOLOURRGB".into(), args: vec![IfcValue::Unset, IfcValue::Real(0.), IfcValue::Real(1.), IfcValue::Real(0.)], complex: vec![] },
            IfcEntity { id: 3, name: "IFCFILLAREASTYLE".into(), args: vec![IfcValue::String("Colour".into()), IfcValue::Aggregate((1..=count).map(|id| IfcValue::Reference(id as _)).collect()), IfcValue::Enum("T".into())], complex: vec![] },
        ],
        ..Default::default()
    }
}
#[test]
fn original_full_population_enforces_released_max_one_colour_and_pins_both_source_models() {
    let contract: serde_json::Value = serde_json::from_str(include_str!("typeof-select-cases.json")).unwrap();
    for (index, expected) in contract["models"].as_array().unwrap().iter().enumerate() {
        let source = source(index + 1);
        let foreign = source.clone();
        let mut caller = OriginalWorkFixture::with_identity(semio_framework_job::OperationId(934), semio_framework_job::Generation(index as u64 + 1));
        let mut overlay = PopulationOverlay::default();
        let mut receiver = PopulationReceiver::default();
        for turn in 0..100_000 {
            if caller.work_turn(grant(), 1, |cx| measured(cx, |cx| overlay.advance(&source, &[], cx))).unwrap() {
                break;
            }
            assert!(turn < 99_999);
        }
        for turn in 0..100_000 {
            if caller.work_turn(grant(), 1, |cx| measured(cx, |cx| receiver.advance(&overlay, &source, &[], cx))).unwrap() {
                break;
            }
            assert!(turn < 99_999);
        }
        let env = MachineEnvironment { source: Some(&source), overlay: Some(&overlay), receiver: Some(&receiver), mutations: &[] };
        let other = MachineEnvironment { source: Some(&foreign), ..env };
        let mut machine = RetainedMachine::default();
        assert!(caller.work_turn(grant(), 1, |cx| measured(cx, |cx| machine.start_population(cx))).unwrap());
        let mut done = false;
        for turn in 0..1_000_000 {
            caller
                .work_turn(grant(), 1, |cx| {
                    let before = cx.retained_progress();
                    assert!(measured(cx, |cx| machine.advance(&other, cx)).is_err());
                    assert_eq!(before, cx.retained_progress());
                    Ok(())
                })
                .unwrap();
            if advance(&mut machine, &env, &mut caller) {
                done = true;
                break;
            }
            assert!(turn < 999_999);
        }
        assert!(done);
        let report = machine.report().unwrap();
        assert!(report.complete);
        assert_eq!(report.valid, expected["valid"].as_bool().unwrap());
        let row = report.results.iter().find(|row| row.owner == Some("IFCFILLAREASTYLE") && row.rule == Some("MAXONECOLOUR")).unwrap();
        assert_eq!(row.outcome, if expected["result"] == "true" { RetainedRuleOutcome::True } else { RetainedRuleOutcome::False });
        close(&mut machine, &mut receiver, &mut overlay, &mut caller);
    }
}
#[test]
fn select_queue_cancellation_preserves_each_traversal_stage_for_original_controlled_close() {
    for stage in 4..=8 {
        let mut caller = OriginalWorkFixture::with_identity(semio_framework_job::OperationId(935), semio_framework_job::Generation(stage as u64));
        let mut machine = RetainedMachine::default();
        assert!(caller.work_turn(grant(), 1, |cx| measured(cx, |cx| machine.start(&EXPRESSIONS[1], cx))).unwrap());
        let env = MachineEnvironment::empty();
        let mut found = false;
        for turn in 0..1_000_000 {
            if machine.store().tasks.iter().any(|frame| matches!(frame.kind, Kind::TypeOf) && frame.stage == stage) {
                found = true;
                break;
            }
            assert!(!advance(&mut machine, &env, &mut caller));
            assert!(turn < 999_999);
        }
        assert!(found);
        caller.cancel().cancel_now();
        caller
            .work_turn(grant(), 1, |cx| {
                let before = cx.retained_progress();
                assert!(measured(cx, |cx| machine.advance(&env, cx)).is_err());
                assert_eq!(before, cx.retained_progress());
                Ok(())
            })
            .unwrap();
        assert!(machine.result_scalar(&env).is_err());
        close(&mut machine, &mut PopulationReceiver::default(), &mut PopulationOverlay::default(), &mut caller);
    }
}

#[test]
fn native_typeof_static_parent_slices_match_exact_released_members_without_heap_birth() {
    for entity in metadata::ENTITIES {
        let expected: Vec<_> = metadata::TYPES.iter().filter(|declaration| declaration.kind == "select" && declaration.members.contains(&entity.key)).map(|declaration| declaration.key).collect();
        let (actual, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| metadata::select_parents(entity.key));
        assert_eq!(actual, expected.as_slice(), "{}", entity.key);
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
    }
    for declaration in metadata::TYPES {
        let expected: Vec<_> = metadata::TYPES.iter().filter(|parent| parent.kind == "select" && parent.members.contains(&declaration.key)).map(|parent| parent.key).collect();
        let (actual, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| metadata::select_parents(declaration.key));
        assert_eq!(actual, expected.as_slice(), "{}", declaration.key);
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
    }
}

#[test]
fn native_typeof_retained_queue_visits_only_original_seed_direct_parents_and_exhaustion() {
    static EXPRESSION: ExpressExpression = call!("TYPEOF", COLOUR);
    let mut caller = OriginalWorkFixture::with_identity(semio_framework_job::OperationId(936), semio_framework_job::Generation(1));
    let mut machine = RetainedMachine::default();
    let env = MachineEnvironment::empty();
    assert!(caller.work_turn(grant(), 1, |cx| measured(cx, |cx| machine.start(&EXPRESSION, cx))).unwrap());
    let mut visited = std::collections::BTreeSet::new();
    let mut done = false;
    for turn in 0..100_000 {
        if let Some(frame) = machine.store().tasks.last().filter(|frame| matches!(frame.kind, Kind::TypeOf) && frame.stage == 5) {
            let Raw::String(seed) = machine.raw(frame.a, &env).unwrap() else { panic!("TYPEOF seed must remain text") };
            let seed = machine.text(seed, &env).unwrap();
            let parents = metadata::select_parents(seed.strip_prefix("IFC4X3_ADD2.").unwrap_or(seed));
            assert_eq!(frame.c, 0);
            assert!(frame.position <= parents.len(), "{} parent {} exceeds {}", seed, frame.position, parents.len());
            visited.insert((frame.d, frame.position));
        }
        if advance(&mut machine, &env, &mut caller) {
            done = true;
            break;
        }
        assert!(turn < 99_999);
    }
    assert!(done);
    let result = *machine.store().results.last().unwrap();
    let Raw::Aggregate { mut cursor, .. } = machine.raw(result, &env).unwrap() else { panic!("TYPEOF must remain a SET") };
    let mut expected = 0;
    let mut names = std::collections::BTreeSet::new();
    while let Some(edge) = cursor.next {
        let edge = machine.store().edges[edge];
        cursor.next = edge.next;
        let Raw::String(seed) = machine.raw(edge.value, &env).unwrap() else { panic!("TYPEOF result must remain text") };
        let seed = machine.text(seed, &env).unwrap();
        assert!(names.insert(seed));
        expected += metadata::select_parents(seed.strip_prefix("IFC4X3_ADD2.").unwrap_or(seed)).len() + 1;
    }
    assert!(names.contains("IFC4X3_ADD2.IFCCOLOUR"));
    assert!(names.contains("IFC4X3_ADD2.IFCFILLSTYLESELECT"));
    assert_eq!(visited.len(), expected);
    drop(names);
    close(&mut machine, &mut PopulationReceiver::default(), &mut PopulationOverlay::default(), &mut caller);
}
