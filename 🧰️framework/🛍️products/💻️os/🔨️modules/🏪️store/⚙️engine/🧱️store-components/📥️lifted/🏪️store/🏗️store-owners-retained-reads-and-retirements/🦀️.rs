//! 🏗️ Store owners retained reads and retirements: the retained derived reads, the snapshot alias and read lease retirement, the pruned conflicts and the factory alias check of the store owners. See [`super`].

use super::*;

//#region 🔖️EphemeralLanes
//#region 🔖️Backbone
//#region 🔖️Backbone
//#region 🔖️Authority
//#region 🔖️EphemeralLanes
//#region 🔖️Text
//#region 🔖️Pack
//#region 🔖️CodecRegistry
//#region 🔖️TextFormat
//#region 🔖️History
//#region 🔖️ArtifactStore
//#region 🔖️ArtifactStore
impl<P, Mutation> StoreOwners<P, Mutation>
where
    P: ToValue + Send + Sync + 'static,
    Mutation: ToValue + FromValue + super::Mutation<P> + OpBinary + OpText + Send + 'static,
{
    /// 🧳️ Retains an original projection from this exact issuer, including an older captured frontier, for readonly custody.
    

    /// 🧰️ Prepares the exact history retirement frame before the caller commits its original Session.
    pub fn prepare_history_read_retirement(&self,grant:RetainedCloneGrant)->Result<Option<(PreparedHistoryReadRetirement<P,Mutation>,RetainedCloneProgress)>,ValueError>
    where P:Sync {
        let snapshots=self.initial_snapshot_retirement_factory.as_ref().ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"history preparation requires its exact snapshot issuer"))?;
        let mutations=self.mutation_retirement_factory.as_ref().ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"history preparation requires its exact mutation issuer"))?;
        let Some((mut prepared,receipt))=history_read_admission::prepare_history_read_retirement(snapshots,mutations,grant)?else{return Ok(None)};
        prepared.bind_registry(self.snapshot_read_leases.identity());Ok(Some((prepared,receipt)))
    }

    pub fn admit_history_owner_retirement<T>(&self,owner:&mut Option<T>,grant:RetainedCloneGrant,install:fn(&mut ArtifactHistoryReadRetirement<P,Mutation>,T))->Result<Option<(Box<dyn ErasedSnapshotRetirement>,RetainedCloneProgress)>,ValueError>
    where P:Sync {
        if owner.is_none(){return Ok(None);}
        let snapshots=self.initial_snapshot_retirement_factory.as_ref().ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"history cancellation requires its exact snapshot issuer"))?;
        let mutations=self.mutation_retirement_factory.as_ref().ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"history cancellation requires its exact mutation issuer"))?;
        history_read_admission::admit_history_read_retirement(owner,snapshots,mutations,grant,install)
    }

    /// 🛑️ Transfers a cancelled preview's exact owners to bounded cold retirement, including stale frontiers.
    

    /// 🛑️ Transfers a cancelled replay and its planner to bounded cold retirement without publishing any fold.
    

    /// 🧹️ Transfers a discarded completed history review to the same bounded registered owner retirement.
    pub fn retire_finished_history_replay(&self, owner: &mut Option<EditReplayResult<P, Mutation>>,grant:RetainedCloneGrant) -> Result<Option<(Box<dyn ErasedSnapshotRetirement>,RetainedCloneProgress)>, ValueError>
    where P: Sync {
        self.admit_history_owner_retirement(owner,grant,|retirement,finished|*retirement.finished=Some(finished))
    }

    /// 🪪️ Borrows the original installed snapshot issuer without creating a lease or frame.
    pub fn owned_snapshot_retirement_factory(&self)->Result<&Arc<dyn ArtifactOwnedValueRetirementFactory<P>>,ValueError>{
        self.initial_snapshot_retirement_factory.as_ref().ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"derived snapshot retirement requires its original installed issuer"))
    }

    /// 🪪️ Borrows the installed issuer without cloning or transferring original ownership.
    pub fn owned_mutation_retirement_factory(&self)->Result<&Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>,ValueError>{
        self.mutation_retirement_factory.as_ref().ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"owned mutation retirement requires its installed exact issuer"))
    }

    /// 📏️ Quotes the original-value frame while requiring its installed mutation issuer.
    pub fn owned_mutation_retirement_birth_bytes(&self)->Result<usize,ValueError>{
        self.owned_mutation_retirement_factory()?;
        Ok(semio_framework_value::FactoryOwnedRetirement::<Mutation>::constructor_capacity_bytes())
    }

    /// ♻️ Preserves the exact caller mutation until its installed issuer admits every frame currency.
    pub fn retire_owned_mutation(&self,owner:&mut Option<Mutation>,grant:RetainedCloneGrant)->Result<Option<(Box<dyn ErasedSnapshotRetirement>,RetainedCloneProgress)>,ValueError>{
        if owner.is_none(){return Ok(None);}
        let factory=self.owned_mutation_retirement_factory()?;
        semio_framework_value::FactoryOwnedRetirement::<Mutation>::admit_original(owner,factory,grant).map(|admitted|admitted.map(|(frame,progress)|(frame as Box<dyn ErasedSnapshotRetirement>,progress)))
    }

    /// 🔐️ Installs the one domain-supplied owner catalog before a store can enter
    /// retained replacement or close. There is no default catalog and a second installation faults.
    /// The catalog also carries the canonical operation wire of its mutation vocabulary (a catalog cannot be built
    /// for a mutation without a canonical tree) and hands it to the store unless the store already holds a richer one,
    /// so a store that can record history (which needs a catalog) can always author.
    pub fn install_document_store_owners_exact(&mut self, store_log: &mut StoreLog<P, Mutation>, mut owners: DocumentStoreOwners<P, Mutation>) -> Result<(), (ValueError, DocumentStoreOwners<P, Mutation>)> {
        if !owners.constructor_is_complete() { return Err((ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"installed original catalog requires every admitted factory ticket before transfer"),owners)); }
        if self.snapshot_retirement_factory.is_some() || self.initial_snapshot_retirement_factory.is_some() || self.mutation_retirement_factory.is_some() || self.owned_disposer.is_some() || self.owned_disposer_terminal { return Err((ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"store already retains its original owner catalog"),owners)); }
        *self.factory_retirement_tickets = owners.take_factory_retirement_tickets();
        *self.snapshot_retirement_factory = owners.snapshot_retirement.take();
        *self.initial_snapshot_retirement_factory = owners.initial_snapshot_retirement.take();
        *self.mutation_retirement_factory = owners.mutation_retirement.take();
        *self.owned_disposer = owners.store_disposer.take();
        *self.one_item_preparation_factory = owners.one_item_preparation.take();
        *self.replay_preparation_factory = owners.replay_preparation.take();
        *self.one_item_wire_preparation_factory = owners.one_item_wire_preparation.take();
        store_log.operation_wire.get_or_insert(owners.operation_wire);
        Ok(())
    }

    pub fn owned_disposer_installed(&self) -> bool {
        self.owned_disposer.is_some() || self.owned_disposer_terminal
    }

    fn displaced_envelope_retirement(&self, envelope: ArtifactEnvelope<P, Mutation>) -> Result<Box<dyn ErasedSnapshotRetirement>, VcsError> {
        let initial_snapshot_factory = (*self.initial_snapshot_retirement_factory).clone().ok_or_else(|| VcsError::ValidationFailed("artifact store envelope replacement requires its exact initial-snapshot retirement factory".into()))?;
        let mutation_factory = (*self.mutation_retirement_factory).clone().ok_or_else(|| VcsError::ValidationFailed("artifact store envelope replacement requires its exact mutation retirement factory".into()))?;
        Ok(Box::new(ArtifactStoreEnvelopeRetirement::new(envelope, initial_snapshot_factory, mutation_factory)))
    }

    fn replace_envelope_retained(&mut self, store_log: &mut StoreLog<P, Mutation>, next: ArtifactEnvelope<P, Mutation>) -> Result<(), VcsError> {
        self.displaced_retirements.reserve(1)?;
        let initial_snapshot_factory = (*self.initial_snapshot_retirement_factory).clone().ok_or_else(|| VcsError::ValidationFailed("artifact store envelope replacement requires its exact initial-snapshot retirement factory".into()))?;
        let mutation_factory = (*self.mutation_retirement_factory).clone().ok_or_else(|| VcsError::ValidationFailed("artifact store envelope replacement requires its exact mutation retirement factory".into()))?;
        let previous = std::mem::replace(&mut *store_log.envelope, next);
        self.displaced_retirements.push_reserved(Box::new(ArtifactStoreEnvelopeRetirement::new(previous, initial_snapshot_factory, mutation_factory)));
        Ok(())
    }

    pub fn replace_causal_dag_retained(&mut self, store_log: &mut StoreLog<P, Mutation>, next: semio_framework_os_spr::MutationDag) -> Result<(), VcsError> {
        self.displaced_retirements.reserve(1)?;
        let previous = std::mem::replace(&mut *store_log.dag, next);
        self.displaced_retirements.push_reserved(Box::new(ArtifactStoreMutationDagRetirement::new(previous)));
        Ok(())
    }

    pub fn reserve_edit_history_slot(&mut self, store_log: &mut StoreLog<P, Mutation>) -> Result<ArtifactStoreHistoryCommitReservation, VcsError> {
        if self.mutation_retirement_factory.is_none() {
            return Err(VcsError::ValidationFailed("edit history insertion requires its exact mutation retirement factory".into()));
        }
        let rejected_owner = self.displaced_retirements.reserve_owner_slots(1)?;
        match store_log.envelope.vcs.edits.reserve_one() {
            Ok(history) => Ok(ArtifactStoreHistoryCommitReservation { history, rejected_owner }),
            Err(_) => {
                self.displaced_retirements.release_owner_slots(rejected_owner)?;
                Err(VcsError::HistoryFull { capacity: store_log.envelope.vcs.edits.len() })
            }
        }
    }

    pub(super) fn reserve_change_history_slot(&mut self, store_log: &mut StoreLog<P, Mutation>) -> Result<ArtifactStoreHistoryCommitReservation, VcsError> {
        let rejected_owner = self.displaced_retirements.reserve_owner_slots(1)?;
        match store_log.envelope.vcs.changes.reserve_one() {
            Ok(history) => Ok(ArtifactStoreHistoryCommitReservation { history, rejected_owner }),
            Err(_) => {
                self.displaced_retirements.release_owner_slots(rejected_owner)?;
                Err(VcsError::ValidationFailed("change history ledger is saturated".into()))
            }
        }
    }

    pub(super) fn insert_reserved_change_history(&mut self, store_log: &mut StoreLog<P, Mutation>, mut reservation: ArtifactStoreHistoryCommitReservation, change: Change) -> Result<ArtifactHistoryKey, VcsError> {
        match store_log.envelope.vcs.changes.insert_reserved(reservation.history, change) {
            Ok(key) => {
                self.displaced_retirements.release_owner_slots(reservation.rejected_owner)?;
                Ok(key)
            }
            Err((_, change)) => {
                let owner = Box::new(ArtifactStoreHistoryMetadataRetirement::change(change));
                if let Err(owner) = self.displaced_retirements.push_owner_reserved(&mut reservation.rejected_owner, owner) {
                    self.displaced_retirements.push_reserved(owner);
                }
                Err(VcsError::ValidationFailed("change history reservation is stale".into()))
            }
        }
    }

    pub(super) fn reserve_checkpoint_history_slot(&mut self, store_log: &mut StoreLog<P, Mutation>) -> Result<ArtifactStoreHistoryCommitReservation, VcsError> {
        let rejected_owner = self.displaced_retirements.reserve_owner_slots(1)?;
        match store_log.envelope.vcs.checkpoints.reserve_one() {
            Ok(history) => Ok(ArtifactStoreHistoryCommitReservation { history, rejected_owner }),
            Err(_) => {
                self.displaced_retirements.release_owner_slots(rejected_owner)?;
                Err(VcsError::ValidationFailed("checkpoint history ledger is saturated".into()))
            }
        }
    }

    pub(super) fn insert_reserved_checkpoint_history(&mut self, store_log: &mut StoreLog<P, Mutation>, mut reservation: ArtifactStoreHistoryCommitReservation, checkpoint: Checkpoint) -> Result<ArtifactHistoryKey, VcsError> {
        match store_log.envelope.vcs.checkpoints.insert_reserved(reservation.history, checkpoint) {
            Ok(key) => {
                self.displaced_retirements.release_owner_slots(reservation.rejected_owner)?;
                Ok(key)
            }
            Err((_, checkpoint)) => {
                let owner = Box::new(ArtifactStoreHistoryMetadataRetirement::checkpoint(checkpoint));
                if let Err(owner) = self.displaced_retirements.push_owner_reserved(&mut reservation.rejected_owner, owner) {
                    self.displaced_retirements.push_reserved(owner);
                }
                Err(VcsError::ValidationFailed("checkpoint history reservation is stale".into()))
            }
        }
    }

    pub(super) fn reserve_alternative_history_slot(&mut self, store_log: &mut StoreLog<P, Mutation>) -> Result<ArtifactStoreHistoryCommitReservation, VcsError> {
        let rejected_owner = self.displaced_retirements.reserve_owner_slots(1)?;
        match store_log.envelope.vcs.alternatives.reserve_one() {
            Ok(history) => Ok(ArtifactStoreHistoryCommitReservation { history, rejected_owner }),
            Err(_) => {
                self.displaced_retirements.release_owner_slots(rejected_owner)?;
                Err(VcsError::ValidationFailed("alternative history ledger is saturated".into()))
            }
        }
    }

    pub(super) fn insert_reserved_alternative_history(&mut self, store_log: &mut StoreLog<P, Mutation>, mut reservation: ArtifactStoreHistoryCommitReservation, alternative: Alternative) -> Result<ArtifactHistoryKey, VcsError> {
        match store_log.envelope.vcs.alternatives.insert_reserved(reservation.history, alternative) {
            Ok(key) => {
                self.displaced_retirements.release_owner_slots(reservation.rejected_owner)?;
                Ok(key)
            }
            Err((_, alternative)) => {
                let owner = Box::new(ArtifactStoreHistoryMetadataRetirement::alternative(alternative));
                if let Err(owner) = self.displaced_retirements.push_owner_reserved(&mut reservation.rejected_owner, owner) {
                    self.displaced_retirements.push_reserved(owner);
                }
                Err(VcsError::ValidationFailed("alternative history reservation is stale".into()))
            }
        }
    }

    pub fn maintenance_retirements_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        self.displaced_retirements.close_step(grant)
    }

    /// 🚚️ Transfers an interrupted publication to this Store's bounded maintenance queue before
    /// its driver leaves scope. The queue retains the exact preparation and clone cursor owners;
    /// subsequent maintenance turns cancel unpublished work and retire every owner under grants.
    pub fn handoff_batch_publication_close(&mut self, publication: &mut Option<ArtifactStoreBatchPublication<P, Mutation>>) -> Result<bool, VcsError>
    where
        P: Sync,
    {
        if publication.is_none() {
            return Ok(false);
        }
        self.displaced_retirements.reserve(1)?;
        let mut owner = publication.take().expect("validated publication close handoff remains present");
        owner.begin_close();
        self.displaced_retirements.push_reserved(Box::new(owner));
        Ok(true)
    }

    pub fn maintenance_retirements_terminal_is_empty(&self) -> bool {
        self.displaced_retirements.terminal_is_empty()
    }

    /// 📏️ Borrows the queued original owner's next payload or terminal frame release.
    pub fn maintenance_retirements_demands(&self, copy: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> { self.displaced_retirements.demands(copy) }

    /// 🌡️ True while the displaced-owner queue holds at least [`ARTIFACT_STORE_DISPLACED_PRESSURE_OCCUPANCY`] owners.
    pub fn maintenance_retirements_under_pressure(&self) -> bool {
        self.displaced_retirements.under_pressure()
    }

    /// 🔭️ Which cursor phase the installed store disposer is draining right now.
    pub fn close_owned_phase_witness(&self) -> String {
        self.owned_disposer.as_ref().map_or_else(|| "uninstalled".to_string(), |disposer| disposer.close_phase_witness())
    }

    pub fn close_one_factory_alias(&mut self) {
        if self.snapshot_retirement_factory.is_some() { drop(self.snapshot_retirement_factory.take()); }
        else if self.initial_snapshot_retirement_factory.is_some() { drop(self.initial_snapshot_retirement_factory.take()); }
        else if self.mutation_retirement_factory.is_some() { drop(self.mutation_retirement_factory.take()); }
        else if self.one_item_preparation_factory.is_some() { drop(self.one_item_preparation_factory.take()); }
        else if self.replay_preparation_factory.is_some() { drop(self.replay_preparation_factory.take()); }
        else if self.one_item_wire_preparation_factory.is_some() { drop(self.one_item_wire_preparation_factory.take()); }
    }

    pub fn retire_snapshot_alias(&self,alias:&mut Option<Arc<P>>,grant:RetainedCloneGrant)->Result<Option<(Box<dyn ErasedSnapshotRetirement>,RetainedCloneProgress)>,ValueError>
    where P:Sync {
        if alias.is_none(){return Ok(None);}
        let factory=self.initial_snapshot_retirement_factory.as_ref().ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"snapshot alias retirement requires its installed exact issuer"))?;
        ReturnedSnapshotReadRetirement::admit_original(alias,factory,grant)
    }

    /// 📏️ Quotes the frame birth of retiring one captured snapshot alias.
    pub fn snapshot_read_retirement_demand(&self) -> semio_framework_value::RetirementDemand {
        semio_framework_value::RetirementDemand { capacity_bytes: ReturnedSnapshotReadRetirement::<P>::constructor_capacity_bytes(), depth: 2, ..Default::default() }
    }

    /// 📏️ Quotes the frame birth of retiring the next returned original root, nothing when none returned.
    pub fn returned_snapshot_read_retirement_demand(&self) -> Result<semio_framework_value::RetirementDemand, ValueError>
    where
        P: Sync,
    {
        if self.snapshot_read_leases.closed() || !self.snapshot_read_leases.has_returned() { return Ok(Default::default()); }
        self.snapshot_read_leases.returned_admission_demands::<P>(|_| self.snapshot_read_retirement_demand()).map_err(SnapshotReadLeaseRefusal::into_value_error)
    }

    /// 🎟️ Admits one returned original root under the caller's complete physical grant.
    pub fn take_returned_snapshot_read_retirement(&mut self, grant: RetainedCloneGrant) -> Result<(Option<Box<dyn ErasedSnapshotRetirement>>, RetainedCloneProgress), ValueError>
    where
        P: Sync,
    {
        if !self.snapshot_read_leases.has_returned() { return Ok((None, Default::default())); }
        let factory = self.initial_snapshot_retirement_factory.as_ref().ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "returned snapshot read requires its exact installed issuer"))?;
        match self.snapshot_read_leases.try_admit_one_returned::<P, _>(grant, |original, child| {
            match ReturnedSnapshotReadRetirement::admit(original, Arc::clone(factory), child) {
                Ok(admitted) => Ok(admitted),
                Err((error, original, capability)) => { drop(capability); Err((error, original)) }
            }
        }) {
            Ok(admitted) => Ok(admitted),
            Err(SnapshotReadLeaseRefusal::Busy) => Ok((None, Default::default())),
            Err(refusal) => Err(refusal.into_value_error()),
        }
    }

    pub fn snapshot_read_leases_terminal_is_empty(&self) -> bool {
        self.snapshot_read_leases.terminal_is_empty()
    }

    /// 🧾️ Leases returned to the registry whose roots it still holds until a returned-read pump reclaims them —
    /// every one of them keeps a whole document root alive.
    pub fn returned_snapshot_read_count(&self) -> usize {
        self.snapshot_read_leases.returned_count()
    }

    /// 🔬️ Leases still OUT — issued and not yet returned. A close cursor in its `ReturnedReads`
    /// phase answers `Blocked` on exactly this: it has nothing returned to retire and the registry
    /// is not terminal, so it waits for a holder that only the holder can release. `Blocked` is
    /// therefore permanent for the closer, never transient, and a caller that retries it spins for
    /// ever. This names the count so a refusal can say whether a live reader or an unreclaimed
    /// returned lease is the owner.
    pub fn outstanding_snapshot_read_count(&self) -> usize {
        self.snapshot_read_leases.occupied_count().saturating_sub(self.returned_snapshot_read_count())
    }

    pub const RESOLVED_CONFLICT_CAP: usize = 512;

    /// 🎯️ MEDIUM-3: oldest-first eviction of RESOLVED (`Accepted`/`Discarded`) conflicts once the
    /// total exceeds [`Self::RESOLVED_CONFLICT_CAP`] — `Open` conflicts are never touched. See the
    /// const's own doc for the reasoning.
    pub fn prune_resolved_conflicts(&mut self, store_log: &mut StoreLog<P, Mutation>) -> Result<(), VcsError> {
        while store_log.envelope.conflicts.len() > Self::RESOLVED_CONFLICT_CAP {
            let Some(index) = store_log.envelope.conflicts.iter().position(|conflict| conflict.status != semio_framework_os_spr::ConflictStatus::Open) else { break };
            self.displaced_retirements.reserve(1)?;
            let conflict = store_log.envelope.conflicts.remove(index);
            self.displaced_retirements.push_reserved(Box::new(ArtifactStoreConflictRetirement::new(conflict)));
        }
        Ok(())
    }

}
//#endregion 🔖️ArtifactStore
//#endregion 🔖️ArtifactStore
//#endregion 🔖️History
//#endregion 🔖️TextFormat
//#endregion 🔖️CodecRegistry
//#endregion 🔖️Pack
//#endregion 🔖️Text
//#endregion 🔖️EphemeralLanes
//#endregion 🔖️Authority

