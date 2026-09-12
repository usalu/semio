// #region engine
//! 🧵️ The retained-mode `Ui` façade: the missing keystone tying `arena`/`tree`/`reconcile`/`flex`/
//! `paint`/`events`/`scene_slots`/`shell` into one usable pipeline — each of those regions was built
//! and individually tested to its own milestone but nothing ever assembled them together, and nothing
//! in `framework/renderer/wgpu` calls into any of it (see `.🧬semio/🦑️repo/🎫️tickets/26/07/11/RETAINED-MODE-UI-CRATE`'s
//! plan for the historical intent). This module is purely additive: the immediate-mode `widgets`
//! path stays the only pipeline actually driving pixels until a later workstream proves this façade
//! out (via the golden `tests` module below) and cuts over.

use std::collections::HashMap;

use crate::wgpu::component::layout::WindowLayout;
#[cfg(any(test, feature = "testkit"))]
use crate::wgpu::component::ui::UiNode;
use crate::wgpu::draw::{DrawList, IconAtlas};
use crate::wgpu::events::{EventRouter, UiCommand, UiEvent};
use crate::wgpu::flex::{LayoutJobStage, LayoutJobStep};
use crate::wgpu::mounted_layout::{MountedLayoutIdentity, MountedLayoutJob, MountedLayoutResult, RetainedGlyphPreview};
#[cfg(test)]
use crate::wgpu::paint::paint_tree;
use crate::wgpu::paint::{paint_node_step, sync_interactive_state_node_step, RetainedInteractiveSyncCursor, RetainedInteractiveSyncStep, RetainedNodePaintCursor, RetainedNodePaintStep};
#[cfg(test)]
use crate::wgpu::scene_slots::collect_scene_slots;
use crate::wgpu::scene_slots::{scene_slot_for_node, SceneHost, ScenePaintCursor, ScenePaintStep};
use crate::wgpu::shell::{Shell, ShellEvent};
use crate::wgpu::text::FontAtlas;
use crate::wgpu::theme::Theme;
use crate::wgpu::reconcile::{UiDocumentReconcileCursor, UiDocumentReconcileStep};
use crate::wgpu::tree::{NodeFlags, UiDocumentPageRejection, UiDocumentTree, UiDocumentTreeFault, UiTree};
use crate::wgpu::IconName;
use semio_framework_job::StepContext;
use ui_contract::{SurfaceId, UiDocumentLeaseHeader, UiDocumentNodePage, UiFixedList, UiNodeId, UI_DOCUMENT_NODES};

//#region 🔖️UiWindow
/// 🪟️ One window's retained pipeline state: its `UiTree` (`reconcile`'s diff target), the taffy
/// `LayoutEngine` that lays it out (`flex`), the `EventRouter` owning its capture/focus/hover state
/// (`events`), and the `DrawList` `paint::paint_tree` last painted into. Mirrors `tree`'s own doc
/// comment ("the engine facade... holds `HashMap<window_id, UiTree>`") by keying the *whole*
/// per-window pipeline the same way, not just the tree.
struct UiWindow {
    tree: UiTree,
    router: EventRouter,
    draw: DrawList,
    viewport: (f32, f32),
    layout_job: Option<MountedLayoutJob>,
    layout_session: Option<semio_framework_job::MountedWorkerJobSession<MountedLayoutJob>>,
    layout_rejected: Option<semio_framework_job::WorkerJobSessionAdmissionRejected<MountedLayoutJob>>,
    layout_closing: bool,
    layout_preview: Option<MountedLayoutResult>,
    glyph_preview: Option<RetainedGlyphPreview>,
    lane: SurfaceLane,
    queued: bool,
    revision: u64,
    theme_revision: u64,
    viewport_revision: u64,
    layout_generation: u64,
    document_ingress: Option<UiDocumentIngress>,
    retiring_document: Option<UiDocumentTree>,
    /// 🌳️ The published document's own reconcile into the paintable arena — the one thing that ever
    /// sets `tree.root` in production (see `🔁️reconcile.rs`'s `🌳️DocumentTreeReconcile`).
    document_reconcile: UiDocumentReconcileCursor,
    paint_frame: Option<RetainedPaintFrame>,
    retiring_draw: Option<DrawList>,
    paint_census: UiFramePaintCensus,
}

impl UiWindow {
    fn new(window_id: &str) -> Self {
        Self {
            tree: UiTree::new(),
            router: EventRouter::new(window_id),
            draw: DrawList::default(),
            viewport: (0.0, 0.0),
            layout_job: None,
            layout_session: None,
            layout_rejected: None,
            layout_closing: false,
            layout_preview: None,
            glyph_preview: None,
            lane: SurfaceLane::UserVisible,
            queued: false,
            revision: 1,
            theme_revision: 1,
            viewport_revision: 1,
            layout_generation: 1,
            document_ingress: None,
            retiring_document: None,
            document_reconcile: UiDocumentReconcileCursor::default(),
            paint_frame: None,
            retiring_draw: None,
            paint_census: UiFramePaintCensus::default(),
        }
    }

    /// 🚨️ Whether this window's root (and thus, transitively, anything below it per
    /// `UiTree::mark_dirty`'s bubbling) still needs a layout or paint pass.
    fn is_dirty(&self) -> bool {
        self.tree.root.and_then(|root| self.tree.node(root)).is_some_and(|node| node.flags.contains(NodeFlags::DIRTY_LAYOUT) || node.flags.contains(NodeFlags::DIRTY_PAINT) || node.flags.contains(NodeFlags::SUBTREE_DIRTY))
    }
}

const RETAINED_PAINT_DEPTH_CREDITS: usize = 64;

#[derive(Clone, Copy)]
struct RetainedPaintVisit {
    node: crate::wgpu::arena::NodeId,
    origin_x: f32,
    origin_y: f32,
    next_child: Option<crate::wgpu::arena::NodeId>,
    entered: bool,
}

struct RetainedPaintWalk {
    visits: [Option<RetainedPaintVisit>; RETAINED_PAINT_DEPTH_CREDITS],
    len: usize,
}

enum RetainedPaintWalkStep {
    Visit(crate::wgpu::arena::NodeId, f32, f32),
    Scalar,
    Complete,
    DepthFault,
}

impl RetainedPaintWalk {
    fn new(tree: &UiTree, root: crate::wgpu::arena::NodeId) -> Self {
        let root_visit = RetainedPaintVisit { node: root, origin_x: 0.0, origin_y: 0.0, next_child: tree.node(root).and_then(|node| node.first_child), entered: false };
        let mut visits = [None; RETAINED_PAINT_DEPTH_CREDITS];
        visits[0] = Some(root_visit);
        Self { visits, len: 1 }
    }

    fn step(&mut self, tree: &UiTree) -> RetainedPaintWalkStep {
        let Some(index) = self.len.checked_sub(1) else { return RetainedPaintWalkStep::Complete };
        let Some(visit) = self.visits[index].as_mut() else { return RetainedPaintWalkStep::DepthFault };
        if !visit.entered {
            visit.entered = true;
            return RetainedPaintWalkStep::Visit(visit.node, visit.origin_x, visit.origin_y);
        }
        if let Some(child) = visit.next_child {
            visit.next_child = tree.node(child).and_then(|node| node.next_sibling);
            if self.len == RETAINED_PAINT_DEPTH_CREDITS {
                return RetainedPaintWalkStep::DepthFault;
            }
            let Some(layout) = tree.accepted_layout(visit.node) else { return RetainedPaintWalkStep::DepthFault };
            let child_visit = RetainedPaintVisit { node: child, origin_x: visit.origin_x + layout.x, origin_y: visit.origin_y + layout.y, next_child: tree.node(child).and_then(|node| node.first_child), entered: false };
            self.visits[self.len] = Some(child_visit);
            self.len += 1;
            return RetainedPaintWalkStep::Scalar;
        }
        self.visits[index] = None;
        self.len = index;
        RetainedPaintWalkStep::Scalar
    }
}

#[derive(Clone, Copy)]
enum RetainedPaintPhase {
    Synchronize,
    Paint,
    Scenes,
    Publish,
    Complete,
    Fault,
}

/// 📊️ What one window's own retained paint contributed to a frame: non-empty draw layers, quad
/// instances, and the glyph subset of those quads.
///
/// The production path is `frame_into_step`, which paints into the CALLER's draw list and never
/// publishes into `UiWindow::draw` — so `Ui::draw_list` (and every probe reading it) reports an empty
/// list no matter how much the window painted. This census is measured as the DELTA the window's own
/// paint frame appended to whatever target it was handed, so it answers the same question for both
/// paint entries (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-document-reconcile-2026-09-12.md`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct UiFramePaintCensus {
    pub layers: usize,
    pub quads: usize,
    pub glyphs: usize,
    /// 🌍️ The 3d half of the same frame. A `World3d` window's whole output is `ScenePass3d`s —
    /// instanced mesh draws, translucent draws, textured draws and the grid's line draws — and none
    /// of it is a `UiInstance`, so a census counting only quads reported `0` for a surface that was
    /// drawing a solid (ticket 26/09/09/PROCEDURAL-3D-END-TO-END). A layer a scene pass targets now
    /// counts as non-empty for the same reason.
    pub scene_passes: usize,
    pub scene_draws: usize,
    pub scene_instances: usize,
}

impl UiFramePaintCensus {
    fn of(draw: &DrawList) -> Self {
        let mut census = Self::default();
        let scene_layers: std::collections::BTreeSet<usize> = draw.scene_passes.iter().map(|pass| pass.layer_index).collect();
        for (index, layer) in draw.layers.iter().enumerate() {
            if !layer.ui_instances.is_empty() || !layer.raster_instances.is_empty() || !layer.vector_vertices.is_empty() || !layer.overlay_ui_instances.is_empty() || !layer.overlay_vector_vertices.is_empty() || scene_layers.contains(&index) {
                census.layers += 1;
            }
            for instance in layer.ui_instances.iter().chain(layer.overlay_ui_instances.iter()) {
                census.quads += 1;
                if instance.params[2] == crate::wgpu::draw_types::KIND_GLYPH {
                    census.glyphs += 1;
                }
            }
        }
        census.scene_passes = draw.scene_passes.len();
        for pass in &draw.scene_passes {
            census.scene_draws += pass.draws.len() + pass.translucent_draws.len() + pass.textured_draws.len() + pass.line_draws.len();
            census.scene_instances += pass.draws.iter().chain(pass.translucent_draws.iter()).map(|draw| draw.instances.len()).sum::<usize>() + pass.textured_draws.iter().map(|draw| draw.instances.len()).sum::<usize>();
        }
        census
    }

