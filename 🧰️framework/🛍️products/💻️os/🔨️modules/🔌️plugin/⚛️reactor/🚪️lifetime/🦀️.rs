//#region 🚪️GuestLifecycleAuthority
use semio_framework::kernel::{ActorInstanceCloseRequest, ActorInstanceLifecycleAck, ActorInstanceLifecycleReceipt, ActorInstanceLifetime, ActorInstanceOpenRequest};
use std::mem::ManuallyDrop;

/// 🔗️ Process-local descendant identity; its captured weak app lease keeps the allocation address reserved.
#[doc(hidden)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct NativeCloseKey {
    pub lifetime: ActorInstanceLifetime,
    pub allocation: usize,
}
semio_framework_value::artifact_retire_leaf!(NativeCloseKey);

impl NativeCloseKey {
    pub fn capture<PA: semio_framework_plugin_reactor_app::PluginApp + 'static>(lifetime: ActorInstanceLifetime, lease: &crate::life_instance_registry::instance_lifetime::PluginInstanceCloseLease<PA>) -> Result<Self, &'static str> {
        let (instance, allocation) = lease.allocation_identity();
        if !lifetime.is_valid() || lifetime.instance_id != instance {
            return Err("native close allocation does not match guest lifetime");
        }
        Ok(Self { lifetime, allocation })
    }
    pub fn instance(self) -> u32 {
        self.lifetime.instance_id
    }
    pub fn lifetime(self) -> ActorInstanceLifetime {
        self.lifetime
    }
    #[cfg(any(test,feature="original-actor-context-testing"))]
    pub fn fixture(instance_id: u32, guest_lifetime: u64) -> Self {
        Self { lifetime: ActorInstanceLifetime { activation_generation: 1, instance_id, guest_lifetime }, allocation: 1 }
    }
}

struct GuestLifecycleSerial(u64);

impl GuestLifecycleSerial {
    pub fn new(last: u64) -> Self {
        Self(last)
    }
    fn next(&mut self) -> Result<u64, &'static str> {
        let next = self.0.checked_add(1).ok_or("guest lifetime serial exhausted")?;
        self.0 = next;
        Ok(next)
    }
}

mod terminal_owner {
    #[doc(hidden)]
pub trait Sealed {}
}

enum GuestTerminalRelease {
    Pending,
    Released,
}

/// 🧾️ The terminal witness carries the ORIGINATING fault, never a rewritten summary of it: every
/// participant below a guest lifetime answers with the exact `Fault` its own authority raised, so a
/// close that dies names the authority that refused instead of the ladder rung that noticed.
#[doc(hidden)]
pub trait GuestLifetimeOwner: terminal_owner::Sealed {
    fn terminal_is_empty<P: ClosePeers + ?Sized>(&self, peers: &P) -> Result<bool, semio_framework::Fault>;
    fn release_terminal<P: ClosePeers + ?Sized>(owner: &mut Option<Self>, maximum_items: usize, maximum_bytes: usize, peers: &P) -> Result<GuestTerminalRelease, semio_framework::Fault>
    where
        Self: Sized;
}

/// 🔗️ The close participants that sit above the lifecycle owner, the patch tracker and the cold pair ingress: the owner drives them
/// through this port, which the reactor supplies on every call, so the lifecycle names no crate above itself. Every method keeps the
/// reserve, activate, complete, release order of the owner's own participants and answers with the participant's own `Fault`.
#[doc(hidden)]
pub trait ClosePeers {
    fn preflight_patches(&self, key: NativeCloseKey) -> Result<(), semio_framework::Fault>;
    fn reserve_patches(&self, key: NativeCloseKey) -> Result<(), semio_framework::Fault>;
    fn activate_patches(&self, key: NativeCloseKey) -> Result<(), semio_framework::Fault>;
    fn patches_complete(&self, key: NativeCloseKey) -> Result<bool, semio_framework::Fault>;
    fn release_patches(&self, key: NativeCloseKey) -> Result<(), semio_framework::Fault>;
    fn preflight_cold_pair(&self, key: NativeCloseKey) -> Result<(), semio_framework::Fault>;
    fn reserve_cold_pair(&self, key: NativeCloseKey) -> Result<(), semio_framework::Fault>;
    fn activate_cold_pair(&self, key: NativeCloseKey) -> Result<(), semio_framework::Fault>;
    fn cold_pair_complete(&self, key: NativeCloseKey) -> Result<bool, semio_framework::Fault>;
    fn release_cold_pair(&self, key: NativeCloseKey) -> Result<(), semio_framework::Fault>;
}

