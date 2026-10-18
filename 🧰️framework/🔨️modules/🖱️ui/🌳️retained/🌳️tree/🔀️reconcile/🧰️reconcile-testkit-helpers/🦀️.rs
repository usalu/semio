//! 🧰️ Reconcile testkit helpers: the variant discriminant, explicit ids, node keys and children of a record and the testkit `UiTree` accessors. See [`super`].

use super::*;

// #region reconcile
#[cfg(any(test, feature = "testkit"))]
fn variant_discriminant(node: &UiNode) -> u32 {
    match node {
        UiNode::Stack(_) => 0,
        UiNode::Text(_) => 1,
        UiNode::Button(_) => 2,
        UiNode::Separator(_) => 3,
        UiNode::Input(_) => 4,
        UiNode::Select(_) => 5,
        UiNode::Toggle(_) => 6,
        UiNode::KeyValue(_) => 8,
        UiNode::Slider(_) => 9,
        UiNode::NumberStepper(_) => 10,
        UiNode::Ring(_) => 11,
        UiNode::IconSelect(_) => 12,
        UiNode::Progress(_) => 20,
        UiNode::Field(_) => 13,
        UiNode::Section(_) => 14,
        UiNode::Tree(_) => 15,
        UiNode::Image(_) => 16,
        UiNode::ComponentScene(_) => 17,
        UiNode::ExternalSlot(_) => 18,
        UiNode::Group(_) => 19,
    }
}

#[cfg(any(test, feature = "testkit"))]
fn explicit_id(node: &UiNode) -> Option<&str> {
    match node {
        UiNode::Stack(n) => n.id.as_deref(),
        UiNode::Button(n) => n.id.as_deref(),
        UiNode::Input(n) => Some(n.id.as_str()),
        UiNode::Select(n) => Some(n.id.as_str()),
        UiNode::Toggle(n) => Some(n.id.as_str()),
        UiNode::Slider(n) => Some(n.id.as_str()),
        UiNode::NumberStepper(n) => Some(n.id.as_str()),
        UiNode::Ring(n) => Some(n.id.as_str()),
        UiNode::IconSelect(n) => Some(n.id.as_str()),
        UiNode::Progress(n) => Some(n.id.as_str()),
        UiNode::Field(n) => Some(n.id.as_str()),
        UiNode::Section(n) => Some(n.id.as_str()),
        UiNode::Group(n) => Some(n.id.as_str()),
        UiNode::Image(n) => Some(n.id.as_str()),
        UiNode::ComponentScene(n) => Some(n.surface_id.as_str()),
        UiNode::ExternalSlot(n) => Some(n.body_key.as_str()),
        UiNode::Text(_) | UiNode::Separator(_) | UiNode::KeyValue(_) | UiNode::Tree(_) => None,
    }
}

#[cfg(any(test, feature = "testkit"))]
pub(super) fn node_key(node: &UiNode, ordinal: u32) -> NodeKey {
    match explicit_id(node) {
        Some(id) if !id.is_empty() => NodeKey::Explicit(id.to_string()),
        _ => NodeKey::Positional(variant_discriminant(node), ordinal),
    }
}

/// 🌿️ The keyed-diffable children of `node`: `Stack`/`Section`'s own `children`, `Field`'s single
/// `child`, borrowed straight from `node` (no allocation); `Select`/`Tree`'s *synthesized* rows (see
/// `🔖️CompositeExpansion`), freshly built each call since they're derived from non-`UiNode` payload.
/// 🔽️ `open` is the parent's own `tree::WidgetState::open` — a CLOSED `Select` materializes no option
/// rows at all, matching React, whose `SelectContent` mounts only while the dropdown is open. The
/// rows used to be built unconditionally because `WidgetState` had nowhere to record the bit; it does
/// now (`events::toggle_select_popup`/`finish_close` own it), so the gate lives here.
/// Everything else has no nested `UiNode` payload to recurse into. `presence.state == Hidden`
/// children are dropped here — hidden means not rendered at all, so they get no retained node, no
/// layout, no paint, no hit-test; this is the one choke point every caller goes through.
#[cfg(any(test, feature = "testkit"))]
fn children_of(node: &UiNode, open: bool) -> Vec<Cow<'_, UiNode>> {
    let children = match node {
        UiNode::Stack(n) => n.children.iter().map(Cow::Borrowed).collect(),
        UiNode::Section(n) => n.children.iter().map(Cow::Borrowed).collect(),
        UiNode::Group(n) => n.children.iter().map(Cow::Borrowed).collect(),
        UiNode::Field(n) => vec![Cow::Borrowed(n.child.as_ref())],
        UiNode::Select(select) if open => select.items.iter().map(|item| Cow::Owned(select_item_row(select, item))).collect(),
        UiNode::Select(_) => Vec::new(),
        UiNode::Tree(tree_node) => tree_node.sections.iter().map(|section| Cow::Owned(tree_section_row(tree_node, section))).collect(),
        _ => Vec::new(),
    };
    children.into_iter().filter(|child: &Cow<'_, UiNode>| child.presence().visible()).collect()
}

