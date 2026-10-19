//! 🌱️ Runtime seed, construction and owned close: the preflight and adoption of the runtime seed, the store construction, the state and checkpoint pins and the owned demands and close steps. See [`super`].

use super::*;
use semio_framework_os_spr::ColdMutation;

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
impl<P, Mutation> ArtifactStore<P, Mutation>
where
    P: Clone + ToValue + FromValue + Send + Sync + 'static,
    Mutation: Clone + ToValue + FromValue + super::Mutation<P> + OpBinary + OpText + Send + 'static,
{
    pub fn preflight_runtime_seed(identities: &[MutationId]) -> Result<(), VcsError> {
        let mut unique = HashSet::new();
        for identity in identities {
            if identity.0.len() > semio_framework_os_spr::causal::MUTATION_DAG_IDENTIFIER_BYTES {
                return Err(VcsError::ValidationFailed("runtime causal seed identity exceeds its exact byte capacity".into()));
            }
            unique.insert(identity.0.as_str());
            if unique.len() > semio_framework_os_spr::causal::MUTATION_DAG_CAPACITY {
                return Err(VcsError::ValidationFailed("runtime causal seed exceeds its exact identity capacity".into()));
            }
        }
        Ok(())
    }

    pub fn adopt_runtime_seed(identities: Vec<MutationId>) -> (semio_framework_os_spr::MutationDag, Vec<String>) {
        let mut dag = semio_framework_os_spr::MutationDag::new();
        let mut rejected = Vec::new();
        for identity in identities {
            match dag.seed_applied(identity) {
                Ok(()) => {}
                Err(owner) if owner.error == semio_framework_os_spr::MutationDagError::Duplicate => rejected.push(owner.mutation_id.0),
                Err(_) => unreachable!("prevalidated causal seed admission cannot fail after owner adoption"),
            }
        }
        (dag, rejected)
    }

    fn seed_runtime_state(envelope: &ArtifactEnvelope<P, Mutation>, replica: u64) -> Result<(semio_framework_os_spr::MutationDag, i32, HybridLogicalTimestamp, Vec<String>), VcsError> {
        let mut identities = Vec::new();
        let mut edit_sequence = 0;
        let mut clock = HybridLogicalTimestamp::new(replica, now_ms());
        for edit in &envelope.vcs.edits {
            edit_sequence = edit_sequence.max(edit.sequence_number);
            identities.push(MutationId(edit.id.clone()));
            identities.extend(semio_framework_os_spr::mutation_ids_for_edit(edit));
            for meta in &edit.mutation_meta {
                clock.merge(&meta.timestamp);
            }
        }
        for transition in &envelope.transitions {
            identities.push(transition.mutation_id.clone());
            clock.merge(&transition.timestamp);
        }
        Self::preflight_runtime_seed(&identities)?;
        let (dag, rejected) = Self::adopt_runtime_seed(identities);
        Ok((dag, edit_sequence, clock, rejected))
    }

    /// 🚫️ A store is always constructed with no backbone attached — the envelope's
    /// `backbone` field is a descriptor of the last attachment, never an instruction to
    /// reconnect. Callers attach explicitly via {@link attach_backbone}/{@link attach_backbone_uri}.
    ///
    /// 🎯️ `applied_edit_ids`/`redo_edit_ids`/`current_checkpoint_id`/the active alternative
    /// and `current` are derived by folding the envelope's event log ([`fold_envelope_history`]) —
    /// the persisted history carries events only, so a save/load cycle restores the exact position
    /// by replaying them. The caller supplies the opened actor explicitly; history authors never
    /// determine the identity of the instance loading that history.
    ///
    /// 🛡️ A REJECTED construction retires the candidate envelope it consumed. The envelope is
    /// a terminal shell whose `Drop` asserts its owners were detached ([`ArtifactEnvelope::into_owners`]),
    /// so letting a `?` drop it turned every malformed-document refusal — a dangling cursor edit id, a
    /// duplicated authoritative edit, an alternative pinned to a checkpoint that was never recorded —
    /// into a process abort instead of the `VcsError` the caller is meant to read. The candidate lives
    /// in a slot {@link construct} empties only on success; whatever is left is detached here.
    pub async fn new<C:ArtifactStorePublication<P>>(envelope: ArtifactEnvelope<P, Mutation>, actor: ActorId) -> Result<Self, VcsError>
    {
        let mut candidate = Some(envelope);
        let constructed = Self::construct::<C>(&mut candidate, actor).await;
        if let Some(rejected) = candidate {
            rejected.retire_unadopted();
        }
        constructed
    }

    pub async fn construct<C:ArtifactStorePublication<P>>(candidate: &mut Option<ArtifactEnvelope<P, Mutation>>, actor: ActorId) -> Result<Self, VcsError>
    {
        let envelope = candidate.as_mut().expect("artifact store construction holds its candidate envelope");
        if actor.0.trim().is_empty() { return Err(VcsError::ValidationFailed("opened actor identity cannot be empty".into())); }
        validate_durable_history(envelope).await?;
        let semio_framework_os_spr::HistoryFold { applied: loaded_applied_edit_ids, redo: loaded_redo_edit_ids, checkpoint: current_checkpoint_id, alternative, supersessions, .. } = fold_envelope_history(envelope)?;
        envelope.active_alternative_id = alternative;
        validate_history_lanes(envelope, &loaded_applied_edit_ids, &loaded_redo_edit_ids).await?;
        let current = if loaded_applied_edit_ids.is_empty() { envelope.vcs.genesis.facts().share_snapshot() } else { Arc::new(Self::fold_history(envelope, &loaded_applied_edit_ids, &supersessions).await?) };
        let initial_digest = envelope.vcs.genesis.facts().digest();
        let catalog = ArtifactStoreInitializationOwnerCatalog::try_new().map_err(|reason| VcsError::ValidationFailed(reason.into()))?;
        let ArtifactStoreInitializationOwnerCatalog { mut applied_edit_ids, mut redo_edit_ids, mut cursor_applied_edit_ids, mut cursor_redo_edit_ids, applied_revision, redo_revision } = catalog;
        cursor_applied_edit_ids.extend(loaded_applied_edit_ids.iter().cloned());
        cursor_redo_edit_ids.extend(loaded_redo_edit_ids.iter().cloned());
        applied_edit_ids.extend(loaded_applied_edit_ids);
        redo_edit_ids.extend(loaded_redo_edit_ids);
        envelope.cursor = Some(ArtifactCursor::new(cursor_applied_edit_ids, cursor_redo_edit_ids, current_checkpoint_id.clone()));
        let identity_digest = CursorRevisionAccumulator::hash_record(b"initial", &[envelope.id.as_bytes(), envelope.schema.as_bytes(), &initial_digest]);
        let mut revision_accumulator = CursorRevisionAccumulator { identity_digest, applied: applied_revision, redo: redo_revision, applied_tail_chains: None, mutation_positions: protocol::HistoryFoldIndex::new(), indexed_edits: protocol::HistoryFoldIndex::new(), unit_flags: protocol::HistoryFoldIndex::new() };
        revision_accumulator.reconcile::<P, Mutation>(&applied_edit_ids, &redo_edit_ids, &envelope.vcs.edits, &supersessions, StackMirrorDirt::ALL, None);
        if !revision_accumulator.unit_flags.is_empty() {
            let units = EditReplay::new(ReplayMode::Report, envelope.vcs.genesis.facts().share_snapshot(), applied_edit_ids.to_vec(), 0, &envelope.schema, supersessions.clone(), &envelope.vcs.edits)?.run(&envelope.vcs.edits)?;
            revision_accumulator.unit_flags.extend(units.unit_flags.iter().map(|(key, flag)| (*key, *flag)));
            units.retire_unscheduled().map_err(|error| VcsError::ValidationFailed(error.into_message()))?;
        }
        let content_revision = revision_accumulator.revision(current_checkpoint_id.as_deref());
        let local_actor_id = actor;
        let (dag, edit_sequence, clock, rejected_seed_identities) = Self::seed_runtime_state(envelope, fresh_replica_clock().actor)?;
        C::publish_native_snapshot().map_err(|error| VcsError::ValidationFailed(error.to_string()))?;
        let mut displaced_retirements = ArtifactStoreDisplacedRetirements::new();
        if !rejected_seed_identities.is_empty() {
            displaced_retirements.push_reserved(Box::new(ArtifactStoreStringVectorRetirement::new(rejected_seed_identities)));
        }
        let envelope = candidate.take().expect("an accepted construction takes its validated envelope exactly once");
        Ok(Self {
            log: StoreLog {
                envelope: std::mem::ManuallyDrop::new(envelope),
                envelope_detached: false,
                dag: std::mem::ManuallyDrop::new(dag),
                applied_edit_ids: std::mem::ManuallyDrop::new(applied_edit_ids),
                redo_edit_ids: std::mem::ManuallyDrop::new(redo_edit_ids),
                edit_sequence,
                current_checkpoint_id: std::mem::ManuallyDrop::new(current_checkpoint_id),
                local_actor_id: std::mem::ManuallyDrop::new(local_actor_id),
                authoring_verb: None,
                merge_policy: semio_framework_os_spr::MergePolicy::default(),
                clock,
                initial_digest,
                operation_wire: None,
            },
            owners: StoreOwners {
                snapshot_retirement_factory: std::mem::ManuallyDrop::new(None),
                initial_snapshot_retirement_factory: std::mem::ManuallyDrop::new(None),
                mutation_retirement_factory: std::mem::ManuallyDrop::new(None),
                snapshot_read_leases: std::mem::ManuallyDrop::new(SnapshotReadLeaseRegistryOwner::new()),
                displaced_retirements: std::mem::ManuallyDrop::new(displaced_retirements),
                owned_disposer: std::mem::ManuallyDrop::new(None),
                owned_disposer_terminal: false,
                one_item_preparation_factory: std::mem::ManuallyDrop::new(None),
                replay_preparation_factory: std::mem::ManuallyDrop::new(None),
                one_item_wire_preparation_factory: std::mem::ManuallyDrop::new(None),
                factory_retirement_tickets: std::mem::ManuallyDrop::new(semio_framework_value::FactoryChildTickets { slots: std::array::from_fn(|_| semio_framework_value::FactoryChildSlot::empty()), prepared: 0 }),
            },
            wire: StoreWire {
                backbone: std::mem::ManuallyDrop::new(None),
                member_inbox: VecDeque::new(),
                durable_group_root: std::mem::ManuallyDrop::new(None),
            },
            cursor: StoreCursor {
                generation: 0,
                last_projection_cause: None,
                content_revision,
                revision_accumulator: std::mem::ManuallyDrop::new(revision_accumulator),
                revision_dirty_from: None,
                mirror_dirt: StackMirrorDirt::CLEAN,
                marker: std::marker::PhantomData,
            },
            live: StoreLive {
                current: std::mem::ManuallyDrop::new(current),
                current_detached: false,
                tail_undo_cache: std::mem::ManuallyDrop::new(None),
                supersessions: std::mem::ManuallyDrop::new(supersessions),
                prefix_ring: std::mem::ManuallyDrop::new(Vec::with_capacity(PREFIX_SNAPSHOT_RING_CAPACITY)),
                prefix_ring_dirty_from: usize::MAX,
                fold_frontier: None,
                tail_step_generation: None,
                snapshot_equality: None,
                marker: std::marker::PhantomData,
            },
            replay: StoreReplay {
                replay_budget: None,
                local_replay_budget: None,
                pending_reprojection: None,
                pending_report: std::mem::ManuallyDrop::new(PendingCommandReport::default()),
            },
        })
    }

    /// 🏗️ Atomically adopts a domain-validated initialization runtime. Every
    /// history/reference/snapshot owner was prepared under the caller's StepContext before this
    /// non-suspending move; no validation or collection traversal occurs at publication.
    pub fn from_initialized_runtime_with_owners(envelope: ArtifactEnvelope<P, Mutation>, runtime: ArtifactStoreInitializationRuntime<P>, generation: u64, owners: DocumentStoreOwners<P, Mutation>) -> Self {
        config_store_from_initialized_runtime_with_owners(envelope, runtime, generation, owners)
    }

    /// ♻️ Sole public reload API: adopts `envelope`'s event log and derives every position from
    /// it — a reload replays history, it never injects a cursor.
    pub async fn reset(&mut self, envelope: ArtifactEnvelope<P, Mutation>) -> Result<CommandReceipt, VcsError> {
        self.ensure_durable_group_idle()?;
        self.set_state(envelope).await?;
        self.cursor.last_projection_cause = Some(ArtifactProjectionCause::Reset);
        Ok(CommandReceipt { edit_ids: self.log.applied_edit_ids.to_vec(), generation: self.generation(), messages: Vec::new(), worst: None })
    }

    /// 💾️ Adopts full store state from `envelope`'s event log — applied edits, redo stack,
    /// checkpoint and alternative are the fold of its edits and transitions, so `Redo` survives
    /// round-tripping through a serialized envelope (e.g. one `dispatch` call per request). A history change still waiting
    /// for its replay belonged to the replaced log and is dropped with it.
    pub(crate) async fn set_state(&mut self, envelope: ArtifactEnvelope<P, Mutation>) -> Result<(), VcsError> where P: Clone, Mutation: ColdMutation<P> {
        let mut candidate = Some(envelope);
        let adopted = self.adopt_state(&mut candidate).await;
        if let Some(rejected) = candidate {
            rejected.retire_unadopted();
        }
        adopted
    }

    /// 🛡️ The reload proper. Like {@link new}, a REFUSED reload leaves its candidate in the
    /// slot so {@link set_state} can retire it: the live store must survive a rejected reset, and a
    /// dropped populated envelope aborts on the terminal-shell witness instead of returning the
    /// `VcsError` the caller reads.
    async fn adopt_state(&mut self, candidate: &mut Option<ArtifactEnvelope<P, Mutation>>) -> Result<(), VcsError> where P: Clone, Mutation: ColdMutation<P> {
        self.ensure_durable_group_idle()?;
        let envelope = candidate.as_mut().expect("a reload holds its candidate envelope");
        validate_durable_history(envelope).await?;
        let semio_framework_os_spr::HistoryFold { applied: applied_edit_ids, redo: redo_edit_ids, checkpoint: current_checkpoint_id, alternative, supersessions, .. } = fold_envelope_history(envelope)?;
        envelope.active_alternative_id = alternative;
        validate_history_lanes(envelope, &applied_edit_ids, &redo_edit_ids).await?;
        let current = Self::fold_history(envelope, &applied_edit_ids, &supersessions).await?;
        let applied_edit_ids = ArtifactStoreInitializationOwnerCatalog::retain_id_capacity(applied_edit_ids);
        let redo_edit_ids = ArtifactStoreInitializationOwnerCatalog::retain_id_capacity(redo_edit_ids);
        let initial_digest = envelope.vcs.genesis.facts().digest();
        let revision_accumulator = CursorRevisionAccumulator::new(envelope, initial_digest);
        let runtime_slots = usize::from(self.wire.backbone.is_some())
            + StoreLive::<P, Mutation>::string_owner_slots(&self.log.current_checkpoint_id)
            + StoreLive::<P, Mutation>::string_vector_owner_slots(&self.log.applied_edit_ids)
            + StoreLive::<P, Mutation>::string_vector_owner_slots(&self.log.redo_edit_ids)
            + StoreCursor::<P, Mutation>::revision_owner_slots(&self.cursor.revision_accumulator)
            + 1;
        let commit_authority = self.prepare_document_root_commit(runtime_slots)?;
        let (dag, edit_sequence, clock, rejected_seed_identities) = Self::seed_runtime_state(envelope, self.log.clock.actor)?;
        let envelope = candidate.take().expect("an adopted reload takes its validated envelope exactly once");
        self.clear_prefix_ring()?;
        self.commit_document_roots_retained(envelope, Arc::new(current), dag, None, commit_authority);
        self.replace_supersessions_retained(supersessions)?;
        if !rejected_seed_identities.is_empty() {
            self.owners.displaced_retirements.push_reserved(Box::new(ArtifactStoreStringVectorRetirement::new(rejected_seed_identities)));
        }
        if let Some(previous) = self.wire.backbone.take() {
            self.owners.displaced_retirements.push_reserved(Box::new(ArtifactStoreBackboneRetirement::new(previous)));
        }
        self.log.edit_sequence = edit_sequence;
        if let Some(owner) = StoreLive::<P, Mutation>::take_string_replacement(&mut self.log.current_checkpoint_id, current_checkpoint_id) {
            self.owners.displaced_retirements.push_reserved(owner);
        }
        self.log.clock = clock;
        self.log.initial_digest = initial_digest;
        if let Some(owner) = StoreLive::<P, Mutation>::take_string_vector_replacement(&mut self.log.applied_edit_ids, applied_edit_ids) {
            self.owners.displaced_retirements.push_reserved(owner);
        }
        if let Some(owner) = StoreLive::<P, Mutation>::take_string_vector_replacement(&mut self.log.redo_edit_ids, redo_edit_ids) {
            self.owners.displaced_retirements.push_reserved(owner);
        }
        self.push_revision_replacement_reserved(revision_accumulator);
        if let Some(pending) = self.replay.pending_reprojection.take() {
            self.retire_pending_replay(pending.replay)?;
        }
        self.cursor.mirror_dirt = StackMirrorDirt::ALL;
        self.live.fold_frontier = None;
        self.bump()?;
        Ok(())
    }

    /// 📌️ Records the composed children a checkpoint was taken against. Not reachable through
    /// an ordinary `Apply` (no mutation can touch checkpoint metadata), so — like `set_owner` — it
    /// needs its own setter. Called by `VcsArtifactApp`'s checkpoint cascade right after the
    /// checkpoint is created, since a pin can only name a child checkpoint that already exists.
    /// Re-derives `content_addressed_checkpoint_id` so the checkpoint's identity actually covers its
    /// pins rather than merely storing them beside it, and records the re-identification as a
    /// `Repin` transition so every replica and every reload derives the same identity.
    #[must_use]
    pub async fn set_checkpoint_composition_pins(&mut self, checkpoint_id: &str, pins: Vec<semio_framework_os_vcs::CompositionPin>) -> Result<ArtifactProjectionInvalidation, VcsError> {
use semio_framework_artifact_reference::io::text::artifact_reference::ArtifactReferenceText as _;

        self.ensure_durable_group_idle()?;
        validate_composition_pins(&pins).await?;
        let target_index = self.log.envelope.vcs.checkpoints.iter().position(|checkpoint| checkpoint.id == checkpoint_id).ok_or_else(|| VcsError::UnknownChange(checkpoint_id.to_string()))?;
        if self.log.envelope.vcs.checkpoints.iter().any(|checkpoint| checkpoint.parent_id.as_deref() == Some(checkpoint_id)) {
            return Err(VcsError::ValidationFailed(format!("cannot repin checkpoint {checkpoint_id} after a descendant exists")));
        }
        let checkpoint = &self.log.envelope.vcs.checkpoints[target_index];
        let previous_id = checkpoint.id.clone();
        let next_id = content_addressed_checkpoint_id(checkpoint.parent_id.as_deref(), &checkpoint.change_ids, &self.log.envelope.vcs.changes, checkpoint.message.as_deref(), &checkpoint.authors, &checkpoint.timestamp, &pins).await;
        if next_id != previous_id && self.log.envelope.vcs.checkpoints.iter().enumerate().any(|(index, checkpoint)| index != target_index && checkpoint.id == next_id) {
            return Err(VcsError::ValidationFailed(format!("rederived checkpoint identity {next_id} collides with established history")));
        }
        let dependencies = self.checkpoint_origin(&previous_id).into_iter().collect();
        let pins = pins.iter().map(|pin| semio_framework_os_spr::TransitionPin { child_uri: pin.child_ref.to_uri(), checkpoint_id: pin.checkpoint_id.clone() }).collect();
        self.replace_pending_report_retained(PendingCommandReport::default())?;
        self.commit_transition(semio_framework_os_spr::HistoryTransition::Repin { checkpoint_id: previous_id, pinned_checkpoint_id: next_id, pins }, dependencies).await?;
        self.flush_outbound().await?;
        self.invalidate_projections(ArtifactProjectionCause::Checkpoint)
    }

    /// 📏️ Publishes the installed disposer's exact retained payload or terminal allocation demand.
    pub fn close_owned_demands(&self, maximum_body_bytes: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        use semio_framework_value::RetirementDemand;
        if self.owners.owned_disposer_terminal { return Ok(Default::default()); }
        let owner = self.owners.owned_disposer.as_ref().ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "artifact store has no installed owner disposer"))?;
        if !owner.terminal_is_empty(self) {
            let mut demand = owner.demands(self, maximum_body_bytes)?;
            demand.depth = demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "store disposer depth overflow"))?;
            return Ok(demand);
        }
        if !self.resident_backings_terminal_is_empty() { return Ok(RetirementDemand { release_bytes: self.next_resident_backing_byte_demand()?, depth: 2, ..Default::default() }); }
        if !self.owners.snapshot_read_leases.closed() { return Ok(RetirementDemand { release_bytes: self.owners.snapshot_read_leases.custody_byte_demand()?, depth: 2, ..Default::default() }); }
        if self.factory_aliases_present() { return Ok(RetirementDemand { depth: 1, ..Default::default() }); }
        if !self.owners.factory_retirement_tickets.terminal_is_empty() {
            let mut demand = self.owners.factory_retirement_tickets.demands(maximum_body_bytes)?;
            demand.depth = demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "store factory ticket depth overflow"))?;
            return Ok(demand);
        }
        Ok(RetirementDemand { release_bytes: size_of_val(owner.as_ref()), depth: 1, ..Default::default() })
    }

    /// 🧹️ Advances the exact store disposer under the caller's retirement grant.
    pub fn close_owned_store_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let empty = RetainedCloneProgress::default();
        if self.owners.owned_disposer_terminal { return Ok(RetainedCloneStep::Complete(empty)); }
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(empty)); }
        let demand = self.close_owned_demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth < demand.depth { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "store close exceeds admitted owner depth")); }
        if grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes { return Ok(RetainedCloneStep::Progress(empty)); }
        let mut disposer = self.owners.owned_disposer.take().ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "artifact store has no owner-supplied bounded disposer"))?;
        let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant };
        let result = (|| {
            if !disposer.terminal_is_empty(self) {
                let step = disposer.close_step(&mut ArtifactStoreCloseView { store: self }, child)?;
                semio_framework_value::retained_clone::admit_retained_clone_close(child, step, disposer.terminal_is_empty(self), "installed store disposer")?;
                return Ok(RetainedCloneStep::Progress(step.progress()));
            }
            if !self.resident_backings_terminal_is_empty() { return self.close_resident_backing_step(child).map(|step| RetainedCloneStep::Progress(step.progress())); }
            if !self.owners.snapshot_read_leases.closed() { return self.owners.snapshot_read_leases.close_custody_step(child).map(|step| RetainedCloneStep::Progress(step.progress())); }
            if self.factory_aliases_present() {
                self.close_one_factory_alias();
                return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..empty }));
            }
            if !self.owners.factory_retirement_tickets.terminal_is_empty() {
                let step = self.owners.factory_retirement_tickets.close_step(child)?;
                semio_framework_value::retained_clone::admit_retained_clone_close(child, step, self.owners.factory_retirement_tickets.terminal_is_empty(), "store factory tickets")?;
                return Ok(RetainedCloneStep::Progress(step.progress()));
            }
            self.owners.owned_disposer_terminal = true;
            Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, released_bytes: size_of_val(disposer.as_ref()), ..empty }))
        })();
        if self.owners.owned_disposer_terminal { drop(disposer); } else { *self.owners.owned_disposer = Some(disposer); }
        result
    }

    /// 🪹️ Confirms both the store disposer and retained snapshot reads have closed.
    pub fn close_owned_store_terminal_is_empty(&self) -> bool {
        self.owners.owned_disposer_terminal && self.resident_backings_terminal_is_empty() && self.owners.snapshot_read_leases.closed() && !self.factory_aliases_present() && self.owners.factory_retirement_tickets.terminal_is_empty()
    }

    fn next_resident_backing_byte_demand(&self) -> Result<usize, ValueError> {
        if !self.log.dag.backing_is_empty() { return Ok(self.log.dag.next_backing_release_byte_demand()); }
        for demand in [self.log.applied_edit_ids.next_empty_page_release_byte_demand(), self.log.redo_edit_ids.next_empty_page_release_byte_demand(), self.cursor.revision_accumulator.applied.next_empty_page_release_byte_demand(), self.cursor.revision_accumulator.redo.next_empty_page_release_byte_demand()] {
            if let Some(bytes) = demand.filter(|bytes| *bytes != 0) { return Ok(bytes); }
        }
        let (report_messages, report_outbound, report_edit_ids) = self.replay.pending_report.resident_backings();
        for bytes in [resident_backing::deque_demand(&self.owners.displaced_retirements.owners), resident_backing::vec_demand(&self.live.prefix_ring), resident_backing::deque_demand(&self.wire.member_inbox), resident_backing::vec_demand(report_messages), resident_backing::vec_demand(report_outbound), report_edit_ids.map_or(0, resident_backing::vec_demand)] {
            if bytes != 0 { return Ok(bytes); }
        }
        Ok(self.owners.snapshot_read_leases.empty_backing_demands()?.release_bytes)
    }

    fn close_resident_backing_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if !self.close_structural_owners_terminal_is_empty() { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "Store resident backing still holds structural payload owners")); }
        let empty = RetainedCloneProgress::default();
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(empty)); }
        if grant.maximum_depth == 0 { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "resident backing requires owner depth")); }
        if !self.log.dag.backing_is_empty() {
            let (copied_items, released_bytes, _) = self.log.dag.close_backing_step(1, grant.maximum_release_bytes);
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items, released_bytes, ..empty }));
        }
        if self.log.applied_edit_ids.capacity() != 0 { return resident_backing::close_history(&mut self.log.applied_edit_ids, grant); }
        if self.log.redo_edit_ids.capacity() != 0 { return resident_backing::close_history(&mut self.log.redo_edit_ids, grant); }
        if self.cursor.revision_accumulator.applied.capacity() != 0 { return resident_backing::close_history(&mut self.cursor.revision_accumulator.applied, grant); }
        if self.cursor.revision_accumulator.redo.capacity() != 0 { return resident_backing::close_history(&mut self.cursor.revision_accumulator.redo, grant); }
        if resident_backing::deque_demand(&self.owners.displaced_retirements.owners) != 0 { return resident_backing::close_deque(&mut self.owners.displaced_retirements.owners, grant); }
        if self.live.prefix_ring.capacity() != 0 { return resident_backing::close_vec(&mut self.live.prefix_ring, grant); }
        if self.wire.member_inbox.capacity() != 0 { return resident_backing::close_deque(&mut self.wire.member_inbox, grant); }
        let (report_messages, report_outbound, report_edit_ids) = self.replay.pending_report.resident_backings_mut();
        if report_messages.capacity() != 0 { return resident_backing::close_vec(report_messages, grant); }
        if report_outbound.capacity() != 0 { return resident_backing::close_vec(report_outbound, grant); }
        if let Some(ids) = report_edit_ids.filter(|ids| ids.capacity() != 0) { return resident_backing::close_vec(ids, grant); }
        self.owners.snapshot_read_leases.close_empty_backing_step(grant)
    }

    /// 🧹️ Advances owned-store disposal without requiring this store to be a composed
    /// [`SpaceMember`]. Local lanes use the same exact disposer authority.
    pub fn close_owned_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        self.close_owned_store_step(grant)
    }

    /// 🧺️ Confirms owned-store disposal for document and local lane snapshots alike.
    pub fn close_owned_terminal_is_empty(&self) -> bool {
        self.close_owned_store_terminal_is_empty()
    }

    /// 🧹️ Drives owned-store disposal to its terminal witness from the store's own quoted demands for callers no scheduler grants.
    pub fn close_owned_unscheduled(&mut self) -> Result<(), ValueError> {
        for _ in 0..ARTIFACT_STORE_UNSCHEDULED_CLOSE_STEPS {
            if self.close_owned_terminal_is_empty() { return Ok(()); }
            let demand = self.close_owned_demands(ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES)?;
            let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes.max(ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES), maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth.max(1) };
            if let RetainedCloneStep::Complete(_) = self.close_owned_step(grant)? { return if self.close_owned_terminal_is_empty() { Ok(()) } else { Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "store close completed without its terminal-empty witness")) }; }
        }
        Err(ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit, "store close did not reach its terminal-empty witness within its bounded steps"))
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
//#endregion 🔖️Backbone
//#endregion 🔖️Backbone
//#endregion 🔖️EphemeralLanes
