//! 🪞️ Surface shadow and record read: the `SurfaceShadow` document mirror with its reconcile, snapshot, transaction reader, seal, rejection and retirement, and the borrowed `SurfaceRecordRead`. See [`super`].

use super::*;

//#region 🔖️Reconciler
/// ♻️ The retained shadow of one render surface — the component a keyed reconciliation diffs against. Owns a shadow
/// copy of what the receiver has (`document`, `root`) plus the `(parent, key) → id` index (`key_index`) that carries
/// every node's identity across frames, and the monotonic `allocator` that mints an id for a node the first time it
/// is ever seen. A completed reconcile is the only place any of these change together. The authority crate composes
/// it with the handback reservation it owes the process-wide registry.
#[derive(Debug)]
pub struct SurfaceShadow {
    pub(super) surface: ui_contract::SurfaceId,
    pub(super) revision: ui_contract::UiRevision,
    pub(super) allocator: ui_contract::UiNodeIdAllocator,
    pub(super) document: Option<ui_contract::UiDocumentLease>,
    pub(super) native_resources: Option<UiNativeResources>,
    pub(super) native_resource_loans: UiNativeResourceLoans,
    pub(super) assembly: ui_contract::UiDocumentAssembly,
    pub(super) ordinals: SurfaceLinearMap<ui_contract::UiNodeId, usize, SURFACE_RECONCILE_FIXED_NODES>,
    pub(super) key_index: SurfaceLinearMap<NodeIdentity, ui_contract::UiNodeId, SURFACE_RECONCILE_FIXED_NODES>,
    pub(super) root: Option<ui_contract::UiNodeId>,
    pub(super) retire_scalar: u8,
    pub(super) seal_phase: u8,
}

impl SurfaceShadow {
    /// 🌱️ A reconciler for `surface` with no retained state yet — the next [`Self::reconcile`] call
    /// necessarily emits a full `SetRoot` plus one `Upsert` per node, exactly as [`Self::mark_rejected`]
    /// arranges for an existing reconciler to do again.
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
    pub(super) read: ui_contract::UiDocumentRead<'a>,
    pub(super) ordinal: usize,
    pub(super) id: ui_contract::UiNodeId,
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
