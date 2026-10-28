//! 🚆️ The native direct counterpart consumes released neutral laws through original owned work.
use super::*;
use semio_framework_plugin::app_testing::OriginalWorkFixture;
use semio_framework_value::retained_clone::RetainedCloneClose;
fn grant() -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 1 << 20, maximum_capacity_bytes: 1 << 20, maximum_release_bytes: 1 << 20, maximum_depth: 512 }
}
fn entity(id: u64, name: &str, values: &[(&str, IfcValue)]) -> IfcEntity {
    let declaration = metadata::entity(name).unwrap();
    IfcEntity { id, name: name.into(), args: declaration.all_attributes.iter().map(|field| values.iter().find(|(key, _)| *key == field.name).map_or(IfcValue::Unset, |(_, value)| value.clone())).collect(), complex: vec![] }
}
fn set(source: &mut IfcSnapshot, id: u64, name: &str, value: IfcValue) {
    let row = source.entities.iter_mut().find(|entity| entity.id == id).unwrap();
    let index = metadata::entity(&row.name).unwrap().all_attributes.iter().position(|field| field.name == name).unwrap();
    row.args[index] = value;
}
fn typed(name: &str, value: IfcValue) -> IfcValue {
    IfcValue::TypedValue(semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::IfcTypedValue { name: name.into(), items: vec![value] })
}
pub(super) fn fixture(case: &str) -> IfcSnapshot {
    let reference = IfcValue::Reference;
    let mut source = IfcSnapshot {
        schema: "stdio.ifc".into(),
        header: IfcHeader { file_schema: vec![IfcValue::Aggregate(vec![IfcValue::String("IFC4X3_ADD2".into())])], ..Default::default() },
        entities: vec![
            entity(11, "IFCALIGNMENTSEGMENT", &[("DesignParameters", reference(12)), ("Representation", reference(13))]),
            entity(12, "IFCALIGNMENTHORIZONTALSEGMENT", &[("SegmentLength", IfcValue::Real(0.0))]),
            entity(13, "IFCPRODUCTDEFINITIONSHAPE", &[("Representations", IfcValue::Aggregate(vec![reference(14)]))]),
            entity(14, "IFCSHAPEREPRESENTATION", &[("Items", IfcValue::Aggregate(vec![reference(15)]))]),
            entity(15, "IFCCURVESEGMENT", &[("SegmentLength", typed("IFCLENGTHMEASURE", IfcValue::Real(0.0)))]),
        ],
        ..Default::default()
    };
    match case {
        "zero-parameter" => set(&mut source, 15, "SegmentLength", typed("IFCPARAMETERVALUE", IfcValue::Real(0.0))),
        "integer-zero" => set(&mut source, 15, "SegmentLength", typed("IFCLENGTHMEASURE", IfcValue::Integer(0))),
        "negative-zero" => set(&mut source, 15, "SegmentLength", typed("IFCLENGTHMEASURE", IfcValue::Real(-0.0))),
        "mixed-case-tag" => set(&mut source, 15, "SegmentLength", typed("iFcLeNgThMeAsUrE", IfcValue::Real(0.0))),
        "not-applicable" => {
            source.entities.remove(0);
        }
        "wrong-segment" => source.entities[0].name = "IFCBUILDINGELEMENTPROXY".into(),
        "business-nonzero" => {
            set(&mut source, 12, "SegmentLength", IfcValue::Real(1.0));
            set(&mut source, 11, "Representation", IfcValue::Unset);
        }
        "curve-nonzero-decoy" => {
            let mut other = source.entities[4].clone();
            other.id = 16;
            source.entities.push(other);
            set(&mut source, 15, "SegmentLength", typed("IFCLENGTHMEASURE", IfcValue::Real(2.0)));
        }
        "missing-representation" | "parent-only" => set(&mut source, 11, "Representation", IfcValue::Unset),
        "missing-curve" => set(&mut source, 14, "Items", IfcValue::Aggregate(vec![reference(99999)])),
        "two-items" | "repeated-curve" => {
            let next = if case == "two-items" {
                let mut other = source.entities[4].clone();
                other.id = 16;
                source.entities.push(other);
                16
            } else {
                15
            };
            set(&mut source, 14, "Items", IfcValue::Aggregate(vec![reference(15), reference(next)]));
        }
        "two-representations" => {
            let mut other = source.entities[3].clone();
            other.id = 16;
            source.entities.push(other);
            set(&mut source, 13, "Representations", IfcValue::Aggregate(vec![reference(14), reference(16)]));
        }
        "raw-select" => set(&mut source, 15, "SegmentLength", IfcValue::Real(0.0)),
        "unknown-select" => set(&mut source, 15, "SegmentLength", typed("IFCPOSITIVELENGTHMEASURE", IfcValue::Real(0.0))),
        "select-arity" => {
            let mut value = typed("IFCLENGTHMEASURE", IfcValue::Real(0.0));
            let IfcValue::TypedValue(typed) = &mut value else { unreachable!() };
            typed.items.push(IfcValue::Real(0.0));
            set(&mut source, 15, "SegmentLength", value);
        }
        "nonfinite-select" => set(&mut source, 15, "SegmentLength", typed("IFCPARAMETERVALUE", IfcValue::Real(f64::INFINITY))),
        "unknown-constituent" => source.entities[0].complex.push(semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::IfcComplexType { name: "UNKNOWN".into(), args: vec![] }),
        "ambiguous-owner" => {
            let part = source.entities[1].clone();
            source.entities[1].complex.push(semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::IfcComplexType { name: part.name, args: part.args });
        }
        "duplicate-id" => {
            let other = source.entities[4].clone();
            source.entities.push(other);
        }
        _ => {}
    }
    source
}
pub(super) fn request(kind: &str, applicable: bool) -> Arc<IfcAlignmentDirectRequest> {
    Arc::new(IfcAlignmentDirectRequest { targets: vec![IfcAlignmentDirectTarget { segment: "11".into(), kind: kind.into(), applicable }] })
}
fn retire<T: semio_framework_value::retirement::RetireOwned>(caller: &mut OriginalWorkFixture, owner: &mut Option<T>) {
    let mut close = RetainedCloneClose::default();
    for _ in 0..1_000_000 {
        if close.is_empty() && owner.is_none() {
            return;
        }
        caller
            .turn(grant(), |cx| {
                cx.with_original_retirement(|cx| {
                    let demand = close.next_owner_demands::<T>(owner.is_some(), &None, 32768)?;
                    semio_framework_job::close_original_retained_step(cx, demand, |grant| if close.is_empty() { close.begin_granted(owner, grant)?.ok_or_else(|| invalid("mvd.direct.close")) } else { close.step_granted(grant) })?;
                    Ok(())
                })
            })
            .unwrap();
    }
    panic!("direct original owner did not retire");
}
fn evaluate(source: &Arc<IfcSnapshot>, request: &Arc<IfcAlignmentDirectRequest>, inspect: impl FnOnce(&IfcAlignmentDirectReport)) {
    let mut caller = OriginalWorkFixture::new();
    let mut work = IfcAlignmentDirectWork::default();
    let mut done = false;
    for _ in 0..1_000_000 {
        if caller.work_turn(grant(), 1, |cx| work.advance(source, request, 31, cx)).unwrap() {
            done = true;
            break;
        }
    }
    assert!(done);
    let mut output = None;
    assert!(caller.work_turn(grant(), 1, |cx| work.take(&mut output, cx)).unwrap());
    inspect(output.as_ref().unwrap());
    retire(&mut caller, &mut output);
    retire(&mut caller, &mut Some(work));
    caller.finish();
}
#[test]
fn native_direct_counterpart_matches_all_existing_neutral_verdicts() {
    let law: serde_json::Value = serde_json::from_str(include_str!("direct-cases.json")).unwrap();
    for row in law["cases"].as_array().unwrap() {
        let case = row["id"].as_str().unwrap();
        let source = Arc::new(fixture(case));
        let input = request(if case == "wrong-kind" { "vertical" } else { row["kind"].as_str().unwrap() }, row["applicable"].as_bool().unwrap());
        evaluate(&source, &input, |report| {
            assert_eq!(report.status, row["expected"].as_str().unwrap(), "{case}");
            assert_eq!(report.targets[0].status, report.status);
            assert_eq!(report.schema_conformance, "not-evaluated");
            assert_eq!(report.view_conformance, "not-evaluated");
            assert_eq!(report.geometry_conformance, "not-evaluated");
            assert_eq!(report.layout_terminal_conformance, "not-evaluated");
            assert_eq!(report.parent_curve_conformance, "not-evaluated");
        });
    }
}
#[test]
fn native_direct_borrowed_population_preserves_all_prepared_and_unavailable_target_reports() {
    let law: serde_json::Value = serde_json::from_str(include_str!("direct-population-cases.json")).unwrap();
    for row in law["cases"].as_array().unwrap() {
        let mut source = fixture("zero-length");
        match row["variant"].as_str().unwrap() {
            "header" => source.header.file_schema = vec![IfcValue::Aggregate(vec![IfcValue::String("IFC4".into())])],
            "id" => source.entities[0].id = 0,
            "reference" => source.entities[0].args.push(IfcValue::Reference(0)),
            _ => {},
        }
        let source = Arc::new(source);
        let input = Arc::new(IfcAlignmentDirectRequest { targets: law["targets"].as_array().unwrap().iter().map(|target| IfcAlignmentDirectTarget { segment: if target["segment"] == "selected" { "11".into() } else { target["segment"].as_str().unwrap().into() }, kind: target["kind"].as_str().unwrap().into(), applicable: target["applicable"].as_bool().unwrap() }).collect() });
        evaluate(&source, &input, |report| {
            assert_eq!(report.status, row["status"].as_str().unwrap());
            assert_eq!(report.targets.len(), input.targets.len());
            for (ordinal, target) in report.targets.iter().enumerate() {
                assert_eq!(target.status, row["targets"][ordinal].as_str().unwrap());
                assert_eq!((&target.segment, &target.kind, target.applicable), (&input.targets[ordinal].segment, &input.targets[ordinal].kind, input.targets[ordinal].applicable));
            }
            assert_eq!(report.diagnostics.len(), row["diagnostics"].as_array().unwrap().len());
            for (ordinal, diagnostic) in report.diagnostics.iter().enumerate() { assert_eq!(diagnostic.code, row["diagnostics"][ordinal].as_str().unwrap()); }
        });
        assert_eq!(Arc::strong_count(&source), 1);
        assert_eq!(Arc::strong_count(&input), 1);
    }
}
#[test]
fn native_direct_exact_source_order_and_correspondence_are_retained() {
    let mut source = fixture("curve-nonzero-decoy");
    source.entities.reverse();
    let source = Arc::new(source);
    let input = request("horizontal", true);
    evaluate(&source, &input, |report| {
        assert_eq!(report.status, "fail");
        assert_eq!(report.targets[0].curve.as_deref(), Some("15"));
        let index = metadata::entity("IFCCURVESEGMENT").unwrap().all_attributes.iter().position(|field| field.name == "SegmentLength").unwrap();
        let ordinal = source.entities.iter().position(|row| row.id == 15).unwrap();
        assert_eq!(report.targets[0].diagnostics[0].pointer.as_deref(), Some(format!("/entities/{ordinal}/args/{index}").as_str()));
    });
}
#[test]
fn native_direct_denied_axes_source_substitution_and_cancelled_prefix_retain_original_owners() {
    for case in ["zero-length", "curve-nonzero-decoy", "raw-select"] {
        let source = Arc::new(fixture(case));
        let input = request("horizontal", true);
        let mut caller = OriginalWorkFixture::new();
        let mut work = IfcAlignmentDirectWork::default();
        let mut turns = 0;
        loop {
            let demand = work.demands(&source, &input).unwrap();
            for axis in 0..6 {
                let mut denied = grant();
                match axis {
                    0 => denied.maximum_items = 0,
                    1 => denied.maximum_copy_bytes = demand.copy_bytes.saturating_sub(1),
                    2 => {
                        if demand.capacity_bytes == 0 {
                            continue;
                        }
                        denied.maximum_capacity_bytes = demand.capacity_bytes - 1;
                    }
                    3 => {
                        if demand.release_bytes == 0 {
                            continue;
                        }
                        denied.maximum_release_bytes = demand.release_bytes - 1;
                    }
                    4 => denied.maximum_depth = 0,
                    _ => {}
                }
                let before = work.metadata();
                assert!(
                    !caller
                        .work_turn(denied, if axis == 5 { 0 } else { 1 }, |cx| {
                            let before = cx.retained_progress();
                            let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| work.advance(&source, &input, 31, cx));
                            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
                            assert_eq!(cx.retained_progress(), before);
                            result
                        })
                        .unwrap()
                );
                assert_eq!(work.metadata(), before);
            }
            turns += 1;
            if caller
                .work_turn(grant(), 1, |cx| {
                    let before = cx.retained_progress();
                    let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| work.advance(&source, &input, 31, cx));
                    let after = cx.retained_progress();
                    assert_eq!((heap.requested_bytes, heap.released_bytes), (after.retained_capacity_bytes - before.retained_capacity_bytes, after.released_bytes - before.released_bytes));
                    result
                })
                .unwrap()
            {
                break;
            }
            assert!(turns < 1_000_000);
        }
        let other_source = Arc::new(fixture("zero-length"));
        let other_input = request("horizontal", true);
        for mode in 0..3 {
            let before = work.metadata();
            assert!(caller.work_turn(grant(), 1, |cx| work.advance(if mode == 0 { &other_source } else { &source }, if mode == 1 { &other_input } else { &input }, if mode == 2 { 32 } else { 31 }, cx)).is_err());
            assert_eq!(work.metadata(), before);
        }
        retire(&mut caller, &mut Some(work));
        caller.finish();
        for prefix in 0..=turns {
            let mut caller = OriginalWorkFixture::new();
            let mut work = IfcAlignmentDirectWork::default();
            for _ in 0..prefix {
                caller.work_turn(grant(), 1, |cx| work.advance(&source, &input, 31, cx)).unwrap();
            }
            caller.cancel().cancel_now();
            let before = work.metadata();
            assert!(caller.work_turn(grant(), 1, |cx| work.advance(&source, &input, 31, cx)).is_err());
            assert_eq!(work.metadata(), before);
            let mut output = None;
            assert!(caller.work_turn(grant(), 1, |cx| work.take(&mut output, cx)).is_err());
            assert!(output.is_none());
            retire(&mut caller, &mut Some(work));
            caller.finish();
        }
    }
}

