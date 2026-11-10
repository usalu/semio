//! 🧰️ Test-only model surface: the genuine fixture-caller turn every world crate's tests share (feature `testkit`).

use crate::{World3dBuildContext, WorldCursorWakeAuthority, WORLD_PLACEHOLDER_PUMP_FUEL, WORLD_PLACEHOLDER_PUMP_GRANT};

/// ⛽️ The Work the fixture caller's own Source admission consumes, measured once from an exact admission against a surplus and then asserted equal on every later admission.
fn world_fixture_admission_fuel() -> u64 {
    static WORK: std::sync::OnceLock<u64> = std::sync::OnceLock::new();
    *WORK.get_or_init(|| {
        const SURPLUS: u64 = 1 << 20;
        let mut probe = world_fixture_caller(SURPLUS, world_fixture_stock());
        let work = SURPLUS - probe.fuel_remaining();
        while !probe.terminal_is_empty() {
            probe.retire_step(world_fixture_stock()).expect("original fixture probe retirement");
        }
        work
    })
}

/// 🛑️ The fail-fast ceiling of retirement pump turns one fixture caller may take.
pub const WORLD_FIXTURE_RETIREMENT_TURNS: usize = 4096;

fn world_fixture_stock() -> semio_framework_value::RetainedCloneGrant {
    let grant = world_original_fixture_grant();
    semio_framework_value::RetainedCloneGrant { maximum_items: grant.maximum_items * 4, maximum_copy_bytes: grant.maximum_copy_bytes * 4, maximum_capacity_bytes: grant.maximum_capacity_bytes * 4, maximum_release_bytes: grant.maximum_release_bytes * 4, maximum_depth: grant.maximum_depth }
}

fn world_fixture_caller(fuel: u64, stock: semio_framework_value::RetainedCloneGrant) -> semio_framework_job::OriginalJobFixtureCaller {
    semio_framework_job::OriginalJobFixtureCaller::admit(semio_framework_job::OperationId(1), semio_framework_job::Generation(1), stock, semio_framework_job::default_now_us, u64::MAX, fuel)
}

/// 🪙️ Runs one closure in a genuine fixture-caller turn (parent source, shared clock) whose budget is exactly `fuel` after the caller's measured admission Work, and retires the caller afterwards.
pub fn with_world_step_context<T>(fuel: u64, step: impl FnOnce(&mut semio_framework_job::StepContext<'_>) -> T) -> T {
    let grant = world_original_fixture_grant();
    let stock = world_fixture_stock();
    let mut caller = world_fixture_caller(fuel + world_fixture_admission_fuel(), stock);
    assert_eq!(caller.fuel_remaining(), fuel, "the fixture caller's admission consumed exactly its measured Work");
    let result = caller.turn(grant, |cx| Ok(step(cx))).expect("original fixture turn");
    caller.fund_fuel(u64::try_from(stock.maximum_items).unwrap_or(u64::MAX));
    for turn in 0..WORLD_FIXTURE_RETIREMENT_TURNS {
        if caller.terminal_is_empty() {
            break;
        }
        caller.retire_step(stock).expect("original fixture caller retirement");
        assert!(turn + 1 < WORLD_FIXTURE_RETIREMENT_TURNS, "the fixture caller retires within its turn ceiling");
    }
    result
}

/// 🪙️ Runs one closure inside the original allocation effect of a genuine fixture-caller turn funded with the shared world grant.
pub fn with_world_effect<T>(step: impl FnOnce(&mut semio_framework_value::RetainedAllocationEffect<'_>) -> T) -> T {
    let grant = world_original_fixture_grant();
    with_world_step_context(64, |context| context.with_original_effect(grant, |_, effect| step(effect)).expect("original fixture effect").expect("the shared world grant admits the effect").0)
}

/// 🏛️ Runs one closure inside the effect of a host-funded owner-less pump turn with the production placeholder pump grant, the way a render frame steps a placeholder cursor.
pub fn with_world_pump_effect<T>(step: impl FnOnce(&mut semio_framework_value::RetainedAllocationEffect<'_>) -> T) -> T {
    semio_framework_job::OriginalRetirementPump::new().turn(WORLD_PLACEHOLDER_PUMP_FUEL, u64::MAX, || Some(0), WORLD_PLACEHOLDER_PUMP_GRANT).expect("original pump turn").with_effect(step).expect("original pump effect").expect("the pump grant admits the effect").0
}

/// 🎫️ The fixed retained grant of one world fixture turn, read from the shared law.
pub fn world_original_fixture_grant()->semio_framework_value::RetainedCloneGrant{
    let law:serde_json::Value=serde_json::from_str(&std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../🧪️tests/🔬️unit/🧫️retained-attributes.json")).expect("shared retained-attributes law")).unwrap();let axes=&law["grant"];
    semio_framework_value::RetainedCloneGrant{maximum_items:axes["items"].as_u64().unwrap()as usize,maximum_copy_bytes:axes["copyBytes"].as_u64().unwrap()as usize,maximum_capacity_bytes:axes["capacityBytes"].as_u64().unwrap()as usize,maximum_release_bytes:axes["releaseBytes"].as_u64().unwrap()as usize,maximum_depth:axes["depth"].as_u64().unwrap()as usize}
}

/// 🧱️ A world build context whose slot pages are born through genuine fixture-caller turns, the way a frame build prepares its resources before any render.
pub fn prepared_world_build_context() -> World3dBuildContext {
    let mut resources = World3dBuildContext::new(WorldCursorWakeAuthority::new());
    for turn in 0..WORLD_FIXTURE_RETIREMENT_TURNS {
        if with_world_step_context(64, |context| resources.prepare_step(context)).expect("original world resource preparation") {
            return resources;
        }
        assert!(turn + 1 < WORLD_FIXTURE_RETIREMENT_TURNS, "the world resources prepare within their turn ceiling");
    }
    unreachable!("the world resources prepare within their turn ceiling")
}

/// 🧹️ Walks a world build context through its paid close, one genuine fixture-caller turn per step.
pub fn retire_world_build_context(mut resources: World3dBuildContext) {
    for turn in 0..WORLD_FIXTURE_RETIREMENT_TURNS {
        if resources.terminal_is_empty() {
            return;
        }
        with_world_step_context(64, |context| resources.close_step(context)).expect("original world resource close");
        assert!(turn + 1 < WORLD_FIXTURE_RETIREMENT_TURNS, "the world resources close within their turn ceiling");
    }
}

/// 🚧️ The mesh authority and the announcement registries are process-wide by design, so every law of a test binary takes this one gate for its whole body and runs alone whatever `--test-threads` says.
pub fn world_test_gate() -> std::sync::MutexGuard<'static, ()> {
    static GATE: std::sync::Mutex<()> = std::sync::Mutex::new(());
    GATE.lock().unwrap_or_else(std::sync::PoisonError::into_inner)
}

