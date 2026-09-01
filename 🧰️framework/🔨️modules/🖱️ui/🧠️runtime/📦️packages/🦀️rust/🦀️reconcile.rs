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
//! The frame path uses [`SurfaceReconcileCursor`] internally: presentation discovery, identity
//! allocation, postorder record diffing, and stale-tree removal each advance one node at a time.
//! Plain synchronous calls are cooperative scheduler slices, not a hidden run-to-completion frame.

#[cfg(test)]
use std::collections::HashSet;
use std::mem::{size_of, take};
use std::sync::{LazyLock, Mutex};

#[path = "../../📤️output/🦀️component.rs"]
mod output;
pub use output::{SurfaceReconcileOutputReservation, SurfaceReconcileOutputs, SurfaceReconcileOutputTransfer};

//#region 🔖️Identity

/// 🔑️ A node's reconciliation identity: which parent it hangs under (`None` only for the root, which
/// has no parent) plus its own sibling `key`. Two [`crate::TreeNode`]s presented on different frames
/// with the same identity are the SAME node as far as reconciliation is concerned, regardless of what
/// position either occupied among its siblings — this is the one invariant every other rule here
/// exists to preserve.
type NodeIdentity = (Option<ui_contract::UiNodeId>, ui_contract::UiText);

const SURFACE_RECONCILE_FIXED_NODES: usize = ui_contract::UI_DOCUMENT_NODES;
const SURFACE_RECONCILE_FIXED_OPS: usize = SURFACE_RECONCILE_FIXED_NODES * 9 + 1;

#[derive(Debug)]
struct SurfaceFixedVec<T, const N: usize> {
    entries: Box<[Option<T>]>,
    len: usize,
}

impl<T, const N: usize> Default for SurfaceFixedVec<T, N> {
    fn default() -> Self {
        let mut entries = Vec::with_capacity(N);
        entries.resize_with(N, || None);
        Self { entries: entries.into_boxed_slice(), len: 0 }
    }
}

impl<T, const N: usize> SurfaceFixedVec<T, N> {
    fn try_push(&mut self, value: T) -> Result<(), T> {
        if self.len == N || self.len == self.entries.len() {
            return Err(value);
        }
        self.entries[self.len] = Some(value);
        self.len += 1;
        Ok(())
    }

    fn pop(&mut self) -> Option<T> {
        let index = self.len.checked_sub(1)?;
        self.len = index;
        self.entries[index].take()
    }

    fn get(&self, index: usize) -> Option<&T> {
        (index < self.len).then(|| self.entries[index].as_ref()).flatten()
    }

    fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        (index < self.len).then(|| self.entries[index].as_mut()).flatten()
    }

    fn first(&self) -> Option<&T> {
        self.get(0)
    }

    fn last_mut(&mut self) -> Option<&mut T> {
        self.len.checked_sub(1).and_then(|index| self.entries[index].as_mut())
    }

    fn len(&self) -> usize {
        self.len
    }

    fn is_empty(&self) -> bool {
        self.len == 0
    }

    fn iter(&self) -> impl Iterator<Item = &T> {
        self.entries[..self.len].iter().filter_map(Option::as_ref)
    }

    fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.entries[..self.len].iter_mut().filter_map(Option::as_mut)
    }

    fn take_all(&mut self) -> Self {
        std::mem::replace(self, Self { entries: Box::new([]), len: 0 })
    }
}

impl<T: PartialEq, const N: usize> SurfaceFixedVec<T, N> {
    fn contains(&self, value: &T) -> bool {
        self.iter().any(|candidate| candidate == value)
    }
}

impl<T, const N: usize> std::ops::Index<usize> for SurfaceFixedVec<T, N> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.get(index).expect("fixed reconcile index was admitted")
    }
}

impl<T, const N: usize> std::ops::IndexMut<usize> for SurfaceFixedVec<T, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.get_mut(index).expect("fixed reconcile index was admitted")
    }
}

#[derive(Debug)]
struct SurfaceLinearMap<K, V, const N: usize> {
    entries: SurfaceFixedVec<(K, V), N>,
}

impl<K, V, const N: usize> Default for SurfaceLinearMap<K, V, N> {
    fn default() -> Self {
        Self { entries: SurfaceFixedVec::default() }
    }
}

impl<K, V, const N: usize> SurfaceLinearMap<K, V, N> {
    fn take_all(&mut self) -> Self {
        Self { entries: self.entries.take_all() }
    }
}

impl<K: Eq, V, const N: usize> SurfaceLinearMap<K, V, N> {
    fn get(&self, key: &K) -> Option<&V> {
        self.entries.iter().find(|(candidate, _)| candidate == key).map(|(_, value)| value)
    }

    fn try_insert(&mut self, key: K, value: V) -> Result<Option<V>, (K, V)> {
        if let Some((_, current)) = self.entries.iter_mut().find(|(candidate, _)| candidate == &key) {
            return Ok(Some(std::mem::replace(current, value)));
        }
        self.entries.try_push((key, value)).map(|()| None)
    }

    fn remove(&mut self, key: &K) -> Option<V> {
        let index = self.entries.iter().position(|(candidate, _)| candidate == key)?;
        let last = self.entries.pop()?;
        if index == self.entries.len() {
            return Some(last.1);
        }
        let removed = std::mem::replace(&mut self.entries[index], last);
        Some(removed.1)
    }

    fn contains_key(&self, key: &K) -> bool {
        self.entries.iter().any(|(candidate, _)| candidate == key)
    }

    fn keys(&self) -> impl Iterator<Item = &K> {
        self.entries.iter().map(|(key, _)| key)
    }

    fn values(&self) -> impl Iterator<Item = &V> {
        self.entries.iter().map(|(_, value)| value)
    }

    fn clear(&mut self) {
        while self.entries.pop().is_some() {}
    }

    fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    fn len(&self) -> usize {
        self.entries.len()
    }
}

#[derive(Debug)]
struct SurfaceLinearSet<T, const N: usize> {
    entries: SurfaceFixedVec<T, N>,
}

impl<T, const N: usize> Default for SurfaceLinearSet<T, N> {
    fn default() -> Self {
        Self { entries: SurfaceFixedVec::default() }
    }
}

impl<T: Eq, const N: usize> SurfaceLinearSet<T, N> {
    fn try_insert(&mut self, value: T) -> Result<bool, T> {
        if self.entries.contains(&value) {
            return Ok(false);
        }
        self.entries.try_push(value).map(|()| true)
    }

    fn iter(&self) -> impl Iterator<Item = &T> {
        self.entries.iter()
    }

    fn remove(&mut self, value: &T) -> bool {
        let Some(index) = self.entries.iter().position(|candidate| candidate == value) else { return false };
        let Some(last) = self.entries.pop() else { return false };
        if index < self.entries.len() {
            self.entries[index] = last;
        }
        true
    }

    fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

/// 🔑️ `node`'s identity under `parent`, as looked up in / inserted into [`SurfaceReconciler::key_index`].
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
#[cfg(test)]
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
#[cfg(test)]
fn assert_unique_child_keys(parent: ui_contract::UiNodeId, children: &ui_contract::BuiltChildren) {
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
/// A completed reconcile is the only place any of these four change together.
#[derive(Debug)]
pub struct SurfaceReconciler {
    surface: ui_contract::SurfaceId,
    revision: ui_contract::UiRevision,
    allocator: ui_contract::UiNodeIdAllocator,
    document: Option<ui_contract::UiDocumentLease>,
    assembly: ui_contract::UiDocumentAssembly,
    ordinals: SurfaceLinearMap<ui_contract::UiNodeId, usize, SURFACE_RECONCILE_FIXED_NODES>,
    key_index: SurfaceLinearMap<NodeIdentity, ui_contract::UiNodeId, SURFACE_RECONCILE_FIXED_NODES>,
    root: Option<ui_contract::UiNodeId>,
    retire_scalar: u8,
    seal_phase: u8,
    handback: Option<SurfaceReconcileHandbackReservation>,
    retirement_armed: bool,
}

impl SurfaceReconciler {
    /// 🌱️ A reconciler for `surface` with no retained state yet — the next [`Self::reconcile`] call
    /// necessarily emits a full `SetRoot` plus one `Upsert` per node, exactly as [`Self::mark_rejected`]
    /// arranges for an existing reconciler to do again.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    #[cfg(not(test))]
    pub fn new(surface: ui_contract::SurfaceId) -> Self {
        Self::from_surface_id(surface)
    }

    #[cfg(test)]
    pub fn new(surface: impl AsRef<str>) -> Self {
        let surface = ui_contract::UiText::try_from_str(surface.as_ref()).map(ui_contract::SurfaceId).expect("bounded test surface");
        Self::from_surface_id(surface)
    }

    fn from_surface_id(surface: ui_contract::SurfaceId) -> Self {
        Self {
            surface,
            revision: ui_contract::UiRevision::default(),
            allocator: ui_contract::UiNodeIdAllocator::default(),
            document: None,
            assembly: ui_contract::UiDocumentAssembly::default(),
            ordinals: SurfaceLinearMap::default(),
            key_index: SurfaceLinearMap::default(),
            root: None,
            retire_scalar: 0,
            seal_phase: 0,
            handback: None,
            retirement_armed: true,
        }
    }

    /// ♻️ Diffs `tree` against this reconciler's retained state, mutating that state to match and
    /// returning the minimal [`ui_contract::UiPatch`] that carries the difference — or `None` when
    /// `tree` is structurally and semantically identical to what was last presented, so an idle surface
    /// produces no wire traffic at all. `base_revision` is the revision the receiver is assumed to be
    /// at; `revision` is one past it — this reconciler never emits a gap or a repeat.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    #[cfg(test)]
    pub fn reconcile(&mut self, tree: &crate::ComponentTree) -> Option<ui_contract::UiPatch> {
        let mut oracle = SurfaceReconcileOracle::from_current(self);
        let patch = oracle.reconcile(tree);
        if patch.is_some() { self.install_oracle(oracle); }
        patch
    }

    /// 📸️ The complete current state as a fresh [`ui_contract::UiSnapshot`] — what a new subscriber
    /// receives instead of a patch stream. `root` falls back to [`ui_contract::UiNodeId::default`] when
    /// nothing has ever been reconciled yet; `nodes` is then empty too, so that sentinel never resolves
    /// to a real record.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    #[cfg(test)]
    pub fn snapshot(&self) -> ui_contract::UiSnapshot {
        let mut nodes = ui_contract::UiSnapshotNodes::default();
        if let Some(document) = self.document.as_ref() {
            let read = document.try_read().expect("cold snapshot owns readable canonical root");
            for ordinal in 0..read.len() {
                nodes.try_push(read.node_at(ordinal).unwrap().credited_clone().expect("test snapshot alias credit")).expect("test snapshot remains bounded");
            }
        }
        ui_contract::UiSnapshot { surface: self.surface.clone(), revision: self.revision, root: self.root.unwrap_or_default(), nodes, layout_epoch: 0 }
    }

    /// 🧬️ Returns the retained scalar revision without cloning the retained document.
    pub fn revision(&self) -> ui_contract::UiRevision {
        self.revision
    }

    /// 🪪️ Returns the reconciler's surface identity without allocating a second owner.
    pub fn surface(&self) -> &ui_contract::SurfaceId {
        &self.surface
    }

    /// 📖️ Captures the canonical root without a second payload tree or reservation.
    pub fn capture_document(&self, target: &mut Option<ui_contract::UiDocumentLease>, admitted_bytes: usize) -> Result<bool, ui_contract::UiDocumentLeaseError> {
        match self.document.as_ref() {
            Some(document) => document.try_alias_into(target, admitted_bytes),
            None => Ok(false),
        }
    }

    #[cfg(test)]
    pub(crate) fn transaction_reader(&self) -> Self {
        let mut reader = Self::from_surface_id(self.surface.clone());
        reader.revision = self.revision;
        reader.allocator = self.allocator.clone();
        reader.root = self.root;
        reader.seal_phase = self.seal_phase;
        self.capture_document(&mut reader.document, SURFACE_RECONCILE_PAGE_BYTES).expect("test transaction captures the original canonical root");
        if let Some((source, captured)) = self.document.as_ref().zip(reader.document.as_ref()) { assert!(source.same_root(captured)); }
        for (id, ordinal) in self.ordinals.entries.iter() { reader.ordinals.try_insert(*id, *ordinal).unwrap(); }
        for (identity, id) in self.key_index.entries.iter() { reader.key_index.try_insert(identity.clone(), *id).unwrap(); }
        reader
    }

    #[cfg(test)]
    pub(crate) fn close_transaction_oracle(&mut self) {
        while !self.retire_one() {}
    }

    fn read_record(&self, id: ui_contract::UiNodeId) -> Result<Option<SurfaceRecordRead<'_>>, ui_contract::UiDocumentLeaseError> {
        let Some(ordinal) = self.ordinals.get(&id).copied() else { return Ok(None); };
        let document = self.document.as_ref().ok_or(ui_contract::UiDocumentLeaseError::StaleHandle)?;
        let read = document.try_read()?;
        read.exact_node(ordinal, id)?;
        Ok(Some(SurfaceRecordRead { read, ordinal, id }))
    }

    fn seal_step(&mut self, usage: SurfaceReconcileUsage, output: &mut Option<ui_contract::UiResidentPermit>, has_output: bool) -> Result<bool, ui_contract::UiDocumentAssemblyError> {
        if self.document.is_some() { return Ok(true); }
        let progress = match self.seal_phase {
            0 => {
                let fixed = size_of::<Self>() + std::mem::size_of_val(self.ordinals.entries.entries.as_ref()) + std::mem::size_of_val(self.key_index.entries.entries.as_ref()) + ui_contract::UiDocumentAssembly::required_open_bytes();
                let allocated = self.assembly.allocated_bytes()?;
                let bytes = usage.bytes.checked_add(fixed).and_then(|bytes| bytes.checked_add(allocated)).unwrap_or(usize::MAX);
                self.assembly.shrink_resident(ui_contract::UiResidentLimits { items: usage.items.max(self.ordinals.len()).max(1), bytes }, 1, SURFACE_RECONCILE_PAGE_BYTES)?
            }
            1 if has_output => self.assembly.split_resident_output(output, 1, SURFACE_RECONCILE_PAGE_BYTES)?,
            1 => { self.seal_phase = 2; return Ok(false); }
            _ => self.assembly.finish_into(&mut self.document, self.revision, 1, SURFACE_RECONCILE_PAGE_BYTES)?,
        };
        if progress.complete { self.seal_phase += 1; }
        Ok(self.document.is_some())
    }

    /// 🔄️ Forces the next [`Self::reconcile`] to emit a full re-send — the recovery path for the
    /// existing `patch-rejected` wire event. Drops every retained node and resets the assumed receiver
    /// revision to zero (mirroring the fresh, empty document a rejection leaves the receiver at); the
    /// [`ui_contract::UiNodeIdAllocator`] is deliberately left untouched, so the re-sent nodes get IDs
    /// that continue monotonically rather than reusing any id a stale renderer reference might still
    /// name.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    #[cfg(test)]
    pub fn mark_rejected(&mut self) {
        if let Some(document) = self.document.as_mut() { while !document.close_read_step_with_grant(1, 4096).unwrap().complete {} }
        self.document = None;
        self.ordinals.clear();
        self.key_index.clear();
        self.root = None;
        self.revision = ui_contract::UiRevision::default();
    }

    fn retire_one(&mut self) -> bool {
        if let Some(document) = self.document.as_mut() {
            if document.close_read_step_with_grant(1, SURFACE_COMPONENT_COPY_WORK_BYTES).expect("canonical read retirement preserves exact fault authority").complete { self.document = None; }
            return false;
        }
        if !self.assembly.terminal_is_empty() {
            self.assembly.close_step(1, SURFACE_COMPONENT_COPY_WORK_BYTES).expect("canonical assembly retirement preserves exact fault authority");
            return false;
        }
        if self.ordinals.take_first().is_some() {
            return false;
        }
        let indexed_identity = self.key_index.keys().next().cloned();
        if let Some(identity) = indexed_identity {
            self.key_index.remove(&identity);
            return false;
        }
        match self.retire_scalar {
            0 => self.root = None,
            1 => self.surface.0 = ui_contract::UiText::default(),
            2 => self.revision = ui_contract::UiRevision::default(),
            3 => self.allocator = ui_contract::UiNodeIdAllocator::default(),
            4 => self.seal_phase = 0,
            5 => {
                if let Some(handback) = self.handback.take() {
                    release_surface_reconcile_handback(handback);
                }
            }
            _ => return true,
        }
        self.retire_scalar += 1;
        self.retire_scalar >= 6
    }
}

impl Drop for SurfaceReconciler {
    fn drop(&mut self) {
        if !self.retirement_armed {
            return;
        }
        self.retirement_armed = false;
        if self.document.is_none() && self.assembly.terminal_is_empty() && self.ordinals.is_empty() && self.key_index.is_empty() {
            if let Some(handback) = self.handback.take() {
                release_surface_reconcile_handback(handback);
            }
            return;
        }
        let generation = self.handback.as_ref().map_or(0, |reservation| reservation.key.generation);
        let owner = SurfaceReconciler {
            surface: take(&mut self.surface),
            revision: take(&mut self.revision),
            allocator: take(&mut self.allocator),
            document: self.document.take(),
            assembly: take(&mut self.assembly),
            ordinals: self.ordinals.take_all(),
            key_index: self.key_index.take_all(),
            root: self.root.take(),
            retire_scalar: self.retire_scalar,
            seal_phase: self.seal_phase,
            handback: None,
            retirement_armed: false,
        };
        let state = Box::new(SurfaceReconcileRetained {
            output_handback: None,
            generation,
            phase: SurfaceReconcileJobPhase::Closing,
            current: Some(owner),
            source: None,
            cursor: None,
            candidate: None,
            patch: ui_contract::UiPendingPatch::default(),
            retire_tree: SurfaceTreeRetireCursor::default(),
            fault: None,
            usage: SurfaceReconcileUsage::default(),
            credit: None,
            handback: self.handback.take(),
        });
        handback_surface_reconcile(state);
    }
}

struct SurfaceRecordRead<'a> {
    read: ui_contract::UiDocumentRead<'a>,
    ordinal: usize,
    id: ui_contract::UiNodeId,
}

#[cfg(test)]
impl SurfaceReconciler {
    fn install_oracle(&mut self, mut oracle: SurfaceReconcileOracle) {
        if let Some(document) = self.document.as_mut() { while !document.close_read_step_with_grant(1, 4096).unwrap().complete {} }
        self.document = None;
        self.ordinals.clear();
        let mut credit = reserve_surface_reconcile(SurfaceReconcileLimits { max_bytes: 1024 * 1024, ..Default::default() });
        let mut surface = Some(self.surface.clone());
        let mut assembly = ui_contract::UiDocumentAssembly::default();
        assert!(assembly.open_with_permit(&mut credit, &mut surface, oracle.revision.0.checked_add(1).unwrap(), oracle.revision, oracle.root, 0, 1, SURFACE_RECONCILE_PAGE_BYTES).unwrap().progressed);
        while let Some((id, record)) = oracle.retained.take_first() {
            let mut source = Some(record);
            let ordinal = self.ordinals.len();
            while source.is_some() { assembly.place_one(&mut source, 1, SURFACE_RECONCILE_PAGE_BYTES).unwrap(); }
            self.ordinals.try_insert(id, ordinal).unwrap();
        }
        while !assembly.finish_into(&mut self.document, oracle.revision, 1, SURFACE_RECONCILE_PAGE_BYTES).unwrap().complete {}
        self.root = oracle.root;
        self.revision = oracle.revision;
        self.allocator = oracle.allocator;
        self.key_index = oracle.key_index;
    }

    fn install_fixture_record(&mut self, record: ui_contract::UiNodeRecord) {
        let mut oracle = SurfaceReconcileOracle::from_current(self);
        oracle.root = Some(record.id);
        oracle.key_index.try_insert((None, record.key.clone()), record.id).unwrap();
        oracle.retained.try_insert(record.id, record).unwrap();
        self.install_oracle(oracle);
    }
}

impl std::ops::Deref for SurfaceRecordRead<'_> {
    type Target = ui_contract::UiNodeRecord;
    fn deref(&self) -> &Self::Target { self.read.exact_node(self.ordinal, self.id).expect("held canonical identity was checked before exposing the read") }
}

impl<K, V, const N: usize> SurfaceLinearMap<K, V, N> {
    fn get_index(&self, index: usize) -> Option<(&K, &V)> {
        self.entries.get(index).map(|(key, value)| (key, value))
    }

    fn take_first(&mut self) -> Option<(K, V)> {
        if self.entries.is_empty() {
            return None;
        }
        let last = self.entries.pop()?;
        if self.entries.is_empty() {
            return Some(last);
        }
        Some(std::mem::replace(&mut self.entries[0], last))
    }
}

impl<T, const N: usize> SurfaceLinearSet<T, N> {
    fn pop(&mut self) -> Option<T> {
        self.entries.pop()
    }
}

//#endregion 🔖️Reconciler

//#region ⏭️ResumableReconcile

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SurfaceReconcileStage {
    TraversePresentation,
    AllocateIdentities,
    DiffRecords,
    RemoveStale,
    Finalize,
}

#[derive(Debug)]
pub(crate) enum SurfaceReconcileStep {
    Yield { nodes: usize, bytes: usize },
    Complete { reconciler: SurfaceReconciler, patch: Option<ui_contract::UiPatch> },
    Fault(SurfaceReconcileFault),
}

/// 🚧️ Fixed admission bounds for one retained reconciliation authority.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SurfaceReconcileLimits {
    pub max_nodes: usize,
    pub max_items: usize,
    pub max_bytes: usize,
    pub max_identifier_bytes: usize,
}

impl Default for SurfaceReconcileLimits {
    fn default() -> Self {
        Self { max_nodes: SURFACE_RECONCILE_FIXED_NODES, max_items: 4_097, max_bytes: SURFACE_RECONCILE_SURFACE_BYTES, max_identifier_bytes: 256 }
    }
}

/// 📏️ Credits observed while advancing one retained reconciliation.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SurfaceReconcileUsage {
    pub nodes: usize,
    pub items: usize,
    pub bytes: usize,
}

impl SurfaceReconcileUsage {
    fn include(&mut self, nodes: usize, items: usize, bytes: usize) -> bool {
        let Some(next_nodes) = self.nodes.checked_add(nodes) else { return false };
        let Some(next_items) = self.items.checked_add(items) else { return false };
        let Some(next_bytes) = self.bytes.checked_add(bytes) else { return false };
        self.nodes = next_nodes;
        self.items = next_items;
        self.bytes = next_bytes;
        true
    }

    fn fits(self, limits: SurfaceReconcileLimits) -> bool {
        self.nodes <= limits.max_nodes && self.items <= limits.max_items && self.bytes <= limits.max_bytes
    }
}

/// 🚫️ Typed terminal reason retaining the exact in-progress authority.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SurfaceReconcileFault {
    PublicationAuthority(&'static str),
    AliasCapacity,
    CounterOverflow,
    DuplicateSiblingKey,
    IdentifierBytes { actual: usize, max: usize },
    Credits { usage: SurfaceReconcileUsage, limits: SurfaceReconcileLimits },
    PageBytes { actual: usize, max: usize },
    ValueDepth { actual: usize, max: usize },
    StaleGeneration { expected: u64, actual: u64 },
    Cancelled,
}

struct FlatPresentedNode {
    parent: Option<usize>,
    node: crate::TreeNode,
    child_ids: ui_contract::UiNodeChildren,
}

struct PresentationFrame {
    index: usize,
    children: ui_contract::BuiltChildrenIntoIter,
}

struct RemovalFrame {
    id: ui_contract::UiNodeId,
    next_child: usize,
}

enum RecordOwnedCopy {
    Bindings(ui_contract::UiBindingsCopy),
    Component(ui_contract::UiComponentCopy),
    Comparison(ExistingComponentComparison),
    Changed(bool),
}

#[derive(Default)]
struct ExistingComponentComparison {
    cursor: ui_contract::UiComponentComparisonCursor,
    lease: Option<ui_contract::UiDocumentLease>,
    changed: Option<bool>,
}

impl ExistingComponentComparison {
    fn close_step(&mut self) -> bool {
        self.cursor.release_reads();
        if let Some(lease) = self.lease.as_mut() {
            if lease.close_read_step_with_grant(1, SURFACE_COMPONENT_COPY_WORK_BYTES).is_ok_and(|step| step.complete) { self.lease = None; }
            return false;
        }
        true
    }
}

