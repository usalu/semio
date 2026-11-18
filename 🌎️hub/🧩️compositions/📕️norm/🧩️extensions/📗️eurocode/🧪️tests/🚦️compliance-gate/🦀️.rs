//! 🚦️ Compliance gate — examples, locales, paths, remedies, annex divergence for the families this plugin owns.

#![allow(clippy::type_complexity)]

use std::fmt::Write as _;

include!("../../../../🧪️tests/🚦️compliance-gate/🧰️harness/🦀️.rs");

/// 🖨️ Leak a printed DSL document for gate example registration.
fn leaked_print_dsl<D: pack::snapshot::ArtifactDsl>(doc: &D) -> &'static str {
    Box::leak(doc.print_dsl().into_boxed_str())
}

#[test]
fn compliance_gate_all_families() {
    let mut results = Vec::new();

    results.push(gate_family::<semio_s_artifact_norm_en1990_editor::En1990Family>(
        "en1990",
        &[
            (
                "high_consequence_office",
                semio_s_artifact_norm_en1990_examples::high_consequence_office::PRIMARY_TEXT,
            ),
            (
                "road_bridge_compliant",
                semio_s_artifact_norm_en1990_examples::road_bridge_compliant::PRIMARY_TEXT,
            ),
            (
                "road_bridge_failing",
                semio_s_artifact_norm_en1990_examples::road_bridge_failing::PRIMARY_TEXT,
            ),
        ],
    ));
    results.push(gate_family::<semio_s_artifact_norm_en1991_editor::En1991Family>(
        "en1991",
        &[
            ("de_office_compliant", semio_s_artifact_norm_en1991_examples::de_office_compliant::PRIMARY_TEXT),
            ("multi_fail_noncompliant", semio_s_artifact_norm_en1991_examples::multi_fail_noncompliant::PRIMARY_TEXT),
        ],
    ));
    results.push(gate_family::<semio_s_artifact_norm_en1997_editor::En1997Family>(
        "en1997",
        &[
            ("demo", semio_s_artifact_norm_en1997_examples::demo::PRIMARY_TEXT),
            ("compliant", semio_s_artifact_norm_en1997_examples::compliant::PRIMARY_TEXT),
            ("noncompliant", semio_s_artifact_norm_en1997_examples::noncompliant::PRIMARY_TEXT),
        ],
    ));
    results.push(gate_family::<semio_s_artifact_norm_en1998_editor::En1998Family>(
        "en1998",
        &[
            (
                "seismic_rc_frame",
                leaked_print_dsl(&semio_s_artifact_norm_en1998_examples::seismic_rc_frame::snapshot()),
            ),
            (
                "seismic_rc_frame_fail",
                leaked_print_dsl(&semio_s_artifact_norm_en1998_examples::seismic_rc_frame_fail::snapshot()),
            ),
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
    assert_eq!(results.len(), 4, "gate must cover all 4 families this plugin owns");
    if failed > 0 {
        panic!("{failed}/4 families failed compliance gate:\n{summary}");
    }
}
