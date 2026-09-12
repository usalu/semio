// #region tree
//! 🌲️ Retained scene-graph: one `UiTree` per window, holding `Node`s in a generational `Arena`
//! with parent/first-child/last-child/sibling links and dirty-flag propagation. The engine facade
//! (a later milestone) holds `HashMap<window_id, UiTree>`.

use crate::wgpu::arena::{Arena, NodeId};
use crate::wgpu::component::ui::UiNode;
use ui_contract::{SurfaceId, UiDocumentLeaseHeader, UiNodeId, UiNodeRecord, UiNodeTable, UiRevision, UI_DOCUMENT_NODES};

//#region 🔖️RetainedDocument
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UiDocumentTreeFault {
    Generation,
    Revision,
    PageOrder,
    DuplicateNode,
    NodeCapacity,
    Count,
    MissingRoot,
    MissingChild,
    MultipleParents,
    Cycle,
}

#[derive(Debug)]
pub struct UiDocumentPageRejection {
    pub fault: UiDocumentTreeFault,
    pub generation: u64,
    pub revision: UiRevision,
    pub index: usize,
    pub record: UiNodeRecord,
}

#[derive(Debug)]
pub struct UiDocumentTree {
    pub(crate) generation: u64,
    pub(crate) surface: SurfaceId,
    pub(crate) revision: UiRevision,
    pub(crate) root: UiNodeId,
    pub(crate) layout_epoch: u64,
    pub(crate) node_count: usize,
    pub(crate) nodes: UiNodeTable,
}

impl UiDocumentTree {
    pub fn new(header: UiDocumentLeaseHeader) -> Result<Self, UiDocumentTreeFault> {
        if header.generation == 0 || header.node_count > UI_DOCUMENT_NODES {
            return Err(UiDocumentTreeFault::Generation);
        }
        Ok(Self { generation: header.generation, surface: header.surface, revision: header.revision, root: header.root, layout_epoch: header.layout_epoch, node_count: header.node_count, nodes: UiNodeTable::default() })
    }

    pub fn generation(&self) -> u64 {
        self.generation
    }

    pub fn surface(&self) -> &SurfaceId {
        &self.surface
    }

    pub fn revision(&self) -> UiRevision {
        self.revision
    }

    pub fn layout_epoch(&self) -> u64 {
        self.layout_epoch
    }

    pub fn root_id(&self) -> UiNodeId {
        self.root
    }

    pub fn record(&self, id: UiNodeId) -> Option<&UiNodeRecord> {
        self.nodes.get(&id)
    }

    pub fn parent_of(&self, id: UiNodeId) -> Option<UiNodeId> {
        self.nodes.values().find(|record| record.children.iter().any(|child| *child == id)).map(|record| record.id)
    }

    #[expect(clippy::result_large_err, reason = "Fixed-capacity node admission returns the exact rejected node or page without allocating on refusal.")]
    pub fn try_upsert_record(&mut self, record: UiNodeRecord) -> Result<Option<UiNodeRecord>, UiNodeRecord> {
        self.nodes.try_insert(record)
    }

    pub fn try_reparent(&mut self, child: UiNodeId, parent: UiNodeId) -> Result<(), UiDocumentTreeFault> {
        if child == self.root || self.record(child).is_none() || self.record(parent).is_none() {
            return Err(UiDocumentTreeFault::MissingChild);
        }
        let previous = self.parent_of(child);
        if previous == Some(parent) {
            return Ok(());
        }
        let mut ancestor = Some(parent);
        for _ in 0..UI_DOCUMENT_NODES {
            let Some(current) = ancestor else { break };
            if current == child {
                return Err(UiDocumentTreeFault::Cycle);
            }
            ancestor = self.parent_of(current);
        }
        if ancestor.is_some() {
            return Err(UiDocumentTreeFault::Cycle);
        }
        self.nodes.get_mut(&parent).ok_or(UiDocumentTreeFault::MissingChild)?.children.try_push(child).map_err(|_| UiDocumentTreeFault::NodeCapacity)?;
        if let Some(previous) = previous {
            let children = &mut self.nodes.get_mut(&previous).ok_or(UiDocumentTreeFault::MissingChild)?.children;
            let index = children.iter().position(|id| *id == child).ok_or(UiDocumentTreeFault::MissingChild)?;
            children.swap_remove(index);
            for cursor in index..children.len().saturating_sub(1) {
                let next = *children.get(cursor + 1).ok_or(UiDocumentTreeFault::MissingChild)?;
                let current = std::mem::replace(children.get_mut(cursor).ok_or(UiDocumentTreeFault::MissingChild)?, next);
                *children.get_mut(cursor + 1).ok_or(UiDocumentTreeFault::MissingChild)? = current;
            }
        }
        Ok(())
    }