#[cfg(any(test, feature = "testkit"))]
impl UiTree {
    /// 🔁️ Applies an incoming declarative `UiNode` tree to this retained tree: keyed single-pass
    /// child matching, minimal-dirty-flag diffing of matched nodes, insertion of unmatched incoming
    /// children, removal of unmatched existing children. Re-applying an identical tree sets zero
    /// dirty flags anywhere in the tree.
    pub fn apply_tree(&mut self, incoming: &UiNode) {
        let key = node_key(incoming, 0);
        match self.root {
            Some(root_id) if self.node(root_id).map(|n| &n.key) == Some(&key) => {
                self.diff_and_update(root_id, incoming);
                self.reconcile_children(root_id, incoming);
            }
            Some(root_id) => {
                self.remove(root_id);
                self.root = None;
                self.insert_new_root(key, incoming);
            }
            None => self.insert_new_root(key, incoming),
        }
    }

    fn insert_new_root(&mut self, key: NodeKey, incoming: &UiNode) {
        let id = self.insert_child(None, Node::new(key, WidgetSpec(incoming.clone())));
        self.mark_dirty(id, NodeFlags::DIRTY_LAYOUT);
        self.root = Some(id);
        self.reconcile_children(id, incoming);
    }

    fn diff_and_update(&mut self, id: NodeId, incoming: &UiNode) {
        let (needs_layout, needs_paint) = match self.node(id) {
            Some(node) if own_fields_equal(&node.spec.0, incoming) => (false, false),
            Some(node) if layout_affecting_change(&node.spec.0, incoming) => (true, true),
            Some(_) => (false, true),
            None => return,
        };
        if let Some(node) = self.node_mut(id) {
            if let (Some(draft), UiNode::Slider(slider)) = (node.state.slider_draft_value, incoming) {
                let epsilon = if slider.step > 0.0 { slider.step * 0.25 } else { 1e-9 };
                if (draft - slider.value).abs() <= epsilon {
                    node.state.slider_draft_value = None;
                }
            }
            node.spec = WidgetSpec(incoming.clone());
        }
        if needs_layout {
            self.mark_dirty(id, NodeFlags::DIRTY_LAYOUT);
        } else if needs_paint {
            self.mark_dirty(id, NodeFlags::DIRTY_PAINT);
        }
    }

    /// 🚩️ Keeps structural `NodeFlags` that reflect `incoming`'s own shape (not its diff status) in
    /// sync — currently just `HAS_POPUP` on a `Select` with ≥1 item, so a later events/paint milestone
    /// can find "this Select has synthesized option rows ready under it" (see this module's own doc
    /// comment for the `WidgetState` open/closed wiring request that gates actually showing them)
    /// without re-deriving it from `spec.0` itself. Deliberately bypasses `mark_dirty` — direct flag
    /// mutation, no `SUBTREE_DIRTY` bubbling — since this is bookkeeping metadata, not a repaint signal.
    fn sync_composite_flags(&mut self, id: NodeId, incoming: &UiNode) {
        if let UiNode::Select(select) = incoming {
            if let Some(node) = self.node_mut(id) {
                node.flags.set(NodeFlags::HAS_POPUP, !select.items.is_empty());
            }
        }
    }

    fn reconcile_children(&mut self, parent: NodeId, incoming: &UiNode) {
        self.sync_composite_flags(parent, incoming);
        let open = self.node(parent).is_some_and(|node| node.state.open);
        let incoming_children = children_of(incoming, open);
        let existing_children: Vec<NodeId> = self.children(parent).collect();

        let mut existing_by_key: HashMap<NodeKey, NodeId> = HashMap::with_capacity(existing_children.len());
        for child_id in &existing_children {
            if let Some(node) = self.node(*child_id) {
                existing_by_key.insert(node.key.clone(), *child_id);
            }
        }

        let mut used_keys: HashSet<NodeKey> = HashSet::with_capacity(incoming_children.len());
        let mut matched_ids: HashSet<NodeId> = HashSet::with_capacity(incoming_children.len());
        for (ordinal, child) in incoming_children.iter().enumerate() {
            let key = node_key(child, ordinal as u32);
            let matched_id = match existing_by_key.get(&key) {
                Some(existing_id) if !used_keys.contains(&key) => {
                    used_keys.insert(key);
                    self.diff_and_update(*existing_id, child);
                    self.reconcile_children(*existing_id, child);
                    *existing_id
                }
                _ => {
                    let id = self.insert_child(Some(parent), Node::new(key, WidgetSpec(child.clone().into_owned())));
                    self.mark_dirty(id, NodeFlags::DIRTY_LAYOUT);
                    self.reconcile_children(id, child);
                    id
                }
            };
            matched_ids.insert(matched_id);
        }

        for existing_id in existing_children {
            if !matched_ids.contains(&existing_id) {
                self.remove(existing_id);
            }
        }
    }
}
// #endregion reconcile
