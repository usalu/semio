use crate::editor::puzzle2d::engine::board_host::testkit::*;
use semio_framework_os_infinite::board::BoardPaletteOverlay;
use crate::editor::puzzle2d::engine::canvas;
use crate::editor::puzzle2d::engine::canvas::geom_sel::cubic_bezier_point;
use crate::editor::puzzle2d::engine::canvas::Point;
use crate::editor::puzzle2d::engine::compute_edge_bezier_points;
use crate::editor::puzzle2d::engine::handle_position_on_circle;
use crate::editor::puzzle2d::engine::BoardElementStyleKind;
use crate::editor::puzzle2d::engine::BoardHost;
use crate::editor::puzzle2d::engine::Interaction;
use semio_framework_os_infinite::board::NodeDescriptor;
use crate::editor::puzzle2d::engine::WireDescriptor;
use semio_framework_pack_json::json;
use semio_framework_os_infinite::board_set_camera;
use semio_framework_test_fixtures::fixture_text;

include!("🧾️manifest-descriptor-sync-and-style-laws/🦀️.rs");
include!("🖱️drag-selection-and-lod-hit-laws/🦀️.rs");
include!("📐️generation-area-select-and-minimap-laws/🦀️.rs");
