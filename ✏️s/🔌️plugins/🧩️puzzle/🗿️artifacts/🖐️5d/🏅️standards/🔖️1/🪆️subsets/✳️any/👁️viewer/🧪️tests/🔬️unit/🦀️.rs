use crate::PUZZLE5D_DIALECT;
use semio_framework_plugin::ArtifactViewer;
use semio_framework_plugin::Viewer;
#[allow(unused_imports)]
use crate::{Puzzle5dMutation, Puzzle5dSnapshot, PUZZLE_5D_SCHEMA};
#[allow(unused_imports)]
use crate::viewer::puzzle5d::modes::view;
#[allow(unused_imports)]
use crate::viewer::puzzle5d::modes::view::windows::{board2d, world3d};
#[allow(unused_imports)]
use semio_framework_plugin::app::Dialect;
#[allow(unused_imports)]
use semio_framework_plugin::ArtifactView;
#[allow(unused_imports)]
use semio_framework_plugin::ConfigView;
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

use crate::viewer::puzzle5d::*;

#[test]
fn create_puzzle5d_viewer_builds_a_definition_for_the_viewer_role() {
    let def = create_puzzle5d_viewer();
    assert_eq!(def.role, semio_framework::AppRole::Viewer);
    assert_eq!(def.dialect, PUZZLE5D_DIALECT.into());
}

#[test]
fn viewer_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<Puzzle5dViewer as ArtifactViewer>::DIALECT, PUZZLE5D_DIALECT);
}
