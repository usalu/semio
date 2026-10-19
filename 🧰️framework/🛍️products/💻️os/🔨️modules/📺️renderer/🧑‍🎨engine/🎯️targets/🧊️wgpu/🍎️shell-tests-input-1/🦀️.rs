//! 🍎️ Companion test bin of the wgpu renderer: input and pointer (1).
//!
//! 🧭️ The tests of this bin are mounted from their place in the element trees; the namespace they were written against is rebuilt by the imports below.

#![cfg(test)]

extern crate infinite_canvas as infinite_world;
use semio_framework::ViewModel;
use semio_framework::kernel::UiDirtyScope;
use semio_framework_job::InteractiveJobCloseStep;
use semio_framework_os_renderer_wgpu_canvas2d_host::begin;
use semio_framework_os_renderer_wgpu_contracts::{AdmittedSurfaceMap, SCENE_SURFACE_CAPACITY, ui_document_close_pending, ui_document_close_pending_for};
use semio_framework_os_renderer_wgpu_dock::{DockDragKind, DockDragPayload, DockDragState, DockDropZone, DockStackTab, DockState, WindowSilhouette};
use semio_framework_os_renderer_wgpu_fixtures::shell::{
    boot_manifest, host_test_shell, mouse_pointer, paint_component_pointer_documents, paint_tree_pointer_documents, pointer_interaction, retained_world_sequence_probe, tree_pointer_record, world3d_pointer_record,
};
use semio_framework_os_renderer_wgpu_frame_actions::{collect_fixture_actions, dispatch_normalized_event};
#[cfg(not(target_os = "wasi"))]
use semio_framework_os_renderer_wgpu_host_base::action_args_json;
use semio_framework_os_renderer_wgpu_host_base::app_now_ms;
use semio_framework_os_renderer_wgpu_host_plumbing::deadlines;
use semio_framework_os_renderer_wgpu_host_winit::os_host;
use semio_framework_os_renderer_wgpu_program_bridge::retained_publication_refusal;
use semio_framework_os_renderer_wgpu_scene_canvas2d::ui_image_loading::framework_widget_context;
use semio_framework_os_renderer_wgpu_scene_render::{cancel_canvas_pointer_gesture, cancel_scene_list_transfer};
use semio_framework_os_renderer_wgpu_scene_state::{SceneCameraDispatchCursor, SceneCameraDispatchStep, SceneChromeLabels, SceneEngineHosts, camera_fixture_grant, camera_fixture_lease, retire_scene_identity, sweep_expired_scene_camera_dispatches};
use semio_framework_os_renderer_wgpu_shell_actions::{ActionsCtx, select_boot_program};
use semio_framework_os_renderer_wgpu_shell_chrome_state::{RetainedContentFocus, ShellChromeBuildState};
use semio_framework_os_renderer_wgpu_shell_dock::{PanelAnchorState, ShellDock, dock_window_order};
use semio_framework_os_renderer_wgpu_shell_docs::note_shell_command_action;
use semio_framework_os_renderer_wgpu_shell_input::{OverlayState, PointerHitOwner, decode_window_template_drag, with_chrome_control_names};
use semio_framework_os_renderer_wgpu_shell_pointer::{PointerCtx, content_is_editing, ui_event_from_key_action};
use semio_framework_os_renderer_wgpu_shell_prefs::{DockTabNode, dsl_value_as_json, labels, refusals};
use semio_framework_os_renderer_wgpu_shell_session::{SessionCtx, WINDOW_TOPOLOGY_PUBLICATION_ATTEMPTS, panel_ui_records};
use semio_framework_os_renderer_wgpu_shell_state::ShellState;
use semio_framework_os_renderer_wgpu_shell_surfaces::ShellSettleStep;
use semio_framework_os_renderer_wgpu_shell_view_chrome::{ChromeViewCtx, ShellChromeChildCursor, note_dock_control_name};
use semio_framework_os_renderer_wgpu_shell_view_overlays::intersect_content_rect;
use semio_framework_os_renderer_wgpu_shell_view_paint::{SHELL_WINDOW_PAINT_OPPORTUNITIES, scene_chrome_labels};
use semio_framework_os_renderer_wgpu_shell_view_panels::PanelsViewCtx;
use semio_framework_os_renderer_wgpu_shell_window_actions::BootSelectionRequest;
use semio_framework_os_renderer_wgpu_shell_windows::{PanelAnchor, section_fixtures};
use semio_framework_os_renderer_wgpu_ui_engine::retained_document_consumer::{UiDocumentFrameCursor, render_ui_document_step};
use semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine;
use semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::RetainedNodeFocusKind;
use semio_framework_os_renderer_wgpu_ui_engine_dump::chrome_ledger;
use semio_framework_os_renderer_wgpu_ui_engine_tests::fixture_original_turn;
use semio_framework_ui_locale::{Label, Locale, Terminology};
use semio_framework_value::DslValue;
use serde_json::Value;
use std::cell::RefCell;
use std::collections::HashMap;
use ui_contract::{SurfaceId, UiDocumentLease};
use semio_framework_ui_declaration::layout::{ActionDescriptor, WindowStackCorner};
use semio_framework_ui_draw_contract::{DrawList, IconAtlas};
use semio_framework_ui_text::FontAtlas;
use semio_framework_ui_wgpu_input::{HitKind, HitTarget, InputState, KeyAction, PointerModifiers};
use semio_framework_ui_theme::geometry::Rect;
use semio_framework_ui_theme::Theme;
use semio_framework_ui_declaration::ui::{UiNode, UiPresence, UiTreeItemNode, UiTreeNode, UiTreeSectionNode};
use winit::window::Window;

#[path = "../../../🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-shell-input/🦀️.rs"]
mod shell_input_tests;
