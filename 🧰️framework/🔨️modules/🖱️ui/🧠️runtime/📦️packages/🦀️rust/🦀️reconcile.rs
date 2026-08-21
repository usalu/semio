//! @emoji ♻️ Keyed reconciliation of a [`crate::ComponentTree`] into a minimal transactional
//! [`ui_contract::UiPatch`] — the conversion from the builder-side, id-less, recursive tree
//! [`crate::present`] produces into the flat, id-keyed [`ui_contract`] document every renderer reads.
//!
//! The one property that makes the emitted patches worth anything: identity comes from
//! **`(parent, key)`, never from position**. A [`SurfaceReconciler`] retains its own shadow copy of
//! what the receiver has (mirroring [`ui_contract::UiSnapshotState`]) plus a `(parent, key) → id`
//! index, so a node keeps its [`ui_contract::UiNodeId`] across reorders, insertions and removals of
//! its siblings — which is exactly the property that lets renderer-side state (scroll offset, focus,
//! a DOM node, a GPU cache entry) survive a re-present instead of being torn down and rebuilt every
//! frame the way the old `PatchTracker` full-body-`Replace` stub forced.
//!
//! 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md. Every `fn`
//! below is plain sync by owner ruling U1, which supersedes this program's general async-everything
//! default for exactly this crate.

use std::collections::{HashMap, HashSet};

//#region 🔖️Identity

/// 🔑️ A node's reconciliation identity: which parent it hangs under (`None` only for the root, which
/// has no parent) plus its own sibling `key`. Two [`crate::TreeNode`]s presented on different frames
/// with the same identity are the SAME node as far as reconciliation is concerned, regardless of what
/// position either occupied among its siblings — this is the one invariant every other rule here
/// exists to preserve.
type NodeIdentity = (Option<ui_contract::UiNodeId>, String);

/// 🔑️ `node`'s identity under `parent`, as looked up in / inserted into [`SurfaceReconciler::key_index`].
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn identity_of(parent: Option<ui_contract::UiNodeId>, node: &crate::TreeNode) -> NodeIdentity {
    (parent, node.key.clone())
}

/// 🚨️ Panics naming the first duplicate sibling key found in `children`. [`crate::ComponentTree`]'s
/// own constructor already asserts this on every level of a tree built through `TreeNode::with_children`
/// / `ComponentTree::new`, but `ComponentTree { root }`'s field is `pub`, so a caller can hand this
/// reconciler a tree that skipped that constructor entirely — this is therefore genuine defense in
/// depth, not a redundant re-check, and it is what makes a duplicate key a loud authoring-bug panic
/// here too rather than one key silently shadowing the other during matching.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn assert_unique_child_keys(parent: ui_contract::UiNodeId, children: &[crate::TreeNode]) {
    let mut seen: HashSet<&str> = HashSet::with_capacity(children.len());
    for child in children {
        assert!(seen.insert(child.key.as_str()), "🚫️ duplicate sibling key {:?} under parent {parent:?} — reconciliation keys must be unique among siblings", child.key);
    }
}

//#endregion 🔖️Identity

//#region 🔖️Reconciler

/// ♻️ Keyed differ for one render surface. Owns a shadow copy of what the receiver has (`retained`,
/// `root`) plus the `(parent, key) → id` index (`key_index`) that carries every node's identity across
/// frames, and the monotonic `allocator` that mints an id for a node the first time it is ever seen.
/// [`SurfaceReconciler::reconcile`] is the only place any of these four change together.
#[derive(Debug)]
pub struct SurfaceReconciler {
    surface: ui_contract::SurfaceId,
    revision: ui_contract::UiRevision,
    allocator: ui_contract::UiNodeIdAllocator,
    retained: HashMap<ui_contract::UiNodeId, ui_contract::UiNodeRecord>,
    key_index: HashMap<NodeIdentity, ui_contract::UiNodeId>,
    root: Option<ui_contract::UiNodeId>,
}