impl RecordOwnedCopy {
    fn bindings(&self) -> Option<&ui_contract::UiBindingsCopy> { if let Self::Bindings(value) = self { Some(value) } else { None } }
    fn bindings_mut(&mut self) -> Option<&mut ui_contract::UiBindingsCopy> { if let Self::Bindings(value) = self { Some(value) } else { None } }
    fn component(&self) -> Option<&ui_contract::UiComponentCopy> { if let Self::Component(value) = self { Some(value) } else { None } }
    fn component_mut(&mut self) -> Option<&mut ui_contract::UiComponentCopy> { if let Self::Component(value) = self { Some(value) } else { None } }
    fn close_step(&mut self) -> bool {
        match self {
            Self::Bindings(value) => value.close_step(1, SURFACE_RECONCILE_PAGE_BYTES).is_ok_and(|step| step.complete),
            Self::Component(value) => value.close_step(1, SURFACE_COMPONENT_COPY_WORK_BYTES).is_ok_and(|step| step.complete),
            Self::Comparison(value) => value.close_step(),
            Self::Changed(_) => true,
        }
    }
}

struct RecordDiffCursor {
    id: ui_contract::UiNodeId,
    record: RecordSource,
    field: u8,
    fresh: Option<FreshRecordClone>,
    owned_copy: Option<RecordOwnedCopy>,
}

struct RecordSource(Option<ui_contract::UiNodeRecord>);
impl From<ui_contract::UiNodeRecord> for RecordSource { fn from(record: ui_contract::UiNodeRecord) -> Self { Self(Some(record)) } }
impl std::ops::Deref for RecordSource {
    type Target = ui_contract::UiNodeRecord;
    fn deref(&self) -> &Self::Target { self.0.as_ref().expect("unplaced record remains in its structural source slot") }
}
impl std::ops::DerefMut for RecordSource {
    fn deref_mut(&mut self) -> &mut Self::Target { self.0.as_mut().expect("unplaced record remains in its structural source slot") }
}

struct FreshRecordClone {
    key: Option<ui_contract::UiText>,
    component: Option<ui_contract::Component>,
    layout: Option<ui_contract::LayoutSpec>,
    children: Option<ui_contract::UiNodeChildren>,
    accessibility: Option<ui_contract::AccessibilitySpec>,
    bindings: Option<ui_contract::UiNodeBindings>,
    menu: Option<Option<ui_contract::MenuRef>>,
}

impl Default for FreshRecordClone {
    fn default() -> Self {
        Self { key: None, component: None, layout: None, children: None, accessibility: None, bindings: None, menu: None }
    }
}

const SURFACE_RECONCILE_VALUE_DEPTH: usize = 64;
const SURFACE_RECONCILE_TREE_RETIRE_DEPTH: usize = 4_097;
const SURFACE_RECONCILE_SEMANTIC_COPIES: usize = 3;

#[cfg(test)]
fn admit_vec_backing<T>(owner: &mut Vec<T>, usage: &mut SurfaceReconcileUsage, limits: SurfaceReconcileLimits) -> Result<(), SurfaceReconcileFault> {
    if owner.len() < owner.capacity() {
        return Ok(());
    }
    let before = owner.capacity();
    if owner.try_reserve_exact(1).is_err() {
        return Err(SurfaceReconcileFault::Credits { usage: SurfaceReconcileUsage { bytes: limits.max_bytes.checked_add(1).unwrap_or(usize::MAX), ..*usage }, limits });
    }
    let slots = owner.capacity().checked_sub(before).ok_or(SurfaceReconcileFault::CounterOverflow)?;
    let mut projected = *usage;
    let bytes = slots.checked_mul(size_of::<T>()).ok_or(SurfaceReconcileFault::CounterOverflow)?;
    if !projected.include(0, slots, bytes) || !projected.fits(limits) {
        return Err(SurfaceReconcileFault::Credits { usage: projected, limits });
    }
    *usage = projected;
    Ok(())
}

struct SurfaceSemanticMapPage {
    cursor: ui_contract::UiMapCursor,
    value: Option<ui_contract::UiValue>,
}

enum SurfaceSemanticValueFrame {
    Value(ui_contract::UiValue),
    List { cursor: ui_contract::UiListCursor },
    Map { page: SurfaceSemanticMapPage },
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct SurfaceSemanticUsage {
    items: usize,
    bytes: usize,
}

enum SurfaceSemanticCensusStep {
    Progress(SurfaceSemanticUsage),
    Complete,
    Fault(SurfaceReconcileFault),
}

/// 📏️ Charges complete semantic ownership in bounded pages; a node may span several pages.
struct SurfaceSemanticCensusCursor {
    field: u8,
    container: u8,
    entry: usize,
    binding: usize,
    action: u8,
    data_attribute: u8,
    string_byte: usize,
    depth: usize,
    value_stack: Box<[Option<SurfaceSemanticValueFrame>]>,
}

impl Default for SurfaceSemanticCensusCursor {
    fn default() -> Self {
        let mut value_stack = Vec::with_capacity(SURFACE_RECONCILE_VALUE_DEPTH);
        value_stack.resize_with(SURFACE_RECONCILE_VALUE_DEPTH, || None);
        Self { field: 0, container: 0, entry: 0, binding: 0, action: 0, data_attribute: 0, string_byte: 0, depth: 0, value_stack: value_stack.into_boxed_slice() }
    }
}

impl SurfaceSemanticCensusCursor {
    /// 📏️ UiText storage is inline in the already-counted enclosing payload; traversal remains work.
    fn inline_text(&self, _value: &ui_contract::UiText) -> SurfaceSemanticUsage {
        SurfaceSemanticUsage { items: 1, bytes: 0 }
    }

    fn owner(&mut self, bytes: usize) -> SurfaceSemanticUsage {
        self.string_byte = bytes.checked_mul(SURFACE_RECONCILE_SEMANTIC_COPIES).unwrap_or(usize::MAX);
        SurfaceSemanticUsage { items: SURFACE_RECONCILE_SEMANTIC_COPIES, bytes: 0 }
    }

    fn backing<T>(&mut self, capacity: usize) -> SurfaceSemanticUsage {
        self.owner(capacity.checked_mul(size_of::<T>()).unwrap_or(usize::MAX))
    }

    fn push_value(&mut self, value: &ui_contract::UiValue) -> Result<(), SurfaceReconcileFault> {
        if self.depth == SURFACE_RECONCILE_VALUE_DEPTH {
            return Err(SurfaceReconcileFault::ValueDepth { actual: self.depth.checked_add(1).ok_or(SurfaceReconcileFault::CounterOverflow)?, max: SURFACE_RECONCILE_VALUE_DEPTH });
        }
        let value = value.credited_clone().ok_or(SurfaceReconcileFault::AliasCapacity)?;
        self.push_owned_value(value)
    }

    fn push_owned_value(&mut self, value: ui_contract::UiValue) -> Result<(), SurfaceReconcileFault> {
        if self.depth == SURFACE_RECONCILE_VALUE_DEPTH {
            return Err(SurfaceReconcileFault::ValueDepth { actual: self.depth.checked_add(1).ok_or(SurfaceReconcileFault::CounterOverflow)?, max: SURFACE_RECONCILE_VALUE_DEPTH });
        }
        let next = self.depth.checked_add(1).ok_or(SurfaceReconcileFault::CounterOverflow)?;
        self.value_stack[self.depth] = Some(SurfaceSemanticValueFrame::Value(value));
        self.depth = next;
        Ok(())
    }

    fn value_step(&mut self) -> Option<SurfaceSemanticCensusStep> {
        let frame = self.value_stack.get_mut(self.depth.checked_sub(1)?)?.take()?;
        match frame {
            SurfaceSemanticValueFrame::Value(value) => {
                self.depth -= 1;
                match value {
                    ui_contract::UiValue::Null | ui_contract::UiValue::Bool(_) | ui_contract::UiValue::Number(_) => Some(SurfaceSemanticCensusStep::Progress(SurfaceSemanticUsage::default())),
                    ui_contract::UiValue::Text(value) => Some(SurfaceSemanticCensusStep::Progress(self.owner(value.len()))),
                    ui_contract::UiValue::List(values) => {
                        self.value_stack[self.depth] = Some(SurfaceSemanticValueFrame::List { cursor: values.cursor() });
                        self.depth += 1;
                        Some(SurfaceSemanticCensusStep::Progress(SurfaceSemanticUsage::default()))
                    }
                    ui_contract::UiValue::Map(values) => {
                        self.value_stack[self.depth] = Some(SurfaceSemanticValueFrame::Map { page: SurfaceSemanticMapPage { cursor: values.cursor(), value: None } });
                        self.depth += 1;
                        Some(SurfaceSemanticCensusStep::Progress(SurfaceSemanticUsage::default()))
                    }
                }
            }
            SurfaceSemanticValueFrame::List { mut cursor } => {
                let Some(value) = cursor.next() else {
                    self.depth -= 1;
                    return Some(SurfaceSemanticCensusStep::Progress(SurfaceSemanticUsage::default()));
                };
                self.value_stack[self.depth - 1] = Some(SurfaceSemanticValueFrame::List { cursor });
                Some(match self.push_owned_value(value) {
                    Ok(()) => SurfaceSemanticCensusStep::Progress(SurfaceSemanticUsage::default()),
                    Err(fault) => SurfaceSemanticCensusStep::Fault(fault),
                })
            }
            SurfaceSemanticValueFrame::Map { mut page } => {
                if page.value.is_none() {
                    let Some(key_bytes) = page.cursor.advance().map(|(key, _)| key.len()) else {
                        self.depth -= 1;
                        return Some(SurfaceSemanticCensusStep::Progress(SurfaceSemanticUsage::default()));
                    };
                    page.value = page.cursor.take_current().map(|(_, value)| value);
                    self.value_stack[self.depth - 1] = Some(SurfaceSemanticValueFrame::Map { page });
                    return Some(SurfaceSemanticCensusStep::Progress(self.owner(key_bytes)));
                }
                let Some(value) = page.value.take() else {
                    self.value_stack[self.depth - 1] = Some(SurfaceSemanticValueFrame::Map { page });
                    return Some(SurfaceSemanticCensusStep::Fault(SurfaceReconcileFault::CounterOverflow));
                };
                self.value_stack[self.depth - 1] = Some(SurfaceSemanticValueFrame::Map { page });
                if let Err(fault) = self.push_owned_value(value) {
                    return Some(SurfaceSemanticCensusStep::Fault(fault));
                }
                Some(SurfaceSemanticCensusStep::Progress(SurfaceSemanticUsage::default()))
            }
        }
    }

    fn bindings_step(&mut self, bindings: &ui_contract::UiNodeBindings) -> SurfaceSemanticCensusStep {
        let Some(binding) = bindings.get(self.binding) else {
            self.binding = 0;
            self.action = 0;
            return SurfaceSemanticCensusStep::Complete;
        };
        let usage = match self.action {
            0 => self.inline_text(&binding.action.scope),
            1 => self.inline_text(&binding.action.name),
            2 => {
                if let Some(args) = &binding.args {
                    if let Err(fault) = self.push_value(args) {
                        return SurfaceSemanticCensusStep::Fault(fault);
                    }
                }
                SurfaceSemanticUsage::default()
            }
            3 => binding.capability.as_ref().map_or_else(SurfaceSemanticUsage::default, |value| self.inline_text(&value)),
            _ => {
                self.binding += 1;
                self.action = 0;
                return SurfaceSemanticCensusStep::Progress(SurfaceSemanticUsage::default());
            }
        };
        self.action += 1;
        SurfaceSemanticCensusStep::Progress(usage)
    }

    fn binding_step(&mut self, binding: &ui_contract::ActionBinding) -> SurfaceSemanticCensusStep {
        let usage = match self.action {
            0 => self.inline_text(&binding.action.scope),
            1 => self.inline_text(&binding.action.name),
            2 => {
                if let Some(args) = &binding.args {
                    if let Err(fault) = self.push_value(args) {
                        return SurfaceSemanticCensusStep::Fault(fault);
                    }
                }
                SurfaceSemanticUsage::default()
            }
            3 => binding.capability.as_ref().map_or_else(SurfaceSemanticUsage::default, |value| self.inline_text(&value)),
            _ => {
                self.action = 0;
                return SurfaceSemanticCensusStep::Complete;
            }
        };
        self.action += 1;
        SurfaceSemanticCensusStep::Progress(usage)
    }

    fn component_step(&mut self, component: &ui_contract::Component) -> SurfaceSemanticCensusStep {
        use ui_contract::Component::*;
        let progress = |usage| SurfaceSemanticCensusStep::Progress(usage);
        match component {
            Container(props) => {
                let usage = match self.container {
                    0 => props.label.as_ref().map_or_else(SurfaceSemanticUsage::default, |value| self.inline_text(&value.0)),
                    1 => props.description.as_ref().map_or_else(SurfaceSemanticUsage::default, |value| self.inline_text(&value)),
                    2 => props.error.as_ref().map_or_else(SurfaceSemanticUsage::default, |value| self.inline_text(&value)),
                    3 => props.drop_overlay.as_ref().map_or_else(SurfaceSemanticUsage::default, |value| self.inline_text(&value.title.0)),
                    4 => props.drop_overlay.as_ref().map_or_else(SurfaceSemanticUsage::default, |value| self.inline_text(&value.hint.0)),
                    5 => props.drop_overlay.as_ref().and_then(|value| value.accept.as_ref()).map_or_else(SurfaceSemanticUsage::default, |value| self.inline_text(&value)),
                    _ => return SurfaceSemanticCensusStep::Complete,
                };
                self.container += 1;
                progress(usage)
            }
            Text(props) => match self.container {
                0 => {
                    self.container = 1;
                    progress(self.inline_text(&props.value.0))
                }
                1 => {
                    self.container = 2;
                    progress(props.data_attributes.as_ref().map_or_else(SurfaceSemanticUsage::default, |values| self.backing::<(ui_contract::UiText, ui_contract::UiText)>(values.capacity())))
                }
                2 => {
                    let Some((key, value)) = props.data_attributes.as_ref().and_then(|values| values.get(self.entry)) else { return SurfaceSemanticCensusStep::Complete };
                    let usage = if self.data_attribute == 0 {
                        self.data_attribute = 1;
                        self.inline_text(&key)
                    } else {
                        self.data_attribute = 0;
                        self.entry += 1;
                        self.inline_text(&value)
                    };
                    progress(usage)
                }
                _ => SurfaceSemanticCensusStep::Complete,
            },
            Button(props) => {
                let usage = match self.container {
                    0 => self.inline_text(&props.icon),
                    1 => self.inline_text(&props.label.0),
                    _ => return SurfaceSemanticCensusStep::Complete,
                };
                self.container += 1;
                progress(usage)
            }
            Separator(_) | NumberStepper(_) => SurfaceSemanticCensusStep::Complete,
            Input(props) => {
                let usage = match self.container {
                    0 => self.inline_text(&props.value),
                    1 => props.placeholder.as_ref().map_or_else(SurfaceSemanticUsage::default, |value| self.inline_text(&value.0)),
                    2 => props.commit.as_ref().map_or_else(SurfaceSemanticUsage::default, |value| self.inline_text(&value)),
                    3 => props.accept.as_ref().map_or_else(SurfaceSemanticUsage::default, |value| self.inline_text(&value)),
                    _ => return SurfaceSemanticCensusStep::Complete,
                };
                self.container += 1;
                progress(usage)
            }
            Select(props) => match self.container {
                0 => {
                    self.container = 1;
                    progress(self.inline_text(&props.value))
                }
                1 => {
                    self.container = 2;
                    progress(self.backing::<ui_contract::SelectItem>(props.items.capacity()))
                }
                2 => {
                    let Some(item) = props.items.get(self.entry) else {
                        self.container = 3;
                        return progress(SurfaceSemanticUsage::default());
                    };
                    let usage = if self.data_attribute == 0 {
                        self.data_attribute = 1;
                        self.inline_text(&item.value)
                    } else {
                        self.data_attribute = 0;
                        self.entry += 1;
                        self.inline_text(&item.label.0)
                    };
                    progress(usage)
                }
                3 => {
                    self.container = 4;
                    progress(props.placeholder.as_ref().map_or_else(SurfaceSemanticUsage::default, |value| self.inline_text(&value.0)))
                }
                _ => SurfaceSemanticCensusStep::Complete,
            },
            Toggle(props) => {
                let usage = match self.container {
                    0 => self.inline_text(&props.icon),
                    1 => props.text.as_ref().map_or_else(SurfaceSemanticUsage::default, |value| self.inline_text(&value.0)),
                    _ => return SurfaceSemanticCensusStep::Complete,
                };
                self.container += 1;
                progress(usage)
            }
            KeyValueList(props) => match self.container {
                0 => {
                    self.container = 1;
                    progress(self.backing::<ui_contract::KeyValueEntry>(props.entries.capacity()))
                }
                1 => {
                    let Some(entry) = props.entries.get(self.entry) else { return SurfaceSemanticCensusStep::Complete };
                    let usage = if self.data_attribute == 0 {
                        self.data_attribute = 1;
                        self.inline_text(&entry.label.0)
                    } else {
                        self.data_attribute = 0;
                        self.entry += 1;
                        self.inline_text(&entry.value)
                    };
                    progress(usage)
                }
                _ => SurfaceSemanticCensusStep::Complete,
            },
            Slider(props) => {
                self.container += 1;
                if self.container == 1 {
                    progress(props.unit.as_ref().map_or_else(SurfaceSemanticUsage::default, |value| self.inline_text(&value)))
                } else {
                    SurfaceSemanticCensusStep::Complete
                }
            }
            Ring(props) => {
                self.container += 1;
                if self.container == 1 {
                    progress(self.inline_text(&props.orb_id))
                } else {
                    SurfaceSemanticCensusStep::Complete
                }
            }
            IconSelect(props) => {
                let usage = match self.container {
                    0 => self.inline_text(&props.value),
                    1 => self.inline_text(&props.classifier_kind),
                    _ => return SurfaceSemanticCensusStep::Complete,
                };
                self.container += 1;
                progress(usage)
            }
            Tree(props) => {
                self.container += 1;
                if self.container == 1 {
                    progress(props.interaction_domain.as_ref().map_or_else(SurfaceSemanticUsage::default, |value| self.inline_text(&value)))
                } else {
                    SurfaceSemanticCensusStep::Complete
                }
            }
            TreeSection(props) => {
                self.container += 1;
                if self.container == 1 {
                    progress(props.label.as_ref().map_or_else(SurfaceSemanticUsage::default, |value| self.inline_text(&value.0)))
                } else {
                    SurfaceSemanticCensusStep::Complete
                }
            }
            TreeItem(props) => match self.container {
                0 => {
                    self.container = 1;
                    progress(self.inline_text(&props.label.0))
                }
                1 => {
                    self.container = 2;
                    progress(props.description.as_ref().map_or_else(SurfaceSemanticUsage::default, |value| self.inline_text(&value)))
                }
                2 => {
                    self.container = 3;
                    progress(props.icon.as_ref().map_or_else(SurfaceSemanticUsage::default, |value| self.inline_text(&value)))
                }
                3 => {
                    self.container = 4;
                    progress(props.drag_data.as_ref().map_or_else(SurfaceSemanticUsage::default, |values| self.backing::<(ui_contract::UiText, ui_contract::UiText)>(values.capacity())))
                }
                4 => {
                    let Some((key, value)) = props.drag_data.as_ref().and_then(|values| values.get(self.entry)) else {
                        self.container = 5;
                        self.entry = 0;
                        return progress(SurfaceSemanticUsage::default());
                    };
                    let usage = if self.data_attribute == 0 {
                        self.data_attribute = 1;
                        self.inline_text(&key)
                    } else {
                        self.data_attribute = 0;
                        self.entry += 1;
                        self.inline_text(&value)
                    };
                    progress(usage)
                }
                5 => {
                    self.container = 6;
                    progress(self.backing::<ui_contract::RowAction>(props.row_actions.capacity()))
                }
                6 => {
                    let Some(action) = props.row_actions.get(self.entry) else { return SurfaceSemanticCensusStep::Complete };
                    let step = match self.data_attribute {
                        0 => progress(self.inline_text(&action.icon)),
                        1 => progress(action.label.as_ref().map_or_else(SurfaceSemanticUsage::default, |value| self.inline_text(&value.0))),
                        _ => self.binding_step(&action.action),
                    };
                    if matches!(step, SurfaceSemanticCensusStep::Complete) {
                        self.data_attribute = 0;
                        self.entry += 1;
                        return progress(SurfaceSemanticUsage::default());
                    }
                    self.data_attribute += 1;
                    step
                }
                _ => SurfaceSemanticCensusStep::Complete,
            },
            Image(props) => {
                let usage = match self.container {
                    0 => self.inline_text(&props.src),
                    1 => props.alt.as_ref().map_or_else(SurfaceSemanticUsage::default, |value| self.inline_text(&value.0)),
                    _ => return SurfaceSemanticCensusStep::Complete,
                };
                self.container += 1;
                progress(usage)
            }
            Surface(props) => match self.container {
                0 => {
                    self.container = 1;
                    progress(self.inline_text(&props.doc_schema))
                }
                1 => {
                    self.container = 2;
                    progress(self.backing::<u8>(props.doc.bytes.capacity()))
                }
                2 => {
                    self.container = 3;
                    progress(self.backing::<ui_contract::ActionBinding>(props.bindings.capacity()))
                }
                3 => self.bindings_step(&props.bindings),
                _ => SurfaceSemanticCensusStep::Complete,
            },
            Extension(props) => match self.container {
                0 => {
                    self.container = 1;
                    progress(self.inline_text(&props.extension))
                }
                1 => {
                    self.container = 2;
                    match self.push_value(&props.props) {
                        Ok(()) => progress(SurfaceSemanticUsage::default()),
                        Err(fault) => SurfaceSemanticCensusStep::Fault(fault),
                    }
                }
                _ => SurfaceSemanticCensusStep::Complete,
            },
        }
    }

