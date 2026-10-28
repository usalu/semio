//! 🧪️ Ordered native business-clause laws retain actual owner slots and original paid custody.
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
fn set(row: &mut IfcEntity, name: &str, value: IfcValue) {
    let index = metadata::entity(&row.name).unwrap().all_attributes.iter().position(|field| field.name == name).unwrap();
    row.args[index] = value;
}
fn typed(name: &str, items: Vec<IfcValue>) -> IfcValue {
    IfcValue::TypedValue(semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::IfcTypedValue { name: name.into(), items })
}
fn complex(row: &mut IfcEntity) {
    let declaration = metadata::entity(&row.name).unwrap();
    let args = std::mem::take(&mut row.args);
    let mut parts = vec![];
    let mut name = Some(declaration.key);
    while let Some(key) = name {
        let current = metadata::entity(key).unwrap();
        parts.push(semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::IfcComplexType {
            name: key.into(),
            args: current.attributes.iter().map(|attribute| args[declaration.all_attributes.iter().position(|field| field.name == attribute.name).unwrap()].clone()).collect(),
        });
        name = current.supertype;
    }
    let first = parts.remove(0);
    row.name = first.name;
    row.args = first.args;
    row.complex = parts;
}
pub(super) fn fixture(row: &serde_json::Value) -> IfcSnapshot {
    let layout = kind(row["kind"].as_str().unwrap()).unwrap();
    let mut members = vec![];
    let mut entities = vec![entity(1, LAYOUTS[layout], &[])];
    for (index, length) in row["lengths"].as_array().unwrap().iter().enumerate() {
        let id = 3 + index as u64 * 2;
        members.push(IfcValue::Reference(id));
        entities.push(entity(id, "IFCALIGNMENTSEGMENT", &[("DesignParameters", IfcValue::Reference(id + 1))]));
        entities.push(entity(id + 1, PARAMETERS[layout], &[(LENGTHS[layout], IfcValue::Real(length.as_f64().unwrap()))]));
    }
    entities.insert(1, entity(2, "IFCRELNESTS", &[("RelatingObject", IfcValue::Reference(1)), ("RelatedObjects", IfcValue::Aggregate(members))]));
    let variant = row["variant"].as_str().unwrap_or("");
    let last = entities.len() - 1;
    let terminal = last.saturating_sub(1);
    match variant {
        "signedzero" => set(&mut entities[last], LENGTHS[layout], IfcValue::Real(-0.0)),
        "wrong-design" => {
            let id = entities[last].id;
            entities[last] = entity(id, if layout == 1 { PARAMETERS[0] } else { PARAMETERS[1] }, &[]);
        }
        "complex" => {
            for item in &mut entities {
                complex(item);
            }
        }
        "duplicate-target" => {
            let other = entities[0].clone();
            entities.push(other);
        }
        "missing-target" => {
            entities.remove(0);
        }
        "wrong-target" => entities[0] = entity(1, if layout == 1 { LAYOUTS[0] } else { LAYOUTS[1] }, &[]),
        "unknown-target" => entities[0].name = "IFCUNKNOWNLAYOUT".into(),
        "mixed-layout" => entities[0].complex.push(semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::IfcComplexType { name: LAYOUTS[1].into(), args: vec![] }),
        "competing-lists" => {
            let mut other = entities[1].clone();
            other.id = 99;
            entities.push(other);
        }
        "unrelated-relationship" => entities[1].name = "IFCRELAGGREGATES".into(),
        "other-owner" => {
            set(&mut entities[1], "RelatingObject", IfcValue::Reference(99));
            set(&mut entities[1], "OwnerHistory", IfcValue::Reference(1));
        }
        "duplicate-attribute" => {
            complex(&mut entities[1]);
            let part = entities[1].clone();
            entities[1].complex.push(semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::IfcComplexType { name: "IFCRELNESTS".into(), args: part.args });
        }
        "nonreference-member" | "dangling-terminal" | "earlier-unresolved" => {
            let index = metadata::entity("IFCRELNESTS").unwrap().all_attributes.iter().position(|field| field.name == "RelatedObjects").unwrap();
            let IfcValue::Aggregate(items) = &mut entities[1].args[index] else { unreachable!() };
            let member = if variant == "earlier-unresolved" { 0 } else { items.len() - 1 };
            items[member] = if variant == "nonreference-member" { IfcValue::String("0".into()) } else { IfcValue::Reference(99) };
            if variant == "earlier-unresolved" {
                set(&mut entities[last], LENGTHS[layout], IfcValue::Real(10.0));
            }
        }
        "ambiguous-terminal" => {
            let other = entities[terminal].clone();
            entities.push(other);
        }
        "wrong-member" => entities[terminal].name = "IFCWALL".into(),
        "dangling-design" => set(&mut entities[terminal], "DesignParameters", IfcValue::Reference(99)),
        "typed-measure" => set(&mut entities[last], LENGTHS[layout], typed("IFCNONNEGATIVELENGTHMEASURE", vec![typed("IFCLENGTHMEASURE", vec![IfcValue::Real(0.0)])])),
        "wrong-measure-tag" => set(&mut entities[last], LENGTHS[layout], typed("IFCPARAMETERVALUE", vec![IfcValue::Real(0.0)])),
        "typed-multiple-items" => set(&mut entities[last], LENGTHS[layout], typed("IFCNONNEGATIVELENGTHMEASURE", vec![IfcValue::Real(0.0), IfcValue::Real(0.0)])),
        "integer-zero" | "integer-large" => set(&mut entities[last], LENGTHS[layout], IfcValue::Integer(if variant == "integer-zero" { 0 } else { i64::MAX })),
        "tiny" => set(&mut entities[last], LENGTHS[layout], IfcValue::Real(f64::from_bits(1))),
        "negative" => set(&mut entities[last], LENGTHS[layout], IfcValue::Real(-1.0)),
        "string-zero" => set(&mut entities[last], LENGTHS[layout], IfcValue::String("0".into())),
        "unknown-complex" => entities[0].complex.push(semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::IfcComplexType { name: "IFCUNKNOWN".into(), args: vec![] }),
        _ => {}
    }
    IfcSnapshot { schema: "stdio.ifc".into(), header: IfcHeader { file_schema: vec![IfcValue::Aggregate(vec![IfcValue::String("IFC4X3_ADD2".into())])], ..Default::default() }, entities, ..Default::default() }
}
pub(super) fn request(layout: &str, kind: &str, applicable: Option<bool>) -> Arc<IfcAlignmentTerminalRequest> {
    Arc::new(IfcAlignmentTerminalRequest { targets: vec![IfcAlignmentTerminalTarget { layout: layout.into(), kind: kind.into(), applicable }] })
}
fn retire<T: semio_framework_value::retirement::RetireOwned>(caller: &mut OriginalWorkFixture, owner: &mut Option<T>) {
    let mut close = RetainedCloneClose::default();
    for _ in 0..1_000_000 {
        if owner.is_none() && close.is_empty() {
            return;
        }
        caller
            .turn(grant(), |cx| {
                cx.with_original_retirement(|cx| {
                    let demand = close.next_owner_demands::<T>(owner.is_some(), &None, 32768)?;
                    semio_framework_job::close_original_retained_step(cx, demand, |grant| if close.is_empty() { close.begin_granted(owner, grant)?.ok_or_else(|| invalid("mvd.terminal.close")) } else { close.step_granted(grant) })?;
                    Ok(())
                })
            })
            .unwrap();
    }
    panic!("terminal original owner did not retire");
}
fn evaluate(source: &Arc<IfcSnapshot>, input: &Arc<IfcAlignmentTerminalRequest>, inspect: impl FnOnce(&IfcAlignmentTerminalReport)) {
    let mut caller = OriginalWorkFixture::new();
    let mut work = IfcAlignmentTerminalWork::default();
    let mut done = false;
    for _ in 0..2_000_000 {
        if caller.work_turn(grant(), 1, |cx| work.advance(source, input, 31, cx)).unwrap() {
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
fn native_ordered_business_terminal_consumes_all_existing_neutral_cases() {
    let cases: serde_json::Value = serde_json::from_str(include_str!("cases.json")).unwrap();
    for row in cases["cases"].as_array().unwrap() {
        let source = Arc::new(fixture(row));
        let input = request("1", row["kind"].as_str().unwrap(), row["applicable"].as_bool());
        evaluate(&source, &input, |report| {
            assert_eq!(report.status, row["expected"].as_str().unwrap(), "{}", row["id"]);
            assert_eq!(report.targets[0].status, report.status);
            assert_eq!(report.schema_conformance, "not-evaluated");
            assert_eq!(report.view_conformance, "not-evaluated");
            assert_eq!(report.geometry_conformance, "not-evaluated");
        });
    }
}
#[test]
fn native_ordered_business_terminal_reads_official_lists_without_geometric_ordinal_inference() {
    for (text, layout, kind) in [
        (include_str!("../../../../🧪️fixtures/official/sectioned-solid-horizontal.ifc"), "34", "horizontal"),
        (include_str!("../../../../🧪️fixtures/official/sectioned-solid-horizontal.ifc"), "45", "vertical"),
        (include_str!("../../../../🧪️fixtures/official/segmented-reference-curve.ifc"), "61", "cant"),
        (include_str!("../../../../🧪️fixtures/official/linear-placement-of-signal.ifc"), "2388", "cant"),
    ] {
        let source = Arc::new(crate::io::parse_ifc4x3_step(text).unwrap());
        evaluate(&source, &request(layout, kind, Some(true)), |report| assert_eq!(report.status, "pass"));
    }
}
#[test]
fn native_terminal_relationship_roster_and_diagnostics_observe_original_axes_and_every_cancelled_prefix() {
    let cases: serde_json::Value = serde_json::from_str(include_str!("cases.json")).unwrap();
    for id in ["horizontal-terminal-zero", "horizontal-terminal-nonzero", "competing-lists", "earlier-unresolved"] {
        let row = cases["cases"].as_array().unwrap().iter().find(|row| row["id"] == id || row["variant"] == id).unwrap();
        let source = Arc::new(fixture(row));
        let mut input = request("1", "horizontal", Some(true));
        for id in 100..109 {
            Arc::get_mut(&mut input).unwrap().targets.push(IfcAlignmentTerminalTarget { layout: id.to_string(), kind: "cant".into(), applicable: Some(false) });
        }
        let mut caller = OriginalWorkFixture::new();
        let mut work = IfcAlignmentTerminalWork::default();
        let mut turns = 0;
        loop {
            let demand = work.demands(&source, &input).unwrap();
            for axis in 0..6 {
                let mut denied = grant();
                match axis {
                    0 => denied.maximum_items = 0,
                    1 => denied.maximum_copy_bytes = demand.copy_bytes - 1,
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
                caller
                    .work_turn(denied, if axis == 5 { 0 } else { 1 }, |cx| {
                        let receipt = cx.retained_progress();
                        let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| work.advance(&source, &input, 31, cx));
                        assert!(!result?);
                        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
                        assert_eq!(cx.retained_progress(), receipt);
                        Ok(())
                    })
                    .unwrap();
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
            assert!(turns < 2_000_000);
        }
        retire(&mut caller, &mut Some(work));
        caller.finish();
        for prefix in 0..=turns {
            let mut caller = OriginalWorkFixture::new();
            let mut work = IfcAlignmentTerminalWork::default();
            for _ in 0..prefix {
                caller.work_turn(grant(), 1, |cx| work.advance(&source, &input, 31, cx)).unwrap();
            }
            caller.cancel().cancel_now();
            let before = work.metadata();
            assert!(caller.work_turn(grant(), 1, |cx| work.advance(&source, &input, 31, cx)).is_err());
            assert_eq!(work.metadata(), before);
            assert!(caller.work_turn(grant(), 1, |cx| work.take(&mut None, cx)).is_err());
            retire(&mut caller, &mut Some(work));
            caller.finish();
            assert_eq!(Arc::strong_count(&source), 1);
            assert_eq!(Arc::strong_count(&input), 1);
        }
    }
}

#[test]
fn native_terminal_canonical_request_original_source_and_output_authority_are_closed() {
    let cases: serde_json::Value = serde_json::from_str(include_str!("cases.json")).unwrap();
    let source = Arc::new(fixture(&cases["cases"][0]));
    for id in ["0", "01", "-1", "18446744073709551616"] {
        let input = request(id, "horizontal", Some(true));
        let mut caller = OriginalWorkFixture::new();
        let mut work = IfcAlignmentTerminalWork::default();
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
    let input = request("1", "horizontal", Some(true));
    let other_source = Arc::new(source.as_ref().clone());
    let other_input = request("1", "horizontal", Some(true));
    let mut caller = OriginalWorkFixture::new();
    let mut work = IfcAlignmentTerminalWork::default();
    caller.work_turn(grant(), 1, |cx| work.advance(&source, &input, 31, cx)).unwrap();
    for mode in 0..3 {
        let before = work.metadata();
        caller
            .work_turn(grant(), 1, |cx| {
                let receipt = cx.retained_progress();
                let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| work.advance(if mode == 0 { &other_source } else { &source }, if mode == 1 { &other_input } else { &input }, if mode == 2 { 32 } else { 31 }, cx));
                assert!(result.is_err());
                assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
                assert_eq!(receipt, cx.retained_progress());
                Ok(())
            })
            .unwrap();
        assert_eq!(work.metadata(), before);
    }
    let mut foreign = OriginalWorkFixture::new();
    assert!(foreign.work_turn(grant(), 1, |cx| work.advance(&source, &input, 31, cx)).is_err());
    assert!(foreign.work_turn(grant(), 1, |cx| work.take(&mut None, cx)).is_err());
    foreign.finish();
    for _ in 0..2_000_000 {
        if caller.work_turn(grant(), 1, |cx| work.advance(&source, &input, 31, cx)).unwrap() {
            break;
        }
    }
    let mut output = None;
    let before = work.metadata();
    assert!(!caller.work_turn(RetainedCloneGrant { maximum_copy_bytes: 0, ..grant() }, 1, |cx| work.take(&mut output, cx)).unwrap());
    assert_eq!(work.metadata(), before);
    assert!(output.is_none());
    assert!(caller.work_turn(grant(), 1, |cx| work.take(&mut output, cx)).unwrap());
    let report = output.as_ref().unwrap();
    let definition: serde_json::Value = serde_json::from_str(include_str!("definition.json")).unwrap();
    assert_eq!(report.source.sha256, definition["source"]["sha256"].as_str().unwrap());
    assert_eq!(report.source.commit, definition["source"]["commit"].as_str().unwrap());
    assert_eq!(report.source.url, definition["source"]["url"].as_str().unwrap());
    assert!(caller.work_turn(grant(), 1, |cx| work.take(&mut None, cx)).is_err());
    retire(&mut caller, &mut output);
    retire(&mut caller, &mut Some(work));
    caller.finish();
}
#[test]
fn native_terminal_population_preserves_unavailable_rows_and_late_unknown_discovery() {
    let law: serde_json::Value = serde_json::from_str(include_str!("terminal-population-cases.json")).unwrap();
    for row in law["cases"].as_array().unwrap() {
        let mut source = fixture(&serde_json::json!({"kind":"horizontal","lengths":[10,0]}));
        match row["variant"].as_str().unwrap() {
            "header" => source.header.file_schema = vec![IfcValue::Aggregate(vec![IfcValue::String("IFC4".into())])],
            "id" => source.entities[0].id = 0,
            "reference" => source.entities[0].args.push(IfcValue::Reference(0)),
            "unknown" => source.entities.push(IfcEntity { id: 900, name: "IFCUNKNOWN".into(), args: vec![], complex: vec![] }),
            _ => {},
        }
        let source = Arc::new(source);
        let input = Arc::new(IfcAlignmentTerminalRequest { targets: law["targets"].as_array().unwrap().iter().map(|target| IfcAlignmentTerminalTarget { layout: target["layout"].as_str().unwrap().into(), kind: target["kind"].as_str().unwrap().into(), applicable: target["applicable"].as_bool() }).collect() });
        evaluate(&source, &input, |report| {
            assert_eq!(report.status, row["status"].as_str().unwrap());
            assert_eq!(report.targets.len(), input.targets.len());
            for (ordinal, target) in report.targets.iter().enumerate() {
                assert_eq!(target.status, row["targets"][ordinal].as_str().unwrap());
                assert_eq!((&target.layout, &target.kind, target.applicable), (&input.targets[ordinal].layout, &input.targets[ordinal].kind, input.targets[ordinal].applicable));
            }
            assert_eq!(report.diagnostics.len(), row["diagnostics"].as_array().unwrap().len());
            for (ordinal, diagnostic) in report.diagnostics.iter().enumerate() { assert_eq!(diagnostic.code, row["diagnostics"][ordinal].as_str().unwrap()); }
            assert_eq!(report.targets[0].diagnostics.len(), row["firstDiagnostics"].as_array().unwrap().len());
            for (ordinal, diagnostic) in report.targets[0].diagnostics.iter().enumerate() { assert_eq!(diagnostic.code, row["firstDiagnostics"][ordinal].as_str().unwrap()); }
        });
        assert_eq!(Arc::strong_count(&source), 1);
        assert_eq!(Arc::strong_count(&input), 1);
    }
}

fn query_fixture(row: &serde_json::Value) -> IfcSnapshot {
    let mut source = fixture(&serde_json::json!({"kind":row["kind"],"lengths":[10,0],"variant":"complex"}));
    let variant = row["variant"].as_str().unwrap();
    let part = |name: &str, values: &[(&str, IfcValue)]| semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::IfcComplexType {
        name: name.into(),
        args: metadata::entity(name).unwrap().attributes.iter().map(|field| values.iter().find(|(key, _)| *key == field.name).map_or(IfcValue::Unset, |(_, value)| value.clone())).collect(),
    };
    if variant.starts_with("foreign-relationship-owner") {
        source.entities[1].complex.push(part("IFCRELAGGREGATES", &[("RelatingObject", IfcValue::Reference(99)), ("RelatedObjects", IfcValue::Aggregate(vec![]))]));
    }
    if variant.starts_with("foreign-parameter-owner") {
        source.entities.last_mut().unwrap().complex.push(part("IFCCURVESEGMENT", &[("SegmentLength", typed("IFCLENGTHMEASURE", vec![IfcValue::Real(0.0)]))]));
    }
    if variant == "duplicate-relationship-owner" {
        let row = &mut source.entities[1];
        row.complex.push(semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::IfcComplexType { name: row.name.clone(), args: row.args.clone() });
    }
    let id = match variant {
        "target-late-unknown" => Some(1),
        "segment-late-unknown" => Some(5),
        "parameter-late-unknown" | "foreign-parameter-owner-late-unknown" => Some(6),
        "relation-late-unknown" | "foreign-relationship-owner-late-unknown" => Some(2),
        _ => None,
    };
    if let Some(id) = id {
        source.entities.iter_mut().find(|row| row.id == id).unwrap().complex.push(semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::IfcComplexType { name: "IFCUNKNOWN".into(), args: vec![] });
    }
    if variant == "discovery-unknown-only" {
        source.entities.push(IfcEntity { id: 99, name: "IFCUNKNOWN".into(), args: vec![], complex: vec![] });
    }
    source
}
#[test]
fn native_terminal_owner_query_projection_consumes_the_shared_twenty_nine_cases() {
    let law: serde_json::Value = serde_json::from_str(include_str!("query-cases.json")).unwrap();
    assert_eq!(law["physicalConformance"], "not-evaluated");
    for row in law["cases"].as_array().unwrap() {
        let source = Arc::new(query_fixture(row));
        let input = request("1", row["kind"].as_str().unwrap(), Some(true));
        evaluate(&source, &input, |report| {
            assert_eq!(report.status, row["expected"].as_str().unwrap(), "{}", row["id"]);
            assert_eq!(report.targets[0].status, report.status);
            assert_eq!(report.schema_conformance, "not-evaluated");
            assert_eq!(report.view_conformance, "not-evaluated");
            assert_eq!(report.geometry_conformance, "not-evaluated");
        });
    }
}
#[test]
fn native_terminal_qualified_relationship_result_borrows_the_original_primary_direct_slot() {
    let source = query_fixture(&serde_json::json!({"kind":"horizontal","variant":"foreign-relationship-owner"}));
    let name = "RelatingObject";
    let owner = Some("IFCRELNESTS");
    let mut caller = OriginalWorkFixture::new();
    let mut work = IfcAttributeQueryWork::default();
    let mut done = false;
    for _ in 0..10000 {
        if caller.work_turn(grant(), 1, |cx| work.advance::<Ifc4x3AttributeMetadata, _>(&source, 1, name, owner, 31, cx)).unwrap() {
            done = true;
            break;
        }
    }
    assert!(done);
    let result = caller.work_turn(grant(), 1, |cx| Ok(work.completed::<Ifc4x3AttributeMetadata, _>(&source, 1, name, owner, 31, cx)?)).unwrap().unwrap();
    assert_eq!(result.status, IfcAttributeQueryStatus::Unique);
    let found = result.matches[0].as_ref().unwrap();
    assert_eq!(found.declaration.declared_by, "IfcRelNests");
    assert!(std::ptr::eq(found.value.unwrap(), &source.entities[1].args[0]));
    assert_eq!(found.address.to_string(), "/entities/1/args/0");
    while !work.terminal_is_empty() {
        caller.turn(grant(), |cx| cx.with_original_retirement(|cx| work.close_step(cx))).unwrap();
    }
    caller.finish();
}
