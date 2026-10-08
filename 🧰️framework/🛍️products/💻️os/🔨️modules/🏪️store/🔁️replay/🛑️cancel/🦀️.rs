//! 🛑️ Cancellation retains actual replay state until independently granted owners physically close.
use super::*;
use semio_framework_value::{FactoryAuthority, RetirementDemand, ValueRefusalKind, retirement::controlled::ControlledRetirement};

impl<P, M: Mutation<P>> EditReplay<P, M> {
    pub fn begin_retirement(&mut self) { self.retiring = true; }
    pub fn cancel(&mut self) { self.begin_retirement(); }
    fn originals_empty(&self) -> bool {
        let s = &*self.original;
        s.order.capacity() == 0 && s.schema.capacity() == 0 && s.supersessions.terminal_is_empty() && s.state.is_none() && s.candidate.is_none() && s.edit_messages.terminal_is_empty() && s.edit_inverse.terminal_is_empty() && s.message_settlement.is_none() && s.committed.capacity() == 0 && s.quarantined.capacity() == 0 && s.rebased_inverse.capacity() == 0 && s.replayed.capacity() == 0 && s.outcomes.capacity() == 0 && s.unit_flags.terminal_is_empty() && s.operation_preparation_factory.is_none() && s.operation_preparation.is_none() && s.prepared_operation.is_none() && s.operation_retirement.is_none() && s.operation_message_settlement.is_none() && s.replay_retirement_factory.is_none() && s.replay_retirement.is_none() && s.prefix_message_retirement.is_none() && s.convergence.is_none() && s.recorded.capacity() == 0 && s.drafts.terminal_is_empty() && self.raw_retirement.is_none() && self.raw_factory.is_none()
    }
}

fn unsupported(message: &'static str) -> ValueError { ValueError::literal(ValueRefusalKind::UnsupportedOwner, message) }
fn nested(mut demand: RetirementDemand) -> Result<RetirementDemand, ValueError> { demand.depth = demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(ValueRefusalKind::DepthLimit, "raw replay original child depth overflow"))?; Ok(demand) }
fn birth<T>() -> RetirementDemand { RetirementDemand { capacity_bytes: std::mem::size_of::<T>(), depth: 2, ..Default::default() } }
fn typed_birth<T: semio_framework_value::retirement::RetireOwned>() -> Result<RetirementDemand, ValueError> { if !T::controlled_retirement_supported() { return Err(unsupported("raw replay native metadata has no controlled retirement authority")); } Ok(birth::<ControlledRetirement<T>>()) }
fn admit_typed<T: semio_framework_value::retirement::RetireOwned + Default + Send + 'static>(original: &mut T, slot: &mut Option<Box<dyn ErasedSnapshotRetirement>>, grant: RetainedCloneGrant) -> Result<RetainedCloneProgress, ValueError> {
    typed_birth::<T>()?;
    match admit_artifact_retirement(std::mem::take(original), grant, |owner| ControlledRetirement::new(owner).unwrap_or_else(|_| unreachable!("quoted native retirement authority"))) {
        Ok((owner, progress)) => { *slot = Some(owner); Ok(progress) }
        Err((error, owner)) => { *original = owner; Err(error) }
    }
}

