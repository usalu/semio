// #region engine
//! 🧵️ The retained-mode `Ui` façade: the missing keystone tying `arena`/`tree`/`reconcile`/`flex`/
//! `paint`/`events`/`scene_slots`/`shell` into one usable pipeline — each of those regions was built
//! and individually tested to its own milestone but nothing ever assembled them together, and nothing
//! in `framework/renderer/wgpu` calls into any of it (see `.🦑️repo/🎫️tickets/26/07/11/RETAINED-MODE-UI-CRATE`'s
//! plan for the historical intent). This module is purely additive: the immediate-mode `widgets`
//! path stays the only pipeline actually driving pixels until a later workstream proves this façade
//! out (via the golden `tests` module below) and cuts over.

use std::collections::HashMap;

use crate::wgpu::component::layout::WindowLayout;
#[cfg(test)]
use crate::wgpu::component::ui::UiNode;
use crate::wgpu::draw::{DrawList, IconAtlas};
use crate::wgpu::events::{EventRouter, UiCommand, UiEvent};
use crate::wgpu::flex::{LayoutJobStage, LayoutJobStep};
use crate::wgpu::mounted_layout::{MountedLayoutJob, MountedLayoutResult, RetainedGlyphPreview};
use crate::wgpu::paint::{paint_node_step, paint_tree, sync_interactive_state_node_step, RetainedInteractiveSyncCursor, RetainedInteractiveSyncStep, RetainedNodePaintCursor, RetainedNodePaintStep};
use crate::wgpu::scene_slots::{collect_scene_slots, scene_slot_for_node, SceneHost, ScenePaintCursor, ScenePaintStep};
use crate::wgpu::shell::{Shell, ShellEvent};
use crate::wgpu::text::FontAtlas;
use crate::wgpu::theme::Theme;
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
    paint_frame: Option<RetainedPaintFrame>,
    retiring_draw: Option<DrawList>,
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
            paint_frame: None,
            retiring_draw: None,
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

struct UiSurfaceRegistry {
    slots: [Option<UiSurfaceSlot>; UI_LAYOUT_SURFACE_SLOTS],
    generations: [u64; UI_LAYOUT_SURFACE_SLOTS],
}

impl Default for UiSurfaceRegistry {
    fn default() -> Self {
        Self { slots: std::array::from_fn(|_| None), generations: [0; UI_LAYOUT_SURFACE_SLOTS] }
    }
}

impl UiSurfaceRegistry {
    fn token(&self, id: &str) -> Option<UiSurfaceToken> {
        let slot = self.slots.iter().position(|slot| slot.as_ref().is_some_and(|slot| slot.id.as_ref() == id))?;
        Some(UiSurfaceToken { slot: slot as u8, generation: self.slots[slot].as_ref()?.generation })
    }

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

