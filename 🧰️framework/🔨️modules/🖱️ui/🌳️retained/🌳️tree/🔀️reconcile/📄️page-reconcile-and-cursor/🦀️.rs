//! 📄️ Page reconcile and cursor: the document tree page reconcile, the reconcile depth, faults and steps, the retired scene and witness, the reconcile phase and the `UiDocumentReconcileCursor`. See [`super`].

use super::*;

// #region reconcile
//#region 📄️DocumentPageReconcile
impl UiDocumentTree {
    #[expect(clippy::result_large_err, reason = "Fixed-capacity node admission returns the exact rejected node or page without allocating on refusal.")]
    pub fn try_push_page(&mut self, page: UiDocumentNodePage, expected_index: usize) -> Result<(), UiDocumentPageRejection> {
        let generation = page.generation();
        let revision = page.revision();
        let index = page.index();
        let fault = if generation != self.generation {
            Some(UiDocumentTreeFault::Generation)
        } else if revision != self.revision {
            Some(UiDocumentTreeFault::Revision)
        } else if index != expected_index {
            Some(UiDocumentTreeFault::PageOrder)
        } else if self.nodes.get(&page.record().id).is_some() {
            Some(UiDocumentTreeFault::DuplicateNode)
        } else {
            None
        };
        let record = page.into_record();
        if let Some(fault) = fault {
            return Err(UiDocumentPageRejection { fault, generation, revision, index, record });
        }
        self.nodes.try_insert(record).map(|_| ()).map_err(|record| UiDocumentPageRejection { fault: UiDocumentTreeFault::NodeCapacity, generation, revision, index, record })
    }
}
//#endregion 📄️DocumentPageReconcile

//#region 🌳️DocumentTreeReconcile
/// 🌳️ The deepest published document the reconcile walks in one pass — one frame per nesting level
/// of the DFS plan stack, sized to the same ceiling `mounted_layout`'s own walk carries
/// (`LAYOUT_DEPTH_CREDITS`), because a tree the layout cannot walk is not worth mounting.
pub const UI_DOCUMENT_RECONCILE_DEPTH: usize = 64;

