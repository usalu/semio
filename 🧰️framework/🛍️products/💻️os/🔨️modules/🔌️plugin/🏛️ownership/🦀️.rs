//! 🏛️ Bounded ownership primitives: lifecycle steps, fixed queues and registries, and the grant and retirement helpers every owner shares.
use semio_framework::Fault;
use semio_framework_value::{RetirementDemand, ValueError, retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep}};
use semio_framework_value::retirement::controlled::ControlledRetirement;
use crate::sdk_fault::plugin_sdk_fault;

#[path="⚠️failure/🦀️.rs"]
mod original_failure;
pub use original_failure::{PluginLifecycleFailure,PluginLifecyclePrimary};
pub use original_failure::{original_lifecycle_net_after,original_lifecycle_grant,original_lifecycle_step,original_lifecycle_effect_step,original_lifecycle_source_effect_step,plugin_settle_poll,plugin_grant_step,plugin_unit_step,plugin_effect_close,plugin_effect_idle,plugin_controlled_effect_close,plugin_poll_outcome,plugin_effect_turn,plugin_failure_error,plugin_settle_value,plugin_pay_unit,original_lifecycle_delta,original_lifecycle_unposted};



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
    pub fn with_progress(self, progress: RetainedCloneProgress) -> Self {
        match self {
            Self::Complete(_) => Self::Complete(progress),
            _ => Self::Progress(progress),
        }
    }
    pub fn progress(self)->Option<RetainedCloneProgress>{match self{Self::Progress(progress)|Self::Complete(progress)=>Some(progress),Self::AwaitingInput{..}|Self::Blocked{..}=>None}}
    pub fn retained(step:RetainedCloneStep,parent_terminal_is_empty:bool)->Self{match step{RetainedCloneStep::Complete(progress)if parent_terminal_is_empty=>Self::Complete(progress),RetainedCloneStep::Progress(progress)|RetainedCloneStep::Complete(progress)=>Self::Progress(progress)}}
}

impl semio_framework_value::RetirementReceipt for PluginLifecycleStep {
    fn retirement_progress(&self)->Option<RetainedCloneProgress>{self.progress()}
    fn retirement_complete(&self)->bool{matches!(self,Self::Complete(_))}
    fn retirement_yield()->Self{Self::Progress(Default::default())}
}

/// 📏️ Combines only defining component headers and explicit original backing quotes.
pub fn original_component_birth<S>(backing:&[usize])->Result<RetirementDemand,ValueError>{let capacity_bytes=backing.iter().try_fold(0usize,|total,bytes|total.checked_add(*bytes)).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"original component backing extent overflow"))?;Ok(RetirementDemand{copy_bytes:2*std::mem::size_of::<S>(),capacity_bytes,depth:2,..Default::default()})}
/// 📏️ Infers the actual registry row type from its defining component field without constructing either owner.
pub fn original_registry_backing<S,T>(_:fn(&S)->&ArtifactFixedRegistry<T>,capacity:usize)->Result<usize,ValueError>{ArtifactFixedRegistry::<T>::birth_demands(capacity).map(|d|d.capacity_bytes)}
/// 🌱️ Births the next slot of a deque backing on demand: the exact capacity doubles up to `limit`, the born bytes enter the birth ledger, and the answer says whether a slot is free.
pub fn deque_birth_slot<T>(deque:&mut std::collections::VecDeque<T>,limit:usize)->bool{
    if deque.len()>=limit{return false}
    if deque.len()<deque.capacity(){return true}
    let before=deque.capacity();
    let target=before.saturating_mul(2).max(1).min(limit);
    if deque.try_reserve_exact(target-deque.len()).is_err(){return false}
    semio_framework_value::birth_ledger::record_original_birth((deque.capacity()-before)*size_of::<T>());
    true
}
/// 🌱️ Pushes onto a deque whose backing is born by [`deque_birth_slot`] and never reserved ahead of use.
pub fn deque_push_back_born<T>(deque:&mut std::collections::VecDeque<T>,value:T){deque_birth_slot(deque,usize::MAX);deque.push_back(value)}
/// 📏️ Infers the actual queue element type from its defining component field without constructing backing.
pub fn original_queue_backing<S,T>(_:fn(&S)->&ArtifactFixedQueue<T>,capacity:usize)->Result<usize,ValueError>{ArtifactFixedQueue::<T>::birth_demands(capacity).map(|d|d.capacity_bytes)}