    fn values_mut(&mut self) -> impl Iterator<Item = &mut UiWindow> {
        self.slots.iter_mut().filter_map(|slot| slot.as_mut().map(|slot| &mut slot.window))
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
        LayoutJobStage::PruneRemoved => "Layout.PruneRemoved",
        LayoutJobStage::SyncNodes => "Layout.SyncNodes",
        LayoutJobStage::SolveLayout => "Layout.SolveLayout",
        LayoutJobStage::MeasureFallback => "Layout.MeasureFallback",
        LayoutJobStage::ArrangeFallback => "Layout.ArrangeFallback",
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
/// (app-content trees); window-chrome (dock/split/tab) is the separate `Shell` this façade also owns,
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
    #[cfg(test)]
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

    pub fn close_document_step(&mut self, window_id: &str) -> bool {
        let Some(window) = self.windows.get_mut(window_id) else { return true };
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
    pub fn step_layouts(&mut self, pool: &semio_framework_async::WorkerPool, atlas: &mut FontAtlas, cx: &mut semio_framework_job::StepContext<'_>) -> UiLayoutStep {
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
                if terminal || matches!(publish, Some(LayoutJobStep::Complete | LayoutJobStep::Fault(_))) {
                    session.begin_close();
                    window.layout_closing = true;
                } else if session.resume().is_err() {
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
                    config: semio_framework_job::BatchDriveConfig { site: "ui.layout-text.worker", stage: semio_framework_job::InteractiveStage::UserVisibleSimStep, fuel_per_step: 1, step_budget_ms: 1 },
                    now_ms: semio_framework_job::default_now_ms,
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
        window.layout_job = MountedLayoutJob::try_new(&window.tree, root, token, window.layout_generation, window.revision, window.theme_revision, window.viewport_revision, theme, window.viewport.0, window.viewport.1).ok();
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

    /// 🪟️ Rebuilds the shared `Shell`'s retained dock/split/tab chrome from a declarative
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
    // `os/renderer/engine/Interpreter`, which is outside this crate). Per R11 this is the trivially-
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
        viewport_width: f32,
        viewport_height: f32,
        offset_x: f32,
        offset_y: f32,
        atlas: &mut FontAtlas,
        icons: Option<&IconAtlas>,
        mut scene_host: Option<&mut H>,
        target: &mut DrawList,
    ) -> UiFrameStep {
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
                match host.paint_slot_step(&slot, &mut frame.scene_paint, target, atlas, icons) {
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
                    UiFrameStep::Fault
                }
            },
            RetainedPaintPhase::Publish => {
                frame.phase = RetainedPaintPhase::Complete;
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

    /// 🪟️ Routes `event` through the shared `Shell`'s own hit-testing, surfacing chrome-level
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
/// `.🦑️repo/🎫️tickets/26/07/11/WGPU-RENDERER-FULL-PARITY` and `framework/renderer/wgpu`'s own
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

    pub(crate) fn progressive_layout_preview(&self, window_id: &str) -> Option<MountedLayoutResult> {
        self.windows.get(window_id).and_then(|window| window.layout_preview)
    }

    pub(crate) fn progressive_glyph_preview(&self, window_id: &str) -> Option<RetainedGlyphPreview> {
        self.windows.get(window_id).and_then(|window| window.glyph_preview)
    }

    /// 🎨️ The theme this façade last painted every window with (`Theme` is `Copy`).
    pub fn theme(&self) -> Theme {
        self.theme
    }

    /// 🎯️ Whether `window_id`'s retained content currently has a focused node — `false` if that
    /// window has no retained state at all. Lets a host (`w2-input-wiring`,
    /// `.🦑️repo/🎫️tickets/26/07/11/WGPU-RENDERER-FULL-PARITY/report-w2-input-wiring.md`) decide whether real
    /// keyboard/IME events belong to this window's content (route via `dispatch_event`) or should
    /// fall back to chrome-level shortcuts. Forwards to `EventRouter::is_focused`, itself added this
    /// same pass — both purely additive reads, no change to `dispatch_event`'s own focus logic.
    pub fn window_has_focus(&self, window_id: &str) -> bool {
        self.windows.get(window_id).is_some_and(|window| window.router.is_focused())
    }
}
//#endregion 🔬️Introspection

#[cfg(test)]
mod tests {
    use super::*;
    use crate::wgpu::component::layout::ActionDescriptor;
    use crate::wgpu::component::ui::{
        ui_node_to_control, SurfaceKind, UiButtonNode, UiComponentSceneNode, UiControlNode, UiExternalSlotNode, UiFieldNode, UiGroupNode, UiIconSelectNode, UiImageNode, UiInputNode, UiKeyValueEntry, UiKeyValueNode, UiNumberStepperNode, UiPresence,
        UiRingNode, UiSectionNode, UiSelectItem, UiSelectNode, UiSeparatorNode, UiSliderNode, UiStackNode, UiState, UiTextNode, UiToggleNode, UiTreeActionPlacement, UiTreeItemAction, UiTreeItemNode, UiTreeNode, UiTreeSectionNode,
    };
    use crate::wgpu::events::PointerButton;
    use crate::wgpu::geometry::Rect;
    use crate::wgpu::input::InputState;
    use crate::wgpu::scene_slots::SceneSlot;
    use crate::wgpu::widgets::{
        draw_text_on, draw_text_overlay_on, measure_widget, render_scroll_region, render_widget, wrap_text, ControlNode, InputMeta, KeyValueEntry, RingMeta, SelectItem, SliderMeta, StepperMeta, TreeItem, TreeItemAction, TreeSection, WidgetContext,
        WidgetInteractionMaps, WidgetNode,
    };
    use crate::wgpu::Label;
    use std::collections::HashMap as StdHashMap;

    //#region 🔖️FacadeTests
    fn stack_ui(children: Vec<UiNode>) -> UiNode {
        UiNode::Stack(UiStackNode { direction: "vertical".into(), gap: None, padding: None, id: Some("root".into()), presence: UiPresence::default(), activate: None, drop_action: None, drop_overlay: None, children, menu: None })
    }

    fn action() -> ActionDescriptor {
        ActionDescriptor { controller_id: "ctrl".into(), action: "go".into(), args: None }
    }

    fn button_ui(id: &str, label: &str) -> UiNode {
        UiNode::Button(UiButtonNode { id: Some(id.into()), icon_id: IconName::CircleDot, label: Label::data(label), action: action(), style: None, presence: UiPresence::default(), menu: None })
    }

    fn test_clock() -> u64 {
        0
    }

    fn test_layout_pool() -> semio_framework_async::WorkerPool {
        semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1))
    }

    fn retained_walk_leaf(discriminant: u32, ordinal: u32, value: &str) -> crate::wgpu::tree::Node {
        crate::wgpu::tree::Node::new(
            crate::wgpu::tree::NodeKey::Positional(discriminant, ordinal),
            crate::wgpu::tree::WidgetSpec(UiNode::Text(UiTextNode { value: Label::data(value), emphasize: None, data_attributes: None, presence: UiPresence::default(), menu: None })),
        )
    }

    fn drive_layout(ui: &mut Ui, window_id: &str, width: f32, height: f32, atlas: &mut FontAtlas) {
        ui.set_viewport(window_id, width, height);
        let operation = semio_framework_job::allocate_operation_id();
        let cancel = semio_framework_job::CancelToken::root_now();
        let pool = test_layout_pool();
        let mut preview_sequence = 0;
        loop {
            let mut cx = semio_framework_job::StepContext::new(operation, semio_framework_job::Generation(0), semio_framework_job::StepBudget::new(1, u64::MAX), cancel.clone(), test_clock, &mut preview_sequence);
            if matches!(ui.step_layouts(&pool, atlas, &mut cx), UiLayoutStep::Idle) {
                break;
            }
        }
    }

    #[test]
    fn apply_tree_then_frame_produces_a_non_empty_draw_list() {
        let mut ui = Ui::new();
        let mut atlas = FontAtlas::builtin();
        ui.apply_tree("main", &stack_ui(vec![UiNode::Text(UiTextNode { value: Label::data("hi"), emphasize: None, data_attributes: None, presence: UiPresence::default(), menu: None })]));

        assert!(ui.needs_frame(), "a freshly applied tree must report needing a frame");
        drive_layout(&mut ui, "main", 400.0, 400.0, &mut atlas);
        let draw = ui.frame::<RecordingSceneHost>("main", 400.0, 400.0, &mut atlas, None, None).expect("frame must produce a draw list once a tree was applied");
        let total: usize = draw.layers.iter().map(|layer| layer.ui_instances.len()).sum();
        assert!(total > 0, "expected the text node to emit at least one glyph instance");
    }

    #[test]
    fn frame_before_any_apply_tree_returns_none() {
        let mut ui = Ui::new();
        let mut atlas = FontAtlas::builtin();
        assert!(ui.frame::<RecordingSceneHost>("nonexistent", 400.0, 400.0, &mut atlas, None, None).is_none());
    }

    #[test]
    fn retained_paint_walk_yields_one_node_or_scalar_per_step_in_tree_order() {
        let mut tree = UiTree::new();
        let root = tree.insert_child(None, retained_walk_leaf(0, 0, "root"));
        let first = tree.insert_child(Some(root), retained_walk_leaf(1, 0, "first"));
        let second = tree.insert_child(Some(root), retained_walk_leaf(1, 1, "second"));
        let nested = tree.insert_child(Some(first), retained_walk_leaf(2, 0, "nested"));
        let expected = [root, first, nested, second];
        let mut observed = [None; 4];
        let mut observed_len = 0;
        let mut walk = RetainedPaintWalk::new(&tree, root);
        let mut complete = false;
        for _ in 0..16 {
            match walk.step(&tree) {
                RetainedPaintWalkStep::Visit(node, _, _) => {
                    observed[observed_len] = Some(node);
                    observed_len += 1;
                }
                RetainedPaintWalkStep::Scalar => {}
                RetainedPaintWalkStep::Complete => {
                    complete = true;
                    break;
                }
                RetainedPaintWalkStep::DepthFault => panic!("bounded tree should not exhaust retained depth credits"),
            }
        }
        assert!(complete);
        assert_eq!(observed, expected.map(Some));
    }

    #[test]
    fn retained_paint_walk_depth_cap_plus_one_faults_without_dynamic_spill() {
        let mut tree = UiTree::new();
        let root = tree.insert_child(None, retained_walk_leaf(0, 0, "root"));
        let mut parent = root;
        for ordinal in 1..=RETAINED_PAINT_DEPTH_CREDITS {
            let ordinal = match u32::try_from(ordinal) {
                Ok(ordinal) => ordinal,
                Err(_) => panic!("retained depth credit must fit a node ordinal"),
            };
            parent = tree.insert_child(Some(parent), retained_walk_leaf(1, ordinal, "child"));
        }
        let mut walk = RetainedPaintWalk::new(&tree, root);
        let mut faulted = false;
        for _ in 0..(RETAINED_PAINT_DEPTH_CREDITS * 3) {
            if matches!(walk.step(&tree), RetainedPaintWalkStep::DepthFault) {
                faulted = true;
                break;
            }
        }
        assert!(faulted);
    }

    #[test]
    fn needs_frame_is_false_once_a_stable_tree_has_been_framed() {
        let mut ui = Ui::new();
        let mut atlas = FontAtlas::builtin();
        let ui_node = stack_ui(vec![UiNode::Text(UiTextNode { value: Label::data("hi"), emphasize: None, data_attributes: None, presence: UiPresence::default(), menu: None })]);
        ui.apply_tree("main", &ui_node);
        drive_layout(&mut ui, "main", 400.0, 400.0, &mut atlas);
        ui.frame::<RecordingSceneHost>("main", 400.0, 400.0, &mut atlas, None, None);
        assert!(!ui.needs_frame(), "nothing changed since the last frame, so no frame should be needed");

        ui.apply_tree("main", &ui_node);
        assert!(!ui.needs_frame(), "re-applying an identical tree must set zero dirty flags (reconcile's own golden rule)");
    }

    #[test]
    fn dispatch_event_emits_a_button_click_command_and_it_is_also_drainable() {
        let mut ui = Ui::new();
        let mut atlas = FontAtlas::builtin();
        ui.apply_tree("main", &stack_ui(vec![button_ui("go", "Go")]));
        drive_layout(&mut ui, "main", 400.0, 400.0, &mut atlas);
        ui.frame::<RecordingSceneHost>("main", 400.0, 400.0, &mut atlas, None, None);

        ui.dispatch_event("main", UiEvent::PointerDown { x: 10.0, y: 10.0, button: PointerButton::Primary });
        let commands = ui.dispatch_event("main", UiEvent::PointerUp { x: 10.0, y: 10.0, button: PointerButton::Primary });

        assert!(commands.iter().any(|cmd| matches!(cmd, UiCommand::App { action: fired_action, .. } if *fired_action == action())));
        let drained = ui.drain_commands();
        assert!(!drained.is_empty(), "commands dispatched should also be queryable via drain_commands");
        assert!(ui.drain_commands().is_empty(), "a second drain with nothing new dispatched must be empty");
    }

    #[test]
    fn set_window_layout_wires_into_the_facades_shell() {
        let mut ui = Ui::new();
        ui.set_window_layout(crate::wgpu::even_window_layout(&["app.viewport".to_string()]));
        assert!(ui.shell().window_layout().is_some());
    }

    #[test]
    fn resize_storm_coalesces_to_one_latest_surface_job() {
        let mut ui = Ui::new();
        let mut atlas = FontAtlas::builtin();
        ui.apply_tree("resize", &stack_ui(vec![button_ui("go", "Go")]));
        for width in 1..=2_000 {
            ui.set_viewport("resize", width as f32, 480.0);
        }
        assert_eq!(ui.layout_queues.iter().map(SurfaceLaneRing::len).sum::<usize>(), 1, "a resize storm must retain one coalesced surface entry");

        drive_layout(&mut ui, "resize", 2_000.0, 480.0, &mut atlas);
        let root = ui.tree("resize").and_then(|tree| tree.root).expect("root");
        assert_eq!(ui.tree("resize").and_then(|tree| tree.accepted_layout(root)).expect("accepted root").width, 2_000.0);
    }

    #[test]
    fn interactive_storm_does_not_starve_background_surface_lane() {
        let mut ui = Ui::new();
        let mut atlas = FontAtlas::builtin();
        ui.apply_tree("interactive", &stack_ui(vec![button_ui("go", "Go")]));
        ui.apply_tree("background", &stack_ui(vec![button_ui("bg", "Background")]));
        ui.set_surface_lane("interactive", SurfaceLane::Interactive);
        ui.set_surface_lane("background", SurfaceLane::Background);
        ui.set_viewport("interactive", 400.0, 400.0);
        ui.set_viewport("background", 400.0, 400.0);

        let operation = semio_framework_job::allocate_operation_id();
        let cancel = semio_framework_job::CancelToken::root_now();
        let pool = test_layout_pool();
        let mut preview_sequence = 0;
        let mut background_progress_at = None;
        for slice in 0..12 {
            ui.set_viewport("interactive", 401.0 + slice as f32, 400.0);
            let mut cx = semio_framework_job::StepContext::new(operation, semio_framework_job::Generation(0), semio_framework_job::StepBudget::new(1, u64::MAX), cancel.clone(), test_clock, &mut preview_sequence);
            let step = ui.step_layouts(&pool, &mut atlas, &mut cx);
            if matches!(step, UiLayoutStep::Yielded { ref window_id, .. } | UiLayoutStep::Ready { ref window_id, .. } if window_id.as_ref() == "background") {
                background_progress_at = Some(slice);
                break;
            }
        }
        assert!(background_progress_at.is_some_and(|slice| slice < LANE_WHEEL.len()), "the weighted wheel must service background within one six-slot cycle");
    }

    #[test]
    fn large_layout_and_shaping_job_keeps_every_observed_slice_below_eight_ms() {
        let mut ui = Ui::new();
        let mut atlas = FontAtlas::builtin();
        let children = (0..1_024).map(|index| UiNode::Text(UiTextNode { value: Label::data(format!("node-{index}")), emphasize: None, data_attributes: None, presence: UiPresence::default(), menu: None })).collect();
        ui.apply_tree("large", &stack_ui(children));
        ui.set_viewport("large", 1_920.0, 1_080.0);

        let operation = semio_framework_job::allocate_operation_id();
        let cancel = semio_framework_job::CancelToken::root_now();
        let pool = test_layout_pool();
        let mut preview_sequence = 0;
        let mut slices = 0;
        let mut max_slice = std::time::Duration::ZERO;
        loop {
            let mut cx = semio_framework_job::StepContext::new(operation, semio_framework_job::Generation(0), semio_framework_job::StepBudget::new(1, u64::MAX), cancel.clone(), test_clock, &mut preview_sequence);
            let started = std::time::Instant::now();
            let step = ui.step_layouts(&pool, &mut atlas, &mut cx);
            max_slice = max_slice.max(started.elapsed());
            slices += 1;
            if matches!(step, UiLayoutStep::Idle) {
                break;
            }
        }
        let tree = ui.tree("large").expect("large tree");
        let root = tree.root.expect("large root");
        let children: Vec<_> = tree.children(root).collect();
        assert!(slices > 10_000, "the 1,025-node/text workload must be observably chunked, got {slices} slices");
        assert!(max_slice < std::time::Duration::from_millis(8), "largest observed layout slice was {max_slice:?}");
        assert_eq!(tree.accepted_layout(root).expect("accepted root").width, 1_920.0);
        assert!(tree.accepted_layout(*children.last().expect("last child")).expect("accepted last child").y > tree.accepted_layout(children[0]).expect("accepted first child").y);
    }

    #[test]
    fn mounted_layout_surface_max_plus_one_returns_exact_owner_without_mutation() {
        let mut ui = Ui::new();
        for index in 0..UI_LAYOUT_SURFACE_SLOTS {
            let id = SurfaceId::try_from(format!("mounted-{index}")).unwrap_or_else(|_| panic!("bounded mounted surface"));
            assert!(ui.windows.try_admit(id).is_ok());
        }
        let before: UiFixedList<SurfaceId, UI_LAYOUT_SURFACE_SLOTS> = ui.windows.ids().cloned().fold(UiFixedList::default(), |mut ids, id| {
            assert_eq!(ids.try_push(id), Ok(()));
            ids
        });
        let owner = SurfaceId::try_from("mounted-max-plus-one").unwrap_or_else(|_| panic!("bounded rejected surface"));
        let rejected = ui.windows.try_admit(owner.clone()).unwrap_err();
        assert_eq!(rejected.id, owner);
        assert_eq!(ui.windows.ids().cloned().collect::<Vec<_>>(), before.iter().cloned().collect::<Vec<_>>());
    }

    #[test]
    fn mounted_layout_equal_theme_does_not_invalidate_or_requeue() {
        let mut ui = Ui::new();
        ui.apply_tree("theme", &stack_ui(vec![button_ui("same", "Same")]));
        let before_generation = ui.windows.get("theme").map(|window| window.layout_generation);
        let before_theme_revision = ui.windows.get("theme").map(|window| window.theme_revision);
        let before_queue = ui.layout_queues.iter().map(SurfaceLaneRing::len).sum::<usize>();
        ui.set_theme(ui.theme());
        assert_eq!(ui.windows.get("theme").map(|window| window.layout_generation), before_generation);
        assert_eq!(ui.windows.get("theme").map(|window| window.theme_revision), before_theme_revision);
        assert_eq!(ui.layout_queues.iter().map(SurfaceLaneRing::len).sum::<usize>(), before_queue);
    }

    #[test]
    fn changed_theme_propagates_one_fixed_surface_slot_per_opportunity() {
        let mut ui = Ui::new();
        ui.apply_tree("theme-a", &stack_ui(vec![button_ui("a", "A")]));
        ui.apply_tree("theme-b", &stack_ui(vec![button_ui("b", "B")]));
        let before_a = ui.windows.get("theme-a").map(|window| window.theme_revision);
        let before_b = ui.windows.get("theme-b").map(|window| window.theme_revision);
        let mut changed = ui.theme();
        changed.gap_standard += 1.0;
        ui.set_theme(changed);
        assert_eq!(ui.windows.get("theme-a").map(|window| window.theme_revision), before_a);
        assert_eq!(ui.windows.get("theme-b").map(|window| window.theme_revision), before_b);
        assert!(ui.drive_theme_propagation_one());
        assert!(ui.theme_propagation.as_ref().is_some_and(|cursor| cursor.phase == ThemePropagationPhase::Validate && cursor.slot == 1));
        assert_eq!(ui.windows.get("theme-a").map(|window| window.theme_revision), before_a);
        assert_eq!(ui.windows.get("theme-b").map(|window| window.theme_revision), before_b);
    }

    #[test]
    fn mounted_layout_atomic_snapshot_keeps_last_valid_geometry_until_fresh_swap() {
        let mut ui = Ui::new();
        let mut atlas = FontAtlas::builtin();
        ui.apply_tree("atomic", &stack_ui(vec![UiNode::Text(UiTextNode { value: Label::data("atomic"), emphasize: None, data_attributes: None, presence: UiPresence::default(), menu: None }), button_ui("target", "Target")]));
        drive_layout(&mut ui, "atomic", 320.0, 200.0, &mut atlas);
        let tree = ui.tree("atomic").unwrap_or_else(|| panic!("atomic tree"));
        let root = tree.root.unwrap_or_else(|| panic!("atomic root"));
        let old_generation = tree.accepted_layout_generation();
        let old_layout = tree.accepted_layout(root).unwrap_or_default();
        ui.set_viewport("atomic", 960.0, 540.0);
        let pool = test_layout_pool();
        let cancel = semio_framework_job::CancelToken::root_now();
        let operation = semio_framework_job::allocate_operation_id();
        let mut preview_sequence = 0;
        let mut swaps = 0;
        for _ in 0..100_000 {
            let before = ui.tree("atomic").map(UiTree::accepted_layout_generation).unwrap_or_default();
            let mut cx = semio_framework_job::StepContext::new(operation, semio_framework_job::Generation(0), semio_framework_job::StepBudget::new(1, u64::MAX), cancel.clone(), test_clock, &mut preview_sequence);
            let step = ui.step_layouts(&pool, &mut atlas, &mut cx);
            let tree = ui.tree("atomic").unwrap_or_else(|| panic!("atomic retained tree"));
            let after = tree.accepted_layout_generation();
            swaps += usize::from(before != after);
            if matches!(step, UiLayoutStep::Ready { .. }) {
                assert_eq!(tree.accepted_layout(root).unwrap_or_default().width, 960.0);
                break;
            }
            assert_eq!(after, old_generation);
            assert_eq!(tree.accepted_layout(root).unwrap_or_default(), old_layout);
        }
        assert_eq!(swaps, 1);
        assert!(ui.progressive_layout_preview("atomic").is_some());
        assert!(ui.progressive_glyph_preview("atomic").is_some());
    }

    #[test]
    fn mounted_layout_revision_max_refuses_theme_tree_and_viewport_without_alias() {
        let mut ui = Ui::new();
        let original = stack_ui(vec![button_ui("original", "Original")]);
        ui.apply_tree("max", &original);
        let original_theme = ui.theme();
        if let Some(window) = ui.windows.get_mut("max") {
            window.theme_revision = u64::MAX;
        }
        let mut changed_theme = original_theme;
        changed_theme.gap_standard += 1.0;
        ui.set_theme(changed_theme);
        assert_eq!(theme_layout_identity(&ui.theme()), theme_layout_identity(&original_theme));
        if let Some(window) = ui.windows.get_mut("max") {
            window.theme_revision = 1;
            window.viewport_revision = u64::MAX;
            window.layout_generation = u64::MAX - 1;
        }
        let before_viewport = ui.viewport("max");
        ui.set_viewport("max", 777.0, 333.0);
        assert_eq!(ui.viewport("max"), before_viewport);
        if let Some(window) = ui.windows.get_mut("max") {
            window.viewport_revision = 1;
            window.layout_generation = 7;
            window.revision = u64::MAX;
        }
        ui.apply_tree("max", &stack_ui(vec![button_ui("changed", "Changed")]));
        assert_eq!(ui.tree_revision("max"), Some(u64::MAX));
        assert!(ui.tree("max").and_then(|tree| tree.root).and_then(|root| ui.tree("max").and_then(|tree| tree.node(root))).is_some_and(|node| node.spec.0 == original));
    }

    #[test]
    fn mounted_layout_replay_and_resize_supersede_are_deterministic() {
        let mut ui = Ui::new();
        let mut atlas = FontAtlas::builtin();
        let input = stack_ui(vec![button_ui("replay", "Replay")]);
        ui.apply_tree("replay", &input);
        drive_layout(&mut ui, "replay", 640.0, 480.0, &mut atlas);
        let root = ui.tree("replay").and_then(|tree| tree.root).unwrap_or_else(|| panic!("replay root"));
        let first = ui.tree("replay").and_then(|tree| tree.accepted_layout(root)).unwrap_or_default();
        ui.set_viewport("replay", 641.0, 480.0);
        ui.set_viewport("replay", 640.0, 480.0);
        drive_layout(&mut ui, "replay", 640.0, 480.0, &mut atlas);
        let second = ui.tree("replay").and_then(|tree| tree.accepted_layout(root)).unwrap_or_default();
        assert_eq!(first, second);
        assert_eq!(second.width, 640.0);
    }
    //#endregion 🔖️FacadeTests

    //#region 🔖️SceneHostTests
    fn component_scene_ui(surface_id: &str) -> UiNode {
        UiNode::ComponentScene(UiComponentSceneNode {
            surface_id: surface_id.into(),
            controller_id: "ctrl".into(),
            component_kind: SurfaceKind::World3d,
            pane_id: None,
            binding_id: None,
            presence: UiPresence::default(),
            canvas_2d: None,
            world_3d: None,
            node_graph: None,
            text_editor: None,
            table: None,
            paint_2d: None,
            virtual_file_system: None,
            tiled_map: None,
            board2d: None,
            icon_render: None,
            ink_canvas: None,
            graph_timeline: None,
            block_list: None,
            diff_view: None,
            event_feed: None,
            menu: None,
        })
    }

    /// 🎬️ A bare-bones `SceneHost` recording every call it receives, so tests can assert `Ui::frame`
    /// actually reaches the host (once per slot, with the right payload) instead of just trusting the
    /// wiring compiles. Paints a single filled rect per slot so a hosted frame's `DrawList` is
    /// distinguishable from an unpainted one.
    struct RecordingSceneHost {
        paint_calls: usize,
        last_surface_id: Option<String>,
    }

    impl SceneHost for RecordingSceneHost {
        fn paint_slot_step(&mut self, slot: &SceneSlot<'_>, cursor: &mut ScenePaintCursor, draw: &mut DrawList, _atlas: &mut FontAtlas, _icons: Option<&IconAtlas>) -> ScenePaintStep {
            match cursor.bind(slot.node) {
                Ok(true) => {}
                Ok(false) => return ScenePaintStep::Pending,
                Err(()) => return ScenePaintStep::Fault,
            }
            self.paint_calls += 1;
            self.last_surface_id = slot.surface().map(|(surface_id, _)| surface_id.to_string());
            draw.push_rounded([slot.rect.x, slot.rect.y, slot.rect.w, slot.rect.h], Theme::default().accent, 0.0);
            cursor.finish()
        }
    }

    #[test]
    fn frame_with_no_scene_host_falls_back_to_the_placeholder_chrome() {
        let mut ui = Ui::new();
        let mut atlas = FontAtlas::builtin();
        ui.apply_tree("w", &stack_ui(vec![component_scene_ui("surface.no-host")]));
        drive_layout(&mut ui, "w", 400.0, 400.0, &mut atlas);
        let draw = ui.frame::<RecordingSceneHost>("w", 400.0, 400.0, &mut atlas, None, None).expect("frame must produce a draw list");
        let instances: usize = draw.layers.iter().map(|layer| layer.ui_instances.len()).sum();
        assert!(instances > 0, "with no scene host registered, paint_component_scene's placeholder chrome should still paint");
    }

    #[test]
    fn frame_with_a_scene_host_routes_the_component_scene_leaf_through_it() {
        let mut ui = Ui::new();
        let mut atlas = FontAtlas::builtin();
        ui.apply_tree("w", &stack_ui(vec![component_scene_ui("surface.host-test")]));
        drive_layout(&mut ui, "w", 400.0, 400.0, &mut atlas);

        let mut host = RecordingSceneHost { paint_calls: 0, last_surface_id: None };
        let draw = ui.frame("w", 400.0, 400.0, &mut atlas, None, Some(&mut host)).expect("frame must produce a draw list even with a scene host registered");
        let instances: usize = draw.layers.iter().map(|layer| layer.ui_instances.len()).sum();

        assert_eq!(host.paint_calls, 1, "the host should be invoked exactly once for the single ComponentScene leaf");
        assert_eq!(host.last_surface_id.as_deref(), Some("surface.host-test"));
        assert!(instances > 0, "the host's own draw call should still land in the frame's DrawList");
    }

    #[test]
    fn frame_with_a_scene_host_still_paints_ancestor_chrome_around_the_hosted_slot() {
        // 🌳️ Nests the ComponentScene under a Group (not just a bare Stack) — regression for the
        // shadow-walk gap this bridge replaces: `collect_scene_slots` must still find it, and the
        // Group's own header/frame chrome (unrelated to the scene leaf) must still paint normally.
        let mut ui = Ui::new();
        let mut atlas = FontAtlas::builtin();
        let group_node = UiNode::Group(UiGroupNode {
            id: "group".into(),
            label: Label::data("Group"),
            default_open: None,
            presence: UiPresence::default(),
            children: vec![UiNode::Text(UiTextNode { value: Label::data("label"), emphasize: None, data_attributes: None, presence: UiPresence::default(), menu: None }), component_scene_ui("surface.nested")],
            menu: None,
        });
        ui.apply_tree("w", &group_node);
        drive_layout(&mut ui, "w", 400.0, 400.0, &mut atlas);

        let mut host = RecordingSceneHost { paint_calls: 0, last_surface_id: None };
        ui.frame("w", 400.0, 400.0, &mut atlas, None, Some(&mut host)).expect("frame must produce a draw list");

        assert_eq!(host.paint_calls, 1);
        assert_eq!(host.last_surface_id.as_deref(), Some("surface.nested"));
    }
    //#endregion 🔖️SceneHostTests

    //#region 🔖️GoldenHarness
    /// 🏆️ Acceptance gate for this workstream: for a curated fixture of every `UiNode` variant, runs
    /// the retained façade (`apply_tree` + `frame`) and the immediate-mode path (`render_widget` over
    /// a hand-converted `WidgetNode`) and asserts they emit structurally equivalent `DrawList`s
    /// (same instance/vector/raster counts — not bit-identical geometry, per this ticket's brief).
    /// `to_widget_node`/`control_to_widget`/`tree_*_to_widget` below mirror
    /// `framework/renderer/wgpu/rs/lib.rs`'s private `ui_node_to_widget` conversion; they're
    /// duplicated here (test-only) rather than shared because that crate depends on `ui_wgpu`, never
    /// the reverse — keeping the two in sync is this harness's job.
    fn to_widget_node(node: &UiNode) -> WidgetNode<ActionDescriptor> {
        match node {
            UiNode::Stack(stack) => WidgetNode::Stack { direction: stack.direction.clone(), gap: stack.gap.clone(), padding: stack.padding.clone(), children: stack.children.iter().map(to_widget_node).collect() },
            UiNode::Text(text) => WidgetNode::Text { value: text.value.to_string(), emphasize: text.emphasize.unwrap_or(false) },
            UiNode::Separator(_) => WidgetNode::Separator,
            UiNode::Button(button) => WidgetNode::Button { id: button.id.clone(), icon_id: Some(button.icon_id.clone()), label: button.label.to_string(), event: Some(button.action.clone()) },
            UiNode::Input(input) => WidgetNode::Input {
                id: input.id.clone(),
                input_kind: input.input_kind.clone(),
                value: input.value.clone(),
                placeholder: input.placeholder.clone().map(|l| l.to_string()),
                commit: input.commit.clone(),
                on_change: Some(input.on_change.clone()),
            },
            UiNode::Select(select) => WidgetNode::Select {
                id: select.id.clone(),
                value: select.value.clone(),
                items: select.items.iter().map(|item| SelectItem { value: item.value.clone(), label: item.label.to_string() }).collect(),
                placeholder: select.placeholder.clone().map(|l| l.to_string()),
                on_change: Some(select.on_change.clone()),
            },
            UiNode::Toggle(toggle) => WidgetNode::Toggle { id: toggle.id.clone(), icon_id: toggle.icon_id.clone(), pressed: toggle.presence.selected, text: toggle.text.clone().map(|l| l.to_string()), on_change: Some(toggle.on_change.clone()) },
            UiNode::KeyValue(kv) => WidgetNode::KeyValue { entries: kv.entries.iter().map(|entry| KeyValueEntry { label: entry.label.to_string(), value: entry.value.clone() }).collect() },
            UiNode::Slider(slider) => WidgetNode::Slider { id: slider.id.clone(), value: slider.value, min: slider.min, max: slider.max, step: slider.step, ready: None, disabled: false, on_change: Some(slider.on_change.clone()) },
            UiNode::NumberStepper(stepper) => {
                WidgetNode::NumberStepper { id: stepper.id.clone(), value: stepper.value, step: stepper.step, uniform: stepper.uniform, on_absolute: Some(stepper.on_absolute.clone()), on_delta: Some(stepper.on_delta.clone()) }
            }
            UiNode::Ring(ring) => WidgetNode::Ring { id: ring.id.clone(), t: ring.t, disabled: ring.presence.state == UiState::Disabled, on_change: Some(ring.on_change.clone()) },
            UiNode::IconSelect(select) => WidgetNode::IconSelect { id: select.id.clone(), value: select.value.clone(), uniform: select.uniform, classifier_kind: select.classifier_kind.clone(), on_change: Some(select.on_change.clone()) },
            UiNode::Field(field) => match ui_node_to_control(&field.child) {
                Some(control) => WidgetNode::Field { id: field.id.clone(), label: field.label.to_string(), child: control_to_widget(&control) },
                None => WidgetNode::Section { id: field.id.clone(), label: Some(field.label.to_string()), default_open: true, children: vec![to_widget_node(&field.child)] },
            },
            UiNode::Section(section) => {
                WidgetNode::Section { id: section.id.clone(), label: section.label.clone().map(|l| l.to_string()), default_open: section.default_open.unwrap_or(true), children: section.children.iter().map(to_widget_node).collect() }
            }
            UiNode::Group(group) => WidgetNode::Section { id: group.id.clone(), label: Some(group.label.to_string()), default_open: group.default_open.unwrap_or(true), children: group.children.iter().map(to_widget_node).collect() },
            UiNode::Tree(tree) => WidgetNode::Tree {
                // 🧭️ Per-item `selected`/`highlighted` (see `tree_item_to_widget`) already carry the
                // full signal from `item.presence` — the tree-level id lists are gone, not re-derived.
                sections: tree.sections.iter().map(tree_section_to_widget).collect(),
                selected_ids: Vec::new(),
                highlighted_ids: Vec::new(),
                // 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM W3a: `UiTreeNode.selectionChange`
                // is deleted — selection now flows through `interactionSelect`/`interaction_domain`, not yet
                // wired into this retained-mode engine.
                selection_change: None,
            },
            // KNOWN GAP: `WidgetNode<E>` (the immediate-mode `widgets` region's tree type) has no
            // Image/ComponentScene/ExternalSlot variant at all — the renderer's own
            // `ui_node_to_widget` collapses all three to an empty placeholder `Text` node, which
            // isn't a like-for-like rendering of the same node. There is no immediate-mode output to
            // compare the retained `paint::paint_image`/`paint_component_scene`/`paint_external_slot`
            // against; see the golden tests below for these three, which verify the retained side
            // alone produces sane output and skip the two-pipeline equivalence assertion.
            UiNode::Image(_) | UiNode::ComponentScene(_) | UiNode::ExternalSlot(_) => WidgetNode::Text { value: String::new(), emphasize: false },
        }
    }

    fn control_to_widget(control: &UiControlNode) -> ControlNode<ActionDescriptor> {
        match control {
            UiControlNode::Button(n) => ControlNode::Button { id: n.id.clone(), icon_id: Some(n.icon_id.clone()), label: n.label.to_string(), event: Some(n.action.clone()) },
            UiControlNode::Input(n) => {
                ControlNode::Input { id: n.id.clone(), input_kind: n.input_kind.clone(), value: n.value.clone(), placeholder: n.placeholder.clone().map(|l| l.to_string()), commit: n.commit.clone(), on_change: Some(n.on_change.clone()) }
            }
            UiControlNode::Select(n) => ControlNode::Select {
                id: n.id.clone(),
                value: n.value.clone(),
                items: n.items.iter().map(|item| SelectItem { value: item.value.clone(), label: item.label.to_string() }).collect(),
                placeholder: n.placeholder.clone().map(|l| l.to_string()),
                on_change: Some(n.on_change.clone()),
            },
            UiControlNode::Toggle(n) => ControlNode::Toggle { id: n.id.clone(), icon_id: n.icon_id.clone(), pressed: n.presence.selected, text: n.text.clone().map(|l| l.to_string()), on_change: Some(n.on_change.clone()) },
            UiControlNode::KeyValue(n) => ControlNode::KeyValue { entries: n.entries.iter().map(|entry| KeyValueEntry { label: entry.label.to_string(), value: entry.value.clone() }).collect() },
            UiControlNode::Slider(n) => ControlNode::Slider { id: n.id.clone(), value: n.value, min: n.min, max: n.max, step: n.step, ready: None, disabled: false, on_change: Some(n.on_change.clone()) },
            UiControlNode::NumberStepper(n) => ControlNode::NumberStepper { id: n.id.clone(), value: n.value, step: n.step, uniform: n.uniform, on_absolute: Some(n.on_absolute.clone()), on_delta: Some(n.on_delta.clone()) },
            UiControlNode::Ring(n) => ControlNode::Ring { id: n.id.clone(), t: n.t, disabled: n.presence.state == UiState::Disabled, on_change: Some(n.on_change.clone()) },
            UiControlNode::IconSelect(n) => ControlNode::IconSelect { id: n.id.clone(), value: n.value.clone(), uniform: n.uniform, classifier_kind: n.classifier_kind.clone(), on_change: Some(n.on_change.clone()) },
        }
    }

    /// 🎛️ Same per-variant field mapping as `control_to_widget`, but into a `WidgetNode` instead of a
    /// `ControlNode` — needed for `TreeItem::control: Option<Box<WidgetNode<E>>>`, which (unlike
    /// `Field`'s `child: ControlNode<E>`) embeds a full widget, not a bare control payload.
    fn control_to_widget_node(control: &UiControlNode) -> WidgetNode<ActionDescriptor> {
        match control {
            UiControlNode::Button(n) => WidgetNode::Button { id: n.id.clone(), icon_id: Some(n.icon_id.clone()), label: n.label.to_string(), event: Some(n.action.clone()) },
            UiControlNode::Input(n) => {
                WidgetNode::Input { id: n.id.clone(), input_kind: n.input_kind.clone(), value: n.value.clone(), placeholder: n.placeholder.clone().map(|l| l.to_string()), commit: n.commit.clone(), on_change: Some(n.on_change.clone()) }
            }
            UiControlNode::Select(n) => WidgetNode::Select {
                id: n.id.clone(),
                value: n.value.clone(),
                items: n.items.iter().map(|item| SelectItem { value: item.value.clone(), label: item.label.to_string() }).collect(),
                placeholder: n.placeholder.clone().map(|l| l.to_string()),
                on_change: Some(n.on_change.clone()),
            },
            UiControlNode::Toggle(n) => WidgetNode::Toggle { id: n.id.clone(), icon_id: n.icon_id.clone(), pressed: n.presence.selected, text: n.text.clone().map(|l| l.to_string()), on_change: Some(n.on_change.clone()) },
            UiControlNode::KeyValue(n) => WidgetNode::KeyValue { entries: n.entries.iter().map(|entry| KeyValueEntry { label: entry.label.to_string(), value: entry.value.clone() }).collect() },
            UiControlNode::Slider(n) => WidgetNode::Slider { id: n.id.clone(), value: n.value, min: n.min, max: n.max, step: n.step, ready: None, disabled: false, on_change: Some(n.on_change.clone()) },
            UiControlNode::NumberStepper(n) => WidgetNode::NumberStepper { id: n.id.clone(), value: n.value, step: n.step, uniform: n.uniform, on_absolute: Some(n.on_absolute.clone()), on_delta: Some(n.on_delta.clone()) },
            UiControlNode::Ring(n) => WidgetNode::Ring { id: n.id.clone(), t: n.t, disabled: n.presence.state == UiState::Disabled, on_change: Some(n.on_change.clone()) },
            UiControlNode::IconSelect(n) => WidgetNode::IconSelect { id: n.id.clone(), value: n.value.clone(), uniform: n.uniform, classifier_kind: n.classifier_kind.clone(), on_change: Some(n.on_change.clone()) },
        }
    }

    fn tree_action_to_widget(action: &UiTreeItemAction) -> TreeItemAction<ActionDescriptor> {
        TreeItemAction { icon_id: action.icon_id.clone(), label: action.label.clone().map(|l| l.to_string()), event: action.action.clone(), placement: action.placement() }
    }

    fn tree_item_to_widget(item: &UiTreeItemNode) -> TreeItem<ActionDescriptor> {
        TreeItem {
            id: item.id.clone(),
            label: item.label.to_string(),
            description: item.description.clone(),
            icon_id: item.icon_id.clone(),
            selected: item.presence.selected,
            highlighted: item.presence.state == UiState::Previewed,
            default_open: item.default_open.unwrap_or(false),
            dimmed: item.dimmed.unwrap_or(false),
            event: item.action.clone(),
            // 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM W3a: `UiTreeItemNode.hoverAction`/
            // `unhoverAction` are deleted — hover is now framework-owned per `UiTreeNode.interactionDomain`.
            hover_event: None,
            unhover_event: None,
            actions: item.actions.as_ref().map(|actions| actions.iter().map(tree_action_to_widget).collect()).unwrap_or_default(),
            draggable: item.draggable.unwrap_or(false),
            drag_data: item.drag_data.clone().unwrap_or_default(),
            control: item.control.as_ref().map(|control| Box::new(control_to_widget_node(control))),
            children: item.items.as_ref().map(|items| items.iter().map(tree_item_to_widget).collect()).unwrap_or_default(),
        }
    }

    fn tree_section_to_widget(section: &UiTreeSectionNode) -> TreeSection<ActionDescriptor> {
        TreeSection { id: section.id.clone(), label: section.label.clone().map(|l| l.to_string()), default_open: section.default_open.unwrap_or(true), items: section.items.iter().map(tree_item_to_widget).collect() }
    }

    /// 📊️ Total (ui_instances incl. overlay, vector_vertices incl. overlay, raster_instances) across
    /// every layer of a `DrawList` — the "structurally equivalent" signal this harness compares,
    /// deliberately coarser than exact geometry per this ticket's tolerance allowance.
    fn stats(draw: &DrawList) -> (usize, usize, usize) {
        let instances = draw.layers.iter().map(|layer| layer.ui_instances.len() + layer.overlay_ui_instances.len()).sum();
        let vectors = draw.layers.iter().map(|layer| layer.vector_vertices.len() + layer.overlay_vector_vertices.len()).sum();
        let raster = draw.layers.iter().map(|layer| layer.raster_instances.len()).sum();
        (instances, vectors, raster)
    }

    fn retained_stats(node: &UiNode) -> (usize, usize, usize) {
        let mut ui = Ui::new();
        let mut atlas = FontAtlas::builtin();
        ui.apply_tree("golden", node);
        drive_layout(&mut ui, "golden", 400.0, 400.0, &mut atlas);
        let draw = ui.frame::<RecordingSceneHost>("golden", 400.0, 400.0, &mut atlas, None, None).expect("apply_tree then frame must produce a draw list");
        stats(draw)
    }

    fn immediate_stats(node: &UiNode, bounds: Rect) -> (usize, usize, usize) {
        let widget = to_widget_node(node);
        let mut draw = DrawList::default();
        let mut atlas = FontAtlas::builtin();
        let theme = Theme::default();
        let mut input = InputState::<ActionDescriptor>::default();
        let mut scroll_offsets: StdHashMap<String, f32> = StdHashMap::new();
        let mut collapsed_sections: StdHashMap<String, bool> = StdHashMap::new();
        let mut open_selects: StdHashMap<String, bool> = StdHashMap::new();
        let mut ctx = WidgetContext {
            draw: &mut draw,
            overlay: None,
            atlas: &mut atlas,
            icons: None,
            input: &mut input,
            theme: &theme,
            scroll_offsets: &mut scroll_offsets,
            collapsed_sections: &mut collapsed_sections,
            open_selects: &mut open_selects,
            interaction_maps: None,
            pick_clip: None,
        };
        render_widget(&widget, bounds, &mut ctx);
        stats(&draw)
    }

    const VIEWPORT: Rect = Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 };

    /// 🧱️ Wraps a leaf `UiNode` as the sole child of a gap-less/padding-less vertical `Stack`: on the
    /// retained side, `flex::LayoutEngine` always forces the *root* to the full viewport size
    /// (`compute`'s `root_style.size` override) and gives a `Stack`'s only child `flex_grow: 1.0`, so
    /// the child's resolved `LayoutBucket` is exactly the full viewport. On the immediate side,
    /// `layout::layout_vertical`/`layout_horizontal`'s `extra_per_child` gives a lone child the same
    /// full bounds. Wrapping every leaf fixture this way guarantees both pipelines paint it at
    /// identical bounds, which is what makes an exact instance/vector-count comparison meaningful
    /// instead of an artifact of divergent layout math.
    fn leaf(child: UiNode) -> UiNode {
        UiNode::Stack(UiStackNode {
            direction: "vertical".into(),
            gap: Some("none".into()),
            padding: Some("none".into()),
            id: None,
            presence: UiPresence::default(),
            activate: None,
            drop_action: None,
            drop_overlay: None,
            children: vec![child],
            menu: None,
        })
    }

    fn assert_equivalent(kind: &str, node: &UiNode) {
        let retained = retained_stats(node);
        let immediate = immediate_stats(node, VIEWPORT);
        assert_eq!(retained, immediate, "{kind}: retained (instances, vectors, raster) {retained:?} != immediate {immediate:?}");
    }

    // `action()` is shared with 🔖️FacadeTests above — both sub-regions live in the same `mod tests`.

    #[test]
    fn golden_stack() {
        let node = UiNode::Stack(UiStackNode {
            direction: "vertical".into(),
            gap: Some("none".into()),
            padding: Some("none".into()),
            id: None,
            presence: UiPresence::default(),
            activate: None,
            drop_action: None,
            drop_overlay: None,
            children: vec![
                UiNode::Text(UiTextNode { value: Label::data("hello"), emphasize: None, data_attributes: None, presence: UiPresence::default(), menu: None }),
                UiNode::Separator(UiSeparatorNode { presence: UiPresence::default(), menu: None }),
            ],
            menu: None,
        });
        assert_equivalent("Stack", &node);
    }

    #[test]
    fn golden_text() {
        assert_equivalent("Text", &leaf(UiNode::Text(UiTextNode { value: Label::data("hello world"), emphasize: Some(true), data_attributes: None, presence: UiPresence::default(), menu: None })));
    }

    #[test]
    fn golden_button() {
        assert_equivalent("Button", &leaf(UiNode::Button(UiButtonNode { id: Some("btn".into()), icon_id: IconName::CircleDot, label: Label::data("Go"), action: action(), style: None, presence: UiPresence::default(), menu: None })));
    }

    #[test]
    fn golden_separator() {
        assert_equivalent("Separator", &leaf(UiNode::Separator(UiSeparatorNode { presence: UiPresence::default(), menu: None })));
    }

    #[test]
    fn golden_input() {
        assert_equivalent(
            "Input",
            &leaf(UiNode::Input(UiInputNode {
                id: "in".into(),
                input_kind: "text".into(),
                value: "abc".into(),
                placeholder: None,
                commit: None,
                min: None,
                max: None,
                step: None,
                accept: None,
                on_change: action(),
                presence: UiPresence::default(),
                menu: None,
            })),
        );
    }

    #[test]
    fn golden_select() {
        assert_equivalent(
            "Select",
            &leaf(UiNode::Select(UiSelectNode {
                id: "sel".into(),
                value: "a".into(),
                items: vec![UiSelectItem { value: "a".into(), label: Label::data("Alpha") }, UiSelectItem { value: "b".into(), label: Label::data("Beta") }],
                placeholder: None,
                on_change: action(),
                presence: UiPresence::default(),
                menu: None,
            })),
        );
    }

    #[test]
    fn golden_toggle() {
        // 🚫️ `presence.selected` is intentionally NOT exercised here: the shared `presence_overlay`
        // now draws an outset accent ring for ANY selected element (see
        // `selected_presence_draws_an_outset_ring_on_any_element`, below) — a deliberate new
        // capability `widgets::render_toggle` (the immediate-mode reference this harness compares
        // against) never had, so a selected fixture would fail this equivalence check for the wrong
        // reason. This test stays scoped to the base (unselected) toggle's fill/label parity.
        assert_equivalent("Toggle", &leaf(UiNode::Toggle(UiToggleNode { id: "tog".into(), icon_id: IconName::CircleDot, text: Some(Label::data("On")), on_change: action(), presence: UiPresence::default(), menu: None })));
    }

    /// ✨️ `presence.selected` draws its outset accent ring universally — proven here on `Toggle`, a
    /// non-`Stack` variant, since `selected` used to be a `UiStackNode`-only field. Instance count
    /// grows vs. the unselected fixture (the extra `push_chrome_border` edges), confirming the ring
    /// is now a shared channel every element gets for free from `presence_overlay`.
    #[test]
    fn selected_presence_draws_an_outset_ring_on_any_element() {
        let unselected = UiNode::Toggle(UiToggleNode { id: "tog".into(), icon_id: IconName::CircleDot, text: Some(Label::data("On")), on_change: action(), presence: UiPresence::default(), menu: None });
        let selected = UiNode::Toggle(UiToggleNode { id: "tog".into(), icon_id: IconName::CircleDot, text: Some(Label::data("On")), on_change: action(), presence: UiPresence::selected(true), menu: None });
        let (unselected_instances, _, _) = retained_stats(&leaf(unselected));
        let (selected_instances, _, _) = retained_stats(&leaf(selected));
        assert!(selected_instances > unselected_instances, "a selected element should paint more instances than an unselected one (the outset accent ring)");
    }

    #[test]
    fn golden_key_value() {
        assert_equivalent("KeyValue", &leaf(UiNode::KeyValue(UiKeyValueNode { entries: vec![UiKeyValueEntry { label: Label::data("Name"), value: Label::data("Semio").to_string() }], presence: UiPresence::default(), menu: None })));
    }

    #[test]
    fn golden_slider() {
        assert_equivalent("Slider", &leaf(UiNode::Slider(UiSliderNode { id: "sl".into(), value: 0.5, min: 0.0, max: 1.0, step: 0.01, unit: None, on_change: action(), presence: UiPresence::default(), menu: None })));
    }

    /// KNOWN GAP: `widgets::render_number_stepper` renders its center value segment via a full
    /// `render_input` call (which itself calls `push_control_border` — a background fill plus 4
    /// border-edge quads, 5 instances), giving the center value its own nested input-style border box.
    /// `paint::paint_number_stepper` instead just `draw_text_on`s the formatted value directly with no
    /// surrounding border. Confirmed by running this fixture: retained emits 14 instances (one
    /// `push_control_border` for the whole control + 2 divider lines + 3 text runs), immediate emits
    /// 19 (the same 14 plus the center value's own nested 5-instance border box) — a real, reproducible
    /// paint-logic difference, not a fixture/harness artifact. This is real follow-up work for `paint`
    /// (either add the nested border to `paint_number_stepper`, or confirm the immediate path's nested
    /// border is unintentional and should be dropped there instead — a product decision outside this
    /// façade's scope), not something to paper over here.
    #[test]
    fn golden_number_stepper_known_gap() {
        let (instances, _, _) = retained_stats(&leaf(UiNode::NumberStepper(UiNumberStepperNode { id: "ns".into(), value: 2.0, step: 1.0, uniform: false, on_absolute: action(), on_delta: action(), presence: UiPresence::default(), menu: None })));
        assert!(instances > 0, "NumberStepper should paint its minus/value/plus segments");
    }

    /// 🔒️ Added by `w1c-paint-parity` (see `.🦑️repo/🎫️tickets/26/07/11/WGPU-RENDERER-FULL-PARITY/report-w1c-paint-parity.md`):
    /// `paint::paint_number_stepper` now ports `widgets::render_number_stepper`'s nested
    /// center-value border box (the exact gap `golden_number_stepper_known_gap`'s doc comment
    /// above documents), closing the 14-vs-19-instance divergence for the `uniform: true` case.
    /// Left `golden_number_stepper_known_gap` itself untouched (still valid, still a `uniform: false`
    /// fixture) and added this as a new, additive `assert_equivalent` case for `uniform: true`
    /// instead, per this workstream's "don't modify existing tests" rule.
    #[test]
    fn golden_number_stepper() {
        assert_equivalent("NumberStepper", &leaf(UiNode::NumberStepper(UiNumberStepperNode { id: "ns".into(), value: 2.0, step: 1.0, uniform: true, on_absolute: action(), on_delta: action(), presence: UiPresence::default(), menu: None })));
    }

    #[test]
    fn golden_ring() {
        assert_equivalent("Ring", &leaf(UiNode::Ring(UiRingNode { id: "ring".into(), orb_id: "orb".into(), t: 0.25, on_change: action(), presence: UiPresence::default(), menu: None })));
    }

    #[test]
    fn golden_icon_select() {
        assert_equivalent(
            "IconSelect",
            &leaf(UiNode::IconSelect(UiIconSelectNode { id: "ic".into(), value: IconName::Sparkles.to_string(), uniform: false, classifier_kind: "kind".into(), on_change: action(), presence: UiPresence::default(), menu: None })),
        );
    }

    #[test]
    fn golden_tree() {
        let item = |id: &str, label: &str| UiTreeItemNode {
            id: id.into(),
            label: Label::data(label),
            description: None,
            icon_id: None,
            presence: UiPresence::default(),
            default_open: None,
            action: None,
            actions: None,
            draggable: None,
            drag_data: None,
            items: None,
            control: None,
            dimmed: None,
            menu: None,
        };
        let node = UiNode::Tree(UiTreeNode {
            sections: vec![UiTreeSectionNode { id: "s1".into(), label: None, default_open: Some(true), presence: UiPresence::default(), items: vec![item("i1", "Item One"), item("i2", "Item Two")] }],
            presence: UiPresence::default(),
            drop_action: None,
            menu: None,
            interaction_domain: None,
        });
        assert_equivalent("Tree", &node);
    }

    /// KNOWN GAP: `reconcile` only expands `Field`/`Section` into a real retained child for their
    /// `child`/`children` payload (per `reconcile`'s own module doc comment — M2 recurses into
    /// `Stack`/`Section`/`Field` only), but `flex::LayoutEngine` only grants `flex_grow: 1.0` to a
    /// `Stack`'s children (see `style_with_grow`'s `flex_grow_child` param, gated on
    /// `matches!(node.spec.0, UiNode::Stack(_))`). A `Field`/`Section`'s synthetic retained child is
    /// therefore laid out at its own intrinsic content size instead of filling the label-adjusted
    /// remainder the way `widgets::render_widget`'s hand-rolled `Field`/`Section` branches
    /// (`Rect::new(bounds.x, bounds.y + label_h + gap, bounds.w, bounds.h - label_h - gap)` for
    /// `Field`, per-child accumulated `y` for `Section`) explicitly carve out. The two pipelines'
    /// geometry — and therefore instance counts for size-dependent content like wrapped `Text` — can
    /// genuinely diverge here. This is real follow-up work for `flex`, not something this façade can
    /// paper over; these two tests verify the retained side alone produces sane, non-empty output.
    #[test]
    fn golden_field_known_gap() {
        let node = UiNode::Field(UiFieldNode {
            id: "f".into(),
            label: Label::data("Label"),
            description: None,
            required: None,
            error: None,
            child: Box::new(UiNode::Input(UiInputNode {
                id: "in".into(),
                input_kind: "text".into(),
                value: "abc".into(),
                placeholder: None,
                commit: None,
                min: None,
                max: None,
                step: None,
                accept: None,
                on_change: action(),
                presence: UiPresence::default(),
                menu: None,
            })),
            presence: UiPresence::default(),
            menu: None,
        });
        let (instances, _, _) = retained_stats(&node);
        assert!(instances > 0, "Field should paint its label plus its child control");
    }

    #[test]
    fn golden_section_known_gap() {
        let node = UiNode::Section(UiSectionNode {
            id: "sec".into(),
            label: Some(Label::data("Section")),
            default_open: Some(true),
            presence: UiPresence::default(),
            children: vec![UiNode::Text(UiTextNode { value: Label::data("child"), emphasize: None, data_attributes: None, presence: UiPresence::default(), menu: None })],
            menu: None,
        });
        let (instances, _, _) = retained_stats(&node);
        assert!(instances > 0, "Section should paint its header label plus its children");
    }

    /// KNOWN GAP: see `to_widget_node`'s own `UiNode::Image | UiNode::ComponentScene | UiNode::ExternalSlot`
    /// match arm doc comment — `WidgetNode<E>` has no variant for any of these three, so there is no
    /// immediate-mode equivalent to compare against at all. `paint::paint_image`/
    /// `paint_component_scene`/`paint_external_slot` are themselves documented placeholders (no host
    /// texture-upload queue / scene-host / plugin-body wiring exists in `ui_wgpu` yet either) — these
    /// tests only verify the retained side produces the placeholder chrome its own doc comments
    /// promise, not equivalence with anything immediate-mode.
    #[test]
    fn golden_image_known_gap() {
        let node = UiNode::Image(UiImageNode { id: "img".into(), src: String::new(), alt: Some(Label::data("alt text")), presence: UiPresence::default(), menu: None });
        let (instances, _, _) = retained_stats(&node);
        assert!(instances > 0, "an empty-src Image should still paint its alt text");
    }

    #[test]
    fn golden_component_scene_known_gap() {
        let node = UiNode::ComponentScene(UiComponentSceneNode {
            surface_id: "surf".into(),
            controller_id: "ctrl".into(),
            component_kind: SurfaceKind::World3d,
            pane_id: None,
            binding_id: None,
            presence: UiPresence::default(),
            canvas_2d: None,
            world_3d: None,
            node_graph: None,
            text_editor: None,
            table: None,
            paint_2d: None,
            virtual_file_system: None,
            tiled_map: None,
            board2d: None,
            icon_render: None,
            ink_canvas: None,
            graph_timeline: None,
            block_list: None,
            diff_view: None,
            event_feed: None,
            menu: None,
        });
        let (instances, _, _) = retained_stats(&node);
        assert!(instances > 0, "ComponentScene should paint its placeholder border chrome");
    }

    #[test]
    fn golden_external_slot_known_gap() {
        let node = UiNode::ExternalSlot(UiExternalSlotNode { plugin_id: "plug".into(), app_id: "app".into(), body_key: "body".into(), params_json: "{}".into(), presence: UiPresence::default(), menu: None });
        let (instances, _, _) = retained_stats(&node);
        assert!(instances > 0, "ExternalSlot should paint its placeholder chrome plus its body_key label");
    }
    //#endregion 🔖️GoldenHarness

    //#region 🔬️IntrospectionTests
    #[test]
    fn window_ids_viewport_tree_and_theme_expose_private_window_state() {
        let mut ui = Ui::new();
        assert_eq!(ui.window_ids().count(), 0);
        assert_eq!(ui.viewport("win"), None);
        assert!(ui.tree("win").is_none());

        let node = UiNode::Text(UiTextNode { value: Label::data("hi"), emphasize: None, data_attributes: None, presence: UiPresence::default(), menu: None });
        ui.apply_tree("win", &node);
        ui.set_viewport("win", 800.0, 600.0);

        let ids: Vec<&str> = ui.window_ids().collect();
        assert_eq!(ids, vec!["win"]);
        assert_eq!(ui.viewport("win"), Some((800.0, 600.0)));
        let tree = ui.tree("win").expect("tree exists after apply_tree");
        assert!(tree.root.is_some());
        assert_eq!(ui.theme().text.a, Theme::default().text.a);
    }
    //#endregion 🔬️IntrospectionTests

    //#region 🧩️WidgetsInternalsTests
    /// 🧰️ Owns every piece `widgets::WidgetContext<'_, ActionDescriptor>` borrows, so each test can
    /// build one without fighting lifetimes; `ctx()` re-borrows fresh each call (a `WidgetContext`
    /// isn't `Clone`/reusable once passed to `render_widget`, which can mutate through it).
    struct WidgetHarness {
        draw: DrawList,
        atlas: FontAtlas,
        theme: Theme,
        input: InputState<ActionDescriptor>,
        scroll_offsets: StdHashMap<String, f32>,
        collapsed_sections: StdHashMap<String, bool>,
        open_selects: StdHashMap<String, bool>,
        maps: WidgetInteractionMaps<ActionDescriptor>,
    }

    impl WidgetHarness {
        fn new() -> Self {
            Self {
                draw: DrawList::default(),
                atlas: FontAtlas::builtin(),
                theme: Theme::default(),
                input: InputState::default(),
                scroll_offsets: StdHashMap::new(),
                collapsed_sections: StdHashMap::new(),
                open_selects: StdHashMap::new(),
                maps: WidgetInteractionMaps::default(),
            }
        }

        fn ctx(&mut self) -> WidgetContext<'_, ActionDescriptor> {
            WidgetContext {
                draw: &mut self.draw,
                overlay: None,
                atlas: &mut self.atlas,
                icons: None,
                input: &mut self.input,
                theme: &self.theme,
                scroll_offsets: &mut self.scroll_offsets,
                collapsed_sections: &mut self.collapsed_sections,
                open_selects: &mut self.open_selects,
                interaction_maps: Some(&mut self.maps),
                pick_clip: None,
            }
        }
    }

