//! 🏛️ Bounded ownership primitives: lifecycle steps, fixed queues and registries, and the grant and retirement helpers every owner shares.
use semio_framework::Fault;
use semio_framework_value::{RetirementDemand, ValueError, retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep}};
use semio_framework_value::retirement::controlled::ControlledRetirement;
use crate::sdk_fault::plugin_sdk_fault;

#[path="⚠️failure/🦀️.rs"]
mod original_failure;
pub use original_failure::{PluginLifecycleFailure,PluginLifecyclePrimary};
pub use original_failure::{original_lifecycle_grant,original_lifecycle_step,original_lifecycle_delta,original_lifecycle_unposted};



/// 💰️ The close caller admits the exact physical release against the declared store allocation ceiling.
pub fn artifact_close_release_grant(demand: usize, ordinary_bytes: usize) -> Result<usize, Fault> {
    if demand > store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_CLOSE_ALLOCATION_BYTES {
        return Err(plugin_sdk_fault("artifact close physical allocation exceeds its declared admission"));
    }
    Ok(ordinary_bytes.max(demand))
}

/// 🎟️ Self-funds one turn of a caller no scheduler grants, from the owner's own quote: the copy axis settles on
/// the smallest body at or above `body` whose quoted copy demand it covers, and every other axis is the quoted demand.
fn retirement_self_grant(quote: impl Fn(usize) -> Result<RetirementDemand, ValueError>, body: usize) -> Result<RetainedCloneGrant, ValueError> {
    let mut copy = body;
    for _ in 0..4 {
        let demand = quote(copy)?;
        if demand.copy_bytes <= copy {
            return Ok(RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: copy, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth.max(1) });
        }
        copy = demand.copy_bytes;
    }
    Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "retirement quote did not settle on a stable copy grant"))
}

/// 🎟️ Exact physical lifecycle receipts retain every independent grant currency.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PluginLifecycleStep {
    Progress(RetainedCloneProgress),
    AwaitingInput { reason: &'static str },
    Blocked { reason: &'static str },
    Complete(RetainedCloneProgress),
}

impl PluginLifecycleStep {
    pub fn progress(self)->Option<RetainedCloneProgress>{match self{Self::Progress(progress)|Self::Complete(progress)=>Some(progress),Self::AwaitingInput{..}|Self::Blocked{..}=>None}}
    pub fn retained(step:RetainedCloneStep,parent_terminal_is_empty:bool)->Self{match step{RetainedCloneStep::Complete(progress)if parent_terminal_is_empty=>Self::Complete(progress),RetainedCloneStep::Progress(progress)|RetainedCloneStep::Complete(progress)=>Self::Progress(progress)}}
}

impl semio_framework_value::RetirementReceipt for PluginLifecycleStep {
    fn retirement_progress(&self)->Option<RetainedCloneProgress>{self.progress()}
    fn retirement_complete(&self)->bool{matches!(self,Self::Complete(_))}
    fn retirement_yield()->Self{Self::Progress(Default::default())}
}

#[derive(Debug)]
pub struct ArtifactFixedQueue<T> {
    pub items: std::collections::VecDeque<T>,
    pub capacity: usize,
    pub allocation_admitted: bool,
}

impl<T> ArtifactFixedQueue<T> {
    pub fn new(capacity: usize) -> Self {
        let mut items = std::collections::VecDeque::new();
        let allocation_admitted = items.try_reserve_exact(capacity).is_ok();
        Self { items, capacity, allocation_admitted }
    }

    pub fn push(&mut self, value: T) -> Result<(), T> {
        if !self.allocation_admitted || self.items.len() >= self.capacity {
            return Err(value);
        }
        self.items.push_back(value);
        Ok(())
    }

    pub fn can_push(&self) -> bool { self.allocation_admitted && self.items.len() < self.capacity }

    pub fn pop(&mut self) -> Option<T> {
        self.items.pop_front()
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }
}

/// 🎟️ Funds one normal-operation turn on every independent axis for a caller no scheduler grants.
fn plugin_page_grant(page_bytes: usize) -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: page_bytes, maximum_capacity_bytes: page_bytes, maximum_release_bytes: page_bytes, maximum_depth: semio_framework_value_ordered::ordered::MAX_AVL_HEIGHT * 5 + 16 }
}

/// 🎟️ Self-funds exactly one quoted turn for a caller whose owner publishes an honest demand.
pub fn plugin_demand_grant(demand: RetirementDemand) -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth.max(1) }
}

