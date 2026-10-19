//! 🧯️ Close host, drop and pair merge: the close host, the history finalization and supersede scope, the store drop, the merge by id and the pair merge. See [`super`].

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
impl<P, Mutation> ArtifactStoreCloseHost<P, Mutation> for ArtifactStore<P, Mutation>
where
    P: Clone + ToValue + FromValue + Send + Sync + 'static,
    Mutation: Clone + ToValue + FromValue + super::Mutation<P> + OpBinary + OpText + Send + 'static,
{
    fn close_host_member_inbox_capacity(&self) -> usize { self.wire.member_inbox.capacity() }
    fn close_host_backbone_attached(&self) -> bool { self.wire.backbone.is_some() }
    fn close_host_prefix_ring_is_empty(&self) -> bool { self.live.prefix_ring.is_empty() }
    fn close_host_snapshot_read_leases(&self) -> &SnapshotReadLeaseRegistryOwner { &self.owners.snapshot_read_leases }
    fn close_host_initial_snapshot_retirement_factory(&self) -> Option<&Arc<dyn ArtifactOwnedValueRetirementFactory<P>>> { self.owners.initial_snapshot_retirement_factory.as_ref() }
    fn snapshot_read_leases_terminal_is_empty(&self) -> bool { ArtifactStore::snapshot_read_leases_terminal_is_empty(self) }
    fn maintenance_retirements_terminal_is_empty(&self) -> bool { ArtifactStore::maintenance_retirements_terminal_is_empty(self) }
    fn maintenance_retirements_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> { ArtifactStore::maintenance_retirements_step(self, grant) }
    fn close_take_backbone_retirement(&mut self, grant: RetainedCloneGrant) -> Result<(Option<Box<dyn ErasedSnapshotRetirement>>, RetainedCloneProgress), ValueError> { ArtifactStore::close_take_backbone_retirement(self, grant) }
    fn close_cursor_admit(&mut self, phase: ArtifactStoreCursorDisposerPhase, grant: RetainedCloneGrant) -> Result<(Option<Box<dyn ErasedSnapshotRetirement>>, RetainedCloneProgress), ValueError> { ArtifactStore::close_cursor_admit(self, phase, grant) }
    fn close_cursor_demands(&self, phase: ArtifactStoreCursorDisposerPhase, body: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> { ArtifactStore::close_cursor_demands(self, phase, body) }
    fn owned_roots_terminal_is_empty(&self) -> bool { ArtifactStore::owned_roots_terminal_is_empty(self) }
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
impl<P, Mutation> ArtifactStore<P, Mutation>
where
    P: Clone + PartialEq + ToValue + FromValue + Send + Sync + 'static,
    Mutation: Clone + ToValue + FromValue + super::Mutation<P> + OpBinary + OpText + Send + 'static,
{
    /// 🪞️ Delegates to [`StoreLive::enable_convergence_early_exit`].
    #[inline] pub fn enable_convergence_early_exit(&mut self) { self.live.enable_convergence_early_exit() }
}

/// 🏁️ How a finished replay's drafts are authored: overwrite them in every alternative and checkpoint containing the
/// operations, supersede them within the existing alternative `alternative_id` only (the line the replay was viewed on — the
/// undo or redo of a finalize), or keep the originals and branch a new alternative `name` at the head that supersedes them.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HistoryFinalization {
    Overwrite,
    Scope { alternative_id: String },
    Alternative { name: String },
}

/// ✏️ Where an authored supersession applies: every alternative, one existing alternative, or a new alternative branched at
/// the head.
pub enum SupersedeScope {
    Document,
    Alternative(String),
    NewAlternative(String),
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
impl<P, Mutation> ArtifactStore<P, Mutation>
where
    P: Clone + ToValue + FromValue,
    Mutation: Clone + ToValue + FromValue + super::Mutation<P>,
{
    pub(super) fn resident_backings_terminal_is_empty(&self) -> bool {
        let (report_messages, report_outbound, report_edit_ids) = self.replay.pending_report.resident_backings();
        self.log.dag.backing_is_empty() && self.log.applied_edit_ids.capacity() == 0 && self.log.redo_edit_ids.capacity() == 0 && self.cursor.revision_accumulator.applied.capacity() == 0 && self.cursor.revision_accumulator.redo.capacity() == 0 && self.owners.displaced_retirements.owners.capacity() == 0 && self.live.prefix_ring.capacity() == 0 && self.wire.member_inbox.capacity() == 0 && report_messages.capacity() == 0 && report_outbound.capacity() == 0 && report_edit_ids.is_none_or(|ids| ids.capacity() == 0) && self.owners.snapshot_read_leases.empty_backing_demands().is_ok_and(|demand| demand.release_bytes == 0)
    }
}

impl<P, Mutation> ArtifactStore<P, Mutation>
where
    P: Clone + ToValue + FromValue,
    Mutation: Clone + ToValue + FromValue + super::Mutation<P>,
{
    /// 🪞️ Delegates to [`StoreOwners::factory_aliases_present`].
    #[inline] pub fn factory_aliases_present(&self) -> bool { self.owners.factory_aliases_present() }
}

impl<P, Mutation> Drop for ArtifactStore<P, Mutation>
where
    P: Clone + ToValue + FromValue,
    Mutation: Clone + ToValue + FromValue + super::Mutation<P>,
{
    /// 🪦️ Exact terminal-empty shallow-shell witness, strict on the ordinary path — the store must
    /// have been retired down to an empty shell before it may be dropped.
    ///
    /// 🪲️ `|| std::thread::panicking()` is not a weakening of that witness: a `Drop` that panics
    /// while an earlier panic is already unwinding is a double panic, which Rust escalates to
    /// `panic in a destructor during cleanup` / `thread caused non-unwinding panic. aborting.` —
    /// killing the whole test process and erasing libtest's summary for every other test in the
    /// binary. Without this guard ANY failing assertion that still holds a live `ArtifactStore`
    /// reports as a `SIGABRT` with no counts instead of as one ordinary test failure. See
    /// `📓️fable-mcp-artifact-quick-recursion.md` §"the amplifier".
    fn drop(&mut self) {
        let terminal = self.log.envelope_detached
            && self.live.current_detached
            && self.wire.backbone.is_none()
            && self.log.dag.terminal_is_empty()
            && self.log.applied_edit_ids.is_empty()
            && self.log.redo_edit_ids.is_empty()
            && self.log.current_checkpoint_id.is_none()
            && self.log.local_actor_id.0.is_empty()
            && self.cursor.revision_accumulator.applied.is_empty()
            && self.cursor.revision_accumulator.redo.is_empty()
            && self.cursor.revision_accumulator.mutation_positions.is_empty()
            && self.cursor.revision_accumulator.indexed_edits.is_empty()
            && self.cursor.revision_accumulator.unit_flags.is_empty()
            && self.live.tail_undo_cache.is_none()
            && self.live.supersessions.is_empty()
            && self.live.prefix_ring.is_empty()
            && self.owners.snapshot_read_leases.terminal_is_empty()
            && self.owners.displaced_retirements.terminal_is_empty()
            && self.owners.owned_disposer.is_none()
            && self.owners.owned_disposer_terminal
            && self.resident_backings_terminal_is_empty()
            && self.owners.snapshot_read_leases.closed()
            && !self.factory_aliases_present()
            && self.owners.factory_retirement_tickets.terminal_is_empty()
            && self.replay.pending_report.is_idle()
            && self.wire.durable_group_root.is_none();
        assert!(terminal || std::thread::panicking(), "artifact store reached Drop without its exact terminal-empty shallow-shell witness");
        unsafe {
            drop(std::mem::ManuallyDrop::take(&mut self.wire.backbone));
            drop(std::mem::ManuallyDrop::take(&mut self.log.dag));
            drop(std::mem::ManuallyDrop::take(&mut self.log.applied_edit_ids));
            drop(std::mem::ManuallyDrop::take(&mut self.log.redo_edit_ids));
            drop(std::mem::ManuallyDrop::take(&mut self.log.current_checkpoint_id));
            drop(std::mem::ManuallyDrop::take(&mut self.log.local_actor_id));
            drop(std::mem::ManuallyDrop::take(&mut self.cursor.revision_accumulator));
            drop(std::mem::ManuallyDrop::take(&mut self.live.tail_undo_cache));
            drop(std::mem::ManuallyDrop::take(&mut self.live.supersessions));
            drop(std::mem::ManuallyDrop::take(&mut self.live.prefix_ring));
            drop(std::mem::ManuallyDrop::take(&mut self.owners.snapshot_retirement_factory));
            drop(std::mem::ManuallyDrop::take(&mut self.owners.initial_snapshot_retirement_factory));
            drop(std::mem::ManuallyDrop::take(&mut self.owners.mutation_retirement_factory));
            drop(std::mem::ManuallyDrop::take(&mut self.owners.snapshot_read_leases));
            drop(std::mem::ManuallyDrop::take(&mut self.owners.displaced_retirements));
            drop(std::mem::ManuallyDrop::take(&mut self.owners.owned_disposer));
            drop(std::mem::ManuallyDrop::take(&mut self.owners.one_item_preparation_factory));
            drop(std::mem::ManuallyDrop::take(&mut self.owners.replay_preparation_factory));
            drop(std::mem::ManuallyDrop::take(&mut self.owners.one_item_wire_preparation_factory));
            drop(std::mem::ManuallyDrop::take(&mut self.owners.factory_retirement_tickets));
            drop(std::mem::ManuallyDrop::take(&mut self.replay.pending_report));
            drop(std::mem::ManuallyDrop::take(&mut self.wire.durable_group_root));
        }
    }
}

pub async fn preflight_merge_by_id<T: PartialEq>(local: &[T], remote: &[T], id_of: impl Fn(&T) -> &String) -> Result<(), VcsError> {
    let existing: HashMap<&str, &T> = local.iter().map(|item| (id_of(item).as_str(), item)).collect();
    for item in remote {
        let id = id_of(item).clone();
        match existing.get(id.as_str()) {
            Some(established) if *established == item => {}
            Some(_) => return Err(VcsError::ValidationFailed(format!("remote history conflicts with established record {id}"))),
            None => {}
        }
    }
    Ok(())
}

pub async fn merge_by_id<T: Clone + PartialEq>(local: &mut Vec<T>, remote: Vec<T>, id_of: impl Fn(&T) -> &String) -> Result<(), VcsError> {
    preflight_merge_by_id(local, &remote, &id_of).await?;
    let mut existing: HashSet<String> = local.iter().map(|item| id_of(item).clone()).collect();
    for item in remote {
        if existing.insert(id_of(&item).clone()) {
            local.push(item);
        }
    }
    Ok(())
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
/// 🪒️ The `.spr` byte log without the transitions `refused` retracts ([`transition_retraction_closure`]) — what a sync
/// actor persists after the hub refused them — and the retracted ids.
pub async fn retract_history_transitions_from_spr(spr: &[u8], refused: &[String]) -> Result<(Vec<u8>, Vec<String>), VcsError> {
    let mut log = semio_framework_os_spr::decode_history(spr, &semio_framework_os_spr::DecodeOptions::default()).await.map_err(|error| VcsError::Deserialize(error.to_string()))?;
    let retracted = transition_retraction_closure(&log.transitions, |record| record.id.as_str(), |record| record.dependencies.iter().map(String::as_str).collect(), refused);
    log.transitions.retain(|record| !retracted.contains(&record.id));
    let options = semio_framework_os_spr::EncodeOptions { write_backwards_section: true, ..semio_framework_os_spr::EncodeOptions::default() };
    let bytes = semio_framework_os_spr::encode_history(&log, &options).await.map_err(|error| VcsError::Serialize(error.to_string()))?;
    Ok((bytes, retracted))
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
//#region 🔖️PairMerge
impl<P, Mutation> ArtifactStore<P, Mutation>
where
    P: Clone + ToValue + FromValue + ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + Send + Sync + 'static,
    Mutation: Clone + ToValue + FromValue + super::Mutation<P> + OpBinary + OpText + Send + 'static,
{
    /// 🫸️ [`Self::merge_persisted_history`] over the pair's undecoded `.spr` bytes.
    pub async fn merge_persisted_pair(&mut self, pack: &[u8], spr: &[u8]) -> Result<PairMerge, VcsError> where P: Clone, Mutation: Clone + ColdMutation<P> {
        let history = semio_framework_os_spr::decode_history(spr, &semio_framework_os_spr::DecodeOptions::default()).await.map_err(|error| VcsError::Deserialize(error.to_string()))?;
        self.merge_persisted_history(pack, history).await
    }

    /// 🪭️ Merges the event log of a persisted pair — the genesis `pack` and its decoded `history`, another writer's copy of
    /// THIS document — into this live store. Every operation and history transition the store lacks is ingested exactly like
    /// a remote event ([`Self::ingest_remote`]): this replica keeps its head, its own edits and whatever replay is in flight,
    /// and the pair's viewer head is never read — only a cold load ([`parse_document_pack`]) stands where a pair says. A pair
    /// of another document (id, schema or genesis) is refused before anything is parsed: replacing a document is a load, not
    /// a merge. An ingest refusal stops the merge; what was taken before it stays, as with any remote batch (every ingest is
    /// atomic).
    pub async fn merge_persisted_history(&mut self, pack: &[u8], history: semio_framework_os_spr::HistoryLog) -> Result<PairMerge, VcsError> where P: Clone, Mutation: Clone + ColdMutation<P> {
        self.ensure_durable_group_idle()?;
        if history.doc_id != self.log.envelope.id || history.schema != self.log.envelope.schema || artifact_initial_digest_of_pack(pack) != self.log.initial_digest {
            return Err(VcsError::ValidationFailed(format!("the persisted pair holds document {}, not {}: a merge takes this document's own log only", history.doc_id, self.log.envelope.id)));
        }
        let ParsedDocumentText { envelope: foreign, snapshot } = parse_decoded_document_spr::<P, Mutation>(pack, history, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer).await.map_err(|error| VcsError::Deserialize(error.to_string()))?;
        retire_replayed_projection::<P, Mutation>(snapshot);
        let mut failure = None;
        let mut events = Vec::new();
        for edit in foreign.vcs.edits.iter() {
            match semio_framework_os_spr::mutation_envelopes_from_edit_since::<P, Mutation>(edit, 0, &ArtifactId(foreign.id.clone()), &SchemaId(foreign.schema.clone())) {
                Ok(envelopes) => events.extend(envelopes),
                Err(error) => {
                    failure = Some(VcsError::Serialize(error.to_string()));
                    break;
                }
            }
        }
        events.extend(foreign.transitions.iter().cloned());
        foreign.retire_unadopted();
        if let Some(error) = failure {
            return Err(error);
        }
        let known: HashSet<MutationId> = self.event_log()?.into_iter().map(|event| event.mutation_id).collect();
        let offered: HashSet<MutationId> = events.iter().map(|event| event.mutation_id.clone()).collect();
        let mut merge = PairMerge { merged: 0, ahead: known.iter().filter(|id| !offered.contains(*id)).count() };
        for event in events.into_iter().filter(|event| !known.contains(&event.mutation_id)) {
            if self.ingest_remote(event).await?.accepted {
                merge.merged += 1;
            }
        }
        Ok(merge)
    }
}
//#endregion 🔖️PairMerge
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
