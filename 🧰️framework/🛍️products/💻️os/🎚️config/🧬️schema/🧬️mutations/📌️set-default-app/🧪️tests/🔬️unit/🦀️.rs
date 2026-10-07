use super::*;

#[test]
fn semantic_reference_targets_preserve_independent_identity_components(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../../🧫️fixtures/🧭️semantic-reference-targets/🔣️.json")).unwrap();
 for case in fixture["cases"].as_array().unwrap(){
  let value=&case["dialect"];let text=|key:&str|value[key].as_str().unwrap().to_owned();let dialect=ArtifactDialect{artifact_kind:text("artifactKind"),standard:text("standard"),subset:text("subset")};
  let role=match case["role"].as_str().unwrap(){"viewer"=>AppRole::Viewer,"editor"=>AppRole::Editor,_=>unreachable!()};
  let set=SetDefaultApp{dialect:dialect.clone(),role,app:AppRef{plugin_id:"fixture".into(),app_id:"fixture".into()}};let clear=ClearDefaultApp{dialect,role};
  let actual=MutationKind::<OpeningPreferences,OpeningConfigMutation>::target(&set);let removed=MutationKind::<OpeningPreferences,OpeningConfigMutation>::target(&clear);
  let oracle=serde_json::json!([value["artifactKind"],value["standard"],value["subset"],case["role"]]);assert_eq!(serde_json::to_value(&actual).unwrap(),oracle);assert_eq!(oracle,case["expected"]);assert_eq!(actual,removed);
  eprintln!("[DEBUG] Default app mutation targets retain four owned identity components");
 }
}

#[test]
fn label_names_role_and_dialect() {
    let dialect = ArtifactDialect { artifact_kind: "s.cad.cad".to_string(), standard: "1".to_string(), subset: "*".to_string() };
    let payload = SetDefaultApp { dialect, role: AppRole::Editor, app: AppRef { plugin_id: "cad".to_string(), app_id: "s.cad.cad@1/*#editor".to_string() } };
    assert_eq!(MutationKind::<OpeningPreferences, OpeningConfigMutation>::label(&payload), semio_framework_ui_locale::LocalizedLabel::native("Set default editor for \"s.cad.cad (1, *)\"", "Standard-Editor für \"s.cad.cad (1, *)\" festlegen"));
}

#[test]
fn unpinned_coordinate_inverts_to_clear() {
    let dialect = ArtifactDialect { artifact_kind: "s.cad.cad".to_string(), standard: "1".to_string(), subset: "*".to_string() };
    let payload = SetDefaultApp { dialect: dialect.clone(), role: AppRole::Editor, app: AppRef { plugin_id: "cad".to_string(), app_id: "s.cad.cad@1/*#editor".to_string() } };
    assert_eq!(MutationKind::<OpeningPreferences, OpeningConfigMutation>::inverse(&payload, &OpeningPreferences::default()).expect("valid retained mutation inverse fixture"), vec![OpeningConfigMutation::ClearDefaultApp(ClearDefaultApp { dialect, role: AppRole::Editor })]);
}

#[test]
fn already_pinned_app_is_a_warned_no_op() {
    let dialect = ArtifactDialect { artifact_kind: "s.cad.cad".to_string(), standard: "1".to_string(), subset: "*".to_string() };
    let app = AppRef { plugin_id: "cad".to_string(), app_id: "s.cad.cad@1/*#editor".to_string() };
    let base = OpeningPreferences { defaults: vec![DefaultApp { dialect: dialect.clone(), role: AppRole::Editor, app: app.clone() }] };
    let payload = SetDefaultApp { dialect, role: AppRole::Editor, app };
    let outcome = MutationKind::<OpeningPreferences, OpeningConfigMutation>::diff(&payload, &base);
    assert_eq!(outcome.worst_level(), Some(semio_framework_diagnostic::Severity::Warning));
    assert!(outcome.messages().iter().any(|message| message.code.0 == "mutation.no-op"));
    assert_eq!(outcome.diff(), &base);
}
