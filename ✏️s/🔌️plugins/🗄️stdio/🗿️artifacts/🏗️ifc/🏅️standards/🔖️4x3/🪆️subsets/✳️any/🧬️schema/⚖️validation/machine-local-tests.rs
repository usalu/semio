//! 🧪️ Issued local rules evaluate the received candidate and preserve unknown as incomplete.
use super::*;
use semio_framework_plugin::app_testing::OriginalWorkFixture;
use semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::IfcComplexType;
use semio_s_artifact_stdio_ifc_4_mutations::standards::v4::subsets::any::schema::mutations::SetEntityArg;
fn grant() -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items: 100000, maximum_copy_bytes: 1048576, maximum_capacity_bytes: 1048576, maximum_release_bytes: 1048576, maximum_depth: 256 }
}
fn value(raw: &serde_json::Value) -> IfcValue {
    match raw["kind"].as_str().unwrap() {
        "unset" => IfcValue::Unset,
        "derived" => IfcValue::Derived,
        "reference" => IfcValue::Reference(raw["value"].as_str().unwrap().parse().unwrap()),
        "integer" => IfcValue::Integer(raw["value"].as_str().unwrap().parse().unwrap()),
        "real" => IfcValue::Real(f64::from_bits(raw["value"]["bits"].as_str().unwrap().parse().unwrap())),
        "string" => IfcValue::String(raw["value"].as_str().unwrap().into()),
        "enum" => IfcValue::Enum(raw["value"].as_str().unwrap().into()),
        "binary" => IfcValue::Binary(raw["value"].as_str().unwrap().into()),
        "aggregate" => IfcValue::Aggregate(raw["value"].as_array().unwrap().iter().map(value).collect()),
        other => panic!("unsupported fixture {other}"),
    }
}
fn snapshot(raw: &serde_json::Value) -> IfcSnapshot {
    IfcSnapshot { entities: raw["entities"].as_array().unwrap().iter().map(|row| IfcEntity {
        id: row["id"].as_str().unwrap().parse().unwrap(), name: row["name"].as_str().unwrap().into(),
        args: row["args"].as_array().unwrap().iter().map(value).collect(),
        complex: row["complex"].as_array().unwrap().iter().map(|part| IfcComplexType { name: part["name"].as_str().unwrap().into(), args: part["args"].as_array().unwrap().iter().map(value).collect() }).collect(),
    }).collect(), ..Default::default() }
}
#[test]
fn local_issued_alias_and_entity_rules_use_actual_overlay_candidate() {
    let contract: serde_json::Value = serde_json::from_str(include_str!("machine-local-cases.json")).unwrap();
    let values: serde_json::Value = serde_json::from_str(include_str!("../../../../../../🔎️details/🧪️tests/type-number-binary-input.json")).unwrap();
    for law in contract["cases"].as_array().unwrap() {
        let mut source = snapshot(&values["sources"][law["source"].as_str().unwrap()]);
        let row = values["cases"].as_array().unwrap().iter().find(|row| row["id"] == law["case"]);
        let index = if law["source"] == "binary" { 1 } else { 0 };
        let replacement = row.map(|row| value(&row["args"][index])).unwrap_or_else(|| IfcValue::Binary(law["case"].as_str().unwrap().into()));
        let mutations = vec![IfcMutation::SetEntityArg(SetEntityArg { id: source.entities[0].id, index, value: replacement })];
        if law["id"] == "preserved-format-unknown" { source.entities[0].args[0] = IfcValue::Unset; }
        let mut caller = OriginalWorkFixture::new();
        let mut overlay = PopulationOverlay::default();
        let mut receiver = PopulationReceiver::default();
        for _ in 0..100000 { if caller.work_turn(grant(), 1, |cx| overlay.advance(&source, &mutations, cx)).unwrap() { break; } }
        for _ in 0..100000 { if caller.work_turn(grant(), 1, |cx| receiver.advance(&overlay, &source, &mutations, cx)).unwrap() { break; } }
        let environment = MachineEnvironment { overlay: Some(&overlay), receiver: Some(&receiver), source: Some(&source), mutations: &mutations };
        let position = overlay.find(source.entities[0].id).unwrap();
        let mut machine = RetainedMachine::default();
        assert!(caller.work_turn(grant(), 1, |cx| machine.start_local(position, cx)).unwrap());
        let mut complete = false;
        for turn in 0..1000000 {
            let demand = machine.demands().unwrap();
            for axis in 0..5 {
                let mut loan = grant();
                match axis {
                    0 => loan.maximum_items = 0,
                    1 if demand.copy_bytes > 0 => loan.maximum_copy_bytes = demand.copy_bytes - 1,
                    2 if demand.capacity_bytes > 0 => loan.maximum_capacity_bytes = demand.capacity_bytes - 1,
                    3 if demand.release_bytes > 0 => loan.maximum_release_bytes = demand.release_bytes - 1,
                    4 => loan.maximum_depth = 0,
                    _ => continue,
                }
                caller.work_turn(loan, 1, |cx| { let before = cx.retained_progress(); assert!(!machine.advance_local(&environment, cx)?); assert_eq!(before, cx.retained_progress()); Ok(()) }).unwrap();
            }
            if caller.work_turn(grant(), 1, |cx| machine.advance_local(&environment, cx)).unwrap() { complete = true; break; }
            assert!(turn < 999999);
        }
        assert!(complete);
        assert_eq!(machine.local_status().unwrap(), law["status"].as_str().unwrap());
        let report = machine.report().unwrap();
        if law["status"] != "incomplete" { assert!(report.results.iter().any(|record| record.owner == law["owner"].as_str() && record.rule == law["rule"].as_str())); }
        caller.cancel().cancel_now();
        machine.begin_close(); receiver.begin_close(); overlay.begin_close();
        for _ in 0..100000 {
            if machine.terminal_is_empty() && receiver.terminal_is_empty() && overlay.terminal_is_empty() { break; }
            caller.turn(grant(), |cx| cx.with_original_retirement(|cx| {
                if !machine.terminal_is_empty() { return semio_framework_job::close_original_retained_step(cx, machine.close_demands(4096)?, |loan| machine.close_step(loan)); }
                if !receiver.terminal_is_empty() { return semio_framework_job::close_original_retained_step(cx, receiver.close_demands(4096)?, |loan| receiver.close_step(loan)); }
                semio_framework_job::close_original_retained_step(cx, overlay.close_demands(4096)?, |loan| overlay.close_step(loan))
            })).unwrap();
        }
        assert!(machine.terminal_is_empty() && receiver.terminal_is_empty() && overlay.terminal_is_empty());
        caller.finish();
    }
}