/// 🚦️ Admits a turn only when every axis is funded: an under-granted axis yields, a depth below demand is refused.
pub fn plugin_turn_admitted(demand: RetirementDemand, grant: RetainedCloneGrant) -> Result<bool, Fault> {
    if grant.maximum_items != 0 && grant.maximum_depth < demand.depth {
        return Err(plugin_retirement_fault(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "plugin retirement turn exceeds admitted depth")));
    }
    Ok(plugin_grant_funds(demand, grant))
}

/// 🧯️ Converts a value-layer retirement refusal into the plugin fault.
pub fn plugin_retirement_fault(error: ValueError) -> Fault {
    plugin_sdk_fault(error.to_string())
}

/// 📏️ Observes every independent demand of one interactive job's next close turn.
fn plugin_job_close_demand<J: semio_framework_job::InteractiveJob>(job: &J, body: usize) -> Result<RetirementDemand, ValueError> {
    Ok(RetirementDemand { copy_bytes: job.next_close_copy_byte_demand()?, capacity_bytes: job.next_close_capacity_byte_demand(body)?, release_bytes: job.next_close_release_byte_demand()?, depth: job.next_close_depth_demand()?.max(1) })
}

/// 📏️ Quotes the next close turn of one owned text: its last scalar, then its whole physical backing.
pub fn plugin_text_retirement_demand(text: &String) -> RetirementDemand {
    match text.chars().next_back() {
        Some(scalar) => RetirementDemand { copy_bytes: scalar.len_utf8(), depth: 1, ..Default::default() },
        None => RetirementDemand { release_bytes: text.capacity(), depth: usize::from(text.capacity() != 0), ..Default::default() },
    }
}

/// ♻️ Closes one owned text under the grant; `None` means no physical text remains.
pub fn plugin_text_close(text: &mut String, grant: RetainedCloneGrant) -> Option<RetainedCloneProgress> {
    let empty = RetainedCloneProgress::default();
    if text.capacity() == 0 {
        return None;
    }
    if !plugin_grant_funds(plugin_text_retirement_demand(text), grant) {
        return Some(empty);
    }
    if text.chars().next_back().is_some() {
        let mut copied_bytes = 0;
        while let Some(scalar) = text.chars().next_back() {
            if copied_bytes + scalar.len_utf8() > grant.maximum_copy_bytes {
                break;
            }
            text.pop();
            copied_bytes += scalar.len_utf8();
        }
        return Some(RetainedCloneProgress { copied_items: 1, copied_bytes, ..empty });
    }
    let released_bytes = text.capacity();
    *text = String::new();
    Some(RetainedCloneProgress { copied_items: 1, released_bytes, ..empty })
}

/// 📏️ Quotes the next turn of an original typed owner: its controlled frame admission, then its live frontier.
pub fn plugin_typed_owner_demand<T: semio_framework_value::retirement::RetireOwned>(original: &Option<T>, active: &Option<ControlledRetirement<T>>, body: usize) -> Result<RetirementDemand, ValueError> {
    if let Some(owner) = active.as_ref() {
        return Ok({let mut demand=owner.next_demand(body)?;demand.depth=demand.depth.max(1);demand});
    }
    Ok(if original.is_some() { RetirementDemand { depth: 1, ..Default::default() } } else { RetirementDemand::default() })
}

/// 🧳️ Retains an original typed owner in its controlled frame, then advances that frame under the unchanged grant.
pub fn plugin_typed_owner_step<T: semio_framework_value::retirement::RetireOwned>(original: &mut Option<T>, active: &mut Option<ControlledRetirement<T>>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
    use semio_framework_value::retirement::controlled::ControlledRetirement;
    let empty = RetainedCloneProgress::default();
    if active.is_none() {
        if original.is_none() {
            return Ok(RetainedCloneStep::Complete(empty));
        }
        if grant.maximum_items == 0 {
            return Ok(RetainedCloneStep::Progress(empty));
        }
        let value = original.take().expect("typed original was observed");
        return match ControlledRetirement::new(value) {
            Ok(owner) => {
                *active = Some(owner);
                Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..empty }))
            }
            Err((error, value)) => {
                *original = Some(value);
                Err(error)
            }
        };
    }
    let owner = active.as_mut().expect("typed owner frame was observed");
    let step = owner.step(grant)?;
    semio_framework_value::retained_clone::admit_retained_clone_progress(grant, step.progress(), "typed owner retirement")?;
    if owner.terminal_is_empty() {
        *active = None;
    }
    Ok(RetainedCloneStep::Progress(step.progress()))
}

