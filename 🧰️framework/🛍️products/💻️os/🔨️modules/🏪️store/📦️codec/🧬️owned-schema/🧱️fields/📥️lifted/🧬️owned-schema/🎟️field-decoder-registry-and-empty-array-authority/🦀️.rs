//! 🎟️ Field decoder registry and empty array authority: the history entry authority operations, the field decoder trait, ticket, registry, lease and returned decoder and the empty array authority. See [`super`].

use super::*;

impl<T: FromValue + Send + 'static> ArtifactOwnedHistoryEntryAuthority<T> for ArtifactRepositoryHistoryEntryAuthority<T> {
    fn next_close_copy_byte_demand(&self) -> Result<usize, OwnedSchemaDecodeDiagnostic> { self.close_demands(0).map(|demand| demand.copy_bytes).map_err(|error| self.diagnostic("artifact-envelope.history-entry-copy-demand", 0).with_native(error)) }
    fn next_close_capacity_byte_demand(&self, copy: usize) -> Result<usize, OwnedSchemaDecodeDiagnostic> { self.close_demands(copy).map(|demand| demand.capacity_bytes).map_err(|error| self.diagnostic("artifact-envelope.history-entry-capacity-demand", 0).with_native(error)) }
    fn next_close_release_byte_demand(&self) -> Result<usize, OwnedSchemaDecodeDiagnostic> { self.close_demands(0).map(|demand| demand.release_bytes).map_err(|error| self.diagnostic("artifact-envelope.history-entry-release-demand", 0).with_native(error)) }
    fn next_close_depth_demand(&self) -> Result<usize, OwnedSchemaDecodeDiagnostic> { self.close_demands(0).map(|demand| demand.depth).map_err(|error| self.diagnostic("artifact-envelope.history-entry-depth-demand", 0).with_native(error)) }
    fn accept_token(&mut self, token: OwnedSchemaToken, terminal: bool, source: &OwnedSchemaRecordCursor, cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactEnvelopeFieldDecodeStep, OwnedSchemaDecodeDiagnostic> {
        if cx.operation() != self.operation || cx.generation() != self.generation {
            return Err(self.diagnostic("artifact-envelope.history-entry-stale", token.start));
        }
        if cx.is_cancelled() {
            return Err(self.diagnostic("artifact-envelope.history-entry-cancelled", token.start));
        }
        let path = self.path;
        let diagnostic = |code| OwnedSchemaDecodeDiagnostic { code, offset: token.start, line: 0, column: 0, path , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() };
        let span = usize::try_from(token.end.saturating_sub(token.start)).unwrap_or(usize::MAX);
        let raw = self.raw.as_mut().ok_or(OwnedSchemaDecodeDiagnostic { code: "artifact-envelope.history-entry-closed-source", offset: token.start, line: 0, column: 0, path: self.path , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() })?;
        let Some(end) = self.raw_len.checked_add(span).filter(|end| *end <= raw.len()) else {
            return Err(diagnostic("artifact-envelope.history-entry-byte-capacity"));
        };
        if source.copy_token_bytes(token, 0, &mut raw[self.raw_len..end]) != span {
            return Err(diagnostic("artifact-envelope.history-entry-source"));
        }
        self.raw_len = end;
        cx.consume_fuel(span.max(1) as u64);
        if !terminal {
            return Ok(ArtifactEnvelopeFieldDecodeStep::TokenComplete);
        }
        let text = std::str::from_utf8(&raw[..self.raw_len]).map_err(|_| diagnostic("artifact-envelope.history-entry-decode"))?;
        let value = semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|_| diagnostic("artifact-envelope.history-entry-decode"))?;
        *self.value = Some(value);
        self.raw_len = 0;
        Ok(ArtifactEnvelopeFieldDecodeStep::FieldComplete)
    }

    fn take_value(&mut self) -> Option<T> {
        let value = self.value.take()?;
        self.terminal = true;
        Some(value)
    }

    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, OwnedSchemaDecodeDiagnostic> {
        use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneStep, RetainedCloneProgress, admit_retained_clone_close};
        let empty = RetainedCloneProgress::default();
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(empty)); }
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(empty)); }
        let demand = self.close_demands(grant.maximum_copy_bytes).map_err(|error| (self.diagnostic("artifact-envelope.history-entry-demand", 0)).with_native(error))?;
        if grant.maximum_depth < demand.depth { return Err(self.diagnostic("artifact-envelope.history-entry-depth-refused", 0)); }
        if grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes { return Ok(RetainedCloneStep::Progress(empty)); }
        self.terminal = true;
        if self.retirement.is_some() { return artifact_retirement_box_close_step(&mut self.retirement, grant).map(|step| RetainedCloneStep::Progress(step.progress())).map_err(|error| (self.diagnostic("artifact-envelope.history-entry-retirement-fault", 0)).with_native(error)); }
        if let Some(value) = self.value.take() {
            let child = RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant };
            match self.retirement_factory.as_ref().expect("history value retains its factory").retire_owned(value, child) {
                Ok((owner, progress)) => {
                    *self.retirement = Some(owner);
                    if !progress.fits(child) || progress.retained_capacity_bytes != demand.capacity_bytes { return Err(self.diagnostic("artifact-envelope.history-entry-constructor-receipt", 0).with_progress(progress)); }
                    return Ok(RetainedCloneStep::Progress(progress));
                }
                Err((error, value)) => { *self.value = Some(value); return Err(self.diagnostic("artifact-envelope.history-entry-retirement-admission", 0).with_native(error)); }
            }
        }
        if self.raw_len != 0 { self.raw_len = 0; return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..empty })); }
        if self.raw.is_some() { drop(self.raw.take()); return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: demand.release_bytes, ..empty })); }
        if let Some(factory) = self.retirement_factory.take() {
            let factory: Arc<dyn semio_framework_value::FactoryRetirement> = factory;
            *self.factory_close = Some(semio_framework_value::FactoryAuthority::new(factory));
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..empty }));
        }
        let failure = self.diagnostic("artifact-envelope.history-entry-factory-close", 0);
        let owner = self.factory_close.as_mut().expect("remaining original history factory");
        let child = RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant };
        let step = owner.step(child).map_err(|error| (failure).with_native(error))?;
        let terminal = owner.terminal_is_empty();
        admit_retained_clone_close(child, step, terminal, "original history decoder factory").map_err(|error| (self.diagnostic("artifact-envelope.history-entry-factory-receipt", 0)).with_native(error).with_progress(step.progress()))?;
        if terminal { drop(self.factory_close.take()); }
        Ok(if self.terminal_is_empty() { RetainedCloneStep::Complete(step.progress()) } else { RetainedCloneStep::Progress(step.progress()) })
    }

    fn terminal_is_empty(&self) -> bool {
        self.terminal && self.value.is_none() && self.retirement.is_none() && self.raw_len == 0 && self.raw.is_none() && self.retirement_factory.is_none() && self.factory_close.is_none()
    }
}