#[doc(hidden)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Phase {
    Opening,
    Captured,
    Live,
    Accepted,
    Closing,
    Retired,
    Released,
}

/// 🪪️ Lives in the runtime registry before callbacks; an unwind never removes its owner.
#[doc(hidden)]
pub struct GuestLifecycleCell<O: GuestLifetimeOwner> {
    pub open: ActorInstanceOpenRequest,
    pub lifetime: ActorInstanceLifetime,
    pub owner: ManuallyDrop<Option<O>>,
    pub phase: Phase,
    pub close: Option<(ActorInstanceCloseRequest, u64)>,
    pub receipt: Option<ActorInstanceLifecycleReceipt>,
    staged_ack: Option<ActorInstanceLifecycleAck>,
    previous_ack: Option<ActorInstanceLifecycleAck>,
    owner_released: bool,
}

impl<O: GuestLifetimeOwner> GuestLifecycleCell<O> {
    pub fn admit(open: ActorInstanceOpenRequest, guest_lifetime: u64) -> Result<Self, &'static str> {
        if !open.is_valid() || guest_lifetime == 0 {
            return Err("invalid guest open authority");
        }
        Ok(Self {
            open,
            lifetime: ActorInstanceLifetime { activation_generation: open.activation_generation, instance_id: open.instance_id, guest_lifetime },
            owner: ManuallyDrop::new(None),
            phase: Phase::Opening,
            close: None,
            receipt: None,
            staged_ack: None,
            previous_ack: None,
            owner_released: false,
        })
    }

    pub fn lifetime(&self) -> ActorInstanceLifetime {
        self.lifetime
    }
    pub fn matches_open(&self, open: ActorInstanceOpenRequest) -> bool {
        self.open == open
    }
    pub fn owner(&self) -> Option<&O> {
        self.owner.as_ref()
    }
    pub fn owner_mut(&mut self) -> Option<&mut O> {
        self.owner.as_mut()
    }
    pub fn retained_receipt(&self) -> Option<ActorInstanceLifecycleReceipt> {
        self.receipt
    }
    pub fn is_live(&self) -> bool {
        self.phase == Phase::Live
    }
    fn is_released(&self) -> bool {
        self.phase == Phase::Released
    }
    pub fn install_owner(&mut self, owner: O) -> Result<(), O> {
        if self.phase != Phase::Opening || self.owner.is_some() {
            return Err(owner);
        }
        *self.owner = Some(owner);
        self.receipt = Some(ActorInstanceLifecycleReceipt::Captured { lifetime: self.lifetime, request_sequence: self.open.request_sequence });
        self.phase = Phase::Captured;
        Ok(())
    }

    pub fn validate_close(&self, request: ActorInstanceCloseRequest) -> Result<(), &'static str> {
        if !request.is_valid() || request.lifetime != self.lifetime {
            return Err("foreign guest close authority");
        }
        if let Some((accepted, _)) = self.close {
            return if accepted == request { Ok(()) } else { Err("different close request already owns the lifetime") };
        }
        if !self.is_live() {
            return Err("captured receipt must be acknowledged before close admission");
        }
        Ok(())
    }

    pub fn record_close_admission(&mut self, request: ActorInstanceCloseRequest, close_generation: u64) -> Result<(), &'static str> {
        self.validate_close(request)?;
        if close_generation == 0 {
            return Err("invalid admitted native close generation");
        }
        if let Some((_, generation)) = self.close {
            return if generation == close_generation { Ok(()) } else { Err("native close generation changed") };
        }
        self.close = Some((request, close_generation));
        self.receipt = Some(ActorInstanceLifecycleReceipt::Accepted { lifetime: self.lifetime, request_sequence: request.request_sequence, close_generation });
        self.phase = Phase::Accepted;
        Ok(())
    }

    fn prepare_retired<P: ClosePeers + ?Sized>(&mut self, peers: &P) -> Result<bool, semio_framework::Fault> {
        if self.phase == Phase::Retired {
            return Ok(true);
        }
        if self.phase != Phase::Closing {
            return Ok(false);
        }
        if !self.owner.as_ref().ok_or_else(|| crate::life_reactor_close::reactor_close_fault("guest lifetime lost its native owner"))?.terminal_is_empty(peers)? {
            return Ok(false);
        }
        let (request, close_generation) = self.close.ok_or_else(|| crate::life_reactor_close::reactor_close_fault("guest lifetime lost close admission"))?;
        self.receipt = Some(ActorInstanceLifecycleReceipt::Retired { lifetime: self.lifetime, request_sequence: request.request_sequence, close_generation });
        self.phase = Phase::Retired;
        Ok(true)
    }

    pub fn stage_ack(&mut self, ack: ActorInstanceLifecycleAck) -> Result<(), &'static str> {
        if self.previous_ack == Some(ack) {
            return Ok(());
        }
        if self.receipt != Some(ack.receipt) {
            return Err("ACK does not name the exact retained receipt");
        }
        if self.staged_ack.is_some_and(|staged| staged != ack) {
            return Err("different ACK already staged");
        }
        self.staged_ack = Some(ack);
        Ok(())
    }

    /// ♻️ The sealed domain releases its final shell before the clock verdict; exact completion survives retry.
    /// A `Retired` phase already carries a terminal witness, so a participant that answers "not empty"
    /// during the release ladder is a WAIT for the next opportunity, never a fault — the release runs
    /// across four separate rungs and a momentarily contended authority in between must not kill a
    /// close that already reached its receipt (ticket 26/09/02 wave B17).
    fn release_owner_step<P: ClosePeers + ?Sized>(&mut self, maximum_items: usize, maximum_bytes: usize, peers: &P) -> Result<(), semio_framework::Fault> {
        if self.phase != Phase::Retired || self.staged_ack.is_none() || self.owner_released || maximum_items == 0 {
            return Ok(());
        }
        if !self.owner.as_ref().ok_or_else(|| crate::life_reactor_close::reactor_close_fault("terminal receipt lost its owner"))?.terminal_is_empty(peers)? {
            return Ok(());
        }
        match O::release_terminal(&mut self.owner, maximum_items, maximum_bytes, peers)? {
            GuestTerminalRelease::Pending => {
                if self.owner.is_none() {
                    return Err(crate::life_reactor_close::reactor_close_fault("pending terminal release lost its structural owner"));
                }
            }
            GuestTerminalRelease::Released => {
                if self.owner.is_some() {
                    return Err(crate::life_reactor_close::reactor_close_fault("terminal release receipt still owns its source shell"));
                }
                self.owner_released = true;
            }
        }
        Ok(())
    }

    /// ⏱️ Commits receipt consumption only after the exact turn has a successful real-clock verdict.
    /// `executing_us` is the turn's EXECUTING time — wall time inside the guest's own polls, with
    /// every suspension gap excluded (`⚛️reactor/🔄️turn`'s `guest_turn_executing_us`). `None` means
    /// the host clock was missing or ran backward, which retains the receipt exactly like a deadline.
    pub fn finish_turn(&mut self, executing_us: Option<u64>, succeeded: bool) -> Result<Option<ActorInstanceLifecycleReceipt>, &'static str> {
        if !succeeded {
            return Err("guest lifecycle turn failed; receipt retained");
        }
        let phase = if self.phase == Phase::Retired && !self.owner_released {
            None
        } else if let Some(ack) = self.staged_ack {
            if self.receipt != Some(ack.receipt) {
                return Err("staged ACK no longer matches receipt");
            }
            Some(match self.phase {
                Phase::Captured => Phase::Live,
                Phase::Accepted => Phase::Closing,
                Phase::Retired => {
                    if !self.owner_released || self.owner.is_some() {
                        return Err("terminal shell release is still pending");
                    }
                    Phase::Released
                }
                _ => return Err("ACK cannot consume this lifecycle phase"),
            })
        } else {
            None
        };
        let elapsed = executing_us.ok_or("guest lifecycle clock missing or backward; receipt retained")?;
        if semio_framework_trace::guest_lifecycle_turn_contract_violated(elapsed) {
            return Err(GUEST_LIFECYCLE_TURN_DEADLINE);
        }
        if let Some(phase) = phase {
            self.phase = phase;
            self.receipt = None;
            self.previous_ack = self.staged_ack.take();
        }
        Ok(self.receipt)
    }
}