impl SurfaceReconciler {
    /// 🌱️ A reconciler for `surface` with no retained state yet — the next [`Self::reconcile`] call
    /// necessarily emits a full `SetRoot` plus one `Upsert` per node, exactly as [`Self::mark_rejected`]
    /// arranges for an existing reconciler to do again.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn new(surface: impl Into<ui_contract::SurfaceId>) -> Self {
        Self { surface: surface.into(), revision: ui_contract::UiRevision::default(), allocator: ui_contract::UiNodeIdAllocator::default(), retained: HashMap::new(), key_index: HashMap::new(), root: None }
    }

    /// ♻️ Diffs `tree` against this reconciler's retained state, mutating that state to match and
    /// returning the minimal [`ui_contract::UiPatch`] that carries the difference — or `None` when
    /// `tree` is structurally and semantically identical to what was last presented, so an idle surface
    /// produces no wire traffic at all. `base_revision` is the revision the receiver is assumed to be
    /// at; `revision` is one past it — this reconciler never emits a gap or a repeat.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn reconcile(&mut self, tree: &crate::ComponentTree) -> Option<ui_contract::UiPatch> {
        let mut ops = Vec::new();
        let previous_root = self.root;
        let new_root_id = self.diff_node(None, &tree.root, &mut ops);

        if previous_root != Some(new_root_id) {
            if let Some(stale_root) = previous_root {
                self.remove_subtree(None, stale_root, &mut ops);
            }
            ops.push(ui_contract::UiPatchOp::SetRoot { id: new_root_id });
            self.root = Some(new_root_id);
        }

        if ops.is_empty() {
            return None;
        }
        let base_revision = self.revision;
        self.revision = self.revision.next();
        Some(ui_contract::UiPatch { surface: self.surface.clone(), base_revision, revision: self.revision, ops })
    }

    /// 📸️ The complete current state as a fresh [`ui_contract::UiSnapshot`] — what a new subscriber
    /// receives instead of a patch stream. `root` falls back to [`ui_contract::UiNodeId::default`] when
    /// nothing has ever been reconciled yet; `nodes` is then empty too, so that sentinel never resolves
    /// to a real record.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn snapshot(&self) -> ui_contract::UiSnapshot {
        ui_contract::UiSnapshot { surface: self.surface.clone(), revision: self.revision, root: self.root.unwrap_or_default(), nodes: self.retained.values().cloned().collect(), layout_epoch: 0 }
    }

    /// 🔄️ Forces the next [`Self::reconcile`] to emit a full re-send — the recovery path for the
    /// existing `patch-rejected` wire event. Drops every retained node and resets the assumed receiver
    /// revision to zero (mirroring the fresh, empty document a rejection leaves the receiver at); the
    /// [`ui_contract::UiNodeIdAllocator`] is deliberately left untouched, so the re-sent nodes get IDs
    /// that continue monotonically rather than reusing any id a stale renderer reference might still
    /// name.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn mark_rejected(&mut self) {
        self.retained.clear();
        self.key_index.clear();
        self.root = None;
        self.revision = ui_contract::UiRevision::default();
    }
}

//#endregion 🔖️Reconciler

//#region 🔖️Diff

