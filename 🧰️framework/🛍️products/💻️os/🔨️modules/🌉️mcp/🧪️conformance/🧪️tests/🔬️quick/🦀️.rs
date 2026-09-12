
use super::*;
use crate::catalog::compile;
use crate::source_builders;

fn compiled() -> Catalog {
    compile(&source_builders::note_and_cad_source(), Locale::En, Terminology::Native).expect("compiles")
}

#[test]
fn note_and_cad_fixtures_produce_zero_conformance_findings() {
    let catalog = compiled();
    let findings = check(&catalog);
    assert!(findings.is_empty(), "unexpected findings: {findings:?}");
}

#[test]
fn note_and_cad_fixtures_have_non_empty_bilingual_labels() {
    let findings = check_bilingual_labels(&source_builders::note_and_cad_source());
    assert!(findings.is_empty(), "unexpected bilingual findings: {findings:?}");
}

#[test]
fn mutation_without_writes_is_flagged() {
    let mut catalog = compiled();
    let mut capability = catalog.entries[0].clone();
    capability.kind = CapabilityKind::Mutation;
    capability.effects.writes.clear();
    catalog.entries[0] = capability;
    let findings = check(&catalog);
    assert!(findings.iter().any(|finding| finding.message.contains("declares no writes")));
}

#[test]
fn unknown_scope_is_flagged() {
    let mut catalog = compiled();
    let mut capability = catalog.entries[0].clone();
    capability.policy.scopes.push(semio_framework::manifest::kernel::CapabilityId("not.a.real.scope".to_string()));
    catalog.entries[0] = capability;
    let findings = check(&catalog);
    assert!(findings.iter().any(|finding| finding.message.contains("unknown policy scope")));
}

#[test]
fn bare_action_id_grammar_violation_is_flagged() {
    let mut catalog = compiled();
    let mut capability = catalog.entries[0].clone();
    capability.id = crate::catalog::CapabilityRef("bareActionId".to_string());
    catalog.entries[0] = capability;
    let findings = check(&catalog);
    assert!(findings.iter().any(|finding| finding.message.contains("must start with")));
}

#[test]
fn eval_harness_measures_top1_and_top3_accuracy_deterministically() {
    let catalog = compiled();
    let cases = source_builders::eval_cases(include_str!("../../../🧫️fixtures/🧠️conformance/🔣️.json"));
    let first = run_eval(&catalog, &cases);
    let second = run_eval(&catalog, &cases);
    assert_eq!(first, second, "eval must be fully deterministic");
    assert!(first.total >= 60);
    assert!(first.top1_accuracy() >= 0.0 && first.top1_accuracy() <= 1.0);
    assert!(first.top3_accuracy() >= first.top1_accuracy());
}
