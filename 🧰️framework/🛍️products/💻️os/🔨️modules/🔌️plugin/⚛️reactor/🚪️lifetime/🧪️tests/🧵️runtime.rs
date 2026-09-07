fn reactor_native_lifecycle_budget() -> Budget {
    Budget { fuel: 64, deadline_ms: 1000, max_effects: 16, max_patch_bytes: 65536, max_frames: 16 }
}

fn reactor_native_lifecycle_open(instance: u32, request_sequence: u64, actor: String) -> Event {
    Event::InstanceOpen {
        request: ActorInstanceOpenRequest { activation_generation: 41, instance_id: instance, request_sequence },
        app_id: AppInstanceId(TestApp::<false>::APP_ID.into()),
        actor, config: Vec::new(), assets: Vec::new(), capabilities: Vec::new(), quotas: Default::default(),
    }
}

async fn reactor_native_lifecycle_poll(runtime: &crate::plugin_runtime::PluginRuntime<TestRuntimeApps>, events: Vec<Event>) -> TurnResult {
    for attempt in 0..64 {
        match crate::reactor::poll_kernel(runtime, events.clone(), None, None, reactor_native_lifecycle_budget()).await {
            Ok(result) => return result,
            Err(fault) if fault.code.0 == "plugin.reactor-turn-deadline" && fault.retryable => {
                eprintln!("[DEBUG] retained lifecycle exact-event deadline retry={} events={:?}", attempt + 1, events);
                std::thread::yield_now();
            }
            Err(fault) => panic!("actual native lifecycle turn: {fault:?}"),
        }
    }
    panic!("retained lifecycle could not finish within 64 exact-event retries")
}

async fn reactor_native_lifecycle_ack(runtime: &crate::plugin_runtime::PluginRuntime<TestRuntimeApps>, receipt: ActorInstanceLifecycleReceipt) {
    reactor_native_lifecycle_poll(runtime, vec![Event::InstanceLifecycleAck(ActorInstanceLifecycleAck { receipt })]).await;
}

async fn reactor_native_lifecycle_finish(runtime: &crate::plugin_runtime::PluginRuntime<TestRuntimeApps>, lifetime: ActorInstanceLifetime, request_sequence: u64) {
    use semio_framework::kernel::{ActorInstanceCloseRequest, ActorInstanceLifecycleReceipt as Receipt, Event};
    let request = ActorInstanceCloseRequest { lifetime, request_sequence };
    let first = reactor_native_lifecycle_poll(runtime, vec![Event::InstanceClose(request)]).await.lifecycle_receipt.expect("accepted native close");
    let Receipt::Accepted { close_generation, .. } = first else { panic!("close must publish Accepted before Retired") };
    assert!(close_generation > 0);
    assert!(crate::plugin_runtime::plugin_capture_instance_close(runtime, lifetime.instance_id).is_err());
    let duplicate = reactor_native_lifecycle_poll(runtime, vec![Event::InstanceClose(request)]).await.lifecycle_receipt;
    assert_eq!(duplicate, Some(first), "duplicate exact close retains the same native generation");
    reactor_native_lifecycle_ack(runtime, first).await;
    let mut retired = None;
    for _ in 0..16384 {
        let turn = reactor_native_lifecycle_poll(runtime, Vec::new()).await;
        if let Some(receipt @ Receipt::Retired { .. }) = turn.lifecycle_receipt {
            let lifetimes = runtime.guest_lifetimes.borrow();
            let owner = lifetimes.get(lifetime.instance_id).unwrap().cell.owner().unwrap();
            assert!(crate::reactor::instance_lifetime::GuestLifetimeOwner::terminal_is_empty(owner).unwrap(), "Retired must prove actual native emptiness");
            retired = Some(receipt);
            break;
        }
        std::thread::yield_now();
    }
    let retired = retired.expect("bounded real reactor and native close retirement");
    let Receipt::Retired { close_generation: retired_generation, .. } = retired else { unreachable!() };
    assert_eq!(retired_generation, close_generation);
    assert!(runtime.guest_lifetimes.borrow().get(lifetime.instance_id).is_some(), "terminal receipt keeps its native owner");
    for _ in 0..8 {
        reactor_native_lifecycle_ack(runtime, retired).await;
        if runtime.guest_lifetimes.borrow().get(lifetime.instance_id).is_none() { break; }
    }
    assert!(runtime.guest_lifetimes.borrow().get(lifetime.instance_id).is_none(), "exact final ACK releases the structural owner");
    eprintln!("[DEBUG] native reactor lifetime={} close={} retired and acknowledged", lifetime.guest_lifetime, close_generation);
}

