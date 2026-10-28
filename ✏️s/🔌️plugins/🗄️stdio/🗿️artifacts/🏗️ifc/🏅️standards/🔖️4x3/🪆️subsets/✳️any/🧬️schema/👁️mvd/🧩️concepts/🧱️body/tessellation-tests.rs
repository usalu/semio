//! 🧪️ Explicit released concept rows preserve original source custody and partial conformance scope.
use super::*;
use semio_framework_value::retained_clone::RetainedCloneClose;
use semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::{IfcEntity, IfcHeader, IfcValue};
use semio_framework_plugin::app_testing::OriginalWorkFixture;
pub(super) fn loan() -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 1_048_576, maximum_capacity_bytes: 1_048_576, maximum_release_bytes: 1_048_576, maximum_depth: 256 }
}
pub(super) fn graph(row: &serde_json::Value) -> Arc<IfcSnapshot> {
    let text = |value: &str| IfcValue::String(value.into());
    let items = row["items"].as_array().unwrap();
    let mut entities = vec![IfcEntity {
        id: 1,
        name: "IFCSHAPEREPRESENTATION".into(),
        args: vec![IfcValue::Unset, text(row["identifier"].as_str().unwrap()), text(row["type"].as_str().unwrap()), IfcValue::Aggregate(items.iter().enumerate().map(|(index, _)| IfcValue::Reference(index as u64 + 2)).collect())],
        complex: vec![],
    }];
    entities.extend(items.iter().enumerate().map(|(index, name)| IfcEntity { id: index as u64 + 2, name: name.as_str().unwrap().to_ascii_uppercase(), args: vec![], complex: vec![] }));
    Arc::new(IfcSnapshot { header: IfcHeader { file_description: vec![], file_name: vec![], file_schema: vec![IfcValue::Aggregate(vec![text("IFC4X3_ADD2")])] }, entities, ..Default::default() })
}
pub(super) fn close(work: IfcBodyConceptWork, caller: &mut OriginalWorkFixture) {
    let mut owner = Some(work);
    let mut close = RetainedCloneClose::default();
    while owner.is_some() || !close.is_empty() {
        let demand = close.next_owner_demands::<IfcBodyConceptWork>(owner.is_some(), &None, 4096).unwrap();
        caller
            .turn(loan(), |cx| {
                cx.with_original_retirement(|cx| {
                    let before = cx.retained_progress();
                    let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| {
                        semio_framework_job::close_original_retained_step(cx, demand, |grant| if close.is_empty() { close.begin_granted(&mut owner, grant).map(|step| step.unwrap()) } else { close.step_granted(grant) })
                    });
                    let after = cx.retained_progress();
                    assert_eq!((heap.requested_bytes, heap.released_bytes), (after.retained_capacity_bytes - before.retained_capacity_bytes, after.released_bytes - before.released_bytes));
                    result
                })
            })
            .unwrap();
    }
}
#[test]
fn released_body_tessellation_neutral_rows_execute_under_original_context() {
    let cases: serde_json::Value = serde_json::from_str(include_str!("../🔺️tessellation/cases.json")).unwrap();
    for row in cases["cases"].as_array().unwrap() {
        let snapshot = graph(row);
        let original = snapshot.as_ref().clone();
        let request = Arc::new(IfcBodyConceptRequest { targets: vec![IfcConceptTarget { representation: "1".into(), applicable: row["applicable"].as_bool() }] });
        let mut caller = OriginalWorkFixture::new();
        let mut work = IfcBodyConceptWork::tessellation();
        for turn in 0..100000 {
            if caller.work_turn(loan(), 1, |cx| work.advance(&snapshot, &request, 71, cx)).unwrap() {
                break;
            }
            assert!(turn < 99999);
        }
        let mut report = None;
        caller.work_turn(loan(), 1, |cx| work.take(&mut report, cx)).unwrap();
        let report = report.unwrap();
        assert_eq!(report.status, row["expected"].as_str().unwrap());
        assert_eq!(report.targets[0].status, report.status);
        assert_eq!(report.schema_conformance, "not-evaluated");
        assert_eq!(report.view_conformance, "not-evaluated");
        assert_eq!(snapshot.as_ref(), &original);
        let mut report_owner = Some(report);
        let mut retire = RetainedCloneClose::default();
        while report_owner.is_some() || !retire.is_empty() {
            let demand = retire.next_owner_demands::<IfcBodyConceptReport>(report_owner.is_some(), &None, 4096).unwrap();
            caller
                .turn(loan(), |cx| {
                    cx.with_original_retirement(|cx| {
                        semio_framework_job::close_original_retained_step(cx, demand, |grant| if retire.is_empty() { retire.begin_granted(&mut report_owner, grant).map(|step| step.unwrap()) } else { retire.step_granted(grant) })
                    })
                })
                .unwrap();
        }
        close(work, &mut caller);
        caller.finish();
        assert_eq!(Arc::strong_count(&snapshot), 1);
        assert_eq!(Arc::strong_count(&request), 1);
    }
}

