use crate::standards::v1::subsets::any::schema::records::PUZZLE2D_DIALECT;
use semio_framework_plugin::ArtifactViewer;
#[allow(unused_imports)]
use crate::viewer::puzzle2d::modes::view;
#[allow(unused_imports)]
use crate::viewer::puzzle2d::modes::view::windows::board;
#[allow(unused_imports)]
use crate::{Puzzle2dSnapshot, standards::v1::subsets::any::schema::records::PUZZLE_2D_SCHEMA};
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
use semio_framework_plugin::Viewer;
#[allow(unused_imports)]
use semio_framework_plugin::app::InteractionView;
#[allow(unused_imports)]
use semio_framework_2d_compute::EngineHandles;
use crate::viewer::puzzle2d::*;

#[test]
fn create_puzzle2d_viewer_builds_a_definition_for_the_viewer_role() {
    let def = create_puzzle2d_viewer();
    assert_eq!(def.role, semio_framework::AppRole::Viewer);
    assert_eq!(def.dialect, PUZZLE2D_DIALECT.into());
}

#[test]
fn viewer_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<Puzzle2dViewer as ArtifactViewer>::DIALECT, PUZZLE2D_DIALECT);
}

#[test]
fn viewer_command_default_is_noop() {
    assert_eq!(Puzzle2dViewCommand::default(), Puzzle2dViewCommand::Noop);
}
