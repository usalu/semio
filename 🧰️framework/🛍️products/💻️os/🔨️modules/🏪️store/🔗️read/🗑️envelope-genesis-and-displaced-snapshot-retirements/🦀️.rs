//! 🗑️ Envelope, genesis and displaced snapshot retirements: the envelope metadata and retirement, the genesis retirement, the displaced snapshot retirements with their capacity and the cursor disposer phase. See [`super`].

use super::*;

pub struct ArtifactEnvelopeMetadata {
    pub schema: String,
    pub id: String,
    pub backbone: Option<ArtifactBackboneRef>,
    pub active_alternative_id: Option<String>,
    pub cursor: Option<ArtifactCursor>,
    pub dialect: Option<semio_framework_artifact_reference::ArtifactDialect>,
    pub migrated_from: Option<MigrationProvenance>,
    pub owner: Option<OwnerRef>,
    pub lanes: protocol::HistoryFoldIndex<String, HistoryLane>,
    pub viewer_checkpoint_id: Option<String>,
    pub conflicts: Vec<semio_framework_os_spr::Conflict>,
    pub transitions: Vec<semio_framework_os_spr::MutationEnvelope>,
    pub open_transaction: Option<OpenToolTransaction>,
}

pub struct ArtifactStoreEnvelopeRetirement<P, Mutation> {
    pub vcs: std::mem::ManuallyDrop<Option<ArtifactStoreVcsRetirement<P, Mutation>>>,
    pub messages: std::mem::ManuallyDrop<Option<ArtifactEditMessageLedgerRetirement>>,
    pub metadata: std::mem::ManuallyDrop<Option<ArtifactEnvelopeMetadata>>,
    pub active: std::mem::ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
}

impl<P, Mutation> ArtifactStoreEnvelopeRetirement<P, Mutation> {
    pub fn new(envelope: ArtifactEnvelope<P, Mutation>, snapshots: Arc<dyn ArtifactOwnedValueRetirementFactory<P>>, mutations: Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>) -> Self {
        let ArtifactEnvelopeOwners { schema, id, vcs, backbone, active_alternative_id, cursor, dialect, migrated_from, owner, lanes, viewer_checkpoint_id, edit_messages, conflicts, transitions, history_shape: _, open_transaction } = envelope.into_owners();
        Self {
            vcs: std::mem::ManuallyDrop::new(Some(ArtifactStoreVcsRetirement::new(vcs, snapshots, mutations))),
            messages: std::mem::ManuallyDrop::new(Some(ArtifactEditMessageLedgerRetirement::new(edit_messages))),
            metadata: std::mem::ManuallyDrop::new(Some(ArtifactEnvelopeMetadata { schema, id, backbone, active_alternative_id, cursor, dialect, migrated_from, owner, lanes, viewer_checkpoint_id, conflicts, transitions, open_transaction })),
            active: std::mem::ManuallyDrop::new(None),
        }
    }
    pub fn ownership_is_empty(&self) -> bool { self.vcs.is_none() && self.messages.is_none() && self.metadata.is_none() && self.active.is_none() }
}

impl<P: Send + Sync + 'static, Mutation: Send + 'static> ArtifactStoreEnvelopeRetirement<P, Mutation> {
    pub fn demands(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        if let Some(active) = self.active.as_ref() { return artifact_retirement_box_demands(active, body); }
        if self.vcs.is_some() { return retirement_owner_demands(&self.vcs, body); }
        if self.messages.is_some() { return retirement_owner_demands(&self.messages, body); }
        artifact_retirement_owned_birth_demands(&self.metadata)
    }
}

impl<P: Send + Sync + 'static, Mutation: Send + 'static> ErasedSnapshotRetirement for ArtifactStoreEnvelopeRetirement<P, Mutation> {
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if self.ownership_is_empty() { return Ok(RetainedCloneStep::Complete(Default::default())); }
        let step = if self.active.is_some() { artifact_retirement_box_close_step(&mut self.active, grant) }
            else if self.vcs.is_some() { retirement_owner_close(&mut self.vcs, grant) }
            else if self.messages.is_some() { retirement_owner_close(&mut self.messages, grant) }
            else { artifact_retirement_admit_owned(&mut self.metadata, &mut self.active, grant) }?;
        Ok(if self.ownership_is_empty() { RetainedCloneStep::Complete(step.progress()) } else { RetainedCloneStep::Progress(step.progress()) })
    }
    fn terminal_is_empty(&self) -> bool { self.ownership_is_empty() }
    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.copy_bytes) }
    fn next_capacity_byte_demand(&self, body: usize) -> Result<usize, ValueError> { Ok(self.demands(body)?.capacity_bytes) }
    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.release_bytes) }
    fn next_depth_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.depth) }
    fn next_demand(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> { self.demands(body) }
}