#[test]
fn declared_owner_queries_preserve_complex_and_unknown_tessellation_source_laws(){
    let cases:serde_json::Value=serde_json::from_str(include_str!("../🔺️tessellation/query-cases.json")).unwrap();for row in cases["cases"].as_array().unwrap(){let mut snapshot=graph(&serde_json::json!({"identifier":"Body","type":"Tessellation","items":["IFCTRIANGULATEDFACESET"]})).as_ref().clone();let args=std::mem::take(&mut snapshot.entities[0].args);let name=row["id"].as_str().unwrap();snapshot.entities[0].complex.push(semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::IfcComplexType{name:cases["owner"].as_str().unwrap().into(),args:if name=="flattened-complex-primary"{vec![]}else{args.clone()}});if name=="flattened-complex-primary"{snapshot.entities[0].args=args.clone();}if name=="duplicate-owner"||name=="late-unknown"{snapshot.entities[0].complex.push(semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::IfcComplexType{name:cases["owner"].as_str().unwrap().into(),args});}if name=="late-unknown"{snapshot.entities[0].complex.push(semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::IfcComplexType{name:"UNKNOWN".into(),args:vec![]});}let snapshot=Arc::new(snapshot);let request=Arc::new(IfcBodyConceptRequest{targets:vec![IfcConceptTarget{representation:"1".into(),applicable:Some(true)}]});let mut caller=OriginalWorkFixture::new();let mut work=IfcBodyConceptWork::tessellation();let mut ready=false;for _ in 0..100000{if caller.work_turn(loan(),1,|cx|work.advance(&snapshot,&request,71,cx)).unwrap(){ready=true;break;}}assert!(ready);let mut output=None;assert!(caller.work_turn(loan(),1,|cx|work.take(&mut output,cx)).unwrap());let report=output.as_ref().unwrap();assert_eq!(report.status,row["status"].as_str().unwrap());assert_eq!(report.schema_conformance,"not-evaluated");assert_eq!(report.view_conformance,"not-evaluated");if let Some(code)=row["code"].as_str(){assert!(report.targets[0].diagnostics.iter().any(|diagnostic|diagnostic.code==code));}let mut close_report=RetainedCloneClose::default();while output.is_some()||!close_report.is_empty(){let demand=close_report.next_owner_demands::<IfcBodyConceptReport>(output.is_some(),&None,4096).unwrap();caller.turn(loan(),|cx|cx.with_original_retirement(|cx|semio_framework_job::close_original_retained_step(cx,demand,|grant|if close_report.is_empty(){close_report.begin_granted(&mut output,grant).map(|step|step.unwrap())}else{close_report.step_granted(grant)}))).unwrap();}close(work,&mut caller);caller.finish();}
}
#[test]
fn retained_body_tessellation_denies_original_axes_cancels_every_prefix_and_closes() {
    let cases: serde_json::Value = serde_json::from_str(include_str!("../🔺️tessellation/cases.json")).unwrap();
    let mut snapshot = graph(&cases["cases"][0]);
    let source = Arc::get_mut(&mut snapshot).unwrap();
    source.entities[0].args[1] = IfcValue::String("Axis".into());
    source.entities[1].name = "IFCPOLYLINE".into();
    let request = Arc::new(IfcBodyConceptRequest { targets: vec![IfcConceptTarget { representation: "1".into(), applicable: Some(true) }] });
    let mut turns = 0;
    for cancel_at in std::iter::once(None).chain((1..100000).map(Some)) {
        if cancel_at.is_some_and(|turn| turn > turns) {
            break;
        }
        let mut caller = OriginalWorkFixture::new();
        let mut work = IfcBodyConceptWork::tessellation();
        for turn in 1..100000 {
            let demand = work.demands(&snapshot, &request).unwrap();
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
                let before = work.metadata();
                caller
                    .work_turn(denied, fuel, |cx| {
                        let receipt = cx.retained_progress();
                        let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| work.advance(&snapshot, &request, 71, cx));
                        assert!(!result?);
                        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
                        assert_eq!(receipt, cx.retained_progress());
                        Ok(())
                    })
                    .unwrap();
                assert_eq!(before, work.metadata());
            }
            if cancel_at == Some(turn) {
                caller.cancel().cancel_now();
            }
            let result = caller.work_turn(loan(), 1, |cx| {
                let before = cx.retained_progress();
                let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| work.advance(&snapshot, &request, 71, cx));
                let after = cx.retained_progress();
                assert_eq!((heap.requested_bytes, heap.released_bytes), (after.retained_capacity_bytes - before.retained_capacity_bytes, after.released_bytes - before.released_bytes));
                result
            });
            if cancel_at == Some(turn) {
                assert!(result.is_err());
                break;
            }
            if result.unwrap() {
                turns = turn;
                break;
            }
            assert!(turn < 99999);
        }
        caller.cancel().cancel_now();
        close(work, &mut caller);
        caller.finish();
        assert_eq!(Arc::strong_count(&snapshot), 1);
        assert_eq!(Arc::strong_count(&request), 1);
    }
    assert!(turns > 20);
}