    pub fn remove_record(&mut self, id: UiNodeId) -> Option<UiNodeRecord> {
        self.nodes.remove(&id)
    }

    pub fn close_step(&mut self) -> bool {
        let next = self.nodes.keys().next().copied();
        if let Some(id) = next {
            drop(self.nodes.remove(&id));
            return false;
        }
        true
    }

    pub fn validate_header(&self) -> Result<(), UiDocumentTreeFault> {
        if self.nodes.len() != self.node_count {
            return Err(UiDocumentTreeFault::Count);
        }
        if self.nodes.get(&self.root).is_none() {
            return Err(UiDocumentTreeFault::MissingRoot);
        }
        Ok(())
    }

    pub fn validate_record(&self, index: usize) -> Result<(), UiDocumentTreeFault> {
        let record = self.nodes.get_index(index).ok_or(UiDocumentTreeFault::Count)?;
        for child in &record.children {
            if self.nodes.get(child).is_none() {
                return Err(UiDocumentTreeFault::MissingChild);
            }
            let parents = self.nodes.values().filter(|candidate| candidate.children.iter().any(|candidate_child| candidate_child == child)).count();
            if parents != 1 || *child == self.root {
                return Err(UiDocumentTreeFault::MultipleParents);
            }
        }
        Ok(())
    }
}
//#endregion 🔖️RetainedDocument

/// 🔑️ Stable child identity for keyed reconciliation: the source `UiNode`'s explicit `id` field
/// when it has one, else a `(variant, ordinal)` positional fallback.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum NodeKey {
    Explicit(String),
    Positional(u32, u32),
}

/// 🚩️ Per-node dirty/interaction bits. Hand-rolled over a `u16` (no `bitflags` dep) to keep the
/// crate dependency-free for this ~10-flag set.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct NodeFlags(u16);

impl NodeFlags {
    pub const HOVERED: NodeFlags = NodeFlags(1 << 0);
    pub const ACTIVE: NodeFlags = NodeFlags(1 << 1);
    pub const FOCUSED: NodeFlags = NodeFlags(1 << 2);
    pub const DIRTY_LAYOUT: NodeFlags = NodeFlags(1 << 3);
    pub const DIRTY_PAINT: NodeFlags = NodeFlags(1 << 4);
    pub const SUBTREE_DIRTY: NodeFlags = NodeFlags(1 << 5);
    pub const OVERLAY: NodeFlags = NodeFlags(1 << 6);
    pub const CLIPS_CHILDREN: NodeFlags = NodeFlags(1 << 7);
    pub const HIT_TRANSPARENT: NodeFlags = NodeFlags(1 << 8);
    pub const HAS_POPUP: NodeFlags = NodeFlags(1 << 9);
    /// 🫳️ M5 `events`: this node is a drag source (has, or can have, a registered `DragPayload`).
    /// Purely advisory for paint (grab-cursor affordance)/cursor derivation — `events` itself tracks
    /// draggability via its own `EventRouter::set_drag_payload` registry, not this flag.
    pub const DRAG_SOURCE: NodeFlags = NodeFlags(1 << 10);
    /// 🎯️ M5 `events`: this node accepts drops (paired with `EventRouter::set_drop_accept` for the
    /// finer per-widget predicate). `events::nearest_accepting_drop_target` walks the bubble chain
    /// looking for this flag.
    pub const DROP_TARGET: NodeFlags = NodeFlags(1 << 11);
    /// 🖱️ M5 `events`: this node owns a scrollable viewport (`WidgetState::scroll_offset`).
    /// `events::nearest_scrollable_ancestor` walks the bubble chain from a wheel event's hit target
    /// looking for this flag.
    pub const SCROLLABLE: NodeFlags = NodeFlags(1 << 12);

    pub const fn empty() -> Self {
        NodeFlags(0)
    }

