//! ⏭️ The retained surface shadow and the resumable reconcile cursor that diffs a presented tree against it — presentation discovery, identity allocation, postorder record diffing and stale-tree removal each advance one node at a time, and the shadow is read-only until the cursor completes. The authority crate composes the shadow with the process-wide handback registry; this crate knows no registry, no job and no renderer.
//!
//! The one property that makes the emitted patches worth anything: identity comes from **`(parent, key)`, never from
//! position**. A node keeps its [`ui_contract::UiNodeId`] across reorders, insertions and removals of its siblings,
//! which is exactly what lets renderer-side state (scroll offset, focus, a DOM node, a GPU cache entry) survive a
//! re-present instead of being torn down and rebuilt every frame.

use std::mem::{size_of, take};
#[cfg(any(test, feature = "testkit"))]
use std::collections::HashSet;

use semio_framework_ui_runtime_presentation::{ComponentTree, TreeNode, UiNativeResourceLoans, UiNativeResources};
use semio_framework_ui_runtime_reconcile_primitives::{ExistingComponentComparison, FlatPresentedNode, FreshRecordClone, NodeIdentity, PresentationFrame, RecordDiffCursor, RecordOwnedCopy, RemovalFrame, SURFACE_COMPONENT_COPY_WORK_BYTES, SURFACE_RECONCILE_FIXED_NODES, SURFACE_RECONCILE_FLOOR_BYTES, SURFACE_RECONCILE_FLOOR_ITEMS, SURFACE_RECONCILE_PAGE_BYTES, SURFACE_RECONCILE_SURFACE_BYTES, SURFACE_RECONCILE_VALUE_DEPTH, SurfaceFixedVec, SurfaceLinearMap, SurfaceLinearSet, SurfaceReconcileFault, SurfaceReconcileLimits, SurfaceReconcileUsage, SurfaceSemanticCensusCursor, SurfaceSemanticCensusStep, SurfaceSemanticUsage, SurfaceTreeRetireCursor, build_record_owned, diff_record_field, release_surface_reconcile_one, reserve_surface_reconcile_floor, retire_fresh_record_one, retire_record_one, surface_reconcile_limits_admissible};
#[cfg(any(test, feature = "testkit"))]
use semio_framework_ui_runtime_reconcile_primitives::{assert_unique_child_keys, build_record, identity_of};

#[cfg(any(test, feature = "testkit"))]
#[path = "🧰️testkit/🦀️.rs"]
pub mod testkit;

//#region 🔖️Reconciler

/// ♻️ The retained shadow of one render surface — the component a keyed reconciliation diffs against. Owns a shadow
/// copy of what the receiver has (`document`, `root`) plus the `(parent, key) → id` index (`key_index`) that carries
/// every node's identity across frames, and the monotonic `allocator` that mints an id for a node the first time it
/// is ever seen. A completed reconcile is the only place any of these change together. The authority crate composes
/// it with the handback reservation it owes the process-wide registry.
#[derive(Debug)]
pub struct SurfaceShadow {
    surface: ui_contract::SurfaceId,
    revision: ui_contract::UiRevision,
    allocator: ui_contract::UiNodeIdAllocator,
    document: Option<ui_contract::UiDocumentLease>,
    native_resources: Option<UiNativeResources>,
    native_resource_loans: UiNativeResourceLoans,
    assembly: ui_contract::UiDocumentAssembly,
    ordinals: SurfaceLinearMap<ui_contract::UiNodeId, usize, SURFACE_RECONCILE_FIXED_NODES>,
    key_index: SurfaceLinearMap<NodeIdentity, ui_contract::UiNodeId, SURFACE_RECONCILE_FIXED_NODES>,
    root: Option<ui_contract::UiNodeId>,
    retire_scalar: u8,
    seal_phase: u8,
}