#[semio_framework_async_macros::async_test]
async fn reactor_native_lifecycle_retains_exact_close_until_ack() {
    use semio_framework::kernel::{ActorInstanceCloseRequest, ActorInstanceLifecycleReceipt as Receipt, Event};
    let fixture: Value = serde_json::from_str(include_str!("../🧫️fixture/🧵️production.json")).unwrap();
    let instance = fixture["open"]["instance_id"].as_u64().unwrap() as u32;
    let runtime = crate::plugin_runtime::PluginRuntime::<TestRuntimeApps>::new();
    crate::plugin_runtime::install_plugin_bundle(&runtime, __semio_plugin_bundle().await.unwrap());
    let captured = reactor_native_lifecycle_poll(&runtime, vec![reactor_native_lifecycle_open(instance, 8, "native-fixture".into())]).await.lifecycle_receipt.expect("Captured receipt");
    let Receipt::Captured { lifetime, request_sequence } = captured else { panic!("open must emit Captured") };
    assert_eq!(request_sequence, 8);
    assert!(!runtime.guest_lifetimes.borrow().get(instance).unwrap().cell.is_live());
    assert_eq!(crate::plugin_runtime::instance_actor(&runtime, instance).await, "native-fixture");
    let early = Event::InstanceClose(ActorInstanceCloseRequest { lifetime, request_sequence: 9 });
    assert!(crate::reactor::poll_kernel(&runtime, vec![early], None, None, reactor_native_lifecycle_budget()).await.is_err());
    assert!(crate::plugin_runtime::plugin_capture_instance_close(&runtime, instance).is_ok());
    reactor_native_lifecycle_ack(&runtime, captured).await;
    assert!(runtime.guest_lifetimes.borrow().get(instance).unwrap().cell.is_live());
    reactor_native_lifecycle_finish(&runtime, lifetime, 9).await;
}

