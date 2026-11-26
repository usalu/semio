//! 📤️ Posts original prepared bodies through the actor Port's bounded wire queue.

use super::*;
use semio_framework_value::ByteReadSource;

impl AtomicBackboneDelivery {
    pub fn needs_actor_wire(&self) -> bool { !self.sent && self.frame.is_some() }

    pub fn advance_actor_wire(
        &mut self, endpoint: &Backbones, effect: &mut RetainedAllocationEffect<'_>,
    ) -> Result<RetainedCloneProgress, VcsError> {
        let Some(frame) = self.frame.as_ref().filter(|_| !self.sent) else { return Ok(Default::default()); };
        advance_actor_body(frame, endpoint, &mut self.actor_wire, &mut self.sent, effect)
    }

    pub fn advance_ordinary_wire(
        &mut self, body: &dyn ByteReadSource, endpoint: &Backbones, effect: &mut RetainedAllocationEffect<'_>,
    ) -> Result<RetainedCloneProgress, VcsError> {
        if !self.is_ordinary_message() || self.sent { return Err(VcsError::InverseRefused(invalid())); }
        advance_actor_body(body, endpoint, &mut self.actor_wire, &mut self.sent, effect)
    }

    pub(super) fn actor_wire_close_demand(&self) -> RetirementDemand {
        RetirementDemand { copy_bytes: size_of::<Option<Vec<u8>>>(), release_bytes: self.actor_wire.as_ref().map_or(0, Vec::capacity), depth: 1, ..Default::default() }
    }

    pub(super) fn close_actor_wire(&mut self, effect: &mut RetainedAllocationEffect<'_>) -> Result<RetainedCloneProgress, ValueError> {
        let demand = self.actor_wire_close_demand();
        if !retained_clone_grant_admits(effect.grant(), demand) || !effect.performed(1)? {
            return Ok(Default::default());
        }
        drop(self.actor_wire.take());
        Ok(progress(demand))
    }
}

fn advance_actor_body(
    body: &dyn ByteReadSource,
    endpoint: &Backbones,
    actor_wire: &mut Option<Vec<u8>>,
    sent: &mut bool,
    effect: &mut RetainedAllocationEffect<'_>,
) -> Result<RetainedCloneProgress, VcsError> {
        let Backbones::Port(port) = endpoint else { return Err(VcsError::InverseRefused(invalid())); };
        let Some(BackboneChannelPorts::Actor(channel)) = port.channel.as_deref() else { return Err(VcsError::InverseRefused(invalid())); };
        let mut prefix = [1, 1, 0, 1, 0, 8, 0, 0, 0];
        let mut len = body.byte_len();
        if len > semio_os_spr::DOCUMENT_BACKBONE_BATCH_MAXIMUM_PAYLOAD_BYTES { return Err(VcsError::InverseRefused(invalid())); }
        let mut width = 6;
        while len >= 128 {
            prefix[width] = (len as u8 & 127) | 128;
            width += 1;
            len >>= 7;
        }
        prefix[width] = len as u8;
        width += 1;
        let extent = width + body.byte_len();
        let refusal = VcsError::InverseRefused;
        if actor_wire.is_none() {
            let demand = RetirementDemand { copy_bytes: width + size_of::<Option<Vec<u8>>>(), capacity_bytes: extent, depth: 1, ..Default::default() };
            if !retained_clone_grant_admits(effect.grant(), demand) || !effect.performed(1).map_err(refusal)? {
                return Ok(Default::default());
            }
            let mut wire = Vec::new();
            wire.try_reserve_exact(extent).map_err(|_| refusal(invalid()))?;
            wire.extend_from_slice(&prefix[..width]);
            let bytes = wire.capacity();
            *actor_wire = Some(wire);
            return Ok(RetainedCloneProgress { retained_capacity_bytes: bytes, ..progress(demand) });
        }
        let wire = actor_wire.as_mut().unwrap();
        if wire.len() != extent {
            let work = usize::try_from(effect.remaining_work_units().map_err(refusal)?).unwrap_or(usize::MAX);
            let count = (extent - wire.len()).min(4096).min(effect.grant().maximum_copy_bytes).min(work);
            if count == 0 { return Ok(Default::default()); }
            let demand = RetirementDemand { copy_bytes: count, depth: 1, ..Default::default() };
            if !retained_clone_grant_admits(effect.grant(), demand) || !effect.performed(count as u64).map_err(refusal)? {
                return Ok(Default::default());
            }
            let from = wire.len() - width;
            if let Some(bytes) = body.contiguous_bytes() {
                wire.extend_from_slice(&bytes[from..from + count]);
            } else {
                for offset in from..from + count {
                    wire.push(body.byte_at(offset).ok_or_else(|| refusal(invalid()))?);
                }
            }
            return Ok(progress(demand));
        }
        let mut state = channel.state.lock().map_err(|_| refusal(invalid()))?;
        if !state.accepting || state.uri != port.uri { return Err(refusal(invalid())); }
        let next_bytes = state.outbound_bytes.checked_add(extent).ok_or_else(|| refusal(invalid()))?;
        if state.outbound.len() >= super::super::BACKBONE_CHANNEL_MAXIMUM_MESSAGES || next_bytes > super::super::BACKBONE_CHANNEL_MAXIMUM_BYTES {
            return Ok(Default::default());
        }
        if state.outbound.len() == state.outbound.capacity() {
            let unit = size_of::<Vec<u8>>();
            let old = state.outbound.capacity() * unit;
            let demand = RetirementDemand {
                copy_bytes: state.outbound.len() * unit, capacity_bytes: old + unit, release_bytes: old,
                depth: 1,
            };
            if !retained_clone_grant_admits(effect.grant(), demand) || !effect.performed(1).map_err(refusal)? {
                return Ok(Default::default());
            }
            state.outbound.try_reserve_exact(1).map_err(|_| refusal(ValueError::literal(
                ValueRefusalKind::AllocationFailed, "actor outbound queue reservation failed",
            )))?;
            return Ok(RetainedCloneProgress { retained_capacity_bytes: state.outbound.capacity() * unit, ..progress(demand) });
        }
        let demand = RetirementDemand { copy_bytes: size_of::<Option<Vec<u8>>>() + size_of::<usize>() + size_of::<bool>(), depth: 1, ..Default::default() };
        if !retained_clone_grant_admits(effect.grant(), demand) || !effect.performed(1).map_err(refusal)? {
            return Ok(Default::default());
        }
        state.outbound.push_back(actor_wire.take().unwrap());
        state.outbound_bytes = next_bytes;
        *sent = true;
        Ok(progress(demand))
}

