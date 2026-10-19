//! 🧹️ Hydration retirement, drop and completion: the remaining hydration operations, the erased snapshot retirement, the drop and the completed hydration accessors. See [`super`].

use super::*;

impl<P, M> RetainedPersistedDocumentHydration<P, M>
where
    P: Clone + ToValue + FromValue + ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + Send + Sync + 'static,
    M: Clone + ToValue + FromValue + Mutation<P> + OpBinary + OpText + Send + 'static,
{
    pub fn close_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        if self.terminal_is_empty_unbounded() { return Ok(Default::default()); }
        if let Some(active) = self.active.as_ref() { return semio_framework_os_store_read::artifact_retirement_box_demands(active, body); }
        macro_rules! original {
            ($field:ident) => { if self.$field.is_some() { return semio_framework_os_store_read::artifact_retirement_owned_birth_demands(&self.$field); } };
        }
        let nested = |mut demand: RetirementDemand| -> Result<RetirementDemand, ValueError> { demand.depth = demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(ValueRefusalKind::DepthLimit, "hydration child depth overflow"))?; Ok(demand) };
        let frame = |bytes| RetirementDemand { capacity_bytes: bytes, depth: 2, ..Default::default() };
        if self.fold_job.is_some() { return semio_framework_os_store_read::retirement_owner_demands(&self.fold_job, body); }
        original!(normalized_conflicts);
        original!(normalized_transitions);
        if self.target_decoder.is_some() { return semio_framework_os_store_read::retirement_owner_demands(&self.target_decoder, body); }
        original!(pending_target);
        original!(target_address);
        original!(target_source);
        original!(replay_ids);
        original!(replay_order);
        if let Some(catalog) = self.initialization_catalog.as_ref() { return nested(catalog.close_demands()?); }
        if let Some(runtime) = self.runtime.as_ref() { return nested(runtime.initialization_retirement_demands(body)?); }
        if self.pending_edit.is_some() { return Ok(frame(size_of::<semio_framework_os_store_ledger::edit_retirement::ArtifactStoreDecodedEditRetirement<M>>())); }
        if self.pending_messages.is_some() { return Ok(frame(size_of::<semio_framework_os_spr::command::MutationMessageLedgerRetirement>())); }
        if self.replay.is_some() || self.loaded_result.is_some() { return Ok(frame(size_of::<semio_framework_os_store_read::ArtifactHistoryReadRetirement<P, M>>())); }
        if self.envelope.is_some() { return Ok(frame(size_of::<semio_framework_os_store_read::ArtifactStoreEnvelopeRetirement<P, M>>())); }
        if let Some(initial) = self.initial.as_ref() {
            let owners = self.owners.as_ref().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "hydration original projection lost its declared factory"))?;
            return Ok(frame(owners.initial_snapshot_retirement.retirement_birth_bytes(initial)));
        }
        original!(history);
        original!(mutation_lookup);
        original!(pin_refs);
        original!(edit_lookup);
        original!(fold);
        original!(pack);
        original!(expected);
        original!(owner);
        if self.actor.0.has_owner() { return Ok(frame(semio_framework_value::retirement::owned_retirement_birth_bytes::<semio_framework_value::SharedUtf8>())); }
        original!(schema);
        let owners = self.owners.as_ref().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "hydration retained owner catalog is missing"))?;
        nested(owners.uninstalled_owners_demands(body)?)
    }
}