impl<T> Drop for ArtifactRepositoryHistoryEntryAuthority<T> {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || (self.terminal && self.value.is_none() && self.retirement.is_none() && self.raw_len == 0 && self.raw.is_none() && self.retirement_factory.is_none() && self.factory_close.is_none()), "repository history entry authority reached Drop before exact terminal ownership");
    }
}

/// 🧬️ Domain-supplied typed field owner used by envelope decode; no serde type crosses this seam.
pub trait ArtifactEnvelopeFieldDecoder<P, Mutation>: Send {
    fn accept_field_token(&mut self, field_id: u16, token: OwnedSchemaToken, terminal: bool, source: &OwnedSchemaRecordCursor, cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactEnvelopeFieldDecodeStep, OwnedSchemaDecodeDiagnostic>;
    fn finish_record(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactEnvelopeFieldDecodeStep, OwnedSchemaDecodeDiagnostic>;
    fn maximum_close_byte_demand(&self) -> usize;
    fn maximum_retained_close_bytes(&self) -> usize;
    fn next_close_copy_byte_demand(&self) -> Result<usize, OwnedSchemaDecodeDiagnostic>;
    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, OwnedSchemaDecodeDiagnostic>;
    fn next_close_release_byte_demand(&self) -> Result<usize, OwnedSchemaDecodeDiagnostic>;
    fn next_close_depth_demand(&self) -> Result<usize, OwnedSchemaDecodeDiagnostic>;
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, OwnedSchemaDecodeDiagnostic>;
    fn terminal_is_empty(&self) -> bool;
}

/// 📏️ Observes every independent demand of one field decoder's next close turn.
pub fn artifact_envelope_field_demands<P, Mutation>(owner: &dyn ArtifactEnvelopeFieldDecoder<P, Mutation>, maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, OwnedSchemaDecodeDiagnostic> {
    if owner.terminal_is_empty() { return Ok(Default::default()); }
    Ok(semio_framework_value::RetirementDemand { copy_bytes: owner.next_close_copy_byte_demand()?, capacity_bytes: owner.next_close_capacity_byte_demand(maximum_copy_bytes)?, release_bytes: owner.next_close_release_byte_demand()?, depth: owner.next_close_depth_demand()? })
}

/// 🎟️ Whether one admitted item, copy, capacity and release grant covers every independent demand axis.
pub fn artifact_retirement_grant_covers(grant: RetainedCloneGrant, demand: semio_framework_value::RetirementDemand) -> bool {
    grant.maximum_items != 0 && grant.maximum_copy_bytes >= demand.copy_bytes && grant.maximum_capacity_bytes >= demand.capacity_bytes && grant.maximum_release_bytes >= demand.release_bytes && grant.maximum_depth >= demand.depth
}

/// 🎟️ Self-funds one turn from its own quoted demand for a caller no scheduler grants.
pub fn artifact_retirement_self_grant(demand: semio_framework_value::RetirementDemand, minimum_copy_bytes: usize) -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes.max(minimum_copy_bytes), maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth.max(1) }
}

/// 🧯️ Converts a decode diagnostic into the value-layer invariant refusal.
pub fn artifact_envelope_diagnostic_error(diagnostic: OwnedSchemaDecodeDiagnostic) -> ValueError {
    ValueError::literal(diagnostic.refusal_kind, diagnostic.code).with_retained_progress(diagnostic.retained_progress)
}

pub const ARTIFACT_ENVELOPE_FIELD_DECODER_CAPACITY: usize = 64;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ArtifactEnvelopeFieldDecoderTicket {
    pub index: u16,
    pub generation: u64,
}

impl ArtifactEnvelopeFieldDecoderTicket {
    pub fn index(self) -> usize {
        usize::from(self.index)
    }

