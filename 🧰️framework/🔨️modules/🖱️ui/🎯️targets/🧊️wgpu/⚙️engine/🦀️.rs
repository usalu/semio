// #region engine
//! 🧵️ The retained-mode `Ui` façade: the missing keystone tying `arena`/`tree`/`reconcile`/`flex`/
//! `paint`/`events`/`scene_slots` into the one pipeline that drives the renderer's pixels.

use std::collections::VecDeque;

use semio_ui_arena::NodeId;
use semio_ui_wgpu_layout::chrome::{UiDriverChrome, UiDriverDrag};
use semio_ui_declaration::ui::UiNode;
use semio_ui_draw_contract::{DrawList, IconAtlas};
use semio_ui_wgpu_events::{resolve_overlay_placement_side, AccessibilityUiEvent, DragPayload, EventRouter, OverlayAnchor, OverlayKind, TooltipStep, UiCommand, UiEvent};
use semio_ui_wgpu_layout::flex::{LayoutJobStage, LayoutJobStep};
use semio_ui_wgpu_mounted_layout::hit::RetainedHitRegistration;
use semio_ui_wgpu_layout::layout::TreeRowMetrics;
use semio_ui_wgpu_mounted_layout::{MountedLayoutIdentity, MountedLayoutJob};
#[cfg(any(test, feature = "testkit"))] use semio_ui_wgpu_mounted_layout::{MountedLayoutResult, RetainedGlyphPreview};
#[cfg(any(test, feature = "testkit"))] use semio_ui_wgpu_paint_reference::paint_tree;
use semio_ui_wgpu_paint::{paint_node_step_with_driver, retained_overlay_chrome_step, sync_interactive_state_node_step, RetainedInteractiveSyncCursor, RetainedInteractiveSyncStep, RetainedNodePaintCursor, RetainedNodePaintStep};
use semio_ui_retained_tree::reconcile::{UiComponentSceneWitness, UiDocumentReconcileStep, UiRetiredComponentScene};
#[cfg(any(test, feature = "testkit"))] use semio_ui_wgpu_hosting::scene_slots::collect_scene_slots;
use semio_ui_wgpu_hosting::scene_slots::{scene_slot_for_node, SceneHost, ScenePaintCursor, ScenePaintStep};
use semio_ui_text::FontAtlas;
use semio_ui_theme::Theme;
use semio_ui_retained_tree::tree::{NodeFlags, UiDocumentPageRejection, UiDocumentTree, UiDocumentTreeFault, UiTree};
use semio_framework_job::StepContext;
use ui_contract::{SurfaceId, UiDocumentLeaseHeader, UiDocumentNodePage, UiFixedList, UiNodeId};

pub use semio_ui_surface_identity::{UiSurfaceToken, UI_LAYOUT_SURFACE_SLOTS};
use semio_ui_wgpu_engine_surface::{LANE_WHEEL, PresentedTooltip, RetainedPaintFrame, RetainedPaintPhase, RetainedPaintWalk, RetainedPaintWalkStep, SurfaceLane, SurfaceLaneEntry, SurfaceLaneRing, SurfaceLayoutReason, ThemePropagationCursor, ThemePropagationPhase, UI_FRAME_OVERLAY_CHROME, UI_RETIRED_COMPONENT_SCENE_CAPACITY, UiCandidateBaseline, UiCandidateBaselineStep, UiDocumentIngress, UiDocumentIngressFault, UiDocumentIngressStatus, UiFramePaintCensus, UiFrameStep, UiLayoutStep, UiOverlayPlacement, UiPresentedCaret, UiPresentedSourceIdentity, UiPresentedTreeLoan, UiSceneHit, UiSealedVisibilityCandidate, UiSurfaceAdmissionRejected, UiSurfaceClosePhase, UiSurfaceCloseStep, UiSurfaceRegistry, UiWindow, UiWindowClockStep, UiWorkerRetirementRefusal, close_layout_worker_session, close_layout_admission, mark_window_paint_dirty, offset_rect, paint_retained_scrollbar, paint_retained_tooltip, presented_caret, register_retained_hit, retained_node_clip, retained_tooltip_paint, retire_surface_scene, scene_slot_for_node_absolute, stage_label, synchronize_presented_tooltip, theme_layout_identity, tooltip_label_in_tree};

#[path = "🏁️ui-state-and-presented-input/🦀️.rs"]
mod ui_state_and_presented_input;
pub use ui_state_and_presented_input::{RetainedInputDraft, Ui};

#[path = "📄️viewport-and-document-ingress/🦀️.rs"]
mod viewport_and_document_ingress;

#[path = "📐️layout-stepping/🦀️.rs"]
mod layout_stepping;

#[path = "🎞️frame-step/🦀️.rs"]
mod frame_step;

#[path = "🖥️frame-into-step-and-frame/🦀️.rs"]
mod frame_into_step_and_frame;

#[path = "📣️dispatch-and-overlay-api/🦀️.rs"]
mod dispatch_and_overlay_api;
pub use semio_ui_wgpu_light::drag_places::{drag_ghost_position, drag_starts, insert_slot, tree_drop_band, DropPlaceState, DropRect, TreeBand, DRAG_GHOST_OFFSET};
pub use semio_ui_wgpu_light::tree_drop::{tree_drop_position, TreeDrop, TREE_CATALOGUE_DRAG_MIME};

#[path = "🔬️introspection-queries/🦀️.rs"]
mod introspection_queries;

#[cfg(feature = "testkit")]
#[path = "🧪️fixture/🦀️.rs"]
pub mod testkit;

//#region 🧪️RetainedDocumentHostileFixtures
//#endregion 🧪️RetainedDocumentHostileFixtures

// #endregion engine
