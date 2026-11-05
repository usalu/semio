use crate::viewer::pdf14::modes::view::windows::main;
use crate::standards::v1_4::subsets::base::schema::{mutations::PdfMutation, snapshot::PdfSnapshot};
use crate::standards::v1_4::subsets::base::schema::snapshot::STDIO_PDF_DOCUMENT_SCHEMA;
use semio_plugin::{ArtifactViewer, Viewer};
#[allow(unused_imports)]
use crate::viewer::pdf14::modes::view;
#[allow(unused_imports)]
use crate::standards::v1_7::subsets::base::schema::PDF_ARTIFACT_SCHEMA_ID;
#[allow(unused_imports)]
use {semio_plugin::built_to_component_tree, semio_plugin::ArtifactView, semio_plugin::ComponentTree, semio_plugin::ConfigView, semio_framework_artifact_reference::Dialect, semio_plugin::Fault, semio_plugin::NoConfig, semio_plugin::NoConfigMutation, semio_plugin::NoPresence, semio_plugin::NoPresenceMutation, semio_plugin::NoTransient, semio_plugin::NoTransientMutation, semio_framework_artifact_reference::StandardId, semio_framework_artifact_reference::SubsetId, semio_plugin::ViewEmit};
use crate::viewer::pdf14::*;

#[semio_framework_async_macros::async_test]
async fn create_pdf14_viewer_builds_a_definition_for_the_viewer_role() {
    let def = create_pdf14_viewer();
    assert_eq!(def.role, semio_plugin::AppRole::Viewer);
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
    assert_eq!(<Pdf14Viewer as ArtifactViewer>::DOCUMENT_SCHEMA, crate::standards::v1_4::subsets::base::schema::snapshot::STDIO_PDF_DOCUMENT_SCHEMA);
    assert_eq!(std::any::type_name::<<Pdf14Viewer as ArtifactViewer>::Snapshot>(), std::any::type_name::<crate::standards::v1_4::subsets::base::schema::snapshot::PdfSnapshot>());
    assert_eq!(std::any::type_name::<<Pdf14Viewer as ArtifactViewer>::Mutation>(), std::any::type_name::<crate::standards::v1_4::subsets::base::schema::mutations::PdfMutation>());
}