impl<O: GuestLifetimeOwner> Drop for GuestLifecycleCell<O> {
    fn drop(&mut self) {
        assert!(self.owner.is_none() && matches!(self.phase, Phase::Opening | Phase::Released), "guest lifecycle owner and receipt must remain mounted through final exact ACK");
    }
}

#[doc(hidden)]
pub const GUEST_LIFECYCLE_TURN_DEADLINE: &str = "guest lifecycle turn exceeded strict time authority; receipt retained";

type NativeCell<PA> = GuestLifecycleCell<NativeLifetimeOwner<PA>>;

/// 🧷️ The native allocation and every close participant remain attached until terminal ACK.
#[doc(hidden)]
pub struct NativeLifetimeOwner<PA: semio_framework_plugin_reactor_app::PluginApp> {
    pub lease: crate::life_instance_registry::instance_lifetime::PluginInstanceCloseLease<PA>,
    pub key: NativeCloseKey,
    pub request: Option<ActorInstanceCloseRequest>,
    pub reserved: [bool; 4],
    pub active: [bool; 4],
    pub released: [bool; 4],
}

impl<PA: semio_framework_plugin_reactor_app::PluginApp> NativeLifetimeOwner<PA> {
    pub fn from_lease(lifetime: ActorInstanceLifetime, lease: crate::life_instance_registry::instance_lifetime::PluginInstanceCloseLease<PA>) -> Result<Self, semio_framework::Fault> {
        let key = NativeCloseKey::capture(lifetime, &lease).map_err(crate::life_reactor_close::reactor_close_fault)?;
        Ok(Self { lease, key, request: None, reserved: [false; 4], active: [false; 4], released: [false; 4] })
    }