    fn step(&mut self, node: &crate::TreeNode) -> SurfaceSemanticCensusStep {
        if self.string_byte > 0 {
            let bytes = self.string_byte.min(SURFACE_RECONCILE_PAGE_BYTES);
            self.string_byte -= bytes;
            return SurfaceSemanticCensusStep::Progress(SurfaceSemanticUsage { items: 0, bytes });
        }
        if let Some(step) = self.value_step() {
            return step;
        }
        match self.field {
            0 => {
                self.field = 1;
                let bytes = size_of::<crate::TreeNode>().checked_mul(SURFACE_RECONCILE_SEMANTIC_COPIES).unwrap_or(usize::MAX);
                SurfaceSemanticCensusStep::Progress(SurfaceSemanticUsage { items: SURFACE_RECONCILE_SEMANTIC_COPIES, bytes })
            }
            1 => {
                self.field = 2;
                SurfaceSemanticCensusStep::Progress(self.inline_text(&node.key))
            }
            2 => match self.component_step(&node.component) {
                SurfaceSemanticCensusStep::Complete => {
                    self.field = 3;
                    self.container = 0;
                    self.entry = 0;
                    self.binding = 0;
                    self.action = 0;
                    self.data_attribute = 0;
                    SurfaceSemanticCensusStep::Progress(SurfaceSemanticUsage::default())
                }
                step => step,
            },
            3 => {
                self.field = 4;
                SurfaceSemanticCensusStep::Progress(node.accessibility.label.as_ref().map_or_else(SurfaceSemanticUsage::default, |value| self.inline_text(&value.0)))
            }
            4 => {
                self.field = 5;
                SurfaceSemanticCensusStep::Progress(node.accessibility.description.as_ref().map_or_else(SurfaceSemanticUsage::default, |value| self.inline_text(&value.0)))
            }
            5 => {
                self.field = 6;
                SurfaceSemanticCensusStep::Progress(node.accessibility.shortcut.as_ref().map_or_else(SurfaceSemanticUsage::default, |value| self.inline_text(&value)))
            }
            6 => {
                self.field = 7;
                SurfaceSemanticCensusStep::Progress(self.backing::<ui_contract::ActionBinding>(node.bindings.capacity()))
            }
            7 => match self.bindings_step(&node.bindings) {
                SurfaceSemanticCensusStep::Complete => {
                    self.field = 8;
                    SurfaceSemanticCensusStep::Progress(SurfaceSemanticUsage::default())
                }
                step => step,
            },
            8 => {
                self.field = 9;
                SurfaceSemanticCensusStep::Progress(node.menu.as_ref().map_or_else(SurfaceSemanticUsage::default, |menu| self.inline_text(&menu.id)))
            }
            9 => {
                self.field = 10;
                if let Some(args) = node.menu.as_ref().and_then(|menu| menu.args.as_ref()) {
                    if let Err(fault) = self.push_value(args) {
                        return SurfaceSemanticCensusStep::Fault(fault);
                    }
                }
                SurfaceSemanticCensusStep::Progress(SurfaceSemanticUsage::default())
            }
            10 => {
                self.field = 11;
                SurfaceSemanticCensusStep::Progress(self.backing::<Option<Box<crate::TreeNode>>>(node.children.capacity()))
            }
            _ => SurfaceSemanticCensusStep::Complete,
        }
    }
}

/// ⏭️ Persistent one-node-at-a-time traversal and keyed differ for a single presented surface.
/// The retained reconciler is read-only until `Complete`; abandoning this cursor therefore abandons
/// every candidate identity, record, operation, and revision together.
pub(crate) struct SurfaceReconcileCursor {
    stage: SurfaceReconcileStage,
    surface: ui_contract::SurfaceId,
    base_revision: ui_contract::UiRevision,
    allocator: ui_contract::UiNodeIdAllocator,
    old_root: Option<ui_contract::UiNodeId>,
    pending_root: Option<crate::TreeNode>,
    traversal: SurfaceFixedVec<PresentationFrame, SURFACE_RECONCILE_VALUE_DEPTH>,
    overflow_frame: Option<PresentationFrame>,
    flat: SurfaceFixedVec<FlatPresentedNode, SURFACE_RECONCILE_FIXED_NODES>,
    postorder: SurfaceFixedVec<usize, SURFACE_RECONCILE_FIXED_NODES>,
    seen: SurfaceLinearSet<(Option<usize>, ui_contract::UiText), SURFACE_RECONCILE_FIXED_NODES>,
    ids: SurfaceFixedVec<ui_contract::UiNodeId, SURFACE_RECONCILE_FIXED_NODES>,
    allocate_index: usize,
    diff_index: usize,
    record_diff: Option<RecordDiffCursor>,
    assembly: ui_contract::UiDocumentAssembly,
    assembly_open: bool,
    assembly_surface: Option<ui_contract::SurfaceId>,
    assembly_credit: Option<ui_contract::UiResidentPermit>,
    assembly_generation: u64,
    new_ordinals: SurfaceLinearMap<ui_contract::UiNodeId, usize, SURFACE_RECONCILE_FIXED_NODES>,
    new_key_index: SurfaceLinearMap<NodeIdentity, ui_contract::UiNodeId, SURFACE_RECONCILE_FIXED_NODES>,
    remove_next: Option<ui_contract::UiNodeId>,
    removal: SurfaceFixedVec<RemovalFrame, SURFACE_RECONCILE_FIXED_NODES>,
    ops: ui_contract::UiPatchOps,
    pending_op: ui_contract::UiPendingPatchOp,
    root_patch_prepared: bool,
    limits: SurfaceReconcileLimits,
    usage: SurfaceReconcileUsage,
    held_node: Option<(Option<usize>, crate::TreeNode)>,
    semantic_census: Option<SurfaceSemanticCensusCursor>,
    semantic_usage: SurfaceSemanticUsage,
    fault: Option<SurfaceReconcileFault>,
    retire_tree: SurfaceTreeRetireCursor,
    retire_fresh_field: u8,
    retire_record_field: u8,
    #[cfg(test)]
    fail_binding_step: bool,
    #[cfg(test)]
    fail_component_step: bool,
}

impl SurfaceReconcileCursor {
    pub(crate) fn new(tree: crate::ComponentTree, current: &SurfaceReconciler) -> Self {
        Self::new_with_limits(tree, current, SurfaceReconcileLimits::default())
    }

    pub(crate) fn new_with_limits(tree: crate::ComponentTree, current: &SurfaceReconciler, limits: SurfaceReconcileLimits) -> Self {
        let credit = reserve_surface_reconcile(limits);
        Self::new_admitted(tree, current, limits, current.revision.0.checked_add(1).unwrap_or(0), credit)
    }

    fn new_admitted(tree: crate::ComponentTree, current: &SurfaceReconciler, limits: SurfaceReconcileLimits, generation: u64, credit: Option<ui_contract::UiResidentPermit>) -> Self {
        Self {
            stage: SurfaceReconcileStage::TraversePresentation,
            surface: current.surface.clone(),
            base_revision: current.revision,
            allocator: current.allocator.clone(),
            old_root: current.root,
            pending_root: Some(tree.root),
            traversal: SurfaceFixedVec::default(),
            overflow_frame: None,
            flat: SurfaceFixedVec::default(),
            postorder: SurfaceFixedVec::default(),
            seen: SurfaceLinearSet::default(),
            ids: SurfaceFixedVec::default(),
            allocate_index: 0,
            diff_index: 0,
            record_diff: None,
            assembly: ui_contract::UiDocumentAssembly::default(),
            assembly_open: false,
            assembly_surface: Some(current.surface.clone()),
            assembly_credit: credit,
            assembly_generation: generation,
            new_ordinals: SurfaceLinearMap::default(),
            new_key_index: SurfaceLinearMap::default(),
            remove_next: None,
            removal: SurfaceFixedVec::default(),
            ops: ui_contract::UiPatchOps::default(),
            pending_op: ui_contract::UiPendingPatchOp::default(),
            root_patch_prepared: false,
            limits,
            usage: SurfaceReconcileUsage { nodes: 0, items: 1, bytes: 0 },
            held_node: None,
            semantic_census: None,
            semantic_usage: SurfaceSemanticUsage::default(),
            fault: None,
            retire_tree: SurfaceTreeRetireCursor::default(),
            retire_fresh_field: 0,
            retire_record_field: 0,
            #[cfg(test)]
            fail_binding_step: false,
            #[cfg(test)]
            fail_component_step: false,
        }
    }

    pub(crate) fn step(&mut self, current: &SurfaceReconciler) -> SurfaceReconcileStep {
        if let Some(fault) = self.fault.clone() {
            return SurfaceReconcileStep::Fault(fault);
        }
        if current.revision != self.base_revision {
            let fault = SurfaceReconcileFault::StaleGeneration { expected: self.base_revision.0, actual: current.revision.0 };
            self.fault = Some(fault.clone());
            return SurfaceReconcileStep::Fault(fault);
        }
        if self.pending_op.get().is_some() {
            let returning_component = self.record_diff.as_ref().is_some_and(|diff| diff.fresh.is_none() && diff.field == 0 && diff.owned_copy.as_ref().and_then(RecordOwnedCopy::component).is_some_and(ui_contract::UiComponentCopy::terminal_is_empty));
            let step = if returning_component { self.advance_existing_component(current) } else { self.advance_pending_patch() };
            #[cfg(test)]
            if self.fail_component_step { panic!("injected retained component output callback unwind"); }
            return step;
        }
        let step = match self.stage {
            SurfaceReconcileStage::TraversePresentation => {
                if self.held_node.is_none() {
                    if let Some(root) = self.pending_root.take() {
                        self.held_node = Some((None, root));
                        self.semantic_census = Some(SurfaceSemanticCensusCursor::default());
                        self.semantic_usage = SurfaceSemanticUsage::default();
                        return SurfaceReconcileStep::Yield { nodes: 0, bytes: 0 };
                    }
                    if let Some(frame) = self.traversal.last_mut() {
                        if let Some(child) = frame.children.next() {
                            self.held_node = Some((Some(frame.index), child));
                            self.semantic_census = Some(SurfaceSemanticCensusCursor::default());
                            self.semantic_usage = SurfaceSemanticUsage::default();
                            return SurfaceReconcileStep::Yield { nodes: 0, bytes: size_of::<usize>() };
                        }
                        let Some(complete) = self.traversal.pop() else {
                            self.fault = Some(SurfaceReconcileFault::CounterOverflow);
                            return SurfaceReconcileStep::Fault(SurfaceReconcileFault::CounterOverflow);
                        };
                        if self.postorder.try_push(complete.index).is_err() {
                            self.overflow_frame = Some(complete);
                            let fault = SurfaceReconcileFault::Credits { usage: SurfaceReconcileUsage { nodes: self.limits.max_nodes.checked_add(1).unwrap_or(usize::MAX), ..self.usage }, limits: self.limits };
                            self.fault = Some(fault.clone());
                            return SurfaceReconcileStep::Fault(fault);
                        }
                        return SurfaceReconcileStep::Yield { nodes: 0, bytes: size_of::<usize>() };
                    }
                    self.stage = SurfaceReconcileStage::AllocateIdentities;
                    return SurfaceReconcileStep::Yield { nodes: 0, bytes: 0 };
                }
                if let Some((_, node)) = self.held_node.as_ref() {
                    let key_bytes = node.key.len();
                    if key_bytes > self.limits.max_identifier_bytes {
                        let fault = SurfaceReconcileFault::IdentifierBytes { actual: key_bytes, max: self.limits.max_identifier_bytes };
                        self.fault = Some(fault.clone());
                        return SurfaceReconcileStep::Fault(fault);
                    }
                    if self.flat.len() >= self.limits.max_nodes {
                        let usage = SurfaceReconcileUsage { nodes: self.flat.len().checked_add(1).unwrap_or(usize::MAX), items: self.usage.items, bytes: self.usage.bytes };
                        let fault = SurfaceReconcileFault::Credits { usage, limits: self.limits };
                        self.fault = Some(fault.clone());
                        return SurfaceReconcileStep::Fault(fault);
                    }
                    let Some(semantic_census) = self.semantic_census.as_mut() else {
                        self.fault = Some(SurfaceReconcileFault::CounterOverflow);
                        return SurfaceReconcileStep::Fault(SurfaceReconcileFault::CounterOverflow);
                    };
                    let semantic = semantic_census.step(node);
                    match semantic {
                        SurfaceSemanticCensusStep::Progress(delta) => {
                            let Some(semantic_items) = self.semantic_usage.items.checked_add(delta.items) else {
                                self.fault = Some(SurfaceReconcileFault::CounterOverflow);
                                return SurfaceReconcileStep::Fault(SurfaceReconcileFault::CounterOverflow);
                            };
                            let Some(semantic_bytes) = self.semantic_usage.bytes.checked_add(delta.bytes) else {
                                self.fault = Some(SurfaceReconcileFault::CounterOverflow);
                                return SurfaceReconcileStep::Fault(SurfaceReconcileFault::CounterOverflow);
                            };
                            self.semantic_usage.items = semantic_items;
                            self.semantic_usage.bytes = semantic_bytes;
                            let Some(node_page_bytes) = size_of::<FlatPresentedNode>().checked_add(self.semantic_usage.bytes) else {
                                self.fault = Some(SurfaceReconcileFault::CounterOverflow);
                                return SurfaceReconcileStep::Fault(SurfaceReconcileFault::CounterOverflow);
                            };
                            let Some(projected_nodes) = self.usage.nodes.checked_add(1) else {
                                self.fault = Some(SurfaceReconcileFault::CounterOverflow);
                                return SurfaceReconcileStep::Fault(SurfaceReconcileFault::CounterOverflow);
                            };
                            let Some(projected_items) = self.usage.items.checked_add(self.semantic_usage.items) else {
                                self.fault = Some(SurfaceReconcileFault::CounterOverflow);
                                return SurfaceReconcileStep::Fault(SurfaceReconcileFault::CounterOverflow);
                            };
                            let Some(projected_bytes) = self.usage.bytes.checked_add(node_page_bytes) else {
                                self.fault = Some(SurfaceReconcileFault::CounterOverflow);
                                return SurfaceReconcileStep::Fault(SurfaceReconcileFault::CounterOverflow);
                            };
                            let projected = SurfaceReconcileUsage { nodes: projected_nodes, items: projected_items, bytes: projected_bytes };
                            if !projected.fits(self.limits) {
                                let fault = SurfaceReconcileFault::Credits { usage: projected, limits: self.limits };
                                self.fault = Some(fault.clone());
                                return SurfaceReconcileStep::Fault(fault);
                            }
                            return SurfaceReconcileStep::Yield { nodes: 0, bytes: delta.bytes };
                        }
                        SurfaceSemanticCensusStep::Fault(fault) => {
                            self.fault = Some(fault.clone());
                            return SurfaceReconcileStep::Fault(fault);
                        }
                        SurfaceSemanticCensusStep::Complete => {}
                    }
                    let Some(node_page_bytes) = size_of::<FlatPresentedNode>().checked_add(self.semantic_usage.bytes) else {
                        self.fault = Some(SurfaceReconcileFault::CounterOverflow);
                        return SurfaceReconcileStep::Fault(SurfaceReconcileFault::CounterOverflow);
                    };
                    let Some(projected_nodes) = self.usage.nodes.checked_add(1) else {
                        self.fault = Some(SurfaceReconcileFault::CounterOverflow);
                        return SurfaceReconcileStep::Fault(SurfaceReconcileFault::CounterOverflow);
                    };
                    let Some(projected_items) = self.usage.items.checked_add(self.semantic_usage.items) else {
                        self.fault = Some(SurfaceReconcileFault::CounterOverflow);
                        return SurfaceReconcileStep::Fault(SurfaceReconcileFault::CounterOverflow);
                    };
                    let Some(projected_bytes) = self.usage.bytes.checked_add(node_page_bytes) else {
                        self.fault = Some(SurfaceReconcileFault::CounterOverflow);
                        return SurfaceReconcileStep::Fault(SurfaceReconcileFault::CounterOverflow);
                    };
                    let projected = SurfaceReconcileUsage { nodes: projected_nodes, items: projected_items, bytes: projected_bytes };
                    let Some((parent, node)) = self.held_node.take() else {
                        self.fault = Some(SurfaceReconcileFault::CounterOverflow);
                        return SurfaceReconcileStep::Fault(SurfaceReconcileFault::CounterOverflow);
                    };
                    let mut node = node;
                    self.semantic_census = None;
                    match self.seen.try_insert((parent, node.key.clone())) {
                        Ok(true) => {}
                        Ok(false) => {
                            self.held_node = Some((parent, node));
                            self.fault = Some(SurfaceReconcileFault::DuplicateSiblingKey);
                            return SurfaceReconcileStep::Fault(SurfaceReconcileFault::DuplicateSiblingKey);
                        }
                        Err(_) => {
                            self.held_node = Some((parent, node));
                            let fault = SurfaceReconcileFault::Credits { usage: SurfaceReconcileUsage { nodes: self.limits.max_nodes.checked_add(1).unwrap_or(usize::MAX), ..self.usage }, limits: self.limits };
                            self.fault = Some(fault.clone());
                            return SurfaceReconcileStep::Fault(fault);
                        }
                    }
                    let children = take(&mut node.children).into_iter();
                    let index = self.flat.len();
                    let flat = FlatPresentedNode { parent, node, child_ids: ui_contract::UiNodeChildren::default() };
                    if let Err(flat) = self.flat.try_push(flat) {
                        self.held_node = Some((parent, flat.node));
                        self.overflow_frame = Some(PresentationFrame { index, children });
                        let fault = SurfaceReconcileFault::Credits { usage: SurfaceReconcileUsage { nodes: self.limits.max_nodes.checked_add(1).unwrap_or(usize::MAX), ..self.usage }, limits: self.limits };
                        self.fault = Some(fault.clone());
                        return SurfaceReconcileStep::Fault(fault);
                    }
                    if let Err(frame) = self.traversal.try_push(PresentationFrame { index, children }) {
                        self.overflow_frame = Some(frame);
                        let fault = SurfaceReconcileFault::ValueDepth { actual: SURFACE_RECONCILE_VALUE_DEPTH.checked_add(1).unwrap_or(usize::MAX), max: SURFACE_RECONCILE_VALUE_DEPTH };
                        self.fault = Some(fault.clone());
                        return SurfaceReconcileStep::Fault(fault);
                    }
                    self.usage = projected;
                    SurfaceReconcileStep::Yield { nodes: 0, bytes: 0 }
                } else {
                    SurfaceReconcileStep::Yield { nodes: 0, bytes: 0 }
                }
            }
            SurfaceReconcileStage::AllocateIdentities => {
                if self.allocate_index < self.flat.len() {
                    let parent_index = self.flat[self.allocate_index].parent;
                    let parent = parent_index.map(|index| self.ids[index]);
                    let key = self.flat[self.allocate_index].node.key.clone();
                    let key_bytes = key.len();
                    let identity = (parent, key);
                    let id = match current.key_index.get(&identity).copied() {
                        Some(id) => id,
                        None => {
                            let Some(id) = self.allocator.try_allocate() else {
                                self.fault = Some(SurfaceReconcileFault::CounterOverflow);
                                return SurfaceReconcileStep::Fault(SurfaceReconcileFault::CounterOverflow);
                            };
                            id
                        }
                    };
                    if self.new_key_index.try_insert(identity, id).is_err() {
                        let fault = SurfaceReconcileFault::Credits { usage: SurfaceReconcileUsage { nodes: self.limits.max_nodes.checked_add(1).unwrap_or(usize::MAX), ..self.usage }, limits: self.limits };
                        self.fault = Some(fault.clone());
                        return SurfaceReconcileStep::Fault(fault);
                    }
                    if self.ids.try_push(id).is_err() {
                        let fault = SurfaceReconcileFault::Credits { usage: SurfaceReconcileUsage { nodes: self.limits.max_nodes.checked_add(1).unwrap_or(usize::MAX), ..self.usage }, limits: self.limits };
                        self.fault = Some(fault.clone());
                        return SurfaceReconcileStep::Fault(fault);
                    }
                    if let Some(parent) = parent_index {
                        if self.flat[parent].child_ids.try_push(id).is_err() {
                            let fault = SurfaceReconcileFault::Credits { usage: SurfaceReconcileUsage { nodes: self.limits.max_nodes.checked_add(1).unwrap_or(usize::MAX), ..self.usage }, limits: self.limits };
                            self.fault = Some(fault.clone());
                            return SurfaceReconcileStep::Fault(fault);
                        }
                    }
                    self.allocate_index += 1;
                    let Some(bytes) = size_of::<NodeIdentity>().checked_add(key_bytes) else {
                        self.fault = Some(SurfaceReconcileFault::CounterOverflow);
                        return SurfaceReconcileStep::Fault(SurfaceReconcileFault::CounterOverflow);
                    };
                    SurfaceReconcileStep::Yield { nodes: 0, bytes }
                } else {
                    self.stage = SurfaceReconcileStage::DiffRecords;
                    SurfaceReconcileStep::Yield { nodes: 0, bytes: 0 }
                }
            }
            SurfaceReconcileStage::DiffRecords => {
                if self.record_diff.as_ref().is_some_and(|diff| diff.field >= 8 && diff.fresh.is_none()) { return self.advance_record_assembly(); }
                if self.record_diff.as_ref().is_some_and(|diff| diff.fresh.is_none() && diff.field == 0) {
                    let step = self.advance_existing_component(current);
                    #[cfg(test)]
                    if self.fail_component_step { panic!("injected existing component callback unwind"); }
                    return step;
                }
                if self.record_diff.as_ref().is_some_and(|diff| diff.fresh.is_some() && diff.field == 1) {
                    let step = Self::advance_fresh_component(self.record_diff.as_mut().expect("checked retained record"), &mut self.usage, self.limits, &mut self.fault);
                    #[cfg(test)]
                    if self.fail_component_step { panic!("injected component callback unwind"); }
                    return step;
                }
                if self.record_diff.as_ref().is_some_and(|diff| diff.fresh.is_some() && diff.field == 5) {
                    let step = Self::advance_fresh_bindings(self.record_diff.as_mut().expect("checked retained record"), &mut self.usage, self.limits, &mut self.fault);
                    #[cfg(test)]
                    if self.fail_binding_step { panic!("injected binding callback unwind"); }
                    return step;
                }
                if let Some(mut diff) = self.record_diff.take() {
                    if let Some(fresh) = diff.fresh.as_mut() {
                        let copied = match diff.field {
                            0 => {
                                fresh.key = Some(diff.record.key.clone());
                                true
                            }
                            2 => {
                                fresh.layout = Some(diff.record.layout.clone());
                                true
                            }
                            3 => {
                                fresh.children = Some(diff.record.children.clone());
                                true
                            }
                            4 => {
                                fresh.accessibility = Some(diff.record.accessibility.clone());
                                true
                            }
                            6 => match diff.record.menu.as_ref() {
                                Some(menu) => match menu.credited_clone() {
                                    Some(menu) => {
                                        fresh.menu = Some(Some(menu));
                                        true
                                    }
                                    None => false,
                                },
                                None => {
                                    fresh.menu = Some(None);
                                    true
                                }
                            },
                            _ => {
                                let Some(mut fresh) = diff.fresh.take() else {
                                    self.record_diff = Some(diff);
                                    self.fault = Some(SurfaceReconcileFault::CounterOverflow);
                                    return SurfaceReconcileStep::Fault(SurfaceReconcileFault::CounterOverflow);
                                };
                                let Some(key) = fresh.key.take() else {
                                    diff.fresh = Some(fresh);
                                    self.record_diff = Some(diff);
                                    self.fault = Some(SurfaceReconcileFault::CounterOverflow);
                                    return SurfaceReconcileStep::Fault(SurfaceReconcileFault::CounterOverflow);
                                };
                                let Some(component) = fresh.component.take() else {
                                    fresh.key = Some(key);
                                    diff.fresh = Some(fresh);
                                    self.record_diff = Some(diff);
                                    self.fault = Some(SurfaceReconcileFault::CounterOverflow);
                                    return SurfaceReconcileStep::Fault(SurfaceReconcileFault::CounterOverflow);
                                };
                                let Some(layout) = fresh.layout.take() else {
                                    fresh.key = Some(key);
                                    fresh.component = Some(component);
                                    diff.fresh = Some(fresh);
                                    self.record_diff = Some(diff);
                                    self.fault = Some(SurfaceReconcileFault::CounterOverflow);
                                    return SurfaceReconcileStep::Fault(SurfaceReconcileFault::CounterOverflow);
                                };
                                let Some(accessibility) = fresh.accessibility.take() else {
                                    fresh.key = Some(key);
                                    fresh.component = Some(component);
                                    fresh.layout = Some(layout);
                                    diff.fresh = Some(fresh);
                                    self.record_diff = Some(diff);
                                    self.fault = Some(SurfaceReconcileFault::CounterOverflow);
                                    return SurfaceReconcileStep::Fault(SurfaceReconcileFault::CounterOverflow);
                                };
                                let Some(bindings) = fresh.bindings.take() else {
                                    fresh.key = Some(key);
                                    fresh.component = Some(component);
                                    fresh.layout = Some(layout);
                                    fresh.accessibility = Some(accessibility);
                                    diff.fresh = Some(fresh);
                                    self.record_diff = Some(diff);
                                    self.fault = Some(SurfaceReconcileFault::CounterOverflow);
                                    return SurfaceReconcileStep::Fault(SurfaceReconcileFault::CounterOverflow);
                                };
                                let Some(menu) = fresh.menu.take() else {
                                    fresh.key = Some(key);
                                    fresh.component = Some(component);
                                    fresh.layout = Some(layout);
                                    fresh.accessibility = Some(accessibility);
                                    fresh.bindings = Some(bindings);
                                    diff.fresh = Some(fresh);
                                    self.record_diff = Some(diff);
                                    self.fault = Some(SurfaceReconcileFault::CounterOverflow);
                                    return SurfaceReconcileStep::Fault(SurfaceReconcileFault::CounterOverflow);
                                };
                                let Some(children) = fresh.children.take() else {
                                    fresh.key = Some(key);
                                    fresh.component = Some(component);
                                    fresh.layout = Some(layout);
                                    fresh.accessibility = Some(accessibility);
                                    fresh.bindings = Some(bindings);
                                    fresh.menu = Some(menu);
                                    diff.fresh = Some(fresh);
                                    self.record_diff = Some(diff);
                                    self.fault = Some(SurfaceReconcileFault::CounterOverflow);
                                    return SurfaceReconcileStep::Fault(SurfaceReconcileFault::CounterOverflow);
                                };
                                let op = ui_contract::UiPatchOp::Upsert(ui_contract::UiNodeRecord {
                                    id: diff.record.id,
                                    key,
                                    component,
                                    layout,
                                    style: diff.record.style,
                                    activity: diff.record.activity,
                                    disabled: diff.record.disabled,
                                    transition: diff.record.transition,
                                    accessibility,
                                    bindings,
                                    menu,
                                    children,
                                });
                                *self.pending_op.source_mut().expect("active cursor owns writable pending patch") = Some(op);
                                true
                            }
                        };
                        if !copied {
                            self.record_diff = Some(diff);
                            self.fault = Some(SurfaceReconcileFault::AliasCapacity);
                            return SurfaceReconcileStep::Fault(SurfaceReconcileFault::AliasCapacity);
                        }
                        diff.field += 1;
                        self.record_diff = Some(diff);
                        return SurfaceReconcileStep::Yield { nodes: 0, bytes: if self.pending_op.get().is_some() { size_of::<ui_contract::UiPatchOp>() } else { 0 } };
                    }
                    if diff.field < 8 {
                        let old = match current.read_record(diff.id) {
                            Ok(Some(old)) => old,
                            Err(ui_contract::UiDocumentLeaseError::Contended) => { self.record_diff = Some(diff); return SurfaceReconcileStep::Yield { nodes: 0, bytes: 0 }; }
                            _ => {
                            self.record_diff = Some(diff);
                            self.fault = Some(SurfaceReconcileFault::AliasCapacity);
                            return SurfaceReconcileStep::Fault(SurfaceReconcileFault::AliasCapacity);
                            }
                        };
                        let op = match diff_record_field(&old, &diff.record, diff.field) {
                            Ok(op) => op,
                            Err(fault) => {
                                self.record_diff = Some(diff);
                                self.fault = Some(fault.clone());
                                return SurfaceReconcileStep::Fault(fault);
                            }
                        };
                        diff.field += 1;
                        *self.pending_op.source_mut().expect("active cursor owns writable pending patch") = op;
                        self.record_diff = Some(diff);
                        return SurfaceReconcileStep::Yield { nodes: 0, bytes: if self.pending_op.get().is_some() { size_of::<ui_contract::UiPatchOp>() } else { 0 } };
                    }
                    self.record_diff = Some(diff);
                    self.advance_record_assembly()
                } else if self.diff_index < self.flat.len() {
                    let index = self.postorder[self.diff_index];
                    let id = self.ids[index];
                    let children = take(&mut self.flat[index].child_ids);
                    let transition = match current.read_record(id) {
                        Ok(record) => record.and_then(|record| record.transition),
                        Err(ui_contract::UiDocumentLeaseError::Contended) => { self.flat[index].child_ids = children; return SurfaceReconcileStep::Yield { nodes: 0, bytes: 0 }; }
                        Err(_) => { self.flat[index].child_ids = children; return self.fail(SurfaceReconcileFault::AliasCapacity); }
                    };
                    let node = std::mem::replace(&mut self.flat[index].node, crate::TreeNode::empty_separator());
                    let record = build_record_owned(id, node, children, transition);
                    let fresh = (!current.ordinals.contains_key(&id)).then(FreshRecordClone::default);
                    self.record_diff = Some(RecordDiffCursor { id, record: record.into(), field: 0, fresh, owned_copy: None });
                    SurfaceReconcileStep::Yield { nodes: 0, bytes: 0 }
                } else {
                    self.remove_next = self.old_root;
                    self.stage = SurfaceReconcileStage::RemoveStale;
                    SurfaceReconcileStep::Yield { nodes: 0, bytes: 0 }
                }
            }
            SurfaceReconcileStage::RemoveStale => {
                if let Some(id) = self.remove_next.take() {
                    if self.new_ordinals.contains_key(&id) {
                        if self.removal.try_push(RemovalFrame { id, next_child: 0 }).is_err() {
                            self.remove_next = Some(id);
                            let fault = SurfaceReconcileFault::Credits { usage: SurfaceReconcileUsage { nodes: self.limits.max_nodes.checked_add(1).unwrap_or(usize::MAX), ..self.usage }, limits: self.limits };
                            self.fault = Some(fault.clone());
                            return SurfaceReconcileStep::Fault(fault);
                        }
                    } else {
                        let op = ui_contract::UiPatchOp::Remove { id };
                        *self.pending_op.source_mut().expect("active cursor owns writable pending patch") = Some(op);
                        return SurfaceReconcileStep::Yield { nodes: 0, bytes: size_of::<ui_contract::UiPatchOp>() };
                    }
                    SurfaceReconcileStep::Yield { nodes: 0, bytes: size_of::<ui_contract::UiNodeId>() }
                } else if let Some(frame) = self.removal.last_mut() {
                    let child = match current.read_record(frame.id) {
                        Ok(record) => record.and_then(|record| record.children.get(frame.next_child).copied()),
                        Err(ui_contract::UiDocumentLeaseError::Contended) => return SurfaceReconcileStep::Yield { nodes: 0, bytes: 0 },
                        Err(_) => return self.fail(SurfaceReconcileFault::AliasCapacity),
                    };
                    if let Some(child) = child {
                        frame.next_child += 1;
                        self.remove_next = Some(child);
                    } else {
                        self.removal.pop();
                    }
                    SurfaceReconcileStep::Yield { nodes: 0, bytes: size_of::<ui_contract::UiNodeId>() }
                } else {
                    self.stage = SurfaceReconcileStage::Finalize;
                    SurfaceReconcileStep::Yield { nodes: 0, bytes: 0 }
                }
            }
            SurfaceReconcileStage::Finalize => {
                let new_root = self.ids.first().copied();
                if !self.root_patch_prepared && self.old_root != new_root {
                    self.root_patch_prepared = true;
                    if let Some(id) = new_root {
                        *self.pending_op.source_mut().expect("active cursor owns writable pending patch") = Some(ui_contract::UiPatchOp::SetRoot { id });
                        return SurfaceReconcileStep::Yield { nodes: 0, bytes: size_of::<ui_contract::UiPatchOp>() };
                    }
                }
                let revision = if self.ops.is_empty() {
                    self.base_revision
                } else {
                    let Some(revision) = self.base_revision.try_next() else {
                        self.fault = Some(SurfaceReconcileFault::CounterOverflow);
                        return SurfaceReconcileStep::Fault(SurfaceReconcileFault::CounterOverflow);
                    };
                    revision
                };
                let patch = if self.ops.is_empty() { None } else { Some(ui_contract::UiPatch { surface: self.surface.clone(), base_revision: self.base_revision, revision, ops: take(&mut self.ops) }) };
                let reconciler = SurfaceReconciler {
                    surface: self.surface.clone(),
                    revision,
                    allocator: self.allocator.clone(),
                    document: None,
                    assembly: take(&mut self.assembly),
                    ordinals: self.new_ordinals.take_all(),
                    key_index: self.new_key_index.take_all(),
                    root: new_root,
                    retire_scalar: 0,
                    seal_phase: 0,
                    handback: None,
                    retirement_armed: true,
                };
                SurfaceReconcileStep::Complete { reconciler, patch }
            }
        };
        if let SurfaceReconcileStep::Yield { nodes, bytes } = step {
            if bytes > SURFACE_RECONCILE_PAGE_BYTES {
                let fault = SurfaceReconcileFault::PageBytes { actual: bytes, max: SURFACE_RECONCILE_PAGE_BYTES };
                self.fault = Some(fault.clone());
                return SurfaceReconcileStep::Fault(fault);
            }
            let Some(items) = nodes.checked_mul(8) else {
                self.fault = Some(SurfaceReconcileFault::CounterOverflow);
                return SurfaceReconcileStep::Fault(SurfaceReconcileFault::CounterOverflow);
            };
            if !self.usage.include(nodes, items, bytes) || !self.usage.fits(self.limits) {
                let fault = SurfaceReconcileFault::Credits { usage: self.usage, limits: self.limits };
                self.fault = Some(fault.clone());
                return SurfaceReconcileStep::Fault(fault);
            }
        }
        step
    }