    pub fn set(&mut self, flag: NodeFlags, on: bool) {
        if on {
            self.0 |= flag.0;
        } else {
            self.0 &= !flag.0;
        }
    }

    pub fn contains(&self, flag: NodeFlags) -> bool {
        self.0 & flag.0 == flag.0
    }
}

/// 🧩️ Retained per-node widget spec. For M2 a thin clone of the last-applied `UiNode` (used as the
/// reconcile diff baseline); refined into per-variant retained fields in M4.
#[derive(Clone, Debug, PartialEq)]
pub struct WidgetSpec(pub UiNode);

/// ✍️ A focused editable text widget's live buffer (`events`' M5 key-routing writes here). Byte
/// offsets throughout (`caret`/`anchor`), not char indices: Rust string slicing/`replace_range` are
/// natively byte-indexed, and `events::{prev_char_boundary, next_char_boundary}` step these
/// safely across multi-byte UTF-8 without an O(n) char-counting pass on every keystroke. Selection
/// is `anchor..caret` in either order (mirrors the DOM `Selection` model: `anchor` is where the
/// selection started, `caret`/`focus` is the live end that arrow keys move).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct EditState {
    pub text: String,
    pub caret: usize,
    pub anchor: usize,
    /// 🈶️ IME preedit text, `Some` only mid-composition (`events::UiEvent::Ime`). Modeled so the
    /// shape is ready to receive real OS IME events; actually wiring winit's `Ime`/a hidden DOM
    /// input to this is a later `host`-region concern, out of `events`' scope.
    pub composition: Option<String>,
    pub scroll_x: f32,
}

/// 🎛️ Interactive per-node state that survives `reconcile::apply_tree` untouched (only `spec` is
/// ever overwritten by reconciliation — see `reconcile::diff_and_update`), which is exactly the
/// "focused buffer wins over a fresh incoming `value`" guarantee M5 `events` needs: as long as
/// `edit` stays `Some`, an `apply_tree` call re-diffing this node's declarative `value` never
/// touches it. `events::FocusState::set_focus` seeds `edit` from the widget's declarative value on
/// focus and clears it on blur, so external state governs again once editing ends.
#[derive(Clone, Debug, Default)]
pub struct WidgetState {
    pub edit: Option<EditState>,
    /// 🖱️ M5 `events` scroll routing's live offset for a `NodeFlags::SCROLLABLE` node.
    pub scroll_offset: (f32, f32),
    /// 🔽️ M5/W2 wiring: whether a `Select`'s synthesized popup (`reconcile::children_of`'s `Select`
    /// arm always builds the item rows unconditionally, per that module's own doc comment) is
    /// currently shown. Toggled by `events::EventRouter::dispatch`'s `Select`-click handling (via
    /// `open_overlay`/`close_overlay`, see `EventRouter::toggle_select_popup`/`finish_close`), read
    /// by `paint::paint_select` to decide whether to paint the popup at all.
    pub open: bool,
}

/// 📐️ Resolved rect from the last taffy layout pass, in the node's **parent-relative** coordinate
/// space (taffy's own `Layout::location`/`Layout::size` semantics — no extra transform needed when
/// consuming it; a later paint milestone accumulates ancestor offsets while walking the tree, same
/// as it already walks parent/child links for painting). `cached_text_measure` mirrors the last
/// `(text, wrap width bucket)` this node was measured at, so `flex::LayoutEngine` can skip
/// re-shaping an unchanged text node against an unchanged constraint.
/// 📏️ `(text, wrap width bucket)` key paired with its measured `(width, height)` — see
/// `LayoutBucket::cached_text_measure`.
pub type TextMeasureCache = Option<((String, Option<u32>), (f32, f32))>;

