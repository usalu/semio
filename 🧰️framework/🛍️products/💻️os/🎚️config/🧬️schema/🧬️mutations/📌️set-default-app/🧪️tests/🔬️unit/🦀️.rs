use super::*;

#[test]
fn label_names_role_and_dialect() {
    let dialect = ArtifactDialect { artifact_kind: "s.cad.cad".to_string(), standard: "1".to_string(), subset: "*".to_string() };
    let payload = SetDefaultApp { dialect, role: AppRole::Editor, app: AppRef { plugin_id: "cad".to_string(), app_id: "s.cad.cad@1/*#editor".to_string() } };
    assert_eq!(MutationKind::<OpeningPreferences, OpeningConfigMutation>::label(&payload), "Set default editor for \"s.cad.cad@1/*\"");
}

#[test]
fn unpinned_coordinate_inverts_to_clear() {
    let dialect = ArtifactDialect { artifact_kind: "s.cad.cad".to_string(), standard: "1".to_string(), subset: "*".to_string() };
    let payload = SetDefaultApp { dialect: dialect.clone(), role: AppRole::Editor, app: AppRef { plugin_id: "cad".to_string(), app_id: "s.cad.cad@1/*#editor".to_string() } };
    assert_eq!(MutationKind::<OpeningPreferences, OpeningConfigMutation>::inverse(&payload, &OpeningPreferences::default()), vec![OpeningConfigMutation::ClearDefaultApp(ClearDefaultApp { dialect, role: AppRole::Editor })]);
}

#[test]
fn already_pinned_app_is_a_warned_no_op() {
    let dialect = ArtifactDialect { artifact_kind: "s.cad.cad".to_string(), standard: "1".to_string(), subset: "*".to_string() };
    let app = AppRef { plugin_id: "cad".to_string(), app_id: "s.cad.cad@1/*#editor".to_string() };
    let base = OpeningPreferences { defaults: vec![DefaultApp { dialect: dialect.clone(), role: AppRole::Editor, app: app.clone() }] };
    let payload = SetDefaultApp { dialect, role: AppRole::Editor, app };
    let outcome = MutationKind::<OpeningPreferences, OpeningConfigMutation>::diff(&payload, &base);
    assert_eq!(outcome.worst_level(), Some(protocol::Severity::Warning));
    assert!(outcome.messages().iter().any(|message| message.code.0 == "mutation.no-op"));
    assert_eq!(outcome.diff(), &base);
}