    fn clear_fault(&mut self) {
        self.fault = None;
    }

    fn fail(&mut self, fault: SurfaceReconcileFault) -> SurfaceReconcileStep {
        self.fault = Some(fault.clone());
        SurfaceReconcileStep::Fault(fault)
    }

    fn advance_existing_component(&mut self, current: &SurfaceReconciler) -> SurfaceReconcileStep {
        self.advance_existing_component_with_grant(current, SURFACE_COMPONENT_COPY_WORK_BYTES)
    }

    fn advance_existing_component_with_grant(&mut self, current: &SurfaceReconciler, work: usize) -> SurfaceReconcileStep {
        if work == 0 { return SurfaceReconcileStep::Yield { nodes: 0, bytes: 0 }; }
        let required = size_of::<ExistingComponentComparison>();
        if self.record_diff.as_ref().unwrap().owned_copy.is_none() {
            let mut projected = self.usage;
            if !projected.include(0, 1, required) || !projected.fits(self.limits) {
                return self.fail(SurfaceReconcileFault::Credits { usage: projected, limits: self.limits });
            }
            if required > work { return SurfaceReconcileStep::Yield { nodes: 0, bytes: 0 }; }
            self.record_diff.as_mut().unwrap().owned_copy = Some(RecordOwnedCopy::Comparison(ExistingComponentComparison::default()));
            return SurfaceReconcileStep::Yield { nodes: 0, bytes: required };
        }
        let diff = self.record_diff.as_mut().unwrap();
        if let Some(RecordOwnedCopy::Comparison(comparison)) = diff.owned_copy.as_mut() {
            if comparison.changed.is_none() {
                if let Some(equal) = comparison.cursor.result() {
                    comparison.cursor.release_reads();
                    comparison.changed = Some(!equal);
                    return SurfaceReconcileStep::Yield { nodes: 0, bytes: size_of::<bool>() };
                }
                if comparison.lease.is_none() {
                    return match current.capture_document(&mut comparison.lease, work) {
                        Ok(true) => SurfaceReconcileStep::Yield { nodes: 0, bytes: size_of::<ui_contract::UiDocumentLease>() },
                        Ok(false) | Err(ui_contract::UiDocumentLeaseError::Contended) => SurfaceReconcileStep::Yield { nodes: 0, bytes: 0 },
                        Err(_) => self.fail(SurfaceReconcileFault::AliasCapacity),
                    };
                }
                let Some(ordinal) = current.ordinals.get(&diff.id).copied() else { return self.fail(SurfaceReconcileFault::AliasCapacity); };
                let progress = (|| {
                    let read = comparison.lease.as_ref().unwrap().try_read()?;
                    let old = read.exact_node(ordinal, diff.id)?;
                    comparison.cursor.advance(&old.component, &diff.record.component, work).map_err(|_| ui_contract::UiDocumentLeaseError::NodeIdentity)
                })();
                return match progress {
                    Ok(progress) => SurfaceReconcileStep::Yield { nodes: 0, bytes: progress.compared_bytes },
                    Err(ui_contract::UiDocumentLeaseError::Contended) => SurfaceReconcileStep::Yield { nodes: 0, bytes: 0 },
                    Err(_) => self.fail(SurfaceReconcileFault::AliasCapacity),
                };
            }
            if let Some(lease) = comparison.lease.as_mut() {
                if lease.terminal_is_empty() {
                    if work < size_of::<ui_contract::UiDocumentLease>() { return SurfaceReconcileStep::Yield { nodes: 0, bytes: 0 }; }
                    comparison.lease = None;
                    return SurfaceReconcileStep::Yield { nodes: 0, bytes: size_of::<ui_contract::UiDocumentLease>() };
                }
                return match lease.close_read_step_with_grant(1, work) {
                    Ok(progress) => SurfaceReconcileStep::Yield { nodes: 0, bytes: progress.released_bytes },
                    Err(_) => self.fail(SurfaceReconcileFault::AliasCapacity),
                };
            }
            if work < required { return SurfaceReconcileStep::Yield { nodes: 0, bytes: 0 }; }
            let changed = comparison.changed.unwrap();
            diff.owned_copy = Some(RecordOwnedCopy::Changed(changed));
            return SurfaceReconcileStep::Yield { nodes: 0, bytes: required };
        }
        if let Some(RecordOwnedCopy::Changed(changed)) = diff.owned_copy.as_ref() {
            if !changed { diff.owned_copy = None; diff.field = 1; return SurfaceReconcileStep::Yield { nodes: 0, bytes: size_of::<bool>() }; }
            let initialized = size_of::<ui_contract::UiComponentCopy>() + size_of::<ui_contract::Component>();
            if initialized > SURFACE_RECONCILE_PAGE_BYTES || work < size_of::<ui_contract::Component>() { return SurfaceReconcileStep::Yield { nodes: 0, bytes: 0 }; }
            let source = std::mem::replace(&mut diff.record.component, ui_contract::Component::Separator(ui_contract::SeparatorProps {}));
            diff.owned_copy = Some(RecordOwnedCopy::Component(ui_contract::UiComponentCopy::new(source)));
            return SurfaceReconcileStep::Yield { nodes: 0, bytes: initialized };
        }
        let copy = diff.owned_copy.as_mut().and_then(RecordOwnedCopy::component_mut).expect("changed component retains its copy cursor");
        if copy.terminal_is_empty() {
            diff.owned_copy = None;
            diff.field = 1;
            return SurfaceReconcileStep::Yield { nodes: 0, bytes: size_of::<ui_contract::UiComponentCopy>() };
        }
        if copy.candidate().is_some() {
            if work < size_of::<ui_contract::Component>() { return SurfaceReconcileStep::Yield { nodes: 0, bytes: 0 }; }
            if let Some(source) = copy.take_completed_source_with_grant(work) {
                diff.record.component = source;
                return SurfaceReconcileStep::Yield { nodes: 0, bytes: size_of::<ui_contract::Component>() };
            }
            let target = self.pending_op.source_mut().expect("active pending operation is writable");
            if target.is_some() || size_of::<ui_contract::UiPatchOp>() > SURFACE_RECONCILE_PAGE_BYTES { return SurfaceReconcileStep::Yield { nodes: 0, bytes: 0 }; }
            let candidate = copy.take_completed_candidate_with_grant(work).expect("completed candidate remains in its exact owner");
            *target = Some(ui_contract::UiPatchOp::SetComponent { id: diff.id, component: candidate });
            return SurfaceReconcileStep::Yield { nodes: 0, bytes: size_of::<ui_contract::UiPatchOp>() };
        }
        let requested = match copy.next_allocation_bytes() { Ok(bytes) => bytes, Err(_) => return self.fail(SurfaceReconcileFault::CounterOverflow) };
        let mut projected = self.usage;
        if requested > SURFACE_RECONCILE_PAGE_BYTES || !projected.include(0, usize::from(requested != 0), requested) || !projected.fits(self.limits) {
            return self.fail(SurfaceReconcileFault::Credits { usage: projected, limits: self.limits });
        }
        let child = if requested == 0 { copy.advance(1, 0, work) } else { copy.reserve_next(requested) };
        let progress = match child {
            Ok(progress) => progress,
            Err(error) => { self.usage.include(0, usize::from(error.allocated_bytes != 0), error.allocated_bytes); return self.fail(SurfaceReconcileFault::AliasCapacity); }
        };
        if !self.usage.include(0, usize::from(progress.allocated_bytes != 0), progress.allocated_bytes) || !self.usage.fits(self.limits) {
            return self.fail(SurfaceReconcileFault::Credits { usage: self.usage, limits: self.limits });
        }
        SurfaceReconcileStep::Yield { nodes: 0, bytes: progress.allocated_bytes + progress.copied_bytes }
    }

    fn advance_record_assembly(&mut self) -> SurfaceReconcileStep {
        if !self.assembly_open {
            if self.assembly_credit.is_none() { return self.fail(SurfaceReconcileFault::Credits { usage: self.usage, limits: self.limits }); }
            let root = self.ids.first().copied().or_else(|| self.record_diff.as_ref().map(|record| record.id));
            match self.assembly.open_with_permit(&mut self.assembly_credit, &mut self.assembly_surface, self.assembly_generation, self.base_revision, root, 0, 1, SURFACE_RECONCILE_PAGE_BYTES) {
                Ok(progress) => { self.assembly_open = progress.progressed; return SurfaceReconcileStep::Yield { nodes: 0, bytes: progress.initialized_bytes + progress.moved_bytes }; }
                Err(error) if error.kind == ui_contract::UiDocumentAssemblyErrorKind::Contended => return SurfaceReconcileStep::Yield { nodes: 0, bytes: 0 },
                Err(_) => return self.fail(SurfaceReconcileFault::AliasCapacity),
            }
        }
        let id = self.record_diff.as_ref().unwrap().id;
        let requested = match self.assembly.next_allocation_bytes() {
            Ok(bytes) => bytes,
            Err(error) if error.kind == ui_contract::UiDocumentAssemblyErrorKind::Contended => return SurfaceReconcileStep::Yield { nodes: 0, bytes: 0 },
            Err(_) => return self.fail(SurfaceReconcileFault::AliasCapacity),
        };
        let mut projected = self.usage;
        if requested > SURFACE_RECONCILE_PAGE_BYTES || !projected.include(0, usize::from(requested != 0), requested) || !projected.fits(self.limits) {
            return self.fail(SurfaceReconcileFault::Credits { usage: projected, limits: self.limits });
        }
        let progress = match self.assembly.place_one(&mut self.record_diff.as_mut().unwrap().record.0, 1, SURFACE_RECONCILE_PAGE_BYTES) {
            Ok(progress) => progress,
            Err(error) if error.kind == ui_contract::UiDocumentAssemblyErrorKind::Contended => return SurfaceReconcileStep::Yield { nodes: 0, bytes: 0 },
            Err(error) => { self.usage.include(0, usize::from(error.allocated_bytes != 0), error.allocated_bytes); return self.fail(SurfaceReconcileFault::AliasCapacity); }
        };
        if !self.usage.include(0, usize::from(progress.allocated_bytes != 0), progress.allocated_bytes) || !self.usage.fits(self.limits) {
            return self.fail(SurfaceReconcileFault::Credits { usage: self.usage, limits: self.limits });
        }
        if self.record_diff.as_ref().unwrap().record.0.is_none() {
            if self.new_ordinals.try_insert(id, self.diff_index).is_err() { return self.fail(SurfaceReconcileFault::CounterOverflow); }
            self.record_diff = None;
            self.diff_index += 1;
        }
        SurfaceReconcileStep::Yield { nodes: 0, bytes: progress.allocated_bytes + progress.initialized_bytes + progress.compared_bytes + progress.moved_bytes }
    }

    fn advance_fresh_component(diff: &mut RecordDiffCursor, usage: &mut SurfaceReconcileUsage, limits: SurfaceReconcileLimits, fault_owner: &mut Option<SurfaceReconcileFault>) -> SurfaceReconcileStep {
        if diff.owned_copy.is_none() {
            let source = std::mem::replace(&mut diff.record.component, ui_contract::Component::Separator(ui_contract::SeparatorProps {}));
            diff.owned_copy = Some(RecordOwnedCopy::Component(ui_contract::UiComponentCopy::new(source)));
            return SurfaceReconcileStep::Yield { nodes: 0, bytes: size_of::<ui_contract::UiComponentCopy>() + size_of::<ui_contract::Component>() };
        }
        let copy = diff.owned_copy.as_mut().and_then(RecordOwnedCopy::component_mut).expect("component copy retained in its cursor");
        if copy.candidate().is_some() {
            if let Some(source) = copy.take_completed_source_with_grant(SURFACE_COMPONENT_COPY_WORK_BYTES) {
                diff.record.component = source;
                return SurfaceReconcileStep::Yield { nodes: 0, bytes: size_of::<ui_contract::Component>() };
            }
            let target = &mut diff.fresh.as_mut().expect("fresh component field retained").component;
            if target.is_some() { return SurfaceReconcileStep::Yield { nodes: 0, bytes: 0 }; }
            *target = copy.take_completed_candidate_with_grant(SURFACE_COMPONENT_COPY_WORK_BYTES);
            return SurfaceReconcileStep::Yield { nodes: 0, bytes: if target.is_some() { size_of::<ui_contract::Component>() } else { 0 } };
        }
        if copy.terminal_is_empty() {
            diff.owned_copy = None;
            diff.field += 1;
            return SurfaceReconcileStep::Yield { nodes: 0, bytes: size_of::<ui_contract::UiComponentCopy>() };
        }
        let requested = match copy.next_allocation_bytes() {
            Ok(bytes) => bytes,
            Err(_) => { *fault_owner = Some(SurfaceReconcileFault::CounterOverflow); return SurfaceReconcileStep::Fault(SurfaceReconcileFault::CounterOverflow); }
        };
        let mut projected = *usage;
        if requested > SURFACE_RECONCILE_PAGE_BYTES || !projected.include(0, usize::from(requested != 0), requested) || !projected.fits(limits) {
            let fault = SurfaceReconcileFault::Credits { usage: projected, limits };
            *fault_owner = Some(fault.clone());
            return SurfaceReconcileStep::Fault(fault);
        }
        let progress = match if requested == 0 { copy.advance(1, 0, SURFACE_COMPONENT_COPY_WORK_BYTES) } else { copy.reserve_next(requested) } {
            Ok(progress) => progress,
            Err(error) => {
                let counted = usage.include(0, usize::from(error.allocated_bytes != 0), error.allocated_bytes);
                let fault = if !counted { SurfaceReconcileFault::CounterOverflow } else if error.allocated_bytes != 0 { SurfaceReconcileFault::Credits { usage: *usage, limits } } else { SurfaceReconcileFault::AliasCapacity };
                *fault_owner = Some(fault.clone());
                return SurfaceReconcileStep::Fault(fault);
            }
        };
        if !usage.include(0, usize::from(progress.allocated_bytes != 0), progress.allocated_bytes) || !usage.fits(limits) {
            let fault = SurfaceReconcileFault::Credits { usage: *usage, limits };
            *fault_owner = Some(fault.clone());
            return SurfaceReconcileStep::Fault(fault);
        }
        SurfaceReconcileStep::Yield { nodes: 0, bytes: progress.allocated_bytes + progress.copied_bytes }
    }

    fn advance_fresh_bindings(diff: &mut RecordDiffCursor, usage: &mut SurfaceReconcileUsage, limits: SurfaceReconcileLimits, fault_owner: &mut Option<SurfaceReconcileFault>) -> SurfaceReconcileStep {
        let fresh = diff.fresh.as_mut().expect("fresh binding copy owns the field");
        if diff.owned_copy.is_none() {
            diff.owned_copy = Some(RecordOwnedCopy::Bindings(ui_contract::UiBindingsCopy::new(std::mem::take(&mut diff.record.bindings))));
            return SurfaceReconcileStep::Yield { nodes: 0, bytes: size_of::<ui_contract::UiBindingsCopy>() };
        }
        let copy = diff.owned_copy.as_mut().and_then(RecordOwnedCopy::bindings_mut).expect("binding copy retained above");
        if copy.is_complete() {
            if let Some(source) = copy.take_completed_source_with_grant(SURFACE_RECONCILE_PAGE_BYTES) {
                diff.record.bindings = source;
                return SurfaceReconcileStep::Yield { nodes: 0, bytes: size_of::<ui_contract::UiNodeBindings>() };
            }
            if copy.candidate().is_some() {
                if fresh.bindings.is_some() { return SurfaceReconcileStep::Yield { nodes: 0, bytes: 0 }; }
                fresh.bindings = copy.take_completed_candidate_with_grant(SURFACE_RECONCILE_PAGE_BYTES);
                return SurfaceReconcileStep::Yield { nodes: 0, bytes: if fresh.bindings.is_some() { size_of::<ui_contract::UiNodeBindings>() } else { 0 } };
            }
            diff.owned_copy = None;
            diff.field += 1;
            return SurfaceReconcileStep::Yield { nodes: 0, bytes: size_of::<ui_contract::UiBindingsCopy>() };
        }
        let requested = match copy.next_allocation_bytes() {
            Ok(bytes) => bytes,
            Err(_) => { *fault_owner = Some(SurfaceReconcileFault::CounterOverflow); return SurfaceReconcileStep::Fault(SurfaceReconcileFault::CounterOverflow); }
        };
        let mut projected = *usage;
        if requested > SURFACE_RECONCILE_PAGE_BYTES || !projected.include(0, usize::from(requested != 0), requested) || !projected.fits(limits) {
            let fault = SurfaceReconcileFault::Credits { usage: projected, limits };
            *fault_owner = Some(fault.clone());
            return SurfaceReconcileStep::Fault(fault);
        }
        let progress = match copy.advance(1, requested, if requested == 0 { SURFACE_RECONCILE_PAGE_BYTES } else { 0 }) {
            Ok(progress) => progress,
            Err(error) => {
                let counted = usage.include(0, usize::from(error.allocated_bytes != 0), error.allocated_bytes);
                let fault = if !counted { SurfaceReconcileFault::CounterOverflow } else if error.allocated_bytes != 0 { SurfaceReconcileFault::Credits { usage: *usage, limits } } else { SurfaceReconcileFault::AliasCapacity };
                *fault_owner = Some(fault.clone());
                return SurfaceReconcileStep::Fault(fault);
            }
        };
        if !usage.include(0, usize::from(progress.allocated_bytes != 0), progress.allocated_bytes) || !usage.fits(limits) {
            let fault = SurfaceReconcileFault::Credits { usage: *usage, limits };
            *fault_owner = Some(fault.clone());
            return SurfaceReconcileStep::Fault(fault);
        }
        SurfaceReconcileStep::Yield { nodes: 0, bytes: progress.allocated_bytes + progress.copied_bytes + progress.placed_bytes }
    }