    pub fn generation(self) -> u64 {
        self.generation
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArtifactEnvelopeFieldDecoderRegistryFault {
    Contended,
    Capacity,
    CloseByteCapacity,
    GenerationExhausted,
    Stale,
    Returned,
    FalseTerminal,
}

pub struct ArtifactEnvelopeFieldDecoderSlot<P, Mutation> {
    pub generation: u64,
    pub owner: std::mem::ManuallyDrop<Option<Box<dyn ArtifactEnvelopeFieldDecoder<P, Mutation>>>>,
}

pub struct ArtifactEnvelopeFieldDecoderRegistryState<P, Mutation> {
    pub slots: Box<[ArtifactEnvelopeFieldDecoderSlot<P, Mutation>]>,
    pub free: [u16; ARTIFACT_ENVELOPE_FIELD_DECODER_CAPACITY],
    pub free_len: usize,
}

impl<P, Mutation> Drop for ArtifactEnvelopeFieldDecoderRegistryState<P, Mutation> {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || (self.slots.iter().all(|slot| slot.owner.is_none())), "artifact envelope field registry reached Drop before every exact decoder owner was terminal-empty");
    }
}

/// 🗄️ Fixed app-owned registry that keeps nested field decoders out of stack/job destructors.
pub struct ArtifactEnvelopeFieldDecoderRegistry<P, Mutation> {
    pub state: Mutex<ArtifactEnvelopeFieldDecoderRegistryState<P, Mutation>>,
    pub returned: Box<[std::sync::atomic::AtomicU64]>,
    returned_mask: std::sync::atomic::AtomicU64,
    pub live: std::sync::atomic::AtomicUsize,
    reclaimed: Box<[std::sync::atomic::AtomicU64]>,
}

impl<P, Mutation> ArtifactEnvelopeFieldDecoderRegistry<P, Mutation> {
    /// 🧾️ Quotes the exact final shared issuer and fixed empty backing; outstanding aliases keep custody unchanged.
    pub fn empty_release_demands(owner:&Arc<Self>)->Result<Option<semio_framework_value::RetirementDemand>,ValueError>{
        if Arc::strong_count(owner)!=1 || Arc::weak_count(owner)!=0{return Ok(None)}
        let state=match owner.state.try_lock(){Ok(state)=>state,Err(std::sync::TryLockError::WouldBlock)=>return Ok(None),Err(std::sync::TryLockError::Poisoned(_))=>return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original envelope issuer mutex is poisoned"))};
        if owner.live.load(std::sync::atomic::Ordering::Acquire) != 0 || state.free_len != ARTIFACT_ENVELOPE_FIELD_DECODER_CAPACITY{return Ok(None)}
        let backing=state.slots.len()*std::mem::size_of::<ArtifactEnvelopeFieldDecoderSlot<P,Mutation>>() + (owner.returned.len()+owner.reclaimed.len())*std::mem::size_of::<std::sync::atomic::AtomicU64>();
        Ok(Some(semio_framework_value::RetirementDemand{copy_bytes:std::mem::size_of::<Self>()+backing+2*std::mem::size_of::<Option<Arc<Self>>>(),release_bytes:semio_framework_value::shared_retirement_allocation_bytes::<Self>()+backing,depth:2,..Default::default()}))
    }
    /// ♻️ Releases only this unique empty original issuer after every requested closure currency is admitted.
    pub fn release_empty_original(owner:&mut Option<Arc<Self>>,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        let Some(current)=owner.as_ref()else{return Ok(RetainedCloneStep::Complete(RetainedCloneProgress::default()))};
        let Some(d)=Self::empty_release_demands(current)?else{return Ok(RetainedCloneStep::Progress(Default::default()))};
        if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,d){return Ok(RetainedCloneStep::Progress(Default::default()))}
        drop(owner.take());
        Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,released_bytes:d.release_bytes,..Default::default()}))
    }

    /// 📏️ Declares the actual shared issuer and every fixed backing allocation before original native birth.
    pub fn birth_demands()->semio_framework_value::RetirementDemand{semio_framework_value::RetirementDemand{copy_bytes:std::mem::size_of::<Self>(),capacity_bytes:semio_framework_value::shared_retirement_allocation_bytes::<Self>()+ARTIFACT_ENVELOPE_FIELD_DECODER_CAPACITY * (std::mem::size_of::<ArtifactEnvelopeFieldDecoderSlot<P, Mutation>>() + 2*std::mem::size_of::<std::sync::atomic::AtomicU64>()),depth:2,..Default::default()}}

    /// 🏗️ Preallocates the entire fixed decoder authority before any input owner exists.
    pub fn new() -> Arc<Self> {
        let slots = (0..ARTIFACT_ENVELOPE_FIELD_DECODER_CAPACITY).map(|_| ArtifactEnvelopeFieldDecoderSlot { generation: 0, owner: std::mem::ManuallyDrop::new(None) }).collect::<Vec<_>>().into_boxed_slice();
        let returned = (0..ARTIFACT_ENVELOPE_FIELD_DECODER_CAPACITY).map(|_| std::sync::atomic::AtomicU64::new(0)).collect::<Vec<_>>().into_boxed_slice();
        let reclaimed = (0..ARTIFACT_ENVELOPE_FIELD_DECODER_CAPACITY).map(|_| std::sync::atomic::AtomicU64::new(0)).collect::<Vec<_>>().into_boxed_slice();
        let mut free = [0; ARTIFACT_ENVELOPE_FIELD_DECODER_CAPACITY];
        for (index, value) in free.iter_mut().enumerate() {
            *value = index as u16;
        }
        Arc::new(Self {
            state: Mutex::new(ArtifactEnvelopeFieldDecoderRegistryState { slots, free, free_len: ARTIFACT_ENVELOPE_FIELD_DECODER_CAPACITY }),
            returned,
            returned_mask: std::sync::atomic::AtomicU64::new(0),
            live: std::sync::atomic::AtomicUsize::new(0),
            reclaimed,
        })
    }

    /// 🎫️ Admits one exact decoder or returns it untouched on contention/capacity/exhaustion.
    pub fn try_admit(
        self: &Arc<Self>,
        owner: Box<dyn ArtifactEnvelopeFieldDecoder<P, Mutation>>,
    ) -> Result<ArtifactEnvelopeFieldDecoderLease<P, Mutation>, (ArtifactEnvelopeFieldDecoderRegistryFault, Box<dyn ArtifactEnvelopeFieldDecoder<P, Mutation>>)> {
        let mut state = match self.state.try_lock() {
            Ok(state) => state,
            Err(_) => return Err((ArtifactEnvelopeFieldDecoderRegistryFault::Contended, owner)),
        };
        if state.free_len == 0 {
            return Err((ArtifactEnvelopeFieldDecoderRegistryFault::Capacity, owner));
        }
        state.free_len -= 1;
        let index = usize::from(state.free[state.free_len]);
        let Some(generation) = state.slots[index].generation.checked_add(1) else {
            let free_index = state.free_len;
            state.free[free_index] = index as u16;
            state.free_len += 1;
            return Err((ArtifactEnvelopeFieldDecoderRegistryFault::GenerationExhausted, owner));
        };
        state.slots[index].generation = generation;
        *state.slots[index].owner = Some(owner);
        self.returned[index].store(0, std::sync::atomic::Ordering::Release);
        self.returned_mask.fetch_and(!(1u64 << index), std::sync::atomic::Ordering::AcqRel);
        self.reclaimed[index].store(0, std::sync::atomic::Ordering::Release);
        self.live.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
        Ok(ArtifactEnvelopeFieldDecoderLease { registry: Arc::clone(self), ticket: ArtifactEnvelopeFieldDecoderTicket { index: index as u16, generation }, returned: false })
    }

    pub fn with_owner<R>(&self, ticket: ArtifactEnvelopeFieldDecoderTicket, f: impl FnOnce(&mut dyn ArtifactEnvelopeFieldDecoder<P, Mutation>) -> R) -> Result<R, ArtifactEnvelopeFieldDecoderRegistryFault> {
        let mut state = self.state.try_lock().map_err(|_| ArtifactEnvelopeFieldDecoderRegistryFault::Contended)?;
        let index = usize::from(ticket.index);
        let Some(slot) = state.slots.get_mut(index) else { return Err(ArtifactEnvelopeFieldDecoderRegistryFault::Stale) };
        if slot.generation != ticket.generation {
            return Err(ArtifactEnvelopeFieldDecoderRegistryFault::Stale);
        }
        if self.returned[index].load(std::sync::atomic::Ordering::Acquire) == ticket.generation {
            return Err(ArtifactEnvelopeFieldDecoderRegistryFault::Returned);
        }
        let owner = slot.owner.as_deref_mut().ok_or(ArtifactEnvelopeFieldDecoderRegistryFault::Stale)?;
        Ok(f(owner))
    }

    /// 🧹️ Detaches one exact returned decoder without calling it or dropping it under the registry lock.
    pub fn take_returned_ticket(&self, ticket: ArtifactEnvelopeFieldDecoderTicket) -> Result<ArtifactEnvelopeReturnedFieldDecoder<P, Mutation>, ArtifactEnvelopeFieldDecoderRegistryFault> {
        let owner = {
            let mut state = self.state.try_lock().map_err(|_| ArtifactEnvelopeFieldDecoderRegistryFault::Contended)?;
            let index = usize::from(ticket.index);
            let Some(slot) = state.slots.get_mut(index) else { return Err(ArtifactEnvelopeFieldDecoderRegistryFault::Stale) };
            if slot.generation != ticket.generation {
                return Err(ArtifactEnvelopeFieldDecoderRegistryFault::Stale);
            }
            if self.returned[index].load(std::sync::atomic::Ordering::Acquire) != ticket.generation {
                return Err(ArtifactEnvelopeFieldDecoderRegistryFault::Returned);
            }
            let owner = slot.owner.take().expect("validated returned decoder owner remains present");
            self.returned[index].store(0, std::sync::atomic::Ordering::Release);
            self.returned_mask.fetch_and(!(1u64 << index), std::sync::atomic::Ordering::AcqRel);
            self.reclaimed[index].store(ticket.generation, std::sync::atomic::Ordering::Release);
            self.live.fetch_sub(1, std::sync::atomic::Ordering::AcqRel);
            let free_index = state.free_len;
            state.free[free_index] = ticket.index;
            state.free_len += 1;
            owner
        };
        Ok(ArtifactEnvelopeReturnedFieldDecoder { owner: std::mem::ManuallyDrop::new(Some(owner)) })
    }

    /// 🔎 Returns one exact atomically handed-back ticket without scanning the fixed registry.
    pub fn next_returned_ticket(&self) -> Option<ArtifactEnvelopeFieldDecoderTicket> {
        let mask = self.returned_mask.load(std::sync::atomic::Ordering::Acquire);
        if mask == 0 {
            return None;
        }
        let index = mask.trailing_zeros() as usize;
        let generation = self.returned[index].load(std::sync::atomic::Ordering::Acquire);
        (generation != 0).then_some(ArtifactEnvelopeFieldDecoderTicket { index: index as u16, generation })
    }

    /// ✅ Exact generation witness that the app maintenance owner detached this ticket.
    /// 🪪️ Whether THIS ticket's decoder has been handed back. Monotone by slot generation, so
    /// a later lease reusing the same slot can never make an already-reclaimed ticket read as
    /// outstanding again: an `==` test flipped a finished authority's own `Drop` witness back to
    /// false the moment a sibling authority took the next generation of the same slot, which is
    /// exactly the ABA the per-slot generation counter exists to rule out.
    pub fn ticket_reclaimed(&self, ticket: ArtifactEnvelopeFieldDecoderTicket) -> bool {
        self.reclaimed[ticket.index()].load(std::sync::atomic::Ordering::Acquire) >= ticket.generation
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.live.load(std::sync::atomic::Ordering::Acquire) == 0
    }
}