    pub fn key(&self) -> NativeCloseKey {
        self.key
    }

    pub fn request_close<P: ClosePeers + ?Sized>(&mut self, request: ActorInstanceCloseRequest, runtime: &crate::life_instance_registry::PluginRuntime<PA>, peers: &P) -> Result<(), semio_framework::Fault> {
        if self.request == Some(request) {
            return Ok(());
        }
        if self.request.is_some() || request.lifetime != self.key.lifetime {
            return Err(crate::life_reactor_close::reactor_close_fault("close owner request changed"));
        }
        crate::life_reactor_close::preflight_reactor_close(self.key)?;
        peers.preflight_patches(self.key)?;
        crate::with_state(|pending| pending.try_borrow().map_err(|_| "pending close preflight busy")?.preflight_close_instance(self.key)).map_err(crate::life_reactor_close::reactor_close_fault)?;
        peers.preflight_cold_pair(self.key)?;
        self.lease.preflight_close(runtime)?;
        self.request = Some(request);
        Ok(())
    }

    pub fn advance_admission<P: ClosePeers + ?Sized>(&mut self, runtime: &crate::life_instance_registry::PluginRuntime<PA>, peers: &P) -> Result<Option<(ActorInstanceCloseRequest, u64)>, semio_framework::Fault> {
        let Some(request) = self.request else { return Ok(None) };
        if !self.reserved[0] {
            crate::life_reactor_close::reserve_reactor_close(self.key)?;
            self.reserved[0] = true;
        }
        if !self.reserved[1] {
            peers.reserve_patches(self.key)?;
            self.reserved[1] = true;
        }
        if !self.reserved[2] {
            crate::with_state(|pending| pending.borrow_mut().reserve_close_instance(self.key)).map_err(crate::life_reactor_close::reactor_close_fault)?;
            self.reserved[2] = true;
        }
        if !self.reserved[3] {
            peers.reserve_cold_pair(self.key)?;
            self.reserved[3] = true;
        }
        if !self.active[3] {
            peers.activate_cold_pair(self.key)?;
            self.active[3] = true;
        }
        self.lease.begin_close(runtime)?;
        if !self.active[0] {
            crate::life_reactor_close::activate_reactor_close(self.key)?;
            self.active[0] = true;
        }
        if !self.active[1] {
            peers.activate_patches(self.key)?;
            self.active[1] = true;
        }
        if !self.active[2] {
            crate::with_state(|pending| pending.borrow_mut().activate_close_instance(self.key)).map_err(crate::life_reactor_close::reactor_close_fault)?;
            self.active[2] = true;
        }
        semio_framework_plugin_reactor_tasks::JOB_RENDER_BINDINGS.with(|bindings| bindings.borrow_mut().close_instance(self.key.instance()));
        Ok(Some((request, self.lease.close_generation().ok_or_else(|| crate::life_reactor_close::reactor_close_fault("native close generation missing"))?)))
    }
}

