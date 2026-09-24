//! 🦀️ Rust side of the filesystem-fault case. Replays the fixture's failure indices against a real
//! log and projects, per index, whether the append was refused and the committed prefix survived.

use semio_framework_repo_coordinator as coordinator;
use semio_framework_repo_coordinator::serde_json::{self, Value as SerdeJson};
use semio_repo_test_host::{Adapter, Context, Json, Outcome};
use std::fs;
use std::path::Path;

//#region 🔖️Vectors
struct FaultPlan {
    seed: coordinator::EventInput,
    second: coordinator::EventInput,
    fail_at: Vec<usize>,
}

fn input_of(value: &SerdeJson) -> coordinator::EventInput {
    coordinator::EventInput {
        stream: value.get("stream").and_then(SerdeJson::as_str).unwrap_or_default().to_string(),
        id: value.get("id").and_then(SerdeJson::as_str).unwrap_or_default().to_string(),
        generation: value.get("generation").and_then(SerdeJson::as_u64).unwrap_or_default(),
        kind: value.get("type").and_then(SerdeJson::as_str).unwrap_or_default().to_string(),
        payload: value.get("payload").cloned().unwrap_or(SerdeJson::Null),
    }
}

fn fault_plan(ctx: &Context) -> Result<FaultPlan, String> {
    let bytes = ctx.fixture_bytes("shared://💥️filesystem-fault-recovery/💥️fault-plan.json")?;
    let parsed: SerdeJson = serde_json::from_slice(&bytes).map_err(|error| error.to_string())?;
    Ok(FaultPlan {
        seed: input_of(parsed.get("seed").ok_or("fault plan has no seed")?),
        second: input_of(parsed.get("second").ok_or("fault plan has no second")?),
        fail_at: parsed
            .get("failAt")
            .and_then(SerdeJson::as_array)
            .ok_or("fault plan has no failAt array")?
            .iter()
            .filter_map(SerdeJson::as_u64)
            .map(|value| value as usize)
            .collect(),
    })
}

fn recovery_artifacts(path: &Path) -> Vec<Json> {
    [".stage", ".stage.next", ".next", ".backup"]
        .iter()
        .filter(|suffix| Path::new(&format!("{}{suffix}", path.display())).exists())
        .map(|suffix| Json::String((*suffix).to_string()))
        .collect()
}
//#endregion 🔖️Vectors

//#region 🔖️Scenarios
fn every_injected_fault_preserves_the_committed_log(ctx: &Context) -> Result<Outcome, String> {
    let plan = fault_plan(ctx)?;
    let root = ctx.work_dir.join("filesystem-fault-recovery");
    if root.exists() {
        fs::remove_dir_all(&root).map_err(|error| error.to_string())?;
    }
    fs::create_dir_all(&root).map_err(|error| error.to_string())?;
    let mut attempts = Vec::new();
    for index in &plan.fail_at {
        let path = root.join("fault.jsonl");
        if path.exists() {
            fs::remove_file(&path).map_err(|error| error.to_string())?;
        }
        let store = coordinator::EventStore::open(&path, coordinator::StoreLimits::default()).map_err(|error| error.to_string())?;
        store.append(0, std::slice::from_ref(&plan.seed)).map_err(|error| error.to_string())?;
        let before = fs::read(&path).map_err(|error| error.to_string())?;
        store.arm_fault(*index);
        let attempted = store.append(1, std::slice::from_ref(&plan.second));
        store.arm_fault(0);
        let after = fs::read(&path).map_err(|error| error.to_string())?;
        let reopened = coordinator::EventStore::open(&path, coordinator::StoreLimits::default());
        let replayed = reopened.as_ref().ok().map(coordinator::EventStore::replay);
        attempts.push(Json::Object(vec![
            ("failAt".to_string(), Json::Number(*index as f64)),
            ("refused".to_string(), Json::Bool(attempted.is_err())),
            ("committed".to_string(), Json::Bool(matches!(&attempted, Ok(result) if result.committed) || matches!(&attempted, Err(coordinator::StoreError::PendingCleanup(_))))),
            ("logUnchanged".to_string(), Json::Bool(before == after)),
            ("reopened".to_string(), Json::Bool(reopened.is_ok())),
            ("replayOk".to_string(), Json::Bool(replayed.as_ref().is_some_and(Result::is_ok))),
            ("replayCount".to_string(), Json::Number(replayed.as_ref().and_then(|result| result.as_ref().ok()).map_or(0, Vec::len) as f64)),
            ("artifacts".to_string(), Json::Array(recovery_artifacts(&path))),
        ]));
    }
    Ok(Outcome::projection(Json::Object(vec![("attempts".to_string(), Json::Array(attempts))])))
}
//#endregion 🔖️Scenarios

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> Adapter {
    Adapter::new("rust").subject("every-injected-fault-preserves-the-committed-log", every_injected_fault_preserves_the_committed_log)
}
//#endregion 🔖️Registration