pub(super) fn request(targets: &[(&str, Option<bool>)]) -> Arc<IfcBodyConceptRequest> {
    Arc::new(IfcBodyConceptRequest { targets: targets.iter().map(|(id, applicable)| IfcConceptTarget { representation: (*id).into(), applicable: *applicable }).collect() })
}
fn sample() -> Arc<IfcSnapshot> {
    let cases: serde_json::Value = serde_json::from_str(include_str!("../🔺️tessellation/cases.json")).unwrap();
    graph(&cases["cases"][0])
}
fn evaluate(source: &Arc<IfcSnapshot>, input: &Arc<IfcBodyConceptRequest>, check: impl FnOnce(&IfcBodyConceptReport)) {
    let mut caller = OriginalWorkFixture::new();
    let mut work = IfcBodyConceptWork::tessellation();
    for turn in 0..100000 {
        if caller.work_turn(loan(), 1, |cx| work.advance(source, input, 71, cx)).unwrap() {
            break;
        }
        assert!(turn < 99999);
    }
    let mut report = None;
    assert!(caller.work_turn(loan(), 1, |cx| work.take(&mut report, cx)).unwrap());
    check(report.as_ref().unwrap());
    let mut retire = RetainedCloneClose::default();
    while report.is_some() || !retire.is_empty() {
        let demand = retire.next_owner_demands::<IfcBodyConceptReport>(report.is_some(), &None, 4096).unwrap();
        caller
            .turn(loan(), |cx| {
                cx.with_original_retirement(|cx| semio_framework_job::close_original_retained_step(cx, demand, |grant| if retire.is_empty() { retire.begin_granted(&mut report, grant).map(|step| step.unwrap()) } else { retire.step_granted(grant) }))
            })
            .unwrap();
    }
    close(work, &mut caller);
    caller.finish();
    assert_eq!(Arc::strong_count(source), 1);
    assert_eq!(Arc::strong_count(input), 1);
}
#[test]
fn body_tessellation_counterexamples_preserve_exact_actual_owner_pointers() {
    let mut source = sample();
    Arc::get_mut(&mut source).unwrap().entities[0].args[1] = IfcValue::String("Axis".into());
    let input = request(&[("1", Some(true))]);
    evaluate(&source, &input, |report| {
        assert_eq!(report.status, "fail");
        let diagnostic = &report.targets[0].diagnostics[0];
        assert_eq!(diagnostic.code, "mvd.equals");
        assert_eq!(diagnostic.path, ["RepresentationIdentifier"]);
        assert_eq!(diagnostic.pointer.as_deref(), Some("/entities/0/args/1"));
    });
    let graph = Arc::get_mut(&mut source).unwrap();
    graph.entities[0].args[1] = IfcValue::String("Body".into());
    graph.entities[1].name = "IFCPOLYLINE".into();
    evaluate(&source, &input, |report| {
        assert_eq!(report.status, "fail");
        let diagnostic = &report.targets[0].diagnostics[0];
        assert_eq!(diagnostic.code, "mvd.items.type");
        assert_eq!(diagnostic.path, ["Items", "0"]);
        assert_eq!(diagnostic.pointer.as_deref(), Some("/entities/0/args/3/value/0"));
    });
}
#[test]
fn body_tessellation_unresolved_source_evidence_is_incomplete() {
    for (variant, code) in [
        (0, "mvd.target.missing"),
        (1, "mvd.target.ambiguous"),
        (2, "mvd.items.missing"),
        (3, "mvd.items.ambiguous"),
        (4, "mvd.items.unknown"),
        (5, "mvd.items.reference"),
        (6, "mvd.attribute.shape"),
        (7, "mvd.attribute.missing"),
        (8, "mvd.target.unknown"),
    ] {
        let mut source = sample();
        let graph = Arc::get_mut(&mut source).unwrap();
        match variant {
            0 => {
                graph.entities.remove(0);
            }
            1 => graph.entities.push(graph.entities[0].clone()),
            2 => {
                graph.entities.pop();
            }
            3 => graph.entities.push(graph.entities[1].clone()),
            4 => graph.entities[1].name = "IFCUNKNOWNENTITY".into(),
            5 => graph.entities[0].args[3] = IfcValue::Aggregate(vec![IfcValue::String("not a reference".into())]),
            6 => graph.entities[0].args[3] = IfcValue::Unset,
            7 => graph.entities[0].args.truncate(3),
            _ => graph.entities[0].name = "IFCUNKNOWNENTITY".into(),
        }
        evaluate(&source, &request(&[("1", Some(true))]), |report| {
            assert_eq!(report.status, "incomplete");
            assert!(report.targets[0].diagnostics.iter().any(|diagnostic| diagnostic.code == code));
        });
    }
}
#[test]
fn body_tessellation_complex_declared_owners_and_typed_label_follow_the_same_contract() {
    use semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::{IfcComplexType, IfcTypedValue};
    let mut source = sample();
    let graph = Arc::get_mut(&mut source).unwrap();
    let args = std::mem::take(&mut graph.entities[0].args);
    graph.entities[0].complex = vec![IfcComplexType { name: "IFCREPRESENTATION".into(), args }];
    graph.entities[1].name = "IFCTESSELLATEDFACESET".into();
    graph.entities[1].complex = vec![IfcComplexType { name: "IFCTRIANGULATEDFACESET".into(), args: vec![] }];
    let input = request(&[("1", Some(true))]);
    evaluate(&source, &input, |report| assert_eq!(report.status, "pass"));
    Arc::get_mut(&mut source).unwrap().entities[0].complex[0].args[2] = IfcValue::String("SurfaceModel".into());
    evaluate(&source, &input, |report| assert_eq!(report.targets[0].diagnostics[0].pointer.as_deref(), Some("/entities/0/complex/0/args/2")));
    let graph = Arc::get_mut(&mut source).unwrap();
    let duplicate = graph.entities[0].complex[0].clone();
    graph.entities[0].complex.push(duplicate);
    evaluate(&source, &input, |report| assert_eq!(report.targets[0].diagnostics[0].code, "mvd.attribute.ambiguous"));
    for (name, status) in [("IFCLABEL", "pass"), ("IFCTEXT", "fail")] {
        let mut source = sample();
        Arc::get_mut(&mut source).unwrap().entities[0].args[1] = IfcValue::TypedValue(IfcTypedValue { name: name.into(), items: vec![IfcValue::String("Body".into())] });
        evaluate(&source, &input, |report| assert_eq!(report.status, status));
    }
    let mut source = sample();
    Arc::get_mut(&mut source).unwrap().entities[0].args[3] = IfcValue::Aggregate(vec![]);
    evaluate(&source, &input, |report| {
        assert_eq!(report.status, "pass");
        assert_eq!(report.schema_conformance, "not-evaluated");
    });
}
#[test]
fn body_tessellation_exact_u64_aggregate_precedence_and_invalid_source_are_explicit() {
    let mut source = sample();
    Arc::get_mut(&mut source).unwrap().entities[0].id = u64::MAX;
    let maximum = "18446744073709551615";
    evaluate(&source, &request(&[(maximum, Some(true)), ("99", Some(false))]), |report| {
        assert_eq!(report.status, "pass");
        assert_eq!(report.targets[0].representation, maximum);
    });
    evaluate(&source, &request(&[(maximum, Some(false)), ("99", Some(false))]), |report| assert_eq!(report.status, "not-applicable"));
    Arc::get_mut(&mut source).unwrap().entities[0].args[1] = IfcValue::String("Axis".into());
    evaluate(&source, &request(&[(maximum, Some(true)), ("99", Some(false))]), |report| assert_eq!(report.status, "fail"));
    evaluate(&source, &request(&[(maximum, Some(true)), ("99", Some(true))]), |report| assert_eq!(report.status, "incomplete"));
    for variant in 0..4 {
        let mut source = sample();
        let graph = Arc::get_mut(&mut source).unwrap();
        match variant {
            0 => graph.schema = "foreign".into(),
            1 => graph.header.file_schema = vec![],
            2 => graph.entities[1].id = 0,
            _ => graph.entities[0].args[3] = IfcValue::Aggregate(vec![IfcValue::Reference(0)]),
        }
        evaluate(&source, &request(&[("1", Some(true)), ("99", Some(false))]), |report| {
            assert_eq!(report.status, "incomplete");
            assert_eq!(report.diagnostics[0].code, "mvd.source");
            assert_eq!(report.targets[1].status, "not-applicable");
        });
    }
}
#[test]
fn body_tessellation_original_source_request_revision_and_caller_cannot_be_substituted() {
    let source = sample();
    let input = request(&[("1", Some(true))]);
    let other_source = Arc::new(source.as_ref().clone());
    let other_input = request(&[("1", Some(true))]);
    let mut caller = OriginalWorkFixture::new();
    let mut work = IfcBodyConceptWork::tessellation();
    caller.work_turn(loan(), 1, |cx| work.advance(&source, &input, 71, cx)).unwrap();
    for (source, input, revision) in [(&other_source, &input, 71), (&source, &other_input, 71), (&source, &input, 72)] {
        caller
            .work_turn(loan(), 1, |cx| {
                let before = cx.retained_progress();
                assert!(work.advance(source, input, revision, cx).is_err());
                assert_eq!(before, cx.retained_progress());
                Ok(())
            })
            .unwrap();
    }
    let mut other = OriginalWorkFixture::new();
    other
        .work_turn(loan(), 1, |cx| {
            assert!(work.advance(&source, &input, 71, cx).is_err());
            Ok(())
        })
        .unwrap();
    other.finish();
    close(work, &mut caller);
    caller.finish();
}
#[test]
fn body_tessellation_invalid_requests_refuse_before_population_reads() {
    for input in [request(&[]), request(&[("1", Some(true)), ("1", Some(false))]), request(&[("0", Some(true))]), request(&[("01", Some(true))]), request(&[("18446744073709551616", Some(true))])] {
        let source = sample();
        let mut caller = OriginalWorkFixture::new();
        let mut work = IfcBodyConceptWork::tessellation();
        let mut refused = false;
        for turn in 0..100000 {
            if caller.work_turn(loan(), 1, |cx| work.advance(&source, &input, 71, cx)).is_err() {
                refused = true;
                break;
            }
            assert!(turn < 99999);
        }
        assert!(refused);
        assert!(work.preparation_stage() < 3);
        close(work, &mut caller);
        caller.finish();
        assert_eq!(Arc::strong_count(&input), 1);
    }
}

