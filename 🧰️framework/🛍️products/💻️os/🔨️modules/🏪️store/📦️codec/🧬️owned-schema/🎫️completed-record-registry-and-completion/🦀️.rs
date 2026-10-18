//! 🎫️ Completed record registry and completion: the completed record ticket, fault, target and owner, the registry state and registry and the decode completion. See [`super`].

use super::*;

pub const ARTIFACT_ENVELOPE_COMPLETED_RECORD_CAPACITY: usize = 64;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ArtifactEnvelopeCompletedRecordTicket {
    pub index: u16,
    pub generation: u64,
}

impl ArtifactEnvelopeCompletedRecordTicket {
    pub fn index(self) -> usize {
        usize::from(self.index)
    }

    pub fn generation(self) -> u64 {
        self.generation
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArtifactEnvelopeCompletedRecordFault {
    Contended,
    Capacity,
    GenerationExhausted,
    Stale,
    NotReady,
    FalseTerminal,
}

/// 🎯 Atomic consumer for one completed envelope. Rejection returns the exact record to
/// its app-owned slot; no caller receives a stack-local owner on a fallible branch.
pub trait ArtifactEnvelopeCompletedRecordTarget<P, Mutation> {
    fn try_adopt_completed(&mut self, envelope: ArtifactEnvelope<P, Mutation>) -> Result<(), ArtifactEnvelope<P, Mutation>>;
}

/// 🎁️ Exact completed envelope owner retained by the app until one consumer takes it or close retires it.
pub trait ArtifactEnvelopeCompletedRecord<P, Mutation>: Send {
    fn try_publish_to(&mut self, target: &mut dyn ArtifactEnvelopeCompletedRecordTarget<P, Mutation>) -> bool;
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError>;
    fn next_close_copy_byte_demand(&self) -> Result<usize, ValueError>;
    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, ValueError>;
    fn next_close_release_byte_demand(&self) -> Result<usize, ValueError>;
    fn next_close_depth_demand(&self) -> Result<usize, ValueError>;
    fn terminal_is_empty(&self) -> bool;
}

pub struct ArtifactEnvelopeCompletedRecordOwner<P, Mutation> {
    pub envelope: std::mem::ManuallyDrop<Option<ArtifactEnvelope<P, Mutation>>>,
    pub retirement: std::mem::ManuallyDrop<Option<ArtifactStoreEnvelopeRetirement<P, Mutation>>>,
    pub initial_snapshot_factory: std::mem::ManuallyDrop<Option<Arc<dyn ArtifactOwnedValueRetirementFactory<P>>>>,
    pub mutation_factory: std::mem::ManuallyDrop<Option<Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>>>,
    pub factory_retirement: [Option<semio_framework_value::FactoryAuthority>; 2],
}

impl<P, Mutation> ArtifactEnvelopeCompletedRecordOwner<P, Mutation> {
    pub fn new(envelope: ArtifactEnvelope<P, Mutation>, initial_snapshot_factory: Arc<dyn ArtifactOwnedValueRetirementFactory<P>>, mutation_factory: Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>) -> Self {
        Self { envelope: std::mem::ManuallyDrop::new(Some(envelope)), retirement: std::mem::ManuallyDrop::new(None), initial_snapshot_factory: std::mem::ManuallyDrop::new(Some(initial_snapshot_factory)), mutation_factory: std::mem::ManuallyDrop::new(Some(mutation_factory)), factory_retirement: std::array::from_fn(|_| None) }
    }

    pub fn ownership_is_empty(&self) -> bool {
        self.envelope.is_none() && self.retirement.is_none() && self.initial_snapshot_factory.is_none() && self.mutation_factory.is_none() && self.factory_retirement.iter().all(Option::is_none)
    }
}

impl<P, Mutation> ArtifactEnvelopeCompletedRecord<P, Mutation> for ArtifactEnvelopeCompletedRecordOwner<P, Mutation>
where
    P: Send + Sync + 'static,
    Mutation: Send + 'static,
{
    fn try_publish_to(&mut self, target: &mut dyn ArtifactEnvelopeCompletedRecordTarget<P, Mutation>) -> bool {
        if self.retirement.is_some() || self.initial_snapshot_factory.is_none() || self.mutation_factory.is_none() { return false; }
        let Some(envelope) = self.envelope.take() else { return false };
        match target.try_adopt_completed(envelope) {
            Ok(()) => true,
            Err(envelope) => { *self.envelope = Some(envelope); false }
        }
    }

    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let empty = RetainedCloneProgress::default();
        if self.ownership_is_empty() { return Ok(RetainedCloneStep::Complete(empty)); }
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(empty)); }
        let demand = self.close_demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth < demand.depth { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "completed record exceeds admitted depth")); }
        if grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes { return Ok(RetainedCloneStep::Progress(empty)); }
        if self.retirement.is_some() {
            let step = retirement_owner_close(&mut self.retirement, grant)?;
            return Ok(if self.ownership_is_empty() { RetainedCloneStep::Complete(step.progress()) } else { RetainedCloneStep::Progress(step.progress()) });
        }
        if self.envelope.is_some() {
            let snapshot = self.initial_snapshot_factory.as_ref().ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "completed envelope lost its snapshot factory"))?;
            let mutation = self.mutation_factory.as_ref().ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "completed envelope lost its mutation factory"))?;
            let envelope = self.envelope.take().expect("completed envelope remains retained before original retirement");
            *self.retirement = Some(ArtifactStoreEnvelopeRetirement::new(envelope, Arc::clone(snapshot), Arc::clone(mutation)));
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..empty }));
        }
        if let Some(factory) = self.initial_snapshot_factory.take() {
            let factory: Arc<dyn semio_framework_value::FactoryRetirement> = factory;
            self.factory_retirement[0] = Some(semio_framework_value::FactoryAuthority::new(factory));
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..empty }));
        }
        if let Some(factory) = self.mutation_factory.take() {
            let factory: Arc<dyn semio_framework_value::FactoryRetirement> = factory;
            self.factory_retirement[1] = Some(semio_framework_value::FactoryAuthority::new(factory));
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..empty }));
        }
        for index in 0..self.factory_retirement.len() {
            if let Some(factory) = self.factory_retirement[index].as_mut() {
                let child = RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant };
                let step = factory.step(child)?;
                semio_framework_value::retained_clone::admit_retained_clone_close(child, step, factory.terminal_is_empty(), "completed original factory")?;
                if factory.terminal_is_empty() { self.factory_retirement[index] = None; }
                return Ok(if self.ownership_is_empty() { RetainedCloneStep::Complete(step.progress()) } else { RetainedCloneStep::Progress(step.progress()) });
            }
        }
        Ok(RetainedCloneStep::Complete(empty))
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.close_demands(0)?.copy_bytes) }
    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, ValueError> { Ok(self.close_demands(maximum_copy_bytes)?.capacity_bytes) }
    fn next_close_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.close_demands(0)?.release_bytes) }
    fn next_close_depth_demand(&self) -> Result<usize, ValueError> { Ok(self.close_demands(0)?.depth) }
    fn terminal_is_empty(&self) -> bool { self.ownership_is_empty() }
}

