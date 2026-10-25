//! 🔮️ Reconcile oracle: the testkit `SurfaceReconcileOracle` that replays a reconcile against an independent shadow. See [`super`].

use super::*;

//#region 🔖️Diff
#[cfg(any(test, feature = "testkit"))]
pub(super) struct SurfaceReconcileOracle {
    pub(super) surface: ui_contract::SurfaceId,
    pub(super) revision: ui_contract::UiRevision,
    pub(super) allocator: ui_contract::UiNodeIdAllocator,
    pub(super) root: Option<ui_contract::UiNodeId>,
    pub(super) retained: SurfaceLinearMap<ui_contract::UiNodeId, ui_contract::UiNodeRecord, SURFACE_RECONCILE_FIXED_NODES>,
    pub(super) key_index: SurfaceLinearMap<NodeIdentity, ui_contract::UiNodeId, SURFACE_RECONCILE_FIXED_NODES>,
}

#[cfg(any(test, feature = "testkit"))]
impl SurfaceReconcileOracle {
    pub(super) fn from_current(current: &SurfaceShadow) -> Self {
        let mut retained = SurfaceLinearMap::default();
        let mut snapshot = current.snapshot();
        while let Some(record) = snapshot.nodes.pop() { retained.try_insert(record.id, record).unwrap(); }
        let mut key_index = SurfaceLinearMap::default();
        for index in 0..current.key_index.len() {
            let (identity, id) = current.key_index.get_index(index).unwrap();
            key_index.try_insert(identity.clone(), *id).unwrap();
        }
        Self { surface: current.surface.clone(), revision: current.revision, allocator: current.allocator.clone(), root: current.root, retained, key_index }
    }

    pub(super) fn reconcile(&mut self, tree: &ComponentTree) -> Option<ui_contract::UiPatch> {
        let mut ops = ui_contract::UiPatchOps::default();
        let previous_root = self.root;
        let new_root_id = self.diff_node(None, &tree.root, &mut ops);
        if previous_root != Some(new_root_id) {
            if let Some(stale_root) = previous_root { self.remove_subtree(None, stale_root, &mut ops); }
            ops.try_push(ui_contract::UiPatchOp::SetRoot { id: new_root_id }).unwrap();
            self.root = Some(new_root_id);
        }
        if ops.is_empty() { return None; }
        let base_revision = self.revision;
        self.revision = self.revision.try_next().unwrap();
        Some(ui_contract::UiPatch { surface: self.surface.clone(), base_revision, revision: self.revision, ops })
    }
    /// ♻️ Resolves `node`'s identity under `parent` against [`Self::key_index`]: a hit reuses the
    /// existing id and diffs field-by-field via [`Self::diff_existing`]; a miss mints a fresh id via
    /// the allocator and inserts the node wholesale via one `Upsert` — the only two ways any node ever
    /// enters `ops`. Returns the id `node` now has, whichever path was taken.
    fn diff_node(&mut self, parent: Option<ui_contract::UiNodeId>, node: &TreeNode, ops: &mut ui_contract::UiPatchOps) -> ui_contract::UiNodeId {
        let identity = identity_of(parent, node);
        if let Some(&id) = self.key_index.get(&identity) {
            self.diff_existing(id, node, ops);
            id
        } else {
            let id = self.allocator.try_allocate().expect("test allocator fixture remains below u64::MAX");
            let _ = self.key_index.try_insert(identity, id);
            let child_ids = self.diff_children(id, &ui_contract::UiNodeChildren::default(), &node.children, ops);
            let record = build_record(id, node, child_ids, None);
            let retained = record.credited_clone().expect("test retained record alias credit");
            let _ = self.retained.try_insert(id, retained);
            ops.try_push(ui_contract::UiPatchOp::Upsert(record)).expect("test patch remains bounded");
            id
        }
    }