#[test]
fn native_direct_official_segments_retain_exact_counterpart_identity_and_scope() {
    for (text, segment, kind, expected, curve) in [
        (include_str!("../../../../🧪️fixtures/official/sectioned-solid-horizontal.ifc"), "1101", "horizontal", "pass", Some("1201")),
        (include_str!("../../../../🧪️fixtures/official/sectioned-solid-horizontal.ifc"), "2001", "vertical", "pass", Some("2101")),
        (include_str!("../../../../🧪️fixtures/official/linear-placement-of-signal.ifc"), "6001", "cant", "pass", Some("6201")),
        (include_str!("../../../../🧪️fixtures/official/segmented-reference-curve.ifc"), "149", "cant", "incomplete", None),
    ] {
        let source = Arc::new(crate::io::parse_ifc4x3_step(text).unwrap());
        let input = Arc::new(IfcAlignmentDirectRequest { targets: vec![IfcAlignmentDirectTarget { segment: segment.into(), kind: kind.into(), applicable: true }] });
        evaluate(&source, &input, |report| {
            assert_eq!(report.status, expected);
            assert_eq!(report.targets[0].curve.as_deref(), curve);
        });
    }
}

#[test]
fn native_direct_global_source_failure_and_ordered_status_precedence_remain_explicit() {
    let mut source = fixture("zero-length");
    source.header.file_schema.clear();
    let source = Arc::new(source);
    evaluate(&source, &request("horizontal", false), |report| {
        assert_eq!(report.status, "incomplete");
        assert_eq!(report.targets[0].status, "not-applicable");
        assert_eq!(report.diagnostics[0].code, "mvd.direct.source");
        assert!(report.diagnostics[0].path.is_empty());
    });
    let source = Arc::new(fixture("curve-nonzero-decoy"));
    let input = Arc::new(IfcAlignmentDirectRequest {
        targets: vec![
            IfcAlignmentDirectTarget { segment: "11".into(), kind: "horizontal".into(), applicable: true },
            IfcAlignmentDirectTarget { segment: "9998".into(), kind: "cant".into(), applicable: false },
            IfcAlignmentDirectTarget { segment: "9999".into(), kind: "vertical".into(), applicable: true },
        ],
    });
    evaluate(&source, &input, |report| {
        assert_eq!(report.status, "incomplete");
        assert_eq!(report.targets.iter().map(|row| row.status).collect::<Vec<_>>(), vec!["fail", "not-applicable", "incomplete"]);
        assert!(report.targets[2].diagnostics[0].path.is_empty());
    });
}