    fn since(self, baseline: Self) -> Self {
        Self {
            layers: self.layers.saturating_sub(baseline.layers),
            quads: self.quads.saturating_sub(baseline.quads),
            glyphs: self.glyphs.saturating_sub(baseline.glyphs),
            scene_passes: self.scene_passes.saturating_sub(baseline.scene_passes),
            scene_draws: self.scene_draws.saturating_sub(baseline.scene_draws),
            scene_instances: self.scene_instances.saturating_sub(baseline.scene_instances),
        }
    }
}

struct RetainedPaintFrame {
    phase: RetainedPaintPhase,
    walk: RetainedPaintWalk,
    candidate: DrawList,
    sync_node: Option<crate::wgpu::arena::NodeId>,
    node_sync: RetainedInteractiveSyncCursor,
    paint_node: Option<(crate::wgpu::arena::NodeId, f32, f32)>,
    node_paint: RetainedNodePaintCursor,
    scene_node: Option<(crate::wgpu::arena::NodeId, f32, f32)>,
    scene_paint: ScenePaintCursor,
    revision: u64,
    theme_revision: u64,
    viewport_revision: u64,
    baseline: UiFramePaintCensus,
    /// 🩺️ Which sub-step drove this frame terminal — a `UiFrameStep::Fault` is otherwise
    /// undiagnosable from outside the engine.
    fault_site: Option<&'static str>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UiFrameStep {
    Pending,
    Ready,
    Missing,
    Fault,
}
pub const UI_LAYOUT_SURFACE_SLOTS: usize = 64;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UiSurfaceToken {
    slot: u8,
    generation: u64,
}

