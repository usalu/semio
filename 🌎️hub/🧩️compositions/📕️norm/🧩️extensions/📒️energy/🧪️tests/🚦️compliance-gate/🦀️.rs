//! 🚦️ Compliance gate — examples, locales, paths, remedies, annex divergence for the families this plugin owns.

#![allow(clippy::type_complexity)]

use std::fmt::Write as _;

include!("../../../../🧪️tests/🚦️compliance-gate/🧰️harness/🦀️.rs");

#[test]
fn compliance_gate_all_families() {
    let mut results = Vec::new();

    results.push(gate_family::<semio_s_artifact_norm_din18599_editor::DinV18599Family>(
        "din18599",
        &[
            ("demo", semio_s_artifact_norm_din18599_examples::demo::PRIMARY_TEXT),
            ("compliant_detached", semio_s_artifact_norm_din18599_examples::compliant_detached::PRIMARY_TEXT),
            ("noncompliant_detached", semio_s_artifact_norm_din18599_examples::noncompliant_detached::PRIMARY_TEXT),
        ],
    ));

    let mut summary = String::new();
    let mut failed = 0usize;
    for result in &results {
        let _ = writeln!(summary, "{}", result.summary());
        if !result.ok() {
            failed += 1;
        }
    }
    assert_eq!(results.len(), 1, "gate must cover all 1 families this plugin owns");
    if failed > 0 {
        panic!("{failed}/1 families failed compliance gate:\n{summary}");
    }
}