/// 🪪️ Shallow non-cloneable lease; Drop atomically hands its exact decoder back to the registry.
pub struct ArtifactEnvelopeFieldDecoderLease<P, Mutation> {
    pub registry: Arc<ArtifactEnvelopeFieldDecoderRegistry<P, Mutation>>,
    pub ticket: ArtifactEnvelopeFieldDecoderTicket,
    pub returned: bool,
}

impl<P, Mutation> ArtifactEnvelopeFieldDecoderLease<P, Mutation> {
    pub fn with_owner<R>(&self, f: impl FnOnce(&mut dyn ArtifactEnvelopeFieldDecoder<P, Mutation>) -> R) -> Result<R, ArtifactEnvelopeFieldDecoderRegistryFault> {
        self.registry.with_owner(self.ticket, f)
    }

    pub fn ticket(&self) -> ArtifactEnvelopeFieldDecoderTicket {
        self.ticket
    }

    /// ↩️ Atomically returns the lease exactly once without acquiring or blocking on the owner mutex.
    pub fn return_now(&mut self) -> bool {
        if self.returned {
            return false;
        }
        let index = usize::from(self.ticket.index);
        self.returned = self.registry.returned[index].compare_exchange(0, self.ticket.generation, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire).is_ok();
        if self.returned {
            self.registry.returned_mask.fetch_or(1u64 << index, std::sync::atomic::Ordering::Release);
        }
        self.returned
    }
}

