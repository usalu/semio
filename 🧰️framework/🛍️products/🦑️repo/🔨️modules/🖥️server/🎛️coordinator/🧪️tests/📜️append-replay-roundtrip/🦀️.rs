//! 🦀️ Rust side of the append/replay case. Appends the fixture vectors into the case work directory
//! and projects sequences, checksums, the log bytes and the refusals.

use semio_framework_repo_coordinator as coordinator;
use semio_framework_repo_coordinator::serde_json::{self, Value as SerdeJson};
use semio_repo_test_host::{Adapter, Context, Json, Outcome};
use std::fs;
use std::path::PathBuf;

//#region 🔖️Vectors
struct AppendVectors {
    inputs: Vec<coordinator::EventInput>,
    sequences: Vec<u64>,
}

fn append_vectors(ctx: &Context) -> Result<AppendVectors, String> {
    let bytes = ctx.fixture_bytes("shared://📜️append-vectors.json")?;
    let parsed: SerdeJson = serde_json::from_slice(&bytes).map_err(|error| error.to_string())?;
    let inputs = parsed
        .get("inputs")
        .and_then(SerdeJson::as_array)
        .ok_or("append vectors have no inputs array")?
        .iter()
        .map(|input| coordinator::EventInput {
            stream: input.get("stream").and_then(SerdeJson::as_str).unwrap_or_default().to_string(),
            id: input.get("id").and_then(SerdeJson::as_str).unwrap_or_default().to_string(),
            generation: input.get("generation").and_then(SerdeJson::as_u64).unwrap_or_default(),
            kind: input.get("type").and_then(SerdeJson::as_str).unwrap_or_default().to_string(),
            payload: input.get("payload").cloned().unwrap_or(SerdeJson::Null),
        })
        .collect();
    let sequences = parsed
        .get("sequences")
        .and_then(SerdeJson::as_array)
        .ok_or("append vectors have no sequences array")?
        .iter()
        .filter_map(SerdeJson::as_u64)
        .collect();
    Ok(AppendVectors { inputs, sequences })
}

/// 🧹️ An empty directory of its own for one scenario, because the harness reuses one work directory
/// per (case, role, implementation) and never empties it.
fn fresh_scenario_dir(ctx: &Context, name: &str) -> Result<PathBuf, String> {
    let dir = ctx.work_dir.join(name);
    if dir.exists() {
        fs::remove_dir_all(&dir).map_err(|error| error.to_string())?;
    }
    fs::create_dir_all(&dir).map_err(|error| error.to_string())?;
    Ok(dir)
}
//#endregion 🔖️Vectors

//#region 🔖️Scenarios
fn append_then_replay_yields_frozen_sequences(ctx: &Context) -> Result<Outcome, String> {
    let vectors = append_vectors(ctx)?;
    let dir = fresh_scenario_dir(ctx, "append-then-replay")?;
    let first_path = dir.join("first.jsonl");
    let first = coordinator::EventStore::open(&first_path, coordinator::StoreLimits::default()).map_err(|error| error.to_string())?;
    first.append(0, &vectors.inputs).map_err(|error| error.to_string())?;
    let first_bytes = fs::read(&first_path).map_err(|error| error.to_string())?;
    let replayed = first.replay().map_err(|error| error.to_string())?;
    let second_path = dir.join("second.jsonl");
    let second = coordinator::EventStore::open(&second_path, coordinator::StoreLimits::default()).map_err(|error| error.to_string())?;
    second.append(0, &vectors.inputs).map_err(|error| error.to_string())?;
    let second_bytes = fs::read(&second_path).map_err(|error| error.to_string())?;
    Ok(Outcome::projection(Json::Object(vec![
        ("sequences".to_string(), Json::Array(replayed.iter().map(|event| Json::Number(event.sequence as f64)).collect())),
        ("frozen".to_string(), Json::Array(vectors.sequences.iter().map(|value| Json::Number(*value as f64)).collect())),
        ("checksums".to_string(), Json::Array(replayed.iter().map(|event| Json::String(event.checksum.clone())).collect())),
        ("log".to_string(), Json::String(String::from_utf8_lossy(&first_bytes).to_string())),
        ("deterministic".to_string(), Json::Bool(first_bytes == second_bytes)),
    ])))
}

fn duplicate_is_idempotent_and_corruption_is_refused(ctx: &Context) -> Result<Outcome, String> {
    let vectors = append_vectors(ctx)?;
    let dir = fresh_scenario_dir(ctx, "duplicate-and-corrupt")?;
    let path = dir.join("refused.jsonl");
    let store = coordinator::EventStore::open(&path, coordinator::StoreLimits::default()).map_err(|error| error.to_string())?;
    store.append(0, &vectors.inputs[..1]).map_err(|error| error.to_string())?;
    let before = fs::read(&path).map_err(|error| error.to_string())?;
    let duplicate = store.append(1, &vectors.inputs[..1]);
    let after_duplicate = fs::read(&path).map_err(|error| error.to_string())?;
    let stale = store.append(0, &vectors.inputs[1..2]);
    let mut corrupt = before.clone();
    let middle = corrupt.len() / 2;
    corrupt[middle] ^= 1;
    fs::write(&path, &corrupt).map_err(|error| error.to_string())?;
    let replayed = store.replay();
    Ok(Outcome::projection(Json::Object(vec![
        ("duplicateReported".to_string(), Json::Bool(duplicate.map(|result| result.duplicate).unwrap_or(false))),
        ("logUnchanged".to_string(), Json::Bool(before == after_duplicate)),
        ("staleRefused".to_string(), Json::Bool(matches!(stale, Err(coordinator::StoreError::SequenceConflict(_))))),
        ("corruptDetected".to_string(), Json::Bool(matches!(replayed, Err(coordinator::StoreError::Corrupt(_))))),
    ])))
}
//#endregion 🔖️Scenarios

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> Adapter {
    Adapter::new("rust")
        .subject("append-then-replay-yields-frozen-sequences", append_then_replay_yields_frozen_sequences)
        .subject("duplicate-is-idempotent-and-corruption-is-refused", duplicate_is_idempotent_and_corruption_is_refused)
}
//#endregion 🔖️Registration
