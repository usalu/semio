use crate::SOURCING_DIALECT;
use semio_framework_plugin::{ArtifactViewer, Viewer};
#[allow(unused_imports)]
use crate::{CurationSnapshot, SOURCING_CURATION_SCHEMA};
#[allow(unused_imports)]
use crate::viewer::sourcing::modes::view;
#[allow(unused_imports)]
use crate::viewer::sourcing::modes::view::windows::pool;
#[allow(unused_imports)]
use {semio_framework_plugin::ArtifactView, semio_framework_plugin::ConfigView, semio_framework_artifact_reference::Dialect, semio_framework_plugin::Fault, semio_framework_plugin::NoConfig, semio_framework_plugin::NoConfigMutation, semio_framework_plugin::NoPresence, semio_framework_plugin::NoPresenceMutation, semio_framework_plugin::NoTransient, semio_framework_plugin::NoTransientMutation, semio_framework_plugin::ViewEmit};
#[allow(unused_imports)]
use semio_framework_plugin::app::InteractionView;
#[allow(unused_imports)]
use semio_framework_2d_compute::EngineHandles;

use crate::viewer::sourcing::*;

#[semio_framework_async_macros::async_test]
async fn create_sourcing_viewer_builds_a_definition_for_the_viewer_role() {
    let def = create_sourcing_viewer();
    assert_eq!(def.role, semio_framework::AppRole::Viewer);
    assert_eq!(def.dialect, SOURCING_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn viewer_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<SourcingViewer as ArtifactViewer>::DIALECT, SOURCING_DIALECT);
}
