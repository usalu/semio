use super::*;

#[semio_framework_async_macros::async_test]
async fn create_pdf14_viewer_builds_a_definition_for_the_viewer_role() {
    let def = create_pdf14_viewer();
    assert_eq!(def.role, semio_framework_plugin::AppRole::Viewer);
    assert_eq!(def.dialect, PDF14_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn viewer_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<Pdf14Viewer as ArtifactViewer>::DIALECT, PDF14_DIALECT);
}

#[semio_framework_async_macros::async_test]
async fn viewer_declares_the_main_window() {
    let def = create_pdf14_viewer();
    assert!(def.window_kinds.iter().any(|w| w.id == main::WINDOW_KIND_ID));
}

#[test]
fn own14_declared_document_and_mutation_owners_are_exact() {
    assert_eq!(<Pdf14Viewer as ArtifactViewer>::DOCUMENT_SCHEMA, crate::STDIO_PDF_DOCUMENT_SCHEMA);
    assert_eq!(std::any::type_name::<<Pdf14Viewer as ArtifactViewer>::Snapshot>(), std::any::type_name::<crate::standards::v1_4::subsets::base::schema::snapshot::PdfSnapshot>());
    assert_eq!(std::any::type_name::<<Pdf14Viewer as ArtifactViewer>::Mutation>(), std::any::type_name::<crate::standards::v1_4::subsets::base::schema::mutations::PdfMutation>());
}