#[test]
fn body_tessellation_growth_and_partial_diagnostic_owners_quote_actual_physical_receipts() {
    let mut source = sample();
    let graph = Arc::get_mut(&mut source).unwrap();
    graph.entities[0].args[3] = IfcValue::Aggregate((2..=14).map(IfcValue::Reference).collect());
    graph.entities[1].name = "IFCPOLYLINE".into();
    graph.entities.extend((3..=20).map(|id| IfcEntity { id, name: "IFCPOLYLINE".into(), args: vec![], complex: vec![] }));
    let input = Arc::new(IfcBodyConceptRequest { targets: (1..=10).map(|id| IfcConceptTarget { representation: id.to_string(), applicable: Some(id == 1) }).collect() });
    let mut caller = OriginalWorkFixture::new();
    let mut work = IfcBodyConceptWork::tessellation();
    let mut release_seen = false;
    let mut partial_seen = false;
    for turn in 0..100000 {
        let demand = work.demands(&source, &input).unwrap();
        release_seen |= demand.release_bytes > 0;
        partial_seen |= work.clause.diagnostic.is_some();
        for axis in 0..5 {
            let mut denied = loan();
            match axis {
                0 => denied.maximum_items = 0,
                1 => denied.maximum_copy_bytes = demand.copy_bytes - 1,
                2 if demand.capacity_bytes > 0 => denied.maximum_capacity_bytes = demand.capacity_bytes - 1,
                3 if demand.release_bytes > 0 => denied.maximum_release_bytes = demand.release_bytes - 1,
                4 => denied.maximum_depth = 0,
                _ => continue,
            }
            let state = work.metadata();
            caller
                .work_turn(denied, 1, |cx| {
                    let before = cx.retained_progress();
                    let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| work.advance(&source, &input, 71, cx));
                    assert!(!result?);
                    assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
                    assert_eq!(before, cx.retained_progress());
                    Ok(())
                })
                .unwrap();
            assert_eq!(state, work.metadata());
        }
        if caller
            .work_turn(loan(), 1, |cx| {
                let before = cx.retained_progress();
                let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| work.advance(&source, &input, 71, cx));
                let after = cx.retained_progress();
                assert_eq!((heap.requested_bytes, heap.released_bytes), (after.retained_capacity_bytes - before.retained_capacity_bytes, after.released_bytes - before.released_bytes));
                result
            })
            .unwrap()
        {
            break;
        }
        assert!(turn < 99999);
    }
    assert!(release_seen && partial_seen);
    assert_eq!(work.clause.report.as_ref().unwrap().status, "fail");
    assert_eq!(work.clause.report.as_ref().unwrap().targets[0].diagnostics.len(), 13);
    caller.cancel().cancel_now();
    close(work, &mut caller);
    caller.finish();
    assert_eq!(Arc::strong_count(&source), 1);
    assert_eq!(Arc::strong_count(&input), 1);
}