#[semio_framework_async_macros::async_test]
async fn reactor_native_lifecycle_rejects_foreign_and_colliding_owners() {
    use semio_framework::kernel::{ActorInstanceCloseRequest, ActorInstanceLifecycleAck, ActorInstanceLifecycleReceipt as Receipt, Event};
    let runtime = crate::plugin_runtime::PluginRuntime::<TestRuntimeApps>::new();
    crate::plugin_runtime::install_plugin_bundle(&runtime, __semio_plugin_bundle().await.unwrap());
    assert!(crate::reactor::poll_kernel(&runtime, vec![reactor_native_lifecycle_open(7, 8, "a".repeat(super::PLUGIN_RUNTIME_ACTOR_BYTES + 1))], None, None, reactor_native_lifecycle_budget()).await.is_err());
    assert!(runtime.guest_lifetimes.borrow().get(7).is_none());
    assert!(crate::plugin_runtime::plugin_capture_instance_close(&runtime, 7).is_err());
    let captured = reactor_native_lifecycle_poll(&runtime, vec![reactor_native_lifecycle_open(7, 8, "native-fixture".into())]).await.lifecycle_receipt.unwrap();
    let Receipt::Captured { lifetime, .. } = captured else { panic!("real captured owner") };
    assert!(crate::reactor::poll_kernel(&runtime, vec![reactor_native_lifecycle_open(1031, 8, "collision".into())], None, None, reactor_native_lifecycle_budget()).await.is_err());
    assert_eq!(runtime.guest_lifetimes.borrow().get(7).unwrap().cell.lifetime(), lifetime);
    reactor_native_lifecycle_ack(&runtime, captured).await;
    for foreign in [
        ActorInstanceLifetime { activation_generation: 42, ..lifetime },
        ActorInstanceLifetime { guest_lifetime: lifetime.guest_lifetime + 1, ..lifetime },
    ] {
        assert!(crate::reactor::poll_kernel(&runtime, vec![Event::InstanceClose(ActorInstanceCloseRequest { lifetime: foreign, request_sequence: 9 })], None, None, reactor_native_lifecycle_budget()).await.is_err());
        assert!(crate::plugin_runtime::plugin_capture_instance_close(&runtime, 7).is_ok());
        assert!(runtime.guest_lifetimes.borrow().get(7).unwrap().cell.is_live());
    }
    let wrong = Event::InstanceLifecycleAck(ActorInstanceLifecycleAck { receipt: Receipt::Retired { lifetime, request_sequence: 9, close_generation: 1 } });
    assert!(crate::reactor::poll_kernel(&runtime, vec![wrong], None, None, reactor_native_lifecycle_budget()).await.is_err());
    reactor_native_lifecycle_finish(&runtime, lifetime, 9).await;
    let reopened = reactor_native_lifecycle_poll(&runtime, vec![reactor_native_lifecycle_open(7, 10, "native-fixture".into())]).await.lifecycle_receipt.unwrap();
    let Receipt::Captured { lifetime: replacement, .. } = reopened else { panic!("new captured owner") };
    assert!(replacement.guest_lifetime > lifetime.guest_lifetime);
    assert!(crate::reactor::poll_kernel(&runtime, vec![Event::InstanceLifecycleAck(ActorInstanceLifecycleAck { receipt: captured })], None, None, reactor_native_lifecycle_budget()).await.is_err());
    reactor_native_lifecycle_ack(&runtime, reopened).await;
    reactor_native_lifecycle_finish(&runtime, replacement, 11).await;
}

#[semio_framework_async_macros::async_test]
async fn reactor_native_lifecycle_output_failure_preserves_ack_and_owner() {
    use semio_framework::kernel::{ActorInstanceLifecycleAck, ActorInstanceLifecycleReceipt as Receipt, Event};
    let runtime = crate::plugin_runtime::PluginRuntime::<TestRuntimeApps>::new();
    crate::plugin_runtime::install_plugin_bundle(&runtime, __semio_plugin_bundle().await.unwrap());
    let captured = reactor_native_lifecycle_poll(&runtime, vec![reactor_native_lifecycle_open(7, 8, "native-fixture".into())]).await.lifecycle_receipt.unwrap();
    let Receipt::Captured { lifetime, .. } = captured else { panic!("real captured owner") };
    let ack = Event::InstanceLifecycleAck(ActorInstanceLifecycleAck { receipt: captured });
    let failed = crate::reactor::test_support::poll_with_output_failure(&runtime, vec![ack], reactor_native_lifecycle_budget()).await;
    assert!(failed.is_err());
    {
        let lifetimes = runtime.guest_lifetimes.borrow();
        let slot = lifetimes.get(7).unwrap();
        assert_eq!(slot.cell.retained_receipt(), Some(captured));
        assert!(!slot.cell.is_live());
        assert!(slot.cell.owner().is_some());
    }
    reactor_native_lifecycle_ack(&runtime, captured).await;
    reactor_native_lifecycle_finish(&runtime, lifetime, 9).await;
}