#[derive(Clone, Debug, Default)]
pub struct LayoutBucket {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub cached_text_measure: TextMeasureCache,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(crate) struct AcceptedLayout {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
struct MountedLayoutRecord {
    generation: u64,
    layout: AcceptedLayout,
}

/// 🎨️ M4 decision: stays an empty marker. Every `paint::paint_*` function recomputes its `DrawList`
/// entries fresh from `spec`+`layout`+`theme` on each visit instead of caching tessellation output —
/// the composite widgets that would benefit most from caching (Select's open menu, Tree's rows)
/// aren't retained children yet (that's a later reconcile milestone), so there's nothing stable to
/// key a cache on without duplicating that future work; recomputing a handful of quads/glyph-runs
/// per dirty node per frame is cheap relative to the tessellation this replaces. Revisit once
/// composite expansion lands and glyph-run caching becomes worth the bookkeeping.
#[derive(Clone, Debug, Default)]
pub struct PaintBucket;

/// 🍃️ One retained tree node: tree links, identity, spec/state/layout/paint buckets, dirty flags.
pub struct Node {
    pub parent: Option<NodeId>,
    pub first_child: Option<NodeId>,
    pub last_child: Option<NodeId>,
    pub prev_sibling: Option<NodeId>,
    pub next_sibling: Option<NodeId>,
    pub key: NodeKey,
    pub spec: WidgetSpec,
    pub state: WidgetState,
    pub layout: LayoutBucket,
    mounted_layout: [MountedLayoutRecord; 2],
    pub paint: PaintBucket,
    pub flags: NodeFlags,
}

impl Node {
    pub fn new(key: NodeKey, spec: WidgetSpec) -> Self {
        Self {
            parent: None,
            first_child: None,
            last_child: None,
            prev_sibling: None,
            next_sibling: None,
            key,
            spec,
            state: WidgetState::default(),
            layout: LayoutBucket::default(),
            mounted_layout: [MountedLayoutRecord::default(); 2],
            paint: PaintBucket,
            flags: NodeFlags::empty(),
        }
    }
}

/// 🌲️ One window's retained scene-graph: a generational arena of `Node`s plus its root.
///
/// `document_nodes` is the identity ledger the retained-document reconcile
/// (`🔁️reconcile.rs`'s `🌳️DocumentTreeReconcile`) keys on: one `UiNodeId → NodeId` binding per
/// mounted record, kept sorted so a lookup is a binary search and never an allocation. A
/// `UiNodeId` is minted per `(parent, key)` by the producer's own reconciler
/// (`🧠️runtime`'s `SurfaceReconcileStage::AllocateIdentities`) and reused for as long as that
/// identity survives, so a binding held across document generations is exactly the "same element"
/// guarantee React's `Interpreter` gets from its `uiChildReactKeys`.
#[derive(Default)]
pub struct UiTree {
    arena: Arena<Node>,
    pub root: Option<NodeId>,
    document: Option<UiDocumentTree>,
    document_nodes: Vec<(UiNodeId, NodeId)>,
    mounted_layout_active: usize,
    mounted_layout_generation: u64,
}

impl UiTree {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn node(&self, id: NodeId) -> Option<&Node> {
        self.arena.get(id)
    }

    pub fn node_mut(&mut self, id: NodeId) -> Option<&mut Node> {
        self.arena.get_mut(id)
    }

    pub(crate) fn accepted_layout(&self, id: NodeId) -> Option<AcceptedLayout> {
        let node = self.arena.get(id)?;
        let mounted = node.mounted_layout[self.mounted_layout_active];
        if self.mounted_layout_generation != 0 && mounted.generation == self.mounted_layout_generation {
            Some(mounted.layout)
        } else {
            Some(AcceptedLayout { x: node.layout.x, y: node.layout.y, width: node.layout.width, height: node.layout.height })
        }
    }

    pub(crate) fn write_inactive_layout(&mut self, id: NodeId, generation: u64, layout: AcceptedLayout) -> bool {
        let inactive = self.mounted_layout_active ^ 1;
        let Some(node) = self.arena.get_mut(id) else { return false };
        node.mounted_layout[inactive] = MountedLayoutRecord { generation, layout };
        true
    }

    pub(crate) fn commit_inactive_layout(&mut self, generation: u64) {
        self.mounted_layout_active ^= 1;
        self.mounted_layout_generation = generation;
    }

    #[cfg(test)]
    pub(crate) fn accepted_layout_generation(&self) -> u64 {
        self.mounted_layout_generation
    }

    /// 📐️ The layout the PAINT path actually reads for `id`, as `(x, y, width, height)` in its
    /// parent's space. `Node::layout` is the immediate-mode bucket and stays zero on the retained
    /// path — `mounted_layout`'s double-buffered `AcceptedLayout` is what `paint_node_step` and
    /// `RetainedPaintWalk` consume, so a probe reading `Node::layout` reports an unlaid-out tree even
    /// when layout published (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    pub fn mounted_layout(&self, id: NodeId) -> Option<(f32, f32, f32, f32)> {
        self.accepted_layout(id).map(|layout| (layout.x, layout.y, layout.width, layout.height))
    }

