//! 🧪️ Full retained population scheduling preserves independent rule identities and valid authoring populations.
use super::{machine::*, overlay::*, receive::*};
use semio_framework_plugin::app_testing::OriginalWorkFixture;
use semio_framework_value::retained_clone::RetainedCloneGrant;
include!("official-cases.rs");
fn run(source: &IfcSnapshot) -> (bool, bool, Vec<String>) {
    let grant = RetainedCloneGrant { maximum_items: 100_000, maximum_copy_bytes: 1_048_576, maximum_capacity_bytes: 64 * 1_048_576, maximum_release_bytes: 64 * 1_048_576, maximum_depth: 256 };
    let stock = RetainedCloneGrant { maximum_items: 1 << 28, maximum_copy_bytes: usize::MAX / 4, maximum_capacity_bytes: 1 << 30, maximum_release_bytes: 1 << 30, maximum_depth: 4096 };
    let mut caller = OriginalWorkFixture::with_source(semio_framework_job::OperationId(925), semio_framework_job::Generation(1), stock, || Some(1), u64::MAX, 1 << 28);
    let mutations: &[IfcMutation] = &[];
    let mut overlay = PopulationOverlay::default();
    while !caller.work_turn(grant, 1, |cx| overlay.advance(source, mutations, cx)).unwrap() {}
    let mut receiver = PopulationReceiver::default();
    while !caller.work_turn(grant, 1, |cx| receiver.advance(&overlay, source, mutations, cx)).unwrap() {}
    let mut machine = RetainedMachine::new(super::ifc4x3_express_program());
    assert!(caller.work_turn(grant, 1, |cx| machine.start_population(cx)).unwrap());
    let env = MachineEnvironment { overlay: Some(&overlay), receiver: Some(&receiver), source: Some(source), mutations };
    let mut finished = false;
    for turn in 0..10_000_100 {
        if caller.work_turn(grant, 1, |cx| machine.advance(&env, cx)).unwrap() {
            finished = true;
            break;
        }
        assert!(turn < 10_000_099);
    }
    assert!(finished);
    let report = machine.report().unwrap();
    let result = (report.complete, report.valid, report.diagnostics.iter().filter_map(|row| Some(format!("{}.{}", row.owner?, row.rule?))).collect());
    machine.begin_close();
    for _ in 0..1_000_000 {
        if machine.terminal_is_empty() {
            break;
        }
        caller.turn(grant, |cx| cx.with_original_retirement(|cx| semio_framework_job::close_original_retained_step(cx, machine.close_demands(4096)?, |loan| machine.close_step(loan)))).unwrap();
    }
    assert!(machine.terminal_is_empty());
    receiver.begin_close();
    while !receiver.terminal_is_empty() {
        caller.turn(grant, |cx| cx.with_original_retirement(|cx| semio_framework_job::close_original_retained_step(cx, receiver.close_demands(4096)?, |loan| receiver.close_step(loan)))).unwrap();
    }
    overlay.begin_close();
    while !overlay.terminal_is_empty() {
        caller.turn(grant, |cx| cx.with_original_retirement(|cx| semio_framework_job::close_original_retained_step(cx, overlay.close_demands(4096)?, |loan| overlay.close_step(loan)))).unwrap();
    }
    caller.finish();
    result
}

#[test]
fn full_retained_scheduler_matches_all_forty_six_pinned_official_populations() {
    let manifest: serde_json::Value = serde_json::from_str(include_str!("../../🧪️fixtures/official/cases.json")).unwrap();
    assert_eq!(OFFICIAL_CASES.len(), 46);
    let mut entities = 0;
    for (name, source, count) in OFFICIAL_CASES {
        let expected = manifest["cases"].as_array().unwrap().iter().find(|row| row["file"].as_str() == Some(*name)).unwrap();
        let graph = crate::io::parse_ifc4x3_step(source).unwrap();
        assert_eq!(graph.entities.len(), *count);
        entities += count;
        let (complete, valid, diagnostics) = run(&graph);
        if expected["diagnostics"].as_array().unwrap().is_empty() {
            assert!(complete && valid, "{name}: {:?}", diagnostics);
        } else {
            assert!(!complete && !valid, "{name}");
        }
    }
    assert_eq!(entities, 8172);
}

#[test]
fn full_retained_scheduler_preserves_both_inherited_derived_binding_populations() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("models.json")).unwrap();
    let cases: serde_json::Value = serde_json::from_str(include_str!("derived-binding-cases.json")).unwrap();
    for case in cases.as_array().unwrap() {
        let graph = crate::io::parse_ifc4x3_step(&format!("{}{}{}", fixture["header"].as_str().unwrap(), case["body"].as_str().unwrap(), fixture["footer"].as_str().unwrap())).unwrap();
        let (complete, valid, diagnostics) = run(&graph);
        assert!(complete, "{}: {:?}", case["name"], diagnostics);
        assert_eq!(valid, case["northFailures"].as_array().unwrap().is_empty());
        assert_eq!(diagnostics.iter().filter(|rule| rule.ends_with(".NORTH2D")).count(), case["northFailures"].as_array().unwrap().len());
    }
}
#[test]
fn retained_population_rules_match_independent_named_failures() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("models.json")).unwrap();
    let source = format!("{}{}{}", fixture["header"].as_str().unwrap(), fixture["body"].as_array().unwrap().iter().map(|row| row.as_str().unwrap()).collect::<Vec<_>>().join("\n"), fixture["footer"].as_str().unwrap());
    let source = crate::io::parse_ifc4x3_step(&source).unwrap();
    let (complete, valid, diagnostics) = run(&source);
    assert!(complete);
    assert!(!valid);
    let actual: std::collections::BTreeSet<_> = diagnostics.into_iter().collect();
    let expected: std::collections::BTreeSet<_> = fixture["expectedFailures"].as_array().unwrap().iter().map(|row| row.as_str().unwrap().to_string()).collect();
    assert_eq!(actual, expected);
}
#[test]
fn full_retained_scheduler_admits_every_independent_template_golden() {
    let fixtures: serde_json::Value = serde_json::from_str(include_str!("../🏷️templates/✍️apply/native-cases.json")).unwrap();
    for case in fixtures["cases"].as_array().unwrap() {
        let source = crate::io::parse_ifc4x3_step(case["expected"].as_str().unwrap()).unwrap();
        let (complete, valid, diagnostics) = run(&source);
        assert!(complete && valid, "{} {:?}", case["name"], diagnostics);
    }
}
#[test]
fn full_retained_scheduler_preserves_complex_derived_resolution() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("models.json")).unwrap();
    let source = format!("{}#1=(IFCREPRESENTATIONITEM() IFCGEOMETRICREPRESENTATIONITEM() IFCDIRECTION((0.,0.,0.)));{}", fixture["header"].as_str().unwrap(), fixture["footer"].as_str().unwrap());
    let (complete, valid, diagnostics) = run(&crate::io::parse_ifc4x3_step(&source).unwrap());
    assert!(complete && !valid);
    assert!(diagnostics.iter().any(|rule| rule == "IFCDIRECTION.MAGNITUDEGREATERZERO"));
}