impl UiSurfaceToken {
    #[cfg(test)]
    pub(crate) const fn new(slot: u8, generation: u64) -> Self {
        Self { slot, generation }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct UiSurfaceAdmissionRejected {
    pub id: SurfaceId,
}

struct UiSurfaceSlot {
    id: SurfaceId,
    generation: u64,
    window: UiWindow,
}

/// 🧱️ The retained window table. `slots` is boxed because `UiSurfaceSlot` carries a whole
/// [`UiWindow`] (~186 KiB): held inline, the 64-slot array is ~11.9 MiB of `Ui`, materialised in the
/// constructing frame by `array::from_fn` and copied again by every by-value move of `Ui`
/// (`Ui::new` → `Mutex::new` → `OnceLock::get_or_init` stacked ~88 MiB). Boxed, the array is built
/// straight on the heap by [`semio_framework_async::boxed_fixed_slots`] and `Ui` stays pointer-sized
/// here. `generations` is `[u64; 64]` = 512 B and stays inline.
struct UiSurfaceRegistry {
    slots: Box<[Option<UiSurfaceSlot>; UI_LAYOUT_SURFACE_SLOTS]>,
    generations: [u64; UI_LAYOUT_SURFACE_SLOTS],
}

impl Default for UiSurfaceRegistry {
    fn default() -> Self {
        Self { slots: semio_framework_async::boxed_fixed_slots(|| None), generations: [0; UI_LAYOUT_SURFACE_SLOTS] }
    }
}

impl UiSurfaceRegistry {
    fn token(&self, id: &str) -> Option<UiSurfaceToken> {
        let slot = self.slots.iter().position(|slot| slot.as_ref().is_some_and(|slot| slot.id.as_ref() == id))?;
        Some(UiSurfaceToken { slot: slot as u8, generation: self.slots[slot].as_ref()?.generation })
    }

    #[expect(clippy::result_large_err, reason = "Surface and document admission return the exact refused identity or page without allocating a rejection wrapper.")]
    fn try_admit(&mut self, id: SurfaceId) -> Result<UiSurfaceToken, UiSurfaceAdmissionRejected> {
        if let Some(token) = self.token(id.as_ref()) {
            return Ok(token);
        }
        let Some(slot) = self.slots.iter().position(Option::is_none) else { return Err(UiSurfaceAdmissionRejected { id }) };
        let Some(generation) = self.generations[slot].checked_add(1) else { return Err(UiSurfaceAdmissionRejected { id }) };
        self.generations[slot] = generation;
        let token = UiSurfaceToken { slot: slot as u8, generation };
        self.slots[slot] = Some(UiSurfaceSlot { window: UiWindow::new(id.as_ref()), id, generation });
        Ok(token)
    }

    fn get(&self, id: &str) -> Option<&UiWindow> {
        self.get_token(self.token(id)?)
    }

    fn get_mut(&mut self, id: &str) -> Option<&mut UiWindow> {
        self.get_token_mut(self.token(id)?)
    }

    fn get_token(&self, token: UiSurfaceToken) -> Option<&UiWindow> {
        let slot = self.slots.get(token.slot as usize)?.as_ref()?;
        (slot.generation == token.generation).then_some(&slot.window)
    }

    fn get_token_mut(&mut self, token: UiSurfaceToken) -> Option<&mut UiWindow> {
        let slot = self.slots.get_mut(token.slot as usize)?.as_mut()?;
        (slot.generation == token.generation).then_some(&mut slot.window)
    }

    fn id(&self, token: UiSurfaceToken) -> Option<&SurfaceId> {
        let slot = self.slots.get(token.slot as usize)?.as_ref()?;
        (slot.generation == token.generation).then_some(&slot.id)
    }

    fn token_at(&self, index: usize) -> Option<UiSurfaceToken> {
        let slot = self.slots.get(index)?.as_ref()?;
        Some(UiSurfaceToken { slot: index as u8, generation: slot.generation })
    }

    fn values(&self) -> impl Iterator<Item = &UiWindow> {
        self.slots.iter().filter_map(|slot| slot.as_ref().map(|slot| &slot.window))
    }

    fn ids(&self) -> impl Iterator<Item = &SurfaceId> {
        self.slots.iter().filter_map(|slot| slot.as_ref().map(|slot| &slot.id))
    }
}
//#endregion 🔖️UiWindow

//#region 📄️DocumentIngress
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UiDocumentIngressFault {
    Cancelled,
    Deadline,
    StaleGeneration,
    InterruptedClose,
    ValidationPending,
    Invalid(UiDocumentTreeFault),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UiDocumentIngressStatus {
    Vacant,
    Pending { next_page: usize, node_count: usize },
    Published,
}

struct UiDocumentIngress {
    document: UiDocumentTree,
    next_page: usize,
    node_count: usize,
    validation_cursor: usize,
    validation_started: bool,
    validation_stack: UiFixedList<UiNodeId, UI_DOCUMENT_NODES>,
    validation_seen: UiFixedList<UiNodeId, UI_DOCUMENT_NODES>,
}
//#endregion 📄️DocumentIngress

//#region 🚦️SurfaceScheduling
/// 🚦️Priority lane for resumable per-surface layout. The weighted wheel favors direct
/// interaction without allowing user-visible or background surfaces to starve.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SurfaceLane {
    Interactive,
    #[default]
    UserVisible,
    Background,
}

impl SurfaceLane {
    const fn index(self) -> usize {
        match self {
            Self::Interactive => 0,
            Self::UserVisible => 1,
            Self::Background => 2,
        }
    }
}

struct SurfaceLaneRing {
    slots: [Option<SurfaceLaneEntry>; UI_LAYOUT_SURFACE_SLOTS],
    head: usize,
    len: usize,
}

impl Default for SurfaceLaneRing {
    fn default() -> Self {
        Self { slots: [None; UI_LAYOUT_SURFACE_SLOTS], head: 0, len: 0 }
    }
}

impl SurfaceLaneRing {
    fn try_push(&mut self, entry: SurfaceLaneEntry) -> Result<(), SurfaceLaneEntry> {
        if self.len == UI_LAYOUT_SURFACE_SLOTS {
            return Err(entry);
        }
        let tail = (self.head + self.len) % UI_LAYOUT_SURFACE_SLOTS;
        self.slots[tail] = Some(entry);
        self.len += 1;
        Ok(())
    }

    fn pop(&mut self) -> Option<SurfaceLaneEntry> {
        if self.len == 0 {
            return None;
        }
        let token = self.slots[self.head].take();
        self.head = (self.head + 1) % UI_LAYOUT_SURFACE_SLOTS;
        self.len -= 1;
        token
    }

    #[cfg(test)]
    fn len(&self) -> usize {
        self.len
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SurfaceLayoutReason {
    Dirty,
    Metrics,
    Theme,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct SurfaceLaneEntry {
    token: UiSurfaceToken,
    reason: SurfaceLayoutReason,
    epoch: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ThemePropagationPhase {
    Validate,
    Apply,
    Publish,
}

struct ThemePropagationCursor {
    theme: Theme,
    tokens: [Option<UiSurfaceToken>; UI_LAYOUT_SURFACE_SLOTS],
    slot: usize,
    phase: ThemePropagationPhase,
}

impl ThemePropagationCursor {
    fn new(theme: Theme) -> Self {
        Self { theme, tokens: [None; UI_LAYOUT_SURFACE_SLOTS], slot: 0, phase: ThemePropagationPhase::Validate }
    }
}

const LANE_WHEEL: [SurfaceLane; 6] = [SurfaceLane::Interactive, SurfaceLane::Interactive, SurfaceLane::UserVisible, SurfaceLane::Interactive, SurfaceLane::UserVisible, SurfaceLane::Background];

/// 🧭️Observable result of exactly one bounded surface-layout scheduling call.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum UiLayoutStep {
    Idle,
    Yielded { window_id: SurfaceId, lane: SurfaceLane, stage: &'static str, nodes: usize, glyphs: usize },
    Ready { window_id: SurfaceId, lane: SurfaceLane },
    Cancelled { window_id: SurfaceId, lane: SurfaceLane },
}

fn stage_label(stage: LayoutJobStage) -> &'static str {
    match stage {
        LayoutJobStage::CollectNodes => "Layout.CollectNodes",
        LayoutJobStage::ShapeText => "Layout.ShapeText",
        #[cfg(test)]
        LayoutJobStage::PruneRemoved => "Layout.PruneRemoved",
        #[cfg(test)]
        LayoutJobStage::SyncNodes => "Layout.SyncNodes",
        #[cfg(test)]
        LayoutJobStage::SolveLayout => "Layout.SolveLayout",
        LayoutJobStage::MeasureFallback => "Layout.MeasureFallback",
        LayoutJobStage::ArrangeFallback => "Layout.ArrangeFallback",
        #[cfg(test)]
        LayoutJobStage::CollectResults => "Layout.CollectResults",
        LayoutJobStage::PublishResults => "Layout.PublishResults",
    }
}

fn theme_layout_identity(theme: &Theme) -> [u32; 4] {
    [theme.gap_standard.to_bits(), theme.padding_standard.to_bits(), theme.font_size_small.to_bits(), theme.font_size_body.to_bits()]
}

fn worker_lane(lane: SurfaceLane) -> semio_framework_async::Lane {
    match lane {
        SurfaceLane::Interactive => semio_framework_async::Lane::Interactive,
        SurfaceLane::UserVisible => semio_framework_async::Lane::UserVisible,
        SurfaceLane::Background => semio_framework_async::Lane::Background,
    }
}
//#endregion 🚦️SurfaceScheduling

//#region 🔖️Ui
/// 🧵️ Assembles the individually-milestoned retained modules into the one façade a host drives per
/// tick: `apply_tree` runs `reconcile`, `frame` runs `flex` (dirty-gated) then `paint` then hands
/// `scene_slots` to an optional `SceneHost`, `dispatch_event` runs `events::EventRouter`, and
/// `needs_frame` reads the same dirty flags `frame` itself gates on. One `UiWindow` per window id
/// (app-content trees); window-chrome (dock/split/tab) is the separate `🐚️Shell` this façade also owns,
/// since `shell`'s own doc comment models it as independent of any single window's content tree.
/// Never submits to the GPU itself — `frame` returns a `&DrawList` for the caller to hand to the
/// existing `gpu::GpuContext::render_frame`, exactly like the immediate-mode `widgets` path's callers
/// already do; wiring that hand-off into a real host event loop is later, renderer-thinning work.
pub struct Ui {
    windows: UiSurfaceRegistry,
    shell: Shell,
    theme: Theme,
    pending_commands: Vec<UiCommand>,
    layout_queues: [SurfaceLaneRing; 3],
    layout_pressure: Option<SurfaceLaneEntry>,
    lane_cursor: usize,
    theme_propagation: Option<ThemePropagationCursor>,
    pending_theme: Option<Theme>,
    theme_fault: bool,
}

#[cfg(not(target_arch = "wasm32"))]
const _: fn() = || {
    fn assert_send<T: Send>() {}
    assert_send::<Ui>();
};

impl Ui {
    pub fn new() -> Self {
        Self {
            windows: UiSurfaceRegistry::default(),
            shell: Shell::new(),
            theme: Theme::default(),
            pending_commands: Vec::new(),
            layout_queues: std::array::from_fn(|_| SurfaceLaneRing::default()),
            layout_pressure: None,
            lane_cursor: 0,
            theme_propagation: None,
            pending_theme: None,
            theme_fault: false,
        }
    }

    pub fn set_theme(&mut self, theme: Theme) {
        let layout_changed = theme_layout_identity(&self.theme) != theme_layout_identity(&theme);
        if !layout_changed && self.theme_propagation.is_none() {
            self.theme = theme;
            return;
        }
        if self.theme_propagation.is_some() {
            self.pending_theme = Some(theme);
        } else {
            self.theme_propagation = Some(ThemePropagationCursor::new(theme));
        }
    }

    #[expect(clippy::result_large_err, reason = "Surface and document admission return the exact refused identity or page without allocating a rejection wrapper.")]
    pub fn try_admit_surface(&mut self, window_id: &str) -> Result<UiSurfaceToken, UiSurfaceAdmissionRejected> {
        let id = SurfaceId::try_from(window_id).map_err(|_| UiSurfaceAdmissionRejected { id: SurfaceId::default() })?;
        self.windows.try_admit(id)
    }

    fn window_mut(&mut self, window_id: &str) -> Option<&mut UiWindow> {
        let token = self.try_admit_surface(window_id).ok()?;
        self.windows.get_token_mut(token)
    }

    /// 📐️ Stores the viewport a later `frame` call lays out against for `window_id`, creating that
    /// window's retained state on first use.
    pub fn set_viewport(&mut self, window_id: &str, width: f32, height: f32) {
        let Some(window) = self.window_mut(window_id) else { return };
        if window.viewport == (width, height) {
            return;
        }
        let Some(next_generation) = window.layout_generation.checked_add(1) else { return };
        let Some(next_viewport_revision) = window.viewport_revision.checked_add(1) else { return };
        window.viewport = (width, height);
        window.layout_generation = next_generation;
        window.viewport_revision = next_viewport_revision;
        if let Some(root) = window.tree.root {
            window.tree.mark_dirty(root, NodeFlags::DIRTY_LAYOUT);
        }
        self.enqueue_layout_reason(window_id, SurfaceLayoutReason::Metrics);
    }

    /// 🔁️ Runs `UiTree::apply_tree` (`reconcile`) to diff `ui_node` into `window_id`'s retained tree,
    /// creating that window's tree/layout-engine/event-router on first use.
    #[cfg(any(test, feature = "testkit"))]
    pub fn apply_tree(&mut self, window_id: &str, ui_node: &UiNode) {
        let Some(window) = self.window_mut(window_id) else { return };
        let unchanged = window.tree.root.and_then(|root| window.tree.node(root)).is_some_and(|node| node.spec.0 == *ui_node);
        if !unchanged && (window.revision == u64::MAX || window.layout_generation == u64::MAX) {
            return;
        }
        window.tree.apply_tree(ui_node);
        if !unchanged {
            window.revision += 1;
            window.layout_generation += 1;
        }
        let needs_layout = window.tree.root.and_then(|root| window.tree.node(root)).is_some_and(|node| node.flags.contains(NodeFlags::DIRTY_LAYOUT) || node.flags.contains(NodeFlags::SUBTREE_DIRTY));
        if needs_layout {
            self.enqueue_layout(window_id);
        }
    }

    //#region 📄️DocumentIngress
    pub fn document_status(&self, window_id: &str, generation: u64) -> UiDocumentIngressStatus {
        let Some(window) = self.windows.get(window_id) else { return UiDocumentIngressStatus::Vacant };
        if window.tree.document().is_some_and(|document| document.generation() == generation) {
            return UiDocumentIngressStatus::Published;
        }
        window
            .document_ingress
            .as_ref()
            .filter(|ingress| ingress.document.generation() == generation)
            .map_or(UiDocumentIngressStatus::Vacant, |ingress| UiDocumentIngressStatus::Pending { next_page: ingress.next_page, node_count: ingress.node_count })
    }

    #[expect(clippy::result_large_err, reason = "Surface and document admission return the exact refused identity or page without allocating a rejection wrapper.")]
    pub fn begin_document(&mut self, window_id: &str, header: UiDocumentLeaseHeader, cx: &mut StepContext<'_>) -> Result<(), (UiDocumentIngressFault, UiDocumentLeaseHeader)> {
        if cx.is_cancelled() {
            return Err((UiDocumentIngressFault::Cancelled, header));
        }
        if cx.should_yield() {
            return Err((UiDocumentIngressFault::Deadline, header));
        }
        if cx.generation().0 != header.generation {
            return Err((UiDocumentIngressFault::StaleGeneration, header));
        }
        let Some(window) = self.window_mut(window_id) else { return Err((UiDocumentIngressFault::StaleGeneration, header)) };
        if let Some(retiring) = window.retiring_document.as_mut() {
            if !retiring.close_step() {
                return Err((UiDocumentIngressFault::InterruptedClose, header));
            }
            window.retiring_document = None;
            return Err((UiDocumentIngressFault::InterruptedClose, header));
        }
        if let Some(ingress) = window.document_ingress.as_mut() {
            if ingress.document.generation() == header.generation {
                return Ok(());
            }
            if !ingress.document.close_step() {
                return Err((UiDocumentIngressFault::InterruptedClose, header));
            }
            window.document_ingress = None;
            return Err((UiDocumentIngressFault::InterruptedClose, header));
        }
        if window.tree.document().is_some_and(|document| document.generation() >= header.generation) {
            return Err((UiDocumentIngressFault::StaleGeneration, header));
        }
        let node_count = header.node_count;
        let document = UiDocumentTree::new(header.clone()).map_err(|fault| (UiDocumentIngressFault::Invalid(fault), header))?;
        window.document_ingress = Some(UiDocumentIngress { document, next_page: 0, node_count, validation_cursor: 0, validation_started: false, validation_stack: UiFixedList::default(), validation_seen: UiFixedList::default() });
        Ok(())
    }

    #[expect(clippy::result_large_err, reason = "Surface and document admission return the exact refused identity or page without allocating a rejection wrapper.")]
    pub fn apply_document_page(&mut self, window_id: &str, page: UiDocumentNodePage, cx: &mut StepContext<'_>) -> Result<usize, UiDocumentPageRejection> {
        let Some(window) = self.window_mut(window_id) else {
            return Err(UiDocumentPageRejection { fault: UiDocumentTreeFault::Generation, generation: page.generation(), revision: page.revision(), index: page.index(), record: page.into_record() });
        };
        let Some(ingress) = window.document_ingress.as_mut() else {
            return Err(UiDocumentPageRejection { fault: UiDocumentTreeFault::Generation, generation: page.generation(), revision: page.revision(), index: page.index(), record: page.into_record() });
        };
        if cx.is_cancelled() || cx.should_yield() || cx.generation().0 != ingress.document.generation() {
            return Err(UiDocumentPageRejection { fault: UiDocumentTreeFault::Generation, generation: page.generation(), revision: page.revision(), index: page.index(), record: page.into_record() });
        }
        ingress.document.try_push_page(page, ingress.next_page)?;
        ingress.next_page += 1;
        cx.consume_fuel(1);
        Ok(ingress.next_page)
    }

    pub fn finish_document(&mut self, window_id: &str, generation: u64, cx: &mut StepContext<'_>) -> Result<(), UiDocumentIngressFault> {
        if cx.is_cancelled() {
            return Err(UiDocumentIngressFault::Cancelled);
        }
        if cx.should_yield() {
            return Err(UiDocumentIngressFault::Deadline);
        }
        if cx.generation().0 != generation {
            return Err(UiDocumentIngressFault::StaleGeneration);
        }
        let Some(window) = self.window_mut(window_id) else { return Err(UiDocumentIngressFault::StaleGeneration) };
        let Some(ingress) = window.document_ingress.as_mut() else { return Err(UiDocumentIngressFault::StaleGeneration) };
        if ingress.document.generation() != generation || ingress.next_page != ingress.node_count {
            return Err(UiDocumentIngressFault::StaleGeneration);
        }
        ingress.document.validate_header().map_err(UiDocumentIngressFault::Invalid)?;
        if ingress.validation_cursor < ingress.node_count {
            ingress.document.validate_record(ingress.validation_cursor).map_err(UiDocumentIngressFault::Invalid)?;
            ingress.validation_cursor += 1;
            return Err(UiDocumentIngressFault::ValidationPending);
        }
        if !ingress.validation_started {
            ingress.validation_stack.try_push(ingress.document.root_id()).map_err(|_| UiDocumentIngressFault::Invalid(UiDocumentTreeFault::NodeCapacity))?;
            ingress.validation_started = true;
            return Err(UiDocumentIngressFault::ValidationPending);
        }
        if let Some(id) = ingress.validation_stack.pop() {
            if ingress.validation_seen.iter().any(|visited| *visited == id) {
                return Err(UiDocumentIngressFault::Invalid(UiDocumentTreeFault::Cycle));
            }
            ingress.validation_seen.try_push(id).map_err(|_| UiDocumentIngressFault::Invalid(UiDocumentTreeFault::NodeCapacity))?;
            let record = ingress.document.record(id).ok_or(UiDocumentIngressFault::Invalid(UiDocumentTreeFault::MissingChild))?;
            for child in record.children.iter().rev() {
                ingress.validation_stack.try_push(*child).map_err(|_| UiDocumentIngressFault::Invalid(UiDocumentTreeFault::NodeCapacity))?;
            }
            return Err(UiDocumentIngressFault::ValidationPending);
        }
        if ingress.validation_seen.len() != ingress.node_count {
            return Err(UiDocumentIngressFault::Invalid(UiDocumentTreeFault::Cycle));
        }
        let Some(next_revision) = window.revision.checked_add(1) else { return Err(UiDocumentIngressFault::StaleGeneration) };
        let Some(next_layout_generation) = window.layout_generation.checked_add(1) else { return Err(UiDocumentIngressFault::StaleGeneration) };
        let Some(ingress) = window.document_ingress.take() else { return Err(UiDocumentIngressFault::StaleGeneration) };
        window.retiring_document = window.tree.publish_document(ingress.document);
        window.revision = next_revision;
        window.layout_generation = next_layout_generation;
        if let Some(root) = window.tree.root {
            window.tree.mark_dirty(root, NodeFlags::DIRTY_LAYOUT);
        }
        self.enqueue_layout(window_id);
        Ok(())
    }

    /// 📄️ Publishes an already-assembled document into `window_id` WITHOUT replaying the paged
    /// ingress — the page ladder is pinned by its own laws (`📃️document-lease-owner-move`), and a
    /// `UiDocumentNodePage` has no public constructor, so a reconcile law would otherwise have to
    /// drive the process-wide `UI_DOCUMENT_ARENA` to say anything about the arena. Bookkeeping is
    /// byte-identical to `finish_document`'s own tail.
    #[cfg(any(test, feature = "testkit"))]
    pub fn publish_document(&mut self, window_id: &str, document: UiDocumentTree) -> bool {
        let Some(window) = self.window_mut(window_id) else { return false };
        let Some(next_revision) = window.revision.checked_add(1) else { return false };
        let Some(next_layout_generation) = window.layout_generation.checked_add(1) else { return false };
        window.retiring_document = window.tree.publish_document(document);
        window.revision = next_revision;
        window.layout_generation = next_layout_generation;
        if let Some(root) = window.tree.root {
            window.tree.mark_dirty(root, NodeFlags::DIRTY_LAYOUT);
        }
        self.enqueue_layout(window_id);
        true
    }

    /// 🌳️ Advances `window_id`'s published document into its paintable arena, spending this
    /// opportunity's whole budget rather than one unit, and enqueues the layout the freshly mounted
    /// root now needs.
    ///
    /// `UiTree::publish_document` stores the document and nothing read it back — `tree.root` stayed
    /// `None` forever and `frame_into_step` answered `Missing` on its first line
    /// (`📓️wgpu-blank-paint-2026-09-12.md` §5). This is that missing edge: the ONLY production writer
    /// of the arena, `Ui::apply_tree` being `cfg(test/testkit)`.
    pub fn step_document_reconcile(&mut self, window_id: &str, controller: &str, cx: &mut StepContext<'_>) -> UiDocumentReconcileStep {
        let Some(window) = self.window_mut(window_id) else { return UiDocumentReconcileStep::Pending };
        let Some(generation) = window.tree.document().map(UiDocumentTree::generation) else { return UiDocumentReconcileStep::Pending };
        window.document_reconcile.rearm(generation);
        if window.document_reconcile.terminal_is_complete() {
            return UiDocumentReconcileStep::Complete;
        }
        let UiWindow { tree, document_reconcile, .. } = window;
        let step = loop {
            let step = tree.step_document_reconcile(document_reconcile, window_id, controller);
            cx.consume_fuel(1);
            if !matches!(step, UiDocumentReconcileStep::Pending) || cx.is_cancelled() || cx.should_yield() {
                break step;
            }
        };
        if matches!(step, UiDocumentReconcileStep::Complete) {
            self.enqueue_layout(window_id);
        }
        step
    }

    pub fn close_document_step(&mut self, window_id: &str) -> bool {
        let Some(window) = self.windows.get_mut(window_id) else { return true };
        if !window.tree.close_document_binding_step() {
            return false;
        }
        window.document_reconcile = UiDocumentReconcileCursor::default();
        if let Some(ingress) = window.document_ingress.as_mut() {
            if !ingress.document.close_step() {
                return false;
            }
            window.document_ingress = None;
            return false;
        }
        if let Some(retiring) = window.retiring_document.as_mut() {
            if !retiring.close_step() {
                return false;
            }
            window.retiring_document = None;
            return false;
        }
        if let Some(document) = window.tree.take_document() {
            window.retiring_document = Some(document);
            return false;
        }
        true
    }
    //#endregion 📄️DocumentIngress

    /// 🚦️Changes a surface lane without duplicating its pending queue entry.
    pub fn set_surface_lane(&mut self, window_id: &str, lane: SurfaceLane) {
        let Some(window) = self.window_mut(window_id) else { return };
        if window.lane == lane {
            return;
        }
        window.lane = lane;
    }

    /// 🧵️Advances one surface layout by one cursor unit under the caller's fuel/deadline and
    /// cancellation context. Completed geometry publishes only after the whole job is consistent.
    pub fn step_layouts(&mut self, pool: &semio_framework_async::WorkerPool, atlas: &mut FontAtlas, cx: &mut StepContext<'_>) -> UiLayoutStep {
        let _ = atlas;
        if cx.should_yield() {
            return UiLayoutStep::Idle;
        }
        if self.drive_theme_propagation_one() {
            return UiLayoutStep::Idle;
        }
        if let Some(entry) = self.layout_pressure.take() {
            let lane = self.windows.get_token(entry.token).map_or(SurfaceLane::Background, |window| window.lane);
            if let Err(entry) = self.layout_queues[lane.index()].try_push(entry) {
                self.layout_pressure = Some(entry);
            }
            return UiLayoutStep::Idle;
        }
        let Some((token, window_id, lane, _reason)) = self.next_layout() else { return UiLayoutStep::Idle };
        let theme = self.theme;
        let Some(window) = self.windows.get_token_mut(token) else { return UiLayoutStep::Idle };
        window.queued = false;
        let Some(root) = window.tree.root else { return UiLayoutStep::Idle };
        if cx.is_cancelled() {
            if let Some(session) = window.layout_session.as_mut() {
                session.begin_close();
            }
            if let Some(job) = window.layout_job.as_mut() {
                job.begin_close();
            }
            window.layout_closing = window.layout_session.is_some() || window.layout_job.is_some();
            self.enqueue_layout(window_id.as_ref());
            return UiLayoutStep::Cancelled { window_id, lane };
        }
        if let Some(rejected) = window.layout_rejected.as_mut() {
            let _ = rejected.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
            if rejected.terminal_is_empty() {
                window.layout_rejected = None;
            }
            self.enqueue_layout(window_id.as_ref());
            return UiLayoutStep::Yielded { window_id, lane, stage: "Layout.CloseRejected", nodes: 1, glyphs: 0 };
        }
        if let Some(session) = window.layout_session.as_mut() {
            if session.generation().0 != window.layout_generation {
                session.begin_close();
                window.layout_closing = true;
            }
            if window.layout_closing {
                let _ = session.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
                if session.terminal_is_empty() {
                    window.layout_session = None;
                    window.layout_closing = false;
                }
                self.enqueue_layout(window_id.as_ref());
                return UiLayoutStep::Yielded { window_id, lane, stage: "Layout.CloseSession", nodes: 1, glyphs: 0 };
            }
            if session.poll() == semio_framework_job::WorkerJobPoll::CheckedOut {
                let terminal = session.checked_out_outcome().is_some_and(semio_framework_job::StepOutcome::is_terminal);
                let _ = session.take_checked_out_outcome();
                let identity = (token, window.layout_generation, window.revision, window.theme_revision, window.viewport_revision, window.viewport.0, window.viewport.1);
                let layout_preview = session.checked_out_job_mut().and_then(MountedLayoutJob::take_preview_one);
                let glyph_preview = session.checked_out_job_mut().and_then(|job| job.latest_glyph_preview());
                if let Some(preview) = layout_preview {
                    window.layout_preview = Some(preview);
                }
                if let Some(preview) = glyph_preview.filter(|preview| preview.generation == window.layout_generation && preview.revision == window.revision) {
                    window.glyph_preview = Some(preview);
                }
                let publish = session.checked_out_job_mut().filter(|job| job.stage() == LayoutJobStage::PublishResults).map(|job| job.publish_one(&mut window.tree, identity));
                if terminal || matches!(publish, Some(LayoutJobStep::Complete | LayoutJobStep::Fault(_))) || session.resume().is_err() {
                    session.begin_close();
                    window.layout_closing = true;
                }
                self.enqueue_layout(window_id.as_ref());
                return match publish {
                    Some(LayoutJobStep::Complete) => UiLayoutStep::Ready { window_id, lane },
                    Some(LayoutJobStep::Fault(_)) => UiLayoutStep::Cancelled { window_id, lane },
                    _ if terminal => UiLayoutStep::Cancelled { window_id, lane },
                    _ => UiLayoutStep::Yielded { window_id, lane, stage: "Layout.WorkerOutcome", nodes: usize::from(publish.is_some()), glyphs: 0 },
                };
            }
            let poll = session.pump_one(pool, worker_lane(lane));
            self.enqueue_layout(window_id.as_ref());
            return UiLayoutStep::Yielded {
                window_id,
                lane,
                stage: if matches!(poll, Ok(semio_framework_job::WorkerJobPoll::Outcome | semio_framework_job::WorkerJobPoll::Terminal)) { "Layout.WorkerTake" } else { "Layout.WorkerPool.UserVisible" },
                nodes: 0,
                glyphs: 0,
            };
        }
        if let Some(job) = window.layout_job.as_mut() {
            let identity = (token, window.layout_generation, window.revision, window.theme_revision, window.viewport_revision, window.viewport.0, window.viewport.1);
            if job.identity() != identity {
                job.begin_close();
                window.layout_closing = true;
            }
            if window.layout_closing {
                if job.close_one() && job.terminal_is_empty() {
                    window.layout_job = None;
                    window.layout_closing = false;
                }
                self.enqueue_layout(window_id.as_ref());
                return UiLayoutStep::Yielded { window_id, lane, stage: "Layout.CloseUnadmitted", nodes: 1, glyphs: 0 };
            }
            if job.is_admitted() {
                let Some(job) = window.layout_job.take() else {
                    self.enqueue_layout(window_id.as_ref());
                    return UiLayoutStep::Cancelled { window_id, lane };
                };
                let generation = window.layout_generation;
                let params = semio_framework_job::BatchJobParams {
                    operation: cx.operation(),
                    generation: semio_framework_job::Generation(generation),
                    cancel: cx.cancel_token(),
                    config: semio_framework_job::BatchDriveConfig { site: "ui.layout-text.worker", stage: semio_framework_job::InteractiveStage::UserVisibleSimStep, fuel_per_step: 1, step_budget_us: 1000 },
                    now_us: semio_framework_job::default_now_us,
                };
                match semio_framework_job::MountedWorkerJobSession::try_new(job, params) {
                    Ok(session) => window.layout_session = Some(session),
                    Err(rejected) => window.layout_rejected = Some(rejected),
                }
                self.enqueue_layout(window_id.as_ref());
                return UiLayoutStep::Yielded { window_id, lane, stage: "Layout.Mount", nodes: 0, glyphs: 0 };
            }
            let outcome = job.admit_one(&window.tree, cx);
            match outcome {
                LayoutJobStep::Yield { stage, nodes, glyphs } => {
                    self.enqueue_layout(window_id.as_ref());
                    return UiLayoutStep::Yielded { window_id, lane, stage: stage_label(stage), nodes, glyphs };
                }
                LayoutJobStep::Cancelled | LayoutJobStep::Fault(_) => {
                    job.begin_close();
                    window.layout_closing = true;
                    self.enqueue_layout(window_id.as_ref());
                    return UiLayoutStep::Cancelled { window_id, lane };
                }
                LayoutJobStep::Complete => {}
            }
        }
        window.layout_job = MountedLayoutJob::try_new(&window.tree, root, MountedLayoutIdentity { surface: token, generation: window.layout_generation, revision: window.revision, theme_revision: window.theme_revision, viewport_revision: window.viewport_revision }, theme, window.viewport.0, window.viewport.1).ok();
        if window.layout_job.is_some() {
            self.enqueue_layout(window_id.as_ref());
            UiLayoutStep::Yielded { window_id, lane, stage: "Layout.Preadmit", nodes: 0, glyphs: 0 }
        } else {
            UiLayoutStep::Ready { window_id, lane }
        }
    }

    fn drive_theme_propagation_one(&mut self) -> bool {
        let Some(mut cursor) = self.theme_propagation.take() else {
            let Some(theme) = self.pending_theme.take() else { return false };
            self.theme_propagation = Some(ThemePropagationCursor::new(theme));
            return true;
        };
        match cursor.phase {
            ThemePropagationPhase::Validate => {
                if cursor.slot == UI_LAYOUT_SURFACE_SLOTS {
                    cursor.slot = 0;
                    cursor.phase = ThemePropagationPhase::Apply;
                    self.theme_propagation = Some(cursor);
                    return true;
                }
                let token = self.windows.token_at(cursor.slot);
                if let Some(window) = token.and_then(|token| self.windows.get_token(token)) {
                    if window.layout_generation == u64::MAX || window.theme_revision == u64::MAX {
                        self.theme_fault = true;
                        self.theme_propagation = Some(cursor);
                        return true;
                    }
                }
                cursor.tokens[cursor.slot] = token;
                cursor.slot += 1;
                self.theme_propagation = Some(cursor);
                true
            }
            ThemePropagationPhase::Apply => {
                if cursor.slot == UI_LAYOUT_SURFACE_SLOTS {
                    cursor.phase = ThemePropagationPhase::Publish;
                    self.theme_propagation = Some(cursor);
                    return true;
                }
                let token = cursor.tokens[cursor.slot];
                cursor.slot += 1;
                if let Some(token) = token {
                    let Some(window) = self.windows.get_token_mut(token) else {
                        self.theme_propagation = Some(cursor);
                        return true;
                    };
                    let Some(layout_generation) = window.layout_generation.checked_add(1) else {
                        self.theme_fault = true;
                        self.theme_propagation = Some(cursor);
                        return true;
                    };
                    let Some(theme_revision) = window.theme_revision.checked_add(1) else {
                        self.theme_fault = true;
                        self.theme_propagation = Some(cursor);
                        return true;
                    };
                    window.layout_generation = layout_generation;
                    window.theme_revision = theme_revision;
                    if let Some(root) = window.tree.root {
                        window.tree.mark_dirty(root, NodeFlags::DIRTY_LAYOUT);
                    }
                    self.enqueue_layout_token(token, SurfaceLayoutReason::Theme);
                }
                self.theme_propagation = Some(cursor);
                true
            }
            ThemePropagationPhase::Publish => {
                self.theme = cursor.theme;
                self.theme_fault = false;
                if let Some(theme) = self.pending_theme.take() {
                    if theme_layout_identity(&self.theme) == theme_layout_identity(&theme) {
                        self.theme = theme;
                    } else {
                        self.theme_propagation = Some(ThemePropagationCursor::new(theme));
                    }
                }
                true
            }
        }
    }

    fn enqueue_layout(&mut self, window_id: &str) {
        self.enqueue_layout_reason(window_id, SurfaceLayoutReason::Dirty);
    }

    fn enqueue_layout_reason(&mut self, window_id: &str, reason: SurfaceLayoutReason) {
        let Some(token) = self.windows.token(window_id) else { return };
        self.enqueue_layout_token(token, reason);
    }

    fn enqueue_layout_token(&mut self, token: UiSurfaceToken, reason: SurfaceLayoutReason) {
        let Some(window) = self.windows.get_token_mut(token) else { return };
        if window.queued {
            return;
        }
        window.queued = true;
        let lane = window.lane;
        let entry = SurfaceLaneEntry { token, reason, epoch: window.layout_generation };
        if let Err(entry) = self.layout_queues[lane.index()].try_push(entry) {
            self.layout_pressure = Some(entry);
        }
    }

    fn next_layout(&mut self) -> Option<(UiSurfaceToken, SurfaceId, SurfaceLane, SurfaceLayoutReason)> {
        for _ in 0..LANE_WHEEL.len() {
            let lane = LANE_WHEEL[self.lane_cursor];
            self.lane_cursor = (self.lane_cursor + 1) % LANE_WHEEL.len();
            let Some(entry) = self.layout_queues[lane.index()].pop() else { continue };
            let Some(window) = self.windows.get_token(entry.token) else { continue };
            if window.layout_generation != entry.epoch {
                let current = SurfaceLaneEntry { epoch: window.layout_generation, ..entry };
                if let Err(current) = self.layout_queues[window.lane.index()].try_push(current) {
                    self.layout_pressure = Some(current);
                }
                continue;
            }
            if window.lane != lane {
                if let Err(entry) = self.layout_queues[window.lane.index()].try_push(entry) {
                    self.layout_pressure = Some(entry);
                }
                continue;
            }
            let id = self.windows.id(entry.token)?.clone();
            return Some((entry.token, id, lane, entry.reason));
        }
        None
    }

    pub fn set_window_kind_icons(&mut self, icons: HashMap<String, IconName>) {
        self.shell.set_window_kind_icons(icons);
    }

    /// 🪟️ Rebuilds the shared `🐚️Shell`'s retained dock/split/tab chrome from a declarative
    /// `WindowLayout` (independent of any window's `apply_tree`d content — see `shell`'s doc comment).
    pub fn set_window_layout(&mut self, layout: WindowLayout) {
        self.shell.set_window_layout(layout);
    }

    /// 🧭️ Forwards to `Shell::set_navbar` (stub — see that method's doc comment).
    pub fn set_navbar(&mut self, items: Vec<String>) {
        self.shell.set_navbar(items);
    }

    pub fn shell(&self) -> &Shell {
        &self.shell
    }

    /// 🚦️ True when any window's retained tree still carries `DIRTY_LAYOUT`/`DIRTY_PAINT`/
    /// `SUBTREE_DIRTY` on its root. No animation-clock scaffolding exists anywhere in this crate yet
    /// (nothing under `arena`/`tree`/`reconcile`/`flex`/`paint`/`events`/`scene_slots`/`shell`
    /// schedules a future wake), so this is purely dirty-flag-driven; wiring a real animation deadline
    /// is separate follow-up work, not this façade's job to invent.
    pub fn needs_frame(&self) -> bool {
        self.theme_propagation.is_some() || self.pending_theme.is_some() || self.windows.values().any(UiWindow::is_dirty)
    }

    /// 🖼️ The dirty-gated per-tick pipeline for `window_id`: `flex::LayoutEngine::compute` (itself a
    /// no-operation unless the root carries `DIRTY_LAYOUT`/`SUBTREE_DIRTY`) followed — only if that or the
    /// root's own `DIRTY_PAINT` fired — by `paint::paint_tree`, then handing every
    /// `scene_slots::collect_scene_slots` leaf to `scene_host`, when the caller passed one this tick.
    /// Returns `None` if `window_id` has no tree yet (`apply_tree` never called). A dirty window
    /// always repaints its whole tree — `paint::paint_tree`'s own doc comment: `DrawList` only
    /// supports a full clear-and-rebuild, no incremental dirty-subtree replacement yet.
    ///
    /// 🖋️ `atlas`/`icons` are the CALLER's own `FontAtlas`/`IconAtlas` — `Ui` never owns either (see
    /// this region's top-of-file doc comment): the host must pass the SAME instances it already
    /// `GpuContext::upload_font_atlas`/`upload_icon_atlas`s every frame, exactly like `flex::LayoutEngine::
    /// compute`/`paint::paint_tree` already receive them as parameters rather than fields. This lets
    /// retained-mode content share glyph/icon UVs with the rest of the host's chrome instead of
    /// clobbering (or never populating) a second, independent GPU texture.
    ///
    /// 🎬️ `scene_host` is a PER-FRAME parameter, not a stored field (there used to be a stored
    /// `Option<Box<dyn SceneHost>>` — removed): a caller-owned host typically needs to borrow the
    /// same per-frame state this call site already has in scope (a `GpuContext`, per-surface state
    /// maps, …), which a `Box<dyn SceneHost>` stored on `Ui` itself could never hold, exactly like
    /// `atlas`/`icons` above are parameters rather than fields for the same reason. `paint_tree`
    /// already knows (via `scene_host.is_some()`) whether to paint its own placeholder chrome for
    /// `ComponentScene`/`Image` leaves this tick or leave that rect for the host to fill in below —
    /// see `paint`'s own doc comment on that gate.
    // 🧬️ A former `Option<&mut dyn SceneHost>` — `SceneHost` is a genuine OPEN extension point (its own
    // doc comment: "the only place vello/world3d/raster-decode-specific code may live"; two real
    // implementors already exist, `RecordingSceneHost` here in tests and `FrameworkSceneHost` in
    // `os/renderer/engine/🟦️Interpreter`, which is outside this crate). Per R11 this is the trivially-
    // generic argument-position case (R11(a)): each call site already hands `frame` ONE concrete host
    // reference, so `H: SceneHost` loses no expressiveness versus `dyn` and every existing caller's
    // call syntax (`Some(&mut concrete_host)`) is unchanged — `H` is inferred from the argument.
    /// 🧵️ Advances one retained paint node, traversal scalar, scene child, publication swap, or
    /// retirement scalar for one mounted window.
    pub fn frame_step<H: SceneHost>(&mut self, window_id: &str, viewport_width: f32, viewport_height: f32, atlas: &mut FontAtlas, icons: Option<&IconAtlas>, mut scene_host: Option<&mut H>) -> UiFrameStep {
        self.set_viewport(window_id, viewport_width, viewport_height);
        let theme = self.theme;
        let Some(window) = self.windows.get_mut(window_id) else { return UiFrameStep::Missing };
        let Some(root) = window.tree.root else { return UiFrameStep::Missing };
        if let Some(retiring) = window.retiring_draw.as_mut() {
            if !retiring.retire_step() {
                return UiFrameStep::Pending;
            }
            window.retiring_draw = None;
            return UiFrameStep::Pending;
        }
        let layout_dirty = window.tree.node(root).is_some_and(|node| node.flags.contains(NodeFlags::DIRTY_LAYOUT) || node.flags.contains(NodeFlags::SUBTREE_DIRTY));
        if layout_dirty {
            return UiFrameStep::Pending;
        }
        if window.paint_frame.is_none() {
            let dirty = window.tree.node(root).is_some_and(|node| node.flags.contains(NodeFlags::DIRTY_PAINT));
            if !dirty {
                return UiFrameStep::Ready;
            }
            window.paint_frame = Some(RetainedPaintFrame {
                phase: RetainedPaintPhase::Synchronize,
                walk: RetainedPaintWalk::new(&window.tree, root),
                candidate: DrawList::default(),
                sync_node: None,
                node_sync: RetainedInteractiveSyncCursor::default(),
                paint_node: None,
                node_paint: RetainedNodePaintCursor::default(),
                scene_node: None,
                scene_paint: ScenePaintCursor::default(),
                revision: window.revision,
                theme_revision: window.theme_revision,
                viewport_revision: window.viewport_revision,
                baseline: UiFramePaintCensus::default(),
                fault_site: None,
            });
            return UiFrameStep::Pending;
        }
        let fresh = window.paint_frame.as_ref().is_some_and(|frame| frame.revision == window.revision && frame.theme_revision == window.theme_revision && frame.viewport_revision == window.viewport_revision);
        if !fresh {
            if !window.paint_frame.as_mut().is_some_and(|frame| frame.node_sync.close_step()) {
                return UiFrameStep::Pending;
            }
            let Some(frame) = window.paint_frame.take() else { return UiFrameStep::Fault };
            window.retiring_draw = Some(frame.candidate);
            return UiFrameStep::Pending;
        }
        let Some(frame) = window.paint_frame.as_mut() else { return UiFrameStep::Fault };
        if matches!(frame.phase, RetainedPaintPhase::Fault) {
            if !frame.node_sync.close_step() {
                return UiFrameStep::Pending;
            }
            frame.sync_node = None;
            return UiFrameStep::Fault;
        }
        if matches!(frame.phase, RetainedPaintPhase::Synchronize) {
            if let Some(node) = frame.sync_node {
                match sync_interactive_state_node_step(&mut window.tree, node, &theme, &mut frame.node_sync) {
                    RetainedInteractiveSyncStep::Pending => return UiFrameStep::Pending,
                    RetainedInteractiveSyncStep::Complete => {
                        frame.sync_node = None;
                        return UiFrameStep::Pending;
                    }
                    RetainedInteractiveSyncStep::Fault => {
                        frame.phase = RetainedPaintPhase::Fault;
                        return UiFrameStep::Pending;
                    }
                }
            }
        }
        if matches!(frame.phase, RetainedPaintPhase::Paint) {
            if let Some((node, origin_x, origin_y)) = frame.paint_node {
                match paint_node_step(&window.tree, node, origin_x, origin_y, &theme, atlas, icons, scene_host.is_some(), &mut frame.candidate, &mut frame.node_paint) {
                    RetainedNodePaintStep::Pending => return UiFrameStep::Pending,
                    RetainedNodePaintStep::Complete => {
                        frame.paint_node = None;
                        if let Some(node) = window.tree.node_mut(node) {
                            node.flags.set(NodeFlags::DIRTY_PAINT, false);
                        }
                        return UiFrameStep::Pending;
                    }
                    RetainedNodePaintStep::Fault => {
                        frame.phase = RetainedPaintPhase::Fault;
                        return UiFrameStep::Fault;
                    }
                }
            }
        }
        if matches!(frame.phase, RetainedPaintPhase::Scenes) {
            if let Some((node, origin_x, origin_y)) = frame.scene_node {
                let Some(host) = scene_host.as_deref_mut() else {
                    frame.phase = RetainedPaintPhase::Fault;
                    return UiFrameStep::Fault;
                };
                let Some(slot) = scene_slot_for_node(&window.tree, node, origin_x, origin_y) else {
                    frame.phase = RetainedPaintPhase::Fault;
                    return UiFrameStep::Fault;
                };
                match host.paint_slot_step(&slot, &mut frame.scene_paint, &mut frame.candidate, atlas, icons) {
                    ScenePaintStep::Pending => return UiFrameStep::Pending,
                    ScenePaintStep::Complete => {
                        frame.scene_node = None;
                        return UiFrameStep::Pending;
                    }
                    ScenePaintStep::Fault => {
                        frame.phase = RetainedPaintPhase::Fault;
                        return UiFrameStep::Fault;
                    }
                }
            }
        }
        match frame.phase {
            RetainedPaintPhase::Synchronize => match frame.walk.step(&window.tree) {
                RetainedPaintWalkStep::Visit(node, _, _) => {
                    frame.sync_node = Some(node);
                    UiFrameStep::Pending
                }
                RetainedPaintWalkStep::Scalar => UiFrameStep::Pending,
                RetainedPaintWalkStep::Complete => {
                    frame.phase = RetainedPaintPhase::Paint;
                    frame.walk = RetainedPaintWalk::new(&window.tree, root);
                    UiFrameStep::Pending
                }
                RetainedPaintWalkStep::DepthFault => {
                    frame.phase = RetainedPaintPhase::Fault;
                    UiFrameStep::Fault
                }
            },
            RetainedPaintPhase::Paint => match frame.walk.step(&window.tree) {
                RetainedPaintWalkStep::Visit(node, origin_x, origin_y) => {
                    frame.paint_node = Some((node, origin_x, origin_y));
                    UiFrameStep::Pending
                }
                RetainedPaintWalkStep::Scalar => UiFrameStep::Pending,
                RetainedPaintWalkStep::Complete => {
                    frame.phase = RetainedPaintPhase::Scenes;
                    frame.walk = RetainedPaintWalk::new(&window.tree, root);
                    UiFrameStep::Pending
                }
                RetainedPaintWalkStep::DepthFault => {
                    frame.phase = RetainedPaintPhase::Fault;
                    UiFrameStep::Fault
                }
            },
            RetainedPaintPhase::Scenes => match frame.walk.step(&window.tree) {
                RetainedPaintWalkStep::Visit(node, origin_x, origin_y) => {
                    if scene_host.is_some() && scene_slot_for_node(&window.tree, node, origin_x, origin_y).is_some() {
                        frame.scene_node = Some((node, origin_x, origin_y));
                    }
                    UiFrameStep::Pending
                }
                RetainedPaintWalkStep::Scalar => UiFrameStep::Pending,
                RetainedPaintWalkStep::Complete => {
                    frame.phase = RetainedPaintPhase::Publish;
                    UiFrameStep::Pending
                }
                RetainedPaintWalkStep::DepthFault => {
                    frame.phase = RetainedPaintPhase::Fault;
                    UiFrameStep::Fault
                }
            },
            RetainedPaintPhase::Publish => {
                std::mem::swap(&mut window.draw, &mut frame.candidate);
                window.retiring_draw = Some(std::mem::take(&mut frame.candidate));
                frame.phase = RetainedPaintPhase::Complete;
                window.paint_census = UiFramePaintCensus::of(&window.draw);
                UiFrameStep::Pending
            }
            RetainedPaintPhase::Complete => {
                window.paint_frame = None;
                UiFrameStep::Ready
            }
            RetainedPaintPhase::Fault => UiFrameStep::Fault,
        }
    }

    /// 🧱️ Advances one retained UI node directly into a caller-owned unpublished frame candidate.
    pub fn frame_into_step<H: SceneHost>(
        &mut self,
        window_id: &str,
        viewport: crate::wgpu::geometry::Rect,
        atlas: &mut FontAtlas,
        icons: Option<&IconAtlas>,
        mut scene_host: Option<&mut H>,
        target: &mut DrawList,
    ) -> UiFrameStep {
        let crate::wgpu::geometry::Rect { x: offset_x, y: offset_y, w: viewport_width, h: viewport_height } = viewport;
        self.set_viewport(window_id, viewport_width, viewport_height);
        let theme = self.theme;
        let Some(window) = self.windows.get_mut(window_id) else { return UiFrameStep::Missing };
        let Some(root) = window.tree.root else { return UiFrameStep::Missing };
        let layout_dirty = window.tree.node(root).is_some_and(|node| node.flags.contains(NodeFlags::DIRTY_LAYOUT) || node.flags.contains(NodeFlags::SUBTREE_DIRTY));
        if layout_dirty {
            return UiFrameStep::Pending;
        }
        if window.paint_frame.is_none() {
            window.paint_frame = Some(RetainedPaintFrame {
                phase: RetainedPaintPhase::Synchronize,
                walk: RetainedPaintWalk::new(&window.tree, root),
                candidate: DrawList::default(),
                sync_node: None,
                node_sync: RetainedInteractiveSyncCursor::default(),
                paint_node: None,
                node_paint: RetainedNodePaintCursor::default(),
                scene_node: None,
                scene_paint: ScenePaintCursor::default(),
                revision: window.revision,
                theme_revision: window.theme_revision,
                viewport_revision: window.viewport_revision,
                baseline: UiFramePaintCensus::of(target),
                fault_site: None,
            });
            return UiFrameStep::Pending;
        }
        let fresh = window.paint_frame.as_ref().is_some_and(|frame| frame.revision == window.revision && frame.theme_revision == window.theme_revision && frame.viewport_revision == window.viewport_revision);
        if !fresh {
            if !window.paint_frame.as_mut().is_some_and(|frame| frame.node_sync.close_step()) {
                return UiFrameStep::Pending;
            }
            let Some(frame) = window.paint_frame.take() else { return UiFrameStep::Fault };
            window.retiring_draw = Some(frame.candidate);
            return UiFrameStep::Pending;
        }
        let Some(frame) = window.paint_frame.as_mut() else { return UiFrameStep::Fault };
        if matches!(frame.phase, RetainedPaintPhase::Fault) {
            if !frame.node_sync.close_step() {
                return UiFrameStep::Pending;
            }
            frame.sync_node = None;
            return UiFrameStep::Fault;
        }
        if matches!(frame.phase, RetainedPaintPhase::Synchronize) {
            if let Some(node) = frame.sync_node {
                match sync_interactive_state_node_step(&mut window.tree, node, &theme, &mut frame.node_sync) {
                    RetainedInteractiveSyncStep::Pending => return UiFrameStep::Pending,
                    RetainedInteractiveSyncStep::Complete => {
                        frame.sync_node = None;
                        return UiFrameStep::Pending;
                    }
                    RetainedInteractiveSyncStep::Fault => {
                        frame.phase = RetainedPaintPhase::Fault;
                        frame.fault_site = Some("synchronize-node");
                        return UiFrameStep::Pending;
                    }
                }
            }
        }
        if matches!(frame.phase, RetainedPaintPhase::Paint) {
            if let Some((node, origin_x, origin_y)) = frame.paint_node {
                match paint_node_step(&window.tree, node, origin_x, origin_y, &theme, atlas, icons, scene_host.is_some(), target, &mut frame.node_paint) {
                    RetainedNodePaintStep::Pending => return UiFrameStep::Pending,
                    RetainedNodePaintStep::Complete => {
                        frame.paint_node = None;
                        if let Some(node) = window.tree.node_mut(node) {
                            node.flags.set(NodeFlags::DIRTY_PAINT, false);
                        }
                        return UiFrameStep::Pending;
                    }
                    RetainedNodePaintStep::Fault => {
                        frame.phase = RetainedPaintPhase::Fault;
                        frame.fault_site = Some("paint-node");
                        return UiFrameStep::Fault;
                    }
                }
            }
        }
        if matches!(frame.phase, RetainedPaintPhase::Scenes) {
            if let Some((node, origin_x, origin_y)) = frame.scene_node {
                let Some(host) = scene_host.as_deref_mut() else {
                    frame.phase = RetainedPaintPhase::Fault;
                    frame.fault_site = Some("scenes-no-host");
                    return UiFrameStep::Fault;
                };
                let Some(slot) = scene_slot_for_node(&window.tree, node, origin_x, origin_y) else {
                    frame.phase = RetainedPaintPhase::Fault;
                    frame.fault_site = Some("scenes-slot-missing");
                    return UiFrameStep::Fault;
                };
                match host.paint_slot_step(&slot, &mut frame.scene_paint, target, atlas, icons) {
                    ScenePaintStep::Pending => return UiFrameStep::Pending,
                    ScenePaintStep::Complete => {
                        frame.scene_node = None;
                        return UiFrameStep::Pending;
                    }
                    ScenePaintStep::Fault => {
                        frame.phase = RetainedPaintPhase::Fault;
                        frame.fault_site = Some("scenes-host");
                        return UiFrameStep::Fault;
                    }
                }
            }
        }
        match frame.phase {
            RetainedPaintPhase::Synchronize => match frame.walk.step(&window.tree) {
                RetainedPaintWalkStep::Visit(node, _, _) => {
                    frame.sync_node = Some(node);
                    UiFrameStep::Pending
                }
                RetainedPaintWalkStep::Scalar => UiFrameStep::Pending,
                RetainedPaintWalkStep::Complete => {
                    frame.phase = RetainedPaintPhase::Paint;
                    frame.walk = RetainedPaintWalk::new(&window.tree, root);
                    UiFrameStep::Pending
                }
                RetainedPaintWalkStep::DepthFault => {
                    frame.phase = RetainedPaintPhase::Fault;
                    frame.fault_site = Some("walk-depth-synchronize");
                    UiFrameStep::Fault
                }
            },
            RetainedPaintPhase::Paint => match frame.walk.step(&window.tree) {
                RetainedPaintWalkStep::Visit(node, origin_x, origin_y) => {
                    frame.paint_node = Some((node, origin_x + offset_x, origin_y + offset_y));
                    UiFrameStep::Pending
                }
                RetainedPaintWalkStep::Scalar => UiFrameStep::Pending,
                RetainedPaintWalkStep::Complete => {
                    frame.phase = RetainedPaintPhase::Scenes;
                    frame.walk = RetainedPaintWalk::new(&window.tree, root);
                    UiFrameStep::Pending
                }
                RetainedPaintWalkStep::DepthFault => {
                    frame.phase = RetainedPaintPhase::Fault;
                    frame.fault_site = Some("walk-depth-paint");
                    UiFrameStep::Fault
                }
            },
            RetainedPaintPhase::Scenes => match frame.walk.step(&window.tree) {
                RetainedPaintWalkStep::Visit(node, origin_x, origin_y) => {
                    let origin_x = origin_x + offset_x;
                    let origin_y = origin_y + offset_y;
                    if scene_host.is_some() && scene_slot_for_node(&window.tree, node, origin_x, origin_y).is_some() {
                        frame.scene_node = Some((node, origin_x, origin_y));
                    }
                    UiFrameStep::Pending
                }
                RetainedPaintWalkStep::Scalar => UiFrameStep::Pending,
                RetainedPaintWalkStep::Complete => {
                    frame.phase = RetainedPaintPhase::Publish;
                    UiFrameStep::Pending
                }
                RetainedPaintWalkStep::DepthFault => {
                    frame.phase = RetainedPaintPhase::Fault;
                    frame.fault_site = Some("walk-depth-scenes");
                    UiFrameStep::Fault
                }
            },
            RetainedPaintPhase::Publish => {
                frame.phase = RetainedPaintPhase::Complete;
                window.paint_census = UiFramePaintCensus::of(target).since(frame.baseline);
                UiFrameStep::Pending
            }
            RetainedPaintPhase::Complete => {
                window.paint_frame = None;
                UiFrameStep::Ready
            }
            RetainedPaintPhase::Fault => UiFrameStep::Fault,
        }
    }

    #[cfg(test)]
    pub fn frame<H: SceneHost>(&mut self, window_id: &str, viewport_width: f32, viewport_height: f32, atlas: &mut FontAtlas, icons: Option<&IconAtlas>, scene_host: Option<&mut H>) -> Option<&DrawList> {
        self.set_viewport(window_id, viewport_width, viewport_height);
        let window = self.windows.get_mut(window_id)?;
        let root = window.tree.root?;
        let layout_dirty = window.tree.node(root).is_some_and(|node| node.flags.contains(NodeFlags::DIRTY_LAYOUT) || node.flags.contains(NodeFlags::SUBTREE_DIRTY));
        if layout_dirty {
            return Some(&window.draw);
        }
        let dirty = window.tree.node(root).is_some_and(|node| node.flags.contains(NodeFlags::DIRTY_PAINT));
        if !dirty {
            return Some(&window.draw);
        }
        window.draw.clear();
        paint_tree(&mut window.tree, root, &self.theme, atlas, icons, scene_host.is_some(), &mut window.draw);
        if let Some(host) = scene_host {
            for slot in collect_scene_slots(&window.tree, root) {
                let mut cursor = ScenePaintCursor::default();
                while matches!(host.paint_slot_step(&slot, &mut cursor, &mut window.draw, atlas, icons), ScenePaintStep::Pending) {}
            }
        }
        Some(&window.draw)
    }

    /// 📤️ Direct access to `window_id`'s last-painted `DrawList` without re-running the pipeline.
    /// 🩺️ Which phase `window_id`'s in-flight retained paint frame is standing on — the only way a
    /// `UiFrameStep::Fault` can name where it came from without widening the step enum. `None` when no
    /// paint frame is in flight.
    /// 📐️ Whether THIS window's root still carries a layout obligation.
    ///
    /// ⚖️ `step_layouts` answers for the layout QUEUE, not for a window: `Idle` means the queue was
    /// empty and `Ready { window_id }` may name a different surface entirely. A caller that reads
    /// either as "my window is laid out" advances to paint against a dirty root, and
    /// `frame_into_step` then answers `Pending` for as long as the shell runs — the wgpu shell burned
    /// its whole 1 048 576-opportunity window-paint budget on `procedural-main` this way, twenty
    /// seconds per chrome walk (ticket 26/09/09/PROCEDURAL-3D-END-TO-END). This is the per-window
    /// predicate that question actually needs.
    pub fn layout_is_dirty(&self, window_id: &str) -> bool {
        let Some(window) = self.windows.get(window_id) else { return false };
        let Some(root) = window.tree.root else { return false };
        window.tree.node(root).is_some_and(|node| node.flags.contains(NodeFlags::DIRTY_LAYOUT) || node.flags.contains(NodeFlags::SUBTREE_DIRTY))
    }

    /// 📐️ Re-arms one window's layout lane. Idempotent — a window already queued is left alone — so a
    /// caller that finds [`Self::layout_is_dirty`] true can always make the obligation reachable
    /// again instead of waiting on a wake-up that was never posted.
    pub fn request_layout(&mut self, window_id: &str) {
        self.enqueue_layout(window_id);
    }

    /// 🩺️ Why one window's retained paint is not finishing, in the exact order `frame_into_step`
    /// decides it: a missing window, a rootless tree, a root the layout engine still calls dirty, a
    /// paint frame that has not been opened, or a frame whose revision stamps no longer match the
    /// window's (which retires the frame and starts over). A bare phase could not tell those apart —
    /// `paint_frame_phase` answers `None` for FOUR different stalls
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    pub fn paint_stall_census(&self, window_id: &str) -> String {
        let Some(window) = self.windows.get(window_id) else { return "window=absent".to_string() };
        let Some(root) = window.tree.root else { return "root=absent".to_string() };
        let dirty_layout = window.tree.node(root).is_some_and(|node| node.flags.contains(NodeFlags::DIRTY_LAYOUT));
        let subtree_dirty = window.tree.node(root).is_some_and(|node| node.flags.contains(NodeFlags::SUBTREE_DIRTY));
        let frame = window.paint_frame.as_ref();
        format!(
            "root=present dirty-layout={dirty_layout} subtree-dirty={subtree_dirty} frame={} revision={}/{} theme={}/{} viewport={}/{} phase={:?} fault-site={:?}",
            frame.is_some(),
            frame.map_or(0, |frame| frame.revision),
            window.revision,
            frame.map_or(0, |frame| frame.theme_revision),
            window.theme_revision,
            frame.map_or(0, |frame| frame.viewport_revision),
            window.viewport_revision,
            self.paint_frame_phase(window_id),
            frame.and_then(|frame| frame.fault_site),
        )
    }

    pub fn paint_frame_phase(&self, window_id: &str) -> Option<&'static str> {
        self.windows.get(window_id).and_then(|window| window.paint_frame.as_ref()).map(|frame| match (frame.fault_site, frame.phase) {
            (Some(site), _) => site,
            _ => match frame.phase {
            RetainedPaintPhase::Synchronize => "synchronize",
            RetainedPaintPhase::Paint => "paint",
            RetainedPaintPhase::Scenes => "scenes",
            RetainedPaintPhase::Publish => "publish",
            RetainedPaintPhase::Complete => "complete",
            RetainedPaintPhase::Fault => "fault",
            },
        })
    }

    /// 📊️ What `window_id`'s own retained paint contributed to the LAST frame it completed — the
    /// only honest answer for the production `frame_into_step` path, whose output lands in the
    /// caller's draw list and never in `draw_list` below. See [`UiFramePaintCensus`].
    pub fn paint_census(&self, window_id: &str) -> Option<UiFramePaintCensus> {
        self.windows.get(window_id).map(|window| window.paint_census)
    }

    pub fn draw_list(&self, window_id: &str) -> Option<&DrawList> {
        self.windows.get(window_id).map(|window| &window.draw)
    }

    /// 🕹️ Routes `event` through `window_id`'s `events::EventRouter` (hit-test, capture, focus, hover
    /// updates), returning the `UiCommand`s it produced and also queuing them for a later
    /// `drain_commands` call — callers may use either.
    #[allow(clippy::needless_pass_by_value, reason = "changing to &UiEvent is a breaking public API change across ~30 downstream plugins, out of T1 scope")]
    pub fn dispatch_event(&mut self, window_id: &str, event: UiEvent) -> Vec<UiCommand> {
        let Some(window) = self.windows.get_mut(window_id) else { return Vec::new() };
        let Some(root) = window.tree.root else { return Vec::new() };
        let commands = window.router.dispatch(&mut window.tree, root, &event);
        self.pending_commands.extend(commands.iter().cloned());
        commands
    }

    /// 🪟️ Routes `event` through the shared `🐚️Shell`'s own hit-testing, surfacing chrome-level
    /// `ShellEvent`s (tab activation today; drag/drop is `Shell::dispatch`'s own documented gap).
    pub fn dispatch_shell_event(&mut self, event: &UiEvent) -> Vec<ShellEvent> {
        self.shell.dispatch(event)
    }

    /// 📥️ Drains every `UiCommand` queued by `dispatch_event` calls since the last drain.
    pub fn drain_commands(&mut self) -> Vec<UiCommand> {
        std::mem::take(&mut self.pending_commands)
    }
}

impl Default for Ui {
    fn default() -> Self {
        Self::new()
    }
}
//#endregion 🔖️Ui

//#region 🔬️Introspection
/// 🔬️ Read-only accessors for the wgpu↔React parity structural-dump harness (see
/// `.🧬semio/🦑️repo/🎫️tickets/26/07/11/WGPU-RENDERER-FULL-PARITY` and `framework/renderer/wgpu`'s own
/// `🔬️Introspection` region, which is the actual JSON-building caller): exposes just enough of
/// `Ui`'s private `windows`/`theme` state for a caller to walk a window's retained `UiTree` (via
/// `UiTree::node`/`UiTree::children`, both already public) and know which theme it last painted
/// with. Purely additive and read-only — no new engine behavior, nothing here is called from
/// `apply_tree`/`frame`/`dispatch_event`'s own pipeline.
impl Ui {
    /// 🪟️ Every window id this façade currently tracks retained state for (`HashMap` iteration
    /// order — not insertion order; a caller needing a deterministic pick must sort/filter itself).
    pub fn window_ids(&self) -> impl Iterator<Item = &str> {
        self.windows.ids().map(AsRef::as_ref)
    }

    /// 📐️ `window_id`'s last `set_viewport`/`frame` viewport, if that window has any retained state.
    pub fn viewport(&self, window_id: &str) -> Option<(f32, f32)> {
        self.windows.get(window_id).map(|window| window.viewport)
    }

    /// 🌲️ Read-only access to `window_id`'s retained tree (root + `Node` arena) for a caller to walk.
    pub fn tree(&self, window_id: &str) -> Option<&UiTree> {
        self.windows.get(window_id).map(|window| &window.tree)
    }

    /// 🧬️ Returns the retained tree identity revision used to reject stale interactive intents.
    pub fn tree_revision(&self, window_id: &str) -> Option<u64> {
        self.windows.get(window_id).map(|window| window.revision)
    }

    #[cfg(test)]
    pub(crate) fn progressive_layout_preview(&self, window_id: &str) -> Option<MountedLayoutResult> {
        self.windows.get(window_id).and_then(|window| window.layout_preview)
    }

    #[cfg(test)]
    pub(crate) fn progressive_glyph_preview(&self, window_id: &str) -> Option<RetainedGlyphPreview> {
        self.windows.get(window_id).and_then(|window| window.glyph_preview)
    }

    /// 🎨️ The theme this façade last painted every window with (`Theme` is `Copy`).
    pub fn theme(&self) -> Theme {
        self.theme
    }

    /// 🎯️ Whether `window_id`'s retained content currently has a focused node — `false` if that
    /// window has no retained state at all. Lets a host (`w2-input-wiring`,
    /// `.🧬semio/🦑️repo/🎫️tickets/26/07/11/WGPU-RENDERER-FULL-PARITY/report-w2-input-wiring.md`) decide whether real
    /// keyboard/IME events belong to this window's content (route via `dispatch_event`) or should
    /// fall back to chrome-level shortcuts. Forwards to `EventRouter::is_focused`, itself added this
    /// same pass — both purely additive reads, no change to `dispatch_event`'s own focus logic.
    pub fn window_has_focus(&self, window_id: &str) -> bool {
        self.windows.get(window_id).is_some_and(|window| window.router.is_focused())
    }
}
//#endregion 🔬️Introspection

#[cfg(test)]
#[path = "../../../🧪️tests/🔬️targets-wgpu-engine-unit/🦀️.rs"]
mod tests;
//#region 🧪️RetainedDocumentHostileFixtures
#[cfg(test)]
#[path = "../../../🧪️tests/🔬️targets-wgpu-engine-retained-document-hostile-fixtures/🦀️.rs"]
mod retained_document_hostile_fixtures;
//#endregion 🧪️RetainedDocumentHostileFixtures
// #endregion engine
