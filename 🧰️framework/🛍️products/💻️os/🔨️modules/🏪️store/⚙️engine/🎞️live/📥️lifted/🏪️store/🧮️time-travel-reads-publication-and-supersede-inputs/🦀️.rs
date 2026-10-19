//! 🧮️ Time travel reads, publication and supersede inputs: the supersessions and mutation outcomes, the report replay, the one item publication batches, the tool transactions and the supersede and draft inputs. See [`super`].

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
impl<P, Mutation> StoreLive<P, Mutation>
where
    P: Clone + ToValue + FromValue + Send + Sync + 'static,
    Mutation: Clone + ToValue + FromValue + super::Mutation<P> + OpBinary + OpText + Send + 'static,
{
    //#region 🔖️TimeTravelReads
    /// 🗺️ The effective supersessions the live projection folds with.
    pub fn supersessions(&self) -> &EffectiveSupersessions {
        &self.supersessions
    }

    /// 🧾️ Every applied operation in applied order: its identity, owning edit and applied position, operation index,
    /// recorded forward, metadata, tool transaction and effective supersession — O(applied operations).
    pub fn mutation_ops<'a>(&'a self, store_log: &'a StoreLog<P, Mutation>, store_wire: &StoreWire<P, Mutation>, store_cursor: &StoreCursor<P, Mutation>) -> Result<Vec<AppliedMutation<'a, Mutation>>, VcsError> {
        let mut operations = Vec::new();
        for position in 0..store_wire.applied_edit_ids(store_log).len() {
            operations.extend(self.applied_edit_mutations(store_log, store_wire, store_cursor, position)?);
        }
        Ok(operations)
    }

    /// 🧾️ One applied operation without constructing the rest of the history's rows.
    pub fn applied_mutation<'a>(&'a self, store_log: &'a StoreLog<P, Mutation>, store_wire: &StoreWire<P, Mutation>, store_cursor: &StoreCursor<P, Mutation>, target: &MutationId) -> Result<AppliedMutation<'a, Mutation>, VcsError> {
        let (position, index) = store_cursor.locate_mutation(store_log, store_wire, target)?;
        let edit = store_cursor.applied_edit(store_log, store_wire, position)?;
        let operation = &edit.forwards[index];
        let meta = edit.mutation_meta.get(index);
        let supersession = self.supersessions.get(target);
        let effective = effective_operation::<P, Mutation>(operation, index as u32, target.clone(), &store_log.envelope.schema, &self.supersessions);
        let descriptor = effective.operation().unwrap_or(operation).descriptor();
        let digest = CursorRevisionAccumulator::hash_record(b"edit-id", &[edit.id.as_bytes()]);
        let spans = store_cursor.applied_unit_flag(store_wire, digest, index) || matches!(meta.map(|meta| &meta.origin), Some(semio_framework_os_spr::MutationOrigin::Transaction { .. }));
        let unit = spans.then(|| meta.and_then(|meta| meta.group_id.clone()).unwrap_or_else(|| target.0.clone()));
        Ok(AppliedMutation { mutation_id: target.clone(), edit_id: &edit.id, position, op_index: index as u32, operation, meta, transaction: meta.and_then(|meta| meta.transaction.as_ref()), supersession, effective: descriptor, unit })
    }

    /// 🧾️ The operations of the applied edit at `position` — what one history row reads, O(that edit) (amortized O(1)
    /// for the edit lookup when rows are read in applied order).
    pub fn applied_edit_mutations<'a>(&'a self, store_log: &'a StoreLog<P, Mutation>, store_wire: &StoreWire<P, Mutation>, store_cursor: &StoreCursor<P, Mutation>, position: usize) -> Result<Vec<AppliedMutation<'a, Mutation>>, VcsError> {
        let edit = store_cursor.applied_edit(store_log, store_wire, position)?;
        let units = store_cursor.applied_edit_units(store_log, store_wire, position)?;
        let mut operations = Vec::with_capacity(edit.forwards.len());
        for (index, (mutation_id, operation)) in semio_framework_os_spr::mutation_ids_for_edit::<P, Mutation>(edit).into_iter().zip(edit.forwards.iter()).enumerate() {
            let meta = edit.mutation_meta.get(index);
            let supersession = self.supersessions.get(&mutation_id);
            let effective = match supersession {
                None => operation.descriptor(),
                Some(_) => {
                    let effective = effective_operation::<P, Mutation>(operation, index as u32, mutation_id.clone(), &store_log.envelope.schema, &self.supersessions);
                    effective.operation().unwrap_or(operation).descriptor()
                }
            };
            operations.push(AppliedMutation { mutation_id, edit_id: edit.id.as_str(), position, op_index: index as u32, operation, meta, transaction: meta.and_then(|meta| meta.transaction.as_ref()), supersession, effective, unit: units.get(index).cloned().flatten() });
        }
        Ok(operations)
    }

    /// 📋️ The durable per-mutation outcomes of the applied history: the message ledger split by operation, with every
    /// operation's supersession state.
    pub fn mutation_outcomes(&self, store_log: &StoreLog<P, Mutation>, store_wire: &StoreWire<P, Mutation>, store_cursor: &StoreCursor<P, Mutation>) -> Result<Vec<protocol::MutationReplayOutcome>, VcsError> {
        self.durable_outcomes(store_log, store_wire, store_cursor, 0)
    }

    /// 🗄️ The durable outcomes of `applied_edit_ids[from..]`.
    fn durable_outcomes(&self, store_log: &StoreLog<P, Mutation>, store_wire: &StoreWire<P, Mutation>, store_cursor: &StoreCursor<P, Mutation>, from: usize) -> Result<Vec<protocol::MutationReplayOutcome>, VcsError> {
        let mut outcomes = Vec::new();
        for position in from.min(store_log.applied_edit_ids.len())..store_log.applied_edit_ids.len() {
            outcomes.extend(self.applied_edit_outcomes(store_log, store_wire, store_cursor, position)?);
        }
        Ok(outcomes)
    }

    /// 🗄️ The durable outcomes of the applied edit at `position` — what one history row reads, O(that edit and its
    /// messages).
    pub fn applied_edit_outcomes(&self, store_log: &StoreLog<P, Mutation>, store_wire: &StoreWire<P, Mutation>, store_cursor: &StoreCursor<P, Mutation>, position: usize) -> Result<Vec<protocol::MutationReplayOutcome>, VcsError> {
        let edit = store_cursor.applied_edit(store_log, store_wire, position)?;
        let messages = store_log.messages_for_edit(&edit.id);
        let mut by_operation: Vec<Vec<semio_framework_os_spr::MutationMessage>> = vec![Vec::new(); edit.forwards.len()];
        for message in messages {
            if let Some(slot) = message.op_index.and_then(|index| by_operation.get_mut(index as usize)) {
                slot.push(message.clone());
            }
        }
        Ok(semio_framework_os_spr::mutation_ids_for_edit::<P, Mutation>(edit)
            .into_iter()
            .zip(by_operation)
            .enumerate()
            .map(|(index, (mutation_id, operation_messages))| {
                let supersession = self.supersessions.get(&mutation_id);
                protocol::MutationReplayOutcome {
                    worst: semio_framework_os_spr::worst_level(&operation_messages),
                    messages: operation_messages,
                    superseded: supersession.is_some(),
                    withdrawn: supersession.is_some_and(|supersession| supersession.replacement == protocol::InputReplacement::Withdrawn),
                    mutation_id,
                    edit_id: edit.id.clone(),
                    op_index: index as u32,
                }
            })
            .collect())
    }

    /// 🫂️ The cross-artifact unit `mutation_id` belongs to; `None` for an operation that is its own unit
    /// ([`Self::applied_edit_units`]). A unit is superseded in all of its documents at once, which no replica can do yet
    /// ([`VcsError::UnitSpansDocuments`]).
    pub fn unit_id(&self, store_log: &StoreLog<P, Mutation>, store_wire: &StoreWire<P, Mutation>, store_cursor: &StoreCursor<P, Mutation>, mutation_id: &MutationId) -> Result<Option<String>, VcsError> {
        Ok(self.applied_mutation(store_log, store_wire, store_cursor, mutation_id)?.unit)
    }

    /// 🕰️ The projection right before operation `mutation_id` with `drafts` laid over the effective inputs — nothing
    /// downstream applied: the time-travel preview base. Folds at most one prefix-ring stride (plus the operation's own
    /// edit) and retains the live prefix projections it folds through.
    pub fn state_before(&mut self, store_log: &mut StoreLog<P, Mutation>, store_owners: &mut StoreOwners<P, Mutation>, store_wire: &mut StoreWire<P, Mutation>, store_cursor: &mut StoreCursor<P, Mutation>, mutation_id: &MutationId, drafts: &protocol::HistoryInputDrafts) -> Result<Arc<P>, VcsError> where P: Clone, Mutation: ColdMutation<P> {
        let (position, op_index) = store_cursor.locate_mutation(store_log, store_wire, mutation_id)?;
        let mut live_until = position;
        for target in drafts.keys() {
            live_until = live_until.min(store_cursor.locate_mutation(store_log, store_wire, target)?.0);
        }
        let supersessions = draft_supersessions(&self.supersessions, drafts);
        let applied: Vec<String> = store_log.applied_edit_ids.to_vec();
        let (base, recorded) = self.prefix_state_recorded(store_log, store_cursor, &applied, position, &supersessions, Self::prefix_stride(applied.len()))?;
        let (mut live, stale): (Vec<(usize, Arc<P>)>, Vec<(usize, Arc<P>)>) = recorded.into_iter().partition(|(length, _)| *length <= live_until);
        for (_, snapshot) in stale {
            retire_shared_projection::<P, Mutation>(snapshot);
        }
        if position <= live_until {
            live.push((position, Arc::clone(&base)));
        }
        if let Err(error) = self.retain_prefix_snapshots(store_log, store_owners, store_cursor, live) {
            retire_shared_projection::<P, Mutation>(base);
            return Err(error);
        }
        let Some(edit) = store_log.envelope.vcs.edits.find_near(|edit| edit.id == applied[position]) else {
            retire_shared_projection::<P, Mutation>(base);
            return Err(VcsError::UnknownEdit(applied[position].clone()));
        };
        let mut running = base;
        fold_effective_edit(&mut running, edit, &store_log.envelope.schema, &supersessions, op_index);
        Ok(running)
    }

    /// 🎞️ A Report replay of the applied history with `drafts` laid over the effective inputs, from `from` (or the first
    /// operation a draft changes, whichever is earlier). A job steps it against {@link replay_edits} under its deadline
    /// with progress; cancelling drops it and leaves this store untouched; {@link replay_report} reads the result.
    pub fn begin_report_replay(&self, store_log: &StoreLog<P, Mutation>, store_wire: &StoreWire<P, Mutation>, store_cursor: &StoreCursor<P, Mutation>, drafts: &protocol::HistoryInputDrafts, from: Option<&MutationId>) -> Result<EditReplay<P, Mutation>, VcsError> {
        let supersessions = draft_supersessions(&self.supersessions, drafts);
        let applied: Vec<String> = store_log.applied_edit_ids.to_vec();
        let (window, horizon) = self.replay_window(store_log, &applied, &supersessions)?;
        let from = match from {
            Some(mutation_id) => store_cursor.locate_mutation(store_log, store_wire, mutation_id)?.0.min(window),
            None => window,
        };
        Ok(self.report_replay(store_log, store_cursor, &applied, from, supersessions, horizon, usize::MAX)?.with_drafts(drafts.clone()))
    }

    /// 🛡️ Judges the folded history as a whole under `MergePolicy::Normal`: one Report replay of every applied edit from the
    /// genesis, refused with its blocking messages when the report blocks finalize.
    pub fn refuse_blocking_history(&self, store_log: &StoreLog<P, Mutation>) -> Result<(), VcsError> {
        let result = EditReplay::new(ReplayMode::Report, store_log.envelope.vcs.genesis.facts().share_snapshot(), store_log.applied_edit_ids.to_vec(), 0, &store_log.envelope.schema, (*self.supersessions).clone(), &store_log.envelope.vcs.edits)?.run(&store_log.envelope.vcs.edits)?;
        let blocks = result.report().blocks_finalize();
        let messages: Vec<_> = if blocks { result.report().outcomes.iter().flat_map(|outcome| outcome.messages.iter()).filter(|message| semio_framework_os_spr::MergePolicy::Normal.rejects(message.level)).cloned().collect() } else { Vec::new() };
        result.retire_unscheduled().map_err(|error| VcsError::ValidationFailed(error.into_message()))?;
        if !blocks {
            return Ok(());
        }
        Err(VcsError::Rejected { policy: semio_framework_os_spr::MergePolicy::Normal, messages })
    }

    /// 📑️ A finished replay's per-mutation report; a replay that converged gets the unreplayed tail's durable outcomes
    /// spliced in, so an early exit reports exactly what a full replay would.
    pub fn replay_report(&self, store_log: &StoreLog<P, Mutation>, store_wire: &StoreWire<P, Mutation>, store_cursor: &StoreCursor<P, Mutation>, result: &EditReplayResult<P, Mutation>) -> Result<protocol::ReplayReport, VcsError> {
        let mut report = result.report().clone();
        if let Some(length) = result.converged_at() {
            report.outcomes.extend(self.durable_outcomes(store_log, store_wire, store_cursor, length)?);
            report.worst = report.outcomes.iter().filter_map(|outcome| outcome.worst).max();
        }
        Ok(report)
    }

    //#endregion 🔖️TimeTravelReads
    //#region 📬️OneItemPublication
    /// 🧵️ Mints the base read one staged item prepares against: the store's own committed root for
    /// the first item, and the previous item's post root — leased through the same exact registry —
    /// for every later one.
    pub fn batch_item_base(&self, store_owners: &StoreOwners<P, Mutation>, store_wire: &StoreWire<P, Mutation>, store_cursor: &StoreCursor<P, Mutation>, staged_root: Option<&Arc<P>>) -> Result<SnapshotRead<P>, VcsError>
    where
        P: Sync,
    {
        let Some(root) = staged_root else { return self.snapshot_read(store_owners, store_wire, store_cursor) };
        if !store_owners.snapshot_read_leases.publish_authority(store_cursor.generation, store_cursor.content_revision) {
            return Err(VcsError::ValidationFailed("staged batch read commit authority is busy or exhausted".into()));
        }
        let owner = Arc::clone(root);
        let lease = store_owners.snapshot_read_leases.try_issue(owner.clone()).map_err(|_| VcsError::ValidationFailed("staged batch read lease registry is busy, saturated, or exhausted".into()))?;
        Ok(SnapshotRead::new(owner, lease))
    }

    /// 🫳️ Keeps the genuine prepared forward and original staged/current prebase borrowed together.
    fn original_proposal_forward<'a>(&'a self, store_cursor: &StoreCursor<P, Mutation>,publication:&'a ArtifactStoreBatchPublication<P,Mutation>)->Result<(&'a Mutation,&'a P),ValueError>{
        use semio_framework_value::ValueRefusalKind;
        
        if !publication.original_registry.as_ref().is_some_and(|registry|registry.ptr_eq(&self.snapshot_read_leases)){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"prepared proposal requires its original Store issuer"));}
