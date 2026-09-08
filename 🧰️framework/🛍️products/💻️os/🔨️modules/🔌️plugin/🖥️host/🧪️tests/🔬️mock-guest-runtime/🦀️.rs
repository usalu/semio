use super::*;

async fn hash(byte: u8) -> PackageHash {
    PackageHash([byte; 32])
}

#[semio_framework_async_macros::async_test]
async fn scripted_turn_is_returned_exactly_once_fifo() {
    let runtime = MockGuestRuntime::new().await;
    let compiled = runtime.compile(&PackageRef { package: PackageId("stdio".to_string()), hash: hash(1).await }, &[]).await.expect("compile");
    let actor = RuntimeActorId(42);
    let mut inst = runtime.instantiate(&compiled, actor, &[], &Budget { fuel: 1000, deadline_ms: 4, max_effects: 8, max_patch_bytes: 4096, max_frames: 1 }).await.expect("instantiate");

    let mut first = MockGuestRuntime::idle_turn().await;
    first.fuel_used = 7;
    let mut second = MockGuestRuntime::idle_turn().await;
    second.fuel_used = 9;
    runtime.script_turn(actor, first).await;
    runtime.script_turn(actor, second).await;

    // 👶️ host-dedyn: `#[test] fn` is a sanctioned `block_on` entry point (R4 clause 5) — this
    // test drives `GuestRuntime`'s async methods directly against the concrete `MockGuestRuntime`
    // (not through the `GuestRuntimes` enum), same as it did through the deleted `poll_ready`.
    let got_first = semio_framework_async::block_on(runtime.execute_turn(&mut inst, &[], Budget { fuel: 1000, deadline_ms: 4, max_effects: 8, max_patch_bytes: 4096, max_frames: 1 })).expect("first scripted turn");
    assert_eq!(got_first.fuel_used, 7);
    let got_second = semio_framework_async::block_on(runtime.execute_turn(&mut inst, &[], Budget { fuel: 1000, deadline_ms: 4, max_effects: 8, max_patch_bytes: 4096, max_frames: 1 })).expect("second scripted turn");
    assert_eq!(got_second.fuel_used, 9);
}

#[semio_framework_async_macros::async_test]
async fn exhausted_script_queue_is_a_loud_error_not_a_fabricated_idle_turn() {
    let runtime = MockGuestRuntime::new().await;
    let compiled = runtime.compile(&PackageRef { package: PackageId("cad".to_string()), hash: hash(2).await }, &[]).await.expect("compile");
    let actor = RuntimeActorId(7);
    let mut inst = runtime.instantiate(&compiled, actor, &[], &Budget { fuel: 1, deadline_ms: 1, max_effects: 1, max_patch_bytes: 1, max_frames: 1 }).await.expect("instantiate");
    let error = semio_framework_async::block_on(runtime.execute_turn(&mut inst, &[], Budget { fuel: 1, deadline_ms: 1, max_effects: 1, max_patch_bytes: 1, max_frames: 1 })).expect_err("no script queued");
    assert!(matches!(error, TurnFault::Exhausted));
}

#[semio_framework_async_macros::async_test]
async fn scripted_fault_surfaces_as_trapped() {
    let runtime = MockGuestRuntime::new().await;
    let compiled = runtime.compile(&PackageRef { package: PackageId("block".to_string()), hash: hash(3).await }, &[]).await.expect("compile");
    let actor = RuntimeActorId(9);
    let mut inst = runtime.instantiate(&compiled, actor, &[], &Budget { fuel: 1, deadline_ms: 1, max_effects: 1, max_patch_bytes: 1, max_frames: 1 }).await.expect("instantiate");
    runtime.script_fault(actor, "epoch deadline exceeded").await;
    let error = semio_framework_async::block_on(runtime.execute_turn(&mut inst, &[], Budget { fuel: 1, deadline_ms: 1, max_effects: 1, max_patch_bytes: 1, max_frames: 1 })).expect_err("scripted fault");
    assert!(matches!(error, TurnFault::Trapped(message) if message == "epoch deadline exceeded"));
}

#[semio_framework_async_macros::async_test]
async fn checkpoint_then_restore_round_trips_through_a_fresh_instance() {
    let runtime = MockGuestRuntime::new().await;
    let compiled = runtime.compile(&PackageRef { package: PackageId("puzzle".to_string()), hash: hash(4).await }, &[]).await.expect("compile");
    let actor = RuntimeActorId(11);
    let mut inst = runtime.instantiate(&compiled, actor, &[], &Budget { fuel: 1, deadline_ms: 1, max_effects: 1, max_patch_bytes: 1, max_frames: 1 }).await.expect("instantiate");
    let snapshot = semio_framework_async::block_on(runtime.checkpoint(&mut inst)).expect("checkpoint");

    let mut restored = runtime.instantiate(&compiled, actor, &[], &Budget { fuel: 1, deadline_ms: 1, max_effects: 1, max_patch_bytes: 1, max_frames: 1 }).await.expect("re-instantiate");
    semio_framework_async::block_on(runtime.restore(&mut restored, &snapshot)).expect("restore");
    let GuestInstanceState::Mock(state) = &restored.state else { panic!("expected a Mock instance") };
    assert_eq!(state.checkpoint.as_deref(), Some(snapshot.as_slice()));
}

#[semio_framework_async_macros::async_test]
async fn controllable_clock_advances_deterministically() {
    let runtime = MockGuestRuntime::new().await;
    runtime.set_now_ms(1_000).await;
    assert_eq!(runtime.now_ms().await, 1_000);
    runtime.advance_ms(250).await;
    assert_eq!(runtime.now_ms().await, 1_250);
}

#[semio_framework_async_macros::async_test]
async fn drop_instance_forgets_the_actors_script_queue() {
    let runtime = MockGuestRuntime::new().await;
    let compiled = runtime.compile(&PackageRef { package: PackageId("layout".to_string()), hash: hash(5).await }, &[]).await.expect("compile");
    let actor = RuntimeActorId(13);
    let inst = runtime.instantiate(&compiled, actor, &[], &Budget { fuel: 1, deadline_ms: 1, max_effects: 1, max_patch_bytes: 1, max_frames: 1 }).await.expect("instantiate");
    runtime.script_turn(actor, MockGuestRuntime::idle_turn().await).await;
    runtime.drop_instance(inst).await;
    assert!(!runtime.scripts.lock().expect("lock").contains_key(&actor.0));
}