    #[test]
    fn wrap_text_wraps_long_text_across_multiple_lines() {
        let mut atlas = FontAtlas::builtin();
        let long = "word ".repeat(40);
        let lines = wrap_text(&mut atlas, &long, 100.0, 16.0);
        assert!(lines.len() > 1, "text far wider than max_width must wrap into multiple lines");
        for line in &lines {
            assert!(!line.is_empty());
        }
    }

    #[test]
    fn wrap_text_of_empty_string_yields_one_empty_line() {
        let mut atlas = FontAtlas::builtin();
        let lines = wrap_text(&mut atlas, "", 200.0, 16.0);
        assert_eq!(lines, vec![String::new()], "an empty input must still produce a single (empty) line, never zero lines");
    }

    #[test]
    fn measure_widget_stack_vertical_sums_child_heights_and_maxes_width() {
        let mut atlas = FontAtlas::builtin();
        let theme = Theme::default();
        let node = WidgetNode::<ActionDescriptor>::Stack { direction: "vertical".into(), gap: Some("none".into()), padding: Some("none".into()), children: vec![WidgetNode::Separator, WidgetNode::Separator] };
        let (_, h) = measure_widget(&mut atlas, &theme, &node);
        let (_, single_h) = measure_widget(&mut atlas, &theme, &WidgetNode::<ActionDescriptor>::Separator);
        assert!((h - single_h * 2.0).abs() < 0.001, "two stacked separators with no gap/padding must measure to exactly twice one separator's height, got {h} vs {single_h}");
    }

