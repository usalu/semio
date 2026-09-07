use super::*;

fn event(kind: &str) -> Event {
    let lifetime = semio_framework::kernel::ActorInstanceLifetime { activation_generation: 41, instance_id: 7, guest_lifetime: 3 };
    match kind {
        "open" => Event::InstanceOpen {
            request: semio_framework::kernel::ActorInstanceOpenRequest { activation_generation: 41, instance_id: 7, request_sequence: 8 },
            app_id: semio_framework::kernel::AppInstanceId("s.test.synthetic@1/*#editor".into()), actor: "fixture".into(),
            config: Vec::new(), assets: Vec::new(), capabilities: Vec::new(), quotas: Default::default(),
        },
        "close" => Event::InstanceClose(semio_framework::kernel::ActorInstanceCloseRequest { lifetime, request_sequence: 9 }),
        "ack" => Event::InstanceLifecycleAck(semio_framework::kernel::ActorInstanceLifecycleAck { receipt: semio_framework::kernel::ActorInstanceLifecycleReceipt::Captured { lifetime, request_sequence: 8 } }),
        "page" => Event::CommandIngressPage { cursor: semio_framework::kernel::CommandPageCursor { owner: 1, generation: 1, command_index: 0, command_count: 1, instance: 7, seq: 1, kind: 28, page_index: 0, page_count: 1, item_count: 0, metadata: 0 }, bytes: semio_framework::kernel::FixedCommandPage::try_copy_from(&[]).unwrap() },
        "wake" => Event::Wake,
        _ => panic!("fixture event kind"),
    }
}

#[test]
fn structured_guest_fault_survives_wit_owned_and_async_channels() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixture/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        let fault = semio_framework::Fault::new(semio_framework::FaultOrigin::Framework, semio_framework::FaultCode::new(row["code"].as_str().unwrap()), "retained lifecycle fixture").with_retryable(row["retryable"].as_bool().unwrap());
        let encoded = dsl::encode_fault_bytes(&fault);
        let events: Vec<_> = row["events"].as_array().unwrap().iter().map(|kind| event(kind.as_str().unwrap())).collect();
        let expected = row["eligible"].as_bool().unwrap();
        let wit = decode_guest_plugin_error(wit_types::PluginError::Fault(encoded.clone()));
        let owned = decode_owned_result::<()>(serde_json::to_vec(&Result::<(), Vec<u8>>::Err(encoded)).unwrap().as_slice()).unwrap_err();
        for transported in [wit, owned] {
            let TurnFault::Guest(ref decoded) = transported else { panic!("guest fault was flattened") };
            assert_eq!(decoded.code, fault.code);
            assert_eq!(decoded.retryable, fault.retryable);
            assert_eq!(retryable_lifecycle_turn(&transported, &events), expected, "{}", row["id"]);
            let (sender, mut receiver) = tokio::sync::oneshot::channel::<Result<TurnResult, TurnFault>>();
            sender.send(Err(transported)).expect("typed poll oneshot");
            let received = receiver.try_recv().unwrap().unwrap_err();
            assert_eq!(retryable_lifecycle_turn(&received, &events), expected);
        }
    }
    eprintln!("[DEBUG] structured guest fault transports=3 neutral-cases=10");
}

#[test]
fn malformed_and_oversized_guest_faults_cannot_authorize_retry() {
    for bytes in [b"plugin.reactor-turn-deadline".to_vec(), vec![b'x'; GUEST_FAULT_MAXIMUM_BYTES + 1]] {
        let fault = decode_guest_plugin_error(wit_types::PluginError::Fault(bytes));
        assert!(!retryable_lifecycle_turn(&fault, &[]));
    }
}