/// 📦️ Quotes and retires the physical backing of an empty original container.
pub trait EmptyBacking {
    fn release_bytes(&self)->Option<usize>;
    fn release(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>;
}

/// 📮️ Returns one fully funded physical release for an empty original backing.
pub fn release_empty_backing(bytes:Option<usize>,grant:RetainedCloneGrant,release:impl FnOnce())->Result<RetainedCloneStep,ValueError>{
    let bytes=bytes.ok_or_else(||ValueError::invariant("mounted backing retains an active item or reservation"))?;
    let idle=RetainedCloneProgress::default();
    if bytes==0{return Ok(RetainedCloneStep::Complete(idle))}
    if !plugin_grant_funds(RetirementDemand{release_bytes:bytes,depth:1,..Default::default()},grant){return Ok(RetainedCloneStep::Progress(idle))}
    release();
    Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes:bytes,..idle}))
}

impl<T> EmptyBacking for ArtifactFixedRegistry<T>{
    fn release_bytes(&self)->Option<usize>{self.empty_backing_byte_demand()}
    fn release(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{self.close_empty_backing_granted(grant)}
}

impl<T> EmptyBacking for std::collections::VecDeque<T>{
    fn release_bytes(&self)->Option<usize>{self.is_empty().then(||self.capacity()*size_of::<T>())}
    fn release(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        release_empty_backing(self.release_bytes(),grant,||*self=Self::new())
    }
}

impl<T> EmptyBacking for ArtifactFixedQueue<T>{
    fn release_bytes(&self)->Option<usize>{self.items.release_bytes()}
    fn release(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        release_empty_backing(self.release_bytes(),grant,||{self.items=std::collections::VecDeque::new();self.allocation_admitted=false;})
    }
}

#[derive(Debug)]
pub struct ArtifactFixedQueue<T> {
    pub items: std::collections::VecDeque<T>,
    pub capacity: usize,
    pub allocation_admitted: bool,
}

impl<T> ArtifactFixedQueue<T> {
    /// 📏️ Declares the physical queue backing born with the owner: none, because slots are born on first use up to the authored capacity.
    pub fn birth_demands(capacity:usize)->Result<RetirementDemand,ValueError>{capacity.checked_mul(std::mem::size_of::<T>()).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"fixed queue backing extent overflow"))?;Ok(RetirementDemand{copy_bytes:std::mem::size_of::<Self>(),depth:1,..Default::default()})}

    /// 🌱️ Retains the authored queue capacity until its original caller admits the queue.
    pub fn unallocated(capacity:usize)->Self{Self{items:std::collections::VecDeque::new(),capacity,allocation_admitted:false}}

    /// 🎟️ Admits this same queue independently of any enclosing service frame; its slots are born by [`Self::push`].
    pub fn admit_backing_granted(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{if self.allocation_admitted{return Ok(RetainedCloneStep::Complete(Default::default()))}let d=Self::birth_demands(self.capacity)?;if !plugin_grant_funds(d,grant){return Ok(RetainedCloneStep::Progress(Default::default()))}self.allocation_admitted=true;Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))}

    /// 📍️ Borrows the next unchanged physical quote without allocating any backing.
    pub fn backing_demands(&self)->Result<RetirementDemand,ValueError>{if self.allocation_admitted{Ok(Default::default())}else{Self::birth_demands(self.capacity)}}

    pub fn new(capacity: usize) -> Self {
        Self { items: std::collections::VecDeque::new(), capacity, allocation_admitted: true }
    }

    pub fn push(&mut self, value: T) -> Result<(), T> {
        if !self.allocation_admitted || !deque_birth_slot(&mut self.items, self.capacity) {
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

/// 🧮️ The copy-byte body a self-funded caller pages its owner's retirement by.
pub const PLUGIN_SELF_FUNDED_BODY_BYTES: usize = 65_536;

/// 🪙️ Self-funds exactly one quoted turn for a caller that pages its owner by `body`: the owner's demand on every axis, with the
/// copy axis widened to the body the demand was quoted against. See [`plugin_demand_grant`] for owners quoted without a body.
pub fn plugin_self_funded_grant(demand: RetirementDemand, body: usize) -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_copy_bytes: demand.copy_bytes.max(body), ..plugin_demand_grant(demand) }
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

/// 🧮️ Quotes work from the selected original frame or its controlled retirement frontier.
pub fn plugin_typed_owner_work_demand<T: semio_framework_value::retirement::RetireOwned>(original: &Option<T>, active: &Option<ControlledRetirement<T>>, body: usize) -> Result<u64, ValueError> {
    if let Some(owner) = active.as_ref() { return if owner.terminal_is_empty() { Ok(1) } else { owner.next_computational_work(body) }; }
    Ok(u64::from(original.is_some()))
}

/// 🎟️ Preserves actual typed-frame work on success and refusal.
pub fn plugin_typed_owner_original<T: semio_framework_value::retirement::RetireOwned>(original: &mut Option<T>, active: &mut Option<ControlledRetirement<T>>, effect: &mut semio_framework_value::RetainedAllocationEffect<'_>) -> semio_framework_value::OriginalRetirementPoll {
 use semio_framework_value::RetirementPollExit as Exit;
 let result=(||->Result<RetainedCloneStep,Exit>{
    use semio_framework_value::retirement::controlled::ControlledRetirement;
    let grant = effect.grant();
    let empty = RetainedCloneProgress::default();
    if active.is_none() {
        if original.is_none() {
            return Ok(RetainedCloneStep::Complete(empty));
        }
        if grant.maximum_items == 0 || grant.maximum_depth == 0 {
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
                Err(error.into())
            }
        };
    }
    let owner = active.as_mut().expect("typed owner frame was observed");
    if owner.terminal_is_empty() {
        if grant.maximum_items == 0 || grant.maximum_depth == 0 { return Ok(RetainedCloneStep::Progress(empty)); }
        *active = None;
        return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..empty }));
    }
    let step = Exit::resume(owner.step(grant.maximum_copy_bytes, effect))?;
    semio_framework_value::retained_clone::admit_retained_clone_progress(grant, step.progress(), "typed owner retirement")?;
    Ok(RetainedCloneStep::Progress(step.progress()))
 })();Exit::settle(effect,result)
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

