//#region 🚪️InstanceLifecycleWire
use serde::{Deserialize, Serialize};

#[path = "🩹️patch/🦀️.rs"]
mod patch_receipt;
pub use patch_receipt::{ActorUiPatchReceipt, ACTOR_UI_PATCH_RECEIPT_MAXIMUM_BYTES};

pub const ACTOR_INSTANCE_LIFECYCLE_MAXIMUM_BYTES: usize = 44;
pub(crate) const REQUEST_SEQUENCE_MAXIMUM: u64 = 9_007_199_254_740_991;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ActorInstanceLifetime {
    #[serde(with = "decimal_generation")]
    pub activation_generation: u64,
    pub instance_id: u32,
    #[serde(with = "decimal_generation")]
    pub guest_lifetime: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ActorInstanceOpenRequest {
    #[serde(with = "decimal_generation")]
    pub activation_generation: u64,
    pub instance_id: u32,
    #[serde(with = "request_sequence")]
    pub request_sequence: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ActorInstanceCloseRequest {
    pub lifetime: ActorInstanceLifetime,
    #[serde(with = "request_sequence")]
    pub request_sequence: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase", deny_unknown_fields)]
pub enum ActorInstanceLifecycleReceipt {
    Captured {
        lifetime: ActorInstanceLifetime,
        #[serde(with = "request_sequence")]
        request_sequence: u64,
    },
    Accepted {
        lifetime: ActorInstanceLifetime,
        #[serde(with = "request_sequence")]
        request_sequence: u64,
        #[serde(with = "decimal_generation")]
        close_generation: u64,
    },
    Retired {
        lifetime: ActorInstanceLifetime,
        #[serde(with = "request_sequence")]
        request_sequence: u64,
        #[serde(with = "decimal_generation")]
        close_generation: u64,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ActorInstanceLifecycleAck { pub receipt: ActorInstanceLifecycleReceipt }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ActorInstanceLifecycleWire {
    Open(ActorInstanceOpenRequest),
    Close(ActorInstanceCloseRequest),
    Receipt(ActorInstanceLifecycleReceipt),
    Ack(ActorInstanceLifecycleAck),
}

impl ActorInstanceLifetime {
    pub fn is_valid(self) -> bool { self.activation_generation != 0 && self.guest_lifetime != 0 }
}

impl ActorInstanceOpenRequest {
    pub fn is_valid(self) -> bool { self.activation_generation != 0 && valid_request(self.request_sequence) }
}

impl ActorInstanceCloseRequest {
    pub fn is_valid(self) -> bool { self.lifetime.is_valid() && valid_request(self.request_sequence) }
}

impl ActorInstanceLifecycleReceipt {
    pub fn lifetime(self) -> ActorInstanceLifetime {
        match self { Self::Captured { lifetime, .. } | Self::Accepted { lifetime, .. } | Self::Retired { lifetime, .. } => lifetime }
    }

    pub fn request_sequence(self) -> u64 {
        match self { Self::Captured { request_sequence, .. } | Self::Accepted { request_sequence, .. } | Self::Retired { request_sequence, .. } => request_sequence }
    }

    pub fn close_generation(self) -> Option<u64> {
        match self { Self::Captured { .. } => None, Self::Accepted { close_generation, .. } | Self::Retired { close_generation, .. } => Some(close_generation) }
    }

    pub fn is_valid(self) -> bool { self.lifetime().is_valid() && valid_request(self.request_sequence()) && self.close_generation() != Some(0) }

    fn tag(self) -> u8 {
        match self { Self::Captured { .. } => 1, Self::Accepted { .. } => 3, Self::Retired { .. } => 4 }
    }
}

impl ActorInstanceLifecycleWire {
    /// 📤️ Encodes the complete fixed authority; invalid input leaves the caller's output untouched.
    pub fn encode(&self, output: &mut [u8; ACTOR_INSTANCE_LIFECYCLE_MAXIMUM_BYTES]) -> Result<usize, &'static str> {
        let (tag, activation, instance, guest, request, close) = match *self {
            Self::Open(open) => {
                if !open.is_valid() { return Err("actor-lifecycle.invalid-authority"); }
                (0, open.activation_generation, open.instance_id, None, open.request_sequence, None)
            }
            Self::Close(close) => {
                if !close.is_valid() { return Err("actor-lifecycle.invalid-authority"); }
                (2, close.lifetime.activation_generation, close.lifetime.instance_id, Some(close.lifetime.guest_lifetime), close.request_sequence, None)
            }
            Self::Receipt(receipt) | Self::Ack(ActorInstanceLifecycleAck { receipt }) => {
                if !receipt.is_valid() { return Err("actor-lifecycle.invalid-authority"); }
                let tag = if matches!(self, Self::Ack(_)) { if receipt.tag() == 1 { 5 } else { receipt.tag() + 3 } } else { receipt.tag() };
                let lifetime = receipt.lifetime();
                (tag, lifetime.activation_generation, lifetime.instance_id, Some(lifetime.guest_lifetime), receipt.request_sequence(), receipt.close_generation())
            }
        };
        output[0] = tag;
        let mut offset = 1;
        for mut value in [Some(activation), Some(u64::from(instance)), guest, Some(request), close].into_iter().flatten() {
            loop {
                let byte = (value & 127) as u8;
                value >>= 7;
                output[offset] = byte | if value == 0 { 0 } else { 128 };
                offset += 1;
                if value == 0 { break; }
            }
        }
        Ok(offset)
    }

    /// 📥️ Decodes canonical unsigned LEB128 without allocating, coercing, or accepting old authority shapes.
    pub fn decode(bytes: &[u8]) -> Result<Self, &'static str> {
        if bytes.is_empty() || bytes.len() > ACTOR_INSTANCE_LIFECYCLE_MAXIMUM_BYTES { return Err("actor-lifecycle.envelope"); }
        let tag = bytes[0];
        if tag > 7 { return Err("actor-lifecycle.tag"); }
        let mut offset = 1;
        let activation_generation = read_unsigned(bytes, &mut offset, u64::MAX, true)?;
        let instance_id = read_unsigned(bytes, &mut offset, u64::from(u32::MAX), false)? as u32;
        let guest_lifetime = if tag == 0 { None } else { Some(read_unsigned(bytes, &mut offset, u64::MAX, true)?) };
        let request_sequence = read_unsigned(bytes, &mut offset, REQUEST_SEQUENCE_MAXIMUM, true)?;
        let value = if tag == 0 {
            Self::Open(ActorInstanceOpenRequest { activation_generation, instance_id, request_sequence })
        } else {
            let lifetime = ActorInstanceLifetime { activation_generation, instance_id, guest_lifetime: guest_lifetime.expect("non-open tag owns its parsed guest lifetime") };
            if tag == 2 { Self::Close(ActorInstanceCloseRequest { lifetime, request_sequence }) }
            else {
                let receipt = if tag == 1 || tag == 5 { ActorInstanceLifecycleReceipt::Captured { lifetime, request_sequence } }
                else {
                    let close_generation = read_unsigned(bytes, &mut offset, u64::MAX, true)?;
                    if tag == 3 || tag == 6 { ActorInstanceLifecycleReceipt::Accepted { lifetime, request_sequence, close_generation } }
                    else { ActorInstanceLifecycleReceipt::Retired { lifetime, request_sequence, close_generation } }
                };
                if tag >= 5 { Self::Ack(ActorInstanceLifecycleAck { receipt }) } else { Self::Receipt(receipt) }
            }
        };
        if offset != bytes.len() { return Err("actor-lifecycle.trailing"); }
        Ok(value)
    }
}

pub(crate) fn valid_request(value: u64) -> bool { value != 0 && value <= REQUEST_SEQUENCE_MAXIMUM }

pub(crate) fn read_unsigned(bytes: &[u8], offset: &mut usize, maximum: u64, nonzero: bool) -> Result<u64, &'static str> {
    let mut value = 0u64;
    for index in 0..10 {
        let byte = *bytes.get(*offset).ok_or("actor-lifecycle.truncated")?;
        *offset += 1;
        if index == 9 && byte & 126 != 0 { return Err("actor-lifecycle.overflow"); }
        value |= u64::from(byte & 127) << (index * 7);
        if byte & 128 == 0 {
            if (index != 0 && byte == 0) || value > maximum || (nonzero && value == 0) { return Err("actor-lifecycle.noncanonical-authority"); }
            return Ok(value);
        }
    }
    Err("actor-lifecycle.overlong")
}

/// 🪪️ Matches the first captured response to its exact open request; the caller must retain that guest lifetime.
pub fn actor_instance_captured_receipt_matches(open: ActorInstanceOpenRequest, receipt: ActorInstanceLifecycleReceipt) -> bool {
    open.is_valid() && receipt.is_valid() && matches!(receipt, ActorInstanceLifecycleReceipt::Captured { lifetime, request_sequence } if lifetime.activation_generation == open.activation_generation && lifetime.instance_id == open.instance_id && request_sequence == open.request_sequence)
}

/// 📨️ Checks receipt identity and acceptance ordering without manufacturing descendant terminal proof.
pub fn actor_instance_close_receipt_matches(request: ActorInstanceCloseRequest, accepted: Option<ActorInstanceLifecycleReceipt>, receipt: ActorInstanceLifecycleReceipt) -> bool {
    if !request.is_valid() || !receipt.is_valid() || request.lifetime != receipt.lifetime() || request.request_sequence != receipt.request_sequence() || matches!(receipt, ActorInstanceLifecycleReceipt::Captured { .. }) { return false; }
    match accepted {
        None => matches!(receipt, ActorInstanceLifecycleReceipt::Accepted { .. }),
        Some(accepted) => matches!(accepted, ActorInstanceLifecycleReceipt::Accepted { .. }) && accepted.is_valid() && accepted.lifetime() == receipt.lifetime() && accepted.request_sequence() == receipt.request_sequence() && accepted.close_generation() == receipt.close_generation(),
    }
}

pub(crate) mod decimal_generation {
    use serde::{Deserializer, Serializer};

    pub fn serialize<S: Serializer>(value: &u64, serializer: S) -> Result<S::Ok, S::Error> {
        if *value == 0 { return Err(serde::ser::Error::custom("zero lifecycle generation")); }
        serializer.collect_str(value)
    }

    struct DecimalVisitor;

    impl<'de> serde::de::Visitor<'de> for DecimalVisitor {
        type Value = u64;
        fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { formatter.write_str("a canonical nonzero unsigned 64-bit decimal string") }
        fn visit_str<E: serde::de::Error>(self, value: &str) -> Result<u64, E> {
            if value.is_empty() || value.len() > 20 || value.as_bytes()[0] == b'0' || !value.bytes().all(|byte| byte.is_ascii_digit()) { return Err(E::custom("noncanonical lifecycle generation")); }
            value.parse().map_err(E::custom)
        }
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<u64, D::Error> { deserializer.deserialize_str(DecimalVisitor) }
}

pub(crate) mod request_sequence {
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S: Serializer>(value: &u64, serializer: S) -> Result<S::Ok, S::Error> {
        if !super::valid_request(*value) { return Err(serde::ser::Error::custom("invalid lifecycle request")); }
        serializer.serialize_u64(*value)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<u64, D::Error> {
        let value = u64::deserialize(deserializer)?;
        if !super::valid_request(value) { return Err(serde::de::Error::custom("invalid lifecycle request")); }
        Ok(value)
    }
}

//#region 🧪️SharedWireLaws
#[cfg(test)]
#[path = "🩹️patch/🧪️tests/🦀️.rs"]
mod patch_receipt_tests;

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture() -> serde_json::Value { serde_json::from_str(include_str!("🧪️fixture/🔣️.json")).unwrap() }

    fn lifetime(value: &serde_json::Value) -> ActorInstanceLifetime {
        ActorInstanceLifetime { activation_generation: value["activationGeneration"].as_str().unwrap().parse().unwrap(), instance_id: value["instanceId"].as_u64().unwrap() as u32, guest_lifetime: value["guestLifetime"].as_str().unwrap().parse().unwrap() }
    }

    fn receipt(value: &serde_json::Value) -> ActorInstanceLifecycleReceipt {
        let lifetime = lifetime(&value["lifetime"]);
        let request_sequence = value["requestSequence"].as_u64().unwrap();
        match value["kind"].as_str().unwrap() {
            "captured" => ActorInstanceLifecycleReceipt::Captured { lifetime, request_sequence },
            kind => {
                let close_generation = value["closeGeneration"].as_str().unwrap().parse().unwrap();
                if kind == "accepted" { ActorInstanceLifecycleReceipt::Accepted { lifetime, request_sequence, close_generation } }
                else { assert_eq!(kind, "retired"); ActorInstanceLifecycleReceipt::Retired { lifetime, request_sequence, close_generation } }
            }
        }
    }

    #[test]
    fn actor_instance_lifecycle_wire_matches_shared_independent_leb128_vectors() {
        for row in fixture()["vectors"].as_array().unwrap() {
            let value = &row["value"];
            let wire = match value["kind"].as_str().unwrap() {
                "open" => ActorInstanceLifecycleWire::Open(ActorInstanceOpenRequest { activation_generation: value["activationGeneration"].as_str().unwrap().parse().unwrap(), instance_id: value["instanceId"].as_u64().unwrap() as u32, request_sequence: value["requestSequence"].as_u64().unwrap() }),
                "close" => ActorInstanceLifecycleWire::Close(ActorInstanceCloseRequest { lifetime: lifetime(&value["lifetime"]), request_sequence: value["requestSequence"].as_u64().unwrap() }),
                "ack" => ActorInstanceLifecycleWire::Ack(ActorInstanceLifecycleAck { receipt: receipt(&value["receipt"]) }),
                _ => ActorInstanceLifecycleWire::Receipt(receipt(value)),
            };
            let mut bytes = [0; ACTOR_INSTANCE_LIFECYCLE_MAXIMUM_BYTES];
            let length = wire.encode(&mut bytes).unwrap();
            let hex: String = bytes[..length].iter().map(|byte| format!("{byte:02x}")).collect();
            assert_eq!(hex, row["hex"].as_str().unwrap());
            assert_eq!(ActorInstanceLifecycleWire::decode(&bytes[..length]), Ok(wire));
            for prefix in 0..length { assert!(ActorInstanceLifecycleWire::decode(&bytes[..prefix]).is_err()); }
            if length < bytes.len() { assert!(ActorInstanceLifecycleWire::decode(&bytes[..=length]).is_err()); }
            if let ActorInstanceLifecycleWire::Receipt(receipt) = wire {
                assert_eq!(serde_json::to_value(receipt).unwrap(), *value);
                assert_eq!(serde_json::from_value::<ActorInstanceLifecycleReceipt>(value.clone()).unwrap(), receipt);
            }
            if let ActorInstanceLifecycleWire::Ack(ack) = wire { assert_eq!(serde_json::to_value(ack.receipt).unwrap(), value["receipt"]); }
        }
    }

    #[test]
    fn actor_instance_lifecycle_wire_rejects_invalid_authority_before_writing() {
        let mut output = [37; ACTOR_INSTANCE_LIFECYCLE_MAXIMUM_BYTES];
        for request_sequence in [0, REQUEST_SEQUENCE_MAXIMUM + 1, u64::MAX] {
            let value = ActorInstanceLifecycleWire::Close(ActorInstanceCloseRequest { lifetime: ActorInstanceLifetime { activation_generation: 1, instance_id: 7, guest_lifetime: 13 }, request_sequence });
            assert!(value.encode(&mut output).is_err());
            assert_eq!(output, [37; ACTOR_INSTANCE_LIFECYCLE_MAXIMUM_BYTES]);
        }
        for bytes in [vec![8, 1, 7, 9], vec![0, 0, 7, 9], vec![0, 129, 0, 7, 9], vec![0, 1, 7, 0], vec![2, 1, 7, 0, 9], vec![4, 1, 7, 13, 9, 0], vec![0, 255, 255, 255, 255, 255, 255, 255, 255, 255, 2, 7, 9]] { assert!(ActorInstanceLifecycleWire::decode(&bytes).is_err()); }
    }

    #[test]
    fn actor_instance_lifecycle_wire_requires_exact_accepted_identity_before_terminal() {
        let fixture = fixture();
        let request = ActorInstanceCloseRequest { lifetime: lifetime(&fixture["reopen"]["current"]), request_sequence: 9 };
        let accepted = ActorInstanceLifecycleReceipt::Accepted { lifetime: request.lifetime, request_sequence: 9, close_generation: 13 };
        let retired = ActorInstanceLifecycleReceipt::Retired { lifetime: request.lifetime, request_sequence: 9, close_generation: 13 };
        assert!(!actor_instance_close_receipt_matches(request, None, retired));
        assert!(actor_instance_close_receipt_matches(request, None, accepted));
        assert!(actor_instance_close_receipt_matches(request, Some(accepted), retired));
        assert!(!actor_instance_close_receipt_matches(request, Some(accepted), ActorInstanceLifecycleReceipt::Retired { lifetime: lifetime(&fixture["reopen"]["prior"]), request_sequence: 9, close_generation: 13 }));
        assert!(!actor_instance_close_receipt_matches(request, Some(accepted), ActorInstanceLifecycleReceipt::Retired { lifetime: request.lifetime, request_sequence: 9, close_generation: 12 }));
        let open = ActorInstanceOpenRequest { activation_generation: 41, instance_id: 7, request_sequence: 8 };
        let captured = ActorInstanceLifecycleReceipt::Captured { lifetime: request.lifetime, request_sequence: 8 };
        assert!(actor_instance_captured_receipt_matches(open, captured));
        assert!(!actor_instance_captured_receipt_matches(open, accepted));
        assert!(!actor_instance_captured_receipt_matches(ActorInstanceOpenRequest { request_sequence: 7, ..open }, captured));
        assert!(!actor_instance_close_receipt_matches(request, None, captured));
    }

    fn turn(lifecycle_receipt: Option<ActorInstanceLifecycleReceipt>) -> crate::TurnResult {
        crate::TurnResult { ui_patches: vec![], effects: vec![], command_ingress: vec![], lifecycle_receipt, ui_patch_receipt: None, next_wake: None, status: crate::TurnStatus::Idle, usage: crate::Usage::default() }
    }

    #[semio_framework_async_macros::async_test]
    async fn actor_instance_lifecycle_turn_round_trips_shared_outer_vectors() {
        let fixture = fixture();
        for row in fixture["turnResults"].as_array().unwrap() {
            let expected = turn(row["vector"].as_u64().map(|index| receipt(&fixture["vectors"][index as usize]["value"])));
            let mut bytes = Vec::new();
            expected.pack_encode(&mut bytes).await.expect("valid exact receipt");
            let hex: String = bytes.iter().map(|byte| format!("{byte:02x}")).collect();
            assert_eq!(hex, row["hex"].as_str().unwrap());
            let mut offset = 0;
            assert_eq!(crate::TurnResult::pack_decode(&bytes, &mut offset).await.unwrap(), expected);
            assert_eq!(offset, bytes.len());
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn actor_instance_lifecycle_turn_rejects_invalid_receipt_without_partial_output() {
        let invalid = ActorInstanceLifecycleReceipt::Captured { lifetime: ActorInstanceLifetime { activation_generation: 1, instance_id: 7, guest_lifetime: 0 }, request_sequence: 8 };
        let mut bytes = vec![91, 92];
        assert!(turn(Some(invalid)).pack_encode(&mut bytes).await.is_err());
        assert_eq!(bytes, [91, 92]);
        let fixture = fixture();
        for index in [0usize, 2, 5] {
            let hex = fixture["vectors"][index]["hex"].as_str().unwrap();
            let mut bytes = vec![0, 0, 0, (hex.len() / 2) as u8];
            for offset in (0..hex.len()).step_by(2) { bytes.push(u8::from_str_radix(&hex[offset..offset + 2], 16).unwrap()); }
            bytes.extend_from_slice(&[0; 5]);
            assert!(crate::TurnResult::pack_decode(&bytes, &mut 0).await.is_err(), "only receipt wire tags may enter TurnResult");
        }
        let mut oversized = vec![0, 0, 0, 45];
        oversized.extend_from_slice(&[0; 50]);
        assert!(crate::TurnResult::pack_decode(&oversized, &mut 0).await.is_err());
    }
}
//#endregion 🧪️SharedWireLaws
//#endregion 🚪️InstanceLifecycleWire