/// 📏️ Quotes the next close turn of one owned byte vector: any payload truncation, then its whole physical backing.
pub fn plugin_bytes_retirement_demand(bytes: &Vec<u8>) -> RetirementDemand {
    if bytes.is_empty() {
        RetirementDemand { release_bytes: bytes.capacity(), depth: usize::from(bytes.capacity() != 0), ..Default::default() }
    } else {
        RetirementDemand { copy_bytes: 1, depth: 1, ..Default::default() }
    }
}

/// ♻️ Closes one owned byte vector under the grant; `None` means no physical bytes remain.
pub fn plugin_bytes_close(bytes: &mut Vec<u8>, grant: RetainedCloneGrant) -> Option<RetainedCloneProgress> {
    let empty = RetainedCloneProgress::default();
    if bytes.capacity() == 0 {
        return None;
    }
    if !plugin_grant_funds(plugin_bytes_retirement_demand(bytes), grant) {
        return Some(empty);
    }
    if !bytes.is_empty() {
        let copied_bytes = bytes.len().min(grant.maximum_copy_bytes);
        bytes.truncate(bytes.len() - copied_bytes);
        return Some(RetainedCloneProgress { copied_items: 1, copied_bytes, ..empty });
    }
    let released_bytes = bytes.capacity();
    *bytes = Vec::new();
    Some(RetainedCloneProgress { copied_items: 1, released_bytes, ..empty })
}

pub const ARTIFACT_LIVE_OUTPUT_SLOTS: usize = 64;

/// 📦️ Concurrent document archive loads one instance admits. A load slot carries the whole archive decode and hydration
/// cursors inline (~25 KiB), so the slot count is what keeps the registry's one backing allocation, and the single
/// release its close owes, under every mounted caller grant and the guest's contiguous request ceiling.
pub const DOCUMENT_ARCHIVE_LOAD_SLOTS: usize = 2;

pub struct ArtifactFixedRegistry<T> {
    pub slots: Box<[std::mem::MaybeUninit<(u64, T)>]>,
    pub occupied: u64,
    pub allocation_admitted: bool,
    capacity: usize,
}

impl<T> ArtifactFixedRegistry<T> {
    /// 🪹️ Reserves every slot on the heap while leaving the occupied mask as the sole
    /// initialization witness. A successful exact reservation provides capacity for all slots;
    /// `MaybeUninit` permits extending their length without values. Insertion writes before setting
    /// a bit, removal clears the bit before reading, and the zero initial mask prevents every read.
    pub fn new() -> Self {
        Self::with_slots(ARTIFACT_LIVE_OUTPUT_SLOTS)
    }

    /// 📏️ A direct-mapped table of exactly `capacity` slots (a divisor of the 64-bit occupancy mask), for owners whose
    /// slot is too large to reserve sixty-four of them inside one physical release grant.
    pub fn with_slots(capacity: usize) -> Self {
        assert!(capacity != 0 && capacity <= ARTIFACT_LIVE_OUTPUT_SLOTS && ARTIFACT_LIVE_OUTPUT_SLOTS % capacity == 0, "fixed artifact registry capacity divides its occupancy mask");
        let mut slots = Vec::<std::mem::MaybeUninit<(u64, T)>>::new();
        let allocation_admitted = slots.try_reserve_exact(capacity).is_ok();
        if allocation_admitted {
            unsafe { slots.set_len(capacity) };
        }
        Self { slots: slots.into_boxed_slice(), occupied: 0, allocation_admitted, capacity }
    }

    /// 🎟️ Retains the authored slot count without allocating before an original caller admits its backing.
    pub fn unallocated(capacity:usize)->Self{
        assert!(capacity!=0&&capacity<=ARTIFACT_LIVE_OUTPUT_SLOTS&&ARTIFACT_LIVE_OUTPUT_SLOTS%capacity==0);
        Self{slots:Box::new([]),occupied:0,allocation_admitted:false,capacity}
    }