//#region 🔖️Authority
//#region 🔖️EphemeralLanes
//#region 🔖️Text
//#region 🔖️Pack
//#region 🔖️CodecRegistry
//#region 🔖️TextFormat
//#region 🔖️History
//#region 🔖️ArtifactStore
//#region 🔖️ArtifactStore
impl<P, Mutation> StoreOwners<P, Mutation>
where
    P: ToValue,
    Mutation: ToValue + super::Mutation<P>,
{
    pub fn factory_aliases_present(&self) -> bool { self.snapshot_retirement_factory.is_some() || self.initial_snapshot_retirement_factory.is_some() || self.mutation_retirement_factory.is_some() || self.one_item_preparation_factory.is_some() || self.replay_preparation_factory.is_some() || self.one_item_wire_preparation_factory.is_some() }
}
//#endregion 🔖️ArtifactStore
//#endregion 🔖️ArtifactStore
//#endregion 🔖️History
//#endregion 🔖️TextFormat
//#endregion 🔖️CodecRegistry
//#endregion 🔖️Pack
//#endregion 🔖️Text
//#endregion 🔖️EphemeralLanes
//#endregion 🔖️Authority
//#endregion 🔖️Backbone
//#endregion 🔖️Backbone
//#endregion 🔖️EphemeralLanes