impl<P: Send + Sync + 'static, M: Mutation<P> + Send + 'static> EditReplay<P, M> {
    pub fn retirement_demands(&self, copy: usize) -> Result<RetirementDemand, ValueError> {
        if let Some(owner) = self.raw_retirement.as_ref() { return nested(artifact_retirement_box_demands(owner, copy)?); }
        if let Some(owner) = self.raw_factory.as_ref() { return nested(owner.demands(copy)?); }
        let s = &*self.original;
        macro_rules! child { ($owner:expr) => {{ let owner = $owner; return nested(RetirementDemand { copy_bytes: owner.next_copy_byte_demand()?, capacity_bytes: owner.next_capacity_byte_demand(copy)?, release_bytes: owner.next_release_byte_demand()?, depth: owner.next_depth_demand()? }); }}; }
        if let Some(owner) = s.replay_retirement.as_ref() { child!(owner); }
        if let Some(owner) = s.prefix_message_retirement.as_ref() { return nested(RetirementDemand { copy_bytes: owner.next_copy_byte_demand(), capacity_bytes: 0, release_bytes: owner.next_release_byte_demand()?, depth: owner.next_depth_demand()? }); }
        if s.operation_preparation.is_some() { return Ok(RetirementDemand { depth: 1, ..Default::default() }); }
        if let Some(owner) = s.operation_retirement.as_ref() { child!(owner); }
        if let Some(owner) = s.message_settlement.as_ref() { return nested(RetirementDemand { copy_bytes: owner.next_close_copy_byte_demand()?, capacity_bytes: 0, release_bytes: owner.next_close_release_byte_demand()?, depth: owner.next_close_depth_demand()? }); }
        if let Some(owner) = s.operation_message_settlement.as_ref() { return nested(RetirementDemand { copy_bytes: owner.next_copy_byte_demand()?, capacity_bytes: owner.next_capacity_byte_demand(copy)?, release_bytes: owner.next_release_byte_demand()?, depth: owner.next_depth_demand()? }); }
        if !s.edit_messages.terminal_is_empty() { return nested(RetirementDemand { copy_bytes: s.edit_messages.next_close_copy_byte_demand(), release_bytes: s.edit_messages.next_close_release_byte_demand()?, depth: 1, ..Default::default() }); }
        if let Some(owner) = s.prepared_operation.as_ref() { return if prepared::terminal(owner) { Ok(RetirementDemand { depth: 1, ..Default::default() }) } else { nested(prepared::demands(owner, s.replay_retirement_factory.as_deref())?) }; }
        if s.state.is_some() || s.candidate.is_some() || !s.recorded.is_empty() || s.convergence.as_ref().is_some_and(|owner| !owner.checkpoints.is_empty()) { return Ok(RetirementDemand { capacity_bytes: s.replay_retirement_factory.as_ref().ok_or_else(|| unsupported("raw replay snapshot retains its original without an installed issuer"))?.snapshot_birth_bytes(), depth: 2, ..Default::default() }); }
        if !s.edit_inverse.terminal_is_empty() { return Ok(RetirementDemand { capacity_bytes: s.replay_retirement_factory.as_ref().ok_or_else(|| unsupported("raw replay inverse retains its original without an installed issuer"))?.mutations_birth_bytes(), depth: 2, ..Default::default() }); }
        if s.schema.capacity() != 0 { return typed_birth::<String>(); }
        if !s.rebased_inverse.is_empty() || !s.outcomes.is_empty() { return Ok(RetirementDemand { depth: 1, ..Default::default() }); }
        if s.committed.capacity() != 0 || s.quarantined.capacity() != 0 { return typed_birth::<Vec<String>>(); }
        if s.order.capacity() != 0 { return Ok(birth::<ArtifactStoreStringVectorRetirement>()); }
        if !s.supersessions.terminal_is_empty() { return Ok(birth::<ArtifactStoreSupersessionRetirement>()); }
        if !s.drafts.terminal_is_empty() { return typed_birth::<protocol::HistoryInputDrafts>(); }
        if !s.unit_flags.terminal_is_empty() { return typed_birth::<protocol::HistoryFoldIndex<([u8; 32], usize), bool>>(); }
        if s.replayed.capacity() != 0 { return Ok(birth::<crate::os_spr::command::EditMessageLedgerRetirement>()); }
        let release = if s.rebased_inverse.capacity() != 0 { s.rebased_inverse.capacity() * std::mem::size_of::<(String, semio_framework_value::list::PagedList<M, {usize::MAX}>)>() }
        else if s.outcomes.capacity() != 0 { s.outcomes.capacity() * std::mem::size_of::<protocol::MutationReplayOutcome>() }
        else if s.recorded.capacity() != 0 { s.recorded.capacity() * std::mem::size_of::<(usize, Arc<P>)>() }
        else if let Some(owner) = s.convergence.as_ref() { owner.checkpoints.capacity() * std::mem::size_of::<(usize, Arc<P>)>() } else { 0 };
        if release != 0 || s.convergence.is_some() { return Ok(RetirementDemand { release_bytes: release, depth: 1, ..Default::default() }); }
        if s.operation_preparation_factory.is_some() || s.replay_retirement_factory.is_some() { return Ok(RetirementDemand { depth: 1, ..Default::default() }); }
        Ok(Default::default())
    }

