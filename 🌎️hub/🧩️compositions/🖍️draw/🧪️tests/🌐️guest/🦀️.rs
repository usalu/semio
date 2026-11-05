//! 🖍️ Actual Draw component opening under the independent compiled host runtime.
use semio_plugin_host::{GuestRuntime,GuestInstance,WasmtimeRuntime,SharedEngineConfig,PackageRef,PackageId,PackageHash,retryable_lifecycle_turn};
use semio_framework::kernel::{Budget,Event,TurnResult,TurnStatus};
use semio_framework_actor::ActorId as RuntimeActorId;
use std::path::PathBuf;
#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct OpenScenario {
    app_id: String,
    actor: String,
    expected_status: String,
}

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct OpenScenarios {
    version: u32,
    scenarios: Vec<OpenScenario>,
}

fn open_scenarios() -> Vec<OpenScenario> {
    let fixtures: OpenScenarios = serde_json::from_str(include_str!("scenarios.json")).expect("valid language-neutral opening scenarios");
    assert_eq!(fixtures.version, 1);
    assert_eq!(fixtures.scenarios.len(), 2);
    let mut roles: Vec<_> = fixtures.scenarios.iter().map(|scenario| scenario.actor.as_str()).collect();
    roles.sort_unstable();
    assert_eq!(roles, ["editor", "viewer"]);
    let descriptor_path = std::env::var_os("SEMIO_DRAW_GUEST_DESCRIPTOR").expect("declared guest check supplies its retained model descriptor");
    let descriptor: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(descriptor_path).expect("read actual component descriptor")).expect("valid actual component descriptor");
    for scenario in &fixtures.scenarios {
        assert_eq!(scenario.expected_status, "idle");
        assert!(descriptor["manifest"]["apps"].as_array().expect("descriptor apps").iter().any(|app| app["id"] == scenario.app_id && app["role"] == scenario.actor), "opening scenario must match a shipped surface");
    }
    fixtures.scenarios
}
const OPEN_WALL_BUDGET:std::time::Duration=std::time::Duration::from_secs(120);
fn plugin_wasm()->PathBuf{
    let path = PathBuf::from(std::env::var_os("SEMIO_DRAW_GUEST_COMPONENT").expect("declared guest check supplies its retained model component"));
    assert!(path.is_file(),"Draw component producer prerequisite is required");path
}
fn package_ref(package_id: &str, bytes: &[u8]) -> PackageRef {
    PackageRef { package: PackageId(package_id.to_string()), hash: PackageHash(*semio_framework_hash::hash(bytes).as_bytes()) }
}

fn jit_budget() -> Budget {
    Budget::original(
        semio_framework_value::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 },
        u64::MAX,
        semio_framework_job::default_now_us().expect("native monotonic clock is available"),
        120_000,
        64,
        1 << 20,
        64,
    )
}

fn instance_open_event(app_id: &str, actor: &str, config: Vec<u8>) -> Event {
    Event::InstanceOpen {
        request: semio_framework::kernel::ActorInstanceOpenRequest { activation_generation: 1, instance_id: 1, request_sequence: 1 },
        app_id: semio_framework::kernel::AppInstanceId(app_id.to_string()),
        actor: actor.to_string(),
        config,
        assets: Vec::new(),
        capabilities: Vec::new(),
        quotas: semio_framework::kernel::QuotaSchema::default(),
    }
}

async fn open_to_settle(runtime: &impl GuestRuntime, instance: &mut GuestInstance, scenario: &OpenScenario) -> TurnResult {
    let mut owed = vec![instance_open_event(&scenario.app_id, &scenario.actor, Vec::new())];
    let started = std::time::Instant::now();
    let mut opened = false;
    let mut identity = semio_framework_plugin::artifact_app_laws::fixture_identity();
    loop {
        let events = std::mem::take(&mut owed);
        match runtime.execute_turn(instance, &events, jit_budget(), &mut identity).await {
            Ok(turn) => {
                assert!(!matches!(turn.status, TurnStatus::Faulted(_)), "{} opening faulted: {:?}", scenario.app_id, turn.status);
                if let Some(receipt) = turn.lifecycle_receipt {
                    opened = true;
                    owed.push(Event::InstanceLifecycleAck(semio_framework::kernel::ActorInstanceLifecycleAck { receipt }));
                }
                if matches!(turn.status, TurnStatus::Idle) && owed.is_empty() {
                    assert!(opened, "{} became idle without acknowledging InstanceOpen", scenario.app_id);
                    return turn;
                }
            }
            Err(fault) if retryable_lifecycle_turn(&fault, &events) => {
                owed.splice(0..0, events);
            }
            Err(fault) => panic!("InstanceOpen faulted after {:?}: {fault:?}", started.elapsed()),
        }
        assert!(started.elapsed() < OPEN_WALL_BUDGET, "InstanceOpen never settled within {OPEN_WALL_BUDGET:?}");
    }
}

mod quick {
use super::*;
#[semio_framework_async_macros::async_test]
async fn wasmtime_runtime_instance_open_settles_against_the_draw_component() {
    let path=plugin_wasm();
    let bytes = std::fs::read(&path).expect("read plugin component");
    let runtime = WasmtimeRuntime::new(SharedEngineConfig::default(),semio_framework_job::default_now_us).await.expect("engine builds");
    let compiled = runtime.compile(&package_ref("semio:draw", &bytes), &bytes).await.expect("compile plugin component");
    for (index, scenario) in open_scenarios().iter().enumerate() {
        let mut instance = runtime.instantiate(&compiled, RuntimeActorId(index as u64 + 1), &[], &jit_budget()).await.expect("instantiate plugin actor");
        let turn = open_to_settle(&runtime, &mut instance, scenario).await;
        assert!(matches!(turn.status, TurnStatus::Idle), "{} settled as {:?}", scenario.app_id, turn.status);
        runtime.drop_instance(instance).await;
    }
}

}