    #[test]
    fn measure_widget_stack_horizontal_sums_child_widths() {
        let mut atlas = FontAtlas::builtin();
        let theme = Theme::default();
        let button = || WidgetNode::<ActionDescriptor>::Button { id: Some("b".into()), icon_id: None, label: Label::data("Go").to_string(), event: None };
        let node = WidgetNode::Stack { direction: "horizontal".into(), gap: Some("none".into()), padding: Some("none".into()), children: vec![button(), button()] };
        let (w, _) = measure_widget(&mut atlas, &theme, &node);
        assert!((w - theme.control_height * 2.0).abs() < 0.001, "two gap-less horizontal buttons must measure to exactly twice one control's width");
    }

    #[test]
    fn measure_widget_separator_uses_theme_control_height_floor() {
        let mut atlas = FontAtlas::builtin();
        let theme = Theme::default();
        let (w, h) = measure_widget(&mut atlas, &theme, &WidgetNode::<ActionDescriptor>::Separator);
        assert_eq!(w, theme.control_height.max(1.0));
        assert_eq!(h, 1.0 + theme.gap_standard);
    }

    #[test]
    fn measure_widget_key_value_grows_with_entry_count() {
        let mut atlas = FontAtlas::builtin();
        let theme = Theme::default();
        let one = WidgetNode::<ActionDescriptor>::KeyValue { entries: vec![KeyValueEntry { label: Label::data("A").to_string(), value: "1".into() }] };
        let two = WidgetNode::<ActionDescriptor>::KeyValue { entries: vec![KeyValueEntry { label: Label::data("A").to_string(), value: "1".into() }, KeyValueEntry { label: Label::data("B").to_string(), value: "2".into() }] };
        let (_, h1) = measure_widget(&mut atlas, &theme, &one);
        let (_, h2) = measure_widget(&mut atlas, &theme, &two);
        assert!((h2 - h1 * 2.0).abs() < 0.001, "KeyValue height must scale linearly with entry count");
    }