#[cfg(any(test, feature = "testkit"))]
#[path = "🧪️effect-fixture/🦀️.rs"]
pub mod effect_fixture;

#[path = "🧱️registry-backing/🦀️.rs"]
mod registry_backing;
use registry_backing::RegistryBacking;

pub struct ArtifactFixedRegistry<T> {
    slots: RegistryBacking<T>,
    pub occupied: u64,
    pub allocation_admitted: bool,
    capacity: usize,
}

impl<T> ArtifactFixedRegistry<T> {
    /// 📏️ Declares every physical backing extent of this fixed registry before native admission.
    pub fn birth_demands(capacity:usize)->Result<RetirementDemand,ValueError>{
        if capacity==0||capacity>ARTIFACT_LIVE_OUTPUT_SLOTS||ARTIFACT_LIVE_OUTPUT_SLOTS%capacity!=0{return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue,"fixed registry capacity does not divide its occupancy mask"))}
        Ok(RetirementDemand{capacity_bytes:RegistryBacking::<T>::birth_bytes(capacity)?,depth:1,..Default::default()})
    }

    /// 📍️ Borrows the next unchanged physical quote without allocating any slot.
    pub fn backing_demands(&self)->Result<RetirementDemand,ValueError>{if self.allocation_admitted{Ok(Default::default())}else{Self::birth_demands(self.capacity)}}

    /// 🪹️ Reserves every slot while leaving the occupied mask as the sole initialization witness.
    pub fn new() -> Self { Self::with_slots(ARTIFACT_LIVE_OUTPUT_SLOTS) }

    /// 📐️ Reserves exactly this divisor of the occupancy mask as direct-mapped slots.
    pub fn with_slots(capacity:usize)->Self{
        let mut registry=Self::unallocated(capacity);
        if let Ok(backing)=RegistryBacking::allocate(capacity){registry.slots=backing;registry.allocation_admitted=true;semio_framework_value::birth_ledger::record_original_birth(RegistryBacking::<T>::birth_bytes(capacity).unwrap_or(0));}
        registry
    }

    /// 🎟️ Retains the authored slot count without allocating before an original caller admits its backing.
    pub fn unallocated(capacity:usize)->Self{
        assert!(capacity!=0&&capacity<=ARTIFACT_LIVE_OUTPUT_SLOTS&&ARTIFACT_LIVE_OUTPUT_SLOTS%capacity==0);
        Self{slots:RegistryBacking::default(),occupied:0,allocation_admitted:false,capacity}
    }

    /// 🪙️ Allocates the exact empty slot body under the receiving caller's independent currencies.
    pub fn admit_backing_granted(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        if self.allocation_admitted{return Ok(RetainedCloneStep::Complete(Default::default()))}
        if self.slots.next_release()!=0{return Err(ValueError::invariant("partly retired registry backing cannot admit new slots"))}
        let demand=Self::birth_demands(self.capacity)?;
        if !plugin_grant_funds(demand,grant){return Ok(RetainedCloneStep::Progress(Default::default()))}
        self.slots=RegistryBacking::allocate(self.capacity)?;
        self.allocation_admitted=true;
        Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,retained_capacity_bytes:demand.capacity_bytes,..Default::default()}))
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
        self.slots.get_or_birth_mut(index).expect("admitted registry slot").write((id, value));
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
        self.slots.get_or_birth_mut(index).expect("admitted registry slot").write((id, value));
        self.occupied |= 1u64 << index;
    }

    /// 🪢️ Borrows the value of `id` mutably and hands every other occupied value to `other` in slot order, all disjoint.
    pub fn partition_mut<'a>(&'a mut self, id: u64, mut other: impl FnMut(&'a T)) -> Option<&'a mut T> {
        let occupied = self.occupied;
        let mut selected = None;
        for (index, slot) in self.slots.resident_pages_mut().flat_map(|(base, page)| page.iter_mut().enumerate().map(move |(offset, slot)| (base + offset, slot))) {
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
        Some(unsafe { self.slots.get_mut(index).expect("occupied registry slot").assume_init_read().1 })
    }

    pub fn id_at(&self, index: usize) -> Option<u64> {
        self.entry(index).map(|(id, _)| *id)
    }

    /// 🔁️ Visits each occupied physical slot once, starting at the cursor and wrapping at capacity.
    pub fn entries_from(&self, cursor: usize) -> impl Iterator<Item = (usize, &(u64, T))> + '_ {
        let start = cursor % self.capacity;
        let mut remaining = self.occupied.rotate_right(start as u32);
        std::iter::from_fn(move || {
            if remaining == 0 { return None; }
            let offset = remaining.trailing_zeros() as usize;
            remaining &= remaining - 1;
            let slot = (start + offset) % self.capacity;
            Some((slot, self.entry(slot).expect("occupied bit witnesses a resident entry")))
        })
    }

    pub fn each_id(&self, mut visit: impl FnMut(u64)) {
        for (_, (id, _)) in self.entries_from(0) { visit(*id); }
    }

    pub fn next_id_from(&self, cursor: usize) -> Option<(usize, u64)> {
        self.entries_from(cursor).next().map(|(index, (id, _))| (index, *id))
    }

    pub fn is_empty(&self) -> bool {
        self.occupied == 0
    }

    /// 🪵️ Borrows the next physical backing release of an already empty retained registry.
    pub fn empty_backing_byte_demand(&self) -> Option<usize> { self.is_empty().then(||self.slots.next_release()) }

    /// 🧭️ Exposes one original vacant slot address for identity observations.
    pub fn backing_slot_address(&self,index:usize)->Option<*const ()>{self.slots.get(index).map(|slot|std::ptr::from_ref(slot).cast())}

    /// 🧺️ Releases one exact original empty registry backing block under independent authority.
    pub fn close_empty_backing_granted(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        let idle=RetainedCloneProgress::default();
        let bytes=self.empty_backing_byte_demand().ok_or_else(||ValueError::invariant("original registry backing still owns an active slot"))?;
        if bytes==0{return Ok(RetainedCloneStep::Complete(idle));}
        if !plugin_grant_funds(RetirementDemand{release_bytes:bytes,depth:1,..Default::default()},grant){return Ok(RetainedCloneStep::Progress(idle));}
        self.slots.release_one();
        self.allocation_admitted=false;
        Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes:bytes,..idle}))
    }

    /// 🧯️ Reports the same physical release through the plugin fault contract.
    pub fn close_empty_backing_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,Fault>{
        let bytes=self.empty_backing_byte_demand().ok_or_else(||plugin_sdk_fault("registry backing still owns an exact active slot"))?;
        if bytes!=0{plugin_turn_admitted(RetirementDemand{release_bytes:bytes,depth:1,..Default::default()},grant)?;}
        self.close_empty_backing_granted(grant).map_err(plugin_retirement_fault)
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
    plugin_grant_shortfall(demand, grant).is_none()
}