    /// 🆚️ Diffs `node` against the retained record already at `id`, choosing the narrowest
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
    fn diff_existing(&mut self, id: ui_contract::UiNodeId, node: &TreeNode, ops: &mut ui_contract::UiPatchOps) {
        let old = self.retained.get(&id).and_then(ui_contract::UiNodeRecord::credited_clone).expect("🚫️ key_index names an id with no retained record or alias credit");
        let new_child_ids = self.diff_children(id, &old.children, &node.children, ops);

        let mut targeted = ui_contract::UiPatchOps::default();
        if old.component != node.component {
            targeted.try_push(ui_contract::UiPatchOp::SetComponent { id, component: node.component.credited_clone().expect("test component alias credit") }).expect("test patch remains bounded");
        }
        if old.layout != node.layout {
            targeted.try_push(ui_contract::UiPatchOp::SetLayout { id, layout: node.layout.clone() }).expect("test patch remains bounded");
        }
        if old.activity != node.activity || old.disabled != node.disabled {
            targeted.try_push(ui_contract::UiPatchOp::SetActivity { id, activity: node.activity, disabled: node.disabled }).expect("test patch remains bounded");
        }
        if old.children != new_child_ids {
            targeted.try_push(ui_contract::UiPatchOp::SetChildren { id, children: new_child_ids.clone() }).expect("test patch remains bounded");
        }
        if old.style != node.style {
            targeted.try_push(ui_contract::UiPatchOp::SetStyle { id, style: node.style }).expect("test patch remains bounded");
        }
        if old.accessibility != node.accessibility {
            targeted.try_push(ui_contract::UiPatchOp::SetAccessibility { id, accessibility: node.accessibility.clone() }).expect("test patch remains bounded");
        }
        if old.bindings != node.bindings {
            targeted.try_push(ui_contract::UiPatchOp::SetBindings { id, bindings: ui_contract::credited_bindings(&node.bindings).expect("test binding alias credit") }).expect("test patch remains bounded");
        }
        if old.menu != node.menu {
            let menu = node.menu.as_ref().map(|menu| menu.credited_clone().expect("test menu alias credit"));
            targeted.try_push(ui_contract::UiPatchOp::SetMenu { id, menu }).expect("test patch remains bounded");
        }

        if targeted.is_empty() {
            return;
        }

        let record = build_record(id, node, new_child_ids, old.transition);
        let upsert = ui_contract::UiPatchOp::Upsert(record.credited_clone().expect("test upsert alias credit"));
        let use_upsert = targeted.len() > 1 && self.estimate_bytes(std::slice::from_ref(&upsert)) < self.estimate_bytes(targeted.iter());

        let _ = self.retained.try_insert(id, record);
        if use_upsert {
            ops.try_push(upsert).expect("test patch remains bounded");
        } else {
            for op in targeted {
                ops.try_push(op).expect("test patch remains bounded");
            }
        }
    }

    /// 💰️ Wire-cost estimate for `candidate_ops`, delegated to [`semio_framework_ui_contract_validation::patch_byte_estimate`]
    /// via a throwaway single-purpose [`ui_contract::UiPatch`] — the byte-accounting logic (including
    /// which fields even count as "text") lives once, in the contract crate that also enforces
    /// `max_patch_bytes`, and is never duplicated here.
    fn estimate_bytes<'a>(&self, candidate_ops: impl IntoIterator<Item = &'a ui_contract::UiPatchOp>) -> usize {
        let mut ops = ui_contract::UiPatchOps::default();
        for op in candidate_ops {
            ops.try_push(op.credited_clone().expect("test patch-op alias credit")).expect("test patch remains bounded");
        }
        let probe = ui_contract::UiPatch { surface: self.surface.clone(), base_revision: ui_contract::UiRevision::default(), revision: ui_contract::UiRevision::default(), ops };
        semio_framework_ui_contract_validation::patch_byte_estimate(&probe)
    }

    /// 👶️ Diffs `new_children` against `old_child_ids` under `parent_id`, matching purely by
    /// `(parent_id, key)` — never by position — so reordering, inserting, and removing siblings each
    /// touch only the ids actually affected. Every old child whose id is not among the freshly diffed
    /// ids is removed as a whole subtree. Returns the new children list in `new_children`'s order,
    /// ready to become the parent's own `children` field.
    fn diff_children(&mut self, parent_id: ui_contract::UiNodeId, old_child_ids: &ui_contract::UiNodeChildren, new_children: &ui_contract::BuiltChildren, ops: &mut ui_contract::UiPatchOps) -> ui_contract::UiNodeChildren {
        assert_unique_child_keys(parent_id, new_children);

        let mut new_ids = ui_contract::UiNodeChildren::default();
        for child in new_children {
            new_ids.try_push(self.diff_node(Some(parent_id), child, ops)).expect("test children remain bounded");
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
    fn remove_subtree(&mut self, parent: Option<ui_contract::UiNodeId>, id: ui_contract::UiNodeId, ops: &mut ui_contract::UiPatchOps) {
        ops.try_push(ui_contract::UiPatchOp::Remove { id }).expect("test patch remains bounded");
        self.purge_subtree(parent, id);
    }

    /// 🧹️ Removes `id` and every node reachable from it via its own retained `children`, purging both
    /// `retained` and `key_index` for each — the local mirror of [`ui_contract::apply_patch`]'s
    /// `remove_subtree`. `id` is never handed back to the allocator, so it can never be reused.
    fn purge_subtree(&mut self, parent: Option<ui_contract::UiNodeId>, id: ui_contract::UiNodeId) {
        if let Some(record) = self.retained.remove(&id) {
            self.key_index.remove(&(parent, record.key));
            for child_id in record.children {
                self.purge_subtree(Some(id), child_id);
            }
        }
    }
}
//#endregion 🔖️Diff
