//! 🪜️ The envelope-decode frontier of one hosted app: worker jobs, returned field decoders and completed records
//! retire one owner per turn under the caller's unchanged grant and exact per-axis quotes.
use super::*;
use semio_framework_value::{RetirementDemand, ValueError, retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep, admit_retained_clone_close}};

fn handoff_demand() -> RetirementDemand {
    RetirementDemand { depth: 1, ..Default::default() }
}

fn widest(left: RetirementDemand, right: RetirementDemand) -> RetirementDemand {
    RetirementDemand { copy_bytes: left.copy_bytes.max(right.copy_bytes), capacity_bytes: left.capacity_bytes.max(right.capacity_bytes), release_bytes: left.release_bytes.max(right.release_bytes), depth: left.depth.max(right.depth) }
}

pub(super) fn erased_demand<T: store::ErasedSnapshotRetirement + ?Sized>(owner: &T, body: usize) -> Result<RetirementDemand, ValueError> {
    Ok(RetirementDemand { copy_bytes: owner.next_copy_byte_demand()?, capacity_bytes: owner.next_capacity_byte_demand(body)?, release_bytes: owner.next_release_byte_demand()?, depth: owner.next_depth_demand()?.max(1) })
}

pub(super) fn erased_step<T: store::ErasedSnapshotRetirement + ?Sized>(owner: &mut T, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, Fault> {
    let empty = RetainedCloneProgress::default();
    if owner.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(empty)); }
    let demand = erased_demand(owner, grant.maximum_copy_bytes).map_err(plugin_retirement_fault)?;
    if !plugin_turn_admitted(demand, grant)? { return Ok(RetainedCloneStep::Progress(empty)); }
    let step = owner.close_step(grant).map_err(plugin_retirement_fault)?;
    admit_retained_clone_close(grant, step, owner.terminal_is_empty(), "envelope returned owner").map_err(plugin_retirement_fault)?;
    Ok(step)
}

fn completed_demand<P, Mutation>(record: &dyn store::ArtifactEnvelopeCompletedRecord<P, Mutation>, body: usize) -> Result<RetirementDemand, ValueError> {
    Ok(RetirementDemand { copy_bytes: record.next_close_copy_byte_demand()?, capacity_bytes: record.next_close_capacity_byte_demand(body)?, release_bytes: record.next_close_release_byte_demand()?, depth: record.next_close_depth_demand()?.max(1) })
}

fn completed_step<P, Mutation>(record: &mut dyn store::ArtifactEnvelopeCompletedRecord<P, Mutation>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, Fault> {
    let empty = RetainedCloneProgress::default();
    if record.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(empty)); }
    let demand = completed_demand(record, grant.maximum_copy_bytes).map_err(plugin_retirement_fault)?;
    if !plugin_turn_admitted(demand, grant)? { return Ok(RetainedCloneStep::Progress(empty)); }
    let step = record.close_step(grant).map_err(plugin_retirement_fault)?;
    admit_retained_clone_close(grant, step, record.terminal_is_empty(), "envelope completed record").map_err(plugin_retirement_fault)?;
    Ok(step)
}

impl<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static> VcsArtifactApp<A, M> {
    /// 📏️ Quotes the next returned field-decoder turn: the cursor owner's step, else one registry hand-off.
    pub(crate) fn envelope_field_decoder_returns_demands(&self, closing: bool, body: usize) -> Result<RetirementDemand, ValueError> {
        let cursor = if closing { self.close_envelope_field_decoder_cursor } else { self.maintenance_envelope_field_decoder_cursor };
        let Some((_, id)) = self.envelope_field_decoder_retirements.next_id_from(cursor) else { return Ok(handoff_demand()) };
        self.envelope_field_decoder_retirements.get(id).map_or(Ok(handoff_demand()), |retirement| erased_demand(retirement, body))
    }