/// 📏️ The first resource of `demand` that `grant` cannot cover, as its name, the demanded and the granted figure.
pub fn plugin_grant_shortfall(demand: RetirementDemand, grant: RetainedCloneGrant) -> Option<(&'static str, u64, u64)> {
    if grant.maximum_items == 0 { return Some(("items", 1, 0)); }
    [("copy bytes", demand.copy_bytes, grant.maximum_copy_bytes), ("capacity bytes", demand.capacity_bytes, grant.maximum_capacity_bytes), ("release bytes", demand.release_bytes, grant.maximum_release_bytes), ("depth", demand.depth, grant.maximum_depth)]
        .into_iter().find(|(_, needed, granted)| needed > granted).map(|(resource, needed, granted)| (resource, needed as u64, granted as u64))
}

/// 📬️ Forwards the original grant once and validates the actual owner receipt and terminal witness.
pub fn runtime_lifecycle_step(demand: RetirementDemand, grant: RetainedCloneGrant, owner: impl FnOnce(RetainedCloneGrant)->Result<(PluginLifecycleStep,bool),Fault>)->Result<PluginLifecycleStep,Fault> {
    semio_framework_value::advance_retirement_turn(demand,grant,|grant|semio_framework_value::RetainedWorkOutcome{result:owner(grant),work:1}).result.map_err(|error|match error{semio_framework_value::RetirementTurnError::Owner(fault)=>fault,semio_framework_value::RetirementTurnError::Receipt(error)=>Fault::from(error.into_message())})
}

