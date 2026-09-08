//#region 🚪️GuestLifecycleAuthority
use semio_framework::kernel::{ActorInstanceCloseRequest, ActorInstanceLifecycleAck, ActorInstanceLifecycleReceipt, ActorInstanceLifetime, ActorInstanceOpenRequest};
use std::mem::ManuallyDrop;

/// 🔗️ Process-local descendant identity; its captured weak app lease keeps the allocation address reserved.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) struct NativeCloseKey {
    lifetime: ActorInstanceLifetime,
    allocation: usize,
}

impl NativeCloseKey {
    pub(crate) fn capture<PA: crate::app::PluginApp + 'static>(lifetime: ActorInstanceLifetime, lease: &crate::plugin_runtime::PluginInstanceCloseLease<PA>) -> Result<Self, &'static str> {
        let (instance, allocation) = lease.allocation_identity();
        if !lifetime.is_valid() || lifetime.instance_id != instance {
            return Err("native close allocation does not match guest lifetime");
        }
        Ok(Self { lifetime, allocation })
    }
    pub(super) fn instance(self) -> u32 {
        self.lifetime.instance_id
    }
    pub(super) fn lifetime(self) -> ActorInstanceLifetime {
        self.lifetime
    }
    #[cfg(test)]
    pub(super) fn fixture(instance_id: u32, guest_lifetime: u64) -> Self {
        Self { lifetime: ActorInstanceLifetime { activation_generation: 1, instance_id, guest_lifetime }, allocation: 1 }
    }
}

pub(crate) struct GuestLifecycleSerial(u64);

impl GuestLifecycleSerial {
    pub(crate) fn new(last: u64) -> Self {
        Self(last)
    }
    pub(crate) fn next(&mut self) -> Result<u64, &'static str> {
        let next = self.0.checked_add(1).ok_or("guest lifetime serial exhausted")?;
        self.0 = next;
        Ok(next)
    }
}

mod terminal_owner {
    pub(crate) trait Sealed {}
}

pub(crate) enum GuestTerminalRelease {
    Pending,
    Released,
}

pub(crate) trait GuestLifetimeOwner: terminal_owner::Sealed {
    fn terminal_is_empty(&self) -> Result<bool, &'static str>;
    fn release_terminal(owner: &mut Option<Self>, maximum_items: usize, maximum_bytes: usize) -> Result<GuestTerminalRelease, &'static str>
    where
        Self: Sized;
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Phase {
    Opening,
    Captured,
    Live,
    Accepted,
    Closing,
    Retired,
    Released,
}

/// 🪪️ Lives in the runtime registry before callbacks; an unwind never removes its owner.
pub(crate) struct GuestLifecycleCell<O: GuestLifetimeOwner> {
    open: ActorInstanceOpenRequest,
    lifetime: ActorInstanceLifetime,
    owner: ManuallyDrop<Option<O>>,
    phase: Phase,
    close: Option<(ActorInstanceCloseRequest, u64)>,
    receipt: Option<ActorInstanceLifecycleReceipt>,
    staged_ack: Option<ActorInstanceLifecycleAck>,
    previous_ack: Option<ActorInstanceLifecycleAck>,
    owner_released: bool,
}

impl<O: GuestLifetimeOwner> GuestLifecycleCell<O> {
    pub(crate) fn admit(open: ActorInstanceOpenRequest, guest_lifetime: u64) -> Result<Self, &'static str> {
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

    pub(crate) fn lifetime(&self) -> ActorInstanceLifetime {
        self.lifetime
    }
    pub(crate) fn matches_open(&self, open: ActorInstanceOpenRequest) -> bool {
        self.open == open
    }
    pub(crate) fn owner(&self) -> Option<&O> {
        self.owner.as_ref()
    }
    pub(crate) fn owner_mut(&mut self) -> Option<&mut O> {
        self.owner.as_mut()
    }
    pub(crate) fn retained_receipt(&self) -> Option<ActorInstanceLifecycleReceipt> {
        self.receipt
    }
    pub(crate) fn is_live(&self) -> bool {
        self.phase == Phase::Live
    }
    pub(crate) fn is_released(&self) -> bool {
        self.phase == Phase::Released
    }
    pub(crate) fn install_owner(&mut self, owner: O) -> Result<(), O> {
        if self.phase != Phase::Opening || self.owner.is_some() {
            return Err(owner);
        }
        *self.owner = Some(owner);
        self.receipt = Some(ActorInstanceLifecycleReceipt::Captured { lifetime: self.lifetime, request_sequence: self.open.request_sequence });
        self.phase = Phase::Captured;
        Ok(())
    }