#[cfg(test)]
#[test]
fn original_ordinary_actor_wire_preserves_paged_body() {
    use crate::{ActorBackboneChannelOwner, PortBackbone, test_effect};
    let law: serde_json::Value = serde_json::from_str(include_str!("📤️original-outbox/transfer.json")).unwrap();
    let pattern = law["pattern"].as_array().unwrap();
    let bytes: Vec<_> = (0..law["bytes"].as_u64().unwrap() as usize)
        .map(|index| pattern[index % pattern.len()].as_u64().unwrap() as u8).collect();
    let body = CausalPayload::try_from_slice(&bytes).unwrap();
    let pointer = body.retained_bytes().backing_ptr(0).unwrap();
    let uri = "actor://ordinary";
    let (channel, host) = ActorBackboneChannelOwner::pair(uri);
    let endpoint = Backbones::Port(PortBackbone { uri: uri.into(), channel: Some(channel) });
    let grant = semio_framework_value::RetainedCloneGrant {
        maximum_items: 1, maximum_copy_bytes: 128, maximum_capacity_bytes: bytes.len() + 16,
        maximum_release_bytes: 0, maximum_depth: 1,
    };
    let mut cancelled = AtomicBackboneDelivery::begin_actor_message([1; 32]);
    test_effect::funded(grant, |effect| cancelled.advance_ordinary_wire(&body, &endpoint, effect).unwrap());
    assert!(!cancelled.terminal_is_empty());
    let close = semio_framework_value::RetainedCloneGrant {
        maximum_release_bytes: cancelled.close_demand().release_bytes, ..grant
    };
    test_effect::funded(close, |effect| cancelled.close(effect).unwrap());
    assert!(cancelled.terminal_is_empty());
    assert!(host.take_outbound().unwrap().is_none());
    let mut delivery = AtomicBackboneDelivery::begin_actor_message([1; 32]);
    for _ in 0..bytes.len() + 4 {
        if delivery.actor_message_sent() { break; }
        assert!(host.take_outbound().unwrap().is_none());
        let progress = test_effect::funded(grant, |effect| delivery.advance_ordinary_wire(&body, &endpoint, effect).unwrap());
        assert!(progress.fits(grant));
    }
    assert!(delivery.actor_message_sent());
    assert!(delivery.terminal_is_empty());
    let wire = host.take_outbound().unwrap().unwrap();
    let prefix = law["actorWirePrefix"].as_str().unwrap();
    assert_eq!(wire[..prefix.len() / 2].iter().map(|byte| format!("{byte:02x}")).collect::<String>(), prefix);
    let BackboneMessage::Mutations { envelopes } = decode_hot_backbone_message_exact(&wire).unwrap() else { panic!("the Port received its exact mutation wire") };
    assert_eq!(envelopes.to_vec_owner(), bytes);
    assert_eq!(body.retained_bytes().backing_ptr(0).unwrap(), pointer);
    host.begin_retire().unwrap();
}