    pub fn close_original_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let idle = RetainedCloneProgress::default();
        if !self.retiring || grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(idle)); }
        if self.originals_empty() { return Ok(RetainedCloneStep::Complete(idle)); }
        let demand = self.retirement_demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth < demand.depth || grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes { return Ok(RetainedCloneStep::Progress(idle)); }
        let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth.saturating_sub(1), ..grant };
        if self.raw_retirement.is_some() { let step = artifact_retirement_box_close_step(&mut self.raw_retirement, child)?; return Ok(RetainedCloneStep::Progress(step.progress())); }
        if let Some(owner) = self.raw_factory.as_mut() { let step = owner.step(child)?; semio_framework_value::retained_clone::admit_retained_clone_close(child, step, owner.terminal_is_empty(), "raw replay original factory")?; if owner.terminal_is_empty() { self.raw_factory.take(); } return Ok(RetainedCloneStep::Progress(step.progress())); }
        let s = &mut *self.original;
        macro_rules! close_child { ($slot:expr, $call:expr) => {{ let step = $call?; let terminal = $slot.as_ref().is_some_and(|owner| owner.terminal_is_empty()); semio_framework_value::retained_clone::admit_retained_clone_close(child, step, terminal, "raw replay retained child")?; if terminal { $slot = None; } return Ok(RetainedCloneStep::Progress(step.progress())); }}; }
        if let Some(owner) = s.replay_retirement.as_mut() { close_child!(s.replay_retirement, owner.close_step(child)); }
        if let Some(owner) = s.prefix_message_retirement.as_mut() { close_child!(s.prefix_message_retirement, owner.close_step(child)); }
        if let Some(owner) = s.operation_preparation.take() { s.operation_retirement = Some(owner.into_retirement()); return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..idle })); }
        if let Some(owner) = s.operation_retirement.as_mut() { close_child!(s.operation_retirement, owner.close_step(child)); }
        if let Some(owner) = s.message_settlement.as_mut() { owner.begin_close(); close_child!(s.message_settlement, owner.close_step(child)); }
        if let Some(owner) = s.operation_message_settlement.as_mut() { owner.begin_close(); close_child!(s.operation_message_settlement, owner.close_step(child)); }
        if !s.edit_messages.terminal_is_empty() { return Ok(RetainedCloneStep::Progress(s.edit_messages.close_step(child)?)); }
        if let Some(owner) = s.prepared_operation.as_mut() {
            if prepared::terminal(owner) { s.prepared_operation = None; return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..idle })); }
            let step = prepared::close(owner, &mut self.raw_retirement, s.replay_retirement_factory.as_deref(), child)?;
            semio_framework_value::retained_clone::admit_retained_clone_close(child, step, prepared::terminal(owner), "raw replay original prepared fields")?;
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        let snapshot = if let Some(owner) = s.state.take() { Some((0, 0, owner)) } else if let Some(owner) = s.candidate.take() { Some((1, 0, owner)) } else if let Some((position, owner)) = s.recorded.pop() { Some((2, position, owner)) } else { s.convergence.as_mut().and_then(|owner| owner.checkpoints.pop()).map(|(position, owner)| (3, position, owner)) };
        if let Some((lane, position, owner)) = snapshot {
            match s.replay_retirement_factory.as_ref().expect("quoted original snapshot issuer").snapshot(owner, child) {
                Ok((owner, progress)) => { *self.raw_retirement = Some(owner); return Ok(RetainedCloneStep::Progress(progress)); }
                Err((error, owner)) => { match lane { 0 => s.state = Some(owner), 1 => s.candidate = Some(owner), 2 => s.recorded.push((position, owner)), _ => s.convergence.as_mut().unwrap().checkpoints.push((position, owner)) }; return Err(error); }
            }
        }
        if !s.edit_inverse.terminal_is_empty() {
            match s.replay_retirement_factory.as_ref().expect("quoted original mutation issuer").mutations(std::mem::take(&mut s.edit_inverse), child) {
                Ok((owner, progress)) => { *self.raw_retirement = Some(owner); return Ok(RetainedCloneStep::Progress(progress)); }
                Err((error, original)) => { s.edit_inverse = original; return Err(error); }
            }
        }
        if s.schema.capacity() != 0 { return admit_typed(&mut s.schema, &mut self.raw_retirement, child).map(RetainedCloneStep::Progress); }
        if let Some((id, inverse)) = s.rebased_inverse.pop() { s.schema = id; s.edit_inverse = inverse; return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..idle })); }
        if let Some(outcome) = s.outcomes.pop() { s.prefix_message_retirement = Some(ArtifactStoreMessageLedgerRetirement::from_replay_outcome(outcome)); return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..idle })); }
        let progress = if s.schema.capacity() != 0 { admit_typed(&mut s.schema, &mut self.raw_retirement, child)? }
        else if s.committed.capacity() != 0 { admit_typed(&mut s.committed, &mut self.raw_retirement, child)? }
        else if s.quarantined.capacity() != 0 { admit_typed(&mut s.quarantined, &mut self.raw_retirement, child)? }
        else if s.order.capacity() != 0 {
            match admit_artifact_retirement(std::mem::replace(&mut s.order, crate::os_vcs::HistoryPageStack::empty()), child, ArtifactStoreStringVectorRetirement::new) { Ok((owner, progress)) => { *self.raw_retirement = Some(owner); progress }, Err((error, original)) => { s.order = original; return Err(error); } }
        } else if !s.supersessions.terminal_is_empty() {
            match admit_artifact_retirement(std::mem::take(&mut s.supersessions), child, ArtifactStoreSupersessionRetirement::new) { Ok((owner, progress)) => { *self.raw_retirement = Some(owner); progress }, Err((error, original)) => { s.supersessions = original; return Err(error); } }
        } else if !s.drafts.terminal_is_empty() { admit_typed(&mut s.drafts, &mut self.raw_retirement, child)? }
        else if !s.unit_flags.terminal_is_empty() { admit_typed(&mut s.unit_flags, &mut self.raw_retirement, child)? }
        else if s.replayed.capacity() != 0 {
            match admit_artifact_retirement(std::mem::take(&mut s.replayed), child, crate::os_spr::command::EditMessageLedgerRetirement::new) { Ok((owner, progress)) => { *self.raw_retirement = Some(owner); progress }, Err((error, original)) => { s.replayed = original; return Err(error); } }
        } else if s.rebased_inverse.capacity() != 0 { drop(std::mem::take(&mut s.rebased_inverse)); RetainedCloneProgress { copied_items: 1, released_bytes: demand.release_bytes, ..idle } }
        else if s.outcomes.capacity() != 0 { drop(std::mem::take(&mut s.outcomes)); RetainedCloneProgress { copied_items: 1, released_bytes: demand.release_bytes, ..idle } }
        else if s.recorded.capacity() != 0 { drop(std::mem::take(&mut s.recorded)); RetainedCloneProgress { copied_items: 1, released_bytes: demand.release_bytes, ..idle } }
        else if s.convergence.is_some() { s.convergence.take(); RetainedCloneProgress { copied_items: 1, released_bytes: demand.release_bytes, ..idle } }
        else {
            let factory: Arc<dyn semio_framework_value::FactoryRetirement> = if let Some(owner) = s.operation_preparation_factory.take() { owner } else { s.replay_retirement_factory.take().expect("quoted original replay issuer") };
            *self.raw_factory = Some(FactoryAuthority::new(factory)); RetainedCloneProgress { copied_items: 1, ..idle }
        };
        if !progress.fits(grant) { return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "raw replay original close exceeded its unchanged grant")); }
        Ok(RetainedCloneStep::Progress(progress))
    }
}