    /// 🎟️ One granted returned field-decoder turn; `Complete` means no returned decoder is left to hand off.
    pub(crate) fn drive_envelope_field_decoder_returns(&mut self, grant: RetainedCloneGrant, closing: bool) -> Result<PluginLifecycleStep, Fault> {
        let empty = RetainedCloneProgress::default();
        let item = RetainedCloneProgress { copied_items: 1, ..empty };
        if grant.maximum_items == 0 {
            return Ok(PluginLifecycleStep::Progress(empty));
        }
        let cursor = if closing { &mut self.close_envelope_field_decoder_cursor } else { &mut self.maintenance_envelope_field_decoder_cursor };
        if let Some((index, id)) = self.envelope_field_decoder_retirements.next_id_from(*cursor) {
            let retirement = self
                .envelope_field_decoder_retirements
                .get_mut(id)
                .ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-envelope.field-retirement-authority"), "returned envelope field decoder changed during one bounded maintenance step"))?;
            let RetainedCloneStep::Complete(progress) = erased_step(retirement, grant)? else {
                *cursor = index;
                return Ok(PluginLifecycleStep::Progress(empty));
            };
            if !store::ErasedSnapshotRetirement::terminal_is_empty(retirement) {
                return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-envelope.field-retirement-false-terminal"), "returned envelope field decoder reported Complete without terminal-empty ownership"));
            }
            let retirement = self
                .envelope_field_decoder_retirements
                .remove(id)
                .ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-envelope.field-retirement-authority"), "terminal envelope field decoder changed before exact removal"))?;
            drop(retirement);
            *cursor = (index + 1) % ARTIFACT_LIVE_OUTPUT_SLOTS;
            return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress { copied_items: progress.copied_items.max(1), ..progress }));
        }
        let Some(ticket) = self.envelope_field_decoders.next_returned_ticket() else {
            return if closing && !self.envelope_field_decoders.terminal_is_empty() {
                Ok(PluginLifecycleStep::Blocked { reason: "artifact envelope field lease remains live outside the bounded app return pump" })
            } else {
                Ok(PluginLifecycleStep::Complete(empty))
            };
        };
        if !plugin_turn_admitted(handoff_demand(), grant)? {
            return Ok(PluginLifecycleStep::Progress(empty));
        }
        let id = ticket.index() as u64;
        if !self.envelope_field_decoder_retirements.can_insert(id) {
            return Ok(PluginLifecycleStep::Progress(empty));
        }
        let retirement = match self.envelope_field_decoders.take_returned_ticket(ticket) {
            Ok(retirement) => retirement,
            Err(store::ArtifactEnvelopeFieldDecoderRegistryFault::Contended | store::ArtifactEnvelopeFieldDecoderRegistryFault::Returned | store::ArtifactEnvelopeFieldDecoderRegistryFault::Stale) => {
                return Ok(PluginLifecycleStep::Progress(empty));
            }
            Err(_) => {
                return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-envelope.field-return-invalid"), "returned envelope field decoder lost its exact registry generation before bounded handoff"));
            }
        };
        self.envelope_field_decoder_retirements.insert_admitted(id, retirement);
        *cursor = ticket.index();
        Ok(PluginLifecycleStep::Progress(item))
    }

    /// 📏️ Quotes the next completed-record turn: the cursor owner's step, else one registry hand-off.
    pub(crate) fn envelope_completed_record_returns_demands(&self, closing: bool, body: usize) -> Result<RetirementDemand, ValueError> {
        let cursor = if closing { self.close_envelope_completed_record_cursor } else { self.maintenance_envelope_completed_record_cursor };
        let Some((_, id)) = self.envelope_completed_record_retirements.next_id_from(cursor) else { return Ok(handoff_demand()) };
        self.envelope_completed_record_retirements.get(id).map_or(Ok(handoff_demand()), |record| completed_demand(record.as_ref(), body))
    }

    /// 🎟️ One granted completed-record turn; `Complete` means no completed record is left to hand off.
    pub(crate) fn drive_envelope_completed_record_returns(&mut self, grant: RetainedCloneGrant, closing: bool) -> Result<PluginLifecycleStep, Fault> {
        let empty = RetainedCloneProgress::default();
        let item = RetainedCloneProgress { copied_items: 1, ..empty };
        if grant.maximum_items == 0 {
            return Ok(PluginLifecycleStep::Progress(empty));
        }
        let cursor = if closing { &mut self.close_envelope_completed_record_cursor } else { &mut self.maintenance_envelope_completed_record_cursor };
        if let Some((index, id)) = self.envelope_completed_record_retirements.next_id_from(*cursor) {
            let retirement = self
                .envelope_completed_record_retirements
                .get_mut(id)
                .ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-envelope.completed-retirement-authority"), "completed envelope changed during one bounded maintenance step"))?;
            let RetainedCloneStep::Complete(progress) = completed_step(retirement.as_mut(), grant)? else {
                *cursor = index;
                return Ok(PluginLifecycleStep::Progress(empty));
            };
            if !retirement.terminal_is_empty() {
                return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-envelope.completed-retirement-false-terminal"), "completed envelope reported Complete without terminal-empty ownership"));
            }
            let retirement = self
                .envelope_completed_record_retirements
                .remove(id)
                .ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-envelope.completed-retirement-authority"), "terminal completed envelope changed before exact removal"))?;
            drop(retirement);
            *cursor = (index + 1) % ARTIFACT_LIVE_OUTPUT_SLOTS;
            return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress { copied_items: progress.copied_items.max(1), ..progress }));
        }
        let ticket = match if closing { self.envelope_completed_records.try_next_ticket(cursor) } else { self.envelope_completed_records.try_next_close_ticket(cursor) } {
            Ok(ticket) => ticket,
            Err(store::ArtifactEnvelopeCompletedRecordFault::Contended) => return Ok(PluginLifecycleStep::Progress(empty)),
            Err(_) => {
                return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-envelope.completed-registry-invalid"), "completed envelope registry lost its exact generation authority"));
            }
        };
        let Some(ticket) = ticket else {
            return if closing && !self.envelope_completed_records.terminal_is_empty() {
                Ok(PluginLifecycleStep::Blocked { reason: "completed envelope owner remains live outside the bounded app close pump" })
            } else {
                Ok(PluginLifecycleStep::Complete(empty))
            };
        };
        if !plugin_turn_admitted(handoff_demand(), grant)? {
            return Ok(PluginLifecycleStep::Progress(empty));
        }
        let id = ticket
            .generation()
            .checked_mul(ARTIFACT_LIVE_OUTPUT_SLOTS as u64)
            .and_then(|base| base.checked_add(ticket.index() as u64))
            .ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-envelope.completed-generation-exhausted"), "completed envelope retirement identity overflowed"))?;
        if !self.envelope_completed_record_retirements.can_insert(id) {
            return Ok(PluginLifecycleStep::Progress(empty));
        }
        let retirement = match self.envelope_completed_records.try_detach(ticket) {
            Ok(retirement) => retirement,
            Err(store::ArtifactEnvelopeCompletedRecordFault::Contended | store::ArtifactEnvelopeCompletedRecordFault::Stale) => {
                return Ok(PluginLifecycleStep::Progress(empty));
            }
            Err(_) => {
                return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-envelope.completed-detach-invalid"), "completed envelope could not transfer its exact owner into bounded retirement"));
            }
        };
        self.envelope_completed_record_retirements.insert_admitted(id, retirement);
        Ok(PluginLifecycleStep::Progress(item))
    }

    /// 📏️ Quotes the next worker-job turn of the cursor's live envelope decode.
    pub(crate) fn envelope_decode_jobs_demands(&self, closing: bool, body: usize) -> Result<RetirementDemand, ValueError> {
        let cursor = if closing { self.close_envelope_decode_cursor } else { self.maintenance_envelope_decode_cursor };
        let Some((_, operation_id)) = self.envelope_decode_jobs.next_id_from(cursor) else { return Ok(RetirementDemand::default()) };
        self.envelope_decode_jobs.get(operation_id).map_or(Ok(handoff_demand()), |active| active.drive_demands(body))
    }

    /// 🎟️ One granted turn of the cursor's live envelope decode; a terminal decode leaves its fixed slot in the same turn.
    pub(crate) fn drive_envelope_decode_jobs(&mut self, grant: RetainedCloneGrant, closing: bool) -> Result<PluginLifecycleStep, Fault> {
        let empty = RetainedCloneProgress::default();
        let cursor = if closing { &mut self.close_envelope_decode_cursor } else { &mut self.maintenance_envelope_decode_cursor };
        let Some((index, operation_id)) = self.envelope_decode_jobs.next_id_from(*cursor) else {
            return Ok(PluginLifecycleStep::Complete(empty));
        };
        *cursor = (index + 1) % ARTIFACT_LIVE_OUTPUT_SLOTS;
        let pool = semio_framework_async::process_worker_pool(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::InteractiveNative, std::thread::available_parallelism().map_or(1, std::num::NonZeroUsize::get)));
        let live_generation = self
            .envelope_decode_jobs
            .get(operation_id)
            .map(|active| {
                if closing {
                    active.cancel.cancel_now();
                }
                active.generation
            })
            .ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-envelope.maintenance-owner"), "live envelope operation changed during one fixed maintenance step"))?;
        let step = self
            .envelope_decode_jobs
            .get_mut(operation_id)
            .ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-envelope.maintenance-owner"), "live envelope operation changed before worker advancement"))?
            .drive(&pool, live_generation, &self.envelope_completed_records, grant)?;
        #[cfg(target_arch = "wasm32")]
        if let Some(now_ms) = semio_framework_job::default_now_ms() {
            pool.pump(now_ms);
        }
        if self.envelope_decode_jobs.get(operation_id).is_some_and(|active| active.terminal_is_empty(&self.envelope_completed_records)) {
            let active = self.envelope_decode_jobs.remove(operation_id).ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-envelope.maintenance-owner"), "terminal envelope operation changed before exact removal"))?;
            drop(active);
            let progress = match step {
                PluginLifecycleStep::Progress(progress) | PluginLifecycleStep::Complete(progress) => progress,
                PluginLifecycleStep::AwaitingInput { .. } | PluginLifecycleStep::Blocked { .. } => empty,
            };
            return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress { copied_items: progress.copied_items.max(1), ..progress }));
        }
        Ok(step)
    }

    /// 🪹️ No live decode, returned field decoder, or completed record remains.
    pub(crate) fn envelope_decode_terminal_is_empty(&self) -> bool {
        self.envelope_decode_jobs.is_empty()
            && self.envelope_field_decoder_retirements.is_empty()
            && self.envelope_completed_record_retirements.is_empty()
            && self.envelope_field_decoders.terminal_is_empty()
            && self.envelope_completed_records.terminal_is_empty()
    }

    /// 📏️ Quotes the most demanding candidate of the next envelope close turn.
    pub(crate) fn envelope_decode_close_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        Ok(widest(widest(self.envelope_completed_record_returns_demands(true, body)?, self.envelope_field_decoder_returns_demands(true, body)?), self.envelope_decode_jobs_demands(true, body)?))
    }

    /// 🎟️ Advances exactly one envelope owner under the caller's unchanged grant: completed records first, then returned field decoders, then the next live decode.
    pub(crate) fn envelope_decode_close_step(&mut self, grant: RetainedCloneGrant) -> Result<PluginLifecycleStep, Fault> {
        let empty = RetainedCloneProgress::default();
        let mut blocked = None;
        for unit in 0..3 {
            let step = match unit {
                0 => self.drive_envelope_completed_record_returns(grant, true)?,
                1 => self.drive_envelope_field_decoder_returns(grant, true)?,
                _ => self.drive_envelope_decode_jobs(grant, true)?,
            };
            match step {
                PluginLifecycleStep::Progress(progress) if progress != empty => return Ok(step),
                PluginLifecycleStep::Blocked { .. } => blocked = blocked.or(Some(step)),
                _ => {}
            }
        }
        Ok(blocked.unwrap_or(PluginLifecycleStep::Progress(empty)))
    }
}
