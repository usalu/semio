//! 🌿️ Ui tree core and layout: the `UiTree` state, its node access, authored tree items, disclosures, and the accepted, inactive and mounted layout writes. See [`super`].

use super::*;

// #region tree
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
    pub(super) arena: Arena<Node>,
    pub root: Option<NodeId>,
    pub(super) document: Option<UiDocumentTree>,
    pub(super) document_nodes: Vec<(UiNodeId, NodeId)>,
    mounted_layout_active: usize,
    mounted_layout_generation: u64,
    /// 🪟️ Where each OPEN floating overlay's content root sits this frame: the absolute
    /// (window-local) top-left `events::resolve_overlay_placement_side` settled on for it, written
    /// once per frame by `engine::Ui`'s paint entry and read by EVERY geometry walk — the retained
    /// paint/scene/hit walk AND `events::hit_test`/`absolute_rect`. It lives on the TREE rather than
    /// on the `EventRouter` precisely so the free-standing geometry functions can honour it without a
    /// new parameter: an overlay that paints at its placement but hit-tests at its in-flow position
    /// is the one failure mode a floating surface must not have.
    pub(super) overlay_origins: Vec<(NodeId, f32, f32)>,
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
    pub(super) composite_rows: Vec<(NodeId, NodeId)>,
    /// 👥️ Host-localized lines about peers acting on a record, by record key, sorted by key — React's
    /// `UiPresenceOverlayEntry.notes` ("Ada is editing this in time travel"). Presence, never document state: a tree row
    /// paints and announces them after its own description, and a reconcile leaves them alone.
    presence_notes: Vec<(String, String)>,
}

/// 🧱️ The ceiling on synthesized composite rows held at once. One open popup at a time is the shell's
/// own rule, and `RETAINED_SYNC_COLLECTION_ITEMS` caps a single popup at 256 rows, so this leaves
/// room for a handful of simultaneously open ones without ever being unbounded.
pub const UI_COMPOSITE_ROWS: usize = 1_024;

impl UiTree {
    pub fn new() -> Self {
        Self::default()
    }

    /// 🗒️ The peers' note on the record keyed `key`, its lines joined, if any peer acts on it.
    pub fn presence_note(&self, key: &str) -> Option<&str> {
        self.presence_notes.binary_search_by(|(candidate, _)| candidate.as_str().cmp(key)).ok().map(|index| self.presence_notes[index].1.as_str())
    }

    /// ✏️ Replaces the peers' notes (sorted by key, as [`UiTree::presence_note`] searches them) and marks the root for
    /// repaint when they changed; `true` when they did.
    pub fn set_presence_notes(&mut self, notes: &[(String, String)]) -> bool {
        debug_assert!(notes.windows(2).all(|pair| pair[0].0 < pair[1].0), "presence notes are sorted by key, one entry per key");
        if self.presence_notes == notes {
            return false;
        }
        self.presence_notes = notes.to_vec();
        if let Some(root) = self.root {
            self.mark_dirty(root, NodeFlags::DIRTY_PAINT);
        }
        true
    }

    pub fn node(&self, id: NodeId) -> Option<&Node> {
        self.arena.get(id)
    }

    pub fn node_mut(&mut self, id: NodeId) -> Option<&mut Node> {
        self.arena.get_mut(id)
    }

    pub fn explicit_child(&self, parent: NodeId, key: &str) -> Option<NodeId> {
        self.children(parent).find(|child| matches!(self.node(*child).map(|node| &node.key), Some(NodeKey::Explicit(candidate)) if candidate == key))
    }

    pub fn authored_tree_section(&self, id: NodeId) -> Option<&UiTreeSectionNode> {
        let node = self.node(id)?;
        let UiNode::Tree(owner) = &self.node(node.parent?)?.spec.0 else { return None };
        let NodeKey::Explicit(key) = &node.key else { return None };
        owner.sections.iter().find(|section| &section.id == key)
    }