impl<P, M: Mutation<P>> Drop for EditReplay<P, M> {
    fn drop(&mut self) { let empty = self.originals_empty(); assert!(empty || std::thread::panicking(), "raw replay reached Drop while original custody remains attached"); if empty { unsafe { std::mem::ManuallyDrop::drop(&mut self.original); std::mem::ManuallyDrop::drop(&mut self.raw_retirement); std::mem::ManuallyDrop::drop(&mut self.raw_factory); } } }
}

impl<P, M: Mutation<P>> EditReplay<P, M> {
    pub fn terminal_is_empty(&self) -> bool { self.originals_empty() }
}

impl<P: Send + Sync + 'static, M: Mutation<P> + Send + 'static> ErasedSnapshotRetirement for EditReplay<P, M> {
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> { self.close_original_step(grant) }
    fn terminal_is_empty(&self) -> bool { self.originals_empty() }
    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.retirement_demands(0)?.copy_bytes) }
    fn next_capacity_byte_demand(&self, copy: usize) -> Result<usize, ValueError> { Ok(self.retirement_demands(copy)?.capacity_bytes) }
    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.retirement_demands(0)?.release_bytes) }
    fn next_depth_demand(&self) -> Result<usize, ValueError> { Ok(self.retirement_demands(0)?.depth) }
}

