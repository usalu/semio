//! 🚦️ Compliance gate — examples, locales, paths, remedies, annex divergence for the families this plugin owns.

#![allow(clippy::type_complexity)]

use std::fmt::Write as _;

include!("../../../../🧪️tests/🚦️compliance-gate/🧰️harness/🦀️.rs");

#[test]
fn compliance_gate_all_families() {
    let mut results = Vec::new();

    results.push(gate_family::<semio_s_artifact_norm_en1992_editor::En1992Family>(
        "en1992",
        &[
            ("compliant_office_frame", semio_s_artifact_norm_en1992_examples::compliant_office_frame::PRIMARY_TEXT),
            ("failing_under_reinforced", semio_s_artifact_norm_en1992_examples::failing_under_reinforced::PRIMARY_TEXT),
            ("liquid_retaining_fem_anchor", semio_s_artifact_norm_en1992_examples::liquid_retaining_fem_anchor::PRIMARY_TEXT),
        ],
    ));
    results.push(gate_family::<semio_s_artifact_norm_en1993_editor::En1993Family>(
        "en1993",
        &[
            ("heb240_compliant", semio_s_artifact_norm_en1993_examples::heb240_compliant::PRIMARY_TEXT),
            ("high_strength_connection", semio_s_artifact_norm_en1993_examples::high_strength_connection::PRIMARY_TEXT),
        ],
    ));
    results.push(gate_family::<semio_s_artifact_norm_en1994_editor::En1994Family>(
        "en1994",
        &[
            ("composite_floor_beam", semio_s_artifact_norm_en1994_examples::composite_floor_beam::PRIMARY_TEXT),
            ("composite_floor_beam_failing", semio_s_artifact_norm_en1994_examples::composite_floor_beam_failing::PRIMARY_TEXT),
            ("composite_bridge_girder", semio_s_artifact_norm_en1994_examples::composite_bridge_girder::PRIMARY_TEXT),
        ],
    ));
    results.push(gate_family::<semio_s_artifact_norm_en1995_editor::En1995Family>(
        "en1995",
        &[
            ("glulam_footbridge", semio_s_artifact_norm_en1995_examples::glulam_footbridge::PRIMARY_TEXT),
            ("multi_fail_timber", semio_s_artifact_norm_en1995_examples::multi_fail_timber::PRIMARY_TEXT),
        ],
    ));
    results.push(gate_family::<semio_s_artifact_norm_en1996_editor::En1996Family>(
        "en1996",
        &[
            ("loadbearing_wall", semio_s_artifact_norm_en1996_examples::loadbearing_wall::PRIMARY_TEXT),
            ("multi_fail_masonry", semio_s_artifact_norm_en1996_examples::multi_fail_masonry::PRIMARY_TEXT),
        ],
    ));
    results.push(gate_family::<semio_s_artifact_norm_en1999_editor::En1999Family>(
        "en1999",
        &[
            ("aluminium_roof_purlin", semio_s_artifact_norm_en1999_examples::aluminium_roof_purlin::PRIMARY_TEXT),
            ("noncompliant_multi_fail", semio_s_artifact_norm_en1999_examples::noncompliant_multi_fail::PRIMARY_TEXT),
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
    assert_eq!(results.len(), 6, "gate must cover all 6 families this plugin owns");
    if failed > 0 {
        panic!("{failed}/6 families failed compliance gate:\n{summary}");
    }
}