    pub fn contains(&self, id: NodeId) -> bool {
        self.arena.contains(id)
    }

    pub fn document(&self) -> Option<&UiDocumentTree> {
        self.document.as_ref()
    }

    pub fn publish_document(&mut self, document: UiDocumentTree) -> Option<UiDocumentTree> {
        self.document.replace(document)
    }

    pub fn take_document(&mut self) -> Option<UiDocumentTree> {
        self.document.take()
    }

    //#region 🪪️DocumentIdentity
    /// 🪪️ The arena node currently standing for `id`, if one is mounted.
    pub(crate) fn document_node(&self, id: UiNodeId) -> Option<NodeId> {
        self.document_nodes.binary_search_by_key(&id, |(document, _)| *document).ok().and_then(|index| self.document_nodes.get(index)).map(|(_, node)| *node)
    }

    /// 🪪️ Binds `id` to `node`, replacing any previous binding for the same record.
    pub(crate) fn bind_document_node(&mut self, id: UiNodeId, node: NodeId) {
        match self.document_nodes.binary_search_by_key(&id, |(document, _)| *document) {
            Ok(index) => {
                if let Some(entry) = self.document_nodes.get_mut(index) {
                    entry.1 = node;
                }
            }
            Err(index) => self.document_nodes.insert(index, (id, node)),
        }
    }

    /// 🪪️ The ledger in `UiNodeId` order — the retirement sweep's own iteration order.
    pub(crate) fn document_bindings(&self) -> &[(UiNodeId, NodeId)] {
        &self.document_nodes
    }

    /// 🧹️ Drops the binding at `index` and returns the arena node it named.
    pub(crate) fn unbind_document_node_at(&mut self, index: usize) -> Option<NodeId> {
        if index >= self.document_nodes.len() {
            return None;
        }
        Some(self.document_nodes.remove(index).1)
    }

    /// 🔗️ Severs every tree link on `id` without touching the node itself — the first half of a
    /// relink pass, so a later `attach_child` can rebuild sibling order from the published document
    /// without any node losing its arena identity (and with it its `WidgetState`, its focused edit
    /// buffer and its interaction flags).
    pub(crate) fn clear_links(&mut self, id: NodeId) {
        if let Some(node) = self.arena.get_mut(id) {
            node.parent = None;
            node.first_child = None;
            node.last_child = None;
            node.prev_sibling = None;
            node.next_sibling = None;
        }
    }

    /// 🔗️ Appends the already-inserted `child` as `parent`'s last child.
    pub(crate) fn attach_child(&mut self, parent: NodeId, child: NodeId) {
        let prev_last = self.arena.get(parent).and_then(|node| node.last_child);
        if let Some(node) = self.arena.get_mut(child) {
            node.parent = Some(parent);
            node.prev_sibling = prev_last;
            node.next_sibling = None;
        }
        if let Some(previous) = prev_last.and_then(|id| self.arena.get_mut(id)) {
            previous.next_sibling = Some(child);
        }
        if let Some(node) = self.arena.get_mut(parent) {
            if node.first_child.is_none() {
                node.first_child = Some(child);
            }
            node.last_child = Some(child);
        }
    }

    /// 🍃️ Inserts `node` unparented and unlinked — the reconcile links it in a later step.
    pub(crate) fn insert_detached(&mut self, node: Node) -> NodeId {
        self.arena.insert(node)
    }

    /// 🧹️ Frees one already-unlinked slot. Unlike [`UiTree::remove`] this never recurses, because a
    /// relink pass has already cleared the node's children.
    pub(crate) fn remove_detached(&mut self, id: NodeId) {
        if self.root == Some(id) {
            self.root = None;
        }
        drop(self.arena.remove(id));
    }

    /// 🧹️ Retires one document identity binding per call, freeing the arena slot it named — the
    /// retirement half of the ledger, driven by `Ui::close_document_step` so a surface that drops its
    /// document does not keep its records' arena nodes alive. `true` once the ledger is empty.
    pub(crate) fn close_document_binding_step(&mut self) -> bool {
        let Some((_, node)) = self.document_nodes.pop() else { return true };
        self.clear_links(node);
        self.remove_detached(node);
        false
    }
    //#endregion 🪪️DocumentIdentity

