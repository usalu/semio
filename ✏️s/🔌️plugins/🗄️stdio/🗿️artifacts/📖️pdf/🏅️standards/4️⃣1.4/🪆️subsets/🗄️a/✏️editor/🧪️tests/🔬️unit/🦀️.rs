use super::*;

#[semio_framework_async_macros::async_test]
async fn create_pdf14_a_editor_builds_a_definition_for_the_editor_role() {
    let def = create_pdf14_a_editor();
    assert_eq!(def.role, semio_framework_plugin::AppRole::Editor);
    assert_eq!(def.dialect, PDF14A_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn editor_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<Pdf14AEditor as ArtifactEditor>::DIALECT, PDF14A_DIALECT);
}

#[semio_framework_async_macros::async_test]
async fn editor_declares_the_main_window() {
    let def = create_pdf14_a_editor();
    assert!(def.window_kinds.iter().any(|w| w.id == main::WINDOW_KIND_ID));
}

#[semio_framework_async_macros::async_test]
async fn missing_set_page_payload_is_rejected() {
    assert!(<Pdf14AEditor as ArtifactEditor>::command_from_action("set-page", None).is_err());
}

#[semio_framework_async_macros::async_test]
async fn explicit_nonzero_page_payload_is_preserved() {
    let args = semio_framework_value::DslValue::Object(vec![
        ("page".into(), semio_framework_value::DslValue::float(3.0)),
        ("item".into(), semio_framework_value::DslValue::float(0.0)),
        ("revision".into(), semio_framework_value::DslValue::String("0123456789abcdef".into())),
        ("text".into(), semio_framework_value::DslValue::String("replacement".into())),
    ]);
    let command = <Pdf14AEditor as ArtifactEditor>::command_from_action("set-page", Some(&args)).expect("typed payload");
    assert!(matches!(command, semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Native(Pdf14AEditorCommand::SetPage { page: 3, item: 0, revision, text }) if revision == "0123456789abcdef" && text == "replacement"));
}

#[test]
fn own14_declared_document_and_mutation_owners_are_exact() {
    assert_eq!(<Pdf14AEditor as ArtifactEditor>::DOCUMENT_SCHEMA, crate::STDIO_PDF_DOCUMENT_SCHEMA);
    assert_eq!(std::any::type_name::<<Pdf14AEditor as ArtifactEditor>::Snapshot>(), std::any::type_name::<crate::standards::v1_4::subsets::base::schema::snapshot::PdfSnapshot>());
    assert_eq!(std::any::type_name::<<Pdf14AEditor as ArtifactEditor>::Mutation>(), std::any::type_name::<crate::standards::v1_4::subsets::base::schema::mutations::PdfMutation>());
}

#[test]
fn own14_typed_page_command_uses_the_own_mutation_codec() {
    use protocol::{OpBinary, OpText, Mutation, MutationDiff};
    let args = semio_framework_value::DslValue::Object(vec![("page".into(), semio_framework_value::DslValue::float(0.0)), ("width".into(), semio_framework_value::DslValue::float(8.0)), ("height".into(), semio_framework_value::DslValue::float(9.0))]);
    let command = <Pdf14AEditor as ArtifactEditor>::command_from_action("set-page-size", Some(&args)).unwrap();
    let semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Native(native) = command else { panic!("own native page command"); };
    assert_eq!(Pdf14AEditorCommand::decode_op(&native.encode_op().unwrap()).unwrap(), native);
    assert_eq!(Pdf14AEditorCommand::parse_op(&native.print_op()).unwrap(), native);
    let Pdf14AEditorCommand::PageEdit { action, payload } = native else { panic!("actual own page action"); };
    let original = <Pdf14AEditor as ArtifactEditor>::initial_snapshot();
    let emit = page::emit_page_edit(&original, &action, &payload).unwrap();
    assert_eq!(emit.artifact_mutations.len(), 1);
    let op = &emit.artifact_mutations[0];
    let next = protocol::apply_diff(op.diff(&original).diff(), &original).unwrap();
    assert_eq!(next.pages[0].width.to_bits(), 8.0f64.to_bits());
    assert_eq!(next.pages[0].height.to_bits(), 9.0f64.to_bits());
    assert_eq!(next.pages[0].text, original.pages[0].text);
    let mut restored = next;
    for inverse in op.inverse(&original).unwrap() { restored = protocol::apply_diff(inverse.diff(&restored).diff(), &restored).unwrap(); }
    assert_eq!(restored, original);
}

semio_framework_plugin::history_edit_acceptance_law!("stdio/Pdf14AEditor", Pdf14AEditor, || semio_framework_plugin::App { definition: create_pdf14_a_editor(), examples: Vec::new() }, "../..");
