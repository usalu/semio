use super::*;

const BEFORE: &str = r#"{"zoom":1}"#;
const AFTER: &str = r#"{"zoom":2}"#;
const DIFF: &str = r#"{"zoom":2}"#;
const APPLIED: &str = r#"{"status":"applied","messages":[]}"#;
const KEPT: &str = r#"{"status":"no-op","messages":[{"level":"warning","code":"mutation.no-op"}]}"#;

fn vector(before: &'static str, after: &'static str, outcome: &'static str, observable: bool) -> Vector {
    Vector { before, mutation: r#"{"kind":"set-zoom","zoom":2}"#, after, diff: DIFF, outcome, observable }
}

fn report(base: &str, snapshot: &str, expected: &str, messages: &str, restored: &str, inverse_messages: &str) -> String {
    format!(r#"{{"base":{base},"expectedSnapshot":{expected},"snapshot":{snapshot},"diff":{DIFF},"messages":{messages},"inverseSteps":[],"inverseSnapshot":{restored},"inverseMessages":{inverse_messages}}}"#)
}

#[test]
fn an_applied_vector_holds_when_every_committed_part_matches() {
    let applied = mutate("set-zoom", &report(BEFORE, AFTER, AFTER, "[]", BEFORE, "[]"), &vector(BEFORE, AFTER, APPLIED, true)).expect("forward law holds");
    assert_eq!(applied.to_string(), parse_json(AFTER).unwrap().to_string());
    inverse("set-zoom", &report(BEFORE, AFTER, AFTER, "[]", BEFORE, "[]")).expect("inverse law holds");
}

#[test]
fn a_snapshot_that_misses_the_committed_after_fails_with_the_first_divergence() {
    let error = mutate("set-zoom", &report(BEFORE, BEFORE, AFTER, "[]", BEFORE, "[]"), &vector(BEFORE, AFTER, APPLIED, true)).unwrap_err();
    assert!(error.contains("not the committed after-snapshot"), "{error}");
}

#[test]
fn an_applied_vector_that_does_not_move_the_snapshot_fails_the_observability_law() {
    let error = mutate("set-zoom", &report(AFTER, AFTER, AFTER, "[]", AFTER, "[]"), &vector(AFTER, AFTER, APPLIED, true)).unwrap_err();
    assert!(error.contains("observability law violated"), "{error}");
    mutate("set-zoom", &report(AFTER, AFTER, AFTER, "[]", AFTER, "[]"), &vector(AFTER, AFTER, APPLIED, false)).expect("a record with nothing observable is exempt by name");
}

#[test]
fn a_no_op_vector_requires_exactly_the_warned_no_op_diagnostic() {
    let warned = r#"[{"level":"warning","code":"mutation.no-op","message":"unchanged"}]"#;
    mutate("set-zoom", &report(AFTER, AFTER, AFTER, warned, AFTER, "[]"), &vector(AFTER, AFTER, KEPT, true)).expect("a warned no-op holds");
    let error = mutate("set-zoom", &report(AFTER, AFTER, AFTER, "[]", AFTER, "[]"), &vector(AFTER, AFTER, KEPT, true)).unwrap_err();
    assert!(error.contains("declares the diagnostics"), "{error}");
}

#[test]
fn a_refused_inverse_step_fails_the_inverse_law() {
    let refused = r#"[{"level":"fatal","code":"mutation.missing-target","message":"gone"}]"#;
    let error = inverse("set-zoom", &report(BEFORE, AFTER, AFTER, "[]", AFTER, refused)).unwrap_err();
    assert!(error.contains("was refused"), "{error}");
    let error = inverse("set-zoom", &report(BEFORE, AFTER, AFTER, "[]", AFTER, "[]")).unwrap_err();
    assert!(error.contains("inverse law violated"), "{error}");
}