#[semio_framework_async_macros::async_test]
async fn reactor_output_fault_returns_real_patch_and_preserves_other_lifecycle_ack() {
    use semio_framework::kernel::{ActorInstanceLifecycleAck, ActorInstanceLifecycleReceipt as Receipt, Event};
    for late_clock in [false, true] {
        let runtime = crate::plugin_runtime::PluginRuntime::<TestRuntimeApps>::new();
        crate::plugin_runtime::install_plugin_bundle(&runtime, __semio_plugin_bundle().await.unwrap());
        let captured_a = reactor_native_lifecycle_poll(&runtime, vec![reactor_native_lifecycle_open(7, 8, "native-fixture".into())]).await.lifecycle_receipt.unwrap();
        let Receipt::Captured { lifetime: a, .. } = captured_a else { unreachable!() };
        reactor_native_lifecycle_ack(&runtime, captured_a).await;
        let captured_b = reactor_native_lifecycle_poll(&runtime, vec![reactor_native_lifecycle_open(8, 8, "native-fixture".into())]).await.lifecycle_receipt.unwrap();
        let Receipt::Captured { lifetime: b, .. } = captured_b else { unreachable!() };
        crate::reactor::test_support::queue_external_patch(UiPatch {
            surface: semio_framework_ui_contract::SurfaceId::try_from("7:window").unwrap(),
            base_revision: semio_framework_ui_contract::UiRevision(0), revision: semio_framework_ui_contract::UiRevision(2), ops: Default::default(),
        });
        let preparation = reactor_native_lifecycle_poll(&runtime, Vec::new()).await;
        assert!(preparation.ui_patches.is_empty());
        let event = Event::InstanceLifecycleAck(ActorInstanceLifecycleAck { receipt: captured_b });
        let (failed, provisional) = crate::reactor::test_support::poll_with_patch_output_fault(&runtime, vec![event.clone()], reactor_native_lifecycle_budget(), late_clock).await;
        let failure = failed.expect_err("injected real output failure");
        if late_clock { assert_eq!(failure.code.0, "plugin.reactor-turn-deadline"); assert!(failure.retryable); }
        let provisional = provisional.expect("the real pending patch was staged");
        assert!(!crate::reactor::test_support::patch_receipt_is_issued(provisional));
        assert_eq!(runtime.guest_lifetimes.borrow().get(8).unwrap().cell.retained_receipt(), Some(captured_b));
        assert!(!runtime.guest_lifetimes.borrow().get(8).unwrap().cell.is_live());
        let emitted = reactor_native_lifecycle_poll(&runtime, vec![event]).await;
        let issued = emitted.ui_patch_receipt.unwrap();
        assert_eq!(issued.lifetime, a);
        assert!(issued.patch_sequence > provisional.patch_sequence);
        let patch = emitted.ui_patches.iter().next().unwrap();
        assert_eq!(patch.surface.0.as_str(), "7:window");
        assert_eq!(patch.revision.0, 2);
        assert!(runtime.guest_lifetimes.borrow().get(8).unwrap().cell.is_live());
        let foreign = ActorUiPatchReceipt { lifetime: ActorInstanceLifetime { guest_lifetime: a.guest_lifetime + 1, ..a }, ..issued };
        for receipt in [provisional, foreign] {
            reactor_native_lifecycle_poll(&runtime, vec![Event::PatchAck { receipt, surface: "7:window".into(), revision: 2 }]).await;
            assert!(crate::reactor::test_support::patch_receipt_is_issued(issued));
        }
        reactor_native_lifecycle_poll(&runtime, vec![Event::PatchAck { receipt: issued, surface: "7:window".into(), revision: 2 }]).await;
        assert!(!crate::reactor::test_support::patch_receipt_is_issued(issued));
        drop(emitted);
        reactor_native_lifecycle_finish(&runtime, b, 9).await;
        reactor_native_lifecycle_finish(&runtime, a, 9).await;
        eprintln!("[DEBUG] real patch handback late_clock={} provisional={} issued={} foreign feedback inert", late_clock, provisional.patch_sequence, issued.patch_sequence);
    }
}