impl SurfaceShadow {
    /// 🌱️ A reconciler for `surface` with no retained state yet — the next [`Self::reconcile`] call
    /// necessarily emits a full `SetRoot` plus one `Upsert` per node, exactly as [`Self::mark_rejected`]
    /// arranges for an existing reconciler to do again.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn new(surface: ui_contract::SurfaceId) -> Self {
        Self {
            surface,
            revision: ui_contract::UiRevision::default(),
            allocator: ui_contract::UiNodeIdAllocator::default(),
            document: None,
            native_resources: None,
            native_resource_loans: Default::default(),
            assembly: ui_contract::UiDocumentAssembly::default(),
            ordinals: SurfaceLinearMap::default(),
            key_index: SurfaceLinearMap::default(),
            root: None,
            retire_scalar: 0,
            seal_phase: 0,
        }
    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn from_text(surface: impl AsRef<str>) -> Self {
        let surface = ui_contract::UiText::try_from_str(surface.as_ref()).map(ui_contract::SurfaceId).expect("bounded test surface");
        Self::new(surface)
    }

    /// ♻️ Diffs `tree` against this reconciler's retained state, mutating that state to match and
    /// returning the minimal [`ui_contract::UiPatch`] that carries the difference — or `None` when
    /// `tree` is structurally and semantically identical to what was last presented, so an idle surface
    /// produces no wire traffic at all. `base_revision` is the revision the receiver is assumed to be
    /// at; `revision` is one past it — this reconciler never emits a gap or a repeat.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    #[cfg(any(test, feature = "testkit"))]
    pub fn reconcile(&mut self, tree: &ComponentTree) -> Option<ui_contract::UiPatch> {
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
    #[cfg(any(test, feature = "testkit"))]
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

    #[cfg(any(test, feature = "testkit"))]
    pub fn transaction_reader(&self) -> Self {
        let mut reader = Self::new(self.surface.clone());
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

    pub fn read_record(&self, id: ui_contract::UiNodeId) -> Result<Option<SurfaceRecordRead<'_>>, ui_contract::UiDocumentLeaseError> {
        let Some(ordinal) = self.ordinals.get(&id).copied() else { return Ok(None); };
        let document = self.document.as_ref().ok_or(ui_contract::UiDocumentLeaseError::StaleHandle)?;
        let read = document.try_read()?;
        read.exact_node(ordinal, id)?;
        Ok(Some(SurfaceRecordRead { read, ordinal, id }))
    }

    pub fn seal_step(&mut self, owner_bytes: usize, usage: SurfaceReconcileUsage, output: &mut Option<ui_contract::UiResidentPermit>, has_output: bool) -> Result<bool, ui_contract::UiDocumentAssemblyError> {
        if self.document.is_some() { return Ok(true); }
        let progress = match self.seal_phase {
            0 => {
                let fixed = owner_bytes + size_of_val(self.ordinals.entries.entries.as_ref()) + size_of_val(self.key_index.entries.entries.as_ref()) + ui_contract::UiDocumentAssembly::required_open_bytes();
                let allocated = self.assembly.allocated_bytes()?;
                let bytes = usage.bytes.checked_add(fixed).and_then(|bytes| bytes.checked_add(allocated)).unwrap_or(usize::MAX);
                self.assembly.reprice_resident(ui_contract::UiResidentLimits { items: usage.items.max(self.ordinals.len()).max(1), bytes }, 1, SURFACE_RECONCILE_PAGE_BYTES)?
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
    #[cfg(any(test, feature = "testkit"))]
    pub fn mark_rejected(&mut self) {
        if let Some(document) = self.document.as_mut() { while !document.close_read_step_with_grant(1, 4096).unwrap().complete {} }
        self.document = None;
        self.ordinals.clear();
        self.key_index.clear();
        self.root = None;
        self.revision = ui_contract::UiRevision::default();
    }

    pub fn native_resources(&self) -> Option<&UiNativeResources> { self.native_resources.as_ref() }

    pub fn native_resource_loans_terminal_is_empty(&self)->bool{self.native_resource_loans.terminal_is_empty()}


    pub fn with_native_resource_loans_on_original<R>(&mut self,cx:&mut semio_framework_job::StepContext<'_>,receive:impl FnOnce(&mut UiNativeResourceLoans,&mut semio_framework_job::StepContext<'_>)->Result<R,semio_framework_job::OriginalTurnFailure>)->Result<Option<R>,semio_framework_job::OriginalTurnFailure>{
        let Some(resources)=self.native_resources.as_ref() else{return Ok(None)};
        if !self.native_resource_loans.capture_complete(){if self.native_resource_loans.capture_step_on_original(resources,cx)?!=Some(true){return Ok(None)}}
        receive(&mut self.native_resource_loans,cx).map(Some)
    }

    pub fn close_native_resources_on_original(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<Option<bool>, semio_framework_job::OriginalTurnFailure> {
        if !self.native_resource_loans.terminal_is_empty(){if self.native_resource_loans.close_step_on_original(cx)?!=Some(true){return Ok(None)}return Ok(Some(false));}
        let Some(resources) = self.native_resources.as_mut() else { return Ok(Some(true)); };
        if resources.close_step_on_original(cx)? != Some(true) { return Ok(None); }
        self.native_resources = None;
        Ok(Some(false))
    }

    /// 🧹️ One retirement unit of this shadow, answering whether the shadow is fully retired. The handback
    /// reservation the owner holds is released at the same step the monolithic owner released it, through the
    /// explicit `release` parameter, so the shadow never learns the registry that issued it.
    pub fn retire_one<H>(&mut self, handback: &mut Option<H>, release: impl FnOnce(H)) -> bool {
        if self.native_resources.is_some() || !self.native_resource_loans.terminal_is_empty() { return false; }
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
                if let Some(handback) = handback.take() {
                    release(handback);
                }
            }
            _ => return true,
        }
        self.retire_scalar += 1;
        self.retire_scalar >= 6
    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn retire_one_detached(&mut self) -> bool {
        self.retire_one(&mut None::<()>, drop)
    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn retire_scalar(&self) -> u8 {
        self.retire_scalar
    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn document(&self) -> Option<&ui_contract::UiDocumentLease> {
        self.document.as_ref()
    }

    /// 📦️ Moves every retained owner into a fresh shadow, leaving this one empty.
    pub fn take_all(&mut self) -> Self {
        Self {
            surface: take(&mut self.surface),
            revision: take(&mut self.revision),
            allocator: take(&mut self.allocator),
            document: self.document.take(),
            native_resources: self.native_resources.take(),
            native_resource_loans: take(&mut self.native_resource_loans),
            assembly: take(&mut self.assembly),
            ordinals: self.ordinals.take_all(),
            key_index: self.key_index.take_all(),
            root: self.root.take(),
            retire_scalar: self.retire_scalar,
            seal_phase: self.seal_phase,
        }
    }

    /// 🧾️ `true` when no retained owner is left to retire.
    pub fn terminal_is_empty(&self) -> bool {
        self.document.is_none() && self.native_resources.is_none() && self.native_resource_loans.terminal_is_empty() && self.assembly.terminal_is_empty() && self.ordinals.is_empty() && self.key_index.is_empty()
    }

    /// 📖️ `true` when the canonical root has been sealed.
    pub fn has_document(&self) -> bool {
        self.document.is_some()
    }

    /// 📖️ `true` when `lease` aliases this shadow's canonical root.
    pub fn document_root_matches(&self, lease: Option<&ui_contract::UiDocumentLease>) -> bool {
        self.document.as_ref().zip(lease).is_some_and(|(current, captured)| current.same_root(captured))
    }

    /// 🪪️ The byte length of the surface identifier this shadow was opened for.
    pub fn surface_identifier_bytes(&self) -> usize {
        self.surface.0.len()
    }
}

pub struct SurfaceRecordRead<'a> {
    read: ui_contract::UiDocumentRead<'a>,
    ordinal: usize,
    id: ui_contract::UiNodeId,
}

#[cfg(any(test, feature = "testkit"))]
impl SurfaceShadow {
    fn install_oracle(&mut self, mut oracle: SurfaceReconcileOracle) {
        if let Some(document) = self.document.as_mut() { while !document.close_read_step_with_grant(1, 4096).unwrap().complete {} }
        self.document = None;
        self.ordinals.clear();
        let mut credit = reserve_surface_reconcile_floor(SurfaceReconcileLimits { max_bytes: 1024 * 1024, ..Default::default() });
        let mut surface = Some(self.surface.clone());
        let mut assembly = ui_contract::UiDocumentAssembly::default();
        assert!(assembly.open_with_permit(&mut credit, &mut surface, ui_contract::UiDocumentAssemblyIdentity { generation: oracle.revision.0.checked_add(1).unwrap(), revision: oracle.revision, root: oracle.root, layout_epoch: 0 }, 1, SURFACE_RECONCILE_PAGE_BYTES).unwrap().progressed);
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

    pub fn install_fixture_record(&mut self, record: ui_contract::UiNodeRecord) {
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

//#endregion 🔖️Reconciler

//#region ⏭️ResumableReconcile

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SurfaceReconcileStage {
    TraversePresentation,
    AllocateIdentities,
    DiffRecords,
    RemoveStale,
    Finalize,
}

#[derive(Debug)]
#[expect(clippy::large_enum_variant, reason = "Completion transfers the admitted shadow and patch by value without allocating another owner.")]
pub enum SurfaceReconcileStep {
    Yield { nodes: usize, bytes: usize },
    Complete { shadow: SurfaceShadow, patch: Option<ui_contract::UiPatch> },
    Fault(SurfaceReconcileFault),
}

/// ⏭️ Persistent one-node-at-a-time traversal and keyed differ for a single presented surface.
/// The retained shadow is read-only until `Complete`; abandoning this cursor therefore abandons
/// every candidate identity, record, operation, and revision together.
pub struct SurfaceReconcileCursor {
    stage: SurfaceReconcileStage,
    surface: ui_contract::SurfaceId,
    base_revision: ui_contract::UiRevision,
    allocator: ui_contract::UiNodeIdAllocator,
    old_root: Option<ui_contract::UiNodeId>,
    pending_root: Option<TreeNode>,
    native_resources: Option<UiNativeResources>,
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
    held_node: Option<(Option<usize>, TreeNode)>,
    semantic_census: Option<SurfaceSemanticCensusCursor>,
    semantic_usage: SurfaceSemanticUsage,
    fault: Option<SurfaceReconcileFault>,
    retire_tree: SurfaceTreeRetireCursor,
    retire_fresh_field: u8,
    retire_record_field: u8,
    #[cfg(any(test, feature = "testkit"))]
    fail_binding_step: bool,
    #[cfg(any(test, feature = "testkit"))]
    fail_component_step: bool,
}

impl SurfaceReconcileCursor {
    #[cfg(any(test, feature = "testkit"))]
    pub fn new(tree: ComponentTree, current: &SurfaceShadow) -> Self {
        Self::new_with_limits(tree, current, SurfaceReconcileLimits::default())
    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn new_with_limits(tree: ComponentTree, current: &SurfaceShadow, limits: SurfaceReconcileLimits) -> Self {
        let credit = surface_reconcile_limits_admissible(limits).then(|| reserve_surface_reconcile_floor(limits)).flatten();
        Self::new_admitted(tree, current, limits, current.revision.0.checked_add(1).unwrap_or(0), credit)
    }

    pub fn new_admitted(tree: ComponentTree, current: &SurfaceShadow, limits: SurfaceReconcileLimits, generation: u64, credit: Option<ui_contract::UiResidentPermit>) -> Self {
        Self {
            stage: SurfaceReconcileStage::TraversePresentation,
            surface: current.surface.clone(),
            base_revision: current.revision,
            allocator: current.allocator.clone(),
            old_root: current.root,
            pending_root: Some(tree.root),
            native_resources: tree.native_resources,
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
            #[cfg(any(test, feature = "testkit"))]
            fail_binding_step: false,
            #[cfg(any(test, feature = "testkit"))]
            fail_component_step: false,
        }
    }

    pub fn usage(&self) -> SurfaceReconcileUsage {
        self.usage
    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn assembly_generation(&self) -> u64 {
        self.assembly_generation
    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn pending_root(&self) -> Option<&TreeNode> {
        self.pending_root.as_ref()
    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn held_node(&self) -> Option<&(Option<usize>, TreeNode)> {
        self.held_node.as_ref()
    }

    pub fn step(&mut self, current: &SurfaceShadow) -> SurfaceReconcileStep {
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
            #[cfg(any(test, feature = "testkit"))]
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
                            let fault = SurfaceReconcileFault::Credits { usage: SurfaceReconcileUsage { nodes: self.limits.max_nodes.saturating_add(1), ..self.usage }, limits: self.limits };
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
                        let usage = SurfaceReconcileUsage { nodes: self.flat.len().saturating_add(1), items: self.usage.items, bytes: self.usage.bytes };
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
                    if let Err(fault) = self.admit_credit(projected) {
                        return self.fail(fault);
                    }
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
                            let fault = SurfaceReconcileFault::Credits { usage: SurfaceReconcileUsage { nodes: self.limits.max_nodes.saturating_add(1), ..self.usage }, limits: self.limits };
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
                        let fault = SurfaceReconcileFault::Credits { usage: SurfaceReconcileUsage { nodes: self.limits.max_nodes.saturating_add(1), ..self.usage }, limits: self.limits };
                        self.fault = Some(fault.clone());
                        return SurfaceReconcileStep::Fault(fault);
                    }
                    if let Err(frame) = self.traversal.try_push(PresentationFrame { index, children }) {
                        self.overflow_frame = Some(frame);
                        let fault = SurfaceReconcileFault::ValueDepth { actual: SURFACE_RECONCILE_VALUE_DEPTH.saturating_add(1), max: SURFACE_RECONCILE_VALUE_DEPTH };
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
                        let fault = SurfaceReconcileFault::Credits { usage: SurfaceReconcileUsage { nodes: self.limits.max_nodes.saturating_add(1), ..self.usage }, limits: self.limits };
                        self.fault = Some(fault.clone());
                        return SurfaceReconcileStep::Fault(fault);
                    }
                    if self.ids.try_push(id).is_err() {
                        let fault = SurfaceReconcileFault::Credits { usage: SurfaceReconcileUsage { nodes: self.limits.max_nodes.saturating_add(1), ..self.usage }, limits: self.limits };
                        self.fault = Some(fault.clone());
                        return SurfaceReconcileStep::Fault(fault);
                    }
                    if let Some(parent) = parent_index {
                        if self.flat[parent].child_ids.try_push(id).is_err() {
                            let fault = SurfaceReconcileFault::Credits { usage: SurfaceReconcileUsage { nodes: self.limits.max_nodes.saturating_add(1), ..self.usage }, limits: self.limits };
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
                    #[cfg(any(test, feature = "testkit"))]
                    if self.fail_component_step { panic!("injected existing component callback unwind"); }
                    return step;
                }
                if self.record_diff.as_ref().is_some_and(|diff| diff.fresh.is_some() && diff.field == 1) {
                    let step = Self::advance_fresh_component(self.record_diff.as_mut().expect("checked retained record"), &mut self.usage, self.limits, &mut self.fault);
                    #[cfg(any(test, feature = "testkit"))]
                    if self.fail_component_step { panic!("injected component callback unwind"); }
                    return step;
                }
                if self.record_diff.as_ref().is_some_and(|diff| diff.fresh.is_some() && diff.field == 5) {
                    let step = Self::advance_fresh_bindings(self.record_diff.as_mut().expect("checked retained record"), &mut self.usage, self.limits, &mut self.fault);
                    #[cfg(any(test, feature = "testkit"))]
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
                    let node = std::mem::replace(&mut self.flat[index].node, TreeNode::empty_separator());
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
                            let fault = SurfaceReconcileFault::Credits { usage: SurfaceReconcileUsage { nodes: self.limits.max_nodes.saturating_add(1), ..self.usage }, limits: self.limits };
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
                let shadow = SurfaceShadow {
                    surface: self.surface.clone(),
                    revision,
                    allocator: self.allocator.clone(),
                    document: None,
                    native_resources: self.native_resources.take(),
                    native_resource_loans: Default::default(),
                    assembly: take(&mut self.assembly),
                    ordinals: self.new_ordinals.take_all(),
                    key_index: self.new_key_index.take_all(),
                    root: new_root,
                    retire_scalar: 0,
                    seal_phase: 0,
                };
                SurfaceReconcileStep::Complete { shadow, patch }
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
        if let Err(fault) = self.admit_credit(self.usage) {
            return self.fail(fault);
        }
        step
    }

    /// 🎟️ Grows this reconcile's resident credit to cover what it has ACTUALLY used, in page quanta, before
    /// the next step allocates into it — the reservation is priced by the tree being reconciled instead of by
    /// the per-surface ceiling.
    ///
    /// 🐛️ ticket 26/09/02/PUZZLE-3D-END-TO-END wave B58: `SurfaceReconcileReservation::try_new` took
    /// [`SURFACE_RECONCILE_SURFACE_BYTES`] (8 MiB) of a 32 MiB aggregate for EVERY surface, so three
    /// reconciles ran process-wide while one puzzle3d session mounts thirteen and every fourth surface was
    /// refused `registry-resident-credit-exhausted`, deferred and re-dirtied for ever (wave B54 §8.2, wave
    /// B56 §1.1). The credit now starts at [`SURFACE_RECONCILE_FLOOR_BYTES`] — one assembly root plus four
    /// work pages, more than any single step can charge — and climbs with `usage`, so the per-surface
    /// ceiling is reached only by a body that genuinely fills it. An aggregate that cannot take the increase
    /// is a NAMED terminal fault, never a silent defer: a busy ledger is retried on the next step (the floor
    /// headroom covers it), a full one faults.
    fn admit_credit(&mut self, required: SurfaceReconcileUsage) -> Result<(), SurfaceReconcileFault> {
        if !required.fits(self.limits) {
            return Err(SurfaceReconcileFault::Credits { usage: required, limits: self.limits });
        }
        let needed = ui_contract::UiResidentLimits {
            items: required.items.saturating_add(SURFACE_RECONCILE_FLOOR_ITEMS).min(self.limits.max_items),
            bytes: required.bytes.saturating_add(SURFACE_RECONCILE_FLOOR_BYTES).min(self.limits.max_bytes),
        };
        let refusal = || SurfaceReconcileFault::ResidentCredit { required: needed.bytes, aggregate: ui_contract::UiResidentPermit::snapshot().unwrap_or_default().bytes };
        if let Some(credit) = self.assembly_credit.as_mut() {
            let held = credit.limits();
            if needed.items <= held.items && needed.bytes <= held.bytes {
                return Ok(());
            }
            return match credit.try_reprice(ui_contract::UiResidentLimits { items: needed.items.max(held.items), bytes: needed.bytes.max(held.bytes) }) {
                Ok(_) | Err(ui_contract::UiResidentFault::Contended) => Ok(()),
                Err(_) => Err(refusal()),
            };
        }
        if !self.assembly_open {
            return Ok(());
        }
        let held = match self.assembly.resident_limits() {
            Ok(limits) => limits,
            Err(_) => return Ok(()),
        };
        if needed.items <= held.items && needed.bytes <= held.bytes {
            return Ok(());
        }
        match self.assembly.reprice_resident(ui_contract::UiResidentLimits { items: needed.items.max(held.items), bytes: needed.bytes.max(held.bytes) }, 1, SURFACE_RECONCILE_PAGE_BYTES) {
            Ok(_) => Ok(()),
            Err(error) if error.kind == ui_contract::UiDocumentAssemblyErrorKind::Contended => Ok(()),
            Err(_) => Err(refusal()),
        }
    }

    pub fn clear_fault(&mut self) {
        self.fault = None;
    }

    fn fail(&mut self, fault: SurfaceReconcileFault) -> SurfaceReconcileStep {
        self.fault = Some(fault.clone());
        SurfaceReconcileStep::Fault(fault)
    }

    fn advance_existing_component(&mut self, current: &SurfaceShadow) -> SurfaceReconcileStep {
        self.advance_existing_component_with_grant(current, SURFACE_COMPONENT_COPY_WORK_BYTES)
    }

    fn advance_existing_component_with_grant(&mut self, current: &SurfaceShadow, work: usize) -> SurfaceReconcileStep {
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
            match self.assembly.open_with_permit(&mut self.assembly_credit, &mut self.assembly_surface, ui_contract::UiDocumentAssemblyIdentity { generation: self.assembly_generation, revision: self.base_revision, root, layout_epoch: 0 }, 1, SURFACE_RECONCILE_PAGE_BYTES) {
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
            diff.owned_copy = Some(RecordOwnedCopy::Bindings(ui_contract::UiBindingsCopy::new(take(&mut diff.record.bindings))));
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

    pub fn native_resources(&self) -> Option<&UiNativeResources> { self.native_resources.as_ref() }

    pub fn close_native_resources_on_original(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<Option<bool>, semio_framework_job::OriginalTurnFailure> {
        let Some(resources) = self.native_resources.as_mut() else { return Ok(Some(true)); };
        if resources.close_step_on_original(cx)? != Some(true) { return Ok(None); }
        self.native_resources = None;
        Ok(Some(false))
    }

    pub fn retire_one(&mut self) -> bool {
        if self.native_resources.is_some() { return false; }
        if !self.retire_tree.step() {
            return false;
        }
        if self.retire_tree.try_begin_held(&mut self.held_node) {
            self.semantic_census = None;
            return false;
        }
        if self.retire_tree.try_begin_node(&mut self.pending_root) {
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

//#endregion ⏭️ResumableReconcile

//#region 🔖️Diff

#[cfg(any(test, feature = "testkit"))]
struct SurfaceReconcileOracle {
    surface: ui_contract::SurfaceId,
    revision: ui_contract::UiRevision,
    allocator: ui_contract::UiNodeIdAllocator,
    root: Option<ui_contract::UiNodeId>,
    retained: SurfaceLinearMap<ui_contract::UiNodeId, ui_contract::UiNodeRecord, SURFACE_RECONCILE_FIXED_NODES>,
    key_index: SurfaceLinearMap<NodeIdentity, ui_contract::UiNodeId, SURFACE_RECONCILE_FIXED_NODES>,
}

#[cfg(any(test, feature = "testkit"))]
impl SurfaceReconcileOracle {
    fn from_current(current: &SurfaceShadow) -> Self {
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

    fn reconcile(&mut self, tree: &ComponentTree) -> Option<ui_contract::UiPatch> {
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
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
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

//#endregion 🔖️Diff

//#region 🧪️Tests
#[cfg(test)]
#[path = "../🧪️tests/🔬️cursor-unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
