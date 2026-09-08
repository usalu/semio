
use super::*;

fn fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../🧪️fixture/🔣️.json")).unwrap()
}

fn lifetime(value: &serde_json::Value) -> ActorInstanceLifetime {
    ActorInstanceLifetime {
        activation_generation: value["activationGeneration"].as_str().unwrap().parse().unwrap(),
        instance_id: value["instanceId"].as_u64().unwrap() as u32,
        guest_lifetime: value["guestLifetime"].as_str().unwrap().parse().unwrap(),
    }
}

fn receipt(value: &serde_json::Value) -> ActorInstanceLifecycleReceipt {
    let lifetime = lifetime(&value["lifetime"]);
    let request_sequence = value["requestSequence"].as_u64().unwrap();
    match value["kind"].as_str().unwrap() {
        "captured" => ActorInstanceLifecycleReceipt::Captured { lifetime, request_sequence },
        kind => {
            let close_generation = value["closeGeneration"].as_str().unwrap().parse().unwrap();
            if kind == "accepted" {
                ActorInstanceLifecycleReceipt::Accepted { lifetime, request_sequence, close_generation }
            } else {
                assert_eq!(kind, "retired");
                ActorInstanceLifecycleReceipt::Retired { lifetime, request_sequence, close_generation }
            }
        }
    }
}

#[test]
fn actor_instance_lifecycle_wire_matches_shared_independent_leb128_vectors() {
    for row in fixture()["vectors"].as_array().unwrap() {
        let value = &row["value"];
        let wire = match value["kind"].as_str().unwrap() {
            "open" => ActorInstanceLifecycleWire::Open(ActorInstanceOpenRequest {
                activation_generation: value["activationGeneration"].as_str().unwrap().parse().unwrap(),
                instance_id: value["instanceId"].as_u64().unwrap() as u32,
                request_sequence: value["requestSequence"].as_u64().unwrap(),
            }),
            "close" => ActorInstanceLifecycleWire::Close(ActorInstanceCloseRequest { lifetime: lifetime(&value["lifetime"]), request_sequence: value["requestSequence"].as_u64().unwrap() }),
            "ack" => ActorInstanceLifecycleWire::Ack(ActorInstanceLifecycleAck { receipt: receipt(&value["receipt"]) }),
            _ => ActorInstanceLifecycleWire::Receipt(receipt(value)),
        };
        let mut bytes = [0; ACTOR_INSTANCE_LIFECYCLE_MAXIMUM_BYTES];
        let length = wire.encode(&mut bytes).unwrap();
        let hex: String = bytes[..length].iter().map(|byte| format!("{byte:02x}")).collect();
        assert_eq!(hex, row["hex"].as_str().unwrap());
        assert_eq!(ActorInstanceLifecycleWire::decode(&bytes[..length]), Ok(wire));
        for prefix in 0..length {
            assert!(ActorInstanceLifecycleWire::decode(&bytes[..prefix]).is_err());
        }
        if length < bytes.len() {
            assert!(ActorInstanceLifecycleWire::decode(&bytes[..=length]).is_err());
        }
        if let ActorInstanceLifecycleWire::Receipt(receipt) = wire {
            assert_eq!(serde_json::to_value(receipt).unwrap(), *value);
            assert_eq!(serde_json::from_value::<ActorInstanceLifecycleReceipt>(value.clone()).unwrap(), receipt);
        }
        if let ActorInstanceLifecycleWire::Ack(ack) = wire {
            assert_eq!(serde_json::to_value(ack.receipt).unwrap(), value["receipt"]);
        }
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
    for bytes in [vec![8, 1, 7, 9], vec![0, 0, 7, 9], vec![0, 129, 0, 7, 9], vec![0, 1, 7, 0], vec![2, 1, 7, 0, 9], vec![4, 1, 7, 13, 9, 0], vec![0, 255, 255, 255, 255, 255, 255, 255, 255, 255, 2, 7, 9]] {
        assert!(ActorInstanceLifecycleWire::decode(&bytes).is_err());
    }
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
    crate::TurnResult { ui_patches: vec![], effects: vec![], command_ingress: vec![], cold_pair_ingress: Default::default(), lifecycle_receipt, ui_patch_receipt: None, next_wake: None, status: crate::TurnStatus::Idle, usage: crate::Usage::default() }
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
        let mut bytes = vec![0, 0, 0, 0, (hex.len() / 2) as u8];
        for offset in (0..hex.len()).step_by(2) {
            bytes.push(u8::from_str_radix(&hex[offset..offset + 2], 16).unwrap());
        }
        bytes.extend_from_slice(&[0; 5]);
        assert!(crate::TurnResult::pack_decode(&bytes, &mut 0).await.is_err(), "only receipt wire tags may enter TurnResult");
    }
    let mut oversized = vec![0, 0, 0, 0, 45];
    oversized.extend_from_slice(&[0; 50]);
    assert!(crate::TurnResult::pack_decode(&oversized, &mut 0).await.is_err());
}
