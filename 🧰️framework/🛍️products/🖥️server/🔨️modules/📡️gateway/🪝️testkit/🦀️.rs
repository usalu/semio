//! 🪝️ White-box hooks of the gateway for its companion crate: aliases of the sibling-private types and forwarders of the sibling-private functions the unit laws drive. Compiled only with the `testkit` feature.
use super::{CommandEnvelope, PolicyRequest};

pub type Member = super::presence::Member;
pub type Room = super::presence::Room;
pub type Outflow = super::presence::Outflow;

pub fn admission_request(envelope: &CommandEnvelope) -> PolicyRequest {
    super::server::admission_request(envelope)
}

pub fn wrap_relay(origin: &str, frame: &[u8]) -> Vec<u8> {
    super::streams::wrap_relay(origin, frame)
}

pub fn unwrap_relay(bytes: &[u8]) -> Option<(&str, &[u8])> {
    super::streams::unwrap_relay(bytes)
}
