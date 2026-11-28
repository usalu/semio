//! 🥶️ A fresh MCP process accepts the retained prepared image and opens its genuine Source-bound actor.

use super::*;
use semio_framework_actor::{ActorId, OriginalGuestExecutionBudget, RetainedTurnInput};
use semio_framework_actor::retained_context::OriginalActorContextSlot;
use semio_framework_actor::retained_turn::OriginalActorTurnWork;
use semio_framework_job::{OriginalTurnFailure, StepBudget, StepContext, default_now_us};
use semio_framework_value::{RetainedCloneGrant, RetainedCloneProgress};
use serde_json::json;

const RUNNER: &str = "SEMIO_COLD_PREPARED_MODEL_TEST";
const PAGE: usize = 65_536;

fn hex(value: &Value) -> Vec<u8> {
    let text = value.as_str().unwrap();
    (0..text.len()).step_by(2).map(|offset| u8::from_str_radix(&text[offset..offset + 2], 16).unwrap()).collect()
}

fn cold_fixture(row: &Value) -> (Vec<u8>, Vec<u8>) {
    let mut component = prepared_component();
    for patch in row["patches"].as_array().unwrap() {
        let old = hex(&patch["from"]);
        let new = hex(&patch["to"]);
        let matches: Vec<_> = component.windows(old.len()).enumerate().filter_map(|(index, row)| (row == old).then_some(index)).collect();
        assert_eq!(matches.len(), 1);
        component[matches[0]..matches[0] + old.len()].copy_from_slice(&new);
    }
    let mut module = 9;
    while component[module] & 128 != 0 { module += 1; }
    let fingerprint = component[module + 1..].iter().fold(0xcbf2_9ce4_8422_2325u64, |value, byte| {
        (value ^ u64::from(*byte)).wrapping_mul(0x0000_0100_0000_01b3)
    });
    let mut image = b"SEMIOPA\0\x01".to_vec();
    image.extend_from_slice(&framework_hash::Sha256::digest(&component));
    image.extend_from_slice(&fingerprint.to_le_bytes());
    image.extend_from_slice(&1u64.to_le_bytes());
    image.extend_from_slice(&(PAGE as u64).to_le_bytes());
    let mut memory = vec![0; PAGE];
    for (byte, value) in memory[64..66].iter_mut().zip(row["preparedChannel"].as_array().unwrap()) {
        *byte = value.as_u64().unwrap() as u8;
    }
    image.extend_from_slice(&memory);
    for count in [0u64, 0, 1, 0] {
        image.extend_from_slice(&count.to_le_bytes());
    }
    image.push(0);
    (component, image)
}

struct Scratch(PathBuf);
impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

#[test]
fn cold_retained_image_initializes_then_publishes() {
    let row: Value = serde_json::from_str(include_str!("🧫️fixtures/cold-image.json")).unwrap();
    let (bytes, image) = cold_fixture(&row);
    let plugin = row["pluginId"].as_str().unwrap();
    if let Some(path) = std::env::var_os(RUNNER) {
        let component = PathBuf::from(path);
        let image_path = component.with_file_name("📦️actor.semio");
        let shared = shared_plugin_runtime().unwrap();
        let hash = scoped_component_hash(&bytes, &ActivationScope::detached()).unwrap();
        assert!(cached_component(hash, &ActivationScope::detached()).unwrap().is_none());
        let refused = scoped_compiled_component(shared.as_ref(), plugin, ComponentBytes::File(&component), &ActivationScope::detached()).unwrap_err();
        assert_eq!(refused.code, GatewayErrorCode::PluginUnavailable);
        assert!(refused.message.contains("published digest differs"));
        assert!(cached_component(hash, &ActivationScope::detached()).unwrap().is_none());
        std::fs::write(&image_path, &image).unwrap();
        let used_image = Arc::new(AtomicBool::new(false));
        let reported = Arc::clone(&used_image);
        let scope = ActivationScope::new(semio_framework_async::CancelToken::root_now(), move |phase, progress| {
            if phase == ActivationPhase::CompilingComponent && progress == 0.5 {
                reported.store(true, Ordering::Release);
            }
        });
        let compiled = scoped_compiled_component(shared.as_ref(), plugin, ComponentBytes::File(&component), &scope).unwrap();
        assert!(used_image.load(Ordering::Acquire));
        assert!(cached_component(hash, &ActivationScope::detached()).unwrap().unwrap().same_origin(&compiled));
        assert_prepared_startup(shared.as_ref(), &compiled, &row);
        return
    }

    let mode = env!("SEMIO_BUILD_MODE");
    let crate_path = row["cratePath"].as_str().unwrap();
    let digest = framework_hash::sha256_hex(crate_path.as_bytes());
    let root = Scratch(std::env::temp_dir().join(format!("semio-cold-image-{}", std::process::id())));
    let model = root.0.join("browser").join(mode).join("component-models").join(&digest[..24]).join("0123456789abcdef01234567");
    std::fs::create_dir_all(&model).unwrap();
    let component = model.join("📦️component.wasm");
    std::fs::write(&component, &bytes).unwrap();
    let receipt = json!({"version":1,"mode":mode,"models":[{"pluginId":plugin,"cratePath":crate_path,"root":model,
        "preparedActor":{"imageSha256":framework_hash::sha256_hex(&image),"imageBytes":image.len()}}]});
    let mut altered = image.clone();
    altered[65 + 64] ^= 1;
    std::fs::write(model.join("📦️actor.semio"), &altered).unwrap();
    let (_, module) = module_path!().split_once("::").unwrap();
    let child = std::process::Command::new(std::env::current_exe().unwrap())
        .env(RUNNER, component)
        .env("SEMIO_MCP_COMPONENT_MODELS", receipt.to_string())
        .args(["--exact", &format!("{module}::cold_retained_image_initializes_then_publishes"), "--nocapture"])
        .output().unwrap();
    let output = String::from_utf8_lossy(&child.stdout);
    assert!(child.status.success() && output.contains("1 passed"), "{output}\n{}", String::from_utf8_lossy(&child.stderr));
}