impl<P, Mutation> Drop for ArtifactStoreEnvelopeRetirement<P, Mutation> {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.ownership_is_empty(), "whole envelope reached Drop before every original VCS, message backing, metadata and factory owner closed");
        if self.ownership_is_empty() { unsafe { std::mem::ManuallyDrop::drop(&mut self.vcs); std::mem::ManuallyDrop::drop(&mut self.messages); std::mem::ManuallyDrop::drop(&mut self.metadata); std::mem::ManuallyDrop::drop(&mut self.active); } }
    }
}

pub struct ArtifactGenesisRetirement<P: Send + Sync + 'static> {
    pub snapshot: std::mem::ManuallyDrop<Option<ReturnedSnapshotReadRetirement<P>>>,
    pub pack: std::mem::ManuallyDrop<Option<semio_framework_value::retirement::shared::SharedControlledRetirement<Vec<u8>>>>,
}

impl<P: Send + Sync + 'static> ArtifactGenesisRetirement<P> {
    pub fn new(genesis: AdmittedArtifactGenesis<P>, factory: Arc<dyn ArtifactOwnedValueRetirementFactory<P>>) -> Self {
        let (snapshot, pack) = genesis.into_owners();
        Self { snapshot: std::mem::ManuallyDrop::new(Some(ReturnedSnapshotReadRetirement::new(snapshot, factory))), pack: std::mem::ManuallyDrop::new(Some(semio_framework_value::retirement::shared::SharedControlledRetirement::lease(pack))) }
    }
    pub fn demands(&self, copy: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        if self.snapshot.is_some() { retirement_owner_demands(&self.snapshot, copy) } else { retirement_owner_demands(&self.pack, copy) }
    }
}

impl<P: Send + Sync + 'static> ErasedSnapshotRetirement for ArtifactGenesisRetirement<P> {
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneStep};
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(Default::default())); }
        let grant = RetainedCloneGrant { maximum_items: grant.maximum_items.min(1), ..grant };
        let step = if self.snapshot.is_some() { retirement_owner_close(&mut self.snapshot, grant)? } else { retirement_owner_close(&mut self.pack, grant)? };
        Ok(if self.terminal_is_empty() { RetainedCloneStep::Complete(step.progress()) } else { RetainedCloneStep::Progress(step.progress()) })
    }
    fn terminal_is_empty(&self) -> bool { self.snapshot.is_none() && self.pack.is_none() }
    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.copy_bytes) }
    fn next_capacity_byte_demand(&self, copy: usize) -> Result<usize, ValueError> { Ok(self.demands(copy)?.capacity_bytes) }
    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.release_bytes) }
    fn next_depth_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.depth) }
    fn next_demand(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> { self.demands(body) }
}

impl<P: Send + Sync + 'static> Drop for ArtifactGenesisRetirement<P> {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.terminal_is_empty(), "genesis retired before both immutable owners became terminal-empty");
        if self.terminal_is_empty() { unsafe { std::mem::ManuallyDrop::drop(&mut self.snapshot); std::mem::ManuallyDrop::drop(&mut self.pack); } }
    }
}

/// 📦️ Retains a displaced shared snapshot with its exact factory until a granted turn constructs its retirement frame.
pub struct ArtifactStoreDisplacedSnapshotRetirement<P: Send + Sync + 'static> {
    pub pending: std::mem::ManuallyDrop<Option<Arc<P>>>,
    pub active: std::mem::ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
    pub factory: std::mem::ManuallyDrop<Option<Arc<dyn SnapshotRetirementFactory<P>>>>,
    pub closing_factory: std::mem::ManuallyDrop<Option<semio_framework_value::FactoryAuthority>>,
}