    fn advance_pending_patch(&mut self) -> SurfaceReconcileStep {
        if self.ops.has_reserved_slot() {
            return match self.pending_op.place_into(&mut self.ops, SURFACE_RECONCILE_PAGE_BYTES) {
                Ok(bytes) => SurfaceReconcileStep::Yield { nodes: 0, bytes },
                Err(_) => {
                    self.fault = Some(SurfaceReconcileFault::CounterOverflow);
                    SurfaceReconcileStep::Fault(SurfaceReconcileFault::CounterOverflow)
                }
            };
        }
        let requested = match self.ops.next_allocation_bytes() {
            Ok(bytes) => bytes,
            Err(_) => {
                self.fault = Some(SurfaceReconcileFault::CounterOverflow);
                return SurfaceReconcileStep::Fault(SurfaceReconcileFault::CounterOverflow);
            }
        };
        let mut projected = self.usage;
        if requested > SURFACE_RECONCILE_PAGE_BYTES || !projected.include(0, 1, requested) || !projected.fits(self.limits) {
            let fault = SurfaceReconcileFault::Credits { usage: projected, limits: self.limits };
            self.fault = Some(fault.clone());
            return SurfaceReconcileStep::Fault(fault);
        }
        let (allocated, failed) = match self.ops.try_reserve_one(requested) {
            Ok(bytes) => (bytes, false),
            Err(error) => (error.allocated_bytes, true),
        };
        if !self.usage.include(0, usize::from(allocated != 0), allocated) || !self.usage.fits(self.limits) || failed {
            let fault = SurfaceReconcileFault::Credits { usage: self.usage, limits: self.limits };
            self.fault = Some(fault.clone());
            return SurfaceReconcileStep::Fault(fault);
        }
        SurfaceReconcileStep::Yield { nodes: 0, bytes: allocated }
    }

    fn retire_one(&mut self) -> bool {
        if let Some((_, node)) = self.held_node.take() {
            self.retire_tree.begin(crate::ComponentTree { root: node });
            self.semantic_census = None;
            return false;
        }
        if let Some(node) = self.pending_root.take() {
            self.retire_tree.begin(crate::ComponentTree { root: node });
            return false;
        }
        if !self.retire_tree.step() {
            return false;
        }
        if let Some(mut frame) = self.overflow_frame.take() {
            if let Some(child) = frame.children.next() {
                self.overflow_frame = Some(frame);
                self.held_node = Some((None, child));
                self.semantic_census = Some(SurfaceSemanticCensusCursor::default());
            }
            return false;
        }
        if !self.pending_op.terminal_is_empty() {
            let _ = self.pending_op.close_step(1, SURFACE_RECONCILE_PAGE_BYTES);
            return false;
        }
        if let Some(diff) = self.record_diff.as_mut() {
            if diff.record.0.is_none() { self.record_diff = None; return false; }
            if let Some(fresh) = diff.fresh.as_mut() {
                if retire_fresh_record_one(fresh, &mut self.retire_fresh_field, &mut diff.owned_copy) {
                    diff.fresh = None;
                    self.retire_fresh_field = 0;
                }
                return false;
            }
            if retire_record_one(&mut diff.record, &mut self.retire_record_field, &mut diff.owned_copy) {
                self.record_diff = None;
                self.retire_record_field = 0;
            }
            return false;
        }
        if let Some(frame) = self.traversal.last_mut() {
            if let Some(child) = frame.children.next() {
                self.held_node = Some((None, child));
                self.semantic_census = Some(SurfaceSemanticCensusCursor::default());
                return false;
            }
            self.traversal.pop();
            return false;
        }
        if !self.ops.terminal_is_empty() {
            let _ = self.ops.close_step(1, SURFACE_RECONCILE_PAGE_BYTES);
            return false;
        }
        if self.flat.pop().is_some() || self.postorder.pop().is_some() || self.ids.pop().is_some() || self.removal.pop().is_some() {
            return false;
        }
        if self.seen.pop().is_some() {
            return false;
        }
        if !self.assembly.terminal_is_empty() {
            self.assembly.close_step(1, SURFACE_COMPONENT_COPY_WORK_BYTES).expect("retained assembly close preserves exact fault");
            return false;
        }
        if self.new_ordinals.take_first().is_some() {
            return false;
        }
        if self.assembly_credit.is_some() {
            release_surface_reconcile_one(&mut self.assembly_credit).expect("unattached candidate permit remains exact");
            return false;
        }
        if self.new_key_index.take_first().is_some() {
            return false;
        }
        self.remove_next = None;
        self.fault = None;
        true
    }
}

//#region 🎟️RetainedAuthority

pub const SURFACE_RECONCILE_ADMISSION_SLOTS: usize = ui_contract::UI_RESIDENT_SLOTS;
pub const SURFACE_RECONCILE_PAGE_BYTES: usize = 32 * 1_024;
const SURFACE_COMPONENT_COPY_WORK_BYTES: usize = 4096;
const _: () = assert!(size_of::<ui_contract::Component>() <= SURFACE_COMPONENT_COPY_WORK_BYTES);
const _: () = assert!(size_of::<ui_contract::UiComponentCopy>() <= SURFACE_RECONCILE_PAGE_BYTES);
const _: () = assert!(2 * size_of::<ui_contract::Component>() + SURFACE_COMPONENT_COPY_WORK_BYTES <= SURFACE_RECONCILE_PAGE_BYTES);
const _: () = assert!(size_of::<ui_contract::UiPatchOp>() <= SURFACE_RECONCILE_PAGE_BYTES);
const _: () = assert!(size_of::<ui_contract::UiBindingsCopy>() <= SURFACE_RECONCILE_PAGE_BYTES);
pub const SURFACE_RECONCILE_SURFACE_BYTES: usize = ui_contract::UI_RESIDENT_SURFACE_BYTES;
pub const SURFACE_RECONCILE_AGGREGATE_BYTES: usize = ui_contract::UI_RESIDENT_AGGREGATE_BYTES;
pub const SURFACE_RECONCILE_AGGREGATE_ITEMS: usize = ui_contract::UI_RESIDENT_AGGREGATE_ITEMS;

fn reserve_surface_reconcile(limits: SurfaceReconcileLimits) -> Option<ui_contract::UiResidentPermit> {
    if limits.max_nodes > SurfaceReconcileLimits::default().max_nodes
        || limits.max_items > SurfaceReconcileLimits::default().max_items
        || limits.max_bytes > SurfaceReconcileLimits::default().max_bytes
        || limits.max_identifier_bytes > SurfaceReconcileLimits::default().max_identifier_bytes
    { return None; }
    if !register_surface_reconcile_backing(SURFACE_RECONCILE_PAGE_BYTES).ok()? { return None; }
    let mut permit = None;
    ui_contract::UiResidentPermit::try_reserve(ui_contract::UiResidentLimits { items: limits.max_items, bytes: limits.max_bytes }, &mut permit, SURFACE_RECONCILE_PAGE_BYTES).ok()?;
    permit
}

fn register_surface_reconcile_backing(admitted_bytes: usize) -> Result<bool, ui_contract::UiResidentFault> {
    ui_contract::UiResidentPermit::try_register_runtime_backing(size_of::<LazyLock<Mutex<SurfaceReconcileHandbackRegistry>>>() + SurfaceReconcileOutputs::static_backing_bytes(), admitted_bytes)
}

fn split_surface_reconcile(mut credit: ui_contract::UiResidentPermit) -> Result<(ui_contract::UiResidentPermit, ui_contract::UiResidentPermit), ui_contract::UiResidentPermit> {
    let mut output = None;
    if !matches!(credit.split_output_into(&mut output, SURFACE_RECONCILE_PAGE_BYTES), Ok(true)) { return Err(credit); }
    Ok((credit, output.expect("successful resident split installs the exact output owner")))
}

/// 📉️ Returns unused aggregate admission capacity once reconciliation has measured its retained owners.
fn shrink_surface_reconcile(mut credit: ui_contract::UiResidentPermit, usage: SurfaceReconcileUsage) -> Result<ui_contract::UiResidentPermit, ui_contract::UiResidentPermit> {
    match credit.try_shrink(ui_contract::UiResidentLimits { items: usage.items, bytes: usage.bytes }) {
        Ok(true) => Ok(credit),
        _ => Err(credit),
    }
}

fn release_surface_reconcile(mut credit: ui_contract::UiResidentPermit) {
    if let Err(error) = credit.close_step(1) {
        if !std::thread::panicking() { panic!("{}", error.reason()); }
    }
}

fn release_surface_reconcile_one(owner: &mut Option<ui_contract::UiResidentPermit>) -> Result<ui_contract::UiValueRetirementStep, &'static str> {
    let Some(credit) = owner.as_mut() else { return Ok(ui_contract::UiValueRetirementStep { complete: true, ..Default::default() }); };
    let step = credit.close_step(1).map_err(|error| error.reason())?;
    if step.complete { owner.take(); }
    Ok(ui_contract::UiValueRetirementStep { complete: step.complete, progressed: step.progressed, released_items: step.released_permits, released_bytes: 0 })
}

/// 🎫️ Pre-materialization aggregate reservation transferred into exactly one live job.
pub struct SurfaceReconcileReservation {
    generation: u64,
    limits: SurfaceReconcileLimits,
    credit: Option<ui_contract::UiResidentPermit>,
    handback: Option<SurfaceReconcileHandbackReservation>,
    output_handback: Option<SurfaceReconcileHandbackReservation>,
}

impl SurfaceReconcileReservation {
    pub fn try_new(generation: u64) -> Option<Self> {
        if generation == 0 {
            return None;
        }
        let limits = SurfaceReconcileLimits::default();
        let credit = reserve_surface_reconcile(limits)?;
        let Some(handback) = reserve_surface_reconcile_handback(generation) else {
            release_surface_reconcile(credit);
            return None;
        };
        let Some(output_handback) = reserve_surface_reconcile_handback(generation) else {
            release_surface_reconcile_handback(handback);
            release_surface_reconcile(credit);
            return None;
        };
        Some(Self { generation, limits, credit: Some(credit), handback: Some(handback), output_handback: Some(output_handback) })
    }

    pub fn generation(&self) -> u64 {
        self.generation
    }
}

impl Drop for SurfaceReconcileReservation {
    fn drop(&mut self) {
        if let Some(credit) = self.credit.take() {
            release_surface_reconcile(credit);
        }
        if let Some(handback) = self.handback.take() {
            release_surface_reconcile_handback(handback);
        }
        if let Some(handback) = self.output_handback.take() {
            release_surface_reconcile_handback(handback);
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SurfaceReconcileJobPhase {
    Drive,
    RetireCursor,
    RetirePrevious,
    SealCandidate,
    Ready,
    Fault,
    Closing,
}

struct SurfaceTreeRetireCursor {
    node: Option<crate::TreeNode>,
    frames: Box<[Option<ui_contract::BuiltChildrenIntoIter>]>,
    overflow: Option<ui_contract::BuiltChildrenIntoIter>,
    depth: usize,
}

impl Default for SurfaceTreeRetireCursor {
    fn default() -> Self {
        let mut frames = Vec::with_capacity(SURFACE_RECONCILE_TREE_RETIRE_DEPTH);
        frames.resize_with(SURFACE_RECONCILE_TREE_RETIRE_DEPTH, || None);
        Self { node: None, frames: frames.into_boxed_slice(), overflow: None, depth: 0 }
    }
}

impl SurfaceTreeRetireCursor {
    fn begin(&mut self, tree: crate::ComponentTree) {
        self.node = Some(tree.root);
    }

    fn step(&mut self) -> bool {
        if let Some(children) = self.overflow.as_mut() {
            if let Some(child) = children.next() {
                self.node = Some(child);
            } else {
                self.overflow = None;
            }
            return false;
        }
        if let Some(mut node) = self.node.take() {
            let children = take(&mut node.children).into_iter();
            if children.len() > 0 {
                if self.depth == self.frames.len() {
                    self.overflow = Some(children);
                    return false;
                }
                self.frames[self.depth] = Some(children);
                self.depth += 1;
            }
            return false;
        }
        let Some(depth) = self.depth.checked_sub(1) else { return true };
        let Some(children) = self.frames[depth].as_mut() else {
            self.depth = depth;
            return false;
        };
        if let Some(child) = children.next() {
            self.node = Some(child);
        } else {
            self.frames[depth] = None;
            self.depth = depth;
        }
        false
    }

    fn is_empty(&self) -> bool {
        self.node.is_none() && self.depth == 0
    }
}

pub const SURFACE_RECONCILE_HANDBACK_SLOTS: usize = SURFACE_RECONCILE_ADMISSION_SLOTS * 6;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SurfaceReconcileHandbackKey {
    slot: usize,
    epoch: u64,
    generation: u64,
}

#[derive(Debug)]
struct SurfaceReconcileHandbackReservation {
    key: SurfaceReconcileHandbackKey,
}

struct SurfaceReconcileHandbackSlot {
    epoch: u64,
    generation: u64,
    reserved: bool,
    queued: bool,
    state: Option<Box<SurfaceReconcileRetained>>,
}

impl Default for SurfaceReconcileHandbackSlot {
    fn default() -> Self {
        Self { epoch: 0, generation: 0, reserved: false, queued: false, state: None }
    }
}

struct SurfaceReconcileHandbackRegistry {
    slots: [SurfaceReconcileHandbackSlot; SURFACE_RECONCILE_HANDBACK_SLOTS],
    free: [usize; SURFACE_RECONCILE_HANDBACK_SLOTS],
    free_len: usize,
    retirement: [usize; SURFACE_RECONCILE_HANDBACK_SLOTS],
    retirement_head: usize,
    retirement_len: usize,
}

impl Default for SurfaceReconcileHandbackRegistry {
    fn default() -> Self {
        Self {
            slots: std::array::from_fn(|_| SurfaceReconcileHandbackSlot::default()),
            free: std::array::from_fn(|index| SURFACE_RECONCILE_HANDBACK_SLOTS - 1 - index),
            free_len: SURFACE_RECONCILE_HANDBACK_SLOTS,
            retirement: [usize::MAX; SURFACE_RECONCILE_HANDBACK_SLOTS],
            retirement_head: 0,
            retirement_len: 0,
        }
    }
}

static SURFACE_RECONCILE_HANDBACKS: LazyLock<Mutex<SurfaceReconcileHandbackRegistry>> = LazyLock::new(|| Mutex::new(SurfaceReconcileHandbackRegistry::default()));

fn reserve_surface_reconcile_handback(generation: u64) -> Option<SurfaceReconcileHandbackReservation> {
    if generation == 0 {
        return None;
    }
    let mut registry = SURFACE_RECONCILE_HANDBACKS.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    if registry.free_len == 0 {
        return None;
    }
    let slot = registry.free[registry.free_len - 1];
    let epoch = registry.slots[slot].epoch.checked_add(1)?;
    registry.free_len -= 1;
    registry.slots[slot] = SurfaceReconcileHandbackSlot { epoch, generation, reserved: true, queued: false, state: None };
    Some(SurfaceReconcileHandbackReservation { key: SurfaceReconcileHandbackKey { slot, epoch, generation } })
}

fn release_surface_reconcile_handback(reservation: SurfaceReconcileHandbackReservation) {
    let mut registry = SURFACE_RECONCILE_HANDBACKS.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    let should_free = {
        let Some(slot) = registry.slots.get_mut(reservation.key.slot) else { return };
        if !slot.reserved || slot.epoch != reservation.key.epoch || slot.generation != reservation.key.generation || slot.state.is_some() {
            return;
        }
        slot.reserved = false;
        slot.generation = 0;
        !slot.queued
    };
    if should_free {
        let index = registry.free_len;
        registry.free[index] = reservation.key.slot;
        registry.free_len += 1;
    }
}

fn release_surface_reconcile_handback_one(owner: &mut Option<SurfaceReconcileHandbackReservation>) -> Result<ui_contract::UiValueRetirementStep, &'static str> {
    let Some(reservation) = owner.as_ref() else { return Ok(ui_contract::UiValueRetirementStep { complete: true, ..Default::default() }); };
    let mut registry = match SURFACE_RECONCILE_HANDBACKS.try_lock() {
        Ok(registry) => registry,
        Err(std::sync::TryLockError::WouldBlock) => return Ok(Default::default()),
        Err(std::sync::TryLockError::Poisoned(_)) => return Err("surface handback registry is poisoned"),
    };
    let slot = registry.slots.get(reservation.key.slot).ok_or("surface handback slot is invalid")?;
    if !slot.reserved || slot.epoch != reservation.key.epoch || slot.generation != reservation.key.generation || slot.state.is_some() { return Err("surface handback is not the exact retained owner"); }
    let free = !slot.queued;
    if free && registry.free_len >= SURFACE_RECONCILE_HANDBACK_SLOTS { return Err("surface handback free list exhausted"); }
    let slot = &mut registry.slots[reservation.key.slot];
    slot.reserved = false; slot.generation = 0;
    if free { let index = registry.free_len; registry.free[index] = reservation.key.slot; registry.free_len += 1; }
    owner.take();
    Ok(ui_contract::UiValueRetirementStep { complete: true, progressed: true, released_items: 1, released_bytes: 0 })
}

fn rebind_surface_reconcile_handback(mut reservation: SurfaceReconcileHandbackReservation, generation: u64) -> Result<SurfaceReconcileHandbackReservation, SurfaceReconcileHandbackReservation> {
    let mut registry = SURFACE_RECONCILE_HANDBACKS.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    let Some(slot) = registry.slots.get_mut(reservation.key.slot) else { return Err(reservation) };
    if !slot.reserved || slot.epoch != reservation.key.epoch || slot.generation != reservation.key.generation || slot.state.is_some() || generation == 0 {
        return Err(reservation);
    }
    slot.generation = generation;
    reservation.key.generation = generation;
    Ok(reservation)
}

fn acquire_surface_reconcile_handback(owner: &mut Option<SurfaceReconcileHandbackReservation>, generation: u64) -> Option<SurfaceReconcileHandbackReservation> {
    let Some(reservation) = owner.take() else { return reserve_surface_reconcile_handback(generation) };
    match rebind_surface_reconcile_handback(reservation, generation) {
        Ok(reservation) => Some(reservation),
        Err(reservation) => {
            *owner = Some(reservation);
            None
        }
    }
}

fn acquire_reserved_surface_reconcile_handback(owner: &mut Option<SurfaceReconcileHandbackReservation>, reserved: &mut Option<SurfaceReconcileHandbackReservation>, generation: u64) -> Option<SurfaceReconcileHandbackReservation> {
    if owner.is_none() {
        return reserved.take();
    }
    acquire_surface_reconcile_handback(owner, generation)
}

struct SurfaceReconcileRetained {
    generation: u64,
    phase: SurfaceReconcileJobPhase,
    current: Option<SurfaceReconciler>,
    source: Option<crate::ComponentTree>,
    cursor: Option<SurfaceReconcileCursor>,
    candidate: Option<SurfaceReconciler>,
    patch: ui_contract::UiPendingPatch,
    retire_tree: SurfaceTreeRetireCursor,
    fault: Option<SurfaceReconcileFault>,
    usage: SurfaceReconcileUsage,
    credit: Option<ui_contract::UiResidentPermit>,
    handback: Option<SurfaceReconcileHandbackReservation>,
    output_handback: Option<SurfaceReconcileHandbackReservation>,
}

impl SurfaceReconcileRetained {
    fn refuse_publication(&mut self, reason: &'static str) -> Result<bool, &'static str> {
        self.fault = Some(SurfaceReconcileFault::PublicationAuthority(reason));
        self.phase = SurfaceReconcileJobPhase::Fault;
        Err(reason)
    }

    fn close_step(&mut self) -> bool {
        self.phase = SurfaceReconcileJobPhase::Closing;
        if self.fault.take().is_some() {
            return false;
        }
        if !self.patch.terminal_is_empty() {
            let _ = self.patch.close_step(1, SURFACE_RECONCILE_PAGE_BYTES);
            return false;
        }
        if let Some(candidate) = self.candidate.as_mut() {
            if !candidate.retire_one() {
                return false;
            }
            self.candidate = None;
            return false;
        }
        if let Some(cursor) = self.cursor.as_mut() {
            if !cursor.retire_one() {
                return false;
            }
            self.cursor = None;
            return false;
        }
        if let Some(current) = self.current.as_mut() {
            if !current.retire_one() {
                return false;
            }
            self.current = None;
            return false;
        }
        if let Some(tree) = self.source.take() {
            self.retire_tree.begin(tree);
            return false;
        }
        if !self.retire_tree.step() {
            return false;
        }
        if let Some(credit) = self.credit.take() {
            release_surface_reconcile(credit);
            return false;
        }
        if let Some(handback) = self.handback.take() {
            release_surface_reconcile_handback(handback);
            return false;
        }
        if let Some(handback) = self.output_handback.take() {
            release_surface_reconcile_handback(handback);
            return false;
        }
        true
    }

    fn terminal_is_empty(&self) -> bool {
        self.current.is_none()
            && self.source.is_none()
            && self.cursor.is_none()
            && self.candidate.is_none()
            && self.patch.terminal_is_empty()
            && self.retire_tree.is_empty()
            && self.fault.is_none()
            && self.credit.is_none()
            && self.handback.is_none()
            && self.output_handback.is_none()
    }
}

fn handback_surface_reconcile(mut state: Box<SurfaceReconcileRetained>) {
    if state.handback.is_none() {
        state.handback = state.current.as_mut().and_then(|owner| owner.handback.take()).or_else(|| state.candidate.as_mut().and_then(|owner| owner.handback.take())).or_else(|| state.output_handback.take());
    }
    let Some(reservation) = state.handback.take() else { return };
    let mut registry = SURFACE_RECONCILE_HANDBACKS.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    let enqueue = {
        let slot = &mut registry.slots[reservation.key.slot];
        let enqueue = !slot.queued;
        slot.state = Some(state);
        if enqueue {
            slot.queued = true;
        }
        enqueue
    };
    if enqueue {
        let tail = (registry.retirement_head + registry.retirement_len) % SURFACE_RECONCILE_HANDBACK_SLOTS;
        registry.retirement[tail] = reservation.key.slot;
        registry.retirement_len += 1;
    }
}

/// 🚦️ One admitted reconciliation opportunity result.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SurfaceReconcileJobStep {
    MoreWork,
    Ready,
    Fault,
}

/// 📄️ Why a retained renderer-document producer stopped before publication.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SurfaceDocumentFault {
    StaleGeneration { expected: u64, actual: u64 },
    StaleRevision { expected: ui_contract::UiRevision, actual: ui_contract::UiRevision },
    Cancelled,
    Build(ui_contract::UiDocumentBuildError),
}

/// ⏭️ One bounded renderer-document production opportunity.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SurfaceDocumentProducerStep {
    MoreWork,
    Ready,
    Fault,
}

/// 🧵️ Generation-qualified producer which admits one retained node page per opportunity.
pub struct SurfaceDocumentProducer {
    generation: u64,
    revision: ui_contract::UiRevision,
    lease: Option<ui_contract::UiDocumentLease>,
    ready: bool,
    fault: Option<SurfaceDocumentFault>,
}

impl SurfaceDocumentProducer {
    pub fn try_new(current: &SurfaceReconciler, generation: u64) -> Result<Self, ui_contract::UiDocumentLeaseError> {
        if generation == 0 { return Err(ui_contract::UiDocumentLeaseError::StaleHandle); }
        let mut lease = None;
        if !current.capture_document(&mut lease, SURFACE_RECONCILE_PAGE_BYTES)? { return Err(ui_contract::UiDocumentLeaseError::StaleHandle); }
        Ok(Self { generation, revision: current.revision, lease, ready: false, fault: None })
    }

    pub fn generation(&self) -> u64 { self.generation }
    pub fn fault(&self) -> Option<&SurfaceDocumentFault> { self.fault.as_ref() }