/// 🚨️ Why a published document could not be projected into the paintable arena. Never a panic and
/// never a dropped document: the surface keeps its previous arena content and the caller records the
/// fault, exactly the rule `ui_contract`'s `🗺️surface.rs` states for an unresolvable surface.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UiDocumentReconcileFault {
    /// 🕳️ A record the document's own `children` list names is not in its node table.
    MissingRecord,
    /// 🪜️ The plan stack outgrew [`UI_DOCUMENT_RECONCILE_DEPTH`] or `UI_DOCUMENT_NODES`.
    Depth,
    /// 🌱️ A planned child's parent never mounted — structurally impossible on a pre-order plan, so
    /// this can only mean the arena was mutated under the cursor.
    Detached,
    /// 🎬️ The retained node exhausted its component-mount identity counter.
    ComponentGeneration,
    /// 🪙 The presented document could not acquire an independently credited candidate record.
    DocumentCredits,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UiDocumentReconcileStep {
    Pending,
    Complete,
    Fault(UiDocumentReconcileFault),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UiRetiredComponentScene {
    pub document_id: UiNodeId,
    pub host_id: String,
    pub window_id: String,
    pub window_generation: u64,
    pub component_generation: u64,
    pub node: NodeId,
    pub key: NodeKey,
    pub kind: SurfaceKind,
    pub surface_id: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UiComponentSceneWitness<'a> {
    pub document_id: UiNodeId,
    pub host_id: &'a str,
    pub window_id: &'a str,
    pub window_generation: u64,
    pub component_generation: u64,
    pub key: NodeKeyRef<'a>,
    pub kind: SurfaceKind,
    pub surface_id: &'a str,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(super) enum UiDocumentReconcilePhase {
    /// 🔽️ Drains the tree's synthesized composite rows (an open `Select`'s option rows) one node per
    /// step BEFORE anything relinks the arena. `Unlink`/`Mount` below rebuild the sibling chains from
    /// the published record set alone, which would orphan every synthesized child and leak its arena
    /// slot on each republish; `paint::sync_interactive_state_node_step` re-mints whatever is still
    /// open on the next frame, so nothing is lost and the working set stays bounded
    /// (ticket 26/09/17 packet W15a).
    #[default]
    Compose,
    Adopt,
    Plan,
    Unlink,
    Mount,
    Retire,
    Publish,
    Complete,
    Fault,
}

#[derive(Clone, Copy, Debug)]
pub(super) struct PlannedNode {
    pub(super) id: UiNodeId,
    pub(super) parent: Option<usize>,
    pub(super) node: Option<NodeId>,
}

/// 🌳️ One surface's resumable document→arena reconcile.
///
/// Five phases, each advancing by exactly ONE unit per `step` so the whole pass fits inside the
/// interactive turn the frame build already prices (`📓️wgpu-blank-paint-2026-09-12.md` §3.2): plan
/// one record, unlink one mounted node, mount one record, retire one abandoned node, publish. A pass
/// that runs out of budget resumes at the same unit on the next opportunity, and a pass that faults
/// leaves the previous arena content standing.
///
/// Identity is the `UiNodeId`, not tree position: the producer mints one per `(parent, key)` and
/// reuses it for as long as that identity survives, so a node that merely moved or changed props
/// keeps its arena slot — and with it its `WidgetState` (a focused editor's live buffer, a scroll
/// offset, an open `Select`) and its interaction flags. This is the Rust twin of the React
/// `Interpreter`'s `uiChildReactKeys` identity rule.
#[derive(Default)]
pub struct UiDocumentReconcileCursor {
    pub(super) phase: UiDocumentReconcilePhase,
    pub(super) fault: Option<UiDocumentReconcileFault>,
    pub(super) generation: u64,
    pub(super) plan: Vec<PlannedNode>,
    pub(super) planned_ids: Vec<UiNodeId>,
    pub(super) stack: Vec<(UiNodeId, Option<usize>)>,
    pub(super) sweep: usize,
    pub(super) mount: usize,
}

impl UiDocumentReconcileCursor {
    /// 🧹️ Retires one reconcile index before releasing its fixed-lifetime window owner.
    pub fn close_step(&mut self) -> bool {
        if self.plan.pop().is_some() || self.planned_ids.pop().is_some() || self.stack.pop().is_some() {
            return false;
        }
        if self.plan.capacity() > 0 {
            self.plan = Vec::new();
            return false;
        }
        if self.planned_ids.capacity() > 0 {
            self.planned_ids = Vec::new();
            return false;
        }
        if self.stack.capacity() > 0 {
            self.stack = Vec::new();
            return false;
        }
        self.phase = UiDocumentReconcilePhase::Complete;
        self.fault = None;
        true
    }

    /// 🔁️ Rearms the cursor for `generation`, discarding any half-finished pass for an older one.
    pub fn rearm(&mut self, generation: u64) {
        if self.generation == generation && !matches!(self.phase, UiDocumentReconcilePhase::Complete | UiDocumentReconcilePhase::Fault) {
            return;
        }
        self.phase = UiDocumentReconcilePhase::Compose;
        self.fault = None;
        self.generation = generation;
        self.plan.clear();
        self.planned_ids.clear();
        self.stack.clear();
        self.sweep = 0;
        self.mount = 0;
    }

    pub fn generation(&self) -> u64 {
        self.generation
    }

    pub fn terminal_is_complete(&self) -> bool {
        matches!(self.phase, UiDocumentReconcilePhase::Complete)
    }

    pub fn fault(&self) -> Option<UiDocumentReconcileFault> {
        self.fault
    }

    pub(super) fn refuse(&mut self, fault: UiDocumentReconcileFault) -> UiDocumentReconcileStep {
        self.phase = UiDocumentReconcilePhase::Fault;
        self.fault = Some(fault);
        UiDocumentReconcileStep::Fault(fault)
    }

    pub(super) fn remember_planned(&mut self, id: UiNodeId) {
        if let Err(index) = self.planned_ids.binary_search(&id) {
            self.planned_ids.insert(index, id);
        }
    }
}
//#endregion 🌳️DocumentTreeReconcile
// #endregion reconcile
