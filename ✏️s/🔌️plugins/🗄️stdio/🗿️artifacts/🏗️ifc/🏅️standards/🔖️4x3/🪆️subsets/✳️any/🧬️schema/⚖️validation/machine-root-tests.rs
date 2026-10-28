//! 🪟 Prepared original source queries retain writable aliases without lending canonical fields.
use super::*;
use semio_framework_plugin::app_testing::OriginalWorkFixture;
#[path = "machine-root-fixtures.rs"]
mod fixtures;
use fixtures::*;
const EMPTY: ExpressExpression = ExpressExpression { kind: ExpressExpressionKind::Unknown, offset: 0, text: None, operator: None, name: None, operands: &[], entries: &[] };
#[test]
fn native_prepared_root_contract_keeps_execution_and_publication_unproved() {
    let data:serde_json::Value=serde_json::from_str(include_str!("root-read-cases.json")).unwrap();
    assert_eq!(data["laws"].as_array().unwrap().len(),11);
    assert_eq!(data["nativeExecutionProven"],false);
    assert!(!super::super::IFC4X3_NATIVE_EXPRESS_EXECUTION);
    assert!(!super::super::IFC4X3_RETAINED_EXPRESS_IMPLEMENTATION);
}
static CANONICAL: ExpressExpression = ExpressExpression {
    kind: ExpressExpressionKind::Index,
    operands: &[
        ExpressExpression { kind: ExpressExpressionKind::Attribute, name: Some("DIRECTIONRATIOS"), operands: &[ExpressExpression { kind: ExpressExpressionKind::Identifier, name: Some("E4"), ..EMPTY }], ..EMPTY },
        ExpressExpression { kind: ExpressExpressionKind::Integer, text: Some("1"), ..EMPTY },
    ],
    ..EMPTY
};
const BINDINGS: &[(&str, u64)] = &[("E1", 1), ("E2", 2), ("E3", 3), ("E4", 4), ("E5", 5), ("E6", 6), ("E7", 7), ("E8", 8), ("E9", 9), ("E10", 10), ("E11", 11), ("E12", 12)];
fn grant() -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items: 100_000, maximum_copy_bytes: 1_048_576, maximum_capacity_bytes: 1_048_576, maximum_release_bytes: 1_048_576, maximum_depth: 256 }
}
fn measured<'a, T>(cx: &mut StepContext<'a>, f: impl FnOnce(&mut StepContext<'a>) -> Result<T, OriginalTurnFailure>) -> Result<T, OriginalTurnFailure> {
    let before = cx.retained_progress();
    let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| f(cx));
    let after = cx.retained_progress();
    assert_eq!((heap.requested_bytes, heap.released_bytes), (after.retained_capacity_bytes - before.retained_capacity_bytes, after.released_bytes - before.released_bytes));
    result
}
fn prepare(source: &IfcSnapshot, caller: &mut OriginalWorkFixture) -> (PopulationOverlay, PopulationReceiver) {
    let mut overlay = PopulationOverlay::default();
    let mut receiver = PopulationReceiver::default();
    for _ in 0..100_000 {
        if caller.work_turn(grant(), 1, |cx| measured(cx, |cx| overlay.advance(source, &[], cx))).unwrap() {
            break;
        }
    }
    assert!(overlay.is_ready());
    for _ in 0..100_000 {
        if caller.work_turn(grant(), 1, |cx| measured(cx, |cx| receiver.advance(&overlay, source, &[], cx))).unwrap() {
            break;
        }
    }
    assert!(receiver.arena().is_some());
    (overlay, receiver)
}
fn state(machine:&RetainedMachine)->(u64,[usize;16],Vec<(usize,usize,usize,usize,usize,usize,usize)>,Vec<(usize,usize,u64)>){let s=machine.store();(machine.work,[s.values.len(),s.edges.len(),s.tasks.len(),s.results.len(),s.scopes.len(),s.bindings.len(),s.entities.len(),s.originals.len(),s.memos.len(),s.texts.len(),s.scratch.len(),s.domains.len(),s.records.len(),s.diagnostics.len(),s.views.len(),s.dependencies.len()],s.tasks.iter().map(|f|(f.kind as usize,f.stage,f.position,f.a,f.b,f.c,f.d)).collect(),s.views.iter().map(|v|(v.root,v.original,v.record.version)).collect())}
fn step(machine: &mut RetainedMachine, env: &MachineEnvironment<'_>, caller: &mut OriginalWorkFixture) -> Result<bool, OriginalTurnFailure> {
    let demand = machine.demands().unwrap();
    let stationary=state(machine);
    caller.work_turn(grant(), 0, |cx| { let before = cx.retained_progress(); assert!(!measured(cx, |cx| machine.advance(env,cx))?); assert_eq!(before,cx.retained_progress()); Ok(()) }).unwrap();
    assert_eq!(state(machine),stationary);
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
        assert_eq!(state(machine),stationary);
    }
    caller.work_turn(grant(), 1, |cx| measured(cx, |cx| machine.advance(env, cx)))
}
fn query(machine: &mut RetainedMachine, expression: &'static ExpressExpression, env: &MachineEnvironment<'_>, caller: &mut OriginalWorkFixture) -> Result<(), OriginalTurnFailure> {
    query_bound(machine, expression, BINDINGS, env, caller)
}
fn query_bound(machine: &mut RetainedMachine, expression: &'static ExpressExpression, bindings: &'static [(&'static str,u64)], env: &MachineEnvironment<'_>, caller: &mut OriginalWorkFixture) -> Result<(), OriginalTurnFailure> {
    loop {
        if caller.work_turn(grant(), 1, |cx| measured(cx, |cx| machine.test_prepared_query(expression, bindings, env, cx)))? {
            break;
        }
    }
    for _ in 0..1_000_000 {
        if step(machine, env, caller)? {
            return Ok(());
        }
    }
    panic!("root query did not terminate")
}
fn close(machine: &mut RetainedMachine, receiver: &mut PopulationReceiver, overlay: &mut PopulationOverlay, caller: &mut OriginalWorkFixture) {
    machine.begin_close();
    while !machine.terminal_is_empty() {
        caller.turn(grant(), |cx| cx.with_original_retirement(|cx| measured(cx, |cx| semio_framework_job::close_original_retained_step(cx, machine.close_demands(4096)?, |loan| machine.close_step(loan))))).unwrap();
    }
    assert!(machine.result_scalar(&MachineEnvironment::empty()).is_err());
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
fn values(machine: &RetainedMachine, handle: Handle, env: &MachineEnvironment<'_>) -> Vec<f64> {
    let Raw::Aggregate { mut cursor, .. } = machine.raw(handle, env).unwrap() else { panic!("expected aggregate") };
    let mut result = Vec::new();
    while let Some(index) = cursor.next {
        if cursor.source {
            let edge = &env.receiver.unwrap().arena().unwrap().edges[index];
            cursor.next = edge.next;
            result.push(match env.source_value(edge.value).unwrap() { IfcValue::Real(value) => *value, IfcValue::Integer(value) => *value as f64, _ => panic!("expected source number") });
        } else {
            let edge = machine.store().edges[index];
            cursor.next = edge.next;
            result.push(number(machine.raw(edge.value, env).unwrap()).unwrap());
        }
    }
    result
}
fn field(machine: &RetainedMachine, entity: Entity, name: &str) -> Handle {
    let record = machine.entity_record(entity).unwrap();
    record.fields[..record.length].iter().flatten().find(|(key, _)| key.eq_ignore_ascii_case(name)).unwrap().1
}
fn check(machine: &mut RetainedMachine, env: &MachineEnvironment<'_>, expected: &serde_json::Value) {
    let handle = *machine.store().results.last().unwrap();
    match expected["kind"].as_str().unwrap() {
        "unknown" => assert!(matches!(machine.raw(handle, env).unwrap(), Raw::Missing)),
        "vector" => {
            let Raw::Entity(entity) = machine.raw(handle, env).unwrap() else { panic!("expected vector") };
            let magnitude = number(machine.raw(field(machine, entity, "MAGNITUDE"), env).unwrap()).unwrap();
            assert_eq!(magnitude, expected["magnitude"].as_f64().unwrap());
            let Raw::Entity(direction) = machine.raw(field(machine, entity, "ORIENTATION"), env).unwrap() else { panic!("expected orientation") };
            let actual = values(machine, field(machine, direction, "DIRECTIONRATIOS"), env);
            let wanted: Vec<f64> = expected["orientation"].as_array().unwrap().iter().map(|v| v.as_f64().unwrap()).collect();
            assert_eq!(actual, wanted);
        }
        "direction" => {
            let Raw::Entity(entity) = machine.raw(handle, env).unwrap() else { panic!("expected direction") };
            assert_eq!(machine.canonical_entity(entity), Entity::Original(env.overlay.unwrap().find(4).unwrap()));
            assert_eq!(values(machine, field(machine, entity, "DIRECTIONRATIOS"), env), vec![9., 4.]);
        }
        _ => assert_eq!(machine.result_scalar(env).unwrap().text(), expected["value"].as_str().unwrap()),
    }
}
#[test]
fn native_prepared_arithmetic_reuses_all_shared_issued_cases_without_source_writes() {
    let data: serde_json::Value = serde_json::from_str(include_str!("🧪️vector-arithmetic/cases.json")).unwrap();
    assert_eq!(ARITHMETIC.len(), data["functions"].as_array().unwrap().len());
    for (index, expression) in ARITHMETIC.iter().enumerate() {
        let source = crate::io::parse_ifc4x3_step(include_str!("🧪️vector/source.ifc")).unwrap();
        let before = source.clone();
        let mut caller = OriginalWorkFixture::with_identity(semio_framework_job::OperationId(947), semio_framework_job::Generation(index as u64 + 1));
        let (mut overlay, mut receiver) = prepare(&source, &mut caller);
        let env = MachineEnvironment { source: Some(&source), overlay: Some(&overlay), receiver: Some(&receiver), mutations: &[] };
        let mut machine = RetainedMachine::default();
        query(&mut machine, expression, &env, &mut caller).unwrap();
        check(&mut machine, &env, &data["functions"][index]["expected"]);
        query(&mut machine, &CANONICAL, &env, &mut caller).unwrap();
        assert_eq!(machine.result_scalar(&env).unwrap().text(), "3");
        assert_eq!(source, before);
        close(&mut machine, &mut receiver, &mut overlay, &mut caller);
    }
}
#[test]
fn native_prepared_custom_alias_laws_keep_original_identity_and_returned_owner() {
    let data: serde_json::Value = serde_json::from_str(include_str!("🧪️vector-arithmetic/isolation.json")).unwrap();
    for (index, expression) in PROBE_EXPRESSIONS.iter().enumerate() {
        let source = crate::io::parse_ifc4x3_step(include_str!("🧪️vector/source.ifc")).unwrap();
        let before = source.clone();
        let mut caller = OriginalWorkFixture::with_identity(semio_framework_job::OperationId(948), semio_framework_job::Generation(index as u64 + 1));
        let (mut overlay, mut receiver) = prepare(&source, &mut caller);
        let env = MachineEnvironment { source: Some(&source), overlay: Some(&overlay), receiver: Some(&receiver), mutations: &[] };
        let mut machine = RetainedMachine::new(PROBES[index]);
        let result = query(&mut machine, expression, &env, &mut caller);
        if data["cases"][index]["expected"]["kind"] == "error" {
            assert_eq!(result.unwrap_err().primary.message(),"express.unsupported-call");
        } else {
            result.unwrap();
            check(&mut machine, &env, &data["cases"][index]["expected"]);
            let retained = *machine.store().results.last().unwrap();
            query(&mut machine, &CANONICAL, &env, &mut caller).unwrap();
            assert_eq!(machine.result_scalar(&env).unwrap().text(), "3");
            if data["cases"][index]["expected"]["kind"] == "direction" {
                let Raw::Entity(entity) = machine.raw(retained, &env).unwrap() else { panic!("returned entity lost") };
                assert_eq!(values(&machine, field(&machine, entity, "DIRECTIONRATIOS"), &env), vec![9., 4.]);
            }
        }
        assert_eq!(source, before);
        close(&mut machine, &mut receiver, &mut overlay, &mut caller);
    }
}
#[test]
fn native_prepared_memo_dependency_stamps_and_compound_results_match_shared_root_laws() {
    let data:serde_json::Value=serde_json::from_str(include_str!("root-memo-cases.json")).unwrap();
    for(index,expression)in MEMO_EXPRESSIONS.iter().enumerate(){
        let source=crate::io::parse_ifc4x3_step(if index==3 {include_str!("🧪️placement/source.ifc")} else {include_str!("🧪️vector/source.ifc")}).unwrap();
        let before=source.clone();
        let mut caller=OriginalWorkFixture::with_identity(semio_framework_job::OperationId(949),semio_framework_job::Generation(index as u64+1));
        let(mut overlay,mut receiver)=prepare(&source,&mut caller);
        let env=MachineEnvironment{source:Some(&source),overlay:Some(&overlay),receiver:Some(&receiver),mutations:&[]};
        let mut machine=RetainedMachine::new(MEMOS[index]);
        for _ in 0..2 {
            query_bound(&mut machine,expression,MEMO_BINDINGS[index],&env,&mut caller).unwrap();
            assert_eq!(machine.result_scalar(&env).unwrap().text(),data["cases"][index]["expected"].as_str().unwrap());
        }
        assert_eq!(source,before);
        close(&mut machine,&mut receiver,&mut overlay,&mut caller);
    }
}
#[test]
fn native_prepared_active_derived_cycle_retains_exact_failure_and_original_source() {
    static SELF_DIM:ExpressExpression=ExpressExpression{kind:ExpressExpressionKind::Attribute,name:Some("DIM"),operands:&[ExpressExpression{kind:ExpressExpressionKind::Identifier,name:Some("SELF"),..EMPTY}],..EMPTY};
    static QUERY_DIM:ExpressExpression=ExpressExpression{kind:ExpressExpressionKind::Attribute,name:Some("DIM"),operands:&[ExpressExpression{kind:ExpressExpressionKind::Identifier,name:Some("E4"),..EMPTY}],..EMPTY};
    static CYCLIC:ExpressProgram=ExpressProgram{derived:&[ExpressRule{owner:"IFCDIRECTION",label:"DIM",expression:&SELF_DIM}],..super::super::PROGRAM};
    let source=crate::io::parse_ifc4x3_step(include_str!("🧪️vector/source.ifc")).unwrap();
    let before=source.clone();
    let mut caller=OriginalWorkFixture::with_identity(semio_framework_job::OperationId(951),semio_framework_job::Generation(1));
    let(mut overlay,mut receiver)=prepare(&source,&mut caller);
    let env=MachineEnvironment{source:Some(&source),overlay:Some(&overlay),receiver:Some(&receiver),mutations:&[]};
    let mut machine=RetainedMachine::new(&CYCLIC);
    assert_eq!(query(&mut machine,&QUERY_DIM,&env,&mut caller).unwrap_err().primary.message(),"express.derived-cycle");
    assert_eq!(source,before);
    close(&mut machine,&mut receiver,&mut overlay,&mut caller);
}
#[test]
fn native_prepared_every_negative_root_prefix_cancels_stationary_and_pays_close() {
    let mut terminal=None;
    for cut in 0..20_000 {
        let source=crate::io::parse_ifc4x3_step(include_str!("🧪️vector/source.ifc")).unwrap();
        let before=source.clone();
        let foreign=source.clone();
        let mut caller=OriginalWorkFixture::with_identity(semio_framework_job::OperationId(950),semio_framework_job::Generation(cut+1));
        let(mut overlay,mut receiver)=prepare(&source,&mut caller);
        let env=MachineEnvironment{source:Some(&source),overlay:Some(&overlay),receiver:Some(&receiver),mutations:&[]};
        let other=MachineEnvironment{source:Some(&foreign),..env};
        let mut machine=RetainedMachine::default();
        assert!(caller.work_turn(grant(),1,|cx|measured(cx,|cx|machine.test_prepared_query(&ARITHMETIC[1],BINDINGS,&env,cx))).unwrap());
        let mut finished=false;
        for _ in 0..cut {
            if machine.environment.is_some(){caller.work_turn(grant(),1,|cx|{let before=cx.retained_progress();assert!(measured(cx,|cx|machine.advance(&other,cx)).is_err());assert_eq!(before,cx.retained_progress());Ok(())}).unwrap();}
            if caller.work_turn(grant(),1,|cx|measured(cx,|cx|machine.advance(&env,cx))).unwrap(){finished=true;break;}
        }
        if finished {terminal=Some(cut);} else {
            caller.cancel().cancel_now();
            caller.work_turn(grant(),1,|cx|{let before=cx.retained_progress();assert!(measured(cx,|cx|machine.advance(&env,cx)).is_err());assert_eq!(before,cx.retained_progress());Ok(())}).unwrap();
        }
        assert_eq!(source,before);
        close(&mut machine,&mut receiver,&mut overlay,&mut caller);
        if finished{break;}
    }
    assert!(terminal.is_some());
}
