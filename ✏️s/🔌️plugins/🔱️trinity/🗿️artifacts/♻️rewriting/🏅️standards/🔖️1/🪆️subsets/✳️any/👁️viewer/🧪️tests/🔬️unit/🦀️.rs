use crate::standards::v1::subsets::any::schema::model::TRINITY_REWRITING_DIALECT;
use semio_framework_plugin::ArtifactViewer;
use semio_framework_plugin::Viewer;
#[allow(unused_imports)]
use crate::standards::v1::subsets::any::schema::mutations::RewriteRuleMutation;
#[allow(unused_imports)]
use crate::viewer::rewriting::modes::view;
#[allow(unused_imports)]
use crate::viewer::rewriting::modes::view::windows::rule;
#[allow(unused_imports)]
use crate::standards::v1::subsets::any::schema::snapshot::RewritingSnapshot;
#[allow(unused_imports)]
use crate::standards::v1::subsets::any::schema::model::REWRITE_RULE_SCHEMA;
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
use semio_framework_plugin::ViewEmit;
#[allow(unused_imports)]
use semio_framework_2d_compute::EngineHandles;
use crate::viewer::rewriting::*;

#[semio_framework_async_macros::async_test]
async fn create_trinity_rewriting_viewer_builds_a_definition_for_the_viewer_role() {
    let def = create_trinity_rewriting_viewer();
    assert_eq!(def.role, semio_framework::AppRole::Viewer);
    assert_eq!(def.dialect, TRINITY_REWRITING_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn viewer_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<TrinityRewritingViewer as ArtifactViewer>::DIALECT, TRINITY_REWRITING_DIALECT);
}