impl<P: Send + Sync + 'static, Mutation: Send + 'static> ArtifactEnvelopeCompletedRecordOwner<P, Mutation> {
    /// 📏️ Quotes the original inline envelope child and separately retained factory capabilities.
    pub fn close_demands(&self, maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        if self.retirement.is_some() { return retirement_owner_demands(&self.retirement, maximum_copy_bytes); }
        if self.envelope.is_some() { return Ok(semio_framework_value::RetirementDemand { depth: 1, ..Default::default() }); }
        if self.initial_snapshot_factory.is_some() || self.mutation_factory.is_some() { return Ok(semio_framework_value::RetirementDemand { copy_bytes: size_of::<Arc<dyn semio_framework_value::FactoryRetirement>>(), depth: 1, ..Default::default() }); }
        for factory in self.factory_retirement.iter().flatten() {
            let mut demand = factory.demands(maximum_copy_bytes)?;
            demand.depth = demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "completed factory depth overflow"))?;
            return Ok(demand);
        }
        Ok(Default::default())
    }
}

impl<P, Mutation> Drop for ArtifactEnvelopeCompletedRecordOwner<P, Mutation> {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.ownership_is_empty(), "completed envelope owner reached Drop before its original record and factories were empty");
    }
}

pub struct ArtifactEnvelopeCompletedRecordSlot<P, Mutation> {
    pub generation: u64,
    pub owner: std::mem::ManuallyDrop<Option<Box<dyn ArtifactEnvelopeCompletedRecord<P, Mutation>>>>,
}

