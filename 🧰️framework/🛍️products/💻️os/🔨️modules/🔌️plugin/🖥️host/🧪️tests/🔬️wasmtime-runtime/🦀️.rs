
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
