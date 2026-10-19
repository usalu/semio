//! 🥣️ semio wgpu engine laws: scene lifetime, frame storm, overlays, goldens, document ingress and presented source.

#![cfg(test)]

use std::collections::{HashMap, VecDeque};
use semio_framework_ui_arena::NodeId;
use semio_framework_ui_wgpu_layout::chrome::UiDriverDrag;
use semio_framework_ui_declaration::layout::WindowLayout;
use semio_framework_ui_declaration::ui::UiNode;
use semio_framework_ui_draw_contract::{DrawList, IconAtlas};
use semio_framework_ui_wgpu_events::{resolve_overlay_placement_side, AccessibilityUiEvent, CaretSource, DragPayload, EventRouter, OverlayAnchor, OverlayKind, TooltipStep, UiCommand, UiEvent};
use semio_framework_ui_wgpu_layout::flex::{LayoutJobStage, LayoutJobStep};
use semio_framework_ui_wgpu_mounted_layout::hit::{retained_hit_registration, retained_tree_chevron_registration, retained_tree_drag_handle_registration, RetainedHitRegistration};
use semio_framework_ui_wgpu_input::HitKind;
use semio_framework_ui_wgpu_layout::layout::TreeRowMetrics;
use semio_framework_ui_wgpu_mounted_layout::{MountedLayoutIdentity, MountedLayoutJob, MountedLayoutResult, RetainedGlyphPreview};
#[cfg(test)] use semio_framework_ui_wgpu_paint_reference::paint_tree;
use semio_framework_ui_wgpu_paint::{paint_node_step_with_driver, paint_tooltip, retained_overlay_chrome_step, sync_interactive_state_node_step, tooltip_surface_size, RetainedInteractiveSyncCursor, RetainedInteractiveSyncStep, RetainedNodePaintCursor, RetainedNodePaintStep};
use semio_framework_ui_retained_tree::reconcile::{UiComponentSceneWitness, UiDocumentReconcileCursor, UiDocumentReconcileFault, UiDocumentReconcileStep, UiRetiredComponentScene};
#[cfg(test)] use semio_framework_ui_wgpu_hosting::scene_slots::collect_scene_slots;
use semio_framework_ui_wgpu_hosting::scene_slots::{scene_slot_for_node, SceneHost, ScenePaintCursor, ScenePaintStep};
use semio_framework_ui_wgpu_hosting::shell::{Shell, ShellEvent};
use semio_framework_ui_text::FontAtlas;
use semio_framework_ui_theme::Theme;
use semio_framework_ui_retained_tree::tree::{NodeFlags, UiDocumentPageRejection, UiDocumentTree, UiDocumentTreeFault, UiTree};
use semio_framework_ui_declaration::IconName;
use semio_framework_job::StepContext;
use ui_contract::{SurfaceId, UiDocumentLeaseHeader, UiDocumentNodePage, UiFixedList, UiNodeId, UI_DOCUMENT_NODES};
pub use semio_framework_ui_surface_identity::{UiSurfaceToken, UI_LAYOUT_SURFACE_SLOTS};
use semio_framework_ui_wgpu_engine_surface::{LANE_WHEEL, PresentedTooltip, RetainedPaintFrame, RetainedPaintPhase, RetainedPaintWalk, RetainedPaintWalkStep, SurfaceLane, SurfaceLaneEntry, SurfaceLaneRing, SurfaceLayoutReason, ThemePropagationCursor, ThemePropagationPhase, UI_FRAME_OVERLAY_CHROME, UI_RETIRED_COMPONENT_SCENE_CAPACITY, UiCandidateBaseline, UiCandidateBaselineStep, UiDocumentIngress, UiDocumentIngressFault, UiDocumentIngressStatus, UiFramePaintCensus, UiFrameStep, UiLayoutStep, UiOverlayPlacement, UiPresentedCaret, UiPresentedSourceIdentity, UiPresentedTreeLoan, UiSceneHit, UiSealedVisibilityCandidate, UiSurfaceAdmissionRejected, UiSurfaceClosePhase, UiSurfaceCloseStep, UiSurfaceRegistry, UiWindow, UiWindowClockStep, UiWorkerRetirementRefusal, close_layout_worker_session, consume_layout_retirement, mark_window_paint_dirty, offset_rect, paint_retained_tooltip, presented_caret, register_retained_hit, retained_node_clip, retained_tooltip_paint, retire_surface_scene, scene_slot_for_node_absolute, stage_label, synchronize_presented_tooltip, theme_layout_identity, tooltip_label_in_tree, worker_lane};
use semio_framework_ui_wgpu_engine::*;
use semio_framework_ui_wgpu_engine_surface::*;
use semio_framework_ui_wgpu_engine_surface::LayoutWorkerAdmission;
use semio_framework_ui_wgpu_engine_surface::RETAINED_PAINT_DEPTH_CREDITS;
use semio_framework_ui_wgpu_engine_surface::UiSurfaceSlot;

#[path = "../../../../../🧪️tests/🔬️targets-wgpu-engine-unit/🦀️.rs"]
mod tests;

#[cfg(not(target_arch = "wasm32"))]
#[path = "../../../../../🧪️tests/♻️physical-job-close/⏱️allocator/🦀️.rs"]
mod physical_allocator;