impl<P, M> ErasedSnapshotRetirement for RetainedPersistedDocumentHydration<P, M>
where
    P: Clone + ToValue + FromValue + ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + Send + Sync + 'static,
    M: Clone + ToValue + FromValue + Mutation<P> + OpBinary + OpText + Send + 'static,
{
        fn next_demand(&self,body:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{self.close_demands(body)}

fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.close_demands(0)?.copy_bytes) }
    fn next_capacity_byte_demand(&self, body: usize) -> Result<usize, ValueError> { Ok(self.close_demands(body)?.capacity_bytes) }
    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.close_demands(0)?.release_bytes) }
    fn next_depth_demand(&self) -> Result<usize, ValueError> { Ok(self.close_demands(0)?.depth) }

    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let empty = RetainedCloneProgress::default();
        if self.terminal_is_empty_unbounded() { return Ok(RetainedCloneStep::Complete(empty)); }
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(empty)); }
        let demand = self.close_demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth < demand.depth { return Err(ValueError::literal(ValueRefusalKind::DepthLimit, "hydration closure exceeds supplied depth")); }
        if grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes { return Ok(RetainedCloneStep::Progress(empty)); }
        self.phase = Phase::Rejected;
        self.diagnostic.get_or_insert(MemberOpenDiagnostic::Cancelled);
        if self.active.is_some() { return semio_framework_os_store_read::artifact_retirement_box_close_step(&mut self.active, grant); }
        if self.fold_job.is_some() { return semio_framework_os_store_read::retirement_owner_close(&mut self.fold_job, grant).map(|step| RetainedCloneStep::Progress(step.progress())); }
        macro_rules! original {
            ($field:ident) => { if self.$field.is_some() { return semio_framework_os_store_read::artifact_retirement_admit_owned(&mut self.$field, &mut self.active, grant); } };
        }
        original!(normalized_conflicts);
        original!(normalized_transitions);
        if self.target_decoder.is_some() { return semio_framework_os_store_read::retirement_owner_close(&mut self.target_decoder, grant).map(|step| RetainedCloneStep::Progress(step.progress())); }
        original!(pending_target);
        original!(target_address);
        original!(target_source);
        original!(replay_ids);
        original!(replay_order);
        let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant };
        if let Some(catalog) = self.initialization_catalog.as_mut() {
            let step = catalog.close_step(child)?;
            semio_framework_value::retained_clone::admit_retained_clone_close(child, step, catalog.terminal_is_empty(), "hydration original initialization page catalog")?;
            if catalog.terminal_is_empty() { self.initialization_catalog.take(); }
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if let Some(runtime) = self.runtime.as_mut() {
            let owners = self.owners.as_ref().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "hydration runtime lost its original factory"))?;
            let step = runtime.close_step(&owners.initial_snapshot_retirement, child)?;
            semio_framework_value::retained_clone::admit_retained_clone_close(child, step, runtime.terminal_is_empty(), "hydration inline runtime")?;
            if matches!(step, RetainedCloneStep::Complete(_)) { self.runtime.take(); }
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if self.pending_edit.is_some() {
            let owners = self.owners.as_ref().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "hydration original edit lost its owner catalog"))?;
            let edit = self.pending_edit.take().expect("observed original edit");
            match owners.retire_decoded_edit(edit, child) {
                Ok((owner, progress)) => { *self.active = Some(owner); semio_framework_value::retained_clone::admit_retained_clone_progress(child, progress, "hydration original edit birth")?; if progress.retained_capacity_bytes != demand.capacity_bytes { return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "hydration edit birth differs from its original quote")); } return Ok(RetainedCloneStep::Progress(progress)); }
                Err((error, original)) => { *self.pending_edit = Some(original); return Err(error); }
            }
        }
        if let Some(messages) = self.pending_messages.take() {
            match semio_framework_value::retirement::frame::admit_retirement_frame(messages, child, |messages| semio_framework_os_spr::command::MutationMessageLedgerRetirement::new(messages.edit_id, messages.messages)) {
                Ok((owner, progress)) => { *self.active = Some(owner); return Ok(RetainedCloneStep::Progress(progress)); }
                Err((error, original)) => { *self.pending_messages = Some(original); return Err(error); }
            }
        }
        if self.replay.is_some() || self.loaded_result.is_some() {
            let owners = self.owners.as_ref().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "hydration original replay lost its owner catalog"))?;
            let originals = (self.replay.take(), self.loaded_result.take());
            match semio_framework_value::retirement::frame::admit_retirement_frame(originals, child, |(replay, result)| {
                let mut owner = self.loaded_replay_retirement(owners);
                *owner.loaded_replay = replay;
                *owner.finished = result;
                owner
            }) {
                Ok((owner, progress)) => { *self.active = Some(owner); return Ok(RetainedCloneStep::Progress(progress)); }
                Err((error, (replay, result))) => { *self.replay = replay; *self.loaded_result = result; return Err(error); }
            }
        }
        if self.envelope.is_some() {
            let owners = self.owners.as_ref().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "hydration original envelope lost its owner catalog"))?;
            let envelope = self.envelope.take().expect("observed original envelope");
            match owners.retire_decoded_envelope(envelope, child) {
                Ok((owner, progress)) => { *self.active = Some(owner); semio_framework_value::retained_clone::admit_retained_clone_progress(child, progress, "hydration original envelope birth")?; if progress.retained_capacity_bytes != demand.capacity_bytes { return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "hydration envelope birth differs from its original quote")); } return Ok(RetainedCloneStep::Progress(progress)); }
                Err((error, original)) => { *self.envelope = Some(original); return Err(error); }
            }
        }
        if self.initial.is_some() {
            let owners = self.owners.as_ref().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "hydration original projection lost its owner catalog"))?;
            let initial = self.initial.take().expect("observed original projection");
            match owners.retire_initial_snapshot_owned(initial, child) {
                Ok((owner, progress)) => { *self.active = Some(owner); semio_framework_value::retained_clone::admit_retained_clone_progress(child, progress, "hydration original projection birth")?; if progress.retained_capacity_bytes != demand.capacity_bytes { return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "hydration projection birth differs from its original quote")); } return Ok(RetainedCloneStep::Progress(progress)); }
                Err((error, original)) => { *self.initial = Some(original); return Err(error); }
            }
        }
        original!(history);
        original!(mutation_lookup);
        original!(pin_refs);
        original!(edit_lookup);
        original!(fold);
        original!(pack);
        original!(expected);
        original!(owner);
        if self.actor.0.has_owner() {
            let original = std::mem::take(&mut self.actor.0);
            match semio_framework_value::retirement::admit_owned_retirement(original, child) {
                Ok((owner, progress)) => { *self.active = Some(owner); semio_framework_value::retained_clone::admit_retained_clone_progress(child, progress, "hydration original actor birth")?; if progress.retained_capacity_bytes != demand.capacity_bytes { return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "hydration actor birth differs from its original quote")); } return Ok(RetainedCloneStep::Progress(progress)); }
                Err((error, original)) => { self.actor.0 = original; return Err(error); }
            }
        }
        original!(schema);
        let owners = self.owners.as_mut().expect("hydration final catalog remains original");
        let step = owners.close_uninstalled_owners_step(child)?;
        semio_framework_value::retained_clone::admit_retained_clone_close(child, step, owners.uninstalled_owners_terminal_is_empty(), "hydration final original catalog")?;
        if matches!(step, RetainedCloneStep::Complete(_)) { self.owners.take(); self.terminal = true; return Ok(RetainedCloneStep::Complete(step.progress())); }
        Ok(step)
    }

    fn terminal_is_empty(&self) -> bool { self.terminal_is_empty_unbounded() }
}

