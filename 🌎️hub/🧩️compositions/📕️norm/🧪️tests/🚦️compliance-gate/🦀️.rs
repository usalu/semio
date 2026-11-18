//! 🚦️ Compliance gate — examples, locales, paths, remedies, annex divergence for the families this plugin owns.

#![allow(clippy::type_complexity)]

use semio_framework_test_fixtures::fixture_path;
use std::fmt::Write as _;
use std::path::{Path, PathBuf};

/// 🚫 Forbidden evaluate-path gaming tokens (fingerprint / score folds).
const GAMING_TOKENS: &[&str] = &["field_fingerprint", "id_score", "tag_fp", "en1999.en1990.psi"];

/// 🗂️ Walk artifact rust sources under evaluate paths (schema + inferences), skipping tests.
fn walk_evaluate_rs(root: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(root) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        let name = entry.file_name();
        let name = name.to_string_lossy();
        if path.is_dir() {
            if name.contains("tests") || name == "target" || name.starts_with('.') {
                continue;
            }
            walk_evaluate_rs(&path, out);
            continue;
        }
        if !name.ends_with(".rs") {
            continue;
        }
        let normalized = path.to_string_lossy().replace('\\', "/");
        let on_evaluate_path = normalized.contains("/🧬️schema/") || normalized.contains("/inferences/") || normalized.contains("evaluate");
        if on_evaluate_path {
            out.push(path);
        }
    }
}

/// 🧪 True when `.len().max(1)` sits on a quantity / assessor line (check quantity gaming).
fn len_max_one_is_check_quantity(line: &str) -> bool {
    if !line.contains(".len().max(1)") {
        return false;
    }
    line.contains("Quantity::")
        || line.contains(".minimum(")
        || line.contains(".maximum(")
        || line.contains("computed")
        || line.contains("limit")
        || line.contains("utilization")
}

/// 🔬 Field folded through `1e-9 *` (divide-guard `.max(1e-9)` alone is allowed).
fn has_field_epsilon_fold(line: &str) -> bool {
    let trimmed = line.trim();
    if trimmed.contains(".max(1e-9)") && !trimmed.contains("1e-9 *") && !trimmed.contains("1e-9*") {
        return false;
    }
    trimmed.contains("1e-9 *") || trimmed.contains("1e-9*")
}

/// 🛡️ Source-scan: reject fingerprint / score / epsilon-fold / len-as-quantity gaming in evaluate paths.
fn scan_evaluate_gaming() -> Vec<String> {
    let artifacts = fixture_path!("../../../../../✏️s/🔌️plugins/📕️norm/🗿️artifacts");
    let mut files = Vec::new();
    walk_evaluate_rs(&artifacts, &mut files);
    let mut hits = Vec::new();
    for path in files {
        let Ok(text) = std::fs::read_to_string(&path) else {
            continue;
        };
        let display = path.to_string_lossy().replace('\\', "/");
        for (line_no, line) in text.lines().enumerate() {
            let line_no = line_no + 1;
            for token in GAMING_TOKENS {
                if line.contains(token) {
                    hits.push(format!("{display}:{line_no}: token `{token}`"));
                }
            }
            if has_field_epsilon_fold(line) {
                hits.push(format!("{display}:{line_no}: field fold `1e-9 *`"));
            }
            if len_max_one_is_check_quantity(line) {
                hits.push(format!("{display}:{line_no}: `.len().max(1)` used as check quantity"));
            }
        }
    }
    hits
}

include!("🧰️harness/🦀️.rs");

#[test]
fn compliance_gate_all_families() {
    let gaming = scan_evaluate_gaming();
    if !gaming.is_empty() {
        panic!("evaluate-path gaming source-scan failed ({} hits):\n{}", gaming.len(), gaming.join("\n"));
    }

    let mut results = Vec::new();

    results.push(gate_family::<semio_s_artifact_norm_din4108_editor::Din4108Family>(
        "din4108",
        &[
            ("demo", semio_s_artifact_norm_din4108_examples::demo::PRIMARY_TEXT),
            ("failing_thin_insulation", semio_s_artifact_norm_din4108_examples::failing_thin_insulation::PRIMARY_TEXT),
        ],
    ));
    results.push(gate_family::<semio_s_artifact_norm_din16798_editor::DinEn16798Family>(
        "din16798",
        &[
            ("demo", semio_s_artifact_norm_din16798_examples::demo::PRIMARY_TEXT),
            ("compliant_office", semio_s_artifact_norm_din16798_examples::compliant_office::PRIMARY_TEXT),
            ("noncompliant_office", semio_s_artifact_norm_din16798_examples::noncompliant_office::PRIMARY_TEXT),
        ],
    ));
    results.push(gate_family::<semio_s_artifact_norm_iso16757_editor::Iso16757Family>(
        "iso16757",
        &[
            ("demo", semio_s_artifact_norm_iso16757_examples::demo::PRIMARY_TEXT),
            ("broken", semio_s_artifact_norm_iso16757_examples::broken::PRIMARY_TEXT),
        ],
    ));
    results.push(gate_family::<semio_s_artifact_norm_vdi3805_editor::Vdi3805Family>(
        "vdi3805",
        &[
            ("demo", semio_s_artifact_norm_vdi3805_examples::demo::PRIMARY_TEXT),
            ("nonconforming", semio_s_artifact_norm_vdi3805_examples::nonconforming::PRIMARY_TEXT),
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