    #[test]
    fn measure_widget_ring_is_fixed_size() {
        let mut atlas = FontAtlas::builtin();
        let theme = Theme::default();
        let (w, h) = measure_widget(&mut atlas, &theme, &WidgetNode::<ActionDescriptor>::Ring { id: "r".into(), t: 0.5, disabled: false, on_change: None });
        assert_eq!((w, h), (80.0, 80.0));
    }

    #[test]
    fn measure_widget_field_combines_label_and_child_height() {
        let mut atlas = FontAtlas::builtin();
        let theme = Theme::default();
        let node =
            WidgetNode::<ActionDescriptor>::Field { id: "f".into(), label: Label::data("Label").to_string(), child: ControlNode::Slider { id: "s".into(), value: 0.5, min: 0.0, max: 1.0, step: 0.1, ready: None, disabled: false, on_change: None } };
        let (_, h) = measure_widget(&mut atlas, &theme, &node);
        assert!(h > theme.control_height, "a Field's total height must be its label plus its child control, so it must exceed the control's own height alone");
    }

    #[test]
    fn measure_widget_section_sums_header_and_children_plus_gap() {
        let mut atlas = FontAtlas::builtin();
        let theme = Theme::default();
        let empty = WidgetNode::<ActionDescriptor>::Section { id: "s".into(), label: None, default_open: true, children: vec![] };
        let with_child = WidgetNode::<ActionDescriptor>::Section { id: "s".into(), label: None, default_open: true, children: vec![WidgetNode::Separator] };
        let (_, empty_h) = measure_widget(&mut atlas, &theme, &empty);
        let (_, child_h) = measure_widget(&mut atlas, &theme, &with_child);
        assert!(child_h > empty_h, "adding a child must grow a Section's measured height beyond its bare header height");
    }