    /// 🪙️ Allocates the exact empty slot body under the receiving caller's independent currencies.
    pub fn admit_backing_granted(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        if self.allocation_admitted{return Ok(RetainedCloneStep::Complete(Default::default()))}
        let bytes=self.capacity.checked_mul(size_of::<std::mem::MaybeUninit<(u64,T)>>()).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"original registry backing extent overflow"))?;
        if grant.maximum_items==0||grant.maximum_capacity_bytes<bytes||grant.maximum_depth==0{return Ok(RetainedCloneStep::Progress(Default::default()))}
        let mut slots=Vec::new();slots.try_reserve_exact(self.capacity).map_err(|_|ValueError::literal(semio_framework_value::ValueRefusalKind::AllocationFailed,"original registry backing allocation refused"))?;
        let actual=slots.capacity()*size_of::<std::mem::MaybeUninit<(u64,T)>>();
        unsafe{slots.set_len(self.capacity)};self.slots=slots.into_boxed_slice();self.allocation_admitted=true;
        Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,retained_capacity_bytes:actual,..Default::default()}))
    }

    fn occupied(&self, index: usize) -> bool {
        self.occupied & (1u64 << index) != 0
    }

    /// 🧿️ SAFETY: the occupied bit is set only after `write` and cleared before `assume_init_read`.
    pub fn entry(&self, index: usize) -> Option<&(u64, T)> {
        if !self.occupied(index) {
            return None;
        }
        self.slots.get(index).map(|slot| unsafe { slot.assume_init_ref() })
    }

    /// 🖋️ SAFETY: the occupied bit is set only after `write` and cleared before `assume_init_read`.
    pub fn entry_mut(&mut self, index: usize) -> Option<&mut (u64, T)> {
        if !self.occupied(index) {
            return None;
        }
        self.slots.get_mut(index).map(|slot| unsafe { slot.assume_init_mut() })
    }

    /// 🔑️ Inserts one exact owner into its direct-mapped slot, returning the value when the slot is occupied or the backing is not yet admitted.
    pub fn insert(&mut self, id: u64, value: T) -> Result<(), T> {
        if !self.allocation_admitted {
            return Err(value);
        }
        let index = id as usize % self.capacity;
        if self.entry(index).is_some() {
            return Err(value);
        }
        self.slots[index].write((id, value));
        self.occupied |= 1u64 << index;
        Ok(())
    }

    pub fn can_insert(&self, id: u64) -> bool {
        self.slot_is_vacant(id as usize % self.capacity)
    }

    /// 🎰️ Whether one residue class of this direct-mapped table owns no live value — the per-slot
    /// question an admitting caller asks before it mints an id for that slot.
    pub fn slot_is_vacant(&self, slot: usize) -> bool {
        self.allocation_admitted && self.entry(slot).is_none()
    }

    pub fn insert_admitted(&mut self, id: u64, value: T) {
        let index = id as usize % self.capacity;
        assert!(self.can_insert(id), "fixed artifact registry admission changed before exact owner insertion");
        self.slots[index].write((id, value));
        self.occupied |= 1u64 << index;
    }

    /// 🪢️ Borrows the value of `id` mutably and hands every other occupied value to `other` in slot order, all disjoint.
    pub fn partition_mut<'a>(&'a mut self, id: u64, mut other: impl FnMut(&'a T)) -> Option<&'a mut T> {
        let occupied = self.occupied;
        let mut selected = None;
        for (index, slot) in self.slots.iter_mut().enumerate() {
            if occupied & (1u64 << index) == 0 { continue; }
            let (candidate, value) = unsafe { slot.assume_init_mut() };
            if *candidate == id { selected = Some(value); } else { other(value); }
        }
        selected
    }

    pub fn get(&self, id: u64) -> Option<&T> {
        self.entry(id as usize % self.capacity).and_then(|(candidate, value)| (*candidate == id).then_some(value))
    }

    pub fn get_mut(&mut self, id: u64) -> Option<&mut T> {
        self.entry_mut(id as usize % self.capacity).and_then(|(candidate, value)| (*candidate == id).then_some(value))
    }

    pub fn has_capacity(&self) -> bool {
        self.allocation_admitted && (self.occupied.count_ones() as usize) < self.capacity
    }

    /// ➖️ SAFETY: exact identity and occupied state were checked, and the bit is now cleared.
    pub fn remove(&mut self, id: u64) -> Option<T> {
        let index = id as usize % self.capacity;
        if self.entry(index).is_none_or(|(candidate, _)| *candidate != id) {
            return None;
        }
        self.occupied &= !(1u64 << index);
        Some(unsafe { self.slots[index].assume_init_read().1 })
    }

    pub fn id_at(&self, index: usize) -> Option<u64> {
        self.entry(index).map(|(id, _)| *id)
    }

    pub fn each_id(&self, mut visit: impl FnMut(u64)) {
        for index in 0..self.capacity {
            if let Some(id) = self.id_at(index) {
                visit(id);
            }
        }
    }

    pub fn next_id_from(&self, cursor: usize) -> Option<(usize, u64)> {
        if self.occupied == 0 {
            return None;
        }
        let offset = self.occupied.rotate_right(cursor as u32).trailing_zeros() as usize;
        let index = (cursor + offset) % self.capacity;
        self.id_at(index).map(|id| (index, id))
    }

    pub fn is_empty(&self) -> bool {
        self.occupied == 0
    }

    /// 🪵️ Borrows the full backing extent of an already empty retained registry.
    pub fn empty_backing_byte_demand(&self) -> Option<usize> { self.is_empty().then_some(size_of_val(self.slots.as_ref())) }

    /// 🧺️ Releases the exact original empty registry backing under independent authority.
    pub fn close_empty_backing_granted(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        let idle=RetainedCloneProgress::default();
        let bytes=self.empty_backing_byte_demand().ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original registry backing still owns an active slot"))?;
        if bytes==0{return Ok(RetainedCloneStep::Complete(idle));}
        if grant.maximum_items==0||grant.maximum_release_bytes<bytes||grant.maximum_depth==0{return Ok(RetainedCloneStep::Progress(idle));}
        drop(std::mem::replace(&mut self.slots,Box::default()));self.allocation_admitted=false;
        Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes:bytes,..idle}))
    }

    /// 🧺️ Retains every empty slot until its whole original allocation is funded.
    pub fn close_empty_backing_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, Fault> {
        let empty = RetainedCloneProgress::default();
        let Some(bytes) = self.empty_backing_byte_demand() else {
            return Err(plugin_sdk_fault("registry backing still owns an exact active slot"));
        };
        if bytes == 0 {
            return Ok(RetainedCloneStep::Complete(empty));
        }
        if !plugin_turn_admitted(RetirementDemand { release_bytes: bytes, depth: 1, ..Default::default() }, grant)? {
            return Ok(RetainedCloneStep::Progress(empty));
        }
        self.slots = Box::default();
        self.allocation_admitted = false;
        Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: bytes, ..empty }))
    }
    /// 🔢️ Admitted entries — the occupied mask's population.
    pub fn len(&self) -> usize {
        self.occupied.count_ones() as usize
    }

    #[cfg(test)]
    fn contains(&self, id: u64) -> bool {
        self.get(id).is_some()
    }
}