fn assert_prepared_startup(runtime: &GuestRuntimes, compiled: &semio_plugin_host::CompiledHandle, row: &Value) {
    let mut caller = OriginalActorContextSlot::empty();
    let stock = RetainedCloneGrant { maximum_items: 20_000, maximum_copy_bytes: 4_000_000, maximum_capacity_bytes: 4_000_000, maximum_release_bytes: 4_000_000, maximum_depth: 64 };
    let mut input = RetainedTurnInput { operation: 71, generation: 3, epoch: 1, grant: stock,
        work: OriginalActorTurnWork { clock_binding: 195, absolute_deadline_us: default_now_us().unwrap() + 30_000_000, fuel: 100_000 } };
    let mut admitted = false;
    for _ in 0..30 {
        let call = caller.admit_step(input, startup_budget(input, 65_536), default_now_us);
        settle(&mut input, call.receipt_spent, call.work_units_spent);
        admitted = call.result.unwrap().ready;
        if admitted { break }
    }
    assert!(admitted, "cold image fixture did not admit its original caller");
    let actor = semio_framework_async::poll::resolve_ready(ActorId::new(1, 0, 1, 0));
    let mut opening = turn(&mut caller, &mut input, |cx| runtime.begin_original_actor_step(compiled, actor, cx));
    let mut spent = 0;
    for _ in 0..250 {
        let owner = opening.as_mut().unwrap();
        if owner.ready() { break }
        turn(&mut caller, &mut input, |cx| {
            let arguments = RetainedTurnInput::borrow_original(cx)?;
            let execution = owner.requires_execution().then(|| OriginalGuestExecutionBudget { input: arguments, fuel: 1, deadline_us: cx.deadline_us() });
            let call = runtime.advance_original_actor_step(owner, compiled, execution, cx)?;
            if let Some(receipt) = call.execution { spent += receipt.spent_fuel.unwrap(); }
            call.result?;
            Ok(())
        });
    }
    let owner = opening.as_ref().unwrap();
    assert!(owner.ready());
    let expected: Vec<_> = row["preparedChannel"].as_array().unwrap().iter().map(|byte| byte.as_u64().unwrap() as u8).collect();
    assert_eq!(&owner.original_memory(0).unwrap()[64..66], expected);
    assert_eq!(spent, 2, "only the actual ChannelVersion was executed during native startup");
    for _ in 0..100 {
        if turn(&mut caller, &mut input, |cx| runtime.close_original_actor_step(&mut opening, compiled, cx)) { break }
    }
    assert!(opening.is_none());
    for _ in 0..100 {
        if caller.terminal_is_empty() { return }
        let call = caller.close_step(input, startup_budget(input, 65_536));
        settle(&mut input, call.receipt_spent, call.work_units_spent);
        call.result.unwrap();
    }
    panic!("cold image fixture did not retire its original caller");
}

fn startup_budget(input: RetainedTurnInput, maximum_copy_bytes: usize) -> StepBudget {
    StepBudget::new(2000, input.work.absolute_deadline_us, RetainedCloneGrant {
        maximum_copy_bytes, maximum_items: 128, maximum_capacity_bytes: 200_000, maximum_release_bytes: 200_000, maximum_depth: 64,
    })
}

fn settle(input: &mut RetainedTurnInput, progress: RetainedCloneProgress, work: u64) {
    let receipt = input.return_original(progress, work).unwrap();
    input.epoch += 1;
    input.grant = receipt.remaining;
    input.work.fuel = receipt.remaining_fuel;
}

fn turn<T>(caller: &mut OriginalActorContextSlot, input: &mut RetainedTurnInput, body: impl FnOnce(&mut StepContext<'_>) -> Result<T, OriginalTurnFailure>) -> T {
    let call = caller.dispatch_step(*input, startup_budget(*input, 4096), &mut RetainedCloneProgress::default(), default_now_us, body);
    settle(input, call.receipt_spent, call.work_units_spent);
    call.result.unwrap()
}