    #[test]
    fn measure_widget_tree_skips_dimmed_items_in_height() {
        let mut atlas = FontAtlas::builtin();
        let theme = Theme::default();
        let item = |id: &str, dimmed: bool| TreeItem {
            id: id.into(),
            label: Label::data(id).to_string(),
            description: None,
            icon_id: None,
            selected: false,
            highlighted: false,
            default_open: false,
            dimmed,
            event: None,
            hover_event: None,
            unhover_event: None,
            actions: vec![],
            draggable: false,
            drag_data: StdHashMap::new(),
            control: None,
            children: vec![],
        };
        let visible = WidgetNode::<ActionDescriptor>::Tree { sections: vec![TreeSection { id: "sec".into(), label: None, default_open: true, items: vec![item("a", false)] }], selected_ids: vec![], highlighted_ids: vec![], selection_change: None };
        let dimmed = WidgetNode::<ActionDescriptor>::Tree { sections: vec![TreeSection { id: "sec".into(), label: None, default_open: true, items: vec![item("a", true)] }], selected_ids: vec![], highlighted_ids: vec![], selection_change: None };
        let (_, visible_h) = measure_widget(&mut atlas, &theme, &visible);
        let (_, dimmed_h) = measure_widget(&mut atlas, &theme, &dimmed);
        assert!(dimmed_h < visible_h, "a dimmed tree item must contribute zero height, so the dimmed tree must measure shorter than the visible one");
    }