impl<T> Drop for ArtifactFixedRegistry<T> {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.is_empty(), "fixed artifact owner registry reached Drop before every exact owner was transferred to bounded retirement");
    }
}


/// 💰️ Whether a retirement grant funds one whole retirement demand on every axis (items, copy, capacity, release, depth).
pub fn plugin_grant_funds(demand: RetirementDemand, grant: RetainedCloneGrant) -> bool {
    grant.maximum_items != 0 && grant.maximum_copy_bytes >= demand.copy_bytes && grant.maximum_capacity_bytes >= demand.capacity_bytes && grant.maximum_release_bytes >= demand.release_bytes && grant.maximum_depth >= demand.depth
}

/// 📬️ Forwards the original grant once and validates the actual owner receipt and terminal witness.
pub fn runtime_lifecycle_step(demand: RetirementDemand, grant: RetainedCloneGrant, owner: impl FnOnce(RetainedCloneGrant)->Result<(PluginLifecycleStep,bool),Fault>)->Result<PluginLifecycleStep,Fault> {
    semio_framework_value::advance_retirement_turn(demand,grant,owner).map_err(|error|match error{semio_framework_value::RetirementTurnError::Owner(fault)=>fault,semio_framework_value::RetirementTurnError::Receipt(error)=>Fault::from(error.into_message())})
}

/// 🧮️ Intersects the authored policy with the original caller's remaining physical authority.
pub fn original_plugin_turn_grant(policy:RetainedCloneGrant,remaining:RetainedCloneGrant)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:policy.maximum_items.min(remaining.maximum_items),maximum_copy_bytes:policy.maximum_copy_bytes.min(remaining.maximum_copy_bytes),maximum_capacity_bytes:policy.maximum_capacity_bytes.min(remaining.maximum_capacity_bytes),maximum_release_bytes:policy.maximum_release_bytes.min(remaining.maximum_release_bytes),maximum_depth:policy.maximum_depth.min(remaining.maximum_depth)}}
