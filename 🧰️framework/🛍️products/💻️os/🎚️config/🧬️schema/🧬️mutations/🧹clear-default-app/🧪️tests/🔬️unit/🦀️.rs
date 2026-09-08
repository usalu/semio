use super::*;

#[test]
fn label_names_role_and_dialect() {
    let dialect = ArtifactDialect { artifact_kind: "s.cad.cad".to_string(), standard: "1".to_string(), subset: "*".to_string() };
    let payload = ClearDefaultApp { dialect, role: AppRole::Viewer };
    assert_eq!(MutationKind::<OpeningPreferences, OpeningConfigMutation>::label(&payload), "Clear default viewer for \"s.cad.cad@1/*\"");
}

#[test]
fn absent_coordinate_has_no_inverse() {
    let dialect = ArtifactDialect { artifact_kind: "s.cad.cad".to_string(), standard: "1".to_string(), subset: "*".to_string() };
    let payload = ClearDefaultApp { dialect, role: AppRole::Viewer };
    assert!(MutationKind::<OpeningPreferences, OpeningConfigMutation>::inverse(&payload, &OpeningPreferences::default()).is_empty());
}

#[test]
fn absent_coordinate_is_a_warned_no_op() {
    let dialect = ArtifactDialect { artifact_kind: "s.cad.cad".to_string(), standard: "1".to_string(), subset: "*".to_string() };
    let payload = ClearDefaultApp { dialect, role: AppRole::Viewer };
    let base = OpeningPreferences::default();
    let outcome = MutationKind::<OpeningPreferences, OpeningConfigMutation>::diff(&payload, &base);
    assert_eq!(outcome.worst_level(), Some(protocol::Severity::Warning));
    assert!(outcome.messages().iter().any(|message| message.code.0 == "mutation.no-op"));
    assert_eq!(outcome.diff(), &base);
}