impl<P: Send + Sync + 'static> ArtifactStoreDisplacedSnapshotRetirement<P> {
    pub fn new(snapshot: Arc<P>, factory: Arc<dyn SnapshotRetirementFactory<P>>) -> Self {
        Self { pending: std::mem::ManuallyDrop::new(Some(snapshot)), active: std::mem::ManuallyDrop::new(None), factory: std::mem::ManuallyDrop::new(Some(factory)), closing_factory: std::mem::ManuallyDrop::new(None) }
    }

    pub fn boxed(snapshot: Arc<P>, factory: Arc<dyn SnapshotRetirementFactory<P>>) -> Box<dyn ErasedSnapshotRetirement> { Box::new(Self::new(snapshot, factory)) }

    pub fn demands(&self, copy: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        use semio_framework_value::RetirementDemand;
        let nested = |mut demand: RetirementDemand| -> Result<RetirementDemand, ValueError> { demand.depth = demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "displaced snapshot depth overflow"))?; Ok(demand) };
        if let Some(snapshot) = self.pending.as_ref() { return Ok(RetirementDemand { capacity_bytes: self.factory.as_ref().ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "displaced snapshot lost its original factory"))?.retirement_birth_bytes(snapshot), depth: 2, ..Default::default() }); }
        if let Some(active) = self.active.as_ref() { return nested(semio_framework_value::factory_ticket_demands(active, copy)?); }
        if self.factory.is_some() { return Ok(RetirementDemand { depth: 1, ..Default::default() }); }
        if let Some(authority) = self.closing_factory.as_ref() { return nested(authority.demands(copy)?); }
        Ok(Default::default())
    }
}

impl<P: Send + Sync + 'static> ErasedSnapshotRetirement for ArtifactStoreDisplacedSnapshotRetirement<P> {
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        use semio_framework_value::retained_clone::admit_retained_clone_close;
        let empty = RetainedCloneProgress::default();
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(empty)); }
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(empty)); }
        let demand = self.demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth < demand.depth { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "displaced snapshot exceeds admitted depth")); }
        if grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes { return Ok(RetainedCloneStep::Progress(empty)); }
        let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant };
        if let Some(snapshot) = self.pending.take() {
            return match self.factory.as_ref().expect("pending displaced snapshot retains its original factory").retire(snapshot, child) {
                Ok((owner, progress)) => {
                    *self.active = Some(owner);
                    if !progress.fits(child) || progress.retained_capacity_bytes != demand.capacity_bytes { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "displaced snapshot constructor changed its admitted receipt")); }
                    Ok(RetainedCloneStep::Progress(progress))
                }
                Err((error, snapshot)) => { *self.pending = Some(snapshot); Err(error) }
            };
        }
        if self.active.is_some() { return semio_framework_value::close_factory_ticket(&mut self.active, child).map(|step| RetainedCloneStep::Progress(step.progress())); }
        if let Some(factory) = self.factory.take() {
            *self.closing_factory = Some(semio_framework_value::FactoryAuthority::new(factory));
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..empty }));
        }
        let authority = self.closing_factory.as_mut().expect("displaced snapshot retains its closing factory");
        let step = authority.step(child)?;
        let step = admit_retained_clone_close(child, step, authority.terminal_is_empty(), "displaced snapshot factory capability")?;
        if authority.terminal_is_empty() { drop(self.closing_factory.take()); }
        Ok(if self.terminal_is_empty() { RetainedCloneStep::Complete(step.progress()) } else { RetainedCloneStep::Progress(step.progress()) })
    }
    fn terminal_is_empty(&self) -> bool { self.pending.is_none() && self.active.is_none() && self.factory.is_none() && self.closing_factory.is_none() }
    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.copy_bytes) }
    fn next_capacity_byte_demand(&self, copy: usize) -> Result<usize, ValueError> { Ok(self.demands(copy)?.capacity_bytes) }
    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.release_bytes) }
    fn next_depth_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.depth) }
    fn next_demand(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> { self.demands(body) }
}

impl<P: Send + Sync + 'static> Drop for ArtifactStoreDisplacedSnapshotRetirement<P> {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.terminal_is_empty(), "displaced snapshot retirement abandoned its original snapshot or factory capability");
        if self.terminal_is_empty() { unsafe { std::mem::ManuallyDrop::drop(&mut self.pending); std::mem::ManuallyDrop::drop(&mut self.active); std::mem::ManuallyDrop::drop(&mut self.factory); std::mem::ManuallyDrop::drop(&mut self.closing_factory); } }
    }
}

pub const ARTIFACT_STORE_DISPLACED_RETIREMENT_CAPACITY: usize = 1_024;

pub const ARTIFACT_STORE_DISPLACED_RESERVATION_CAPACITY: usize = 8;