#[test]
fn body_tessellation_output_donation_stays_on_original_funded_caller() {
    let source = sample();
    let input = request(&[("1", Some(true))]);
    let mut work = IfcBodyConceptWork::tessellation();
    let mut caller = OriginalWorkFixture::new();
    for turn in 0..100000 {
        if caller.work_turn(loan(), 1, |cx| work.advance(&source, &input, 71, cx)).unwrap() {
            break;
        }
        assert!(turn < 99999);
    }
    let mut output = None;
    for axis in 0..4 {
        let mut denied = loan();
        let mut fuel = 1;
        match axis {
            0 => denied.maximum_items = 0,
            1 => denied.maximum_copy_bytes = 0,
            2 => denied.maximum_depth = 0,
            _ => fuel = 0,
        }
        let state = work.metadata();
        caller
            .work_turn(denied, fuel, |cx| {
                let before = cx.retained_progress();
                assert!(!work.take(&mut output, cx)?);
                assert_eq!(before, cx.retained_progress());
                Ok(())
            })
            .unwrap();
        assert_eq!(state, work.metadata());
        assert!(output.is_none());
    }
    let mut other = OriginalWorkFixture::new();
    other
        .work_turn(loan(), 1, |cx| {
            let before = cx.retained_progress();
            assert!(work.take(&mut output, cx).is_err());
            assert_eq!(before, cx.retained_progress());
            Ok(())
        })
        .unwrap();
    other.finish();
    caller.cancel().cancel_now();
    assert!(caller.work_turn(loan(), 1, |cx| work.take(&mut output, cx)).is_err());
    assert!(output.is_none());
    close(work, &mut caller);
    caller.finish();
    assert_eq!(Arc::strong_count(&source), 1);
    assert_eq!(Arc::strong_count(&input), 1);
}
