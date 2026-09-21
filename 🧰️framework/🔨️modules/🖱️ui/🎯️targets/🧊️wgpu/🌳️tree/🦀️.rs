// #region tree
//! 🌲️ Retained scene-graph: one `UiTree` per window, holding `Node`s in a generational `Arena`
//! with parent/first-child/last-child/sibling links and dirty-flag propagation. The engine facade
//! (a later milestone) holds `HashMap<window_id, UiTree>`.

use crate::wgpu::arena::{Arena, NodeId};
use crate::wgpu::component::ui::{UiNode, UiTreeItemNode, UiTreeSectionNode};
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
    Credits,
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

    pub(crate) fn node_count(&self) -> usize {
        self.node_count
    }

    /// 🪙 Starts an independently credited copy of this admitted document. Records are copied by
    /// [`UiDocumentTree::credited_record_at`] one bounded unit at a time; this header alone owns no
    /// component, binding, or menu payload credits.
    pub(crate) fn credited_baseline(&self) -> Self {
        Self {
            generation: self.generation,
            surface: self.surface.clone(),
            revision: self.revision,
            root: self.root,
            layout_epoch: self.layout_epoch,
            node_count: self.node_count,
            nodes: UiNodeTable::default(),
        }
    }

    /// 🪙 Copies one record through the contract's explicit credit-accounting seam. A record is
    /// never duplicated with `Clone`: component values, action bindings, and menu references must
    /// each acquire their own bounded owner or the whole candidate copy is refused.
    pub(crate) fn credited_record_at(&self, index: usize) -> Result<UiNodeRecord, UiDocumentTreeFault> {
        self.nodes.get_index(index).ok_or(UiDocumentTreeFault::Count)?.credited_clone().ok_or(UiDocumentTreeFault::Credits)
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
    /// ⌨️ CSS `:focus-visible` in one bit: this node holds focus AND the gesture that gave it focus
    /// was a KEY, not a pointer. React gets the distinction free from the browser's own heuristic;
    /// `EventRouter` reproduces it by stamping this alongside [`NodeFlags::FOCUSED`] only while its
    /// `focus_visible` latch is set (set on `KeyDown`, cleared on `PointerDown`), so a clicked
    /// control takes focus without painting the accent ring React would only show for the keyboard.
    pub const FOCUS_VISIBLE: NodeFlags = NodeFlags(1 << 13);
    pub(crate) const DISCLOSURE_CHANGED: NodeFlags = NodeFlags(1 << 14);

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
    pub disclosure_open: Option<bool>,
    /// ⌨️ Which option row of an OPEN `Select` the keyboard currently highlights — React's
    /// `aria-activedescendant`/`data-highlighted` row (`🧱️elements/🔽️Select/🟦️.tsx`'s `activeId`).
    /// Distinct from `NodeFlags::HOVERED` (a pointer fact) and from the Select's own committed
    /// `value`: arrowing through a popup moves this and nothing else, and only `Enter` commits.
    /// Written by `events::EventRouter::route_select_key`, read by `paint`'s popup rows.
    pub highlighted: Option<usize>,
    pub(crate) select_popup: Option<crate::wgpu::select::SelectPopupGeometry>,
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
    pub(crate) component_generation: u64,
    /// 📐️ The AUTHORED layout for this node — `Some` exactly when a `UiNodeRecord` published one
    /// (`record.layout`, the same value React's `layoutSpecStyle` reads), `None` for the legacy
    /// declarative `UiNode` chrome path that carries no layout vocabulary of its own. `flex` picks
    /// its dialect from this presence; see that region's header for why the two differ.
    pub layout_spec: Option<ui_contract::LayoutSpec>,
    pub state: WidgetState,
    pub layout: LayoutBucket,
    mounted_layout: [MountedLayoutRecord; 2],
    pub paint: PaintBucket,
    pub flags: NodeFlags,
    /// 🎬️ What this node dispatches THROUGH: its published address plus one versioned `ActionId` per
    /// bound `Trigger`, stamped by the document reconcile's mount step and re-stamped on every
    /// revision. `None` for a node no document published (the chrome's immediate-mode widgets, the
    /// testkit's declarative `apply_tree` path) — such a node has no surface/revision to be stale
    /// against and keeps firing the bare `ActionDescriptor` its spec carries, exactly as React keeps
    /// `onAction` for its own unowned scene hosts.
    pub intent: Option<crate::wgpu::UiIntentBindings>,
}

