//#region 🚪️GuestLifecycleAuthority
use semio_framework::kernel::{ActorInstanceCloseRequest, ActorInstanceLifecycleAck, ActorInstanceLifecycleReceipt, ActorInstanceLifetime, ActorInstanceOpenRequest};
use std::mem::ManuallyDrop;

/// 🔗️ Process-local descendant identity; its captured weak app lease keeps the allocation address reserved.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) struct NativeCloseKey { lifetime: ActorInstanceLifetime, allocation: usize }

impl NativeCloseKey {
    pub(crate) fn capture<PA: crate::app::PluginApp + 'static>(lifetime: ActorInstanceLifetime, lease: &crate::plugin_runtime::PluginInstanceCloseLease<PA>) -> Result<Self, &'static str> {
        let (instance, allocation) = lease.allocation_identity();
        if !lifetime.is_valid() || lifetime.instance_id != instance { return Err("native close allocation does not match guest lifetime"); }
        Ok(Self { lifetime, allocation })
    }
    pub(super) fn instance(self) -> u32 { self.lifetime.instance_id }
    pub(super) fn lifetime(self) -> ActorInstanceLifetime { self.lifetime }
    #[cfg(test)]
    pub(super) fn fixture(instance_id: u32, guest_lifetime: u64) -> Self {
        Self { lifetime: ActorInstanceLifetime { activation_generation: 1, instance_id, guest_lifetime }, allocation: 1 }
    }
}

pub(crate) struct GuestLifecycleSerial(u64);

impl GuestLifecycleSerial {
    pub(crate) fn new(last: u64) -> Self { Self(last) }
    pub(crate) fn next(&mut self) -> Result<u64, &'static str> {
        let next = self.0.checked_add(1).ok_or("guest lifetime serial exhausted")?;
        self.0 = next;
        Ok(next)
    }
}

mod terminal_owner { pub(super) trait Sealed {} }

pub(crate) enum GuestTerminalRelease { Pending, Released }

