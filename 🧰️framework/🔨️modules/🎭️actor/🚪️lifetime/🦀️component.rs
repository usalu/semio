//#region 🚪️InstanceCloseWire
pub const ACTOR_INSTANCE_CLOSE_MAXIMUM_BYTES: usize = 34;
const REQUEST_SEQUENCE_MAXIMUM: u64 = 9_007_199_254_740_991;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ActorInstanceLifetime {
    pub activation_generation: u64,
    pub instance_id: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ActorInstanceCloseRequest {
    pub lifetime: ActorInstanceLifetime,
    pub request_sequence: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ActorInstanceCloseReceiptKind { Accepted, Retired }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ActorInstanceCloseReceipt {
    pub kind: ActorInstanceCloseReceiptKind,
    pub lifetime: ActorInstanceLifetime,
    pub request_sequence: u64,
    pub close_generation: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ActorInstanceCloseWire {
    Close(ActorInstanceCloseRequest),
    Receipt(ActorInstanceCloseReceipt),
}

impl ActorInstanceCloseWire {
    /// 📤️ Encodes the schema's fixed authority without allocation; invalid input leaves output untouched.
    pub fn encode(&self, output: &mut [u8; ACTOR_INSTANCE_CLOSE_MAXIMUM_BYTES]) -> Result<usize, &'static str> {
        let (tag, lifetime, request, generation) = match *self {
            Self::Close(request) => (0, request.lifetime, request.request_sequence, None),
            Self::Receipt(receipt) => (if receipt.kind == ActorInstanceCloseReceiptKind::Accepted { 1 } else { 2 }, receipt.lifetime, receipt.request_sequence, Some(receipt.close_generation)),
        };
        if lifetime.activation_generation == 0 || request == 0 || request > REQUEST_SEQUENCE_MAXIMUM || generation == Some(0) { return Err("actor-close.invalid-authority"); }
        output[0] = tag;
        let mut offset = 1;
        for mut value in [Some(lifetime.activation_generation), Some(u64::from(lifetime.instance_id)), Some(request), generation].into_iter().flatten() {
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

    /// 📥️ Accepts canonical unsigned LEB128 only, including the JavaScript-safe request domain.
    pub fn decode(bytes: &[u8]) -> Result<Self, &'static str> {
        if bytes.is_empty() || bytes.len() > ACTOR_INSTANCE_CLOSE_MAXIMUM_BYTES { return Err("actor-close.envelope"); }
        let tag = bytes[0];
        if tag > 2 { return Err("actor-close.tag"); }
        let mut offset = 1;
        let activation_generation = read_unsigned(bytes, &mut offset, u64::MAX, true)?;
        let instance_id = read_unsigned(bytes, &mut offset, u64::from(u32::MAX), false)? as u32;
        let request_sequence = read_unsigned(bytes, &mut offset, REQUEST_SEQUENCE_MAXIMUM, true)?;
        let lifetime = ActorInstanceLifetime { activation_generation, instance_id };
        let value = if tag == 0 {
            Self::Close(ActorInstanceCloseRequest { lifetime, request_sequence })
        } else {
            Self::Receipt(ActorInstanceCloseReceipt { kind: if tag == 1 { ActorInstanceCloseReceiptKind::Accepted } else { ActorInstanceCloseReceiptKind::Retired }, lifetime, request_sequence, close_generation: read_unsigned(bytes, &mut offset, u64::MAX, true)? })
        };
        if offset != bytes.len() { return Err("actor-close.trailing"); }
        Ok(value)
    }
}

fn read_unsigned(bytes: &[u8], offset: &mut usize, maximum: u64, nonzero: bool) -> Result<u64, &'static str> {
    let mut value = 0u64;
    for index in 0..10 {
        let byte = *bytes.get(*offset).ok_or("actor-close.truncated")?;
        *offset += 1;
        if index == 9 && byte & 126 != 0 { return Err("actor-close.overflow"); }
        value |= u64::from(byte & 127) << (index * 7);
        if byte & 128 == 0 {
            if (index != 0 && byte == 0) || value > maximum || (nonzero && value == 0) { return Err("actor-close.noncanonical-authority"); }
            return Ok(value);
        }
    }
    Err("actor-close.overlong")
}

/// 📨️ Checks exact receipt identity; this function does not manufacture descendant terminal proof.
pub fn actor_instance_close_receipt_matches(request: ActorInstanceCloseRequest, accepted: Option<ActorInstanceCloseReceipt>, receipt: ActorInstanceCloseReceipt) -> bool {
    request.lifetime == receipt.lifetime && request.request_sequence == receipt.request_sequence && accepted.map_or(receipt.kind == ActorInstanceCloseReceiptKind::Accepted, |accepted| {
        accepted.kind == ActorInstanceCloseReceiptKind::Accepted && accepted.lifetime == receipt.lifetime && accepted.request_sequence == receipt.request_sequence && accepted.close_generation == receipt.close_generation
    })
}

//#region 🧪️SharedWireLaws
#[cfg(test)]
mod tests {
    use super::*;

    fn fixture() -> serde_json::Value { serde_json::from_str(include_str!("🧪️fixture.json")).unwrap() }

    fn lifetime(value: &serde_json::Value) -> ActorInstanceLifetime {
        ActorInstanceLifetime { activation_generation: value["activationGeneration"].as_str().unwrap().parse().unwrap(), instance_id: value["instanceId"].as_u64().unwrap() as u32 }
    }

    #[test]
    fn actor_instance_close_wire_matches_shared_independent_leb128_vectors() {
        for row in fixture()["vectors"].as_array().unwrap() {
            let value = &row["value"];
            let lifetime = lifetime(&value["lifetime"]);
            let request_sequence = value["requestSequence"].as_u64().unwrap();
            let wire = if value["kind"] == "close" { ActorInstanceCloseWire::Close(ActorInstanceCloseRequest { lifetime, request_sequence }) } else {
                ActorInstanceCloseWire::Receipt(ActorInstanceCloseReceipt { kind: if value["kind"] == "accepted" { ActorInstanceCloseReceiptKind::Accepted } else { ActorInstanceCloseReceiptKind::Retired }, lifetime, request_sequence, close_generation: value["closeGeneration"].as_str().unwrap().parse().unwrap() })
            };
            let mut bytes = [0; ACTOR_INSTANCE_CLOSE_MAXIMUM_BYTES];
            let length = wire.encode(&mut bytes).unwrap();
            let hex: String = bytes[..length].iter().map(|byte| format!("{byte:02x}")).collect();
            assert_eq!(hex, row["hex"].as_str().unwrap());
            assert_eq!(ActorInstanceCloseWire::decode(&bytes[..length]), Ok(wire));
            for prefix in 0..length { assert!(ActorInstanceCloseWire::decode(&bytes[..prefix]).is_err()); }
            if length < bytes.len() { assert!(ActorInstanceCloseWire::decode(&bytes[..=length]).is_err()); }
        }
    }

    #[test]
    fn actor_instance_close_wire_rejects_invalid_authority_before_writing() {
        let mut output = [37; ACTOR_INSTANCE_CLOSE_MAXIMUM_BYTES];
        for request_sequence in [0, REQUEST_SEQUENCE_MAXIMUM + 1, u64::MAX] {
            let value = ActorInstanceCloseWire::Close(ActorInstanceCloseRequest { lifetime: ActorInstanceLifetime { activation_generation: 1, instance_id: 7 }, request_sequence });
            assert!(value.encode(&mut output).is_err());
            assert_eq!(output, [37; ACTOR_INSTANCE_CLOSE_MAXIMUM_BYTES]);
        }
        for bytes in [vec![3, 1, 7, 9], vec![0, 0, 7, 9], vec![0, 129, 0, 7, 9], vec![0, 1, 7, 0], vec![0, 255, 255, 255, 255, 255, 255, 255, 255, 255, 2, 7, 9]] { assert!(ActorInstanceCloseWire::decode(&bytes).is_err()); }
    }

    #[test]
    fn actor_instance_close_wire_requires_exact_accepted_identity_before_terminal() {
        let fixture = fixture();
        let request = ActorInstanceCloseRequest { lifetime: lifetime(&fixture["reopen"]["current"]), request_sequence: 9 };
        let accepted = ActorInstanceCloseReceipt { kind: ActorInstanceCloseReceiptKind::Accepted, lifetime: request.lifetime, request_sequence: 9, close_generation: 13 };
        let retired = ActorInstanceCloseReceipt { kind: ActorInstanceCloseReceiptKind::Retired, ..accepted };
        assert!(!actor_instance_close_receipt_matches(request, None, retired));
        assert!(actor_instance_close_receipt_matches(request, None, accepted));
        assert!(actor_instance_close_receipt_matches(request, Some(accepted), retired));
        assert!(!actor_instance_close_receipt_matches(request, Some(accepted), ActorInstanceCloseReceipt { lifetime: lifetime(&fixture["reopen"]["prior"]), ..retired }));
        assert!(!actor_instance_close_receipt_matches(request, Some(accepted), ActorInstanceCloseReceipt { close_generation: 12, ..retired }));
    }
}
//#endregion 🧪️SharedWireLaws
//#endregion 🚪️InstanceCloseWire
