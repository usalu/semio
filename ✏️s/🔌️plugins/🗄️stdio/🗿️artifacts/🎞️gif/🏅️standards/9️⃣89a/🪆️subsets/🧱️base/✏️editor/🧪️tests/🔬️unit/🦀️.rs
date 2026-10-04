use super::*;

#[semio_framework_async_macros::async_test]
async fn create_editor_builds_a_definition_for_the_editor_role() {
    let def = create_gif_89a_editor();
    assert_eq!(def.role, semio_framework::AppRole::Editor);
    assert_eq!(def.dialect, GIF_89A_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn editor_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<Gif89aEditor as ArtifactEditor>::DIALECT, GIF_89A_DIALECT);
}

semio_framework_plugin::history_edit_acceptance_law!("stdio", super::Gif89aEditor, || semio_framework_plugin::App { definition: super::create_gif_89a_editor(), examples: Vec::new() }, "../../🏅️standards/9️⃣89a/🪆️subsets/🧱️base");