pub(crate) trait GuestLifetimeOwner: terminal_owner::Sealed {
    fn terminal_is_empty(&self) -> Result<bool, &'static str>;
    fn release_terminal(owner: &mut Option<Self>, maximum_items: usize, maximum_bytes: usize) -> Result<GuestTerminalRelease, &'static str> where Self: Sized;
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Phase { Opening, Captured, Live, Accepted, Closing, Retired, Released }

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
        if !open.is_valid() || guest_lifetime == 0 { return Err("invalid guest open authority"); }
        Ok(Self {
            open,
            lifetime: ActorInstanceLifetime { activation_generation: open.activation_generation, instance_id: open.instance_id, guest_lifetime },
            owner: ManuallyDrop::new(None), phase: Phase::Opening, close: None, receipt: None, staged_ack: None, previous_ack: None, owner_released: false,
        })
    }

    pub(crate) fn lifetime(&self) -> ActorInstanceLifetime { self.lifetime }
    pub(crate) fn matches_open(&self, open: ActorInstanceOpenRequest) -> bool { self.open == open }
    pub(crate) fn owner(&self) -> Option<&O> { self.owner.as_ref() }
    pub(crate) fn owner_mut(&mut self) -> Option<&mut O> { self.owner.as_mut() }
    pub(crate) fn retained_receipt(&self) -> Option<ActorInstanceLifecycleReceipt> { self.receipt }
    pub(crate) fn is_live(&self) -> bool { self.phase == Phase::Live }
    pub(crate) fn is_released(&self) -> bool { self.phase == Phase::Released }
    pub(crate) fn is_closing(&self) -> bool { matches!(self.phase, Phase::Accepted | Phase::Closing | Phase::Retired) }

    pub(crate) fn install_owner(&mut self, owner: O) -> Result<(), O> {
        if self.phase != Phase::Opening || self.owner.is_some() { return Err(owner); }
        *self.owner = Some(owner);
        self.receipt = Some(ActorInstanceLifecycleReceipt::Captured { lifetime: self.lifetime, request_sequence: self.open.request_sequence });
        self.phase = Phase::Captured;
        Ok(())
    }

    pub(crate) fn validate_close(&self, request: ActorInstanceCloseRequest) -> Result<(), &'static str> {
        if !request.is_valid() || request.lifetime != self.lifetime { return Err("foreign guest close authority"); }
        if let Some((accepted, _)) = self.close {
            return if accepted == request { Ok(()) } else { Err("different close request already owns the lifetime") };
        }
        if !self.is_live() { return Err("captured receipt must be acknowledged before close admission"); }
        Ok(())
    }

    pub(crate) fn record_close_admission(&mut self, request: ActorInstanceCloseRequest, close_generation: u64) -> Result<(), &'static str> {
        self.validate_close(request)?;
        if close_generation == 0 { return Err("invalid admitted native close generation"); }
        if let Some((_, generation)) = self.close {
            return if generation == close_generation { Ok(()) } else { Err("native close generation changed") };
        }
        self.close = Some((request, close_generation));
        self.receipt = Some(ActorInstanceLifecycleReceipt::Accepted { lifetime: self.lifetime, request_sequence: request.request_sequence, close_generation });
        self.phase = Phase::Accepted;
        Ok(())
    }

    pub(crate) fn prepare_retired(&mut self) -> Result<bool, &'static str> {
        if self.phase == Phase::Retired { return Ok(true); }
        if self.phase != Phase::Closing { return Ok(false); }
        if !self.owner.as_ref().ok_or("guest lifetime lost its native owner")?.terminal_is_empty()? { return Ok(false); }
        let (request, close_generation) = self.close.ok_or("guest lifetime lost close admission")?;
        self.receipt = Some(ActorInstanceLifecycleReceipt::Retired { lifetime: self.lifetime, request_sequence: request.request_sequence, close_generation });
        self.phase = Phase::Retired;
        Ok(true)
    }

    pub(crate) fn stage_ack(&mut self, ack: ActorInstanceLifecycleAck) -> Result<(), &'static str> {
        if self.previous_ack == Some(ack) { return Ok(()); }
        if self.receipt != Some(ack.receipt) { return Err("ACK does not name the exact retained receipt"); }
        if self.staged_ack.is_some_and(|staged| staged != ack) { return Err("different ACK already staged"); }
        self.staged_ack = Some(ack);
        Ok(())
    }

    /// ♻️ The sealed domain releases its final shell before the clock verdict; exact completion survives retry.
    pub(crate) fn release_owner_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<(), &'static str> {
        if self.phase != Phase::Retired || self.staged_ack.is_none() || self.owner_released || maximum_items == 0 { return Ok(()); }
        if !self.owner.as_ref().ok_or("terminal receipt lost its owner")?.terminal_is_empty()? { return Err("terminal receipt still owns native descendants"); }
        match O::release_terminal(&mut self.owner, maximum_items, maximum_bytes)? {
            GuestTerminalRelease::Pending => {
                if self.owner.is_none() { return Err("pending terminal release lost its structural owner"); }
            },
            GuestTerminalRelease::Released => {
                if self.owner.is_some() { return Err("terminal release receipt still owns its source shell"); }
                self.owner_released = true;
            },
        }
        Ok(())
    }

    /// ⏱️ Commits receipt consumption only after the exact turn has a successful real-clock verdict.
    pub(crate) fn finish_turn(&mut self, started_us: Option<u64>, now_us: impl FnOnce() -> Option<u64>, succeeded: bool) -> Result<Option<ActorInstanceLifecycleReceipt>, &'static str> {
        if !succeeded { return Err("guest lifecycle turn failed; receipt retained"); }
        let phase = if let Some(ack) = self.staged_ack {
            if self.receipt != Some(ack.receipt) { return Err("staged ACK no longer matches receipt"); }
            Some(match self.phase {
                Phase::Captured => Phase::Live,
                Phase::Accepted => Phase::Closing,
                Phase::Retired => {
                    if !self.owner_released || self.owner.is_some() { return Err("terminal shell release is still pending"); }
                    Phase::Released
                },
                _ => return Err("ACK cannot consume this lifecycle phase"),
            })
        } else { None };
        let elapsed = now_us().zip(started_us).and_then(|(end, start)| end.checked_sub(start)).ok_or("guest lifecycle clock missing or backward; receipt retained")?;
        if semio_framework_trace::interactive_step_contract_violated(elapsed) { return Err("guest lifecycle turn exceeded strict time authority; receipt retained"); }
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

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
//#endregion 🚪️GuestLifecycleAuthority