    pub(crate) fn validate_close(&self, request: ActorInstanceCloseRequest) -> Result<(), &'static str> {
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

    pub(crate) fn record_close_admission(&mut self, request: ActorInstanceCloseRequest, close_generation: u64) -> Result<(), &'static str> {
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

    pub(crate) fn prepare_retired(&mut self) -> Result<bool, &'static str> {
        if self.phase == Phase::Retired {
            return Ok(true);
        }
        if self.phase != Phase::Closing {
            return Ok(false);
        }
        if !self.owner.as_ref().ok_or("guest lifetime lost its native owner")?.terminal_is_empty()? {
            return Ok(false);
        }
        let (request, close_generation) = self.close.ok_or("guest lifetime lost close admission")?;
        self.receipt = Some(ActorInstanceLifecycleReceipt::Retired { lifetime: self.lifetime, request_sequence: request.request_sequence, close_generation });
        self.phase = Phase::Retired;
        Ok(true)
    }

    pub(crate) fn stage_ack(&mut self, ack: ActorInstanceLifecycleAck) -> Result<(), &'static str> {
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
    pub(crate) fn release_owner_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<(), &'static str> {
        if self.phase != Phase::Retired || self.staged_ack.is_none() || self.owner_released || maximum_items == 0 {
            return Ok(());
        }
        if !self.owner.as_ref().ok_or("terminal receipt lost its owner")?.terminal_is_empty()? {
            return Err("terminal receipt still owns native descendants");
        }
        match O::release_terminal(&mut self.owner, maximum_items, maximum_bytes)? {
            GuestTerminalRelease::Pending => {
                if self.owner.is_none() {
                    return Err("pending terminal release lost its structural owner");
                }
            }
            GuestTerminalRelease::Released => {
                if self.owner.is_some() {
                    return Err("terminal release receipt still owns its source shell");
                }
                self.owner_released = true;
            }
        }
        Ok(())
    }

    /// ⏱️ Commits receipt consumption only after the exact turn has a successful real-clock verdict.
    pub(crate) fn finish_turn(&mut self, started_us: Option<u64>, now_us: impl FnOnce() -> Option<u64>, succeeded: bool) -> Result<Option<ActorInstanceLifecycleReceipt>, &'static str> {
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
        let elapsed = now_us().zip(started_us).and_then(|(end, start)| end.checked_sub(start)).ok_or("guest lifecycle clock missing or backward; receipt retained")?;
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

pub(super) const GUEST_LIFECYCLE_TURN_DEADLINE: &str = "guest lifecycle turn exceeded strict time authority; receipt retained";

type NativeCell<PA> = GuestLifecycleCell<NativeLifetimeOwner<PA>>;

/// 🧷️ The native allocation and every close participant remain attached until terminal ACK.
pub(crate) struct NativeLifetimeOwner<PA: crate::app::PluginApp> {
    lease: crate::plugin_runtime::PluginInstanceCloseLease<PA>,
    key: NativeCloseKey,
    request: Option<ActorInstanceCloseRequest>,
    reserved: [bool; 4],
    active: [bool; 4],
    released: [bool; 4],
}

impl<PA: crate::app::PluginApp> NativeLifetimeOwner<PA> {
    pub(crate) fn from_lease(lifetime: ActorInstanceLifetime, lease: crate::plugin_runtime::PluginInstanceCloseLease<PA>) -> Result<Self, semio_framework::Fault> {
        let key = NativeCloseKey::capture(lifetime, &lease).map_err(super::reactor_close_fault)?;
        Ok(Self { lease, key, request: None, reserved: [false; 4], active: [false; 4], released: [false; 4] })
    }

    pub(crate) fn key(&self) -> NativeCloseKey {
        self.key
    }

    pub(crate) fn request_close(&mut self, request: ActorInstanceCloseRequest, runtime: &crate::plugin_runtime::PluginRuntime<PA>) -> Result<(), semio_framework::Fault> {
        if self.request == Some(request) {
            return Ok(());
        }
        if self.request.is_some() || request.lifetime != self.key.lifetime {
            return Err(super::reactor_close_fault("close owner request changed"));
        }
        super::preflight_reactor_close(self.key)?;
        super::PATCHES.with(|patches| patches.preflight_close_instance(self.key)).map_err(super::reactor_close_fault)?;
        super::pending::with_state(|pending| pending.try_borrow().map_err(|_| "pending close preflight busy")?.preflight_close_instance(self.key)).map_err(super::reactor_close_fault)?;
        super::preflight_cold_pair_close(self.key)?;
        self.lease.preflight_close(runtime)?;
        self.request = Some(request);
        Ok(())
    }

