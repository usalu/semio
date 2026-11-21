//! 🪢 Carries a sealed edit through the existing Channel and Port physical message boundaries.

use super::{Backbone, BackboneChannelPorts, BackboneMessage, Backbones, decode_hot_backbone_message_exact};
use semio_framework_value::{OriginalRetirementPoll, RetainedAllocationEffect, RetainedCloneProgress, RetainedCloneStep, RetirementDemand, RetirementPollExit,
    ValueError, ValueRefusalKind, retained_clone::retained_clone_grant_admits};
use semio_os_spr::{CausalPayload, VerifiedAtomicEdit, decode_verified_atomic_backbone,
    atomic_group::{AtomicBackboneDigest, AtomicBackbonePart, AtomicBackboneSections, AtomicBackboneSender}};
use semio_os_vcs::VcsError;
use std::mem::{ManuallyDrop, size_of};

#[path = "📥️atomic-inbox.rs"]
mod inbox;
#[path = "📤️atomic-actor-send.rs"]
mod actor_send;
pub use inbox::{AtomicBackboneInbox, AtomicDocumentBackboneWork, AtomicInboxStep};

fn invalid() -> ValueError { ValueError::literal(ValueRefusalKind::InvariantViolated, "atomic backbone custody frontier changed") }

fn progress(demand: RetirementDemand) -> RetainedCloneProgress {
    RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, released_bytes: demand.release_bytes, ..Default::default() }
}

/// 📤️ Owns the exact retryable next frame until the attached endpoint accepts it.
pub struct AtomicBackboneDelivery {
    binding: [u8; 32],
    sender: AtomicBackboneSender,
    frame: ManuallyDrop<Option<Vec<u8>>>,
    actor_wire: ManuallyDrop<Option<Vec<u8>>>,
    sent: bool,
}

impl AtomicBackboneDelivery {
    pub fn begin(original: &CausalPayload, binding: [u8; 32]) -> Result<Self, ValueError> {
        Ok(Self { binding, sender: AtomicBackboneSender::begin(original)?, frame: ManuallyDrop::new(None), actor_wire: ManuallyDrop::new(None), sent: false })
    }

    pub fn check_binding(&self, binding: &[u8; 32]) -> Result<(), ValueError> {
        if &self.binding != binding { return Err(invalid()); }
        Ok(())
    }

    pub fn demands(&self) -> Result<RetirementDemand, ValueError> {
        if self.sent { return Ok(self.frame_retirement()); }
        if self.frame.is_some() { return Ok(Default::default()); }
        self.sender.next_demand()
    }

    pub fn advance(
        &mut self, original: &CausalPayload, effect: &mut RetainedAllocationEffect<'_>,
    ) -> Result<RetainedCloneProgress, ValueError> {
        if self.sent { return self.retire_frame(effect); }
        self.sender.advance(original, &mut self.frame, effect)
    }

    pub async fn send_ready(&mut self, backbone: &mut Backbones) -> Result<bool, VcsError> {
        if self.actor_wire.is_some() { return Err(VcsError::InverseRefused(invalid())); }
        let Some(frame) = self.frame.as_ref().filter(|_| !self.sent) else { return Ok(false); };
        send_atomic_frame(backbone, frame).await?;
        self.sent = true;
        Ok(true)
    }

    fn frame_retirement(&self) -> RetirementDemand {
        RetirementDemand { copy_bytes: size_of::<Option<Vec<u8>>>() + size_of::<bool>(),
            release_bytes: self.frame.as_ref().map_or(0, Vec::capacity), depth: 1, ..Default::default() }
    }

    fn retire_frame(&mut self, effect: &mut RetainedAllocationEffect<'_>) -> Result<RetainedCloneProgress, ValueError> {
        let demand = self.frame_retirement();
        if self.frame.is_none() || !retained_clone_grant_admits(effect.grant(), demand) || !effect.performed(1)? {
            return Ok(Default::default());
        }
        drop(self.frame.take());
        self.sent = false;
        Ok(progress(demand))
    }

    pub fn close_demand(&self) -> RetirementDemand {
        if self.actor_wire.is_some() { return self.actor_wire_close_demand(); }
        if self.frame.is_some() { self.frame_retirement() } else { self.sender.close_demand() }
    }