    pub fn authored_tree_item(&self, id: NodeId) -> Option<&UiTreeItemNode> {
        let node = self.node(id)?;
        let NodeKey::Explicit(key) = &node.key else { return None };
        let mut ancestor = node.parent;
        let mut depth = 0usize;
        while let Some(candidate) = ancestor {
            if depth >= TREE_ROW_MAX_DEPTH {
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

    pub fn tree_item_depth(&self, id: NodeId) -> Option<usize> {
        self.authored_tree_item(id)?;
        let mut ancestor = self.node(id)?.parent;
        let mut depth = 1usize;
        while let Some(candidate) = ancestor {
            let owner = self.node(candidate)?;
            if matches!(&owner.spec.0, UiNode::Tree(_)) {
                return Some(depth.saturating_sub(1).max(1));
            }
            depth = depth.checked_add(1)?;
            if depth > TREE_ROW_MAX_DEPTH + 1 {
                return None;
            }
            ancestor = owner.parent;
        }
        None
    }

    pub fn disclosure_open(&self, id: NodeId) -> Option<bool> {
        let node = self.node(id)?;
        if let UiNode::Section(section) = &node.spec.0 {
            return Some(node.state.disclosure_open.unwrap_or(section.default_open.unwrap_or(true)));
        }
        if let Some(section) = self.authored_tree_section(id) {
            return Some(node.state.disclosure_open.unwrap_or_else(|| tree_section_default_open(section)));
        }
        let item = self.authored_tree_item(id).filter(|item| item.has_rows())?;
        Some(node.state.disclosure_open.unwrap_or(item.default_open.unwrap_or(false)))
    }

    pub fn disclosure_is_interactive(&self, id: NodeId) -> bool {
        let Some(node) = self.node(id) else { return false };
        if let UiNode::Section(section) = &node.spec.0 {
            return section.label.is_some();
        }
        if self.authored_tree_section(id).is_some_and(|section| section.label.is_some()) {
            return true;
        }
        self.authored_tree_item(id).is_some_and(UiTreeItemNode::has_rows)
    }

    pub fn toggle_disclosure(&mut self, id: NodeId) -> Option<bool> {
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

    pub fn take_disclosure_changed(&mut self) -> bool {
        let Some(root) = self.root.and_then(|root| self.node_mut(root)) else { return false };
        let changed = root.flags.contains(NodeFlags::DISCLOSURE_CHANGED);
        root.flags.set(NodeFlags::DISCLOSURE_CHANGED, false);
        changed
    }

    pub fn tree_section_open(&self, tree_id: NodeId, section_id: &str, default_open: bool) -> bool {
        self.children(tree_id).find(|child| matches!(self.node(*child).map(|node| &node.key), Some(NodeKey::Explicit(key)) if key == section_id)).and_then(|child| self.disclosure_open(child)).unwrap_or(default_open)
    }

    pub fn accepted_layout(&self, id: NodeId) -> Option<AcceptedLayout> {
        let node = self.arena.get(id)?;
        let mounted = node.mounted_layout[self.mounted_layout_active];
        if self.mounted_layout_generation != 0 && mounted.generation == self.mounted_layout_generation {
            Some(mounted.layout)
        } else {
            Some(AcceptedLayout { x: node.layout.x, y: node.layout.y, width: node.layout.width, height: node.layout.height, inline_suffix_width: node.layout.inline_suffix_width })
        }
    }

    /// 🏷️ Width accepted by layout for a control's shrink-to-content inline suffix.
    pub fn accepted_inline_suffix_width(&self, id: NodeId) -> Option<f32> {
        self.accepted_layout(id).map(|layout| layout.inline_suffix_width)
    }

    pub fn write_inactive_layout(&mut self, id: NodeId, generation: u64, layout: AcceptedLayout) -> bool {
        let inactive = self.mounted_layout_active ^ 1;
        let Some(node) = self.arena.get_mut(id) else { return false };
        node.mounted_layout[inactive] = MountedLayoutRecord { generation, layout };
        true
    }

    /// 📍️ Republishes this frame's open-overlay placements (see [`UiTree::overlay_origins`]).
    pub fn set_overlay_origins(&mut self, origins: Vec<(NodeId, f32, f32)>) {
        self.overlay_origins = origins;
    }

    /// 🧭️ The WALK origin an open overlay root replaces its in-flow parent offset with, so that
    /// adding the node's own accepted layout lands it exactly on its resolved placement. `None` for
    /// every node that is not an open overlay root, which is the in-flow rule unchanged.
    pub fn overlay_walk_origin(&self, id: NodeId) -> Option<(f32, f32)> {
        let (_, x, y) = self.overlay_origins.iter().copied().find(|(node, _, _)| *node == id)?;
        let layout = self.accepted_layout(id)?;
        Some((x - layout.x, y - layout.y))
    }

    /// 📜️ Resolves a parent's content origin from its placed walk origin and live viewport offset.
    pub fn child_walk_origin(&self, parent: NodeId, origin: (f32, f32)) -> Option<(f32, f32)> {
        let layout = self.accepted_layout(parent)?;
        let node = self.node(parent)?;
        let scroll = if node.flags.contains(NodeFlags::SCROLLABLE) { node.state.scroll_offset } else { (0.0, 0.0) };
        Some((origin.0 + layout.x - scroll.0, origin.1 + layout.y - scroll.1))
    }

    /** 🖌️ One node's ABSOLUTE painted rect: its own accepted layout plus every ancestor's
     * origin, with the accumulation stopping at an OPEN overlay (itself or an ancestor) because
     * everything under a floating surface is positioned against that surface's placement rather than
     * against the document it was authored in. The single geometry answer shared by
     * `events::absolute_rect`, `engine::Ui::scene_at` and the retained paint/hit walk. */
    pub fn absolute_rect(&self, id: NodeId) -> Option<semio_framework_ui_theme::geometry::Rect> {
        let layout = self.accepted_layout(id)?;
        if let Some((origin_x, origin_y)) = self.overlay_walk_origin(id) {
            return Some(semio_framework_ui_theme::geometry::Rect::new(origin_x + layout.x, origin_y + layout.y, layout.width, layout.height));
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
                return Some(semio_framework_ui_theme::geometry::Rect::new(x, y, layout.width, layout.height));
            }
            cursor = self.node(parent_id)?.parent;
        }
        Some(semio_framework_ui_theme::geometry::Rect::new(x, y, layout.width, layout.height))
    }

    pub fn commit_inactive_layout(&mut self, generation: u64) {
        self.mounted_layout_active ^= 1;
        self.mounted_layout_generation = generation;
    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn accepted_layout_generation(&self) -> u64 {
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

}
// #endregion tree