/// 🧮️ Intersects the authored policy with the original caller's remaining physical authority.
pub fn original_plugin_turn_grant(policy:RetainedCloneGrant,remaining:RetainedCloneGrant)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:policy.maximum_items.min(remaining.maximum_items),maximum_copy_bytes:policy.maximum_copy_bytes.min(remaining.maximum_copy_bytes),maximum_capacity_bytes:policy.maximum_capacity_bytes.min(remaining.maximum_capacity_bytes),maximum_release_bytes:policy.maximum_release_bytes.min(remaining.maximum_release_bytes),maximum_depth:policy.maximum_depth.min(remaining.maximum_depth)}}

#[cfg(test)]
mod typed_owner_work_tests {
    use super::*;

    #[test]
    fn a_self_funded_grant_funds_its_demand_and_the_body_it_was_quoted_against() {
        let demand = RetirementDemand { copy_bytes: 96, capacity_bytes: 7, release_bytes: 11, depth: 0, ..Default::default() };
        let grant = plugin_self_funded_grant(demand, PLUGIN_SELF_FUNDED_BODY_BYTES);
        assert_eq!(grant, RetainedCloneGrant { maximum_copy_bytes: PLUGIN_SELF_FUNDED_BODY_BYTES, ..plugin_demand_grant(demand) });
        assert!(plugin_grant_funds(demand, grant));
        let large = RetirementDemand { copy_bytes: PLUGIN_SELF_FUNDED_BODY_BYTES + 1, depth: 3, ..Default::default() };
        assert_eq!(plugin_self_funded_grant(large, PLUGIN_SELF_FUNDED_BODY_BYTES), plugin_demand_grant(large));
    }

