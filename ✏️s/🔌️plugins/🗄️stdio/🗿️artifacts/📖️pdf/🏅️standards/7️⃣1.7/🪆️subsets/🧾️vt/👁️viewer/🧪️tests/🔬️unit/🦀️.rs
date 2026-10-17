use crate::viewer::pdf17vt::modes::view::windows::main;
use semio_framework_plugin::{ArtifactViewer, Viewer};
#[allow(unused_imports)]
use crate::viewer::pdf17vt::modes::view;
#[allow(unused_imports)]
use crate::standards::v1_7::subsets::base::schema::mutations::PdfMutation;
#[allow(unused_imports)]
use crate::standards::v1_7::subsets::base::schema::snapshot::{PdfSnapshot, STDIO_PDF17_DOCUMENT_SCHEMA};
#[allow(unused_imports)]
use crate::standards::v1_7::subsets::base::schema::PDF_ARTIFACT_SCHEMA_ID;
#[allow(unused_imports)]
use {semio_framework_plugin::built_to_component_tree, semio_framework_plugin::ArtifactView, semio_framework_plugin::ComponentTree, semio_framework_plugin::ConfigView, semio_framework_artifact_reference::Dialect, semio_framework_plugin::Fault, semio_framework_plugin::NoConfig, semio_framework_plugin::NoConfigMutation, semio_framework_plugin::NoPresence, semio_framework_plugin::NoPresenceMutation, semio_framework_plugin::NoTransient, semio_framework_plugin::NoTransientMutation, semio_framework_artifact_reference::StandardId, semio_framework_artifact_reference::SubsetId, semio_framework_plugin::ViewEmit};
use crate::viewer::pdf17vt::*;

#[semio_framework_async_macros::async_test]
async fn create_pdf17_vt_viewer_builds_a_definition_for_the_viewer_role() {
    let def = create_pdf17_vt_viewer();
    assert_eq!(def.role, semio_framework_plugin::AppRole::Viewer);
    assert_eq!(def.dialect, PDF17VT_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn viewer_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<Pdf17VtViewer as ArtifactViewer>::DIALECT, PDF17VT_DIALECT);
}

#[semio_framework_async_macros::async_test]
async fn viewer_declares_the_main_window() {
    let def = create_pdf17_vt_viewer();
    assert!(def.window_kinds.iter().any(|w| w.id == main::WINDOW_KIND_ID));
}