impl<P, Mutation> Drop for ArtifactEnvelopeFieldDecoderLease<P, Mutation> {
    fn drop(&mut self) {
        let _ = self.return_now();
    }
}

/// 🧹️ Detached decoder owner driven outside the registry lock by the bounded maintenance lane.
pub struct ArtifactEnvelopeReturnedFieldDecoder<P, Mutation> {
    pub owner: std::mem::ManuallyDrop<Option<Box<dyn ArtifactEnvelopeFieldDecoder<P, Mutation>>>>,
}

impl<P, Mutation> ErasedSnapshotRetirement for ArtifactEnvelopeReturnedFieldDecoder<P, Mutation>
where
    P: Send,
    Mutation: Send,
{
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let empty = RetainedCloneProgress::default();
        if self.owner.is_none() { return Ok(RetainedCloneStep::Complete(empty)); }
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(empty)); }
        let demand = self.demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth < demand.depth { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "returned artifact envelope field decoder exceeds admitted depth")); }
        if !artifact_retirement_grant_covers(grant, demand) { return Ok(RetainedCloneStep::Progress(empty)); }
        let owner = self.owner.as_mut().expect("returned field decoder remains present");
        if owner.terminal_is_empty() {
            drop(self.owner.take());
            return Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, released_bytes: demand.release_bytes, ..empty }));
        }
        let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant };
        let step = owner.close_step(child).map_err(artifact_envelope_diagnostic_error)?;
        semio_framework_value::retained_clone::admit_retained_clone_close(child, step, owner.terminal_is_empty(), "returned artifact envelope field decoder")?;
        Ok(RetainedCloneStep::Progress(step.progress()))
    }

    fn terminal_is_empty(&self) -> bool {
        self.owner.is_none()
    }

    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.copy_bytes) }
    fn next_capacity_byte_demand(&self, maximum_body_bytes: usize) -> Result<usize, ValueError> { Ok(self.demands(maximum_body_bytes)?.capacity_bytes) }
    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.release_bytes) }
    fn next_depth_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.depth) }
    fn next_demand(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> { self.demands(body) }
}