    pub fn close(&mut self, effect: &mut RetainedAllocationEffect<'_>) -> Result<RetainedCloneProgress, ValueError> {
        if self.actor_wire.is_some() { return self.close_actor_wire(effect); }
        if self.frame.is_some() { self.retire_frame(effect) } else { self.sender.close(effect) }
    }

    pub fn terminal_is_empty(&self) -> bool { self.actor_wire.is_none() && self.frame.is_none() && self.sender.terminal_is_empty() }
}

impl Drop for AtomicBackboneDelivery {
    fn drop(&mut self) {
        assert!((self.frame.is_none() && self.actor_wire.is_none()) || std::thread::panicking(), "atomic backbone delivery still owns its paid frame");
    }
}

/// 🚇️ Sends one canonical frame through the attached endpoint's ordinary binary envelope.
pub async fn send_atomic_frame(backbone: &mut Backbones, frame: &[u8]) -> Result<(), VcsError> {
    AtomicBackbonePart::parse(frame).map_err(VcsError::InverseRefused)?;
    let envelopes = CausalPayload::try_from_slice(frame).map_err(VcsError::InverseRefused)?;
    backbone.send(BackboneMessage::Mutations { envelopes }).await
}

pub(crate) fn atomic_wire_front(bytes: &[u8]) -> bool {
    const MUTATIONS_BYTES_FIELD: [u8; 6] = [1, 1, 0, 1, 0, 8];
    let Some(after_header) = bytes.strip_prefix(&MUTATIONS_BYTES_FIELD) else { return false; };
    let Some(last_length_byte) = after_header.iter().take(3).position(|byte| byte & 0x80 == 0) else { return false; };
    AtomicBackbonePart::matches(&after_header[last_length_byte + 1..])
}

pub(crate) fn atomic_message_front(message: &BackboneMessage) -> bool {
    matches!(message, BackboneMessage::Mutations { envelopes } if AtomicBackbonePart::matches_paged(envelopes))
}

/// 📥️ Extracts only the front atomic frame from the original authenticated endpoint queue.
pub fn take_atomic_front(backbone: &Backbones) -> Result<Option<CausalPayload>, VcsError> {
    let error = || VcsError::Backbone("original atomic backbone endpoint is unavailable".into());
    let message = match backbone {
        Backbones::Port(port) => {
            let Some(BackboneChannelPorts::Actor(channel)) = port.channel.as_deref() else { return Ok(None); };
            let wire = {
                let mut state = channel.state.lock().map_err(|_| error())?;
                if !state.accepting || state.uri != port.uri { return Err(error()); }
                if !state.inbound.front().is_some_and(|frame| atomic_wire_front(frame)) { return Ok(None); }
                let original = state.inbound.pop_front().ok_or_else(error)?;
                state.inbound_bytes -= original.len();
                original
            };
            decode_hot_backbone_message_exact(&wire)?
        }
        Backbones::Channel(channel) => {
            let Some(inbound) = channel.inbound.as_ref() else { return Ok(None); };
            let mut queue = inbound.lock().map_err(|_| error())?;
            if !queue.front().is_some_and(atomic_message_front) { return Ok(None); }
            queue.pop_front().ok_or_else(error)?
        }
        Backbones::Memory(_) => return Ok(None),
    };
    match message {
        BackboneMessage::Mutations { envelopes } => Ok(Some(envelopes)),
        _ => Err(error()),
    }
}

/// 📥️ Holds one stable original receive extent for an authenticated document actor binding.
pub struct AtomicBackboneReceive {
    binding: [u8; 32],
    sections: ManuallyDrop<Option<Box<AtomicBackboneSections>>>,
    digest: Option<AtomicBackboneDigest>,
    sealed: bool,
}

impl AtomicBackboneReceive {
    pub fn birth_demand() -> RetirementDemand {
        let mut demand = AtomicBackboneSections::birth_demand();
        demand.copy_bytes += size_of::<Self>();
        demand.capacity_bytes = size_of::<AtomicBackboneSections>();
        demand
    }

