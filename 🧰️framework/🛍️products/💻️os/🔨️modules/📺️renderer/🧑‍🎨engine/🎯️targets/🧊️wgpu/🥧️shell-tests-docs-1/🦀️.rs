//! 🥧️ Companion test bin of the wgpu renderer: documents, sync and directory (1).
//!
//! 🧭️ The tests of this bin are mounted from their place in the element trees; the namespace they were written against is rebuilt by the imports below.

#![cfg(test)]

extern crate infinite_canvas as infinite_world;
extern crate semio_framework_os_kernel as store_sync;
use semio_framework::kernel::{HistoryReprojection, HistoryTimeTravel, HistoryTimeTravelReview, HistoryTimeTravelStage, UiDirtyScope};
use semio_framework::{IconName, PanelGroup, PanelTabDefinition, ViewModel};
use semio_framework_os_renderer_wgpu_contracts::ui_document_close_pending_for;
use semio_framework_os_renderer_wgpu_dock::{DockNode, DockStackTab};
use semio_framework_os_renderer_wgpu_fixtures::shell::{host_test_shell, test_app, tree_pointer_record};
use semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions;
#[cfg(not(target_os = "wasi"))]
use semio_framework_os_renderer_wgpu_host_base::action_args_json;
use semio_framework_os_renderer_wgpu_program_bridge::operation_publications_from_json;
use semio_framework_os_renderer_wgpu_scene_canvas2d::ui_image_loading::framework_widget_context;
use semio_framework_os_renderer_wgpu_scene_state::{SceneChromeLabels, SceneEngineHosts};
use semio_framework_os_renderer_wgpu_shell_actions::{ActionsCtx, command_rejection_notice, staged_arg_value, uncommitted_edit_count};
use semio_framework_os_renderer_wgpu_shell_chrome_state::{ChromeDialogRequest, ChromeDialogStop, RefusedGuestFault, SHELL_SHORTCUT_ROWS, format_keybinding_shortcut, reference_selection_ids, shell_shortcut_table, time_travel_chord_yields_to};
use semio_framework_os_renderer_wgpu_shell_directory::time_travel_peer_presence;
use semio_framework_os_renderer_wgpu_shell_doc_open::TransferLane;
use semio_framework_os_renderer_wgpu_shell_history::TimeTravelFocus;
use semio_framework_os_renderer_wgpu_shell_input::{
    ChromeDialogPaintOp, ContextMenuItem, ContextMenuKeyOutcome, ContextMenuState, TIME_TRAVEL_BAND_STATUS_ID, TIME_TRAVEL_EDITOR_ACCEPT_KEY, TIME_TRAVEL_EDITOR_INPUTS_KEY, context_menu_move_active, context_menu_path_for_ordinal, projected_key_is,
    time_travel_editor_focus_node,
};
use semio_framework_os_renderer_wgpu_shell_pointer::{PointerCtx, is_reserved_shell_chord, shell_context_menu_item_from_spec, shell_shortcut_for, shell_shortcut_for_control_id, shell_shortcut_for_in};
use semio_framework_os_renderer_wgpu_shell_prefs::{
    ActiveSession, BAND_LABEL_MISSING, BandTongue, TIME_TRAVEL_PROGRESS_TRACK, TimeTravelVerb, band_label, band_label_in_catalog, classify_dispatch_fault_notice, dsl_value_as_json, history_refusal_is_silent, history_refusal_notice,
    history_refusal_of_fault, refusals, time_travel_band_controls, time_travel_band_lines, time_travel_band_plan, time_travel_review_key, time_travel_severity_text, time_travel_stage_key,
};
use semio_framework_os_renderer_wgpu_shell_session::{IMPORT_ABORT_ACTION_ID, SessionCtx, ShellShortcut, fold_history_patch, time_travel_transition};
use semio_framework_os_renderer_wgpu_shell_state::ShellState;
use semio_framework_os_renderer_wgpu_shell_surfaces::{OpenedFile, PickedImport};
use semio_framework_os_renderer_wgpu_shell_view_chrome::ShellChromeChildCursor;
use semio_framework_os_renderer_wgpu_shell_view_overlays::{
    BAND_FIXED_LABEL_KEYS, CHROME_BAND_FRAME_STEPS, HISTORY_REPROJECTION_PROGRESS_ID, HISTORY_REPROJECTION_STATUS_ID, OverlaysViewCtx, TIME_TRAVEL_BAND_PROGRESS_ID, chrome_dialog_paint_ops, chrome_status_accessibility_node,
    render_context_menu_level, time_travel_band_tone, time_travel_indicator_caption, time_travel_indicator_control_id, time_travel_indicator_text,
};
use semio_framework_os_renderer_wgpu_shell_view_paint::SHELL_WINDOW_PAINT_OPPORTUNITIES;
use semio_framework_os_renderer_wgpu_shell_view_panels::{PanelsViewCtx, TRANSIENT_NOTICE_STATUS_ID};
use semio_framework_os_renderer_wgpu_shell_window_actions::{WindowActionsCtx, history_rows_oldest_first, note_host_window_blur, operation_publication_refresh, staged_action_arg_row};
use semio_framework_os_renderer_wgpu_ui_engine::retained_document_consumer::{UiDocumentFrameCursor, render_ui_document_step};
use semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine;
use semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::RetainedNodeFocusKind;
use semio_framework_os_renderer_wgpu_ui_engine_dump::chrome_ledger;
use semio_framework_os_renderer_wgpu_ui_engine_tests::fixture_original_turn;
use semio_framework_time_travel::TimeTravelLabel;
use semio_framework_ui_locale::{Locale, LocalizedLabel, Terminology};
use semio_framework_value::DslValue;
use serde_json::Value;
use std::cell::RefCell;
use std::collections::{BTreeMap, HashMap};
use store_sync::PresencePeer;
use ui_contract::{SurfaceId, UiDocumentLease};
use semio_framework_ui_declaration::layout::{ActionDescriptor, FRAMEWORK_PANEL_TAB_HISTORY_ID};
use semio_framework_ui_draw_contract::{DrawList, IconAtlas};
use semio_framework_ui_text::FontAtlas;
use semio_framework_ui_wgpu_input::{HitKind, HitTarget, InputState, KeyAction, PointerModifiers};
use semio_framework_ui_theme::geometry::Rect;
use semio_framework_ui_theme::Theme;
use semio_framework_ui_declaration::ui::{UiControlNode, UiInputNode};

#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "../../../🧱️elements/🐚️Shell/🧪️tests/🧪️wgpu-time-travel/🦀️.rs"]
mod time_travel_tests;