    pub fn drive_one(&mut self, current: &SurfaceReconciler, cx: &mut semio_framework_job::StepContext<'_>) -> SurfaceDocumentProducerStep {
        if self.fault.is_some() { return SurfaceDocumentProducerStep::Fault; }
        if cx.generation().0 != self.generation {
            self.fault = Some(SurfaceDocumentFault::StaleGeneration { expected: self.generation, actual: cx.generation().0 });
            return SurfaceDocumentProducerStep::Fault;
        }
        if cx.is_cancelled() { self.fault = Some(SurfaceDocumentFault::Cancelled); return SurfaceDocumentProducerStep::Fault; }
        if cx.should_yield() { return SurfaceDocumentProducerStep::MoreWork; }
        if !current.document.as_ref().zip(self.lease.as_ref()).is_some_and(|(current, captured)| current.same_root(captured)) {
            self.fault = Some(SurfaceDocumentFault::StaleRevision { expected: self.revision, actual: current.revision });
            return SurfaceDocumentProducerStep::Fault;
        }
        self.ready = true;
        cx.consume_fuel(1);
        SurfaceDocumentProducerStep::Ready
    }

    pub fn take_ready(mut self) -> Result<SurfaceDocumentOutcome, Self> {
        if !self.ready || self.fault.is_some() { return Err(self); }
        let Some(lease) = self.lease.take() else { return Err(self); };
        Ok(SurfaceDocumentOutcome { generation: self.generation, lease })
    }

    pub fn close_step(&mut self, items: usize, bytes: usize) -> Result<ui_contract::UiValueRetirementStep, &'static str> {
        let Some(lease) = self.lease.as_mut() else { return Ok(ui_contract::UiValueRetirementStep { complete: true, ..Default::default() }); };
        let progress = lease.close_read_step_with_grant(items, bytes)?;
        if progress.complete { self.lease = None; }
        Ok(progress)
    }
}
/// 📬️ Complete generation-qualified fixed document transferred to a renderer consumer.
pub struct SurfaceDocumentOutcome {
    generation: u64,
    lease: ui_contract::UiDocumentLease,
}

impl SurfaceDocumentOutcome {
    pub fn generation(&self) -> u64 {
        self.generation
    }

    pub fn header(&self) -> Result<ui_contract::UiDocumentLeaseHeader, ui_contract::UiDocumentLeaseError> {
        self.lease.header()
    }

    pub fn try_read(&self) -> Result<ui_contract::UiDocumentRead<'_>, ui_contract::UiDocumentLeaseError> {
        self.lease.try_read()
    }

    pub fn try_alias(&self) -> Result<Self, ui_contract::UiDocumentLeaseError> {
        let mut alias = None;
        if !self.lease.try_alias_into(&mut alias, SURFACE_RECONCILE_PAGE_BYTES)? { return Err(ui_contract::UiDocumentLeaseError::AliasCapacity); }
        Ok(Self { generation: self.generation, lease: alias.unwrap() })
    }

    pub fn into_lease(self) -> ui_contract::UiDocumentLease {
        self.lease
    }

    pub fn close_step(&mut self) -> bool {
        self.lease.close_read_step_with_grant(1, SURFACE_COMPONENT_COPY_WORK_BYTES).expect("document outcome retains exact retirement authority").complete
    }
}

/// 📨️ Generation-qualified patch owner carrying its share of the live reconciliation credit.
pub struct SurfaceReconcileReadyPatch {
    generation: u64,
    patch: ui_contract::UiPendingPatch,
    credit: Option<ui_contract::UiResidentPermit>,
    handback: Option<SurfaceReconcileHandbackReservation>,
}

fn pending_surface_patch(patch: Option<ui_contract::UiPatch>) -> ui_contract::UiPendingPatch {
    let mut owner = ui_contract::UiPendingPatch::default();
    *owner.source_mut().expect("new pending patch is writable") = patch;
    owner
}

fn close_surface_patch_owner(patch: &mut ui_contract::UiPendingPatch, credit: &mut Option<ui_contract::UiResidentPermit>, handback: &mut Option<SurfaceReconcileHandbackReservation>, items: usize, bytes: usize) -> Result<ui_contract::UiValueRetirementStep, &'static str> {
    if items == 0 || bytes == 0 { return Ok(Default::default()); }
    if !patch.terminal_is_empty() {
        let mut step = patch.close_step(1, bytes)?;
        step.complete = false;
        return Ok(step);
    }
    if credit.is_some() { let mut step = release_surface_reconcile_one(credit)?; step.complete = false; return Ok(step); }
    if handback.is_some() { let mut step = release_surface_reconcile_handback_one(handback)?; step.complete = false; return Ok(step); }
    Ok(ui_contract::UiValueRetirementStep { complete: true, ..Default::default() })
}

#[cfg(test)]
#[path = "../../🩹️patch/🧪️component.rs"]
mod patch_handoff_tests;


impl SurfaceReconcileReadyPatch {
    pub fn generation(&self) -> u64 {
        self.generation
    }

    pub fn surface(&self) -> Option<&ui_contract::SurfaceId> {
        self.patch.get().map(|patch| &patch.surface)
    }

    pub fn revision(&self) -> ui_contract::UiRevision {
        self.patch.get().map_or_else(ui_contract::UiRevision::default, |patch| patch.revision)
    }

    pub const fn required_publish_bytes() -> usize { size_of::<SurfaceReconcilePublishedPatch>() + size_of::<ui_contract::UiPatch>() + size_of::<ui_contract::SurfaceId>() }

    /// 📬️ Both destination slots and fixed initialization/moves are admitted before detaching the source.
    pub fn publish_into(&mut self, payload: &mut ui_contract::UiPendingPatch, published: &mut Option<SurfaceReconcilePublishedPatch>, admitted_bytes: usize) -> Result<usize, &'static str> {
        if admitted_bytes < Self::required_publish_bytes() || published.is_some() || !payload.terminal_is_empty() { return Ok(0); }
        if payload.source_mut()?.is_some() { return Ok(0); }
        let Some(source) = self.patch.get() else { return Ok(0); };
        let metadata = pending_surface_patch(Some(ui_contract::UiPatch { surface: source.surface.clone(), base_revision: source.base_revision, revision: source.revision, ops: ui_contract::UiPatchOps::default() }));
        let revision = source.revision;
        *payload.source_mut()? = self.patch.source_mut()?.take();
        *published = Some(SurfaceReconcilePublishedPatch { generation: self.generation, metadata, revision, credit: self.credit.take(), handback: self.handback.take() });
        Ok(Self::required_publish_bytes())
    }

    pub fn close_step(&mut self) -> bool {
        self.close_step_with_grant(1, 4096).expect("ready patch retirement remains exact").complete
    }

    pub fn close_step_with_grant(&mut self, items: usize, bytes: usize) -> Result<ui_contract::UiValueRetirementStep, &'static str> {
        close_surface_patch_owner(&mut self.patch, &mut self.credit, &mut self.handback, items, bytes)
    }
    pub fn terminal_is_empty(&self) -> bool { self.patch.terminal_is_empty() && self.credit.is_none() && self.handback.is_none() }
}

impl Drop for SurfaceReconcileReadyPatch {
    fn drop(&mut self) {
        if self.terminal_is_empty() {
            return;
        }
        let generation = self.generation;
        handback_surface_reconcile(Box::new(SurfaceReconcileRetained {
            output_handback: None,
            generation,
            phase: SurfaceReconcileJobPhase::Closing,
            current: None,
            source: None,
            cursor: None,
            candidate: None,
            patch: take(&mut self.patch),
            retire_tree: SurfaceTreeRetireCursor::default(),
            fault: None,
            usage: SurfaceReconcileUsage::default(),
            credit: self.credit.take(),
            handback: self.handback.take(),
        }));
    }
}

/// 📮️ Credit witness retained by the reactor until the published revision is acknowledged or closed.
pub struct SurfaceReconcilePublishedPatch {
    generation: u64,
    metadata: ui_contract::UiPendingPatch,
    revision: ui_contract::UiRevision,
    credit: Option<ui_contract::UiResidentPermit>,
    handback: Option<SurfaceReconcileHandbackReservation>,
}

impl SurfaceReconcilePublishedPatch {
    /// ♻️ Releases one exact published owner without handing its terminal into a global queue.
    pub fn close_step(&mut self) -> bool {
        self.close_step_with_grant(1, 4096).expect("published patch retirement remains exact").complete
    }

    pub fn close_step_with_grant(&mut self, items: usize, bytes: usize) -> Result<ui_contract::UiValueRetirementStep, &'static str> {
        close_surface_patch_owner(&mut self.metadata, &mut self.credit, &mut self.handback, items, bytes)
    }

    /// 🧾️ Verifies this published handle owns neither payload nor admission/handback authority.
    pub fn terminal_is_empty(&self) -> bool {
        self.metadata.terminal_is_empty() && self.credit.is_none() && self.handback.is_none()
    }

    pub fn generation(&self) -> u64 {
        self.generation
    }

    pub fn matches(&self, surface: &str, revision: u64) -> bool {
        self.surface().is_some_and(|exact| exact.0.as_str() == surface) && self.revision.0 == revision
    }

    pub fn surface(&self) -> Option<&ui_contract::SurfaceId> {
        self.metadata.get().map(|patch| &patch.surface)
    }

    pub fn revision(&self) -> ui_contract::UiRevision {
        self.revision
    }

    pub const fn required_acknowledge_bytes() -> usize { size_of::<Self>() + size_of::<SurfaceReconcilePublishedAck>() }

    /// ✅️ A mismatched receipt, occupied target, or insufficient grant leaves both slots untouched.
    pub fn acknowledge_into(source: &mut Option<Self>, target: &mut Option<SurfaceReconcilePublishedAck>, surface: &str, revision: u64, admitted_bytes: usize) -> Result<bool, &'static str> {
        if admitted_bytes < Self::required_acknowledge_bytes() || target.is_some() || !source.as_ref().is_some_and(|owner| owner.matches(surface, revision)) { return Ok(false); }
        *target = Some(SurfaceReconcilePublishedAck { owner: source.take().ok_or("published source lost after receipt validation")? });
        Ok(true)
    }
}

/// ✅️ Unforgeable ACK authority produced only by consuming the exact published patch owner.
pub struct SurfaceReconcilePublishedAck {
    owner: SurfaceReconcilePublishedPatch,
}

impl SurfaceReconcilePublishedAck {
    pub fn generation(&self) -> u64 {
        self.owner.generation()
    }

    pub fn surface(&self) -> Option<&ui_contract::SurfaceId> {
        self.owner.surface()
    }

    pub fn revision(&self) -> ui_contract::UiRevision {
        self.owner.revision()
    }

    pub fn close_step_with_grant(&mut self, items: usize, bytes: usize) -> Result<ui_contract::UiValueRetirementStep, &'static str> { self.owner.close_step_with_grant(items, bytes) }
    pub fn terminal_is_empty(&self) -> bool { self.owner.terminal_is_empty() }
}

impl Drop for SurfaceReconcilePublishedPatch {
    fn drop(&mut self) {
        if self.terminal_is_empty() {
            return;
        }
        handback_surface_reconcile(Box::new(SurfaceReconcileRetained {
            output_handback: None,
            generation: self.generation,
            phase: SurfaceReconcileJobPhase::Closing,
            current: None,
            source: None,
            cursor: None,
            candidate: None,
            patch: take(&mut self.metadata),
            retire_tree: SurfaceTreeRetireCursor::default(),
            fault: None,
            usage: SurfaceReconcileUsage::default(),
            credit: self.credit.take(),
            handback: self.handback.take(),
        }));
    }
}

/// 🧵️ Generation-keyed by-value reconciliation job advanced once per worker grant.
pub struct SurfaceReconcileJob {
    state: Option<Box<SurfaceReconcileRetained>>,
}

impl std::fmt::Debug for SurfaceReconcileJob {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("SurfaceReconcileJob")
            .field("phase", &self.state.as_ref().map(|state| state.phase))
            .field("usage", &self.state.as_ref().map(|state| state.usage))
            .field("credit", &self.state.as_ref().and_then(|state| state.credit.as_ref()))
            .field("fault", &self.fault())
            .finish()
    }
}

impl SurfaceReconcileJob {
    pub fn try_new(current: SurfaceReconciler, tree: crate::ComponentTree, generation: u64) -> Result<Self, SurfaceReconcileRejected> {
        Self::try_new_with_limits(current, tree, generation, SurfaceReconcileLimits::default())
    }

    pub fn try_new_with_limits(mut current: SurfaceReconciler, tree: crate::ComponentTree, generation: u64, limits: SurfaceReconcileLimits) -> Result<Self, SurfaceReconcileRejected> {
        let surface_bytes = current.surface.0.len();
        let handback = acquire_surface_reconcile_handback(&mut current.handback, generation);
        let credit = if surface_bytes <= limits.max_identifier_bytes { reserve_surface_reconcile(limits) } else { None };
        let output_handback = reserve_surface_reconcile_handback(generation);
        let (credit, handback, output_handback) = match (credit, handback, output_handback) {
            (Some(credit), Some(handback), Some(output_handback)) => (credit, handback, output_handback),
            (credit, handback, output_handback) => {
                return Err(SurfaceReconcileRejected {
                    state: Some(Box::new(SurfaceReconcileRetained {
                        output_handback,
                        generation,
                        phase: SurfaceReconcileJobPhase::Fault,
                        current: Some(current),
                        source: Some(tree),
                        cursor: None,
                        candidate: None,
                        patch: ui_contract::UiPendingPatch::default(),
                        retire_tree: SurfaceTreeRetireCursor::default(),
                        fault: Some(SurfaceReconcileFault::Credits { usage: SurfaceReconcileUsage { nodes: 0, items: 1, bytes: surface_bytes }, limits }),
                        usage: SurfaceReconcileUsage::default(),
                        credit,
                        handback,
                    })),
                })
            }
        };
        let cursor = SurfaceReconcileCursor::new_admitted(tree, &current, limits, generation, Some(credit));
        Ok(Self {
            state: Some(Box::new(SurfaceReconcileRetained {
                output_handback: Some(output_handback),
                generation,
                phase: SurfaceReconcileJobPhase::Drive,
                current: Some(current),
                source: None,
                cursor: Some(cursor),
                candidate: None,
                patch: ui_contract::UiPendingPatch::default(),
                retire_tree: SurfaceTreeRetireCursor::default(),
                fault: None,
                usage: SurfaceReconcileUsage::default(),
                credit: None,
                handback: Some(handback),
            })),
        })
    }

    pub fn try_new_reserved(mut current: SurfaceReconciler, tree: crate::ComponentTree, mut reservation: SurfaceReconcileReservation) -> Result<Self, SurfaceReconcileRejected> {
        let generation = reservation.generation;
        let limits = reservation.limits;
        let surface_bytes = current.surface.0.len();
        let handback = acquire_reserved_surface_reconcile_handback(&mut current.handback, &mut reservation.handback, generation);
        if surface_bytes > limits.max_identifier_bytes {
            return Err(SurfaceReconcileRejected {
                state: Some(Box::new(SurfaceReconcileRetained {
                    output_handback: reservation.output_handback.take(),
                    generation,
                    phase: SurfaceReconcileJobPhase::Fault,
                    current: Some(current),
                    source: Some(tree),
                    cursor: None,
                    candidate: None,
                    patch: ui_contract::UiPendingPatch::default(),
                    retire_tree: SurfaceTreeRetireCursor::default(),
                    fault: Some(SurfaceReconcileFault::IdentifierBytes { actual: surface_bytes, max: limits.max_identifier_bytes }),
                    usage: SurfaceReconcileUsage::default(),
                    credit: reservation.credit.take(),
                    handback,
                })),
            });
        }
        let Some(handback) = handback else {
            return Err(SurfaceReconcileRejected {
                state: Some(Box::new(SurfaceReconcileRetained {
                    output_handback: reservation.output_handback.take(),
                    generation,
                    phase: SurfaceReconcileJobPhase::Fault,
                    current: Some(current),
                    source: Some(tree),
                    cursor: None,
                    candidate: None,
                    patch: ui_contract::UiPendingPatch::default(),
                    retire_tree: SurfaceTreeRetireCursor::default(),
                    fault: Some(SurfaceReconcileFault::Credits { usage: SurfaceReconcileUsage { nodes: 0, items: 1, bytes: surface_bytes }, limits }),
                    usage: SurfaceReconcileUsage::default(),
                    credit: reservation.credit.take(),
                    handback: None,
                })),
            });
        };
        let cursor = SurfaceReconcileCursor::new_admitted(tree, &current, limits, generation, reservation.credit.take());
        Ok(Self {
            state: Some(Box::new(SurfaceReconcileRetained {
                output_handback: reservation.output_handback.take(),
                generation,
                phase: SurfaceReconcileJobPhase::Drive,
                current: Some(current),
                source: None,
                cursor: Some(cursor),
                candidate: None,
                patch: ui_contract::UiPendingPatch::default(),
                retire_tree: SurfaceTreeRetireCursor::default(),
                fault: None,
                usage: SurfaceReconcileUsage::default(),
                credit: reservation.credit.take(),
                handback: Some(handback),
            })),
        })
    }

    pub fn generation(&self) -> u64 {
        self.state.as_ref().map_or(0, |state| state.generation)
    }

    pub fn handback_key(&self) -> Option<SurfaceReconcileHandbackKey> {
        self.state.as_ref()?.handback.as_ref().map(|handback| handback.key)
    }

    pub fn base_revision(&self) -> ui_contract::UiRevision {
        self.state.as_ref().and_then(|state| state.current.as_ref()).map_or(ui_contract::UiRevision::default(), SurfaceReconciler::revision)
    }

    #[cfg(test)]
    pub(crate) fn transaction_usage(&self) -> SurfaceReconcileUsage {
        self.state.as_ref().map_or_else(SurfaceReconcileUsage::default, |state| state.cursor.as_ref().map_or(state.usage, |cursor| cursor.usage))
    }

    pub fn drive_one(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> SurfaceReconcileJobStep {
        let Some(state) = self.state.as_mut() else { return SurfaceReconcileJobStep::Fault };
        if state.phase == SurfaceReconcileJobPhase::Fault || state.phase == SurfaceReconcileJobPhase::Closing {
            return SurfaceReconcileJobStep::Fault;
        }
        if cx.generation().0 != state.generation {
            state.fault = Some(SurfaceReconcileFault::StaleGeneration { expected: state.generation, actual: cx.generation().0 });
            state.phase = SurfaceReconcileJobPhase::Fault;
            return SurfaceReconcileJobStep::Fault;
        }
        if cx.is_cancelled() {
            state.fault = Some(SurfaceReconcileFault::Cancelled);
            state.phase = SurfaceReconcileJobPhase::Fault;
            return SurfaceReconcileJobStep::Fault;
        }
        if cx.should_yield() {
            return SurfaceReconcileJobStep::MoreWork;
        }
        if state.phase == SurfaceReconcileJobPhase::Ready {
            return SurfaceReconcileJobStep::Ready;
        }
        let outcome = match state.phase {
            SurfaceReconcileJobPhase::Drive => {
                let Some(current) = state.current.as_ref() else {
                    state.phase = SurfaceReconcileJobPhase::Fault;
                    return SurfaceReconcileJobStep::Fault;
                };
                let Some(cursor) = state.cursor.as_mut() else {
                    state.phase = SurfaceReconcileJobPhase::Fault;
                    return SurfaceReconcileJobStep::Fault;
                };
                match cursor.step(current) {
                    SurfaceReconcileStep::Yield { .. } => SurfaceReconcileJobStep::MoreWork,
                    SurfaceReconcileStep::Complete { reconciler, patch } => {
                        state.usage = cursor.usage;
                        state.candidate = Some(reconciler);
                        *state.patch.source_mut().expect("ready job retains writable patch owner") = patch;
                        state.phase = SurfaceReconcileJobPhase::RetireCursor;
                        SurfaceReconcileJobStep::MoreWork
                    }
                    SurfaceReconcileStep::Fault(fault) => {
                        state.fault = Some(fault);
                        state.phase = SurfaceReconcileJobPhase::Fault;
                        SurfaceReconcileJobStep::Fault
                    }
                }
            }
            SurfaceReconcileJobPhase::RetireCursor => {
                if state.cursor.as_mut().is_some_and(|cursor| cursor.retire_one()) {
                    state.cursor = None;
                    state.phase = SurfaceReconcileJobPhase::RetirePrevious;
                }
                SurfaceReconcileJobStep::MoreWork
            }
            SurfaceReconcileJobPhase::RetirePrevious => {
                if state.current.as_mut().is_some_and(|current| current.retire_one()) {
                    state.current = None;
                    state.phase = SurfaceReconcileJobPhase::SealCandidate;
                    SurfaceReconcileJobStep::MoreWork
                } else {
                    SurfaceReconcileJobStep::MoreWork
                }
            }
            SurfaceReconcileJobPhase::SealCandidate => {
                let candidate = state.candidate.as_mut().expect("completed cursor retains candidate assembly");
                match candidate.seal_step(state.usage, &mut state.credit, state.patch.get().is_some()) {
                    Ok(true) => { state.phase = SurfaceReconcileJobPhase::Ready; SurfaceReconcileJobStep::Ready }
                    Ok(false) => SurfaceReconcileJobStep::MoreWork,
                    Err(error) if error.kind == ui_contract::UiDocumentAssemblyErrorKind::Contended => SurfaceReconcileJobStep::MoreWork,
                    Err(_) => { state.fault = Some(SurfaceReconcileFault::AliasCapacity); state.phase = SurfaceReconcileJobPhase::Fault; SurfaceReconcileJobStep::Fault }
                }
            }
            SurfaceReconcileJobPhase::Ready => SurfaceReconcileJobStep::Ready,
            SurfaceReconcileJobPhase::Fault | SurfaceReconcileJobPhase::Closing => SurfaceReconcileJobStep::Fault,
        };
        cx.consume_fuel(1);
        if cx.is_cancelled() {
            state.fault = Some(SurfaceReconcileFault::Cancelled);
            state.phase = SurfaceReconcileJobPhase::Fault;
            return SurfaceReconcileJobStep::Fault;
        }
        outcome
    }

    pub fn fault(&self) -> Option<&SurfaceReconcileFault> {
        self.state.as_ref().and_then(|state| state.fault.as_ref())
    }

    pub fn is_ready(&self) -> bool { self.state.as_ref().is_some_and(|state| state.phase == SurfaceReconcileJobPhase::Ready) }

    pub const fn required_ready_transfer_bytes() -> usize {
        2 * size_of::<SurfaceReconciler>() + size_of::<SurfaceReconcileReadyPatch>() + 3 * size_of::<ui_contract::UiPendingPatch>() + 2 * size_of::<ui_contract::UiResidentPermit>() + 4 * size_of::<SurfaceReconcileHandbackReservation>()
    }

    /// 📬️ Preadmitted receivers take the exact roots while the original job shell remains retained.
    pub fn take_ready_into(&mut self, current: &mut Option<SurfaceReconciler>, ready: &mut Option<SurfaceReconcileReadyPatch>, admitted_bytes: usize) -> Result<bool, &'static str> {
        if admitted_bytes < Self::required_ready_transfer_bytes() || current.is_some() || ready.is_some() { return Ok(false); }
        let Some(state) = self.state.as_mut() else { return Ok(false); };
        if state.phase != SurfaceReconcileJobPhase::Ready { return Ok(false); }
        let Some(candidate) = state.candidate.as_ref() else { return state.refuse_publication("ready job has no canonical candidate"); };
        if candidate.document.is_none() || candidate.handback.is_some() || state.handback.is_none() { return state.refuse_publication("ready job canonical owner authority is incomplete"); }
        if state.current.is_some() || state.cursor.is_some() || state.source.is_some() || !state.retire_tree.is_empty() { return state.refuse_publication("ready job still owns unfinished source retirement"); }
        let has_patch = state.patch.get().is_some();
        if has_patch && (state.credit.is_none() || state.output_handback.is_none()) { return state.refuse_publication("ready patch is missing its paired publication authority"); }
        if has_patch {
            *ready = Some(SurfaceReconcileReadyPatch { generation: state.generation, patch: ui_contract::UiPendingPatch::default(), credit: None, handback: None });
            let target = ready.as_mut().expect("preadmitted structural output receiver");
            std::mem::swap(&mut target.patch, &mut state.patch);
            target.credit = state.credit.take();
            target.handback = state.output_handback.take();
        }
        state.candidate.as_mut().expect("preflight canonical candidate").handback = state.handback.take();
        *current = state.candidate.take();
        state.phase = SurfaceReconcileJobPhase::Closing;
        Ok(true)
    }

