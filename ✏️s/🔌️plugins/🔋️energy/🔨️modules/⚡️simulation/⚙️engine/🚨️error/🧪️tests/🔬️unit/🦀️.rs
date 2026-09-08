
use super::*;

#[test]
fn fatal_has_correct_severity() {
    let e = Error::fatal("bad model");
    assert_eq!(e.severity, Severity::Fatal);
}

#[test]
fn severe_and_warning_severities() {
    assert_eq!(Error::severe("x").severity, Severity::Severe);
    assert_eq!(Error::warning("x").severity, Severity::Warning);
}

#[test]
fn with_context_sets_context() {
    let e = Error::fatal("bad").with_context("zone1");
    assert_eq!(e.context.as_deref(), Some("zone1"));
}

#[test]
fn display_includes_context_when_present() {
    let with_ctx = Error::severe("oops").with_context("surf1");
    assert!(format!("{with_ctx}").contains("surf1"));
    let without_ctx = Error::warning("hmm");
    assert!(!format!("{without_ctx}").contains('('));
}

#[test]
fn diagnostics_push_has_fatal_and_merge() {
    let mut diag = Diagnostics::default();
    assert!(!diag.has_fatal());
    diag.push(Error::warning("minor"));
    assert!(!diag.has_fatal());
    diag.push(Error::fatal("boom"));
    assert!(diag.has_fatal());

    let mut other = Diagnostics::default();
    other.push(Error::severe("other issue"));
    diag.merge(other);
    assert_eq!(diag.messages.len(), 3);
}
