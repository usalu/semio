use super::*;

/// 🧮️ Every stdio artifact binds exactly the executables its schema registers; all divergences are reported at once.
#[test]
fn every_stdio_artifact_executable_mapping_matches_its_schema_registrations() {
    let contributions = selected_contributions();
    assert!(contributions.len() >= 30, "expected the full stdio roster, found {}", contributions.len());
    let divergent = contributions
        .iter()
        .filter_map(|contribution| (contribution.definition)().err().map(|error| format!("{}: {}", contribution.identity, error.message)))
        .collect::<Vec<_>>();
    assert!(divergent.is_empty(), "{} of {} artifacts diverge:\n{}", divergent.len(), contributions.len(), divergent.join("\n"));
}