pub struct ArtifactEnvelopeCompletedRecordRegistryState<P, Mutation> {
    pub slots: Box<[ArtifactEnvelopeCompletedRecordSlot<P, Mutation>]>,
    pub free: [u16; ARTIFACT_ENVELOPE_COMPLETED_RECORD_CAPACITY],
    pub free_len: usize,
    pub live: usize,
    occupied: u64,
    pub closing: u64,
}

impl<P, Mutation> Drop for ArtifactEnvelopeCompletedRecordRegistryState<P, Mutation> {
    fn drop(&mut self) {
        assert!(
            std::thread::panicking() || (self.live == 0 && self.occupied == 0 && self.closing == 0 && self.slots.iter().all(|slot| slot.owner.is_none())),
            "completed envelope registry reached Drop before every exact record owner was consumed or cursor-retired"
        );
    }
}

/// 🗄️ Fixed app-owned completed-record registry; result publication never puts a deep envelope on a job stack.
pub struct ArtifactEnvelopeCompletedRecordRegistry<P, Mutation> {
    pub state: Mutex<ArtifactEnvelopeCompletedRecordRegistryState<P, Mutation>>,
}

impl<P, Mutation> ArtifactEnvelopeCompletedRecordRegistry<P, Mutation> {
    pub fn new() -> Arc<Self> {
        let slots = (0..ARTIFACT_ENVELOPE_COMPLETED_RECORD_CAPACITY).map(|_| ArtifactEnvelopeCompletedRecordSlot { generation: 0, owner: std::mem::ManuallyDrop::new(None) }).collect::<Vec<_>>().into_boxed_slice();
        let mut free = [0; ARTIFACT_ENVELOPE_COMPLETED_RECORD_CAPACITY];
        for (index, slot) in free.iter_mut().enumerate() {
            *slot = index as u16;
        }
        Arc::new(Self { state: Mutex::new(ArtifactEnvelopeCompletedRecordRegistryState { slots, free, free_len: ARTIFACT_ENVELOPE_COMPLETED_RECORD_CAPACITY, live: 0, occupied: 0, closing: 0 }) })
    }

    pub fn try_admit(&self, owner: Box<dyn ArtifactEnvelopeCompletedRecord<P, Mutation>>) -> Result<ArtifactEnvelopeCompletedRecordTicket, (ArtifactEnvelopeCompletedRecordFault, Box<dyn ArtifactEnvelopeCompletedRecord<P, Mutation>>)> {
        let mut state = match self.state.try_lock() {
            Ok(state) => state,
            Err(_) => return Err((ArtifactEnvelopeCompletedRecordFault::Contended, owner)),
        };
        if state.free_len == 0 {
            return Err((ArtifactEnvelopeCompletedRecordFault::Capacity, owner));
        }
        state.free_len -= 1;
        let index = usize::from(state.free[state.free_len]);
        let Some(generation) = state.slots[index].generation.checked_add(1) else {
            let free_index = state.free_len;
            state.free[free_index] = index as u16;
            state.free_len += 1;
            return Err((ArtifactEnvelopeCompletedRecordFault::GenerationExhausted, owner));
        };
        state.slots[index].generation = generation;
        *state.slots[index].owner = Some(owner);
        state.live += 1;
        state.occupied |= 1u64 << index;
        Ok(ArtifactEnvelopeCompletedRecordTicket { index: index as u16, generation })
    }

