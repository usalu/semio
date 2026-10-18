//! 🧪️ normal-host tests (🔬️board-host): compiled against the `testkit` surface of the board crates.

use semio_framework_os_infinite_board_schema::NodeDescriptor;
use semio_framework_os_infinite_board_engine::{ActiveUtility, BoardElementStyleKind, GestureStage, HandleDescriptor, Interaction, RegionGrip, SceneDescriptor};
use semio_framework_os_infinite_board_normal_model::{BOARD_DESCRIPTOR_ITEM_CAPACITY, BOARD_EVENT_ITEM_CAPACITY, BOARD_POINTER_BYTE_CAPACITY, BOARD_POINTER_ITEM_CAPACITY, board_pointer_offset, BoardDrawLod, BoardEventKind, BoardFillStage, BoardNaming, BoardOwnedEvent, BoardPlacementNaming, StyleChromePass, WorldContentCache};
use semio_framework_os_infinite_board_normal_scene::{board_edge_curve, BoardPointerPlan, BoardPointerPlanFault, BoardPointerPlanKind, effective_handle_radius, handle_positions_json, region_snap_step};
use semio_framework_os_infinite_board_normal_focus::{BoardPointerIntent, BoardPointerPhase, commit_wheel, interaction_overlay_entity_ids, load_board_snapshot_json, resolve_node_style_kind, set_camera_silent, set_selection_ids, set_selection_ids_silent, wheel_screen};
use semio_framework_os_infinite_board_normal_gestures::{BoardAuthorityStep, BoardEntityRetirement, BoardPointerPlanRetirement, BoardRemovedEntity, event_authority_terminal_is_empty, pointer_lane_is_direct, quarantine_world_content_step, set_transform_flags, target_regions_json};
use semio_framework_os_infinite_board_normal_fill::{BoardFillJob, BoardFillSnapshotIngress, NEUTRAL_BOARD_NAMING};
use semio_framework_os_infinite_board_normal_host::{begin_pointer_commit, board_delete_selection, BoardHost, BoardHostRetirement};
use semio_framework_test_fixtures::fixture_text;
#[cfg(any(test, feature = "testkit"))]
use semio_framework_os_infinite_board_normal_host::plan_delete_selection;
use semio_framework_canvas::{Affine, Color, FillRule, Point, Scene, Vec2};
use semio_framework_value::NativeDecodeControl;
use semio_framework_graph_manifest::PropertyValue;
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress};
use semio_framework_pack_json::Value;
use semio_framework_graph_drawing::routing::handle_position_on_circle;
use semio_framework_value::retirement::sequence;

include!("🪝️pointer-plans-and-commit-turns/🦀️.rs");
include!("🗑️snapshot-json-delete-plans-and-retirement/🦀️.rs");
include!("🎞️fixture-reparse-and-transform-gumball/🦀️.rs");
include!("🩺️handle-vitals-and-target-regions/🦀️.rs");
include!("🎬️icon-clip-gestures-and-placement-naming/🦀️.rs");