impl Node {
    pub fn new(key: NodeKey, spec: WidgetSpec) -> Self {
        let component_generation = u64::from(matches!(&spec.0, UiNode::ComponentScene(_)));
        let disclosure_open = match &spec.0 {
            UiNode::Section(section) => Some(section.default_open.unwrap_or(true)),
            _ => None,
        };
        Self {
            parent: None,
            first_child: None,
            last_child: None,
            prev_sibling: None,
            next_sibling: None,
            key,
            spec,
            component_generation,
            layout_spec: None,
            state: WidgetState { disclosure_open, ..WidgetState::default() },
            layout: LayoutBucket::default(),
            mounted_layout: [MountedLayoutRecord::default(); 2],
            paint: PaintBucket,
            flags: NodeFlags::empty(),
            intent: None,
        }
    }

    pub const fn component_generation(&self) -> u64 {
        self.component_generation
    }

    pub(crate) fn interaction_identity_matches(&self, other: &Self) -> bool {
        self.key == other.key
            && self.component_generation == other.component_generation
            && std::mem::discriminant(&self.spec.0) == std::mem::discriminant(&other.spec.0)
            && match (&self.spec.0, &other.spec.0) {
                (UiNode::ComponentScene(left), UiNode::ComponentScene(right)) => left.host_id == right.host_id,
                _ => true,
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
    /// 🪟️ Where each OPEN floating overlay's content root sits this frame: the absolute
    /// (window-local) top-left `events::resolve_overlay_placement_side` settled on for it, written
    /// once per frame by `engine::Ui`'s paint entry and read by EVERY geometry walk — the retained
    /// paint/scene/hit walk AND `events::hit_test`/`absolute_rect`. It lives on the TREE rather than
    /// on the `EventRouter` precisely so the free-standing geometry functions can honour it without a
    /// new parameter: an overlay that paints at its placement but hit-tests at its in-flow position
    /// is the one failure mode a floating surface must not have.
    overlay_origins: Vec<(NodeId, f32, f32)>,
    /// 🔽️ Arena nodes this target SYNTHESIZED under a composite widget rather than mounting from a
    /// document record — today exactly one kind: the option rows an OPEN `Select` needs so that the
    /// generic arena (paint, hit registry, accessibility projection, any tree walker) sees the popup
    /// React's `SelectContent` mounts as real children. `(owner, row)`, in mint order.
    ///
    /// They are ledgered rather than left in the tree because the document reconcile relinks the
    /// arena from the published record set: it `clear_links`es every document-bound node and
    /// re-attaches the plan, which would orphan a synthesized child forever and leak one arena slot
    /// per republish. [`UiTree::retire_composite_row_step`] drains this ledger before a reconcile,
    /// and `paint::sync_interactive_state_node_step` re-mints for whatever is still open — so the
    /// working set is bounded by [`UI_COMPOSITE_ROWS`] and never grows across frames.
    composite_rows: Vec<(NodeId, NodeId)>,
}

/// 🔽️ The ceiling on synthesized composite rows held at once. One open popup at a time is the shell's
/// own rule, and `RETAINED_SYNC_COLLECTION_ITEMS` caps a single popup at 256 rows, so this leaves
/// room for a handful of simultaneously open ones without ever being unbounded.
pub const UI_COMPOSITE_ROWS: usize = 1_024;

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

    pub(crate) fn explicit_child(&self, parent: NodeId, key: &str) -> Option<NodeId> {
        self.children(parent).find(|child| matches!(self.node(*child).map(|node| &node.key), Some(NodeKey::Explicit(candidate)) if candidate == key))
    }

    pub(crate) fn authored_tree_section(&self, id: NodeId) -> Option<&UiTreeSectionNode> {
        let node = self.node(id)?;
        let UiNode::Tree(owner) = &self.node(node.parent?)?.spec.0 else { return None };
        let NodeKey::Explicit(key) = &node.key else { return None };
        owner.sections.iter().find(|section| &section.id == key)
    }

    pub(crate) fn authored_tree_item(&self, id: NodeId) -> Option<&UiTreeItemNode> {
        let node = self.node(id)?;
        let NodeKey::Explicit(key) = &node.key else { return None };
        let mut ancestor = node.parent;
        let mut depth = 0usize;
        while let Some(candidate) = ancestor {
            if depth >= crate::wgpu::layout::TREE_ROW_MAX_DEPTH {
                return None;
            }
            depth += 1;
            let owner = self.node(candidate)?;
            if let UiNode::Tree(tree) = &owner.spec.0 {
                return tree.sections.iter().find_map(|section| find_tree_item(&section.items, key, 0));
            }
            ancestor = owner.parent;
        }
        None
    }

    pub(crate) fn tree_item_depth(&self, id: NodeId) -> Option<usize> {
        self.authored_tree_item(id)?;
        let mut ancestor = self.node(id)?.parent;
        let mut depth = 1usize;
        while let Some(candidate) = ancestor {
            let owner = self.node(candidate)?;
            if matches!(&owner.spec.0, UiNode::Tree(_)) {
                return Some(depth.saturating_sub(1).max(1));
            }
            depth = depth.checked_add(1)?;
            if depth > crate::wgpu::layout::TREE_ROW_MAX_DEPTH + 1 {
                return None;
            }
            ancestor = owner.parent;
        }
        None
    }

    pub(crate) fn disclosure_open(&self, id: NodeId) -> Option<bool> {
        let node = self.node(id)?;
        if let UiNode::Section(section) = &node.spec.0 {
            return Some(node.state.disclosure_open.unwrap_or(section.default_open.unwrap_or(true)));
        }
        if let Some(section) = self.authored_tree_section(id) {
            return Some(node.state.disclosure_open.unwrap_or(section.default_open.unwrap_or(true)));
        }
        let item = self.authored_tree_item(id)?;
        item.items.as_deref().filter(|items| !items.is_empty())?;
        Some(node.state.disclosure_open.unwrap_or(item.default_open.unwrap_or(false)))
    }

    pub(crate) fn disclosure_is_interactive(&self, id: NodeId) -> bool {
        let Some(node) = self.node(id) else { return false };
        if let UiNode::Section(section) = &node.spec.0 {
            return section.label.is_some();
        }
        if self.authored_tree_section(id).is_some_and(|section| section.label.is_some()) {
            return true;
        }
        self.authored_tree_item(id).and_then(|item| item.items.as_deref()).is_some_and(|items| !items.is_empty())
    }

    pub(crate) fn toggle_disclosure(&mut self, id: NodeId) -> Option<bool> {
        if !self.disclosure_is_interactive(id) {
            return None;
        }
        let open = !self.disclosure_open(id)?;
        self.node_mut(id)?.state.disclosure_open = Some(open);
        self.mark_dirty(id, NodeFlags::DIRTY_LAYOUT);
        if let Some(root) = self.root.and_then(|root| self.node_mut(root)) {
            root.flags.set(NodeFlags::DISCLOSURE_CHANGED, true);
        }
        Some(open)
    }

    pub(crate) fn take_disclosure_changed(&mut self) -> bool {
        let Some(root) = self.root.and_then(|root| self.node_mut(root)) else { return false };
        let changed = root.flags.contains(NodeFlags::DISCLOSURE_CHANGED);
        root.flags.set(NodeFlags::DISCLOSURE_CHANGED, false);
        changed
    }

    pub(crate) fn tree_section_open(&self, tree_id: NodeId, section_id: &str, default_open: bool) -> bool {
        self.children(tree_id)
            .find(|child| matches!(self.node(*child).map(|node| &node.key), Some(NodeKey::Explicit(key)) if key == section_id))
            .and_then(|child| self.disclosure_open(child))
            .unwrap_or(default_open)
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

    /// 🪟️ Republishes this frame's open-overlay placements (see [`UiTree::overlay_origins`]).
    pub(crate) fn set_overlay_origins(&mut self, origins: Vec<(NodeId, f32, f32)>) {
        self.overlay_origins = origins;
    }

    /// 🪟️ The WALK origin an open overlay root replaces its in-flow parent offset with, so that
    /// adding the node's own accepted layout lands it exactly on its resolved placement. `None` for
    /// every node that is not an open overlay root, which is the in-flow rule unchanged.
    pub(crate) fn overlay_walk_origin(&self, id: NodeId) -> Option<(f32, f32)> {
        let (_, x, y) = self.overlay_origins.iter().copied().find(|(node, _, _)| *node == id)?;
        let layout = self.accepted_layout(id)?;
        Some((x - layout.x, y - layout.y))
    }

    /// 📜️ Resolves a parent's content origin from its placed walk origin and live viewport offset.
    pub(crate) fn child_walk_origin(&self, parent: NodeId, origin: (f32, f32)) -> Option<(f32, f32)> {
        let layout = self.accepted_layout(parent)?;
        let node = self.node(parent)?;
        let scroll = if node.flags.contains(NodeFlags::SCROLLABLE) { node.state.scroll_offset } else { (0.0, 0.0) };
        Some((origin.0 + layout.x - scroll.0, origin.1 + layout.y - scroll.1))
    }

    /** @emoji 📐️ One node's ABSOLUTE painted rect: its own accepted layout plus every ancestor's
     * origin, with the accumulation stopping at an OPEN overlay (itself or an ancestor) because
     * everything under a floating surface is positioned against that surface's placement rather than
     * against the document it was authored in. The single geometry answer shared by
     * `events::absolute_rect`, `engine::Ui::scene_at` and the retained paint/hit walk. */
    pub(crate) fn absolute_rect(&self, id: NodeId) -> Option<crate::wgpu::geometry::Rect> {
        let layout = self.accepted_layout(id)?;
        if let Some((origin_x, origin_y)) = self.overlay_walk_origin(id) {
            return Some(crate::wgpu::geometry::Rect::new(origin_x + layout.x, origin_y + layout.y, layout.width, layout.height));
        }
        let mut x = layout.x;
        let mut y = layout.y;
        let mut cursor = self.node(id)?.parent;
        while let Some(parent_id) = cursor {
            let overlay_origin = self.overlay_walk_origin(parent_id);
            let (parent_x, parent_y) = self.child_walk_origin(parent_id, overlay_origin.unwrap_or((0.0, 0.0)))?;
            x += parent_x;
            y += parent_y;
            if overlay_origin.is_some() {
                return Some(crate::wgpu::geometry::Rect::new(x, y, layout.width, layout.height));
            }
            cursor = self.node(parent_id)?.parent;
        }
        Some(crate::wgpu::geometry::Rect::new(x, y, layout.width, layout.height))
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

    pub(crate) fn component_scene_host_is_mounted(&self, host_id: &str) -> bool {
        self.document_nodes.iter().any(|(_, node)| {
            self.node(*node).is_some_and(|node| matches!(&node.spec.0, UiNode::ComponentScene(scene) if scene.host_id == host_id))
        })
    }

    /// 🎞️ Copies one retained interaction record shared by the same authored identity and widget
    /// kind. Candidate structure, specs, layout, paint, and document ownership stay independent.
    /// Returns `true` once `index` is beyond the fixed document ledger.
    pub(crate) fn transfer_interaction_record_from(&mut self, presented: &UiTree, index: usize) -> bool {
        let Some((document_id, target_id)) = self.document_nodes.get(index).copied() else {
            if let Some(root) = self.root {
                self.mark_dirty(root, NodeFlags::DIRTY_LAYOUT);
                self.mark_dirty(root, NodeFlags::DIRTY_PAINT);
            }
            return true;
        };
        let Some(source_id) = presented.document_node(document_id) else { return false };
        let Some(source) = presented.node(source_id) else { return false };
        let Some(target) = self.node_mut(target_id) else { return false };
        if !source.interaction_identity_matches(target) {
            return false;
        }
        target.state = source.state.clone();
        for flag in [NodeFlags::HOVERED, NodeFlags::ACTIVE, NodeFlags::FOCUSED, NodeFlags::FOCUS_VISIBLE, NodeFlags::OVERLAY] {
            target.flags.set(flag, source.flags.contains(flag));
        }
        target.flags.set(NodeFlags::DIRTY_LAYOUT, true);
        target.flags.set(NodeFlags::DIRTY_PAINT, true);
        false
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

    //#region 🔽️CompositeRows
    /// 🔽️ Mints one synthesized child under `owner` and ledgers it. `None` when the ledger is full,
    /// which a caller treats exactly like a row it could not mount — skipped, never a fault.
    pub(crate) fn mint_composite_row(&mut self, owner: NodeId, node: Node) -> Option<NodeId> {
        if self.composite_rows.len() >= UI_COMPOSITE_ROWS || !self.arena.contains(owner) {
            return None;
        }
        let row = self.arena.insert(node);
        self.attach_child(owner, row);
        self.composite_rows.push((owner, row));
        Some(row)
    }

    /// 🔽️ Retires ONE synthesized row — unlinked from its owner and freed. `true` once the ledger is
    /// empty. Driven one row per step so a surface with a long popup open cannot spend a whole frame
    /// budget tearing it down.
    pub(crate) fn retire_composite_row_step(&mut self) -> bool {
        let Some((owner, row)) = self.composite_rows.pop() else { return true };
        self.detach_child(owner, row);
        self.remove(row);
        false
    }

    /// 🔽️ Retires ONE synthesized row belonging to `owner`. `true` once that owner has none left —
    /// what a closing popup drives, so its rows go with it instead of waiting for the next document
    /// reconcile to sweep the whole ledger.
    pub(crate) fn retire_composite_row_of_step(&mut self, owner: NodeId) -> bool {
        let Some(index) = self.composite_rows.iter().rposition(|(candidate, _)| *candidate == owner) else { return true };
        let (_, row) = self.composite_rows.remove(index);
        self.detach_child(owner, row);
        self.remove(row);
        false
    }

    /// 🔽️ How many synthesized rows this tree holds — what a law reads to prove the working set is
    /// bounded and that a closed popup left nothing behind.
    pub(crate) fn composite_row_count(&self) -> usize {
        self.composite_rows.len()
    }

    /// 🔽️ Whether `row` is one synthesized option of a currently open Select popup. These rows
    /// remain children of their trigger so capture and keyed reconciliation share one owner, but
    /// their paint and pointer priority is overlay priority rather than ordinary child priority.
    pub(crate) fn is_open_select_popup_row(&self, row: NodeId) -> bool {
        let Some(owner) = self.composite_rows.iter().find_map(|(owner, candidate)| (*candidate == row).then_some(*owner)) else { return false };
        self.node(owner).is_some_and(|node| matches!(&node.spec.0, UiNode::Select(_)) && node.state.open && node.state.select_popup.is_some())
    }

    /// 🔒️ The still-published Select owner of a synthesized `row`. A captured option uses this to
    /// preserve its owner's rows across a host document refresh; a removed or retyped owner answers
    /// `None`, so the reconcile retires the row and the gesture terminates without dispatch.
    pub(crate) fn surviving_composite_owner(&self, row: NodeId) -> Option<NodeId> {
        let owner = self.composite_rows.iter().find_map(|(owner, candidate)| (*candidate == row).then_some(*owner))?;
        let document_id = self.document_nodes.iter().find_map(|(id, node)| (*node == owner).then_some(*id))?;
        self.document
            .as_ref()
            .and_then(|document| document.record(document_id))
            .filter(|record| matches!(&record.component, ui_contract::Component::Select(_)))
            .map(|_| owner)
    }

    /// 🔽️ Whether `owner` already has its synthesized rows this generation.
    pub(crate) fn composite_rows_of(&self, owner: NodeId) -> usize {
        self.composite_rows.iter().filter(|(candidate, _)| *candidate == owner).count()
    }

    /// 🔒️ Retires one synthesized row outside `preserved_owner`. `true` means only that captured
    /// owner's rows remain, or the ledger is empty.
    pub(crate) fn retire_composite_row_except_step(&mut self, preserved_owner: Option<NodeId>) -> bool {
        let Some(index) = self.composite_rows.iter().rposition(|(owner, _)| Some(*owner) != preserved_owner) else { return true };
        let (owner, row) = self.composite_rows.remove(index);
        self.detach_child(owner, row);
        self.remove(row);
        false
    }

    /// 🔗️ Restores a preserved owner's synthesized child chain after the document relink cleared
    /// its tree links. The fixed ledger order is the popup's existing row order.
    pub(crate) fn reattach_composite_rows(&mut self, owner: NodeId) {
        for index in 0..self.composite_rows.len() {
            let (candidate, row) = self.composite_rows[index];
            if candidate == owner && self.contains(row) {
                self.attach_child(owner, row);
            }
        }
    }

    /// 🔗️ Unlinks `child` from `parent`'s sibling chain without touching `child`'s own subtree — the
    /// inverse of [`UiTree::attach_child`], so a retired composite row leaves the chain it was
    /// appended to exactly as it found it.
    fn detach_child(&mut self, parent: NodeId, child: NodeId) {
        let (prev, next) = match self.arena.get(child) {
            Some(node) if node.parent == Some(parent) => (node.prev_sibling, node.next_sibling),
            _ => return,
        };
        match prev {
            Some(prev) => {
                if let Some(node) = self.arena.get_mut(prev) {
                    node.next_sibling = next;
                }
            }
            None => {
                if let Some(node) = self.arena.get_mut(parent) {
                    node.first_child = next;
                }
            }
        }
        match next {
            Some(next) => {
                if let Some(node) = self.arena.get_mut(next) {
                    node.prev_sibling = prev;
                }
            }
            None => {
                if let Some(node) = self.arena.get_mut(parent) {
                    node.last_child = prev;
                }
            }
        }
        if let Some(node) = self.arena.get_mut(child) {
            node.parent = None;
            node.prev_sibling = None;
            node.next_sibling = None;
        }
    }
    //#endregion 🔽️CompositeRows

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
    pub(crate) fn close_document_binding_step(&mut self, retire_scene: &mut impl FnMut(NodeId, &Node) -> bool) -> bool {
        // 🔽️ Synthesized composite rows go FIRST: they hang off document-bound owners, so freeing the
        // owner before the row would leave the row pointing at a dead slot.
        if !self.retire_composite_row_step() {
            return false;
        }
        let Some((_, node)) = self.document_nodes.last().copied() else { return true };
        let Some(retained) = self.arena.get(node) else {
            self.document_nodes.pop();
            return false;
        };
        if matches!(&retained.spec.0, UiNode::ComponentScene(_)) && !retire_scene(node, retained) {
            return false;
        }
        self.document_nodes.pop();
        self.clear_links(node);
        self.remove_detached(node);
        false
    }
    /// 🧹️ Releases the empty tree's remaining indices after document retirement.
    pub(crate) fn close_storage_step(&mut self, retire_scene: &mut impl FnMut(NodeId, &Node) -> bool) -> bool {
        if self.document.is_some() || !self.document_nodes.is_empty() || !self.composite_rows.is_empty() {
            return false;
        }
        if self.overlay_origins.pop().is_some() {
            return false;
        }
        if self.overlay_origins.capacity() > 0 {
            self.overlay_origins = Vec::new();
            return false;
        }
        if self.document_nodes.capacity() > 0 {
            self.document_nodes = Vec::new();
            return false;
        }
        if self.composite_rows.capacity() > 0 {
            self.composite_rows = Vec::new();
            return false;
        }
        if let Some((id, node)) = self.arena.last() {
            if matches!(&node.spec.0, UiNode::ComponentScene(_)) && !retire_scene(id, node) {
                return false;
            }
            self.clear_links(id);
            self.remove_detached(id);
            return false;
        }
        self.arena.close_vacant_step()
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
    /// always require a repaint), then bubbles `SUBTREE_DIRTY` to the root that owns scheduling.
    /// An accepted generation clears the root before descendant flags, so an already-marked
    /// intermediate ancestor is terminal only while that root still carries a layout obligation.
    pub fn mark_dirty(&mut self, id: NodeId, flags: NodeFlags) {
        let mut flags = flags;
        if flags.contains(NodeFlags::DIRTY_LAYOUT) {
            flags.set(NodeFlags::DIRTY_PAINT, true);
        }
        let root_layout_dirty = self.root.and_then(|root| self.arena.get(root)).is_some_and(|root| root.flags.contains(NodeFlags::DIRTY_LAYOUT) || root.flags.contains(NodeFlags::SUBTREE_DIRTY));
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
            if root_layout_dirty && ancestor.flags.contains(NodeFlags::SUBTREE_DIRTY) {
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

fn find_tree_item<'a>(items: &'a [UiTreeItemNode], key: &str, depth: usize) -> Option<&'a UiTreeItemNode> {
    if depth >= crate::wgpu::layout::TREE_ROW_MAX_DEPTH {
        return None;
    }
    for item in items {
        if item.id == key {
            return Some(item);
        }
        if let Some(found) = item.items.as_deref().and_then(|children| find_tree_item(children, key, depth + 1)) {
            return Some(found);
        }
    }
    None
}

#[cfg(test)]
#[path = "../../../🧪️tests/🔬️targets-wgpu-tree-unit/🦀️.rs"]
mod tests;
// #endregion tree
