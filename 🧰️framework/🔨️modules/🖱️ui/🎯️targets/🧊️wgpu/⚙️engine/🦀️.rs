// #region engine
//! 🧵️ The retained-mode `Ui` façade: the missing keystone tying `arena`/`tree`/`reconcile`/`flex`/
//! `paint`/`events`/`scene_slots`/`shell` into one usable pipeline — each of those regions was built
//! and individually tested to its own milestone but nothing ever assembled them together, and nothing
//! in `framework/renderer/wgpu` calls into any of it (see `.🧬semio/🦑️repo/🎫️tickets/26/07/11/RETAINED-MODE-UI-CRATE`'s
//! plan for the historical intent). This module is purely additive: the immediate-mode `widgets`
//! path stays the only pipeline actually driving pixels until a later workstream proves this façade
//! out (via the golden `tests` module below) and cuts over.

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
#[cfg(any(test, feature = "testkit"))] use semio_framework_ui_wgpu_paint_reference::paint_tree;
use semio_framework_ui_wgpu_paint::{paint_node_step_with_driver, paint_tooltip, retained_overlay_chrome_step, sync_interactive_state_node_step, tooltip_surface_size, RetainedInteractiveSyncCursor, RetainedInteractiveSyncStep, RetainedNodePaintCursor, RetainedNodePaintStep};
use semio_framework_ui_retained_tree::reconcile::{UiComponentSceneWitness, UiDocumentReconcileCursor, UiDocumentReconcileFault, UiDocumentReconcileStep, UiRetiredComponentScene};
#[cfg(any(test, feature = "testkit"))] use semio_framework_ui_wgpu_hosting::scene_slots::collect_scene_slots;
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

#[path = "🔬️introspection-queries/🦀️.rs"]
mod introspection_queries;

//#region 🧪️RetainedDocumentHostileFixtures
//#endregion 🧪️RetainedDocumentHostileFixtures

// #endregion engine