    pub fn admit(first: &CausalPayload, binding: [u8; 32], effect: &mut RetainedAllocationEffect<'_>)
        -> Result<Option<(Self, RetainedCloneProgress)>, ValueError> {
        let part = AtomicBackbonePart::parse_paged(first)?;
        let demand = Self::birth_demand();
        if !retained_clone_grant_admits(effect.grant(), demand) || !effect.performed(1)? { return Ok(None); }
        let sections = Box::new(AtomicBackboneSections::new(part.key)?);
        let receipt = RetainedCloneProgress { retained_capacity_bytes: demand.capacity_bytes, ..progress(demand) };
        Ok(Some((Self { binding, sections: ManuallyDrop::new(Some(sections)), digest: None, sealed: false }, receipt)))
    }

    pub fn push(&mut self, binding: &[u8; 32], original: &mut Option<CausalPayload>, effect: &mut RetainedAllocationEffect<'_>)
        -> Result<RetainedCloneProgress, ValueError> {
        if &self.binding != binding || self.digest.is_some() || self.sealed { return Err(invalid()); }
        let part = AtomicBackbonePart::parse_paged(original.as_ref().ok_or_else(invalid)?)?;
        let (_, receipt) = self.sections.as_mut().ok_or_else(invalid)?.admit(part, original, effect)?;
        Ok(receipt)
    }

    pub fn can_seal(&self) -> bool { self.digest.is_some() || self.sections.as_ref().is_some_and(|owner| owner.complete().is_some()) }

    pub fn is_sealed(&self) -> bool { self.sealed }

    pub fn seal_at_document(&mut self, binding: &[u8; 32], document: &str, effect: &mut RetainedAllocationEffect<'_>)
        -> Result<(Option<VerifiedAtomicEdit>, RetainedCloneProgress), VcsError> {
        self.seal_for_binding(binding, document, None, effect)
    }

    pub fn seal(&mut self, binding: &[u8; 32], document: &str, actor: &str, effect: &mut RetainedAllocationEffect<'_>)
        -> Result<(Option<VerifiedAtomicEdit>, RetainedCloneProgress), VcsError> {
        self.seal_for_binding(binding, document, Some(actor), effect)
    }

    fn seal_for_binding(&mut self, binding: &[u8; 32], document: &str, actor: Option<&str>, effect: &mut RetainedAllocationEffect<'_>)
        -> Result<(Option<VerifiedAtomicEdit>, RetainedCloneProgress), VcsError> {
        if &self.binding != binding || self.sealed { return Err(VcsError::InverseRefused(invalid())); }
        let sections = self.sections.as_ref().ok_or_else(|| VcsError::InverseRefused(invalid()))?;
        if self.digest.is_none() { self.digest = sections.complete(); }
        let Some(digest) = self.digest.as_mut() else { return Ok((None, Default::default())); };
        let (verified, receipt) = digest.advance(sections, 4096, effect).map_err(VcsError::InverseRefused)?;
        let Some(verified) = verified else { return Ok((None, receipt)); };
        let edit = match actor {
            Some(actor) => decode_verified_atomic_backbone(&verified, document, actor),
            None => semio_os_spr::decode_verified_atomic_backbone_at_document(&verified, document),
        }.map_err(|error| VcsError::Deserialize(error.to_string()))?;
        self.sealed = true;
        Ok((Some(edit), receipt))
    }

    pub fn close(&mut self, body: usize, effect: &mut RetainedAllocationEffect<'_>) -> OriginalRetirementPoll {
        let result = (|| -> Result<RetainedCloneStep, RetirementPollExit> {
            let Some(sections) = self.sections.as_mut() else { return Ok(RetainedCloneStep::Complete(Default::default())); };
            if !sections.terminal_is_empty() {
                return RetirementPollExit::resume(sections.close(body, effect)).map(|step| RetainedCloneStep::Progress(step.progress()));
            }
            let demand = RetirementDemand {copy_bytes: size_of::<Option<Box<AtomicBackboneSections>>>(), depth: 1,
                release_bytes: size_of::<AtomicBackboneSections>(), ..Default::default() };
            if !retained_clone_grant_admits(effect.grant(), demand) || !effect.performed(1)? {
                return Ok(RetainedCloneStep::Progress(Default::default()));
            }
            drop(self.sections.take());
            Ok(RetainedCloneStep::Complete(progress(demand)))
        })();
        RetirementPollExit::settle(effect, result)
    }

    pub fn terminal_is_empty(&self) -> bool { self.sections.is_none() }
}

impl Drop for AtomicBackboneReceive {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty() || std::thread::panicking(), "atomic backbone receiver still owns its paid frames");
    }
}
