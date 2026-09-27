use super::*;

#[semio_framework_async_macros::async_test]
async fn create_editor_builds_a_definition_for_the_editor_role() {
    let def = create_bcf_any_editor();
    assert_eq!(def.role, semio_framework_plugin::AppRole::Editor);
    assert_eq!(def.dialect, BCF_ANY_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn editor_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<BcfAnyEditor as ArtifactEditor>::DIALECT, BCF_ANY_DIALECT);
}

#[semio_framework_async_macros::async_test]
async fn editor_and_viewer_share_one_dialect() {
    semio_framework_plugin::artifact_app_laws::assert_editor_and_viewer_share_dialect::<BcfAnyEditor, crate::viewer::bcf::BcfAnyViewer>().await;
}

#[test]
fn set_cell_requires_its_full_address_and_accepts_an_empty_value() {
    let args = dsl::DslValue::object([
        ("row".into(), dsl::DslValue::Number(dsl::Number::UInt(1))),
        ("column".into(), dsl::DslValue::Number(dsl::Number::UInt(2))),
        ("revision".into(), dsl::DslValue::String("rev".into())),
        ("value".into(), dsl::DslValue::String(String::new())),
    ]);
    assert_eq!(bcf_command_from_action("set-cell", Some(&args)).expect("complete cell address"), BcfAnyEditCommand::SetCell { row: 1, column: 2, revision: "rev".into(), value: String::new() });
    assert!(bcf_command_from_action("set-cell", None).is_err());
    let missing_value = dsl::DslValue::object([("row".into(), dsl::DslValue::Number(dsl::Number::UInt(1))), ("column".into(), dsl::DslValue::Number(dsl::Number::UInt(2)))]);
    assert!(bcf_command_from_action("set-cell", Some(&missing_value)).is_err());
}

#[semio_framework_async_macros::async_test]
async fn set_cell_reaches_the_document_through_its_exact_retained_factory() {
    use semio_framework_plugin::PluginApp;
    let source = bcf_example_snapshot(crate::examples::demo::ID);
    assert!(!source.topics.is_empty(), "the BCF example must contain an editable topic");
    let mut app = semio_framework_plugin::artifact_app_laws::new_registered_app::<EditorApp<BcfAnyEditor>, _>(async { semio_framework_plugin::App { definition: create_bcf_any_editor(), examples: Vec::new() } }).await;
    let semio_framework_plugin::Effect::LoadDocument { pack, spr } = semio_s_artifact_stdio_contract::load_example_effect(&source, BCF_ANY_DOCUMENT_SCHEMA) else { panic!("the fixture load is a complete document effect") };
    app.load_document_pack(&store::ArtifactPackFiles { pack, spr, ops: String::new() }).await.expect("load BCF fixture");
    let revision = semio_s_artifact_stdio_contract::window_kit_canonical_revision(app.test_document_revision());
    let args = dsl::DslValue::object([
        ("row".into(), dsl::DslValue::Number(dsl::Number::UInt(0))),
        ("column".into(), dsl::DslValue::Number(dsl::Number::UInt(1))),
        ("revision".into(), dsl::DslValue::String(revision)),
        ("value".into(), dsl::DslValue::String("Retained title".into())),
    ]);
    let meta = semio_framework_plugin::artifact_app_laws::meta("local");
    app.handle_action("set-cell", Some(&args), &meta).await.expect("dispatch set-cell");
    semio_framework_plugin::artifact_app_laws::settle_registered_typed_operation(&mut app, meta.instance_id).await.expect("settle set-cell");
    assert_eq!(app.snapshot().expect("BCF snapshot").topics[0].title, "Retained title");
    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut app);
}

#[test]
fn set_cell_rejects_stale_revisions_and_addresses_without_mutation() {
    let source = bcf_example_snapshot(crate::examples::demo::ID);
    assert!(bcf_emit(&BcfAnyEditCommand::SetCell { row: 0, column: 0, revision: "stale".into(), value: "x".into() }, &source).is_err());
    let revision = semio_s_artifact_stdio_contract::window_kit_snapshot_revision(&source);
    assert!(bcf_emit(&BcfAnyEditCommand::SetCell { row: u32::MAX, column: 0, revision: revision.clone(), value: "x".into() }, &source).is_err());
    assert!(bcf_emit(&BcfAnyEditCommand::SetCell { row: 0, column: u32::MAX, revision, value: "x".into() }, &source).is_err());
}