impl<PA: semio_framework_plugin_reactor_app::PluginApp> terminal_owner::Sealed for NativeLifetimeOwner<PA> {}

impl<PA: semio_framework_plugin_reactor_app::PluginApp> GuestLifetimeOwner for NativeLifetimeOwner<PA> {
    fn terminal_is_empty<P: ClosePeers + ?Sized>(&self, peers: &P) -> Result<bool, semio_framework::Fault> {
        if !self.lease.is_retired()? {
            return Ok(false);
        }
        if !self.released[0] && !crate::life_reactor_close::reactor_close_complete(self.key)? {
            return Ok(false);
        }
        if !self.released[1] && !peers.patches_complete(self.key)? {
            return Ok(false);
        }
        if !self.released[2] && !crate::with_state(|pending| pending.borrow().close_instance_complete(self.key)).map_err(crate::life_reactor_close::reactor_close_fault)? {
            return Ok(false);
        }
        if !self.released[3] && !peers.cold_pair_complete(self.key)? {
            return Ok(false);
        }
        Ok(true)
    }

    fn release_terminal<P: ClosePeers + ?Sized>(owner: &mut Option<Self>, maximum_items: usize, maximum_bytes: usize, peers: &P) -> Result<GuestTerminalRelease, semio_framework::Fault> {
        if maximum_items == 0 || maximum_bytes < size_of::<Self>() {
            return Ok(GuestTerminalRelease::Pending);
        }
        let retained = owner.as_mut().ok_or_else(|| crate::life_reactor_close::reactor_close_fault("terminal native owner missing"))?;
        if !retained.terminal_is_empty(peers)? {
            return Ok(GuestTerminalRelease::Pending);
        }
        if !retained.released[0] {
            crate::life_reactor_close::release_reactor_close(retained.key)?;
            retained.released[0] = true;
            return Ok(GuestTerminalRelease::Pending);
        }
        if !retained.released[1] {
            peers.release_patches(retained.key)?;
            retained.released[1] = true;
            return Ok(GuestTerminalRelease::Pending);
        }
        if !retained.released[2] {
            crate::with_state(|pending| pending.borrow_mut().release_close_instance(retained.key)).map_err(crate::life_reactor_close::reactor_close_fault)?;
            retained.released[2] = true;
            return Ok(GuestTerminalRelease::Pending);
        }
        if !retained.released[3] {
            peers.release_cold_pair(retained.key)?;
            retained.released[3] = true;
            return Ok(GuestTerminalRelease::Pending);
        }
        drop(owner.take());
        Ok(GuestTerminalRelease::Released)
    }
}

