// #region tree
//! 🌲️ Retained scene-graph: one `UiTree` per window, holding `Node`s in a generational `Arena`
//! with parent/first-child/last-child/sibling links and dirty-flag propagation. The engine facade
//! (a later milestone) holds `HashMap<window_id, UiTree>`.

use crate::wgpu::arena::{Arena, NodeId};
use crate::wgpu::component::ui::UiNode;

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
    pub paint: PaintBucket,
    pub flags: NodeFlags,
}

impl Node {
    pub fn new(key: NodeKey, spec: WidgetSpec) -> Self {
        Self { parent: None, first_child: None, last_child: None, prev_sibling: None, next_sibling: None, key, spec, state: WidgetState::default(), layout: LayoutBucket::default(), paint: PaintBucket, flags: NodeFlags::empty() }
    }
}

/// 🌲️ One window's retained scene-graph: a generational arena of `Node`s plus its root.
#[derive(Default)]
pub struct UiTree {
    arena: Arena<Node>,
    pub root: Option<NodeId>,
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

    pub fn contains(&self, id: NodeId) -> bool {
        self.arena.contains(id)
    }

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
mod tests {
    use super::*;
    use crate::wgpu::component::ui::{UiNode, UiPresence, UiTextNode};
    use crate::wgpu::Label;

    fn text(value: &str) -> UiNode {
        UiNode::Text(UiTextNode { value: Label::data(value), emphasize: None, data_attributes: None, presence: UiPresence::default(), menu: None })
    }

    fn leaf(discriminant: u32, ordinal: u32, value: &str) -> Node {
        Node::new(NodeKey::Positional(discriminant, ordinal), WidgetSpec(text(value)))
    }

    #[test]
    fn insert_and_iterate_children_in_order() {
        let mut tree = UiTree::new();
        let root = tree.insert_child(None, leaf(0, 0, "root"));
        let a = tree.insert_child(Some(root), leaf(1, 0, "a"));
        let b = tree.insert_child(Some(root), leaf(1, 1, "b"));
        let grandchild = tree.insert_child(Some(b), leaf(1, 0, "c"));

        let children: Vec<NodeId> = tree.children(root).collect();
        assert_eq!(children, vec![a, b]);
        let grandchildren: Vec<NodeId> = tree.children(b).collect();
        assert_eq!(grandchildren, vec![grandchild]);
        assert_eq!(tree.children(grandchild).count(), 0);
    }

    #[test]
    fn mark_dirty_sets_layout_and_paint_and_bubbles_subtree_dirty_to_root() {
        let mut tree = UiTree::new();
        let root = tree.insert_child(None, leaf(0, 0, "root"));
        let mid = tree.insert_child(Some(root), leaf(1, 0, "mid"));
        let grandchild = tree.insert_child(Some(mid), leaf(1, 0, "leaf"));

        tree.mark_dirty(grandchild, NodeFlags::DIRTY_LAYOUT);

        let grandchild_flags = tree.node(grandchild).unwrap().flags;
        assert!(grandchild_flags.contains(NodeFlags::DIRTY_LAYOUT));
        assert!(grandchild_flags.contains(NodeFlags::DIRTY_PAINT));
        assert!(tree.node(mid).unwrap().flags.contains(NodeFlags::SUBTREE_DIRTY));
        assert!(tree.node(root).unwrap().flags.contains(NodeFlags::SUBTREE_DIRTY));
    }

    #[test]
    fn mark_dirty_stops_bubbling_once_it_hits_an_already_dirty_ancestor() {
        let mut tree = UiTree::new();
        let root = tree.insert_child(None, leaf(0, 0, "root"));
        let mid = tree.insert_child(Some(root), leaf(1, 0, "mid"));
        let leaf_a = tree.insert_child(Some(mid), leaf(1, 0, "a"));
        let leaf_b = tree.insert_child(Some(mid), leaf(1, 1, "b"));

        tree.mark_dirty(leaf_a, NodeFlags::DIRTY_PAINT);
        assert!(tree.node(mid).unwrap().flags.contains(NodeFlags::SUBTREE_DIRTY));
        assert!(tree.node(root).unwrap().flags.contains(NodeFlags::SUBTREE_DIRTY));

        // mid and root already carry SUBTREE_DIRTY; this second call must still end up correct
        // (leaf_b itself dirtied, ancestors still dirtied) even though it stops bubbling at `mid`.
        tree.mark_dirty(leaf_b, NodeFlags::DIRTY_PAINT);
        assert!(tree.node(leaf_b).unwrap().flags.contains(NodeFlags::DIRTY_PAINT));
        assert!(!tree.node(leaf_a).unwrap().flags.contains(NodeFlags::SUBTREE_DIRTY));
        assert!(tree.node(mid).unwrap().flags.contains(NodeFlags::SUBTREE_DIRTY));
        assert!(tree.node(root).unwrap().flags.contains(NodeFlags::SUBTREE_DIRTY));
    }

    #[test]
    fn remove_detaches_node_and_frees_its_children_slots() {
        let mut tree = UiTree::new();
        let root = tree.insert_child(None, leaf(0, 0, "root"));
        let mid = tree.insert_child(Some(root), leaf(1, 0, "mid"));
        let grandchild = tree.insert_child(Some(mid), leaf(1, 0, "leaf"));

        tree.remove(mid);

        assert!(!tree.contains(mid));
        assert!(!tree.contains(grandchild));
        assert!(tree.contains(root));
        assert_eq!(tree.children(root).count(), 0);
    }
}
// #endregion tree