    /// 🔗️ Inserts `node` as the last child of `parent` (or as a root if `parent` is `None` and no
    /// root exists yet), threading the sibling links.
    pub fn insert_child(&mut self, parent: Option<NodeId>, mut node: Node) -> NodeId {
        node.parent = parent;
        let id = self.arena.insert(node);
        match parent {
            Some(parent_id) => {
                let prev_last = self.arena.get(parent_id).and_then(|p| p.last_child);
                if let Some(prev_last_id) = prev_last {
                    if let Some(prev_last_node) = self.arena.get_mut(prev_last_id) {
                        prev_last_node.next_sibling = Some(id);
                    }
                }
                if let Some(child) = self.arena.get_mut(id) {
                    child.prev_sibling = prev_last;
                }
                if let Some(parent_node) = self.arena.get_mut(parent_id) {
                    if parent_node.first_child.is_none() {
                        parent_node.first_child = Some(id);
                    }
                    parent_node.last_child = Some(id);
                }
            }
            None => {
                if self.root.is_none() {
                    self.root = Some(id);
                }
            }
        }
        id
    }

    /// 🧹️ Detaches `id` from its parent/siblings and recursively removes its subtree, freeing every
    /// arena slot involved.
    pub fn remove(&mut self, id: NodeId) {
        let Some(node) = self.arena.get(id) else { return };
        let (parent, prev_sibling, next_sibling) = (node.parent, node.prev_sibling, node.next_sibling);
        let children: Vec<NodeId> = self.children(id).collect();
        for child in children {
            self.remove(child);
        }
        match prev_sibling {
            Some(prev_id) => {
                if let Some(prev) = self.arena.get_mut(prev_id) {
                    prev.next_sibling = next_sibling;
                }
            }
            None => {
                if let Some(parent_id) = parent {
                    if let Some(parent_node) = self.arena.get_mut(parent_id) {
                        parent_node.first_child = next_sibling;
                    }
                }
            }
        }
        match next_sibling {
            Some(next_id) => {
                if let Some(next) = self.arena.get_mut(next_id) {
                    next.prev_sibling = prev_sibling;
                }
            }
            None => {
                if let Some(parent_id) = parent {
                    if let Some(parent_node) = self.arena.get_mut(parent_id) {
                        parent_node.last_child = prev_sibling;
                    }
                }
            }
        }
        if self.root == Some(id) {
            self.root = None;
        }
        self.arena.remove(id);
    }

    /// 🚨️ Sets `flags` on `id` (setting `DIRTY_LAYOUT` implies `DIRTY_PAINT`, since layout changes
    /// always require a repaint), then bubbles `SUBTREE_DIRTY` up the parent chain, stopping at the
    /// first ancestor that already carries it — every ancestor above it is necessarily already
    /// marked too, so walking further is wasted work.
    pub fn mark_dirty(&mut self, id: NodeId, flags: NodeFlags) {
        let mut flags = flags;
        if flags.contains(NodeFlags::DIRTY_LAYOUT) {
            flags.set(NodeFlags::DIRTY_PAINT, true);
        }
        let parent = match self.arena.get_mut(id) {
            Some(node) => {
                node.flags.set(flags, true);
                node.parent
            }
            None => return,
        };
        let mut cursor = parent;
        while let Some(ancestor_id) = cursor {
            let Some(ancestor) = self.arena.get_mut(ancestor_id) else { break };
            if ancestor.flags.contains(NodeFlags::SUBTREE_DIRTY) {
                break;
            }
            ancestor.flags.set(NodeFlags::SUBTREE_DIRTY, true);
            cursor = ancestor.parent;
        }
    }

    /// 🚶️ Iterates the direct children of `id` in tree order via the first-child/next-sibling links.
    pub fn children(&self, id: NodeId) -> impl Iterator<Item = NodeId> + '_ {
        let mut next = self.arena.get(id).and_then(|n| n.first_child);
        std::iter::from_fn(move || {
            let current = next?;
            next = self.arena.get(current).and_then(|n| n.next_sibling);
            Some(current)
        })
    }
}

#[cfg(test)]
#[path = "../../../🧪️tests/🔬️targets-wgpu-tree-unit/🦀️.rs"]
mod tests;
// #endregion tree
