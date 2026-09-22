use super::*;

#[semio_framework_async_macros::async_test]
async fn compile_accepts_a_deterministic_component_and_caches_it() {
    let runtime = WasmtimeRuntime::new(SharedEngineConfig::default()).await.expect("engine builds");
    let package = PackageRef { package: PackageId("stdio".to_string()), hash: PackageHash([9u8; 32]) };
    let compiled = runtime.compile(&package, minimal_component_without_actor_world()).await.expect("a deterministic component compiles even though it does not export the `actor` world");
    assert!(compiled.component.is_some());
}

#[semio_framework_async_macros::async_test]
async fn instantiate_rejects_a_component_that_does_not_export_the_actor_world() {
    let runtime = WasmtimeRuntime::new(SharedEngineConfig::default()).await.expect("engine builds");
    let package = PackageRef { package: PackageId("stdio".to_string()), hash: PackageHash([10u8; 32]) };
    let compiled = runtime.compile(&package, minimal_component_without_actor_world()).await.expect("compiles as a component");
    let budget = Budget { fuel: 1_000_000, deadline_ms: 4, max_effects: 8, max_patch_bytes: 4096, max_frames: 1 };
    let error = runtime.instantiate(&compiled, RuntimeActorId(1), &[], &budget).await.expect_err("minimal component does not export `reactor`/`jobs`/`checkpoint`/`describe`");
    let _ = error;
}

#[semio_framework_async_macros::async_test]
async fn wit_turn_status_conversion_is_a_plain_rename() {
    assert_eq!(wit_turn_status_to_kernel(wit_reactor::TurnStatus::Idle).await, TurnStatus::Idle);
    assert_eq!(wit_turn_status_to_kernel(wit_reactor::TurnStatus::MoreWork).await, TurnStatus::MoreWork);
    assert!(matches!(wit_turn_status_to_kernel(wit_reactor::TurnStatus::Faulted(vec![1, 2, 3])).await, TurnStatus::Faulted(bytes) if bytes == vec![1, 2, 3]));
}

#[semio_framework_async_macros::async_test]
async fn message_endpoint_round_trips_through_wit_and_back() {
    let original = MessageEndpoint::Topic { name: "os.runtime.metrics".to_string() };
    let wit = kernel_message_endpoint_to_wit(&original);
    let back = wit_message_endpoint_to_kernel(wit.await);
    assert_eq!(original, back.await);
}

#[semio_framework_async_macros::async_test]
async fn io_run_effect_is_a_reported_error_not_a_silent_mismap() {
    let effect = wit_effects::Effect::IoRun(wit_effects::IoRunEffect { req: 1, params: wit_effects::IoRunParams { source: "a".to_string(), target: "b".to_string(), payload: vec![] } });
    let result = wit_effect_to_kernel(effect).await;
    assert!(result.is_err(), "io-run must surface as an error until Effect::IoRun exists");
}

// 🩺️ GJ1 — the job lane's trap classifier and the host watchdog it made readable.
// `📓️gj1-guest-inference-job-trap.md` §2/§3: `step-job` reported every trap as an unattributed
// backtrace (`Display` is a wasmtime error's FIRST line only), and its store ceilings came from the
// GUEST's cooperative grant, which is 2 epoch ticks — so a `s.wfc.bitmap.solve` crossing was killed
// after ~2 ms at whatever instruction it had reached.

#[test]
fn a_fuel_cut_and_an_epoch_cut_are_named_rather_than_reported_as_a_backtrace() {
    let fuel = wasmtime::Error::msg("error while executing at wasm backtrace:\n    0: 0x3bf0a - guest!export").context("all fuel consumed by WebAssembly");
    let epoch = wasmtime::Error::msg("error while executing at wasm backtrace:\n    0: 0x3bf0a - guest!export").context("epoch deadline reached");
    assert!(matches!(classify_guest_trap(&fuel), TurnFault::FuelExhausted), "a fuel cut must be FuelExhausted, not Trapped(<backtrace>)");
    assert!(matches!(classify_guest_trap(&epoch), TurnFault::DeadlineExceeded), "an epoch cut must be DeadlineExceeded, not Trapped(<backtrace>)");
}

#[test]
fn a_real_trap_keeps_its_whole_source_chain_not_its_first_line() {
    let trap = wasmtime::Error::msg("wasm trap: unreachable executed").context("error while executing at wasm backtrace:\n    0: 0x30da925 - guest!deallocate");
    let TurnFault::Trapped(message) = classify_guest_trap(&trap) else { panic!("an unreachable is a trap, not a budget cut") };
    assert!(message.contains("unreachable executed"), "the trap code lives in the source chain, which `Display` drops: {message}");
    assert!(message.contains("wasm backtrace"), "and the backtrace is still carried: {message}");
}

#[test]
fn the_host_watchdog_is_not_the_guests_cooperative_grant() {
    // ⏱️ The regression this ratchets: `step_job` used to arm the store from `RELAY_JOB_BUDGET`,
    // whose `deadline_ms` is `USER_VISIBLE_LANE_WALL_US / 1_000`. With `EPOCH_TICK_INTERVAL_MS = 1`
    // that is a two-millisecond hard kill per crossing.
    assert_eq!(RELAY_JOB_BUDGET.deadline_ms as u64, semio_framework_job::USER_VISIBLE_LANE_WALL_US / 1_000, "the guest's grant stays the cooperative slice it always was");
    assert!(
        GUEST_JOB_WATCHDOG_MS >= RELAY_JOB_BUDGET.deadline_ms as u64 * 1_000,
        "the host's wedged watchdog ({GUEST_JOB_WATCHDOG_MS} epoch ticks at {}ms each) must be orders of magnitude above the guest's own step slice ({} ticks), never equal to it",
        EPOCH_TICK_INTERVAL_MS,
        RELAY_JOB_BUDGET.deadline_ms,
    );
}
