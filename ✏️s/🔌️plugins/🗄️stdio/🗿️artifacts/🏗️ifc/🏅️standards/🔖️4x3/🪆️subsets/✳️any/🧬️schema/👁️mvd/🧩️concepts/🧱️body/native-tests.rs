//! 🧪️ The funded Body endpoint executes released clauses with a fixed policy and original preparation.
use super::*;
use super::tessellation_tests::{graph, loan, close, request};
use semio_framework_plugin::app_testing::OriginalWorkFixture;
use semio_framework_value::retained_clone::RetainedCloneClose;
use semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::{IfcComplexType, IfcValue};
use std::sync::Arc;
pub(super) fn new_work(policy: BodyPolicy) -> IfcBodyConceptWork { match policy { BodyPolicy::Tessellation => IfcBodyConceptWork::tessellation(), BodyPolicy::SweptSolid => IfcBodyConceptWork::swept_solid(), BodyPolicy::AdvancedSweptSolid => IfcBodyConceptWork::advanced_swept_solid(), BodyPolicy::SurfaceModel => IfcBodyConceptWork::surface_model(), BodyPolicy::Clipping => IfcBodyConceptWork::clipping(), BodyPolicy::Brep => IfcBodyConceptWork::brep(), BodyPolicy::AdvancedBrep => IfcBodyConceptWork::advanced_brep() } }
pub(super) fn evaluate(policy: BodyPolicy, source: &Arc<IfcSnapshot>, input: &Arc<IfcBodyConceptRequest>, check: impl FnOnce(&IfcBodyConceptReport)) {
    let mut work = new_work(policy);
    let mut caller = OriginalWorkFixture::new();
    let mut ready = false;
    for _ in 0..100000 { if caller.work_turn(loan(), 1, |cx| work.advance(source, input, 71, cx)).unwrap() { ready = true; break; } }
    assert!(ready);
    let mut output = None;
    assert!(caller.work_turn(loan(), 1, |cx| work.take(&mut output, cx)).unwrap());
    let report = output.as_ref().unwrap();
    check(report);
    assert_eq!(report.schema_conformance, "not-evaluated");
    assert_eq!(report.view_conformance, "not-evaluated");
    let mut retire = RetainedCloneClose::default();
    while output.is_some() || !retire.is_empty() {
        let demand = retire.next_owner_demands::<IfcBodyConceptReport>(output.is_some(), &None, 4096).unwrap();
        caller.turn(loan(), |cx| cx.with_original_retirement(|cx| semio_framework_job::close_original_retained_step(cx, demand, |grant| if retire.is_empty() { retire.begin_granted(&mut output, grant).map(|step| step.unwrap()) } else { retire.step_granted(grant) }))).unwrap();
    }
    close(work, &mut caller);
    caller.finish();
    assert_eq!(Arc::strong_count(source), 1);
    assert_eq!(Arc::strong_count(input), 1);
}
#[test]
fn native_body_swept_solid_executes_all_released_neutral_clauses() {
    let cases: serde_json::Value = serde_json::from_str(include_str!("../🌊️swept-solid/cases.json")).unwrap();
    for row in cases["cases"].as_array().unwrap() {
        let source = graph(row);
        let input = request(&[("1", row["applicable"].as_bool())]);
        evaluate(BodyPolicy::SweptSolid, &source, &input, |report| {
            assert_eq!(report.status, row["expected"].as_str().unwrap());
            assert_eq!(report.targets[0].status, report.status);
            assert_eq!(report.concept, "Product_Shape/Product_Geometric_Representation/Body_Geometry/Body_SweptSolid_Geometry");
            assert_eq!(report.source.sha256, "c571f3ac400b669781c09cc666b5cd55031b6a480333df6ac47d8ebb1f8dd7c0");
        });
    }
}
#[test]
fn native_body_whole_endpoint_preserves_late_unknown_family_evidence() {
    let cases: serde_json::Value = serde_json::from_str(include_str!("family-cases.json")).unwrap();
    for row in cases["cases"].as_array().unwrap() {
        let policy = match row["policy"].as_str().unwrap() { "tessellation" => BodyPolicy::Tessellation, "swept-solid" => BodyPolicy::SweptSolid, _ => unreachable!() };
        let names = row["names"].as_array().unwrap();
        let mut source = graph(&serde_json::json!({"identifier":"Body","type":policy.representation_type(),"items":[names[0]]})).as_ref().clone();
        source.entities[1].name = names[0].as_str().unwrap().into();
        source.entities[1].complex = names[1..].iter().map(|name| IfcComplexType { name: name.as_str().unwrap().into(), args: vec![] }).collect();
        let source = Arc::new(source);
        evaluate(policy, &source, &request(&[("1", Some(true))]), |report| {
            assert_eq!(report.status, row["expected"].as_str().unwrap());
            if report.status != "pass" {
                let diagnostic = &report.targets[0].diagnostics[0];
                assert_eq!(diagnostic.code, if report.status == "incomplete" { "mvd.items.unknown" } else { "mvd.items.type" });
                assert_eq!(diagnostic.path, ["Items", "0"]);
                assert_eq!(diagnostic.pointer.as_deref(), Some("/entities/0/args/3/value/0"));
            }
        });
    }
}
#[test]
fn native_body_construction_fixes_policy_across_moves_and_donation() {
    let source = graph(&serde_json::json!({"identifier":"Body","type":"SweptSolid","items":["IfcExtrudedAreaSolidTapered"]}));
    let input = request(&[("1", Some(true))]);
    let mut work = IfcBodyConceptWork::swept_solid();
    let mut caller = OriginalWorkFixture::new();
    let mut ready = false;
    for _ in 0..100000 {
        let mut moved = work;
        let result = caller.work_turn(loan(), 1, |cx| moved.advance(&source, &input, 71, cx)).unwrap();
        work = moved;
        if result { ready = true; break; }
    }
    assert!(ready);
    assert_eq!(work.clause.report.as_ref().unwrap().targets[0].diagnostics[0].label.en, "A representation item is not a basic extruded or revolved area solid.");
    let mut output = None;
    assert!(caller.work_turn(loan(), 1, |cx| work.take(&mut output, cx)).unwrap());
    assert!(caller.work_turn(loan(), 1, |cx| work.take(&mut output, cx)).is_err());
    let mut retire = RetainedCloneClose::default();
    while output.is_some() || !retire.is_empty() {
        let demand = retire.next_owner_demands::<IfcBodyConceptReport>(output.is_some(), &None, 4096).unwrap();
        caller.turn(loan(), |cx| cx.with_original_retirement(|cx| semio_framework_job::close_original_retained_step(cx, demand, |grant| if retire.is_empty() { retire.begin_granted(&mut output, grant).map(|step| step.unwrap()) } else { retire.step_granted(grant) }))).unwrap();
    }
    close(work, &mut caller);
    caller.finish();
}
#[test]
fn native_body_swept_family_turns_deny_all_original_axes_and_cancel_every_prefix() {
    for policy in [BodyPolicy::SweptSolid, BodyPolicy::AdvancedSweptSolid, BodyPolicy::SurfaceModel, BodyPolicy::Clipping, BodyPolicy::Brep, BodyPolicy::AdvancedBrep] {
    let source = graph(&serde_json::json!({"identifier":"Axis","type":policy.representation_type(),"items":["IfcExtrudedAreaSolidTapered","IfcRevolvedAreaSolid"]}));
    let input = request(&[("1", Some(true))]);
    let mut total = 0;
    for cancel_at in std::iter::once(None).chain((1..100000).map(Some)) {
        if cancel_at.is_some_and(|turn| turn > total) { break; }
        let mut caller = OriginalWorkFixture::new();
        let mut work = new_work(policy);
        for turn in 1..100000 {
            let demand = work.demands(&source, &input).unwrap();
            for axis in 0..6 {
                let mut denied = loan();
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
                let metadata = work.metadata();
                caller.work_turn(denied, fuel, |cx| {
                    let receipt = cx.retained_progress();
                    let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| work.advance(&source, &input, 71, cx));
                    assert!(!result?);
                    assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
                    assert_eq!(receipt, cx.retained_progress());
                    Ok(())
                }).unwrap();
                assert_eq!(metadata, work.metadata());
            }
            if cancel_at == Some(turn) { caller.cancel().cancel_now(); }
            let result = caller.work_turn(loan(), 1, |cx| {
                let before = cx.retained_progress();
                let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| work.advance(&source, &input, 71, cx));
                let after = cx.retained_progress();
                assert_eq!((heap.requested_bytes, heap.released_bytes), (after.retained_capacity_bytes - before.retained_capacity_bytes, after.released_bytes - before.released_bytes));
                result
            });
            if cancel_at == Some(turn) { assert!(result.is_err()); break; }
            if result.unwrap() { total = turn; break; }
            assert!(turn < 99999);
        }
        caller.cancel().cancel_now();
        close(work, &mut caller);
        caller.finish();
        assert_eq!(Arc::strong_count(&source), 1);
        assert_eq!(Arc::strong_count(&input), 1);
    }
    assert!(total > 20);
    }
}