#[doc(hidden)]

pub struct NativeLifetimeSlot<PA: semio_framework_plugin_reactor_app::PluginApp> {
    pub cell: NativeCell<PA>,
    pub native_created: bool,
    pub pending_creation: ManuallyDrop<Option<crate::life_cleanup_pumps::PendingNativeCreation<PA>>>,
    pub patch_sequence: u64,
}

/// 🗃️ Fixed allocation-bound ownership belongs to the typed runtime, never a copied thread-local key.
#[doc(hidden)]
pub struct NativeLifecycleRegistry<PA: semio_framework_plugin_reactor_app::PluginApp> {
    pub slots: semio_framework_plugin_reactor_tasks::tasks_fixed_slots::ReactorFixedSlots<NativeLifetimeSlot<PA>>,
    pub serial: GuestLifecycleSerial,
    pub cursor: usize,
}

impl<PA: semio_framework_plugin_reactor_app::PluginApp> NativeLifecycleRegistry<PA> {
    pub fn new() -> Self {
        Self { slots: semio_framework_plugin_reactor_tasks::tasks_fixed_slots::ReactorFixedSlots::new(), serial: GuestLifecycleSerial::new(0), cursor: 0 }
    }
    pub fn index(instance: u32) -> usize {
        instance as usize % semio_framework_plugin_reactor_tasks::tasks_fixed_slots::PLUGIN_REACTOR_INSTANCE_SLOTS
    }
    pub fn admit(&mut self, open: ActorInstanceOpenRequest) -> Result<bool, &'static str> {
        if !open.is_valid() {
            return Err("invalid open request");
        }
        let index = Self::index(open.instance_id);
        if let Some(slot) = self.slots.get(index) {
            return if slot.cell.matches_open(open) { Ok(false) } else { Err("lifecycle slot already owns an allocation") };
        }
        if !self.slots.allocation_admitted {
            return Err("lifecycle backing unavailable");
        }
        let cell = NativeCell::admit(open, self.serial.next()?)?;
        self.slots.insert_admitted(index, NativeLifetimeSlot { cell, native_created: false, pending_creation:ManuallyDrop::new(None), patch_sequence: 0 });
        Ok(true)
    }
    pub fn get(&self, instance: u32) -> Option<&NativeLifetimeSlot<PA>> {
        self.slots.get(Self::index(instance)).filter(|slot| slot.cell.lifetime().instance_id == instance)
    }
    pub fn get_mut(&mut self, instance: u32) -> Option<&mut NativeLifetimeSlot<PA>> {
        self.slots.get_mut(Self::index(instance)).filter(|slot| slot.cell.lifetime().instance_id == instance)
    }
    pub fn remove_uncreated(&mut self, instance: u32) -> Result<(), &'static str> {
        let slot = self.get(instance).ok_or("opening lifecycle missing")?;
        if slot.native_created || slot.cell.owner().is_some() || slot.pending_creation.is_some() {
            return Err("created native owner cannot be forgotten");
        }
        drop(self.slots.take(Self::index(instance)));
        Ok(())
    }
    pub fn pending_opening(&self)->Option<u32>{self.slots.iter().find(|slot|!slot.native_created&&slot.pending_creation.is_some()).map(|slot|slot.cell.lifetime().instance_id)}
    pub fn next_patch_receipt(&mut self, instance: u32) -> Option<semio_framework::kernel::ActorUiPatchReceipt> {
        let slot = self.get_mut(instance).filter(|slot| slot.cell.is_live())?;
        slot.patch_sequence = slot.patch_sequence.checked_add(1)?;
        Some(semio_framework::kernel::ActorUiPatchReceipt { lifetime: slot.cell.lifetime(), patch_sequence: slot.patch_sequence })
    }
    pub fn next_work(&mut self) -> Option<u32> {
        let index = (0..semio_framework_plugin_reactor_tasks::tasks_fixed_slots::PLUGIN_REACTOR_INSTANCE_SLOTS)
            .map(|offset| (self.cursor + offset) % semio_framework_plugin_reactor_tasks::tasks_fixed_slots::PLUGIN_REACTOR_INSTANCE_SLOTS)
            .find(|index| self.slots.get(*index).is_some_and(|slot| !slot.cell.is_live() || slot.cell.owner().is_some_and(|owner| owner.request.is_some())))?;
        self.cursor = (index + 1) % semio_framework_plugin_reactor_tasks::tasks_fixed_slots::PLUGIN_REACTOR_INSTANCE_SLOTS;
        self.slots.get(index).map(|slot| slot.cell.lifetime().instance_id)
    }
    pub fn has_work(&self) -> bool {
        self.slots.iter().any(|slot| !slot.cell.is_live() || slot.cell.owner().is_some_and(|owner| owner.request.is_some()))
    }
    pub fn prepare_turn<P: ClosePeers + ?Sized>(&mut self, instance: u32, peers: &P) -> Result<Option<ActorInstanceLifecycleReceipt>, semio_framework::Fault> {
        let slot = self.get_mut(instance).ok_or_else(|| crate::life_reactor_close::reactor_close_fault("turn lifecycle missing"))?;
        slot.cell.prepare_retired(peers)?;
        slot.cell.release_owner_step(1, 4096, peers)?;
        if slot.cell.staged_ack.is_some() && (slot.cell.phase != Phase::Retired || slot.cell.owner_released) {
            return Ok(None);
        }
        Ok(slot.cell.retained_receipt())
    }
    pub fn finish_turn(&mut self, instance: u32, executing_us: Option<u64>) -> Result<(), &'static str> {
        let slot = self.get_mut(instance).ok_or("turn lifecycle missing")?;
        slot.cell.finish_turn(executing_us, true)?;
        if slot.cell.is_released() {
            drop(self.slots.take(Self::index(instance)));
        }
        Ok(())
    }
}

impl<PA: semio_framework_plugin_reactor_app::PluginApp> Drop for NativeLifecycleRegistry<PA> {
    /// 🧹️ The teardown contract: every lifetime this registry handed out reached its terminal exact
    /// ACK before the registry itself went away. It is asserted only on the ORDINARY drop path.
    /// While the thread is already unwinding, a panic raised here is a panic inside a destructor
    /// during cleanup — a non-unwinding panic that `SIGABRT`s the whole process. In a test binary
    /// that turns one failed law into a silent cancellation of every law scheduled after it (this
    /// is exactly how a single red retention reading took `plugin_builder_contract_tests` down with
    /// it), and in the host it replaces a typed fault with a dead process. The unfinished lifetime
    /// is a consequence of whatever is already unwinding, never the first cause, so the original
    /// panic is the one that must reach the operator.
    fn drop(&mut self) {
        assert!(self.slots.iter().next().is_none() || std::thread::panicking(), "runtime lifetimes require terminal exact ACK before teardown");
    }
}

#[doc(hidden)]
#[cfg(test)]
#[path = "🧪️tests/🚪️lifetime/🦀️.rs"]
pub mod tests;
//#endregion 🚪️GuestLifecycleAuthority
