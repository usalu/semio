use super::*;

#[semio_framework_async_macros::async_test]
async fn create_zip_iso21320_editor_builds_a_definition_for_the_editor_role() {
    let def = create_zip_iso21320_editor();
    assert_eq!(def.role, semio_framework_plugin::AppRole::Editor);
    assert_eq!(def.dialect, ZIP_ISO21320_EDITOR_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn editor_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<ZipIso21320Editor as ArtifactEditor>::DIALECT, ZIP_ISO21320_EDITOR_DIALECT);
}

#[semio_framework_async_macros::async_test]
async fn editor_declares_the_main_window() {
    let def = create_zip_iso21320_editor();
    assert!(def.window_kinds.iter().any(|window| window.id == main::WINDOW_KIND_ID));
}

#[semio_framework_async_macros::async_test]
async fn editor_mounts_the_bounded_zip_preparation_route() {
    use semio_s_artifact_stdio_contract::editing::BoundedNativeEditingEditor;

    assert!(ZipIso21320Editor::native_edit_preparation_route("stdio-zip-iso21320-snapshot-edit").is_some());
}

semio_framework_plugin::history_edit_acceptance_law!("stdio", super::ZipIso21320Editor, || semio_framework_plugin::App { definition: super::create_zip_iso21320_editor(), examples: Vec::new() }, "../../🏅️standards/🔖️2.0/🪆️subsets/🌐️iso21320");
