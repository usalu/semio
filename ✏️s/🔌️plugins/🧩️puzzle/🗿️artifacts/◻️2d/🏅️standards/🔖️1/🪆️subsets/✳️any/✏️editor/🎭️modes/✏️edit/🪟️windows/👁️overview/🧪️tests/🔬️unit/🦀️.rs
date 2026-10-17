#[allow(unused_imports)]
use crate::editor::puzzle2d::engine::BoardHost;
#[allow(unused_imports)]
use crate::editor::puzzle2d::modes::edit;
#[allow(unused_imports)]
use crate::editor::puzzle2d::modes::edit::options;
#[allow(unused_imports)]
use crate::editor::puzzle2d::modes::edit::windows::overview::utilities;
#[allow(unused_imports)]
use crate::editor::puzzle2d::terminology::Puzzle2dLabels;
#[allow(unused_imports)]
use crate::editor::puzzle2d::{terminology::puzzle2d_localized, snapshot::Puzzle2dScene, config::PUZZLE2D_LOD_MODE_AUTOMATIC};
#[allow(unused_imports)]
use semio_framework_plugin::{SurfaceKind, WindowEngagement, WindowEngagementSlot, WindowKindDefinition, WindowMeasure, WindowOptions};
use crate::editor::puzzle2d::modes::edit::windows::overview::*;
use crate::editor::puzzle2d::testkit::*;

#[test]
fn renders_puzzle2d_board_scene() {
    let mut app = app();
    let body = render_body(&mut app, BODY_KEY);
    close_app(&mut app);
    assert!(body.contains("board-2d"));
}
