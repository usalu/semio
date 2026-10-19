//! 🪟️ Engine surfaces: windows, tooltips, retained paint walk, surface registry, document ingress and layout worker admission behind the `Ui` facade.

use std::collections::VecDeque;
use semio_framework_ui_arena::NodeId;
use semio_framework_ui_wgpu_layout::chrome::UiDriverDrag;
use semio_framework_ui_declaration::ui::UiNode;
use semio_framework_ui_draw_contract::DrawList;
use semio_framework_ui_wgpu_events::{resolve_overlay_placement_side, CaretSource, EventRouter, OverlayAnchor, OverlayKind, TooltipStep};
use semio_framework_ui_wgpu_layout::flex::LayoutJobStage;
use semio_framework_ui_wgpu_mounted_layout::hit::{retained_hit_registration, retained_tree_chevron_registration, retained_tree_drag_handle_registration, RetainedHitRegistration};
use semio_framework_ui_wgpu_input::HitKind;
use semio_framework_ui_wgpu_layout::layout::TreeRowMetrics;
use semio_framework_ui_wgpu_mounted_layout::{MountedLayoutJob, MountedLayoutResult, RetainedGlyphPreview};
use semio_framework_ui_wgpu_paint::{paint_tooltip, tooltip_surface_size, RetainedInteractiveSyncCursor, RetainedNodePaintCursor};
use semio_framework_ui_retained_tree::reconcile::{UiDocumentReconcileCursor, UiDocumentReconcileFault, UiRetiredComponentScene};
use semio_framework_ui_wgpu_hosting::scene_slots::ScenePaintCursor;
use semio_framework_ui_text::FontAtlas;
use semio_framework_ui_theme::Theme;
use semio_framework_ui_retained_tree::tree::{NodeFlags, UiDocumentTree, UiDocumentTreeFault, UiTree};
use semio_framework_job::StepContext;
use ui_contract::{SurfaceId, UiFixedList, UiNodeId, UI_DOCUMENT_NODES};
use semio_framework_job_worker_authority::WorkerJobCloseStep;
use semio_framework_ui_surface_identity::{UiSurfaceToken, UI_LAYOUT_SURFACE_SLOTS};

#[path = "📍️window-and-tooltips/🦀️.rs"]
mod window_and_tooltips;
pub use window_and_tooltips::{
    mark_window_paint_dirty, paint_retained_tooltip, presented_caret, retained_tooltip_paint,
    synchronize_presented_tooltip, tooltip_label_in_tree, PresentedTooltip, RetainedTooltipPaint,
    UiCaretSource, UiOverlayPlacement, UiPresentedCaret, UiWindow, UiWindowClockStep,
};

#[path = "🧯️candidate-baseline-close-and-layout-worker/🦀️.rs"]
mod candidate_baseline_close_and_layout_worker;
pub use candidate_baseline_close_and_layout_worker::{
    close_layout_worker_session, consume_layout_retirement, retire_surface_scene,
    LayoutWorkerAdmission, UiCandidateBaseline, UiCandidateBaselineStep, UiSurfaceClosePhase,
    UiSurfaceCloseStep, UiWorkerRetirementRefusal,
};

#[path = "🚶️retained-paint-walk-and-hits/🦀️.rs"]
mod retained_paint_walk_and_hits;
pub use retained_paint_walk_and_hits::{
    offset_rect, register_retained_hit, retained_node_clip, scene_slot_for_node_absolute,
    RetainedPaintFrame, RetainedPaintPhase, RetainedPaintVisit, RetainedPaintWalk,
    RetainedPaintWalkStep, UiFramePaintCensus, UiSceneHit, RETAINED_PAINT_DEPTH_CREDITS,
    UI_FRAME_OVERLAY_CHROME, UI_RETIRED_COMPONENT_SCENE_CAPACITY,
};

#[path = "🗂️surface-registry-ingress-and-lanes/🦀️.rs"]
mod surface_registry_ingress_and_lanes;
pub use surface_registry_ingress_and_lanes::{
    stage_label, theme_layout_identity, ui_document_ingress_generation, worker_lane, SurfaceLane,
    SurfaceLaneEntry, SurfaceLaneRing, SurfaceLayoutReason, ThemePropagationCursor,
    ThemePropagationPhase, UiDocumentIngress, UiDocumentIngressFault, UiDocumentIngressStatus,
    UiFrameStep, UiLayoutStep, UiPresentedSourceIdentity, UiPresentedTreeLoan,
    UiSealedVisibilityCandidate, UiSurfaceAdmissionRejected, UiSurfaceRegistry, UiSurfaceSlot,
    LANE_WHEEL,
};