if publication.phase!=ArtifactStoreOneItemPublicationPhase::Preparing||publication.item_closing||publication.cancel_requested||publication.close_started||store_cursor.generation!=publication.expected_generation||store_cursor.content_revision!=publication.expected_revision||publication.stage.as_ref().is_some_and(|stage|stage.folding.is_some()){
            return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"foreign proposal requires the original unfurled prepared candidate and live prebase"));
        }
        let candidate=publication.preparation.as_ref().and_then(|owner|owner.prepared()).ok_or_else(||ValueError::literal(ValueRefusalKind::InvalidValue,"foreign proposal original domain candidate is not prepared"))?;
        if candidate.edit().forwards.len()!=1{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"foreign proposal requires exactly one original prepared forward"));}
        let original=&candidate.edit().forwards[0];
        let prebase=publication.stage.as_ref().and_then(|stage|stage.post.as_ref()).map_or(self.current.as_ref(),|root|root.as_ref());
        Ok((original,prebase))
    }

    /// 🌐️ Advances original borrowed foreign rows before any fold transfer or replacement of their prebase.
    pub fn advance_prepared_foreign_steps(
        &self, store_cursor: &StoreCursor<P, Mutation>, publication:&ArtifactStoreBatchPublication<P,Mutation>,
        output:&mut protocol::ForeignStepsPreparation, grant:RetainedCloneGrant,
    )->Result<RetainedCloneStep,ValueError> {
        let(original,prebase)=self.original_proposal_forward(store_cursor, publication)?;
        let source=if original.may_emit_foreign_steps(){original.foreign_step_source(prebase,output.source_index())?}else{None};
        let step=output.advance_source(source,grant)?;
        semio_framework_value::retained_clone::admit_retained_clone_progress(grant,step.progress(),"original prepared foreign row turn")?;
        Ok(step)
    }

    /// ✍️ Uses the original installed typed operation codec and caller-funded output prefix without eager encoding.
    pub fn advance_prepared_operation_wire(
        &self, store_cursor: &StoreCursor<P, Mutation>, publication:&ArtifactStoreBatchPublication<P,Mutation>,
        cursor:&mut ArtifactPreparedOperationCursor,output:&mut[u8],grant:RetainedCloneGrant,
    )->Result<ArtifactPreparedOperationProgress,ArtifactPreparedOperationError>{
        let(original,_)=self.original_proposal_forward(store_cursor, publication)?;
        let source=publication.source.as_ref().and_then(|owner|owner.operation_wire_source(original)).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"proposal bytes require their original installed typed codec issuer"))?;
        cursor.advance(source,output,grant)
    }

    //#endregion 📬️OneItemPublication
}