#[test]
fn native_direct_request_refusal_and_paid_output_use_the_original_caller() {
    let source = Arc::new(fixture("zero-length"));
    for segment in ["0", "01", "-1", "18446744073709551616"] {
        let input = Arc::new(IfcAlignmentDirectRequest { targets: vec![IfcAlignmentDirectTarget { segment: segment.into(), kind: "horizontal".into(), applicable: true }] });
        let mut caller = OriginalWorkFixture::new();
        let mut work = IfcAlignmentDirectWork::default();
        let mut refused = false;
        for _ in 0..10000 {
            if caller.work_turn(grant(), 1, |cx| work.advance(&source, &input, 31, cx)).is_err() {
                refused = true;
                break;
            }
        }
        assert!(refused);
        assert_eq!(Arc::strong_count(&source), 2);
        retire(&mut caller, &mut Some(work));
        assert_eq!(Arc::strong_count(&source), 1);
        caller.finish();
    }
    let input = request("horizontal", true);
    let mut caller = OriginalWorkFixture::new();
    let mut work = IfcAlignmentDirectWork::default();
    for _ in 0..1_000_000 {
        if caller.work_turn(grant(), 1, |cx| work.advance(&source, &input, 31, cx)).unwrap() {
            break;
        }
    }
    let mut foreign = OriginalWorkFixture::new();
    let mut output = None;
    assert!(foreign.work_turn(grant(), 1, |cx| work.take(&mut output, cx)).is_err());
    assert!(output.is_none());
    foreign.finish();
    let before = work.metadata();
    assert!(!caller.work_turn(RetainedCloneGrant { maximum_copy_bytes: 0, ..grant() }, 1, |cx| work.take(&mut output, cx)).unwrap());
    assert_eq!(work.metadata(), before);
    assert!(caller.work_turn(grant(), 1, |cx| work.take(&mut output, cx)).unwrap());
    let value = output.as_ref().unwrap();
    let definition: serde_json::Value = serde_json::from_str(include_str!("direct-definition.json")).unwrap();
    assert_eq!(value.source.commit, definition["source"]["commit"].as_str().unwrap());
    assert_eq!(value.source.url, definition["source"]["url"].as_str().unwrap());
    assert_eq!(value.source.sha256, definition["source"]["sha256"].as_str().unwrap());
    assert!(caller.work_turn(grant(), 1, |cx| work.take(&mut None, cx)).is_err());
    retire(&mut caller, &mut output);
    retire(&mut caller, &mut Some(work));
    caller.finish();
}
