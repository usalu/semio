//! 🖍️ Actual Draw component opening under the independent compiled host runtime.
use semio_framework_plugin_host::{GuestRuntime,GuestInstance,WasmtimeRuntime,SharedEngineConfig,PackageRef,PackageId,PackageHash,TurnFault,retryable_lifecycle_turn};
use semio_framework::kernel::{Budget,Event,TurnResult,TurnStatus};
use semio_framework_actor::ActorId as RuntimeActorId;
use std::path::PathBuf;
const DRAW_EDITOR_APP:&str="s.draw.drawing@1/*#editor";
const OPEN_WALL_BUDGET:std::time::Duration=std::time::Duration::from_secs(120);
fn plugin_wasm(_file:&str)->PathBuf{
    let root=PathBuf::from(env!("CARGO_MANIFEST_DIR")).ancestors().find(|path|path.join("nx.json").is_file()).unwrap().to_path_buf();
    let path=root.join("🌎️hub/🧩️compositions/🖍️draw/📦️packages/🦀️rust/dist/component-dev/semio_hub_draw.wasm");
    assert!(path.is_file(),"Draw component producer prerequisite is required");path
}
fn package_ref(package_id: &str, bytes: &[u8]) -> PackageRef {
    PackageRef { package: PackageId(package_id.to_string()), hash: PackageHash(*semio_framework_hash::hash(bytes).as_bytes()) }
}

fn open_budget() -> Budget {
    Budget { fuel: u64::MAX, deadline_ms: 8, max_effects: 64, max_patch_bytes: 1 << 20, max_frames: 64 }
}

fn jit_budget() -> Budget {
    Budget { deadline_ms: 120_000, ..open_budget() }
}

fn instance_open_event(app_id: &str, config: Vec<u8>) -> Event {
    Event::InstanceOpen {
        request: semio_framework::kernel::ActorInstanceOpenRequest { activation_generation: 1, instance_id: 1, request_sequence: 1 },
        app_id: semio_framework::kernel::AppInstanceId(app_id.to_string()),
        actor: "editor".to_string(),
        config,
        assets: Vec::new(),
        capabilities: Vec::new(),
        quotas: semio_framework::kernel::QuotaSchema::default(),
    }
}

async fn open_to_settle(runtime: &impl GuestRuntime, instance: &mut GuestInstance, app_id: &str, budget: impl Fn() -> Budget, mid_flight: impl Fn(&GuestInstance) -> bool) -> TurnResult {
    let mut owed = vec![instance_open_event(app_id, Vec::new())];
    let started = std::time::Instant::now();
    loop {
        let events = if mid_flight(instance) { Vec::new() } else { std::mem::take(&mut owed) };
        match runtime.execute_turn(instance, &events, budget()).await {
            Ok(turn) => {
                if let Some(receipt) = turn.lifecycle_receipt {
                    owed.push(Event::InstanceLifecycleAck(semio_framework::kernel::ActorInstanceLifecycleAck { receipt }));
                }
                if matches!(turn.status, TurnStatus::Idle) && owed.is_empty() {
                    return turn;
                }
            }
            Err(TurnFault::DeadlineExceeded | TurnFault::FuelExhausted) => {}
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
    let path=plugin_wasm("semio_hub_draw.wasm");
    let bytes = std::fs::read(&path).expect("read plugin component");
    let runtime = WasmtimeRuntime::new(SharedEngineConfig::default()).await.expect("engine builds");
    let compiled = runtime.compile(&package_ref("semio:draw", &bytes), &bytes).await.expect("compile plugin component");
    let mut instance = runtime.instantiate(&compiled, RuntimeActorId(1), &[], &jit_budget()).await.expect("instantiate plugin actor");
    let turn = open_to_settle(&runtime, &mut instance, DRAW_EDITOR_APP, jit_budget, |_| false).await;
    assert!(matches!(turn.status, TurnStatus::Idle), "InstanceOpen settled as {:?}", turn.status);
}

}