    pub(crate) fn advance_admission(&mut self, runtime: &crate::plugin_runtime::PluginRuntime<PA>) -> Result<Option<(ActorInstanceCloseRequest, u64)>, semio_framework::Fault> {
        let Some(request) = self.request else { return Ok(None) };
        if !self.reserved[0] {
            super::reserve_reactor_close(self.key)?;
            self.reserved[0] = true;
        }
        if !self.reserved[1] {
            super::PATCHES.with(|patches| patches.reserve_close_instance(self.key)).map_err(super::reactor_close_fault)?;
            self.reserved[1] = true;
        }
        if !self.reserved[2] {
            super::pending::with_state(|pending| pending.borrow_mut().reserve_close_instance(self.key)).map_err(super::reactor_close_fault)?;
            self.reserved[2] = true;
        }
        if !self.reserved[3] {
            super::reserve_cold_pair_close(self.key)?;
            self.reserved[3] = true;
        }
        if !self.active[3] {
            super::activate_cold_pair_close(self.key)?;
            self.active[3] = true;
        }
        self.lease.begin_close(runtime)?;
        if !self.active[0] {
            super::activate_reactor_close(self.key)?;
            self.active[0] = true;
        }
        if !self.active[1] {
            super::PATCHES.with(|patches| patches.activate_close_instance(self.key)).map_err(super::reactor_close_fault)?;
            self.active[1] = true;
        }
        if !self.active[2] {
            super::pending::with_state(|pending| pending.borrow_mut().activate_close_instance(self.key)).map_err(super::reactor_close_fault)?;
            self.active[2] = true;
        }
        super::JOB_RENDER_BINDINGS.with(|bindings| bindings.borrow_mut().close_instance(self.key.instance()));
        Ok(Some((request, self.lease.close_generation().ok_or_else(|| super::reactor_close_fault("native close generation missing"))?)))
    }
}

impl<PA: crate::app::PluginApp> terminal_owner::Sealed for NativeLifetimeOwner<PA> {}

impl<PA: crate::app::PluginApp> GuestLifetimeOwner for NativeLifetimeOwner<PA> {
    fn terminal_is_empty(&self) -> Result<bool, &'static str> {
        if !self.lease.is_retired().map_err(|_| "native close terminal unavailable")? {
            return Ok(false);
        }
        if !self.released[0] && !super::reactor_close_complete(self.key).map_err(|_| "reactor close terminal unavailable")? {
            return Ok(false);
        }
        if !self.released[1] && !super::PATCHES.with(|patches| patches.close_instance_complete(self.key))? {
            return Ok(false);
        }
        if !self.released[2] && !super::pending::with_state(|pending| pending.borrow().close_instance_complete(self.key))? {
            return Ok(false);
        }
        if !self.released[3] && !super::cold_pair_close_complete(self.key)? {
            return Ok(false);
        }
        Ok(true)
    }

    fn release_terminal(owner: &mut Option<Self>, maximum_items: usize, maximum_bytes: usize) -> Result<GuestTerminalRelease, &'static str> {
        if maximum_items == 0 || maximum_bytes < size_of::<Self>() {
            return Ok(GuestTerminalRelease::Pending);
        }
        let retained = owner.as_mut().ok_or("terminal native owner missing")?;
        if !retained.terminal_is_empty()? {
            return Err("native descendants are not terminal");
        }
        if !retained.released[0] {
            super::release_reactor_close(retained.key).map_err(|_| "reactor close release unavailable")?;
            retained.released[0] = true;
            return Ok(GuestTerminalRelease::Pending);
        }
        if !retained.released[1] {
            super::PATCHES.with(|patches| patches.release_close_instance(retained.key))?;
            retained.released[1] = true;
            return Ok(GuestTerminalRelease::Pending);
        }
        if !retained.released[2] {
            super::pending::with_state(|pending| pending.borrow_mut().release_close_instance(retained.key))?;
            retained.released[2] = true;
            return Ok(GuestTerminalRelease::Pending);
        }
        if !retained.released[3] {
            super::release_cold_pair_close(retained.key)?;
            retained.released[3] = true;
            return Ok(GuestTerminalRelease::Pending);
        }
        drop(owner.take());
        Ok(GuestTerminalRelease::Released)
    }
}

pub(crate) struct NativeLifetimeSlot<PA: crate::app::PluginApp> {
    pub(crate) cell: NativeCell<PA>,
    pub(crate) native_created: bool,
    patch_sequence: u64,
}