    #[test]
    fn widget_interaction_maps_clear_frame_empties_every_map() {
        let mut maps = WidgetInteractionMaps::<ActionDescriptor>::default();
        maps.input_metas.insert("i".into(), InputMeta { on_change: action(), commit: None, value: "v".into() });
        maps.select_metas.insert("s".into(), action());
        maps.toggle_metas.insert("t".into(), (true, action()));
        maps.slider_metas.insert("sl".into(), SliderMeta { on_change: action(), min: 0.0, max: 1.0, step: 0.1, value: 0.5, bounds_x: 0.0, bounds_w: 10.0 });
        maps.stepper_metas.insert("st".into(), StepperMeta { on_absolute: action(), on_delta: action(), step: 1.0, value: 1.0 });
        maps.ring_metas.insert("r".into(), RingMeta { on_change: action(), disabled: false, center_x: 0.0, center_y: 0.0, radius: 10.0 });
        maps.slider_live_values.insert("sl".into(), 0.5);
        maps.ring_live_values.insert("r".into(), 0.5);
        maps.tree_hover_commands.insert("h".into(), action());
        maps.tree_unhover_commands.insert("u".into(), action());
        maps.tree_selection_change = Some(action());

        maps.clear_frame();

        assert!(maps.input_metas.is_empty());
        assert!(maps.select_metas.is_empty());
        assert!(maps.toggle_metas.is_empty());
        assert!(maps.slider_metas.is_empty());
        assert!(maps.stepper_metas.is_empty());
        assert!(maps.ring_metas.is_empty());
        assert!(maps.slider_live_values.is_empty());
        assert!(maps.ring_live_values.is_empty());
        assert!(maps.tree_hover_commands.is_empty());
        assert!(maps.tree_unhover_commands.is_empty());
        assert!(maps.tree_selection_change.is_none());
    }

    #[test]
    fn render_widget_input_registers_interaction_meta_when_maps_present() {
        let mut h = WidgetHarness::new();
        let node = WidgetNode::Input { id: "in".into(), input_kind: "text".into(), value: "hello".into(), placeholder: None, commit: Some("blur".into()), on_change: Some(action()) };
        render_widget(&node, VIEWPORT, &mut h.ctx());
        let meta = h.maps.input_metas.get("in").expect("register_input_meta must populate the map when interaction_maps is Some and on_change is Some");
        assert_eq!(meta.value, "hello");
        assert_eq!(meta.commit.as_deref(), Some("blur"));
    }

    #[test]
    fn render_widget_input_with_no_on_change_does_not_register_meta() {
        let mut h = WidgetHarness::new();
        let node = WidgetNode::Input { id: "in".into(), input_kind: "text".into(), value: "hello".into(), placeholder: None, commit: None, on_change: None };
        render_widget(&node, VIEWPORT, &mut h.ctx());
        assert!(h.maps.input_metas.is_empty(), "no on_change means nothing should be wired for the host to fire");
    }

    #[test]
    fn render_widget_select_and_toggle_register_interaction_metas() {
        let mut h = WidgetHarness::new();
        let select = WidgetNode::Select { id: "sel".into(), value: "a".into(), items: vec![SelectItem { value: "a".into(), label: Label::data("Alpha").to_string() }], placeholder: None, on_change: Some(action()) };
        render_widget(&select, VIEWPORT, &mut h.ctx());
        assert!(h.maps.select_metas.contains_key("sel"));

        let toggle = WidgetNode::Toggle { id: "tog".into(), icon_id: IconName::CircleDot, pressed: true, text: Some("On".into()), on_change: Some(action()) };
        render_widget(&toggle, VIEWPORT, &mut h.ctx());
        let (pressed, _) = h.maps.toggle_metas.get("tog").expect("toggle meta must be registered");
        assert!(*pressed);
    }

    #[test]
    fn render_widget_slider_registers_meta_and_live_value_unless_disabled() {
        let mut h = WidgetHarness::new();
        let enabled = WidgetNode::Slider { id: "sl".into(), value: 0.5, min: 0.0, max: 1.0, step: 0.01, ready: None, disabled: false, on_change: Some(action()) };
        render_widget(&enabled, VIEWPORT, &mut h.ctx());
        assert!(h.maps.slider_metas.contains_key("sl"));
        assert!(h.maps.slider_live_values.contains_key("sl"));

        let mut h2 = WidgetHarness::new();
        let disabled = WidgetNode::Slider { id: "sl".into(), value: 0.5, min: 0.0, max: 1.0, step: 0.01, ready: None, disabled: true, on_change: Some(action()) };
        render_widget(&disabled, VIEWPORT, &mut h2.ctx());
        assert!(h2.maps.slider_metas.is_empty(), "a disabled slider must not register interaction metadata");
        assert!(h2.maps.slider_live_values.is_empty());
    }

    #[test]
    fn render_widget_number_stepper_registers_stepper_meta() {
        let mut h = WidgetHarness::new();
        let node = WidgetNode::NumberStepper { id: "ns".into(), value: 3.0, step: 1.0, uniform: false, on_absolute: Some(action()), on_delta: Some(action()) };
        render_widget(&node, VIEWPORT, &mut h.ctx());
        let meta = h.maps.stepper_metas.get("ns").expect("stepper meta must be registered when both on_absolute and on_delta are Some");
        assert_eq!(meta.value, 3.0);
        assert!(h.maps.input_metas.contains_key("ns.input"), "the stepper's embedded value segment renders through render_input and must also register an input meta");
    }

    #[test]
    fn render_widget_ring_registers_meta_and_live_value() {
        let mut h = WidgetHarness::new();
        let node = WidgetNode::Ring { id: "r".into(), t: 0.25, disabled: false, on_change: Some(action()) };
        render_widget(&node, VIEWPORT, &mut h.ctx());
        assert!(h.maps.ring_metas.contains_key("r"));
        assert_eq!(h.maps.ring_live_values.get("r"), Some(&0.25));
    }

    #[test]
    fn render_widget_field_draws_label_and_delegates_to_control() {
        let mut h = WidgetHarness::new();
        let node = WidgetNode::Field { id: "f".into(), label: Label::data("Name").to_string(), child: ControlNode::Input { id: "in".into(), input_kind: "text".into(), value: "x".into(), placeholder: None, commit: None, on_change: Some(action()) } };
        render_widget(&node, VIEWPORT, &mut h.ctx());
        assert!(h.maps.input_metas.contains_key("in"), "Field must render its child control (an Input here), which registers its own interaction meta");
        let total: usize = h.draw.layers.iter().map(|l| l.ui_instances.len()).sum();
        assert!(total > 0, "Field must paint its label plus its child control");
    }

    #[test]
    fn render_widget_section_toggles_collapsed_state_from_default_open() {
        let child = || WidgetNode::<ActionDescriptor>::Text { value: "child text".into(), emphasize: false };
        let mut h = WidgetHarness::new();
        let closed = WidgetNode::<ActionDescriptor>::Section { id: "sec".into(), label: Some(Label::data("Sec").to_string()), default_open: false, children: vec![child()] };
        render_widget(&closed, VIEWPORT, &mut h.ctx());
        assert_eq!(h.collapsed_sections.get("section.sec"), Some(&true), "a Section with default_open: false must seed its collapsed_sections entry as collapsed");

        let mut h2 = WidgetHarness::new();
        let open = WidgetNode::<ActionDescriptor>::Section { id: "sec".into(), label: Some(Label::data("Sec").to_string()), default_open: true, children: vec![child()] };
        render_widget(&open, VIEWPORT, &mut h2.ctx());
        assert_eq!(h2.collapsed_sections.get("section.sec"), Some(&false));
        let closed_instances: usize = h.draw.layers.iter().map(|l| l.ui_instances.len()).sum();
        let open_instances: usize = h2.draw.layers.iter().map(|l| l.ui_instances.len()).sum();
        assert!(open_instances > closed_instances, "an open section must also paint its (visible) child's glyphs, a collapsed one must not");
    }

    #[test]
    fn render_widget_tree_populates_hover_and_unhover_commands() {
        let mut h = WidgetHarness::new();
        let item = TreeItem {
            id: "i1".into(),
            label: Label::data("Item").to_string(),
            description: None,
            icon_id: None,
            selected: false,
            highlighted: false,
            default_open: false,
            dimmed: false,
            event: None,
            hover_event: Some(action()),
            unhover_event: Some(action()),
            actions: vec![],
            draggable: false,
            drag_data: StdHashMap::new(),
            control: None,
            children: vec![],
        };
        let node = WidgetNode::<ActionDescriptor>::Tree {
            sections: vec![TreeSection { id: "s".into(), label: Some(Label::data("Section").to_string()), default_open: true, items: vec![item] }],
            selected_ids: vec![],
            highlighted_ids: vec![],
            selection_change: Some(action()),
        };
        render_widget(&node, VIEWPORT, &mut h.ctx());
        assert!(h.maps.tree_hover_commands.contains_key("i1"));
        assert!(h.maps.tree_unhover_commands.contains_key("i1"));
        assert_eq!(h.maps.tree_selection_change, Some(action()));
    }