    pub fn validate(state: &ArtifactEnvelopeCompletedRecordRegistryState<P, Mutation>, ticket: ArtifactEnvelopeCompletedRecordTicket) -> Result<usize, ArtifactEnvelopeCompletedRecordFault> {
        let index = ticket.index();
        let Some(slot) = state.slots.get(index) else { return Err(ArtifactEnvelopeCompletedRecordFault::Stale) };
        if slot.generation != ticket.generation || slot.owner.is_none() {
            return Err(ArtifactEnvelopeCompletedRecordFault::Stale);
        }
        Ok(index)
    }

    fn release_slot(state: &mut ArtifactEnvelopeCompletedRecordRegistryState<P, Mutation>, index: usize) {
        let free_index = state.free_len;
        state.free[free_index] = index as u16;
        state.free_len += 1;
        state.live -= 1;
        state.occupied &= !(1u64 << index);
        state.closing &= !(1u64 << index);
    }

    pub fn try_publish_to(&self, ticket: ArtifactEnvelopeCompletedRecordTicket, target: &mut dyn ArtifactEnvelopeCompletedRecordTarget<P, Mutation>) -> Result<bool, ArtifactEnvelopeCompletedRecordFault> {
        let mut state = self.state.try_lock().map_err(|_| ArtifactEnvelopeCompletedRecordFault::Contended)?;
        let index = Self::validate(&state, ticket)?;
        if state.closing & (1u64 << index) != 0 { return Err(ArtifactEnvelopeCompletedRecordFault::Stale); }
        let published = state.slots[index].owner.as_mut().is_some_and(|owner| owner.try_publish_to(target));
        if !published { return Ok(false); }
        state.closing |= 1u64 << index;
        Ok(true)
    }

    pub fn try_detach(&self, ticket: ArtifactEnvelopeCompletedRecordTicket) -> Result<Box<dyn ArtifactEnvelopeCompletedRecord<P, Mutation>>, ArtifactEnvelopeCompletedRecordFault> {
        let mut state = self.state.try_lock().map_err(|_| ArtifactEnvelopeCompletedRecordFault::Contended)?;
        let index = Self::validate(&state, ticket)?;
        let owner = state.slots[index].owner.take().expect("validated completed record remains established");
        Self::release_slot(&mut state, index);
        Ok(owner)
    }

    /// 🚫 Marks a rejected/cancelled consumer result for the app maintenance close pump.
    pub fn try_request_close(&self, ticket: ArtifactEnvelopeCompletedRecordTicket) -> Result<(), ArtifactEnvelopeCompletedRecordFault> {
        let mut state = self.state.try_lock().map_err(|_| ArtifactEnvelopeCompletedRecordFault::Contended)?;
        let index = Self::validate(&state, ticket)?;
        state.closing |= 1u64 << index;
        Ok(())
    }

    pub fn try_next_ticket(&self, cursor: &mut usize) -> Result<Option<ArtifactEnvelopeCompletedRecordTicket>, ArtifactEnvelopeCompletedRecordFault> {
        let state = self.state.try_lock().map_err(|_| ArtifactEnvelopeCompletedRecordFault::Contended)?;
        if state.live == 0 {
            *cursor = 0;
            return Ok(None);
        }
        let start = *cursor % ARTIFACT_ENVELOPE_COMPLETED_RECORD_CAPACITY;
        let offset = state.occupied.rotate_right(start as u32).trailing_zeros() as usize;
        let index = (start + offset) % ARTIFACT_ENVELOPE_COMPLETED_RECORD_CAPACITY;
        *cursor = (index + 1) % ARTIFACT_ENVELOPE_COMPLETED_RECORD_CAPACITY;
        Ok(Some(ArtifactEnvelopeCompletedRecordTicket { index: index as u16, generation: state.slots[index].generation }))
    }