impl<P, Mutation> StoreLive<P, Mutation>
where
    P: Clone + ToValue + FromValue + Send + Sync + 'static,
    Mutation: Clone + ToValue + FromValue + super::Mutation<P> + OpBinary + OpText + Send + 'static,
{
    //#region 🔖️ToolTransactions
    /// 🪂️ Takes the open tool transaction's edit off the applied tail before a remote merge: the ledger, the message ledger
    /// and the projection return to the history without it.
    pub async fn lift_open_transaction(&mut self, store_log: &mut StoreLog<P, Mutation>, store_owners: &mut StoreOwners<P, Mutation>, store_cursor: &mut StoreCursor<P, Mutation>) -> Result<Option<(OpenToolTransaction, Edit<Mutation>)>, VcsError> where P: Clone, Mutation: ColdMutation<P> {
        let Some(open) = store_log.envelope.open_transaction.take() else {
            return Ok(None);
        };
        let mut removed = store_log.envelope.vcs.edits.extract_if(|edit| edit.id == open.edit_id).map_err(|fault| VcsError::ValidationFailed(format!("the open transaction edit cannot leave its ledger: {fault:?}")))?;
        let edit = removed.pop().ok_or_else(|| VcsError::UnknownEdit(open.edit_id.clone()))?;
        retire_scratch_edits::<P, Mutation>(removed);
        store_owners.replace_edit_messages(store_log, &open.edit_id, Vec::new())?;
        self.reproject(store_log, store_owners, store_cursor).await?;
        Ok(Some((open, edit)))
    }

    //#endregion 🔖️ToolTransactions
    //#region 🔖️Supersede
    /// 🧪️ The Report replay of everything `fold` changes against the live projection, driven to completion, touching nothing.
    /// Synchronous: the supersede command API (agents, text, hub) gets its verdict inside the dispatch. Under
    /// [`Self::defer_local_replays`] [`Self::author_supersession`] steps the same replay across turns instead.
    pub fn dry_run(&self, store_log: &StoreLog<P, Mutation>, store_cursor: &StoreCursor<P, Mutation>, fold: &semio_framework_os_spr::HistoryFold) -> Result<EditReplayResult<P, Mutation>, VcsError> {
        let (from, horizon) = self.replay_window(store_log, &fold.applied, &fold.supersessions)?;
        self.report_replay(store_log, store_cursor, &fold.applied, from, fold.supersessions.clone(), horizon, Self::prefix_stride(fold.applied.len()))?.run(&store_log.envelope.vcs.edits)
    }

    /// 🛃️ Validates and encodes supersede inputs: every target is an applied operation and every input obeys the supersede
    /// law ([`admit_replacement`]). Also answers the conflict target the transition envelope declares — the longest address
    /// every superseded and replacing operation shares. Every replacement retires once encoded, on success and refusal alike.
    pub fn supersede_inputs(&self, store_log: &StoreLog<P, Mutation>, store_wire: &StoreWire<P, Mutation>, store_cursor: &StoreCursor<P, Mutation>, inputs: Vec<SupersedeInput<Mutation>>) -> Result<(Vec<protocol::SupersededInput>, Vec<String>), VcsError> {
        let mut applied: HashMap<MutationId, &Mutation> = HashMap::new();
        let mut failure = None;
        for edit_id in store_log.applied_edit_ids.iter() {
            match store_log.envelope.vcs.edits.find_near(|edit| edit.id == *edit_id) {
                Some(edit) => applied.extend(semio_framework_os_spr::mutation_ids_for_edit::<P, Mutation>(edit).into_iter().zip(edit.forwards.iter())),
                None => {
                    failure = Some(VcsError::UnknownEdit(edit_id.clone()));
                    break;
                }
            }
        }
        let mut encoded = Vec::with_capacity(inputs.len());
        let mut address: Option<Vec<String>> = None;
        for SupersedeInput { target, replacement } in inputs {
            let checked = match (failure.is_some(), applied.get(&target)) {
                (true, _) => None,
                (false, None) => Some(Err(VcsError::ValidationFailed(format!("supersede target {} is not an applied operation", target.0)))),
                (false, Some(original)) => Some(self.supersede_input(store_log, store_wire, store_cursor, &target, original, replacement.as_ref())),
            };
            if let Some(replacement) = replacement {
                super::Mutation::<P>::retire_cold(replacement);
            }
            match checked {
                Some(Ok((input, written))) => {
                    address = Some(match address {
                        Some(known) => common_address(&known, &written),
                        None => written,
                    });
                    encoded.push(input);
                }
                Some(Err(error)) => failure = Some(error),
                None => {}
            }
        }
        match failure {
            Some(error) => Err(error),
            None => Ok((encoded, address.unwrap_or_default())),
        }
    }

    /// 🧩️ One supersede input: its encoded replacement, held to the supersede law ([`admit_replacement`]) every replica
    /// folds it under, and the address it and the original operation share. An operation of a cross-artifact unit takes no
    /// supersession in one document alone ([`Self::unit_id`]).
    fn supersede_input(&self, store_log: &StoreLog<P, Mutation>, store_wire: &StoreWire<P, Mutation>, store_cursor: &StoreCursor<P, Mutation>, target: &MutationId, original: &Mutation, replacement: Option<&Mutation>) -> Result<(protocol::SupersededInput, Vec<String>), VcsError> {
        if let Some(unit) = self.unit_id(store_log, store_wire, store_cursor, target)? {
            return Err(VcsError::UnitSpansDocuments { mutation_id: target.0.clone(), unit });
        }
        let replacement = match replacement {
            None => protocol::InputReplacement::Withdrawn,
            Some(operation) => protocol::InputReplacement::Input { schema: store_log.envelope.schema.clone(), payload: operation.encode_op().map_err(|error| VcsError::Serialize(error.to_string()))? },
        };
        let admitted = admit_replacement::<P, Mutation>(original, &replacement, &store_log.envelope.schema).map_err(|reason| VcsError::ValidationFailed(format!("the supersession of {} {reason}", target.0)))?;
        let mut address = original.conflict_target();
        if let Some(admitted) = admitted {
            address = common_address(&address, &admitted.conflict_target());
            super::Mutation::<P>::retire_cold(admitted);
        }
        Ok((protocol::SupersededInput { target: target.clone(), replacement }, address))
    }

    /// ✒️ Draft inputs a time-travel session previewed, validated exactly like typed supersede inputs.
    pub fn draft_inputs(&self, store_log: &StoreLog<P, Mutation>, store_wire: &StoreWire<P, Mutation>, store_cursor: &StoreCursor<P, Mutation>, drafts: &protocol::HistoryInputDrafts) -> Result<(Vec<protocol::SupersededInput>, Vec<String>), VcsError> {
        let mut typed = Vec::with_capacity(drafts.len());
        let mut failure = None;
        for (target, replacement) in drafts {
            let decoded = match replacement {
                protocol::InputReplacement::Withdrawn => Ok(None),
                protocol::InputReplacement::Input { schema, payload } if *schema == store_log.envelope.schema => Mutation::decode_op(payload).map(Some).map_err(|error| VcsError::Deserialize(format!("draft of {} does not decode: {error}", target.0))),
                protocol::InputReplacement::Input { schema, .. } => Err(VcsError::ValidationFailed(format!("draft of {} names schema {schema}, not {}", target.0, store_log.envelope.schema))),
            };
            match decoded {
                Ok(replacement) => typed.push(SupersedeInput { target: target.clone(), replacement }),
                Err(error) => {
                    failure.get_or_insert(error);
                }
            }
        }
        if let Some(error) = failure {
            retire_supersede_inputs::<P, Mutation>(typed);
            return Err(error);
        }
        self.supersede_inputs(store_log, store_wire, store_cursor, typed)
    }

    //#endregion 🔖️Supersede
    pub fn snapshot_json(&self, store_wire: &StoreWire<P, Mutation>) -> Result<String, VcsError>  where P: Clone {
        let snapshot = self.snapshot(store_wire)?;
        Ok(semio_framework_pack_json::to_json_string(&snapshot))
    }

    fn take_candidate_current(&mut self) -> Arc<P> {
        assert!(!self.current_detached, "resolution candidate snapshot authority was already detached");
        self.current_detached = true;
        unsafe { std::mem::ManuallyDrop::take(&mut self.current) }
    }

    /// ➖️ Pops the applied tail; its mirrors and the prefix ring are stale past the new length.
    pub fn pop_applied_edit_id(&mut self, store_log: &mut StoreLog<P, Mutation>, store_cursor: &mut StoreCursor<P, Mutation>) -> Option<String> {
        let edit_id = store_log.applied_edit_ids.pop()?;
        let length = store_log.applied_edit_ids.len();
        store_cursor.mirror_dirt.dirty_applied_from(length);
        self.prefix_ring_dirty_from = self.prefix_ring_dirty_from.min(length);
        Some(edit_id)
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