    #[test]
    fn render_widget_tree_row_actions_register_hits_without_hover() {
        let mut h = WidgetHarness::new();
        let item = TreeItem {
            id: "i1".into(),
            label: Label::data("Item").to_string(),
            description: None,
            icon_id: None,
            selected: false,
            highlighted: false,
            default_open: false,
            dimmed: false,
            event: None,
            hover_event: None,
            unhover_event: None,
            actions: vec![TreeItemAction { icon_id: IconName::CircleDot, label: Some(Label::data("Del").to_string()), event: action(), placement: UiTreeActionPlacement::Row }],
            draggable: false,
            drag_data: StdHashMap::new(),
            control: None,
            children: vec![],
        };
        let node = WidgetNode::<ActionDescriptor>::Tree { sections: vec![TreeSection { id: "s".into(), label: None, default_open: true, items: vec![item] }], selected_ids: vec![], highlighted_ids: vec![], selection_change: None };
        render_widget(&node, VIEWPORT, &mut h.ctx());
        let action_hits = h.input.hit_targets.iter().filter(|t| t.control_id.as_deref() == Some("tree.action.i1.0")).count();
        assert_eq!(action_hits, 1, "row-placement actions must register a hit target even when the row is unhovered");
    }

    #[test]
    fn render_widget_tree_menu_placement_skips_row_action_hits() {
        let mut h = WidgetHarness::new();
        let item = TreeItem {
            id: "i1".into(),
            label: Label::data("Item").to_string(),
            description: None,
            icon_id: None,
            selected: false,
            highlighted: false,
            default_open: false,
            dimmed: false,
            event: None,
            hover_event: None,
            unhover_event: None,
            actions: vec![TreeItemAction { icon_id: IconName::CircleDot, label: Some(Label::data("Del").to_string()), event: action(), placement: UiTreeActionPlacement::Menu }],
            draggable: false,
            drag_data: StdHashMap::new(),
            control: None,
            children: vec![],
        };
        let node = WidgetNode::<ActionDescriptor>::Tree { sections: vec![TreeSection { id: "s".into(), label: None, default_open: true, items: vec![item] }], selected_ids: vec![], highlighted_ids: vec![], selection_change: None };
        render_widget(&node, VIEWPORT, &mut h.ctx());
        let action_hits = h.input.hit_targets.iter().filter(|t| t.control_id.as_deref() == Some("tree.action.i1.0")).count();
        assert_eq!(action_hits, 0, "menu-placement actions must not register row hit targets");
    }

    #[test]
    fn render_widget_tree_marks_selected_and_highlighted_ids_via_ids_list() {
        let mut h = WidgetHarness::new();
        let item = TreeItem {
            id: "i1".into(),
            label: Label::data("Item").to_string(),
            description: None,
            icon_id: None,
            selected: false,
            highlighted: false,
            default_open: false,
            dimmed: false,
            event: Some(action()),
            hover_event: None,
            unhover_event: None,
            actions: vec![],
            draggable: false,
            drag_data: StdHashMap::new(),
            control: None,
            children: vec![],
        };
        let node = WidgetNode::<ActionDescriptor>::Tree { sections: vec![TreeSection { id: "s".into(), label: None, default_open: true, items: vec![item] }], selected_ids: vec!["i1".into()], highlighted_ids: vec![], selection_change: None };
        render_widget(&node, VIEWPORT, &mut h.ctx());
        let hit = h.input.hit_targets.iter().find(|t| t.control_id.as_deref() == Some("tree.label.i1")).expect("tree item label must register a hit target");
        assert_eq!(hit.event, Some(action()));
    }

    #[test]
    fn render_scroll_region_clamps_stale_offset_to_new_max_scroll() {
        let mut h = WidgetHarness::new();
        h.scroll_offsets.insert("scroll".into(), 500.0);
        let bounds = Rect::new(0.0, 0.0, 100.0, 100.0);
        {
            let mut ctx = h.ctx();
            render_scroll_region("scroll", bounds, 150.0, &mut ctx, |_content, _ctx| {});
        }
        assert_eq!(h.scroll_offsets.get("scroll"), Some(&50.0), "offset must clamp to max_scroll (content_height - bounds.h) even if a stale value was larger");
    }

    #[test]
    fn render_scroll_region_registers_a_scroll_region_hit_target() {
        let mut h = WidgetHarness::new();
        let bounds = Rect::new(0.0, 0.0, 100.0, 100.0);
        {
            let mut ctx = h.ctx();
            render_scroll_region("myscroll", bounds, 400.0, &mut ctx, |_content, _ctx| {});
        }
        assert!(h.input.hit_targets.iter().any(|t| t.control_id.as_deref() == Some("myscroll")));
    }

    #[test]
    fn draw_text_on_emits_one_glyph_instance_per_character() {
        let mut draw = DrawList::default();
        let mut atlas = FontAtlas::builtin();
        draw_text_on(&mut draw, &mut atlas, "abc", 0.0, 0.0, 16.0, Theme::default().text);
        let total: usize = draw.layers.iter().map(|l| l.ui_instances.len()).sum();
        assert_eq!(total, 3);
    }

    #[test]
    fn draw_text_overlay_on_writes_to_the_overlay_channel_not_the_main_one() {
        let mut draw = DrawList::default();
        let mut atlas = FontAtlas::builtin();
        draw_text_overlay_on(&mut draw, &mut atlas, "hi", 0.0, 0.0, 16.0, Theme::default().text);
        let main: usize = draw.layers.iter().map(|l| l.ui_instances.len()).sum();
        let overlay: usize = draw.layers.iter().map(|l| l.overlay_ui_instances.len()).sum();
        assert_eq!(main, 0, "overlay glyphs must not land in the main ui_instances channel");
        assert_eq!(overlay, 2, "one overlay glyph instance per character");
    }
    //#endregion 🧩️WidgetsInternalsTests
}
//#region 🧪️RetainedDocumentHostileFixtures
#[cfg(test)]
mod retained_document_hostile_fixtures {
    use super::*;
    use ui_contract::{Component, SeparatorProps, SurfaceId, UiDocumentBuilder, UiNodeChildren, UiNodeId, UiNodeRecord, UiRevision};

    fn record(id: u64, children: &[u64]) -> UiNodeRecord {
        let mut child_ids = UiNodeChildren::default();
        for child in children {
            child_ids.try_push(UiNodeId(*child)).expect("bounded hostile child");
        }
        UiNodeRecord {
            id: UiNodeId(id),
            key: format!("node-{id}").try_into().expect("bounded hostile key"),
            component: Component::Separator(SeparatorProps {}),
            layout: Default::default(),
            style: Default::default(),
            activity: Default::default(),
            disabled: false,
            transition: None,
            accessibility: Default::default(),
            bindings: Default::default(),
            menu: None,
            children: child_ids,
        }
    }

    fn lease(generation: u64, nodes: &[(u64, &[u64])]) -> ui_contract::UiDocumentLease {
        let surface = SurfaceId::try_from("hostile.surface").expect("bounded hostile surface");
        let mut builder = UiDocumentBuilder::try_new(generation, surface, UiRevision(generation), Some(UiNodeId(nodes[0].0)), generation).expect("hostile builder");
        for (id, children) in nodes {
            builder.try_push(record(*id, children)).expect("hostile page");
        }
        builder.finish().expect("hostile lease")
    }

    fn step(generation: u64, preview: &mut u64) -> StepContext<'_> {
        let now = semio_framework_job::default_now_ms();
        StepContext::new(
            semio_framework_job::OperationId(generation),
            semio_framework_job::Generation(generation),
            semio_framework_job::StepBudget::new(1, now.saturating_add(100)),
            semio_framework_job::CancelToken::root_now(),
            semio_framework_job::default_now_ms,
            preview,
        )
    }

    fn cancelled_step(generation: u64, preview: &mut u64) -> StepContext<'_> {
        let cancel = semio_framework_job::CancelToken::root_now();
        cancel.cancel_now();
        StepContext::new(semio_framework_job::OperationId(generation), semio_framework_job::Generation(generation), semio_framework_job::StepBudget::new(1, u64::MAX), cancel, semio_framework_job::default_now_ms, preview)
    }

    #[test]
    fn max_plus_one_stale_aba_interrupted_close_nested_depth_lost_handle_device_drop_and_last_valid_snapshot() {
        let mut leases = Vec::new();
        for generation in 1..=ui_contract::UI_DOCUMENT_LEASE_SLOTS as u64 {
            leases.push(lease(generation, &[(1, &[])]));
        }
        let rejected_surface = SurfaceId::try_from("hostile.max-plus-one").expect("bounded hostile surface");
        let rejected = UiDocumentBuilder::try_new(99, rejected_surface, UiRevision(99), Some(UiNodeId(1)), 99).expect_err("max plus one returns surface owner");
        assert_eq!(rejected.1.as_ref(), "hostile.max-plus-one");
        drop(leases);
        while !ui_contract::close_ui_document_page_one() {}

        let mut ui = Ui::new();
        let first = lease(101, &[(1, &[])]);
        let header = first.header().expect("first header");
        let mut preview = 0;
        ui.begin_document("window", header, &mut step(101, &mut preview)).expect("first begin");
        ui.apply_document_page("window", first.read_node_page(0).expect("first read").expect("first page"), &mut step(101, &mut preview)).expect("first apply");
        loop {
            match ui.finish_document("window", 101, &mut step(101, &mut preview)) {
                Ok(()) => break,
                Err(UiDocumentIngressFault::ValidationPending) => {}
                Err(fault) => panic!("first publish failed: {fault:?}"),
            }
        }
        assert_eq!(ui.windows.get("window").and_then(|window| window.tree.document()).map(UiDocumentTree::generation), Some(101));

        let aba = lease(100, &[(1, &[])]);
        let aba_rejected = ui.begin_document("window", aba.header().expect("aba header"), &mut step(100, &mut preview));
        assert!(matches!(aba_rejected, Err((UiDocumentIngressFault::StaleGeneration, _))));
        let cancelled = lease(104, &[(1, &[])]);
        let cancelled_rejected = ui.begin_document("window", cancelled.header().expect("cancelled header"), &mut cancelled_step(104, &mut preview));
        assert!(matches!(cancelled_rejected, Err((UiDocumentIngressFault::Cancelled, _))));

        let nested = lease(102, &[(1, &[2]), (2, &[3]), (3, &[])]);
        ui.begin_document("window", nested.header().expect("nested header"), &mut step(102, &mut preview)).expect("nested begin");
        ui.apply_document_page("window", nested.read_node_page(0).expect("nested read").expect("nested page"), &mut step(102, &mut preview)).expect("nested first page");
        let stale = lease(103, &[(1, &[])]);
        let interrupted = ui.begin_document("window", stale.header().expect("stale header"), &mut step(103, &mut preview));
        assert!(matches!(interrupted, Err((UiDocumentIngressFault::InterruptedClose, _))));
        assert_eq!(ui.windows.get("window").and_then(|window| window.tree.document()).map(UiDocumentTree::generation), Some(101));
        drop(aba);
        drop(cancelled);
        drop(first);
        drop(nested);
        drop(stale);
        while !ui.close_document_step("window") {}
        while !ui_contract::close_ui_document_page_one() {}
        assert!(ui.windows.get("window").and_then(|window| window.tree.document()).is_none());
    }
}
//#endregion 🧪️RetainedDocumentHostileFixtures
// #endregion engine