/// 🗃️ Fixed allocation-bound ownership belongs to the typed runtime, never a copied thread-local key.
pub(crate) struct NativeLifecycleRegistry<PA: crate::app::PluginApp> {
    slots: super::ReactorFixedSlots<NativeLifetimeSlot<PA>>,
    serial: GuestLifecycleSerial,
    cursor: usize,
}

impl<PA: crate::app::PluginApp> NativeLifecycleRegistry<PA> {
    pub(crate) fn new() -> Self {
        Self { slots: super::ReactorFixedSlots::new(), serial: GuestLifecycleSerial::new(0), cursor: 0 }
    }
    fn index(instance: u32) -> usize {
        instance as usize % super::PLUGIN_REACTOR_INSTANCE_SLOTS
    }
    pub(crate) fn admit(&mut self, open: ActorInstanceOpenRequest) -> Result<bool, &'static str> {
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
        self.slots.insert_admitted(index, NativeLifetimeSlot { cell, native_created: false, patch_sequence: 0 });
        Ok(true)
    }
    pub(crate) fn get(&self, instance: u32) -> Option<&NativeLifetimeSlot<PA>> {
        self.slots.get(Self::index(instance)).filter(|slot| slot.cell.lifetime().instance_id == instance)
    }
    pub(crate) fn get_mut(&mut self, instance: u32) -> Option<&mut NativeLifetimeSlot<PA>> {
        self.slots.get_mut(Self::index(instance)).filter(|slot| slot.cell.lifetime().instance_id == instance)
    }
    pub(crate) fn remove_uncreated(&mut self, instance: u32) -> Result<(), &'static str> {
        let slot = self.get(instance).ok_or("opening lifecycle missing")?;
        if slot.native_created || slot.cell.owner().is_some() {
            return Err("created native owner cannot be forgotten");
        }
        drop(self.slots.take(Self::index(instance)));
        Ok(())
    }
    pub(crate) fn next_patch_receipt(&mut self, instance: u32) -> Option<semio_framework::kernel::ActorUiPatchReceipt> {
        let slot = self.get_mut(instance).filter(|slot| slot.cell.is_live())?;
        slot.patch_sequence = slot.patch_sequence.checked_add(1)?;
        Some(semio_framework::kernel::ActorUiPatchReceipt { lifetime: slot.cell.lifetime(), patch_sequence: slot.patch_sequence })
    }
    pub(crate) fn next_work(&mut self) -> Option<u32> {
        let index = (0..super::PLUGIN_REACTOR_INSTANCE_SLOTS)
            .map(|offset| (self.cursor + offset) % super::PLUGIN_REACTOR_INSTANCE_SLOTS)
            .find(|index| self.slots.get(*index).is_some_and(|slot| !slot.cell.is_live() || slot.cell.owner().is_some_and(|owner| owner.request.is_some())))?;
        self.cursor = (index + 1) % super::PLUGIN_REACTOR_INSTANCE_SLOTS;
        self.slots.get(index).map(|slot| slot.cell.lifetime().instance_id)
    }
    pub(crate) fn has_work(&self) -> bool {
        self.slots.iter().any(|slot| !slot.cell.is_live() || slot.cell.owner().is_some_and(|owner| owner.request.is_some()))
    }
    pub(crate) fn prepare_turn(&mut self, instance: u32) -> Result<Option<ActorInstanceLifecycleReceipt>, &'static str> {
        let slot = self.get_mut(instance).ok_or("turn lifecycle missing")?;
        slot.cell.prepare_retired()?;
        slot.cell.release_owner_step(1, 4096)?;
        if slot.cell.staged_ack.is_some() && (slot.cell.phase != Phase::Retired || slot.cell.owner_released) {
            return Ok(None);
        }
        Ok(slot.cell.retained_receipt())
    }
    pub(crate) fn finish_turn(&mut self, instance: u32, started_us: Option<u64>) -> Result<(), &'static str> {
        let slot = self.get_mut(instance).ok_or("turn lifecycle missing")?;
        slot.cell.finish_turn(started_us, semio_framework_job::default_now_us, true)?;
        if slot.cell.is_released() {
            drop(self.slots.take(Self::index(instance)));
        }
        Ok(())
    }
}

impl<PA: crate::app::PluginApp> Drop for NativeLifecycleRegistry<PA> {
    fn drop(&mut self) {
        assert!(self.slots.iter().next().is_none(), "runtime lifetimes require terminal exact ACK before teardown");
    }
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
//#endregion 🚪️GuestLifecycleAuthority