impl<P, M> Drop for RetainedPersistedDocumentHydration<P, M>
where
    P: ToValue,
    M: ToValue + FromValue + Mutation<P>,
{
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.terminal_is_empty_unbounded(), "persisted document hydration reached Drop before exact handoff or bounded retirement");
    }
}

impl<P, M> RetainedPersistedDocumentHydration<P, M>
where
    P: ToValue,
    M: ToValue + FromValue + Mutation<P>,
{
    /// 🔎️ Names the first original owner this hydration still retains, or `None` once every currency is terminal.
    pub fn remaining_owner(&self) -> Option<&'static str> {
        if !self.terminal { return Some("terminal"); }
        if self.actor.0.has_owner() { return Some("actor"); }
        if self.pack.is_some() { return Some("pack"); }
        if self.initial.is_some() { return Some("initial"); }
        if self.replay.is_some() { return Some("replay"); }
        if self.loaded_result.is_some() { return Some("loaded_result"); }
        if self.pin_refs.is_some() { return Some("pin_refs"); }
        if self.edit_lookup.is_some() { return Some("edit_lookup"); }
        if self.mutation_lookup.is_some() { return Some("mutation_lookup"); }
        if self.target_source.is_some() { return Some("target_source"); }
        if self.target_decoder.is_some() { return Some("target_decoder"); }
        if self.pending_target.is_some() { return Some("pending_target"); }
        if self.target_address.is_some() { return Some("target_address"); }
        if self.history.is_some() { return Some("history"); }
        if self.fold_job.is_some() { return Some("fold_job"); }
        if self.replay_ids.is_some() { return Some("replay_ids"); }
        if self.replay_order.is_some() { return Some("replay_order"); }
        if self.normalized_transitions.is_some() { return Some("normalized_transitions"); }
        if self.normalized_conflicts.is_some() { return Some("normalized_conflicts"); }
        if self.fold.is_some() { return Some("fold"); }
        if self.expected.is_some() { return Some("expected"); }
        if self.owner.is_some() { return Some("owner"); }
        if self.schema.is_some() { return Some("schema"); }
        if self.envelope.is_some() { return Some("envelope"); }
        if self.runtime.is_some() { return Some("runtime"); }
        if self.initialization_catalog.is_some() { return Some("initialization_catalog"); }
        if self.owners.is_some() { return Some("owners"); }
        if self.pending_edit.is_some() { return Some("pending_edit"); }
        if self.pending_messages.is_some() { return Some("pending_messages"); }
        if self.active.is_some() { return Some("active"); }
        None
    }

    pub fn terminal_is_empty_unbounded(&self) -> bool {
        self.remaining_owner().is_none()
    }
}