impl SurfaceReconciler {
    /// ♻️ Resolves `node`'s identity under `parent` against [`Self::key_index`]: a hit reuses the
    /// existing id and diffs field-by-field via [`Self::diff_existing`]; a miss mints a fresh id via
    /// the allocator and inserts the node wholesale via one `Upsert` — the only two ways any node ever
    /// enters `ops`. Returns the id `node` now has, whichever path was taken.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn diff_node(&mut self, parent: Option<ui_contract::UiNodeId>, node: &crate::TreeNode, ops: &mut Vec<ui_contract::UiPatchOp>) -> ui_contract::UiNodeId {
        let identity = identity_of(parent, node);
        if let Some(&id) = self.key_index.get(&identity) {
            self.diff_existing(id, node, ops);
            id
        } else {
            let id = self.allocator.allocate();
            self.key_index.insert(identity, id);
            let child_ids = self.diff_children(id, &[], &node.children, ops);
            let record = build_record(id, node, child_ids, None);
            self.retained.insert(id, record.clone());
            ops.push(ui_contract::UiPatchOp::Upsert(record));
            id
        }
    }

    /// ♻️ Diffs `node` against the retained record already at `id`, choosing the narrowest
    /// representation of the change. Children are diffed first (post-order), since the parent's own
    /// children list — and therefore whether `SetChildren` fires — depends on ids children may have
    /// only just been minted with. Every one of the eight field groups
    /// (`component`/`layout`/`activity`+`disabled`/`children`/`style`/`accessibility`/`bindings`/
    /// `menu`) now has its own [`ui_contract::UiPatchOp`] setter, so a change touching only one group
    /// always emits exactly that one op, deterministically, with no byte comparison — that determinism
    /// is load-bearing: it is what keeps a same-size reorder (`SetChildren` alone) from ever being
    /// second-guessed into an `Upsert` merely because the wire-cost estimator does not itself price a
    /// record's `children` list. Only once **more than one** group changed does this weigh a full
    /// `Upsert` against the targeted ops it would replace, via [`Self::estimate_bytes`], and picks
    /// whichever is actually smaller on the wire — so `Upsert` is reserved for a genuinely new node
    /// ([`Self::diff_node`]'s other arm) or for a multi-group change so broad that one full record
    /// beats several targeted ops.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn diff_existing(&mut self, id: ui_contract::UiNodeId, node: &crate::TreeNode, ops: &mut Vec<ui_contract::UiPatchOp>) {
        let old = self.retained.get(&id).cloned().expect("🚫️ key_index names an id with no retained record");
        let new_child_ids = self.diff_children(id, &old.children, &node.children, ops);

        let mut targeted = Vec::new();
        if old.component != node.component {
            targeted.push(ui_contract::UiPatchOp::SetComponent { id, component: node.component.clone() });
        }
        if old.layout != node.layout {
            targeted.push(ui_contract::UiPatchOp::SetLayout { id, layout: node.layout.clone() });
        }
        if old.activity != node.activity || old.disabled != node.disabled {
            targeted.push(ui_contract::UiPatchOp::SetActivity { id, activity: node.activity, disabled: node.disabled });
        }
        if old.children != new_child_ids {
            targeted.push(ui_contract::UiPatchOp::SetChildren { id, children: new_child_ids.clone() });
        }
        if old.style != node.style {
            targeted.push(ui_contract::UiPatchOp::SetStyle { id, style: node.style });
        }
        if old.accessibility != node.accessibility {
            targeted.push(ui_contract::UiPatchOp::SetAccessibility { id, accessibility: node.accessibility.clone() });
        }
        if old.bindings != node.bindings {
            targeted.push(ui_contract::UiPatchOp::SetBindings { id, bindings: node.bindings.clone() });
        }
        if old.menu != node.menu {
            targeted.push(ui_contract::UiPatchOp::SetMenu { id, menu: node.menu.clone() });
        }

        if targeted.is_empty() {
            return;
        }

        let record = build_record(id, node, new_child_ids, old.transition);
        let upsert = ui_contract::UiPatchOp::Upsert(record.clone());
        let use_upsert = targeted.len() > 1 && self.estimate_bytes(std::slice::from_ref(&upsert)) < self.estimate_bytes(&targeted);

        self.retained.insert(id, record);
        if use_upsert {
            ops.push(upsert);
        } else {
            ops.extend(targeted);
        }
    }

    /// 💰️ Wire-cost estimate for `candidate_ops`, delegated to [`ui_contract::patch_byte_estimate`]
    /// via a throwaway single-purpose [`ui_contract::UiPatch`] — the byte-accounting logic (including
    /// which fields even count as "text") lives once, in the contract crate that also enforces
    /// `max_patch_bytes`, and is never duplicated here.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn estimate_bytes(&self, candidate_ops: &[ui_contract::UiPatchOp]) -> usize {
        let probe = ui_contract::UiPatch { surface: self.surface.clone(), base_revision: ui_contract::UiRevision::default(), revision: ui_contract::UiRevision::default(), ops: candidate_ops.to_vec() };
        ui_contract::patch_byte_estimate(&probe)
    }

    /// 👶️ Diffs `new_children` against `old_child_ids` under `parent_id`, matching purely by
    /// `(parent_id, key)` — never by position — so reordering, inserting, and removing siblings each
    /// touch only the ids actually affected. Every old child whose id is not among the freshly diffed
    /// ids is removed as a whole subtree. Returns the new children list in `new_children`'s order,
    /// ready to become the parent's own `children` field.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn diff_children(&mut self, parent_id: ui_contract::UiNodeId, old_child_ids: &[ui_contract::UiNodeId], new_children: &[crate::TreeNode], ops: &mut Vec<ui_contract::UiPatchOp>) -> Vec<ui_contract::UiNodeId> {
        assert_unique_child_keys(parent_id, new_children);

        let mut new_ids = Vec::with_capacity(new_children.len());
        for child in new_children {
            new_ids.push(self.diff_node(Some(parent_id), child, ops));
        }

        let retained_ids: HashSet<ui_contract::UiNodeId> = new_ids.iter().copied().collect();
        for &old_id in old_child_ids {
            if !retained_ids.contains(&old_id) {
                self.remove_subtree(Some(parent_id), old_id, ops);
            }
        }

        new_ids
    }

    /// 🗑️ Emits one `Remove` for `id` — the contract's own [`ui_contract::apply_patch`] deletes the
    /// whole subtree on the receiver side — and mirrors that locally via [`Self::purge_subtree`], so
    /// this reconciler's own `retained`/`key_index` never accumulate an orphan for a node the receiver
    /// no longer has either.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn remove_subtree(&mut self, parent: Option<ui_contract::UiNodeId>, id: ui_contract::UiNodeId, ops: &mut Vec<ui_contract::UiPatchOp>) {
        ops.push(ui_contract::UiPatchOp::Remove { id });
        self.purge_subtree(parent, id);
    }

    /// 🧹️ Removes `id` and every node reachable from it via its own retained `children`, purging both
    /// `retained` and `key_index` for each — the local mirror of [`ui_contract::apply_patch`]'s
    /// `remove_subtree`. `id` is never handed back to the allocator, so it can never be reused.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn purge_subtree(&mut self, parent: Option<ui_contract::UiNodeId>, id: ui_contract::UiNodeId) {
        if let Some(record) = self.retained.remove(&id) {
            self.key_index.remove(&(parent, record.key));
            for child_id in record.children {
                self.purge_subtree(Some(id), child_id);
            }
        }
    }
}