    /// 🧹 Returns only records explicitly handed back by a cancelled/rejected consumer.
    pub fn try_next_close_ticket(&self, cursor: &mut usize) -> Result<Option<ArtifactEnvelopeCompletedRecordTicket>, ArtifactEnvelopeCompletedRecordFault> {
        let state = self.state.try_lock().map_err(|_| ArtifactEnvelopeCompletedRecordFault::Contended)?;
        if state.closing == 0 {
            *cursor = 0;
            return Ok(None);
        }
        let start = *cursor % ARTIFACT_ENVELOPE_COMPLETED_RECORD_CAPACITY;
        let offset = state.closing.rotate_right(start as u32).trailing_zeros() as usize;
        let index = (start + offset) % ARTIFACT_ENVELOPE_COMPLETED_RECORD_CAPACITY;
        *cursor = (index + 1) % ARTIFACT_ENVELOPE_COMPLETED_RECORD_CAPACITY;
        Ok(Some(ArtifactEnvelopeCompletedRecordTicket { index: index as u16, generation: state.slots[index].generation }))
    }

    pub fn try_detach_next(&self, cursor: &mut usize) -> Result<Option<Box<dyn ArtifactEnvelopeCompletedRecord<P, Mutation>>>, ArtifactEnvelopeCompletedRecordFault> {
        let mut state = self.state.try_lock().map_err(|_| ArtifactEnvelopeCompletedRecordFault::Contended)?;
        if state.live == 0 {
            *cursor = 0;
            return Ok(None);
        }
        let start = *cursor % ARTIFACT_ENVELOPE_COMPLETED_RECORD_CAPACITY;
        let offset = state.occupied.rotate_right(start as u32).trailing_zeros() as usize;
        let index = (start + offset) % ARTIFACT_ENVELOPE_COMPLETED_RECORD_CAPACITY;
        *cursor = (index + 1) % ARTIFACT_ENVELOPE_COMPLETED_RECORD_CAPACITY;
        let owner = state.slots[index].owner.take().expect("occupied completed-record bit retains exact owner");
        Self::release_slot(&mut state, index);
        Ok(Some(owner))
    }

    pub fn ticket_reclaimed(&self, ticket: ArtifactEnvelopeCompletedRecordTicket) -> bool {
        self.state.try_lock().is_ok_and(|state| Self::validate(&state, ticket).is_err())
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.state.try_lock().is_ok_and(|state| state.live == 0)
    }
}

/// 🎫️ Shallow publication cell for the exact completed-record ticket produced by one decode job.
pub struct ArtifactEnvelopeDecodeCompletion {
    pub index: std::sync::atomic::AtomicU16,
    pub generation: std::sync::atomic::AtomicU64,
}

impl ArtifactEnvelopeDecodeCompletion {
    pub fn new() -> Arc<Self> {
        Arc::new(Self { index: std::sync::atomic::AtomicU16::new(u16::MAX), generation: std::sync::atomic::AtomicU64::new(0) })
    }

    pub fn publish(&self, ticket: ArtifactEnvelopeCompletedRecordTicket) -> Result<(), ArtifactEnvelopeCompletedRecordTicket> {
        if self.generation.compare_exchange(0, ticket.generation, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire).is_err() {
            return Err(ticket);
        }
        self.index.store(ticket.index, std::sync::atomic::Ordering::Release);
        Ok(())
    }

    pub fn ticket(&self) -> Option<ArtifactEnvelopeCompletedRecordTicket> {
        let index = self.index.load(std::sync::atomic::Ordering::Acquire);
        let generation = self.generation.load(std::sync::atomic::Ordering::Acquire);
        (index != u16::MAX && generation != 0).then_some(ArtifactEnvelopeCompletedRecordTicket { index, generation })
    }
}