impl<P, Mutation> ArtifactEnvelopeReturnedFieldDecoder<P, Mutation> {
    /// 📏️ Observes the detached decoder's next turn, or the separate release of its terminal shell.
    pub fn demands(&self, maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        let Some(owner) = self.owner.as_ref() else { return Ok(Default::default()) };
        if owner.terminal_is_empty() { return Ok(semio_framework_value::RetirementDemand { release_bytes: size_of_val(owner.as_ref()), depth: 1, ..Default::default() }); }
        let demand = artifact_envelope_field_demands(owner.as_ref(), maximum_copy_bytes).map_err(artifact_envelope_diagnostic_error)?;
        Ok(semio_framework_value::RetirementDemand { depth: demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "returned artifact envelope field decoder depth overflow"))?, ..demand })
    }
}

impl<P, Mutation> Drop for ArtifactEnvelopeReturnedFieldDecoder<P, Mutation> {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || (self.owner.is_none()), "returned artifact envelope field decoder reached Drop before its terminal-empty witness");
    }
}

impl<P, Mutation> ArtifactEnvelopeFieldDecoderRegistry<P, Mutation>
where
    P: Send,
    Mutation: Send,
{
    /// ↩️ Detaches and bounded-closes one exact returned decoder on the caller thread so a
    /// worker [`release_step`] can pass its `ticket_reclaimed` gate without waiting for the app's
    /// next maintenance visit (ticket 26/09/09/PROCEDURAL-3D-END-TO-END `env-20`).
    pub fn reclaim_returned_ticket_now(&self, ticket: ArtifactEnvelopeFieldDecoderTicket) -> Result<(), ArtifactEnvelopeFieldDecoderRegistryFault> {
        if self.ticket_reclaimed(ticket) {
            return Ok(());
        }
        let mut retired = self.take_returned_ticket(ticket)?;
        for _ in 0..4096 {
            if retired.terminal_is_empty() {
                drop(retired);
                return Ok(());
            }
            let Ok(demand) = retired.demands(ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES) else { return Err(ArtifactEnvelopeFieldDecoderRegistryFault::FalseTerminal) };
            if ErasedSnapshotRetirement::close_step(&mut retired, artifact_retirement_self_grant(demand, ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES)).is_err() { return Err(ArtifactEnvelopeFieldDecoderRegistryFault::FalseTerminal); }
        }
        Err(ArtifactEnvelopeFieldDecoderRegistryFault::FalseTerminal)
    }
}

