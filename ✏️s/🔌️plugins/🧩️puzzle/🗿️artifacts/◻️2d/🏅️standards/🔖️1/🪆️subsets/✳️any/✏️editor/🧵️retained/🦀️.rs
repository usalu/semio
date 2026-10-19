//! 🧵️ Retained work of the puzzle 2d editor: the board-event batch limits, window and export work, the dispatch emit, the active-example reduce and the force-layout and redraw-handles work. See [`super`].

use crate::{editor::puzzle2d::snapshot::PUZZLE2D_ADD_NODE_DIALOG_ID, editor::puzzle2d::snapshot::PUZZLE2D_BOARD_SNAPSHOT_SCHEMA, editor::puzzle2d::snapshot::PUZZLE2D_INTERACTION_DOMAIN, editor::puzzle2d::snapshot::PUZZLE2D_PROXIMITY_GESTURE_MAX, editor::puzzle2d::snapshot::Puzzle2dInteractionSnapshot, editor::puzzle2d::commands::apply_board_events::apply_host_events, editor::puzzle2d::engine::board_host::drain_board_events_json, editor::puzzle2d::snapshot::puzzle2d_active_utility, editor::puzzle2d::snapshot::puzzle2d_load_document_effect, editor::puzzle2d::snapshot::sync_host_fixture_content, editor::puzzle2d::snapshot::sync_host_runtime_state};
use crate::{editor::puzzle2d::commands::accept_suggestion, editor::puzzle2d::commands::add_node, editor::puzzle2d::commands::add_target_region, editor::puzzle2d::commands::apply_board_events, editor::puzzle2d::commands::close_handle_suggestions, editor::puzzle2d::commands::create_edge, editor::puzzle2d::commands::cycle_candidate, editor::puzzle2d::commands::delete_edge, editor::puzzle2d::commands::delete_selection, editor::puzzle2d::commands::delete_target_region, editor::puzzle2d::commands::duplicate_selection, editor::puzzle2d::commands::engagement_abort, editor::puzzle2d::commands::engagement_control_select, editor::puzzle2d::commands::engagement_input, editor::puzzle2d::commands::engagement_repeat_last, editor::puzzle2d::commands::engagement_submit, editor::puzzle2d::commands::export_snapshot, editor::puzzle2d::commands::focus_selection, editor::puzzle2d::commands::force_layout, editor::puzzle2d::commands::hover_suggestion, editor::puzzle2d::commands::target_brush_suggestions, editor::puzzle2d::commands::import_snapshot, editor::puzzle2d::commands::lod_scale_json, editor::puzzle2d::commands::open_handle_suggestions, editor::puzzle2d::commands::open_import_snapshot, editor::puzzle2d::commands::patch_inspector, editor::puzzle2d::commands::proximity_connect, editor::puzzle2d::commands::relocate_target_region, editor::puzzle2d::commands::rotate_selection, editor::puzzle2d::commands::scale_selection, editor::puzzle2d::commands::select_same_kind, editor::puzzle2d::commands::set_active_example, editor::puzzle2d::commands::set_area_brush_size, editor::puzzle2d::commands::set_brush_kind_weights, editor::puzzle2d::commands::set_brush_node_size, editor::puzzle2d::commands::set_brush_placement_contact_tolerance, editor::puzzle2d::commands::set_brush_placement_overlap_budget, editor::puzzle2d::commands::set_camera, editor::puzzle2d::commands::set_fill_count, editor::puzzle2d::commands::set_grid_factor, editor::puzzle2d::commands::set_grid_snap_enabled, editor::puzzle2d::commands::set_grid_visible, editor::puzzle2d::commands::set_lod_mode_for_pane, editor::puzzle2d::commands::set_proximity_radius, editor::puzzle2d::commands::set_selectable_kind, editor::puzzle2d::commands::set_selection_flag, editor::puzzle2d::commands::set_suggestion_offset, editor::puzzle2d::commands::set_target_region_flag, editor::puzzle2d::commands::set_transform_gumball_flag, editor::puzzle2d::commands::translate_selection};
use crate::{editor::puzzle2d::config::Puzzle2dConfig, editor::puzzle2d::config::Puzzle2dConfigMutation};
use crate::editor::puzzle2d::engine::Point;
use crate::{editor::puzzle2d::window, editor::puzzle2d::window::Puzzle2dWindowConfig, editor::puzzle2d::window::Puzzle2dWindowTransient};
use crate::editor::puzzle2d::snapshot::Puzzle2dPlaySnapshot;
use crate::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation;
use crate::editor::puzzle2d::recorder::Puzzle2dRecorder;
use semio_framework::kernel::UiDirtyScope;
use semio_framework_plugin::kernel::Effect;
use semio_framework_plugin::EditorApp;
use semio_framework_plugin::Emit;
use semio_framework_plugin::Fault;
use semio_framework_plugin::app::EphemeralEmit;
use semio_framework_pack_json::Value;
#[cfg(test)] use std::cell::RefCell;
use std::{collections::HashMap, collections::HashSet};
use crate::editor::puzzle2d::snapshot::puzzle2d_scene_with;
use crate::editor::puzzle2d::snapshot::Puzzle2dEditor;
use crate::editor::puzzle2d::{interaction, recorder, snapshot};
use crate::editor::puzzle2d::snapshot::{PUZZLE2D_GENERIC_TOOL_IDS};
use semio_s_artifact_puzzle_2d_unit_tests::{editor::puzzle2d::snapshot::Puzzle2dCommand, editor::puzzle2d::engine::board_host::puzzle_board_host};
use semio_framework_value::scratch_hash::{ScratchHashMap,ScratchHashSet};


#[path = "🧱️board-event-limits-window-export-and-dispatch-work/🦀️.rs"]
mod board_event_limits_window_export_and_dispatch_work;
pub use board_event_limits_window_export_and_dispatch_work::{
    puzzle2d_active_example_emit, puzzle2d_active_example_extent, puzzle2d_active_example_reduce,
    puzzle2d_add_node_extent, puzzle2d_board_events_extent, puzzle2d_dispatch_emit,
    puzzle2d_generic_extent, puzzle2d_revision_hex, Puzzle2dExportWork, Puzzle2dWindowCommandWork,
};

#[path = "🕸️force-layout-work/🦀️.rs"]
mod force_layout_work;
use force_layout_work::{
    puzzle2d_force_iterations, puzzle2d_force_pairs, Puzzle2dForceLayoutWorkOwners,
    Puzzle2dForceStage, PUZZLE2D_FORCE_ITERATION_STAGE_STEPS, PUZZLE2D_FORCE_MAX_EDGES,
    PUZZLE2D_FORCE_MAX_HANDLES, PUZZLE2D_FORCE_MAX_NODES, PUZZLE2D_FORCE_NODES_PER_STEP,
    PUZZLE2D_FORCE_PROLOGUE_STAGE_STEPS, PUZZLE2D_FORCE_SCAN_PER_STEP,
    PUZZLE2D_FORCE_UNITS_PER_STEP,
};
pub use force_layout_work::Puzzle2dForceLayoutWork;

#[path = "🎨️force-layout-command-and-redraw-handles-work/🦀️.rs"]
mod force_layout_command_and_redraw_handles_work;
pub use force_layout_command_and_redraw_handles_work::Puzzle2dRedrawHandlesWork;








