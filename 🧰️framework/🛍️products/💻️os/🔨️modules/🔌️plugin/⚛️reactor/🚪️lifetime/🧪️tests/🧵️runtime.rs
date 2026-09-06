fn reactor_native_lifecycle_budget() -> semio_framework::kernel::Budget {
    semio_framework::kernel::Budget { fuel: 64, deadline_ms: 1000, max_effects: 16, max_patch_bytes: 65536, max_frames: 16 }
}

fn reactor_native_lifecycle_open(instance: u32, request_sequence: u64, actor: String) -> semio_framework::kernel::Event {
    semio_framework::kernel::Event::InstanceOpen {
        request: semio_framework::kernel::ActorInstanceOpenRequest { activation_generation: 41, instance_id: instance, request_sequence },
        app_id: semio_framework::kernel::AppInstanceId(TestApp::<false>::APP_ID.into()),
        actor, config: Vec::new(), assets: Vec::new(), capabilities: Vec::new(), quotas: Default::default(),
    }
}

async fn reactor_native_lifecycle_poll(runtime: &crate::plugin_runtime::PluginRuntime<TestRuntimeApps>, events: Vec<semio_framework::kernel::Event>) -> semio_framework::kernel::TurnResult {
    crate::reactor::poll_kernel(runtime, events, None, reactor_native_lifecycle_budget()).await.expect("actual native lifecycle turn")
}

async fn reactor_native_lifecycle_ack(runtime: &crate::plugin_runtime::PluginRuntime<TestRuntimeApps>, receipt: semio_framework::kernel::ActorInstanceLifecycleReceipt) {
    reactor_native_lifecycle_poll(runtime, vec![semio_framework::kernel::Event::InstanceLifecycleAck(semio_framework::kernel::ActorInstanceLifecycleAck { receipt })]).await;
}

async fn reactor_native_lifecycle_finish(runtime: &crate::plugin_runtime::PluginRuntime<TestRuntimeApps>, lifetime: semio_framework::kernel::ActorInstanceLifetime, request_sequence: u64) {
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
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixture/🧵️production.json")).unwrap();
    let instance = fixture["open"]["instance_id"].as_u64().unwrap() as u32;
    let runtime = crate::plugin_runtime::PluginRuntime::<TestRuntimeApps>::new();
    crate::plugin_runtime::install_plugin_bundle(&runtime, __semio_plugin_bundle().await.unwrap());
    let captured = reactor_native_lifecycle_poll(&runtime, vec![reactor_native_lifecycle_open(instance, 8, "native-fixture".into())]).await.lifecycle_receipt.expect("Captured receipt");
    let Receipt::Captured { lifetime, request_sequence } = captured else { panic!("open must emit Captured") };
    assert_eq!(request_sequence, 8);
    assert!(!runtime.guest_lifetimes.borrow().get(instance).unwrap().cell.is_live());
    assert_eq!(crate::plugin_runtime::instance_actor(&runtime, instance).await, "native-fixture");
    let early = Event::InstanceClose(ActorInstanceCloseRequest { lifetime, request_sequence: 9 });
    assert!(crate::reactor::poll_kernel(&runtime, vec![early], None, reactor_native_lifecycle_budget()).await.is_err());
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
    assert!(crate::reactor::poll_kernel(&runtime, vec![reactor_native_lifecycle_open(7, 8, "a".repeat(4096))], None, reactor_native_lifecycle_budget()).await.is_err());
    assert!(runtime.guest_lifetimes.borrow().get(7).is_none());
    assert!(crate::plugin_runtime::plugin_capture_instance_close(&runtime, 7).is_err());
    let captured = reactor_native_lifecycle_poll(&runtime, vec![reactor_native_lifecycle_open(7, 8, "native-fixture".into())]).await.lifecycle_receipt.unwrap();
    let Receipt::Captured { lifetime, .. } = captured else { panic!("real captured owner") };
    assert!(crate::reactor::poll_kernel(&runtime, vec![reactor_native_lifecycle_open(1031, 8, "collision".into())], None, reactor_native_lifecycle_budget()).await.is_err());
    assert_eq!(runtime.guest_lifetimes.borrow().get(7).unwrap().cell.lifetime(), lifetime);
    reactor_native_lifecycle_ack(&runtime, captured).await;
    for foreign in [
        semio_framework::kernel::ActorInstanceLifetime { activation_generation: 42, ..lifetime },
        semio_framework::kernel::ActorInstanceLifetime { guest_lifetime: lifetime.guest_lifetime + 1, ..lifetime },
    ] {
        assert!(crate::reactor::poll_kernel(&runtime, vec![Event::InstanceClose(ActorInstanceCloseRequest { lifetime: foreign, request_sequence: 9 })], None, reactor_native_lifecycle_budget()).await.is_err());
        assert!(crate::plugin_runtime::plugin_capture_instance_close(&runtime, 7).is_ok());
        assert!(runtime.guest_lifetimes.borrow().get(7).unwrap().cell.is_live());
    }
    let wrong = Event::InstanceLifecycleAck(ActorInstanceLifecycleAck { receipt: Receipt::Retired { lifetime, request_sequence: 9, close_generation: 1 } });
    assert!(crate::reactor::poll_kernel(&runtime, vec![wrong], None, reactor_native_lifecycle_budget()).await.is_err());
    reactor_native_lifecycle_finish(&runtime, lifetime, 9).await;
    let reopened = reactor_native_lifecycle_poll(&runtime, vec![reactor_native_lifecycle_open(7, 10, "native-fixture".into())]).await.lifecycle_receipt.unwrap();
    let Receipt::Captured { lifetime: replacement, .. } = reopened else { panic!("new captured owner") };
    assert!(replacement.guest_lifetime > lifetime.guest_lifetime);
    assert!(crate::reactor::poll_kernel(&runtime, vec![Event::InstanceLifecycleAck(ActorInstanceLifecycleAck { receipt: captured })], None, reactor_native_lifecycle_budget()).await.is_err());
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