/// 🌡️ One-item retirement steps a change spends on the displaced queue while it is under pressure (`relieve_displaced_pressure`).
pub const DISPLACED_RELIEF_STEPS: usize = 16;

/// 🌡️ Displaced-owner occupancy at which a store reports maintenance PRESSURE — a quarter of the
/// fixed queue. Every commit displaces its envelope, current snapshot and dag (three owners, more
/// with a tail undo entry) and the round-robin maintenance pump drains ONE owner step per visit, so
/// a live cadence of coalesced commits (a playback tick, a gumball drag step) fills the queue a
/// hundred times faster than the fair rotation empties it. Above this mark the host drains the
/// pressured queue out of turn and in bursts until it is back under.
pub const ARTIFACT_STORE_DISPLACED_PRESSURE_OCCUPANCY: usize = ARTIFACT_STORE_DISPLACED_RETIREMENT_CAPACITY / 4;

pub struct ArtifactStoreDisplacedRetirements {
    pub owners: std::mem::ManuallyDrop<VecDeque<Box<dyn ErasedSnapshotRetirement>>>,
    owner_reservations: [Option<(u64, usize)>; ARTIFACT_STORE_DISPLACED_RESERVATION_CAPACITY],
    owner_generation: u64,
}

impl ArtifactStoreDisplacedRetirements {
    pub fn new() -> Self {
        Self { owners: std::mem::ManuallyDrop::new(VecDeque::with_capacity(ARTIFACT_STORE_DISPLACED_RETIREMENT_CAPACITY)), owner_reservations: [None; ARTIFACT_STORE_DISPLACED_RESERVATION_CAPACITY], owner_generation: 0 }
    }

    pub fn reserve(&self, count: usize) -> Result<(), VcsError> {
        let occupied = self.owners.len().checked_add(self.owner_reservations.iter().flatten().map(|(_, count)| *count).sum::<usize>()).ok_or_else(|| VcsError::ValidationFailed("artifact store displaced-owner occupancy overflowed".into()))?;
        if count > ARTIFACT_STORE_DISPLACED_RETIREMENT_CAPACITY.saturating_sub(occupied) {
            return Err(VcsError::ValidationFailed("artifact store displaced-owner fixed retirement authority is saturated".into()));
        }
        Ok(())
    }

    pub fn reserve_owner_slots(&mut self, count: usize) -> Result<ArtifactStoreDisplacedOwnerReservation, VcsError> {
        if count == 0 {
            return Err(VcsError::ValidationFailed("artifact store displaced-owner reservation is empty".into()));
        }
        let slot = self.owner_reservations.iter().position(Option::is_none).ok_or_else(|| VcsError::ValidationFailed("artifact store displaced-owner reservation registry is saturated".into()))?;
        self.reserve(count)?;
        let generation = self.owner_generation.checked_add(1).ok_or_else(|| VcsError::ValidationFailed("artifact store displaced-owner reservation generation exhausted".into()))?;
        self.owner_generation = generation;
        self.owner_reservations[slot] = Some((generation, count));
        Ok(ArtifactStoreDisplacedOwnerReservation { slot: slot as u8, generation, remaining: count })
    }

    pub fn push_owner_reserved(&mut self, reservation: &mut ArtifactStoreDisplacedOwnerReservation, owner: Box<dyn ErasedSnapshotRetirement>) -> Result<(), Box<dyn ErasedSnapshotRetirement>> {
        let slot = reservation.slot as usize;
        if reservation.remaining == 0 || self.owner_reservations.get(slot).copied().flatten() != Some((reservation.generation, reservation.remaining)) || self.owners.len() >= ARTIFACT_STORE_DISPLACED_RETIREMENT_CAPACITY {
            return Err(owner);
        }
        reservation.remaining -= 1;
        self.owner_reservations[slot] = (reservation.remaining != 0).then_some((reservation.generation, reservation.remaining));
        self.owners.push_back(owner);
        Ok(())
    }

    pub fn release_owner_slots(&mut self, reservation: ArtifactStoreDisplacedOwnerReservation) -> Result<(), VcsError> {
        if reservation.remaining == 0 {
            if self.owner_reservations[reservation.slot as usize].is_some_and(|(generation, _)| generation == reservation.generation) {
                return Err(VcsError::ValidationFailed("artifact store consumed displaced-owner reservation retained live credits".into()));
            }
            return Ok(());
        }
        if self.owner_reservations[reservation.slot as usize] != Some((reservation.generation, reservation.remaining)) {
            return Err(VcsError::ValidationFailed("artifact store displaced-owner reservation is stale or partially consumed".into()));
        }
        self.owner_reservations[reservation.slot as usize] = None;
        Ok(())
    }