impl<P, M: Mutation<P>> EditReplayResult<P, M> {
    fn outputs_empty(&self) -> bool {
        let s = &*self.original;
        s.order.capacity() == 0 && s.supersessions.terminal_is_empty() && s.drafts.terminal_is_empty() && s.state.is_none()
            && s.committed.capacity() == 0 && s.quarantined.capacity() == 0 && s.rebased_inverse.capacity() == 0
            && s.replayed.capacity() == 0 && s.recorded.capacity() == 0 && s.report.outcomes.capacity() == 0 && s.unit_flags.terminal_is_empty()
    }
    pub fn begin_retirement(&mut self) { if let Some(owner) = self.residual.as_mut() { owner.begin_retirement(); } }
    pub fn terminal_is_empty(&self) -> bool { self.outputs_empty() && self.residual.is_none() }
}

impl<P: Send + Sync + 'static, M: Mutation<P> + Send + 'static> EditReplayResult<P, M> {
    pub fn retirement_demands(&self, copy: usize) -> Result<RetirementDemand, ValueError> {
        if !self.outputs_empty() { return Ok(RetirementDemand { depth: 1, ..Default::default() }); }
        self.residual.as_ref().map_or(Ok(Default::default()), |owner| if owner.originals_empty() { Ok(RetirementDemand { depth: 1, ..Default::default() }) } else { nested(owner.retirement_demands(copy)?) })
    }
    pub fn close_original_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let idle = RetainedCloneProgress::default();
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(idle)); }
        let Some(owner) = self.residual.as_mut() else { return Err(unsupported("finished replay preserves untaken outputs without its original residual")); };
        if !owner.retiring || grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(idle)); }
        let demand = self.retirement_demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth < demand.depth || grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes { return Ok(RetainedCloneStep::Progress(idle)); }
        if !self.outputs_empty() {
            let s = &mut *self.original;
            let r = &mut *self.residual.as_mut().unwrap().original;
            if r.order.capacity() != 0 || !r.supersessions.terminal_is_empty() || !r.drafts.terminal_is_empty() || r.state.is_some()
                || r.committed.capacity() != 0 || r.quarantined.capacity() != 0 || r.rebased_inverse.capacity() != 0
                || r.replayed.capacity() != 0 || r.recorded.capacity() != 0 || r.outcomes.capacity() != 0 || !r.unit_flags.terminal_is_empty() {
                return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "finished replay original output slots must stay empty until residual handoff"));
            }
            r.order = std::mem::replace(&mut s.order, crate::os_vcs::HistoryPageStack::empty());
            r.supersessions = std::mem::take(&mut s.supersessions);
            r.drafts = std::mem::take(&mut s.drafts);
            r.state = s.state.take();
            r.committed = std::mem::take(&mut s.committed);
            r.quarantined = std::mem::take(&mut s.quarantined);
            r.rebased_inverse = std::mem::take(&mut s.rebased_inverse);
            r.replayed = std::mem::take(&mut s.replayed);
            r.recorded = std::mem::take(&mut s.recorded);
            r.outcomes = std::mem::take(&mut s.report.outcomes);
            r.unit_flags = std::mem::take(&mut s.unit_flags);
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..idle }));
        }
        let owner = self.residual.as_mut().unwrap();
        if owner.originals_empty() { self.residual.take(); return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..idle })); }
        let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant };
        let step = owner.close_original_step(child)?;
        semio_framework_value::retained_clone::admit_retained_clone_close(child, step, owner.originals_empty(), "finished replay original residual")?;
        Ok(RetainedCloneStep::Progress(step.progress()))
    }
}

impl<P: Send + Sync + 'static, M: Mutation<P> + Send + 'static> ErasedSnapshotRetirement for EditReplayResult<P, M> {
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> { self.close_original_step(grant) }
    fn terminal_is_empty(&self) -> bool { EditReplayResult::terminal_is_empty(self) }
    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.retirement_demands(0)?.copy_bytes) }
    fn next_capacity_byte_demand(&self, copy: usize) -> Result<usize, ValueError> { Ok(self.retirement_demands(copy)?.capacity_bytes) }
    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.retirement_demands(0)?.release_bytes) }
    fn next_depth_demand(&self) -> Result<usize, ValueError> { Ok(self.retirement_demands(0)?.depth) }
}

impl<P, M: Mutation<P>> Drop for EditReplayResult<P, M> {
    fn drop(&mut self) {
        let empty = self.terminal_is_empty();
        assert!(empty || std::thread::panicking(), "finished replay reached Drop while untaken outputs or original residual custody remain");
        if empty { unsafe { std::mem::ManuallyDrop::drop(&mut self.original); std::mem::ManuallyDrop::drop(&mut self.residual); } }
    }
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
pub(crate) mod tests;

use crate::os_spr::command::prepared_retirement as prepared;