/// 🧺️ Token-local authority for an exact empty JSON array. The opening token is
/// consumed separately from the terminal token so a surrounding record never replays either one.
pub struct OwnedSchemaEmptyArrayAuthority {
    pub path: OwnedSchemaPath,
    pub opened: bool,
    pub complete: bool,
}

pub const ARTIFACT_ENVELOPE_HISTORY_ITEMS: usize = 64;

pub const ARTIFACT_ENVELOPE_HISTORY_ENTRY_BYTES: usize = ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES;

impl OwnedSchemaEmptyArrayAuthority {
    pub fn new(path: OwnedSchemaPath) -> Self {
        Self { path, opened: false, complete: false }
    }

    pub fn accept(&mut self, token: OwnedSchemaToken, terminal: bool) -> Result<ArtifactEnvelopeFieldDecodeStep, OwnedSchemaDecodeDiagnostic> {
        if self.complete {
            return Err(OwnedSchemaDecodeDiagnostic { code: "schema-json.trailing-empty-array-token", offset: token.start, line: 0, column: 0, path: self.path , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() });
        }
        if !self.opened {
            if token.kind != OwnedSchemaTokenKind::ArrayStart || terminal {
                return Err(OwnedSchemaDecodeDiagnostic { code: "schema-json.expected-empty-array", offset: token.start, line: 0, column: 0, path: self.path , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() });
            }
            self.opened = true;
            return Ok(ArtifactEnvelopeFieldDecodeStep::TokenComplete);
        }
        if token.kind != OwnedSchemaTokenKind::ArrayEnd || !terminal {
            return Err(OwnedSchemaDecodeDiagnostic { code: "schema-json.expected-empty-array-end", offset: token.start, line: 0, column: 0, path: self.path , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() });
        }
        self.complete = true;
        Ok(ArtifactEnvelopeFieldDecodeStep::FieldComplete)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.complete
    }
}

pub const ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES: usize = 4_096;