    pub fn push_reserved(&mut self, owner: Box<dyn ErasedSnapshotRetirement>) {
        let reserved = self.owner_reservations.iter().flatten().map(|(_, count)| *count).sum::<usize>();
        assert!(self.owners.len() < ARTIFACT_STORE_DISPLACED_RETIREMENT_CAPACITY.saturating_sub(reserved), "artifact store displaced-owner admission token was consumed after its fixed slot changed");
        self.owners.push_back(owner);
    }

    pub fn demands(&self, copy: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        if let Some(owner) = self.owners.front() {
            let mut demand = artifact_retirement_box_demands(owner, copy)?;
            demand.depth = demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "displaced queue retirement depth overflow"))?;
            return Ok(demand);
        }
        if self.owner_reservations.iter().any(Option::is_some) { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit, "displaced queue retains reserved original owner slots")); }
        Ok(Default::default())
    }

    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let empty = RetainedCloneProgress::default();
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(empty)); }
        let demand = self.demands(grant.maximum_copy_bytes)?;
        let Some(owner) = self.owners.front_mut() else { return Ok(RetainedCloneStep::Complete(empty)); };
        if grant.maximum_depth < demand.depth { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "displaced queue exceeds admitted original child depth")); }
        if grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes || (demand.copy_bytes != 0 && grant.maximum_copy_bytes == 0) { return Ok(RetainedCloneStep::Progress(empty)); }
        let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant };
        if owner.terminal_is_empty() {
            let bytes = size_of_val(owner.as_ref());
            drop(self.owners.pop_front());
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: bytes, ..empty }));
        }
        let step = owner.close_step(child)?;
        let step = semio_framework_value::retained_clone::admit_retained_clone_close(child, step, owner.terminal_is_empty(), "original displaced queue child")?;
        if matches!(step, RetainedCloneStep::Complete(_)) && !owner.terminal_is_empty() { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "displaced child reported completion while original ownership remains")); }
        Ok(RetainedCloneStep::Progress(step.progress()))
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.owners.is_empty() && self.owner_reservations.iter().all(Option::is_none)
    }

    pub fn backing_release_byte_demand(&self) -> Result<usize, ValueError> {
        if !self.terminal_is_empty() { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "displaced backing remains occupied by original owners or reservations")); }
        std::alloc::Layout::array::<Box<dyn ErasedSnapshotRetirement>>(self.owners.capacity()).map(|layout| layout.size()).map_err(|_| ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "displaced original queue backing extent overflow"))
    }

    pub fn release_empty_backing_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let empty = RetainedCloneProgress::default();
        let bytes = self.backing_release_byte_demand()?;
        if bytes == 0 { return Ok(RetainedCloneStep::Complete(empty)); }
        if grant.maximum_items == 0 || grant.maximum_release_bytes < bytes { return Ok(RetainedCloneStep::Progress(empty)); }
        if grant.maximum_depth == 0 { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "displaced backing requires original frame depth")); }
        drop(std::mem::take(&mut *self.owners));
        Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, released_bytes: bytes, ..empty }))
    }

    /// 🔭️ Why this queue is not terminal — a queued owner the closer is draining, or a RESERVED
    /// slot no owner ever filled. The two answer `Blocked` from the same arm and need different
    /// cures, so the census names them apart.
    pub fn witness(&self) -> String {
        let reserved = self.owner_reservations.iter().flatten().map(|(_, count)| *count).sum::<usize>();
        format!("owners={}/reserved={}", self.owners.len(), reserved)
    }

    pub fn under_pressure(&self) -> bool {
        self.owners.len() >= ARTIFACT_STORE_DISPLACED_PRESSURE_OCCUPANCY
    }
}

impl Drop for ArtifactStoreDisplacedRetirements {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || (self.terminal_is_empty()), "artifact store displaced-owner authority reached Drop before bounded retirement completed");
        let owners = unsafe { std::mem::ManuallyDrop::take(&mut self.owners) };
        drop(owners);
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArtifactStoreCursorDisposerPhase {
    ReturnedReads,
    Displaced,
    RuntimeOwners(u8),
    PrefixSnapshots,
    TailSnapshot,
    CurrentSnapshot,
    Backbone,
    Causal,
    Envelope,
    Complete,
}
