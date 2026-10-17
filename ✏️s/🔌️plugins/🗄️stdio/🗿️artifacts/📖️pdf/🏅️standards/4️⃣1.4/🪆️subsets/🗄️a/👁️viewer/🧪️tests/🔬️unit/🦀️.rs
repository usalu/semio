use crate::viewer::pdf14a::modes::view::windows::main;
use crate::standards::v1_4::subsets::base::schema::{mutations::PdfMutation, snapshot::PdfSnapshot};
use crate::standards::v1_4::subsets::base::schema::snapshot::STDIO_PDF_DOCUMENT_SCHEMA;
use semio_framework_plugin::{ArtifactViewer, Viewer};
#[allow(unused_imports)]
use crate::viewer::pdf14a::modes::view;
#[allow(unused_imports)]
use crate::standards::v1_7::subsets::base::schema::PDF_ARTIFACT_SCHEMA_ID;
#[allow(unused_imports)]
use {semio_framework_plugin::built_to_component_tree, semio_framework_plugin::ArtifactView, semio_framework_plugin::ComponentTree, semio_framework_plugin::ConfigView, semio_framework_artifact_reference::Dialect, semio_framework_plugin::Fault, semio_framework_plugin::NoConfig, semio_framework_plugin::NoConfigMutation, semio_framework_plugin::NoPresence, semio_framework_plugin::NoPresenceMutation, semio_framework_plugin::NoTransient, semio_framework_plugin::NoTransientMutation, semio_framework_artifact_reference::StandardId, semio_framework_artifact_reference::SubsetId, semio_framework_plugin::ViewEmit};
use crate::viewer::pdf14a::*;

#[semio_framework_async_macros::async_test]
async fn create_pdf14_a_viewer_builds_a_definition_for_the_viewer_role() {
    let def = create_pdf14_a_viewer();
    assert_eq!(def.role, semio_framework_plugin::AppRole::Viewer);
    assert_eq!(def.dialect, PDF14A_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn viewer_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<Pdf14AViewer as ArtifactViewer>::DIALECT, PDF14A_DIALECT);
}

#[semio_framework_async_macros::async_test]
async fn viewer_declares_the_main_window() {
    let def = create_pdf14_a_viewer();
    assert!(def.window_kinds.iter().any(|w| w.id == main::WINDOW_KIND_ID));
}

#[test]
fn own14_declared_document_and_mutation_owners_are_exact() {
    assert_eq!(<Pdf14AViewer as ArtifactViewer>::DOCUMENT_SCHEMA, crate::standards::v1_4::subsets::base::schema::snapshot::STDIO_PDF_DOCUMENT_SCHEMA);
    assert_eq!(std::any::type_name::<<Pdf14AViewer as ArtifactViewer>::Snapshot>(), std::any::type_name::<crate::standards::v1_4::subsets::base::schema::snapshot::PdfSnapshot>());
    assert_eq!(std::any::type_name::<<Pdf14AViewer as ArtifactViewer>::Mutation>(), std::any::type_name::<crate::standards::v1_4::subsets::base::schema::mutations::PdfMutation>());
}
