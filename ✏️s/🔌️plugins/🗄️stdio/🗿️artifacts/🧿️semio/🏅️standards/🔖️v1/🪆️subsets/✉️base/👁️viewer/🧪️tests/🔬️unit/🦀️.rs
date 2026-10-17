use semio_framework_plugin::ArtifactViewer;
use semio_framework_plugin::Viewer;
#[allow(unused_imports)]
use crate::schema::mutations::SemioMutation;
#[allow(unused_imports)]
use crate::schema::snapshot::SemioSnapshot;
#[allow(unused_imports)]
use crate::viewer::modes::view;
#[allow(unused_imports)]
use crate::viewer::modes::view::windows::main;
#[allow(unused_imports)]
use semio_framework_plugin::app::InteractionView;
#[allow(unused_imports)]
use semio_framework_plugin::ArtifactView;
#[allow(unused_imports)]
use semio_framework_plugin::ConfigView;
#[allow(unused_imports)]
use semio_framework_artifact_reference::Dialect;
#[allow(unused_imports)]
use semio_framework_plugin::Fault;
#[allow(unused_imports)]
use semio_framework_ui_locale::Label;
#[allow(unused_imports)]
use semio_framework_plugin::NoConfig;
#[allow(unused_imports)]
use semio_framework_plugin::NoConfigMutation;
#[allow(unused_imports)]
use semio_framework_plugin::NoPresence;
#[allow(unused_imports)]
use semio_framework_plugin::NoPresenceMutation;
#[allow(unused_imports)]
use semio_framework_plugin::NoTransient;
#[allow(unused_imports)]
use semio_framework_plugin::NoTransientMutation;
#[allow(unused_imports)]
use semio_framework_artifact_reference::StandardId;
#[allow(unused_imports)]
use semio_framework_artifact_reference::SubsetId;
#[allow(unused_imports)]
use semio_framework_plugin::ViewEmit;
#[allow(unused_imports)]
use semio_framework_2d::compute::EngineHandles;
use crate::viewer::*;

#[semio_framework_async_macros::async_test]
async fn create_viewer_builds_a_definition_for_the_viewer_role() {
    let def = create_semio_base_viewer();
    assert_eq!(def.role, semio_framework_plugin::AppRole::Viewer);
    assert_eq!(def.dialect, SEMIO_ANY_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn viewer_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<SemioAnyViewer as ArtifactViewer>::DIALECT, SEMIO_ANY_DIALECT);
}

#[semio_framework_async_macros::async_test]
async fn viewer_never_mutates_the_document_or_draft_store() {
    semio_framework_plugin::with_authoring_identity!(|identity| semio_framework_plugin::artifact_app_laws::assert_viewer_never_mutates::<SemioAnyViewer>(semio_s_artifact_stdio_contract::testing::mounted_policy(), &mut identity).await);
}
