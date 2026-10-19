//! 📍️ Window and tooltips: the overlay placement, the caret and clock records, the presented tooltip, the `UiWindow` and the tooltip and row reason synchronisation. See [`super`].

use super::*;

//#region 🪟️OverlayPlacement
/// 🪟️ One open overlay, already placed against its window's viewport — see
/// [`Ui::overlay_placements`]. `width`/`height` are the overlay subtree's own mounted layout, so a
/// caller can draw the surface chrome (`paint::paint_overlay_surface`) without re-measuring.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UiOverlayPlacement {
    pub root: NodeId,
    pub kind: OverlayKind,
    /// 🌫️ `Dialog`/`CommandPalette`: draw `paint::paint_overlay_backdrop` first.
    pub backdrop: bool,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub side: semio_framework_ui_wgpu_events::OverlaySide,
}
//#endregion 🪟️OverlayPlacement

/// ⏱️ One retained window's bounded monotonic-clock result.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UiCaretSource {
    Retained,
    Scene,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UiPresentedCaret {
    pub surface: UiSurfaceToken,
    pub document_id: UiNodeId,
    pub source: UiCaretSource,
    pub visible: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UiWindowClockStep {
    pub surface: UiSurfaceToken,
    pub tooltip: TooltipStep,
    pub caret: Option<UiPresentedCaret>,
    pub interaction_changed: bool,
    pub next_deadline: Option<f64>,
}

pub struct PresentedTooltip {
    pub surface: UiSurfaceToken,
    pub document_id: UiNodeId,
    pub label: String,
    pub accessibility_generation: u64,
    pub interaction_epoch: u64,
    /// 💬️ The row action whose disabled reason this hint shows, anchored to its icon (audit W1E-1); `None` for a hover tooltip.
    pub row_action: Option<usize>,
}

pub struct RetainedTooltipPaint {
    pub label: String,
    pub x: f32,
    pub y: f32,
}

//#region 🔖️UiWindow
/// 🪟️ One window's retained pipeline state: its `UiTree` (`reconcile`'s diff target), the taffy
/// `LayoutEngine` that lays it out (`flex`), the `EventRouter` owning its capture/focus/hover state
/// (`events`), and the `DrawList` `paint::paint_tree` last painted into. Mirrors `tree`'s own doc
/// comment ("the engine facade... holds `HashMap<window_id, UiTree>`") by keying the *whole*
/// per-window pipeline the same way, not just the tree.
pub struct UiWindow {
    pub closing: Option<UiSurfaceClosePhase>,
    pub tree: UiTree,
    pub router: EventRouter,
    pub presented_tree: UiTree,
    pub presented_router: EventRouter,
    pub presented_ready: bool,
    pub presented_source:Option<UiPresentedSourceIdentity>,
    pub presented_revision: u64,
    pub presented_accessibility_generation: u64,
    pub presented_interaction_epoch: u64,
    pub candidate_base_interaction_epoch: u64,
    pub candidate_ready: bool,
    pub candidate_reconciled: bool,
    pub candidate_retirements_resolved: bool,
    pub candidate_preserves_prior_mounts: bool,
    pub candidate_interaction_cursor: Option<usize>,
    pub sealed_input_candidate: Option<(u64, u64)>,
    pub presented_tooltip: Option<PresentedTooltip>,
    pub candidate_baseline: Option<Box<UiCandidateBaseline>>,
    pub draw: DrawList,
    pub viewport: (f32, f32),
    pub layout_job: Option<MountedLayoutJob>,
    pub layout_session: Option<semio_framework_job_worker::MountedWorkerJobSession<MountedLayoutJob>>,
    pub layout_admission: LayoutWorkerAdmission,
    pub layout_session_generation: Option<u64>,
    pub layout_closing: bool,
    pub layout_retirement_refusal: Option<UiWorkerRetirementRefusal>,
    pub layout_close_failure:Option<semio_framework_value::retirement::controlled::ControlledRetirement<semio_framework_job::OriginalTurnFailure>>,
    pub layout_preview: Option<MountedLayoutResult>,
    pub glyph_preview: Option<RetainedGlyphPreview>,
    pub lane: SurfaceLane,
    pub queued: bool,
    pub intrinsic_content_height: f32,
    pub presented_intrinsic_content_height: f32,
    pub revision: u64,
    pub theme_revision: u64,
    pub viewport_revision: u64,
    pub layout_generation: u64,
    pub accessibility_generation: u64,
    pub document_ingress: Option<UiDocumentIngress>,
    pub retiring_document: Option<UiDocumentTree>,
    pub retiring_presented_document: Option<UiDocumentTree>,
    /// 🌳️ The published document's own reconcile into the paintable arena — the one thing that ever
    /// sets `tree.root` in production (see `🔁️reconcile.rs`'s `🌳️DocumentTreeReconcile`).
    pub document_reconcile: UiDocumentReconcileCursor,
    pub scene_retirements: VecDeque<UiRetiredComponentScene>,
    pub candidate_scene_retirements: VecDeque<UiRetiredComponentScene>,
    pub accepted_scene_retirements: VecDeque<UiRetiredComponentScene>,
    pub component_mount_generation: u64,
    pub paint_frame: Option<RetainedPaintFrame>,
    pub retiring_draw: Option<DrawList>,
    pub paint_census: UiFramePaintCensus,
    /// 🎯️ Every interactive node of the last published paint, at the rect that paint drew it at.
    /// The host pushes these into its own `InputState` each frame build — see
    /// `input::RetainedHitRegistration`.
    pub hit_registry: Vec<RetainedHitRegistration>,
}

impl UiWindow {
    pub fn retain_layout_close_failure(&mut self,stage:&'static str,failure:semio_framework_job::OriginalTurnFailure)->UiWorkerRetirementRefusal{
        let refusal=UiWorkerRetirementRefusal{stage,kind:failure.primary_value().map_or(semio_framework_value::ValueRefusalKind::InvariantViolated,|error|error.kind),progress:failure.retained_progress()};
        self.layout_close_failure=Some(semio_framework_value::retirement::controlled::ControlledRetirement::new(failure).unwrap_or_else(|_|unreachable!("original paired failure defines whole controlled closure")));self.layout_retirement_refusal=Some(refusal);refusal
    }
    pub(super) fn new(window_id: &str) -> Self {
        Self {
            closing: None,
            tree: UiTree::new(),
            router: EventRouter::new(window_id),
            presented_tree: UiTree::new(),
            presented_router: EventRouter::new(window_id),
            presented_ready: false,
            presented_source:None,
            presented_revision: 0,
            presented_accessibility_generation: 0,
            presented_interaction_epoch: 0,
            candidate_base_interaction_epoch: 0,
            candidate_ready: false,
            candidate_reconciled: false,
            candidate_retirements_resolved: false,
            candidate_preserves_prior_mounts: false,
            candidate_interaction_cursor: None,
            sealed_input_candidate: None,
            presented_tooltip: None,
            candidate_baseline: None,
            draw: DrawList::default(),
            viewport: (0.0, 0.0),
            layout_job: None,
            layout_session: None,
            layout_admission: LayoutWorkerAdmission::default(),
            layout_session_generation: None,
            layout_closing: false,
            layout_retirement_refusal: None,
            layout_close_failure:None,
            layout_preview: None,
            glyph_preview: None,
            lane: SurfaceLane::UserVisible,
            queued: false,
            intrinsic_content_height: f32::NAN,
            presented_intrinsic_content_height: f32::NAN,
            revision: 1,
            theme_revision: 1,
            viewport_revision: 1,
            layout_generation: 1,
            accessibility_generation: 0,
            document_ingress: None,
            retiring_document: None,
            retiring_presented_document: None,
            document_reconcile: UiDocumentReconcileCursor::default(),
            scene_retirements: VecDeque::with_capacity(UI_RETIRED_COMPONENT_SCENE_CAPACITY),
            candidate_scene_retirements: VecDeque::with_capacity(UI_RETIRED_COMPONENT_SCENE_CAPACITY),
            accepted_scene_retirements: VecDeque::with_capacity(UI_RETIRED_COMPONENT_SCENE_CAPACITY),
            component_mount_generation: 0,
            paint_frame: None,
            retiring_draw: None,
            paint_census: UiFramePaintCensus::default(),
            hit_registry: Vec::new(),
        }
    }

    /// 🚨️ Whether this window's root (and thus, transitively, anything below it per
    /// `UiTree::mark_dirty`'s bubbling) still needs a layout or paint pass.
    pub fn is_dirty(&self) -> bool {
        self.closing.is_none() && self.tree.root.and_then(|root| self.tree.node(root)).is_some_and(|node| node.flags.contains(NodeFlags::DIRTY_LAYOUT) || node.flags.contains(NodeFlags::DIRTY_PAINT) || node.flags.contains(NodeFlags::SUBTREE_DIRTY))
    }

    pub fn close_tree_storage_step(&mut self, _window_id: &str) -> bool {
        let Self { tree, .. } = self;
        tree.close_storage_step(&mut |_, _| true)
    }

    pub fn close_presented_tree_storage_step(&mut self, _window_id: &str) -> bool {
        if self.presented_source.is_some(){return false}
        let Self { presented_tree, .. } = self;
        presented_tree.close_storage_step(&mut |_, _| true)
    }

    pub fn begin_presented_interaction_rebase(&mut self) {
        if self.candidate_baseline.is_some() || !self.candidate_reconciled || !self.candidate_retirements_resolved {
            self.candidate_base_interaction_epoch = self.presented_interaction_epoch;
            self.candidate_ready = false;
            self.candidate_interaction_cursor = None;
            return;
        }
        if !self.presented_ready {
            self.candidate_base_interaction_epoch = self.presented_interaction_epoch;
            self.candidate_ready = true;
            self.candidate_interaction_cursor = None;
            return;
        }
        self.candidate_ready = false;
        self.candidate_interaction_cursor = Some(0);
    }

    pub fn step_presented_interaction_rebase(&mut self) -> bool {
        let Some(index) = self.candidate_interaction_cursor else { return self.candidate_ready };
        if !self.tree.transfer_interaction_record_from(&self.presented_tree, index) {
            self.candidate_interaction_cursor = Some(index.saturating_add(1));
            return false;
        }
        self.router.transfer_interaction_from(&self.presented_router, &self.presented_tree, &self.tree);
        self.candidate_base_interaction_epoch = self.presented_interaction_epoch;
        self.candidate_ready = true;
        self.candidate_interaction_cursor = None;
        true
    }

    pub fn advance_presented_interaction(&mut self) {
        let Some(epoch) = self.presented_interaction_epoch.checked_add(1) else { return };
        self.presented_interaction_epoch = epoch;
        self.begin_presented_interaction_rebase();
    }

    pub fn rearm_candidate_scene_retirements(&mut self) {
        while let Some(retirement) = self.accepted_scene_retirements.pop_front() {
            debug_assert!(self.candidate_scene_retirements.len() < UI_RETIRED_COMPONENT_SCENE_CAPACITY);
            self.candidate_scene_retirements.push_back(retirement);
        }
        self.candidate_retirements_resolved = false;
    }
}

pub fn tooltip_label_in_tree(tree: &UiTree, node: NodeId) -> Option<String> {
    let document = tree.document()?;
    let document_id = tree.document_id(node)?;
    let record = document.record(document_id)?;
    if record.accessibility.hidden {
        return None;
    }
    let label = record.accessibility.label.as_ref()?.0.as_str();
    if label.is_empty() {
        return None;
    }
    match record.accessibility.shortcut.as_ref().map(|shortcut| shortcut.as_str()) {
        Some(shortcut) if !shortcut.is_empty() => Some(format!("{label} ({shortcut})")),
        _ => Some(label.to_string()),
    }
}

pub fn synchronize_presented_tooltip(window: &mut UiWindow, surface: UiSurfaceToken) -> bool {
    if let Some(changed) = synchronize_presented_row_reason(window, surface) {
        return changed;
    }
    let valid = window.presented_tooltip.as_ref().is_some_and(|tooltip| {
        let revealed = window.presented_tree.document_node(tooltip.document_id);
        tooltip.row_action.is_none() && tooltip.surface == surface && tooltip.accessibility_generation == window.presented_accessibility_generation && revealed.is_some() && window.presented_router.revealed_tooltip_node() == revealed
    });
    if !valid {
        return window.presented_tooltip.take().is_some();
    }
    if let Some(tooltip) = window.presented_tooltip.as_mut() {
        tooltip.interaction_epoch = window.presented_interaction_epoch;
    }
    false
}

pub fn mark_window_paint_dirty(window: &mut UiWindow) {
    if let Some(root) = window.tree.root {
        window.tree.mark_dirty(root, NodeFlags::DIRTY_PAINT);
    }
}

pub fn presented_caret(surface: UiSurfaceToken, tree: &UiTree, step: semio_framework_ui_wgpu_events::CaretStep) -> Option<UiPresentedCaret> {
    Some(UiPresentedCaret {
        surface,
        document_id: tree.document_id(step.node)?,
        source: match step.source {
            CaretSource::Retained => UiCaretSource::Retained,
            CaretSource::Scene => UiCaretSource::Scene,
        },
        visible: step.visible,
    })
}

/// 💬️ Presents the router's disabled row-action reason hint in place of any hover tooltip, re-stamped with the current interaction
/// epoch; `None` while the router shows no hint. Answers whether the presented hint changed.
fn synchronize_presented_row_reason(window: &mut UiWindow, surface: UiSurfaceToken) -> Option<bool> {
    let (row, index) = window.presented_router.revealed_row_reason()?;
    let next = match (window.presented_tree.document_id(row), semio_framework_ui_wgpu_events::disabled_row_action_reason(&window.presented_tree, row, index)) {
        (Some(document_id), Some(label)) => Some(PresentedTooltip { surface, document_id, label: label.to_string(), accessibility_generation: window.presented_accessibility_generation, interaction_epoch: window.presented_interaction_epoch, row_action: Some(index) }),
        _ => None,
    };
    let shown = |tooltip: &PresentedTooltip| (tooltip.document_id, tooltip.label.clone(), tooltip.row_action);
    let changed = window.presented_tooltip.as_ref().map(shown) != next.as_ref().map(shown);
    window.presented_tooltip = next;
    Some(changed)
}

pub fn retained_tooltip_paint(window: &mut UiWindow, surface: UiSurfaceToken, atlas: &mut FontAtlas, theme: &Theme) -> Option<RetainedTooltipPaint> {
    let tooltip = window.presented_tooltip.as_ref()?;
    let accepted_node = window.presented_tree.document_node(tooltip.document_id);
    let revealed = match tooltip.row_action {
        Some(index) => accepted_node.is_some() && window.presented_router.revealed_row_reason() == accepted_node.map(|node| (node, index)),
        None => window.presented_router.revealed_tooltip_node() == accepted_node,
    };
    let valid = tooltip.surface == surface && tooltip.accessibility_generation == window.presented_accessibility_generation && tooltip.interaction_epoch == window.presented_interaction_epoch && accepted_node.is_some() && revealed;
    if !valid {
        window.presented_tooltip = None;
        return None;
    }
    let candidate_node = window.tree.document_node(tooltip.document_id)?;
    let label = tooltip.label.clone();
    let size = tooltip_surface_size(&label, theme, atlas);
    let placement = match tooltip.row_action {
        Some(index) => {
            let icon = window.router.row_action_icon_rect(&window.tree, candidate_node, index)?;
            ui_contract::resolve_overlay_placement(semio_framework_ui_wgpu_events::overlay_rect(icon), size, window.viewport, OverlayKind::Tooltip.default_placement(), window.router.flow().inline)
        }
        None => resolve_overlay_placement_side(&window.tree, OverlayAnchor::Node(candidate_node), size, window.viewport, OverlayKind::Tooltip.default_placement(), window.router.flow().inline),
    };
    Some(RetainedTooltipPaint { label, x: placement.x, y: placement.y })
}

pub fn paint_retained_tooltip(draw: &mut DrawList, atlas: &mut FontAtlas, tooltip: RetainedTooltipPaint, offset: (f32, f32), theme: &Theme) {
    draw.begin_overlay_route();
    paint_tooltip(draw, atlas, &tooltip.label, (offset.0 + tooltip.x, offset.1 + tooltip.y), theme);
    draw.end_overlay_route();
}
//#endregion 🔖️UiWindow