    #[cfg(test)]
    fn take_ready(mut self) -> Result<(SurfaceReconciler, Option<SurfaceReconcileReadyPatch>), Self> {
        let mut current = None;
        let mut ready = None;
        if self.take_ready_into(&mut current, &mut ready, Self::required_ready_transfer_bytes()) != Ok(true) { return Err(self); }
        let mut terminal = self.into_terminal();
        while !terminal.close_step() {}
        Ok((current.expect("granted exact root transfer"), ready))
    }

    pub fn into_terminal(mut self) -> SurfaceReconcileTerminal {
        SurfaceReconcileTerminal { state: self.state.take() }
    }
}

impl Drop for SurfaceReconcileJob {
    fn drop(&mut self) {
        if let Some(state) = self.state.take() {
            handback_surface_reconcile(state);
        }
    }
}

/// 🔄️ Exact pre-admission owner returned without snapshot/tree cloning.
pub struct SurfaceReconcileRejected {
    state: Option<Box<SurfaceReconcileRetained>>,
}

impl std::fmt::Debug for SurfaceReconcileRejected {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("SurfaceReconcileRejected").field("generation", &self.generation()).finish()
    }
}

impl SurfaceReconcileRejected {
    pub fn generation(&self) -> u64 {
        self.state.as_ref().map_or(0, |state| state.generation)
    }

    pub fn retry(mut self, limits: SurfaceReconcileLimits) -> Result<SurfaceReconcileJob, Self> {
        let Some(mut state) = self.state.take() else { return Err(self) };
        let (Some(mut current), Some(tree)) = (state.current.take(), state.source.take()) else {
            self.state = Some(state);
            return Err(self);
        };
        if let Some(credit) = state.credit.take() { release_surface_reconcile(credit); }
        if let Some(handback) = state.output_handback.take() { release_surface_reconcile_handback(handback); }
        current.handback = state.handback.take();
        match SurfaceReconcileJob::try_new_with_limits(current, tree, state.generation, limits) {
            Ok(job) => Ok(job),
            Err(mut rejected) => {
                if let Some(next) = rejected.state.as_mut() {
                    next.fault = state.fault.take();
                }
                Err(rejected)
            }
        }
    }

    pub fn take_sources(&mut self) -> Option<(SurfaceReconciler, crate::ComponentTree)> {
        let state = self.state.as_mut()?;
        let mut current = state.current.take()?;
        let Some(tree) = state.source.take() else {
            state.current = Some(current);
            return None;
        };
        state.fault = None;
        if let Some(credit) = state.credit.take() { release_surface_reconcile(credit); }
        current.handback = state.handback.take();
        Some((current, tree))
    }

    pub fn handback_key(&self) -> Option<SurfaceReconcileHandbackKey> {
        self.state.as_ref()?.handback.as_ref().map(|handback| handback.key)
    }

    pub fn close_step(&mut self) -> bool {
        self.state.as_mut().is_none_or(|state| state.close_step())
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.state.as_ref().is_none_or(|state| state.terminal_is_empty())
    }

    pub fn into_terminal(mut self) -> SurfaceReconcileTerminal {
        SurfaceReconcileTerminal { state: self.state.take() }
    }
}

impl Drop for SurfaceReconcileRejected {
    fn drop(&mut self) {
        if let Some(state) = self.state.take() {
            handback_surface_reconcile(state);
        }
    }
}

/// 🧹️ Public fault/cancel/close authority; each close grant retires one retained owner.
pub struct SurfaceReconcileTerminal {
    state: Option<Box<SurfaceReconcileRetained>>,
}

impl SurfaceReconcileTerminal {
    pub fn try_from_reserved_sources(mut current: SurfaceReconciler, tree: crate::ComponentTree, mut reservation: SurfaceReconcileReservation) -> Result<Self, (SurfaceReconciler, crate::ComponentTree, SurfaceReconcileReservation)> {
        let generation = reservation.generation;
        let Some(handback) = acquire_reserved_surface_reconcile_handback(&mut current.handback, &mut reservation.handback, generation) else {
            return Err((current, tree, reservation));
        };
        Ok(Self {
            state: Some(Box::new(SurfaceReconcileRetained {
                output_handback: reservation.output_handback.take(),
                generation,
                phase: SurfaceReconcileJobPhase::Closing,
                current: Some(current),
                source: Some(tree),
                cursor: None,
                candidate: None,
                patch: ui_contract::UiPendingPatch::default(),
                retire_tree: SurfaceTreeRetireCursor::default(),
                fault: None,
                usage: SurfaceReconcileUsage::default(),
                credit: reservation.credit.take(),
                handback: Some(handback),
            })),
        })
    }

    pub fn try_from_reconciler(mut reconciler: SurfaceReconciler, generation: u64) -> Result<Self, SurfaceReconciler> {
        let Some(handback) = acquire_surface_reconcile_handback(&mut reconciler.handback, generation) else {
            return Err(reconciler);
        };
        let credit = None;
        Ok(Self {
            state: Some(Box::new(SurfaceReconcileRetained {
                output_handback: None,
                generation,
                phase: SurfaceReconcileJobPhase::Closing,
                current: Some(reconciler),
                source: None,
                cursor: None,
                candidate: None,
                patch: ui_contract::UiPendingPatch::default(),
                retire_tree: SurfaceTreeRetireCursor::default(),
                fault: None,
                usage: SurfaceReconcileUsage::default(),
                credit,
                handback: Some(handback),
            })),
        })
    }

    pub fn handback_key(&self) -> Option<SurfaceReconcileHandbackKey> {
        self.state.as_ref()?.handback.as_ref().map(|handback| handback.key)
    }

    pub fn generation(&self) -> u64 {
        self.state.as_ref().map_or(0, |state| state.generation)
    }

    pub fn fault(&self) -> Option<&SurfaceReconcileFault> {
        self.state.as_ref().and_then(|state| state.fault.as_ref())
    }

    pub fn resume(mut self, generation: u64) -> Result<SurfaceReconcileJob, Self> {
        let Some(state) = self.state.as_mut() else { return Err(self) };
        if state.generation != generation || state.cursor.is_none() || state.current.is_none() {
            return Err(self);
        }
        state.fault = None;
        state.cursor.as_mut().map(SurfaceReconcileCursor::clear_fault);
        state.phase = SurfaceReconcileJobPhase::Drive;
        Ok(SurfaceReconcileJob { state: self.state.take() })
    }

    pub fn close_step(&mut self) -> bool {
        self.state.as_mut().is_none_or(|state| state.close_step())
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.state.as_ref().is_none_or(|state| state.terminal_is_empty())
    }
}

impl Drop for SurfaceReconcileTerminal {
    fn drop(&mut self) {
        if let Some(state) = self.state.take() {
            handback_surface_reconcile(state);
        }
    }
}

pub fn take_surface_reconcile_terminal(key: SurfaceReconcileHandbackKey) -> Result<Option<SurfaceReconcileTerminal>, &'static str> {
    let mut registry = match SURFACE_RECONCILE_HANDBACKS.try_lock() {
        Ok(registry) => registry,
        Err(std::sync::TryLockError::WouldBlock) => return Ok(None),
        Err(std::sync::TryLockError::Poisoned(_)) => return Err("surface handback registry is poisoned"),
    };
    let Some(slot) = registry.slots.get_mut(key.slot) else { return Ok(None); };
    if !slot.reserved || slot.epoch != key.epoch || slot.generation != key.generation { return Ok(None); }
    if slot.state.as_ref().is_some_and(|state| state.handback.is_some()) { return Err("queued surface owner retains an external registry reservation"); }
    let Some(mut state) = slot.state.take() else { return Ok(None); };
    state.handback = Some(SurfaceReconcileHandbackReservation { key });
    Ok(Some(SurfaceReconcileTerminal { state: Some(state) }))
}

pub fn close_surface_reconcile_handback_one() -> Result<bool, &'static str> {
    static RESIDENT_TURN: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
    if RESIDENT_TURN.fetch_xor(true, std::sync::atomic::Ordering::Relaxed) && ui_contract::UiResidentPermit::has_pending_returns() {
        ui_contract::UiResidentPermit::drain_one().map_err(|error| error.reason())?;
        return Ok(false);
    }
    let mut registry = match SURFACE_RECONCILE_HANDBACKS.try_lock() {
        Ok(registry) => registry,
        Err(std::sync::TryLockError::WouldBlock) => return Ok(false),
        Err(std::sync::TryLockError::Poisoned(_)) => return Err("surface handback registry is poisoned"),
    };
    if registry.retirement_len == 0 { return Ok(!ui_contract::UiResidentPermit::has_pending_returns()); }
    let head = registry.retirement_head;
    let index = registry.retirement[head];
    let slot = registry.slots.get_mut(index).ok_or("surface retirement queue contains an invalid slot")?;
    if !slot.queued { return Err("surface retirement queue lost its exact owner"); }
    let complete = if let Some(state) = slot.state.as_mut() {
        if state.handback.is_some() || state.current.as_ref().is_some_and(|owner| owner.handback.is_some()) || state.candidate.as_ref().is_some_and(|owner| owner.handback.is_some()) {
            return Err("queued surface owner retains an external registry reservation");
        }
        if state.fault.is_none() && !state.patch.terminal_is_empty() {
            state.phase = SurfaceReconcileJobPhase::Closing;
            state.patch.close_step(1, SURFACE_RECONCILE_PAGE_BYTES)?;
            false
        } else { state.close_step() && state.terminal_is_empty() }
    } else { true };
    if complete && (slot.state.is_some() || !slot.reserved) && registry.free_len >= SURFACE_RECONCILE_HANDBACK_SLOTS { return Err("surface handback free list exhausted"); }
    registry.retirement[head] = usize::MAX;
    registry.retirement_head = (head + 1) % SURFACE_RECONCILE_HANDBACK_SLOTS;
    registry.retirement_len -= 1;
    if complete {
        let slot = &mut registry.slots[index];
        slot.queued = false;
        if slot.state.is_some() {
            slot.state = None;
            slot.reserved = false;
            slot.generation = 0;
        }
        if !slot.reserved {
            let free = registry.free_len;
            registry.free[free] = index;
            registry.free_len += 1;
        }
    } else {
        let tail = (registry.retirement_head + registry.retirement_len) % SURFACE_RECONCILE_HANDBACK_SLOTS;
        registry.retirement[tail] = index;
        registry.retirement_len += 1;
    }
    Ok(registry.retirement_len == 0 && !ui_contract::UiResidentPermit::has_pending_returns())
}

#[cfg(test)]
#[path = "../../🚪️handback/🧪️component.rs"]
mod handback_entry_tests;

//#endregion 🎟️RetainedAuthority

fn estimate_record_bytes(record: &ui_contract::UiNodeRecord) -> usize {
    record.children.len().checked_mul(size_of::<ui_contract::UiNodeId>()).and_then(|bytes| bytes.checked_add(record.key.len())).and_then(|bytes| bytes.checked_add(size_of::<ui_contract::UiNodeRecord>())).unwrap_or(usize::MAX)
}

fn diff_record_field(old: &ui_contract::UiNodeRecord, new: &ui_contract::UiNodeRecord, field: u8) -> Result<Option<ui_contract::UiPatchOp>, SurfaceReconcileFault> {
    let id = new.id;
    Ok(match field {
        0 if old.component != new.component => Some(ui_contract::UiPatchOp::SetComponent { id, component: new.component.credited_clone().ok_or(SurfaceReconcileFault::AliasCapacity)? }),
        1 if old.layout != new.layout => Some(ui_contract::UiPatchOp::SetLayout { id, layout: new.layout.clone() }),
        2 if old.activity != new.activity || old.disabled != new.disabled => Some(ui_contract::UiPatchOp::SetActivity { id, activity: new.activity, disabled: new.disabled }),
        3 if old.children != new.children => Some(ui_contract::UiPatchOp::SetChildren { id, children: new.children.clone() }),
        4 if old.style != new.style => Some(ui_contract::UiPatchOp::SetStyle { id, style: new.style }),
        5 if old.accessibility != new.accessibility => Some(ui_contract::UiPatchOp::SetAccessibility { id, accessibility: new.accessibility.clone() }),
        6 if old.bindings != new.bindings => Some(ui_contract::UiPatchOp::SetBindings { id, bindings: ui_contract::credited_bindings(&new.bindings).ok_or(SurfaceReconcileFault::AliasCapacity)? }),
        7 if old.menu != new.menu => Some(ui_contract::UiPatchOp::SetMenu {
            id,
            menu: match new.menu.as_ref() {
                Some(menu) => Some(menu.credited_clone().ok_or(SurfaceReconcileFault::AliasCapacity)?),
                None => None,
            },
        }),
        _ => None,
    })
}

fn retire_record_one(record: &mut ui_contract::UiNodeRecord, field: &mut u8, owned_copy: &mut Option<RecordOwnedCopy>) -> bool {
    match *field {
        0 => record.key = ui_contract::UiText::default(),
        1 => {
            if let Some(copy) = owned_copy.as_mut() {
                if !copy.close_step() { return false; }
                *owned_copy = None;
            }
            if !matches!(record.component, ui_contract::Component::Separator(_)) {
                let source = std::mem::replace(&mut record.component, ui_contract::Component::Separator(ui_contract::SeparatorProps {}));
                *owned_copy = Some(RecordOwnedCopy::Component(ui_contract::UiComponentCopy::new(source)));
                return false;
            }
        }
        2 => record.layout = ui_contract::LayoutSpec::default(),
        3 => {
            if record.children.pop().is_some() { return false; }
            if !record.children.terminal_is_empty() {
                let _ = record.children.release_empty_page();
                return false;
            }
        }
        4 => record.accessibility = ui_contract::AccessibilitySpec::default(),
        5 => {
            if let Some(copy) = owned_copy.as_mut() {
                if !copy.close_step() { return false; }
                *owned_copy = None;
            }
            if !record.bindings.terminal_is_empty() {
                *owned_copy = Some(RecordOwnedCopy::Bindings(ui_contract::UiBindingsCopy::new(std::mem::take(&mut record.bindings))));
                return false;
            }
        }
        6 => record.menu = None,
        7 => return true,
        _ => return true,
    }
    *field += 1;
    false
}

fn retire_fresh_record_one(record: &mut FreshRecordClone, field: &mut u8, owned_copy: &mut Option<RecordOwnedCopy>) -> bool {
    match *field {
        0 => record.key = None,
        1 => {
            if let Some(copy) = owned_copy.as_mut() {
                if !copy.close_step() { return false; }
                *owned_copy = None;
            }
            if let Some(component) = record.component.take() {
                *owned_copy = Some(RecordOwnedCopy::Component(ui_contract::UiComponentCopy::new(component)));
                return false;
            }
        }
        2 => record.layout = None,
        3 => record.children = None,
        4 => record.accessibility = None,
        5 => {
            if let Some(copy) = owned_copy.as_mut() {
                if !copy.close_step() { return false; }
                *owned_copy = None;
            }
            if let Some(bindings) = record.bindings.take() {
                *owned_copy = Some(RecordOwnedCopy::Bindings(ui_contract::UiBindingsCopy::new(bindings)));
                return false;
            }
        }
        6 => record.menu = None,
        7 => return true,
        _ => return true,
    }
    *field += 1;
    false
}

//#endregion ⏭️ResumableReconcile

//#region 🔖️Diff

#[cfg(test)]
struct SurfaceReconcileOracle {
    surface: ui_contract::SurfaceId,
    revision: ui_contract::UiRevision,
    allocator: ui_contract::UiNodeIdAllocator,
    root: Option<ui_contract::UiNodeId>,
    retained: SurfaceLinearMap<ui_contract::UiNodeId, ui_contract::UiNodeRecord, SURFACE_RECONCILE_FIXED_NODES>,
    key_index: SurfaceLinearMap<NodeIdentity, ui_contract::UiNodeId, SURFACE_RECONCILE_FIXED_NODES>,
}

