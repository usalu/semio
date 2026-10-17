use crate::SVG_ANY_DIALECT;
use semio_framework_plugin::ArtifactViewer;
use semio_framework_plugin::Viewer;
#[allow(unused_imports)]
use crate::standards::v1_1::subsets::base::schema::mutations::SvgMutation;
#[allow(unused_imports)]
use crate::standards::v1_1::subsets::base::schema::snapshot::SvgSnapshot;
#[allow(unused_imports)]
use crate::viewer::svg_any::modes::view;
#[allow(unused_imports)]
use crate::viewer::svg_any::modes::view::windows::main;
#[allow(unused_imports)]
use crate::STDIO_SVG_DOCUMENT_SCHEMA;
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
use semio_framework_2d::compute::EngineHandles;
use crate::viewer::svg_any::*;

#[semio_framework_async_macros::async_test]
async fn create_viewer_builds_a_definition_for_the_viewer_role() {
    let def = create_svg_any_viewer();
    assert_eq!(def.role, semio_framework::AppRole::Viewer);
    assert_eq!(def.dialect, SVG_ANY_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn viewer_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<SvgAnyViewer as ArtifactViewer>::DIALECT, SVG_ANY_DIALECT);
}