    fn run(caller: &mut semio_framework_job::OriginalJobFixtureCaller, original: &mut Option<String>, active: &mut Option<ControlledRetirement<String>>, grant: RetainedCloneGrant) -> (RetainedCloneStep, u128) {
        let turn = RetainedCloneGrant { maximum_items: 64, maximum_copy_bytes: 1 << 16, maximum_capacity_bytes: 1 << 16, maximum_release_bytes: 1 << 16, maximum_depth: 16 };
        let (poll, performed) = caller.turn(turn, |cx| Ok(cx.with_original_effect(grant, |_, effect| plugin_typed_owner_original(original, active, effect))?)).expect("original fixture effect turn").expect("original effect work quote admitted");
        match poll { semio_framework_value::RetainedWorkPoll::Complete(done) => (done.result.expect("denied admission is not a failure"), performed), semio_framework_value::RetainedWorkPoll::Pending { .. } => panic!("typed owner admission never suspends") }
    }

    #[test]
    fn denied_original_admission_preserves_owner_and_work() {
        let stock = RetainedCloneGrant { maximum_items: 64, maximum_copy_bytes: 1 << 16, maximum_capacity_bytes: 1 << 16, maximum_release_bytes: 1 << 16, maximum_depth: 16 };
        let mut caller = semio_framework_job::OriginalJobFixtureCaller::admit(semio_framework_job::OperationId(61), semio_framework_job::Generation(1), stock, || Some(1), 1_000_000, 1_000_000_000);
        let mut original = Some(String::from("original"));
        let address = original.as_ref().unwrap().as_ptr();
        let mut active = None;
        assert_eq!(plugin_typed_owner_work_demand(&original, &active, 1).unwrap(), 1);
        let (step, work) = run(&mut caller, &mut original, &mut active, RetainedCloneGrant::default());
        assert_eq!(work, 0);
        assert_eq!(step.progress(), RetainedCloneProgress::default());
        assert_eq!(original.as_ref().unwrap().as_ptr(), address);
        assert!(active.is_none());
        let depth_denied = RetainedCloneGrant { maximum_items: 1, ..RetainedCloneGrant::default() };
        let (step, work) = run(&mut caller, &mut original, &mut active, depth_denied);
        assert_eq!(work, 0);
        assert_eq!(step.progress(), RetainedCloneProgress::default());
        assert_eq!(original.as_ref().unwrap().as_ptr(), address);
        assert!(active.is_none());
        caller.retire(stock);
    }
}

#[cfg(test)]
mod lazy_backing_tests {
    use super::*;
    use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;

    /// 🌱️ A fresh queue owns no physical slot and a push births slots by exact doubling up to its authored capacity.
    #[test]
    fn a_fresh_fixed_queue_holds_no_heap_and_births_slots_by_doubling_up_to_its_capacity() {
        type Row = [u8; 4096];
        let (mut queue, heap) = observe(|| ArtifactFixedQueue::<Row>::new(64));
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        assert_eq!(ArtifactFixedQueue::<Row>::birth_demands(64).unwrap().capacity_bytes, 0);
        let mut capacities = Vec::new();
        for _ in 0..64 {
            let (pushed, _) = observe(|| queue.push([1; 4096]));
            assert!(pushed.is_ok());
            if capacities.last() != Some(&queue.items.capacity()) { capacities.push(queue.items.capacity()); }
        }
        assert_eq!(capacities, [1, 2, 4, 8, 16, 32, 64]);
        assert!(queue.push([1; 4096]).is_err(), "the authored capacity bounds the queue");
        while queue.pop().is_some() {}
        let bytes = queue.release_bytes().unwrap();
        assert_eq!(bytes, 64 * size_of::<Row>());
        let grant = RetainedCloneGrant { maximum_items: 1, maximum_release_bytes: bytes, maximum_depth: 1, ..Default::default() };
        let (step, heap) = observe(|| queue.release(grant).unwrap());
        assert_eq!(step.progress().released_bytes, bytes);
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, bytes));
        assert_eq!(queue.release_bytes(), Some(0));
    }

    /// 🌱️ A deque born empty pays for exactly the slots it uses and releases exactly its capacity.
    #[test]
    fn a_born_on_demand_deque_owns_only_the_slots_it_used() {
        type Row = [u8; 2048];
        let mut deque = std::collections::VecDeque::<Row>::new();
        let (_, heap) = observe(|| deque_push_back_born(&mut deque, [7; 2048]));
        assert_eq!(heap.requested_bytes, size_of::<Row>());
        assert_eq!(deque.release_bytes(), None, "a deque holding a row owns no empty backing");
        deque.pop_front();
        assert_eq!(deque.release_bytes(), Some(size_of::<Row>()));
    }
}