#[cfg(test)]
impl SurfaceReconcileOracle {
    fn from_current(current: &SurfaceReconciler) -> Self {
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

    fn reconcile(&mut self, tree: &crate::ComponentTree) -> Option<ui_contract::UiPatch> {
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
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn diff_node(&mut self, parent: Option<ui_contract::UiNodeId>, node: &crate::TreeNode, ops: &mut ui_contract::UiPatchOps) -> ui_contract::UiNodeId {
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
    fn diff_existing(&mut self, id: ui_contract::UiNodeId, node: &crate::TreeNode, ops: &mut ui_contract::UiPatchOps) {
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

    /// 💰️ Wire-cost estimate for `candidate_ops`, delegated to [`ui_contract::patch_byte_estimate`]
    /// via a throwaway single-purpose [`ui_contract::UiPatch`] — the byte-accounting logic (including
    /// which fields even count as "text") lives once, in the contract crate that also enforces
    /// `max_patch_bytes`, and is never duplicated here.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn estimate_bytes<'a>(&self, candidate_ops: impl IntoIterator<Item = &'a ui_contract::UiPatchOp>) -> usize {
        let mut ops = ui_contract::UiPatchOps::default();
        for op in candidate_ops {
            ops.try_push(op.credited_clone().expect("test patch-op alias credit")).expect("test patch remains bounded");
        }
        let probe = ui_contract::UiPatch { surface: self.surface.clone(), base_revision: ui_contract::UiRevision::default(), revision: ui_contract::UiRevision::default(), ops };
        ui_contract::patch_byte_estimate(&probe)
    }

    /// 👶️ Diffs `new_children` against `old_child_ids` under `parent_id`, matching purely by
    /// `(parent_id, key)` — never by position — so reordering, inserting, and removing siblings each
    /// touch only the ids actually affected. Every old child whose id is not among the freshly diffed
    /// ids is removed as a whole subtree. Returns the new children list in `new_children`'s order,
    /// ready to become the parent's own `children` field.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
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
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn remove_subtree(&mut self, parent: Option<ui_contract::UiNodeId>, id: ui_contract::UiNodeId, ops: &mut ui_contract::UiPatchOps) {
        ops.try_push(ui_contract::UiPatchOp::Remove { id }).expect("test patch remains bounded");
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

fn build_record_owned(id: ui_contract::UiNodeId, node: crate::TreeNode, children: ui_contract::UiNodeChildren, transition: Option<ui_contract::TransitionHint>) -> ui_contract::UiNodeRecord {
    let crate::TreeNode { key, component, layout, style, activity, disabled, accessibility, bindings, menu, children: _, rejected_children: _ } = node;
    ui_contract::UiNodeRecord { id, key, component, layout, style, activity, disabled, transition, accessibility, bindings, menu, children }
}

/// 🏗️ Assembles a complete [`ui_contract::UiNodeRecord`] for `node` at `id` with `children` already
/// resolved to ids and `transition` carried over verbatim — [`crate::TreeNode`] has no `transition`
/// field of its own (see `🦀️present.rs`'s module doc: it is builder-side and never diffs against a
/// previous tree), so this reconciler is the one place a record's `transition` is set, and it never
/// invents one: `None` for a freshly seen node, whatever the retained record already carried for an
/// existing one. Driving `Introducing`/`Celebrating` from presence data is out of this packet's scope.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
#[cfg(test)]
fn build_record(id: ui_contract::UiNodeId, node: &crate::TreeNode, children: ui_contract::UiNodeChildren, transition: Option<ui_contract::TransitionHint>) -> ui_contract::UiNodeRecord {
    ui_contract::UiNodeRecord {
        id,
        key: node.key.clone(),
        component: node.component.credited_clone().expect("test component alias credit"),
        layout: node.layout.clone(),
        style: node.style,
        activity: node.activity,
        disabled: node.disabled,
        transition,
        accessibility: node.accessibility.clone(),
        bindings: ui_contract::credited_bindings(&node.bindings).expect("test binding alias credit"),
        menu: node.menu.as_ref().map(|menu| menu.credited_clone().expect("test menu alias credit")),
        children,
    }
}

//#endregion 🔖️Diff

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    mod ownership {
        include!("../../📏️ownership/🧪️component.rs");
    }

    mod canonical_document_tests {
        include!("../../📄️document/🧪️component.rs");
    }

    mod output_pool_tests {
        include!("../../📤️output/🧪️tests/🦀️.rs");
    }

    //#region 🔖️Fixtures
    fn ui_text(value: &str) -> ui_contract::UiText {
        ui_contract::UiText::try_from_str(value).expect("bounded fixture text")
    }

    fn leaf(key: &str) -> crate::TreeNode {
        crate::TreeNode::try_new(key, ui_contract::Component::Separator(ui_contract::SeparatorProps {})).expect("bounded fixture node")
    }

    fn text(key: &str, value: &str) -> crate::TreeNode {
        crate::TreeNode::try_new(key, ui_contract::Component::Text(ui_contract::TextProps { value: ui_contract::Label::try_from(value).expect("bounded fixture label"), emphasize: None, data_attributes: None })).expect("bounded fixture node")
    }

    fn container(key: &str, children: Vec<crate::TreeNode>) -> crate::TreeNode {
        let node = crate::TreeNode::try_new(
            key,
            ui_contract::Component::Container(ui_contract::ContainerProps { role: ui_contract::ContainerRole::Plain, label: None, description: None, required: None, error: None, default_open: None, drop_overlay: None }),
        )
        .expect("bounded fixture node");
        node.try_with_children(children).unwrap_or_else(|_| panic!("bounded fixture children"))
    }

    fn tree(root: crate::TreeNode) -> crate::ComponentTree {
        crate::ComponentTree::new(root)
    }

    fn ui_list(values: impl IntoIterator<Item = ui_contract::UiValue>) -> ui_contract::UiList {
        let mut builder = ui_contract::UiListBuilder::try_new().expect("fixed list builder");
        for value in values {
            builder.push(value).expect("fixed list page");
        }
        builder.finish()
    }

    fn ui_map(entries: impl IntoIterator<Item = (String, ui_contract::UiValue)>) -> ui_contract::UiMap {
        let mut builder = ui_contract::UiMapBuilder::try_new().expect("fixed map builder");
        for (key, value) in entries {
            builder.push(key, value).expect("ascending fixed map page");
        }
        builder.finish()
    }

    fn styled(node: crate::TreeNode, tone: ui_contract::Tone) -> crate::TreeNode {
        crate::TreeNode { style: ui_contract::StyleSpec { tone, ..Default::default() }, ..node }
    }

    fn with_shortcut(node: crate::TreeNode, shortcut: &str) -> crate::TreeNode {
        crate::TreeNode { accessibility: ui_contract::AccessibilitySpec { shortcut: Some(ui_text(shortcut)), ..Default::default() }, ..node }
    }

    fn with_binding(mut node: crate::TreeNode, scope: &str, name: &str) -> crate::TreeNode {
        node.bindings
            .try_push(ui_contract::ActionBinding { trigger: ui_contract::Trigger::Activate, action: ui_contract::ActionId::try_v1(scope, name).expect("bounded fixture action"), args: None, capability: None })
            .expect("bounded fixture bindings");
        node
    }

    fn with_menu(node: crate::TreeNode, menu_id: &str) -> crate::TreeNode {
        crate::TreeNode { menu: Some(ui_contract::MenuRef { id: ui_text(menu_id), args: None }), ..node }
    }

    fn id_of(snapshot: &ui_contract::UiSnapshot, key: &str) -> ui_contract::UiNodeId {
        snapshot.nodes.iter().find(|record| record.key.as_str() == key).unwrap_or_else(|| panic!("no node keyed {key:?} in snapshot")).id
    }

    fn first_op(patch: &ui_contract::UiPatch) -> &ui_contract::UiPatchOp {
        patch.ops.get(0).expect("fixture patch operation")
    }

    fn assert_snapshot_matches_state(snapshot: &ui_contract::UiSnapshot, state: &ui_contract::UiSnapshotState) {
        assert_eq!(snapshot.revision, state.revision);
        assert_eq!(Some(snapshot.root), state.root);
        assert_eq!(snapshot.nodes.len(), state.nodes.len(), "snapshot/state node-count mismatch");
        for record in &snapshot.nodes {
            assert_eq!(state.nodes.get(&record.id), Some(record), "record {:?} diverges between snapshot and applied state", record.id);
        }
    }

    fn reconcile_resumable(current: &SurfaceReconciler, component_tree: crate::ComponentTree) -> (SurfaceReconciler, Option<SurfaceReconcileReadyPatch>, usize) {
        let mut cursor = SurfaceReconcileCursor::new(component_tree, current);
        let mut yields = 0;
        loop {
            match cursor.step(current) {
                SurfaceReconcileStep::Yield { .. } => yields += 1,
                SurfaceReconcileStep::Complete { mut reconciler, patch } => {
                    let mut output = None;
                    while !reconciler.seal_step(cursor.usage, &mut output, patch.is_some()).unwrap() {}
                    let ready = patch.map(|patch| SurfaceReconcileReadyPatch { generation: cursor.assembly_generation, patch: pending_surface_patch(Some(patch)), credit: output, handback: reserve_surface_reconcile_handback(cursor.assembly_generation) });
                    while !cursor.retire_one() {}
                    return (reconciler, ready, yields);
                }
                SurfaceReconcileStep::Fault(fault) => panic!("unexpected reconcile fault: {fault:?}"),
            }
        }
    }
    //#endregion 🔖️Fixtures

    //#region ⏭️ResumableCursor
    #[test]
    fn instance_lifetime_published_patch_close_retains_exact_handback_until_terminal() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../🎭️actor/🚪️lifetime/🧪️fixture.json")).unwrap();
        let mut reservation = SurfaceReconcileReservation::try_new(8_971).expect("real published owner reservation");
        let metadata = pending_surface_patch(Some(ui_contract::UiPatch { surface: ui_contract::SurfaceId::try_from("7:retained").unwrap(), base_revision: ui_contract::UiRevision(0), revision: ui_contract::UiRevision(1), ops: ui_contract::UiPatchOps::default() }));
        let mut owner = SurfaceReconcilePublishedPatch { generation: 8_971, metadata, revision: ui_contract::UiRevision(1), credit: reservation.credit.take(), handback: reservation.handback.take() };
        let key = owner.handback.as_ref().unwrap().key;
        let mut remaining = Vec::new();
        let mut complete = Vec::new();
        let mut prior = vec!["metadata", "credit", "handback"];
        let mut turns = 0;
        let mut bytes = 0;
        assert_eq!(fixture["publishedClose"]["granularity"], "owner-transition");
        for _ in 0..100_000 {
            let step = owner.close_step_with_grant(1, 1).unwrap();
            assert!(step.released_items <= 1 && step.released_bytes <= 1);
            turns += 1;
            bytes += step.released_bytes;
            let fields: Vec<_> = [("metadata", !owner.metadata.terminal_is_empty()), ("credit", owner.credit.is_some()), ("handback", owner.handback.is_some())].into_iter().filter_map(|(name, retained)| retained.then_some(name)).collect();
            if fields != prior || step.complete { prior = fields.clone(); remaining.push(fields); complete.push(step.complete); }
            if step.complete { break; }
        }
        assert_eq!(bytes, "7:retained".len());
        assert_eq!(serde_json::to_value(&remaining).unwrap(), fixture["publishedClose"]["remaining"]);
        assert_eq!(serde_json::to_value(complete).unwrap(), fixture["publishedClose"]["complete"]);
        assert!(owner.terminal_is_empty());
        eprintln!("[DEBUG] published-close owner-transitions={} physical-turns={turns} semantic-bytes={bytes} grant=1", remaining.len());
        let registry = SURFACE_RECONCILE_HANDBACKS.lock().unwrap();
        let slot = &registry.slots[key.slot];
        assert!(slot.epoch != key.epoch || (!slot.reserved && slot.state.is_none()), "the exact handback was released, not queued and forgotten");
        drop(registry);
        drop(owner);
    }

    #[test]
    fn fixed_runtime_owners_keep_bounded_state_off_the_stack() {
        eprintln!("[DEBUG] canonical-owner-layout reconciler={} cursor={} retained={}", size_of::<SurfaceReconciler>(), size_of::<SurfaceReconcileCursor>(), size_of::<SurfaceReconcileRetained>());
        assert!(size_of::<SurfaceReconciler>() <= 1_024);
        assert!(size_of::<SurfaceReconcileCursor>() <= 48 * 1_024);
        assert!(size_of::<SurfaceReconcileRetained>() <= 64 * 1_024);
        assert!(size_of::<crate::TreeNode>() <= 8 * 1_024);
        assert!(size_of::<ui_contract::UiNodeRecord>() <= 8 * 1_024);
    }

    #[test]
    fn resumable_cursor_matches_the_existing_keyed_diff_and_revision_semantics() {
        let component_tree = tree(container("root", vec![text("a", "hello"), container("b", vec![leaf("x"), leaf("y")])]));
        let mut direct = SurfaceReconciler::new("s");
        let expected_patch = direct.reconcile(&component_tree).expect("initial direct patch");

        let current = SurfaceReconciler::new("s");
        let (resumed, mut actual_patch, yields) = reconcile_resumable(&current, component_tree);

        assert_eq!(actual_patch.as_ref().and_then(|patch| patch.patch.get()), Some(&expected_patch));
        if let Some(patch) = actual_patch.as_mut() { while !patch.close_step() {} }
        let actual = resumed.snapshot();
        let expected = direct.snapshot();
        assert_eq!(actual.revision, expected.revision);
        assert_eq!(actual.root, expected.root);
        assert_eq!(actual.nodes.len(), expected.nodes.len());
        assert!(expected.nodes.iter().all(|record| actual.nodes.iter().any(|candidate| candidate == record)));
        assert!(yields >= 15, "five nodes must cross traversal, identity, and diff cursors");
    }

    #[test]
    fn abandoned_large_tree_cursor_leaves_the_retained_shadow_and_revision_unchanged() {
        let mut current = SurfaceReconciler::new("s");
        current.reconcile(&tree(container("root", vec![leaf("baseline")]))).expect("baseline");
        let before = current.snapshot();
        let children = (0..30).map(|index| leaf(&format!("item-{index}"))).collect();
        let mut cursor = SurfaceReconcileCursor::new(tree(container("root", children)), &current);

        assert!(matches!(cursor.step(&current), SurfaceReconcileStep::Yield { .. }));
        drop(cursor);

        assert_eq!(current.snapshot(), before, "cancellation or supersession must discard only candidate state");
    }

    #[test]
    fn every_large_tree_cursor_slice_stays_below_eight_milliseconds() {
        use std::time::{Duration, Instant};

        let children = (0..30).map(|index| leaf(&format!("item-{index}"))).collect();
        let current = SurfaceReconciler::new("s");
        let mut cursor = SurfaceReconcileCursor::new(tree(container("root", children)), &current);
        let mut yields = 0;
        loop {
            let started = Instant::now();
            let step = cursor.step(&current);
            let elapsed = started.elapsed();
            assert!(elapsed < Duration::from_millis(8), "one node cursor slice took {elapsed:?}");
            match step {
                SurfaceReconcileStep::Yield { nodes, .. } => {
                    assert!(nodes <= 1);
                    yields += 1;
                }
                SurfaceReconcileStep::Complete { mut reconciler, patch } => {
                    let mut output = None;
                    loop {
                        let started = Instant::now();
                        let complete = reconciler.seal_step(cursor.usage, &mut output, patch.is_some()).unwrap();
                        assert!(started.elapsed() < Duration::from_millis(8), "canonical seal exceeds one callback contract");
                        if complete { break; }
                    }
                    assert_eq!(reconciler.snapshot().nodes.len(), 31);
                    assert!(patch.is_some());
                    let mut ready = SurfaceReconcileReadyPatch { generation: cursor.assembly_generation, patch: pending_surface_patch(patch), credit: output, handback: reserve_surface_reconcile_handback(cursor.assembly_generation) };
                    while !ready.close_step() {}
                    while !cursor.retire_one() {}
                    while !reconciler.retire_one() {}
                    break;
                }
                SurfaceReconcileStep::Fault(fault) => panic!("unexpected reconcile fault: {fault:?}"),
            }
        }
        assert!(yields >= 93, "every presented node crosses three independent cursor phases");
    }

    #[test]
    fn identifier_cap_plus_one_returns_the_exact_tree_owner_before_cursor_mutation() {
        let surface = "s".repeat(SurfaceReconcileLimits::default().max_identifier_bytes + 1);
        let tree = tree(leaf("exact"));
        let mut rejected = match SurfaceReconcileJob::try_new(SurfaceReconciler::new(surface), tree, 71) {
            Ok(_) => panic!("identifier + 1 must reject"),
            Err(rejected) => rejected,
        };
        let (_, returned) = rejected.take_sources().expect("exact rejected owners");
        assert_eq!(returned.root.key.as_str(), "exact");
        while !rejected.close_step() {}
        assert!(rejected.terminal_is_empty());
    }

    #[test]
    fn semantic_aggregate_quota_faults_before_key_or_record_clone() {
        let mut data_attributes = ui_contract::UiFixedMap::default();
        data_attributes.try_push(ui_text("semantic"), ui_text("payload")).expect("bounded fixture attribute");
        let node = crate::TreeNode::try_new("exact", ui_contract::Component::Text(ui_contract::TextProps { value: ui_contract::Label(ui_text("value")), emphasize: None, data_attributes: Some(data_attributes) })).expect("bounded fixture node");
        let current = SurfaceReconciler::new("s");
        let limits = SurfaceReconcileLimits { max_bytes: SURFACE_RECONCILE_PAGE_BYTES, ..Default::default() };
        let mut cursor = SurfaceReconcileCursor::new_with_limits(tree(node), &current, limits);
        let mut fault = None;
        for _ in 0..4_096 {
            if let SurfaceReconcileStep::Fault(found) = cursor.step(&current) {
                fault = Some(found);
                break;
            }
        }
        assert!(matches!(fault, Some(SurfaceReconcileFault::Credits { .. })));
        let retained = cursor.held_node.as_ref().expect("exact unmaterialized node remains retained");
        assert_eq!(retained.1.key.as_str(), "exact");
        assert!(cursor.flat.is_empty());
        assert!(cursor.seen.is_empty());
        assert!(cursor.new_ordinals.is_empty());
        assert!(cursor.ops.is_empty());
    }

    #[test]
    fn opaque_surface_document_uses_aggregate_credits_instead_of_scalar_page() {
        let payload = vec![7; ui_contract::UI_FIXED_BYTES];
        let props = ui_contract::SurfaceProps {
            kind: ui_contract::SurfaceKind::NodeGraph,
            doc_schema: ui_text("node-graph@1"),
            doc: ui_contract::SurfaceDoc { bytes: ui_contract::UiFixedBytes::try_from_vec(payload.clone()).expect("fixed surface payload") },
            bindings: Default::default(),
        };
        let node = crate::TreeNode::try_new("surface", ui_contract::Component::Surface(props)).expect("bounded surface node");
        let current = SurfaceReconciler::new("s");
        let (reconciled, mut patch, _) = reconcile_resumable(&current, tree(node));

        assert!(patch.is_some(), "the opaque surface publishes through the same transactional patch path");
        if let Some(patch) = patch.as_mut() { while !patch.close_step() {} }
        let snapshot = reconciled.snapshot();
        let ui_contract::Component::Surface(actual) = &snapshot.nodes[0].component else { panic!("surface component") };
        assert_eq!(actual.doc.bytes.as_slice(), payload);
        let json = serde_json::to_value(&snapshot).expect("third-party snapshot serialization");
        assert_eq!(json["nodes"][0]["component"]["doc"]["bytes"].as_array().map(Vec::len), Some(ui_contract::UI_FIXED_BYTES));
    }

    #[test]
    fn semantic_census_zero_fuel_and_expired_deadline_leave_every_cursor_and_owner_unchanged() {
        fn expired_now() -> Option<u64> {
            Some(10)
        }
        let generation = 7_001;
        let mut zero = SurfaceReconcileJob::try_new(SurfaceReconciler::new("s"), tree(leaf("zero")), generation).expect("admitted");
        let mut sequence = 0;
        let mut context = semio_framework_job::StepContext::new(
            semio_framework_job::allocate_operation_id(),
            semio_framework_job::Generation(generation),
            semio_framework_job::StepBudget::new(0, u64::MAX),
            semio_framework_job::root_cancel_token(),
            semio_framework_job::default_now_us,
            &mut sequence,
        );
        assert_eq!(zero.drive_one(&mut context), SurfaceReconcileJobStep::MoreWork);
        let cursor = zero.state.as_ref().and_then(|state| state.cursor.as_ref()).expect("cursor retained");
        assert!(cursor.pending_root.is_some());
        assert!(cursor.held_node.is_none());

        let generation = 7_002;
        let mut expired = SurfaceReconcileJob::try_new(SurfaceReconciler::new("s"), tree(leaf("deadline")), generation).expect("admitted");
        let mut sequence = 0;
        let mut context = semio_framework_job::StepContext::new(
            semio_framework_job::allocate_operation_id(),
            semio_framework_job::Generation(generation),
            semio_framework_job::StepBudget::new(1, 10),
            semio_framework_job::root_cancel_token(),
            expired_now,
            &mut sequence,
        );
        assert_eq!(expired.drive_one(&mut context), SurfaceReconcileJobStep::MoreWork);
        assert!(expired.state.as_ref().and_then(|state| state.cursor.as_ref()).is_some_and(|cursor| cursor.pending_root.is_some() && cursor.held_node.is_none()));
    }

    #[test]
    fn semantic_census_low_fuel_wide_container_and_deep_value_advance_one_unit_without_recursion() {
        let wide = (0..128).map(|index| ui_contract::UiValue::Text(ui_contract::UiText::try_from_string(format!("value-{index}")).expect("bounded fixture text"))).collect::<Vec<_>>();
        let node = crate::TreeNode::try_new("wide", ui_contract::Component::Extension(ui_contract::ExtensionProps { extension: ui_text("fixture"), props: ui_contract::UiValue::List(ui_list(wide)) })).expect("bounded fixture node");
        let current = SurfaceReconciler::new("s");
        let mut cursor = SurfaceReconcileCursor::new(tree(node), &current);
        for _ in 0..32 {
            assert!(matches!(cursor.step(&current), SurfaceReconcileStep::Yield { .. }));
        }
        assert!(cursor.flat.is_empty(), "a wide value cannot complete census in one grant");

        let mut deep = ui_contract::UiValue::Null;
        for _ in 0..=SURFACE_RECONCILE_VALUE_DEPTH {
            deep = ui_contract::UiValue::List(ui_list([deep]));
        }
        let node = crate::TreeNode::try_new("deep", ui_contract::Component::Extension(ui_contract::ExtensionProps { extension: ui_text("fixture"), props: deep })).expect("bounded fixture node");
        let mut cursor = SurfaceReconcileCursor::new(tree(node), &current);
        let mut fault = None;
        for _ in 0..8_192 {
            if let SurfaceReconcileStep::Fault(found) = cursor.step(&current) {
                fault = Some(found);
                break;
            }
        }
        assert!(matches!(fault, Some(SurfaceReconcileFault::ValueDepth { .. })));
        assert!(cursor.flat.is_empty());
    }

    #[test]
    fn retained_map_page_advances_each_key_once_without_rewalking_prior_entries() {
        let value = ui_contract::UiValue::Map(ui_map([("a".to_owned(), ui_contract::UiValue::Null), ("b".to_owned(), ui_contract::UiValue::Null), ("c".to_owned(), ui_contract::UiValue::Null)]));
        let mut cursor = SurfaceSemanticCensusCursor::default();
        cursor.push_value(&value).expect("fixed value depth");
        let mut steps = 0;
        while cursor.depth > 0 {
            cursor.value_step().expect("retained map cursor progress");
            steps += 1;
            assert!(steps <= 11, "three entries must never rewalk prior pages");
        }
        assert_eq!(steps, 11);
    }

    #[test]
    fn allocate_inspect_admit_retains_exact_vector_backing_on_cap_plus_one_without_partial_item_mutation() {
        let mut owner = Vec::<u64>::new();
        let mut usage = SurfaceReconcileUsage::default();
        let limits = SurfaceReconcileLimits { max_nodes: 0, max_items: 0, max_bytes: 0, max_identifier_bytes: 0 };
        let fault = admit_vec_backing(&mut owner, &mut usage, limits).expect_err("first backing slot exceeds zero cap");
        assert!(owner.capacity() >= 1, "allocate-inspect retains the exact allocated page on refusal");
        assert!(owner.is_empty(), "no logical item mutates before page admission");
        assert!(matches!(fault, SurfaceReconcileFault::Credits { .. }));

        let actual_capacity = owner.capacity();
        let exact = SurfaceReconcileLimits { max_nodes: 0, max_items: actual_capacity, max_bytes: actual_capacity * size_of::<u64>(), max_identifier_bytes: 0 };
        let mut admitted = Vec::<u64>::new();
        let mut usage = SurfaceReconcileUsage::default();
        admit_vec_backing(&mut admitted, &mut usage, exact).expect("actual inspected cap admits");
        admitted.push(7);
        assert_eq!(admitted, [7]);
    }

    #[test]
    fn persistent_credit_transfers_through_ready_and_returns_only_after_incremental_retirement() {
        let generation = 7_003;
        let mut job = SurfaceReconcileJob::try_new(SurfaceReconciler::new("s"), tree(leaf("credit")), generation).expect("admitted");
        let mut sequence = 0;
        for _ in 0..4_096 {
            let mut context = semio_framework_job::StepContext::new(
                semio_framework_job::allocate_operation_id(),
                semio_framework_job::Generation(generation),
                semio_framework_job::StepBudget::new(1, u64::MAX),
                semio_framework_job::root_cancel_token(),
                semio_framework_job::default_now_us,
                &mut sequence,
            );
            if job.drive_one(&mut context) == SurfaceReconcileJobStep::Ready {
                break;
            }
        }
        let (reconciler, ready_patch) = match job.take_ready() {
            Ok(ready) => ready,
            Err(_) => panic!("ready owner"),
        };
        let retained_credit = reconciler.document.as_ref().expect("take_ready retains original canonical root").try_read().unwrap().resident_limits();
        assert!(retained_credit.items < SurfaceReconcileLimits::default().max_items, "ready reconciliation returns unused aggregate item capacity");
        assert!(retained_credit.bytes < SurfaceReconcileLimits::default().max_bytes, "ready reconciliation returns unused aggregate byte capacity");
        let mut ready_patch = ready_patch.expect("initial reconciliation publishes a patch");
        let patch_credit = ready_patch.credit.as_ref().expect("ready patch shares the retained credit");
        assert_eq!((patch_credit.limits().items, patch_credit.limits().bytes), (retained_credit.items, retained_credit.bytes));
        while !ready_patch.close_step() {}
        let mut terminal = SurfaceReconcileTerminal::try_from_reconciler(reconciler, generation).expect("pre-admitted terminal handback");
        assert!(!terminal.close_step());
        while !terminal.terminal_is_empty() {
            terminal.close_step();
        }
    }

    #[test]
    fn public_drop_handback_is_lossless_at_terminal_cap_and_plus_one() {
        let first = 80_000;
        let mut keys = Vec::with_capacity(SURFACE_RECONCILE_HANDBACK_SLOTS);
        for offset in 0..SURFACE_RECONCILE_HANDBACK_SLOTS {
            let terminal = SurfaceReconcileTerminal::try_from_reconciler(SurfaceReconciler::new(format!("drop-{offset}")), first + offset as u64).expect("public cap admits");
            keys.push(terminal.handback_key().expect("fixed registry key"));
            drop(terminal);
        }
        let overflow = SurfaceReconciler::new("overflow-owner");
        let returned = match SurfaceReconcileTerminal::try_from_reconciler(overflow, first + SURFACE_RECONCILE_HANDBACK_SLOTS as u64) {
            Ok(_) => panic!("public cap + 1 must refuse"),
            Err(returned) => returned,
        };
        assert_eq!(returned.surface().0.as_str(), "overflow-owner");
        for key in keys {
            let mut terminal = take_surface_reconcile_terminal(key).expect("valid handback registry").expect("every fixed handback owner remains O(1) recoverable");
            while !terminal.terminal_is_empty() {
                terminal.close_step();
            }
        }
    }

    #[test]
    fn stale_cancel_and_drop_handoff_preserve_public_terminal_ownership() {
        let generation = 8_001;
        let mut job = SurfaceReconcileJob::try_new(SurfaceReconciler::new("s"), tree(leaf("exact")), generation).expect("admitted");
        let mut sequence = 0;
        let mut context = semio_framework_job::StepContext::new(
            semio_framework_job::allocate_operation_id(),
            semio_framework_job::Generation(generation + 1),
            semio_framework_job::StepBudget::new(1, u64::MAX),
            semio_framework_job::root_cancel_token(),
            semio_framework_job::default_now_us,
            &mut sequence,
        );
        assert_eq!(job.drive_one(&mut context), SurfaceReconcileJobStep::Fault);
        let handback_key = job.handback_key().expect("fault retains fixed public handback reservation");
        drop(job);
        let mut terminal = take_surface_reconcile_terminal(handback_key).expect("valid handback registry").expect("drop handback is observable in O(1)");
        assert!(matches!(terminal.fault(), Some(SurfaceReconcileFault::StaleGeneration { .. })));
        for _ in 0..32 {
            if terminal.close_step() && terminal.terminal_is_empty() {
                break;
            }
        }
        assert!(terminal.terminal_is_empty());

        let generation = 8_002;
        let mut job = SurfaceReconcileJob::try_new(SurfaceReconciler::new("s"), tree(leaf("cancel")), generation).expect("admitted");
        let cancel = semio_framework_job::root_cancel_token();
        cancel.cancel_now();
        let mut sequence = 0;
        let mut context = semio_framework_job::StepContext::new(
            semio_framework_job::allocate_operation_id(),
            semio_framework_job::Generation(generation),
            semio_framework_job::StepBudget::new(1, u64::MAX),
            cancel,
            semio_framework_job::default_now_us,
            &mut sequence,
        );
        assert_eq!(job.drive_one(&mut context), SurfaceReconcileJobStep::Fault);
        let mut terminal = job.into_terminal();
        assert!(matches!(terminal.fault(), Some(SurfaceReconcileFault::Cancelled)));
        for _ in 0..32 {
            if terminal.close_step() && terminal.terminal_is_empty() {
                break;
            }
        }
        assert!(terminal.terminal_is_empty());
    }
    //#endregion ⏭️ResumableCursor

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
                assert_eq!(component, &ui_contract::Component::Text(ui_contract::TextProps { value: ui_contract::Label::try_from("world").expect("bounded fixture label"), emphasize: None, data_attributes: None }));
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
            ui_contract::UiPatchOp::SetChildren { children, .. } => assert_eq!(children.iter().copied().collect::<Vec<_>>(), vec![c_id, a_id, b_id]),
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
        assert!(patch.ops.iter().any(|op| matches!(op, ui_contract::UiPatchOp::Upsert(record) if record.key.as_str() == "b")));

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

        let mut changed = crate::TreeNode::try_new(
            "a",
            ui_contract::Component::Container(ui_contract::ContainerProps { role: ui_contract::ContainerRole::Plain, label: None, description: None, required: None, error: None, default_open: None, drop_overlay: None }),
        )
        .expect("bounded fixture node");
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
        let mut children = ui_contract::BuiltChildren::default();
        children.try_push(leaf("a")).expect("bounded fixture child");
        children.try_push(leaf("a")).expect("bounded fixture child");
        let root = crate::TreeNode { children, ..leaf("root") };
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
        let mut receiver_state = ui_contract::UiSnapshotState::new(ui_contract::SurfaceId::try_from("s").expect("bounded fixture surface"));
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

        let mut generation = 1u64;
        for component_tree in &frames {
            if let Some(patch) = reconciler.reconcile(component_tree) {
                let mut producer = ui_contract::UiPatchApplyProducer::try_new(receiver_state, patch, limits, generation).expect("every emitted patch must enter the retained contract producer");
                loop {
                    match producer.drive_one(generation, false, false) {
                        ui_contract::UiPatchApplyStep::MoreWork => {}
                        ui_contract::UiPatchApplyStep::Ready => break,
                        ui_contract::UiPatchApplyStep::Rejected => panic!("every emitted patch must apply cleanly against the contract producer: {:?}", producer.rejection()),
                    }
                }
                let mut outcome = producer.take_ready().unwrap_or_else(|_| panic!("ready producer must transfer its exact outcome"));
                while !outcome.close_step() {}
                receiver_state = outcome.take_state().unwrap_or_else(|_| panic!("closed outcome must return the exact new state"));
                generation = generation.checked_add(1).expect("bounded fixture generation");
            }
            assert_snapshot_matches_state(&reconciler.snapshot(), &receiver_state);
        }
    }
    //#endregion 🔖️RoundTripProperty
}
//#endregion 🧪️Tests