/// 🏗️ Assembles a complete [`ui_contract::UiNodeRecord`] for `node` at `id` with `children` already
/// resolved to ids and `transition` carried over verbatim — [`crate::TreeNode`] has no `transition`
/// field of its own (see `🦀️present.rs`'s module doc: it is builder-side and never diffs against a
/// previous tree), so this reconciler is the one place a record's `transition` is set, and it never
/// invents one: `None` for a freshly seen node, whatever the retained record already carried for an
/// existing one. Driving `Introducing`/`Celebrating` from presence data is out of this packet's scope.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn build_record(id: ui_contract::UiNodeId, node: &crate::TreeNode, children: Vec<ui_contract::UiNodeId>, transition: Option<ui_contract::TransitionHint>) -> ui_contract::UiNodeRecord {
    ui_contract::UiNodeRecord {
        id,
        key: node.key.clone(),
        component: node.component.clone(),
        layout: node.layout.clone(),
        style: node.style,
        activity: node.activity,
        disabled: node.disabled,
        transition,
        accessibility: node.accessibility.clone(),
        bindings: node.bindings.clone(),
        menu: node.menu.clone(),
        children,
    }
}

//#endregion 🔖️Diff

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    //#region 🔖️Fixtures
    fn leaf(key: &str) -> crate::TreeNode {
        crate::TreeNode::new(key, ui_contract::Component::Separator(ui_contract::SeparatorProps {}))
    }

    fn text(key: &str, value: &str) -> crate::TreeNode {
        crate::TreeNode::new(key, ui_contract::Component::Text(ui_contract::TextProps { value: ui_contract::Label::from(value), emphasize: None, data_attributes: None }))
    }

    fn container(key: &str, children: Vec<crate::TreeNode>) -> crate::TreeNode {
        crate::TreeNode::new(key, ui_contract::Component::Container(ui_contract::ContainerProps { role: ui_contract::ContainerRole::Plain, label: None, description: None, required: None, error: None, default_open: None, drop_overlay: None }))
            .with_children(children)
    }

    fn tree(root: crate::TreeNode) -> crate::ComponentTree {
        crate::ComponentTree::new(root)
    }

    fn styled(node: crate::TreeNode, tone: ui_contract::Tone) -> crate::TreeNode {
        crate::TreeNode { style: ui_contract::StyleSpec { tone, ..Default::default() }, ..node }
    }

    fn with_shortcut(node: crate::TreeNode, shortcut: &str) -> crate::TreeNode {
        crate::TreeNode { accessibility: ui_contract::AccessibilitySpec { shortcut: Some(shortcut.into()), ..Default::default() }, ..node }
    }

    fn with_binding(node: crate::TreeNode, scope: &str, name: &str) -> crate::TreeNode {
        crate::TreeNode { bindings: vec![ui_contract::ActionBinding { trigger: ui_contract::Trigger::Activate, action: ui_contract::ActionId::v1(scope, name), args: None, capability: None }], ..node }
    }

    fn with_menu(node: crate::TreeNode, menu_id: &str) -> crate::TreeNode {
        crate::TreeNode { menu: Some(ui_contract::MenuRef { id: menu_id.into(), args: None }), ..node }
    }

    fn id_of(snapshot: &ui_contract::UiSnapshot, key: &str) -> ui_contract::UiNodeId {
        snapshot.nodes.iter().find(|record| record.key == key).unwrap_or_else(|| panic!("no node keyed {key:?} in snapshot")).id
    }

    fn assert_snapshot_matches_state(snapshot: &ui_contract::UiSnapshot, state: &ui_contract::UiSnapshotState) {
        assert_eq!(snapshot.revision, state.revision);
        assert_eq!(Some(snapshot.root), state.root);
        assert_eq!(snapshot.nodes.len(), state.nodes.len(), "snapshot/state node-count mismatch");
        for record in &snapshot.nodes {
            assert_eq!(state.nodes.get(&record.id), Some(record), "record {:?} diverges between snapshot and applied state", record.id);
        }
    }
    //#endregion 🔖️Fixtures

    //#region 🔖️FirstReconcileAndIdempotence
    #[test]
    fn first_reconcile_emits_set_root_and_one_upsert_per_node_then_is_idempotent() {
        let mut reconciler = SurfaceReconciler::new("s");
        let component_tree = tree(container("root", vec![leaf("a"), leaf("b")]));

        let patch = reconciler.reconcile(&component_tree).expect("first reconcile must emit a patch");
        assert_eq!(patch.base_revision, ui_contract::UiRevision(0));
        assert_eq!(patch.revision, ui_contract::UiRevision(1));
        assert_eq!(patch.ops.iter().filter(|op| matches!(op, ui_contract::UiPatchOp::Upsert(_))).count(), 3);
        assert_eq!(patch.ops.iter().filter(|op| matches!(op, ui_contract::UiPatchOp::SetRoot { .. })).count(), 1);

        assert!(reconciler.reconcile(&component_tree).is_none(), "an unchanged tree must emit no patch");
    }
    //#endregion 🔖️FirstReconcileAndIdempotence

    //#region 🔖️TargetedOps
    #[test]
    fn changing_one_leaf_text_emits_exactly_one_op_naming_exactly_that_node() {
        let mut reconciler = SurfaceReconciler::new("s");
        reconciler.reconcile(&tree(container("root", vec![text("a", "hello"), leaf("b")]))).unwrap();
        let target_id = id_of(&reconciler.snapshot(), "a");

        let patch = reconciler.reconcile(&tree(container("root", vec![text("a", "world"), leaf("b")]))).expect("a changed leaf must emit a patch");
        assert_eq!(patch.ops.len(), 1, "exactly one op expected, got {:?}", patch.ops);
        match &patch.ops[0] {
            ui_contract::UiPatchOp::SetComponent { id, component } => {
                assert_eq!(*id, target_id);
                assert_eq!(component, &ui_contract::Component::Text(ui_contract::TextProps { value: ui_contract::Label::from("world"), emphasize: None, data_attributes: None }));
            }
            other => panic!("expected SetComponent (not Upsert), got {other:?}"),
        }
    }

    #[test]
    fn reordering_siblings_preserves_every_id_and_emits_only_set_children() {
        let mut reconciler = SurfaceReconciler::new("s");
        reconciler.reconcile(&tree(container("root", vec![leaf("a"), leaf("b"), leaf("c")]))).unwrap();
        let before = reconciler.snapshot();
        let (a_id, b_id, c_id) = (id_of(&before, "a"), id_of(&before, "b"), id_of(&before, "c"));

        let patch = reconciler.reconcile(&tree(container("root", vec![leaf("c"), leaf("a"), leaf("b")]))).expect("a reorder must emit a patch");
        assert_eq!(patch.ops.len(), 1);
        match &patch.ops[0] {
            ui_contract::UiPatchOp::SetChildren { children, .. } => assert_eq!(children, &vec![c_id, a_id, b_id]),
            other => panic!("expected SetChildren, got {other:?}"),
        }

        let after = reconciler.snapshot();
        assert_eq!(id_of(&after, "a"), a_id);
        assert_eq!(id_of(&after, "b"), b_id);
        assert_eq!(id_of(&after, "c"), c_id);
    }

    #[test]
    fn inserting_a_middle_sibling_preserves_the_others_ids() {
        let mut reconciler = SurfaceReconciler::new("s");
        reconciler.reconcile(&tree(container("root", vec![leaf("a"), leaf("c")]))).unwrap();
        let before = reconciler.snapshot();
        let (a_id, c_id) = (id_of(&before, "a"), id_of(&before, "c"));

        let patch = reconciler.reconcile(&tree(container("root", vec![leaf("a"), leaf("b"), leaf("c")]))).expect("an insertion must emit a patch");
        assert!(patch.ops.iter().any(|op| matches!(op, ui_contract::UiPatchOp::Upsert(record) if record.key == "b")));

        let after = reconciler.snapshot();
        assert_eq!(id_of(&after, "a"), a_id);
        assert_eq!(id_of(&after, "c"), c_id);
    }

    #[test]
    fn changed_component_with_unchanged_layout_emits_set_component_not_upsert() {
        let mut reconciler = SurfaceReconciler::new("s");
        reconciler.reconcile(&tree(container("root", vec![leaf("a")]))).unwrap();

        let patch = reconciler.reconcile(&tree(container("root", vec![text("a", "now text")]))).expect("a component change must emit a patch");
        assert_eq!(patch.ops.len(), 1);
        assert!(matches!(patch.ops[0], ui_contract::UiPatchOp::SetComponent { .. }), "expected SetComponent, got {:?}", patch.ops[0]);
    }

    /// 🎨️ The finding this packet exists to fix: a style-only change on a leaf with everything else
    /// unchanged must emit exactly one `SetStyle`, never a whole-node `Upsert`.
    #[test]
    fn changing_only_style_emits_exactly_one_set_style_not_upsert() {
        let mut reconciler = SurfaceReconciler::new("s");
        reconciler.reconcile(&tree(container("root", vec![leaf("a")]))).unwrap();
        let target_id = id_of(&reconciler.snapshot(), "a");

        let patch = reconciler.reconcile(&tree(container("root", vec![styled(leaf("a"), ui_contract::Tone::Danger)]))).expect("a style change must emit a patch");
        assert_eq!(patch.ops.len(), 1, "exactly one op expected, got {:?}", patch.ops);
        match &patch.ops[0] {
            ui_contract::UiPatchOp::SetStyle { id, style } => {
                assert_eq!(*id, target_id);
                assert_eq!(style.tone, ui_contract::Tone::Danger);
            }
            other => panic!("expected SetStyle (not Upsert), got {other:?}"),
        }
    }

    #[test]
    fn changing_only_accessibility_emits_exactly_one_set_accessibility_not_upsert() {
        let mut reconciler = SurfaceReconciler::new("s");
        reconciler.reconcile(&tree(container("root", vec![leaf("a")]))).unwrap();
        let target_id = id_of(&reconciler.snapshot(), "a");

        let patch = reconciler.reconcile(&tree(container("root", vec![with_shortcut(leaf("a"), "Ctrl+S")]))).expect("an accessibility change must emit a patch");
        assert_eq!(patch.ops.len(), 1, "exactly one op expected, got {:?}", patch.ops);
        match &patch.ops[0] {
            ui_contract::UiPatchOp::SetAccessibility { id, accessibility } => {
                assert_eq!(*id, target_id);
                assert_eq!(accessibility.shortcut.as_deref(), Some("Ctrl+S"));
            }
            other => panic!("expected SetAccessibility (not Upsert), got {other:?}"),
        }
    }

    #[test]
    fn changing_only_bindings_emits_exactly_one_set_bindings_not_upsert() {
        let mut reconciler = SurfaceReconciler::new("s");
        reconciler.reconcile(&tree(container("root", vec![leaf("a")]))).unwrap();
        let target_id = id_of(&reconciler.snapshot(), "a");

        let patch = reconciler.reconcile(&tree(container("root", vec![with_binding(leaf("a"), "scope", "name")]))).expect("a bindings change must emit a patch");
        assert_eq!(patch.ops.len(), 1, "exactly one op expected, got {:?}", patch.ops);
        match &patch.ops[0] {
            ui_contract::UiPatchOp::SetBindings { id, bindings } => {
                assert_eq!(*id, target_id);
                assert_eq!(bindings.len(), 1);
            }
            other => panic!("expected SetBindings (not Upsert), got {other:?}"),
        }
    }

    #[test]
    fn changing_only_menu_emits_exactly_one_set_menu_not_upsert() {
        let mut reconciler = SurfaceReconciler::new("s");
        reconciler.reconcile(&tree(container("root", vec![leaf("a")]))).unwrap();
        let target_id = id_of(&reconciler.snapshot(), "a");

        let patch = reconciler.reconcile(&tree(container("root", vec![with_menu(leaf("a"), "menu")]))).expect("a menu change must emit a patch");
        assert_eq!(patch.ops.len(), 1, "exactly one op expected, got {:?}", patch.ops);
        match &patch.ops[0] {
            ui_contract::UiPatchOp::SetMenu { id, menu } => {
                assert_eq!(*id, target_id);
                assert_eq!(menu.as_ref().map(|menu| menu.id.as_str()), Some("menu"));
            }
            other => panic!("expected SetMenu (not Upsert), got {other:?}"),
        }
    }

    /// 💰️ Once several groups change at once, [`SurfaceReconciler::estimate_bytes`] weighs a single
    /// `Upsert` against the pile of targeted ops it would replace — here five groups change on a leaf
    /// whose new component/accessibility still carry no real text, so the targeted ops' fixed per-op
    /// overhead alone outweighs one full-record `Upsert`, and `Upsert` wins.
    #[test]
    fn changing_several_groups_at_once_prefers_a_single_upsert_over_many_targeted_ops() {
        let mut reconciler = SurfaceReconciler::new("s");
        reconciler.reconcile(&tree(container("root", vec![leaf("a")]))).unwrap();
        let target_id = id_of(&reconciler.snapshot(), "a");

        let mut changed =
            crate::TreeNode::new("a", ui_contract::Component::Container(ui_contract::ContainerProps { role: ui_contract::ContainerRole::Plain, label: None, description: None, required: None, error: None, default_open: None, drop_overlay: None }));
        changed.style = ui_contract::StyleSpec { tone: ui_contract::Tone::Danger, ..Default::default() };
        changed.activity = ui_contract::Activity::Loading;
        changed.disabled = true;
        changed.accessibility = ui_contract::AccessibilitySpec { hidden: true, ..Default::default() };

        let patch = reconciler.reconcile(&tree(container("root", vec![changed]))).expect("a multi-group change must emit a patch");
        assert_eq!(patch.ops.len(), 1, "expected one Upsert to beat several targeted ops, got {:?}", patch.ops);
        match &patch.ops[0] {
            ui_contract::UiPatchOp::Upsert(record) => assert_eq!(record.id, target_id),
            other => panic!("expected Upsert, got {other:?}"),
        }
    }
    //#endregion 🔖️TargetedOps

    //#region 🔖️Removal
    #[test]
    fn removing_a_subtree_emits_one_remove_and_leaves_no_orphan_in_retained() {
        let mut reconciler = SurfaceReconciler::new("s");
        reconciler.reconcile(&tree(container("root", vec![container("mid", vec![leaf("x"), leaf("y")]), leaf("z")]))).unwrap();
        let mid_id = id_of(&reconciler.snapshot(), "mid");

        let patch = reconciler.reconcile(&tree(container("root", vec![leaf("z")]))).expect("a removal must emit a patch");
        let removes: Vec<_> = patch.ops.iter().filter(|op| matches!(op, ui_contract::UiPatchOp::Remove { .. })).collect();
        assert_eq!(removes.len(), 1);
        assert!(matches!(removes[0], ui_contract::UiPatchOp::Remove { id } if *id == mid_id));

        let after = reconciler.snapshot();
        assert!(!after.nodes.iter().any(|record| matches!(record.key.as_str(), "mid" | "x" | "y")), "removed subtree must leave no orphan");
        assert_eq!(after.nodes.len(), 2, "only root and z should remain");
    }

    #[test]
    fn ids_are_never_reused_after_removal() {
        let mut reconciler = SurfaceReconciler::new("s");
        reconciler.reconcile(&tree(container("root", vec![leaf("a")]))).unwrap();
        let removed_id = id_of(&reconciler.snapshot(), "a");

        reconciler.reconcile(&tree(container("root", vec![]))).unwrap();
        reconciler.reconcile(&tree(container("root", vec![leaf("a")]))).unwrap();
        let reinserted_id = id_of(&reconciler.snapshot(), "a");

        assert_ne!(reinserted_id, removed_id, "a fresh node at a previously-used key must never reuse a removed id");
    }
    //#endregion 🔖️Removal

    //#region 🔖️Rejection
    #[test]
    fn mark_rejected_then_reconcile_emits_a_full_resend() {
        let mut reconciler = SurfaceReconciler::new("s");
        let component_tree = tree(container("root", vec![leaf("a"), leaf("b")]));
        reconciler.reconcile(&component_tree).unwrap();
        assert!(reconciler.reconcile(&component_tree).is_none());

        reconciler.mark_rejected();
        let patch = reconciler.reconcile(&component_tree).expect("resend after rejection must emit a patch");
        assert_eq!(patch.base_revision, ui_contract::UiRevision(0));
        assert_eq!(patch.ops.iter().filter(|op| matches!(op, ui_contract::UiPatchOp::Upsert(_))).count(), 3);
        assert!(patch.ops.iter().any(|op| matches!(op, ui_contract::UiPatchOp::SetRoot { .. })));
    }
    //#endregion 🔖️Rejection

    //#region 🔖️DuplicateKeys
    #[test]
    fn duplicate_sibling_keys_are_reported_even_when_component_tree_new_is_bypassed() {
        let mut reconciler = SurfaceReconciler::new("s");
        let root = crate::TreeNode { children: vec![leaf("a"), leaf("a")], ..leaf("root") };
        let component_tree = crate::ComponentTree { root };

        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| reconciler.reconcile(&component_tree)));
        assert!(result.is_err(), "a duplicate sibling key must panic, not silently shadow");
    }
    //#endregion 🔖️DuplicateKeys

    //#region 🔖️RoundTripProperty
    /// 🔁️ The property that matters most: every patch this reconciler ever emits must apply cleanly
    /// through the contract's own [`ui_contract::apply_patch`], and doing so must reproduce
    /// [`SurfaceReconciler::snapshot`] exactly. Exercised across a sequence of trees that each mutate a
    /// different axis (reorder, insert, remove, text change, nested restructure, collapse-to-one-child,
    /// style-only, accessibility-only, bindings-only, menu-only, and a multi-group change that should
    /// fall back to `Upsert`) so this one test would catch a producer/consumer disagreement in any of
    /// them — including the four field-targeted ops this packet adds.
    #[test]
    fn round_trip_property_every_emitted_patch_applies_cleanly_and_reproduces_the_snapshot() {
        let mut reconciler = SurfaceReconciler::new("s");
        let mut receiver_state = ui_contract::UiSnapshotState::new(ui_contract::SurfaceId::from("s"));
        let limits = ui_contract::UiDocumentLimits::default();

        let frames = vec![
            tree(container("root", vec![leaf("a"), leaf("b")])),
            tree(container("root", vec![leaf("b"), leaf("a"), text("c", "hi")])),
            tree(container("root", vec![text("c", "bye"), container("mid", vec![leaf("d")])])),
            tree(container("root", vec![container("mid", vec![leaf("d"), leaf("e")])])),
            tree(container("root", vec![leaf("solo")])),
            tree(container("root", vec![styled(leaf("solo"), ui_contract::Tone::Primary)])),
            tree(container("root", vec![with_shortcut(styled(leaf("solo"), ui_contract::Tone::Primary), "Ctrl+K")])),
            tree(container("root", vec![with_binding(with_shortcut(styled(leaf("solo"), ui_contract::Tone::Primary), "Ctrl+K"), "scope", "name")])),
            tree(container("root", vec![with_menu(with_binding(with_shortcut(styled(leaf("solo"), ui_contract::Tone::Primary), "Ctrl+K"), "scope", "name"), "menu")])),
            tree(container("root", vec![leaf("solo"), leaf("solo2")])),
        ];

        for component_tree in &frames {
            if let Some(patch) = reconciler.reconcile(component_tree) {
                ui_contract::apply_patch(&mut receiver_state, &patch, &limits).expect("every emitted patch must apply cleanly against the contract's own apply_patch");
                ui_contract::validate_snapshot(&reconciler.snapshot(), &limits).expect("every reconciled state must remain a valid document");
            }
            assert_snapshot_matches_state(&reconciler.snapshot(), &receiver_state);
        }
    }
    //#endregion 🔖️RoundTripProperty
}
//#endregion 🧪️Tests
