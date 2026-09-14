//! 🦀️ Rust side of the append-only store case. Appends the fixture vectors into the case work
//! directory and projects sequences, file bytes, checksums and the outcome of every interruption.

use semio_framework_repo_events as events;
use semio_framework_repo_events::serde_json::{self, Value as SerdeJson};
use semio_repo_test_host::{Adapter, Context, Json, Outcome};
use std::cell::Cell;
use std::fs;

//#region 🔖️Vectors
struct StoreVectors {
    inputs: Vec<events::Input>,
    interrupt_phases: Vec<String>,
}

fn store_vectors(ctx: &Context) -> Result<StoreVectors, String> {
    let bytes = ctx.fixture_bytes("shared://🗄️store-vectors.json")?;
    let parsed: SerdeJson = serde_json::from_slice(&bytes).map_err(|error| error.to_string())?;
    let inputs = serde_json::from_value(parsed.get("inputs").cloned().ok_or("store vectors have no inputs array")?).map_err(|error: serde_json::Error| error.to_string())?;
    let interrupt_phases = parsed
        .get("interruptPhases")
        .and_then(SerdeJson::as_array)
        .ok_or("store vectors have no interruptPhases array")?
        .iter()
        .map(|phase| phase.as_str().ok_or("interrupt phase is not a string").map(str::to_string))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(StoreVectors { inputs, interrupt_phases })
}

/// 🧹️ An empty directory of its own for one scenario.
///
/// The harness reuses one work directory per (case, role, implementation) and never empties it, so a
/// log file left by an earlier scenario — or by an earlier run of this same case — is still there when
/// the next append starts, and the store rightly refuses it as a duplicate. Every scenario that writes
/// a log therefore owns a directory it clears first.
fn fresh_scenario_dir(ctx: &Context, name: &str) -> Result<std::path::PathBuf, String> {
    let dir = ctx.work_dir.join(name);
    if dir.exists() {
        fs::remove_dir_all(&dir).map_err(|error| error.to_string())?;
    }
    fs::create_dir_all(&dir).map_err(|error| error.to_string())?;
    Ok(dir)
}

struct PhaseCancel {
    phase: String,
    hit: Cell<bool>,
}

impl events::Interrupt for PhaseCancel {
    fn cancelled(&self) -> bool {
        self.hit.get()
    }
    fn report(&self, progress: events::Progress) {
        if progress.step == self.phase {
            self.hit.set(true);
        }
    }
}
//#endregion 🔖️Vectors

//#region 🔖️Scenarios
fn append_then_replay_sequences(ctx: &Context) -> Result<Outcome, String> {
    let vectors = store_vectors(ctx)?;
    let dir = fresh_scenario_dir(ctx, "append-then-replay-sequences")?;
    let first = events::Store::new(dir.join("first.jsonl"));
    first.append(&vectors.inputs, &events::Uninterrupted).map_err(|error| error.to_string())?;
    let first_bytes = fs::read(&first.path).map_err(|error| error.to_string())?;
    let replayed = first.replay(&events::Uninterrupted).map_err(|error| error.to_string())?;
    let second = events::Store::new(dir.join("second.jsonl"));
    second.append(&vectors.inputs, &events::Uninterrupted).map_err(|error| error.to_string())?;
    let second_bytes = fs::read(&second.path).map_err(|error| error.to_string())?;
    Ok(Outcome::projection(Json::Object(vec![
        (
            "sequences".to_string(),
            Json::Array(replayed.iter().map(|event| Json::Number(event.sequence as f64)).collect()),
        ),
        (
            "checksums".to_string(),
            Json::Array(
                replayed.iter().map(|event| Json::String(event.checksum.clone())).collect(),
            ),
        ),
        ("logDigest".to_string(), Json::String(events::digest(&first_bytes))),
        ("deterministic".to_string(), Json::Bool(first_bytes == second_bytes)),
    ])))
}

fn duplicate_and_corrupt_are_refused(ctx: &Context) -> Result<Outcome, String> {
    let vectors = store_vectors(ctx)?;
    let dir = fresh_scenario_dir(ctx, "duplicate-and-corrupt-are-refused")?;
    let store = events::Store::new(dir.join("refused.jsonl"));
    store.append(&vectors.inputs[..1], &events::Uninterrupted).map_err(|error| error.to_string())?;
    let mut before = fs::read(&store.path).map_err(|error| error.to_string())?;
    let duplicate = matches!(
        store.append(&vectors.inputs[..1], &events::Uninterrupted),
        Err(events::StoreError::Duplicate(_))
    );
    let middle = before.len() / 2;
    before[middle] ^= 1;
    fs::write(&store.path, &before).map_err(|error| error.to_string())?;
    let corrupt =
        matches!(store.replay(&events::Uninterrupted), Err(events::StoreError::Corrupt(_)));
    Ok(Outcome::projection(Json::Object(vec![
        ("duplicateRefused".to_string(), Json::Bool(duplicate)),
        ("corruptDetected".to_string(), Json::Bool(corrupt)),
    ])))
}

fn interruption_preserves_committed_log(ctx: &Context) -> Result<Outcome, String> {
    let vectors = store_vectors(ctx)?;
    let dir = fresh_scenario_dir(ctx, "interruption-preserves-committed-log")?;
    let mut phases = Vec::new();
    for phase in &vectors.interrupt_phases {
        let store = events::Store::new(dir.join(format!("{phase}.jsonl")));
        store
            .append(&vectors.inputs[..1], &events::Uninterrupted)
            .map_err(|error| error.to_string())?;
        let before = fs::read(&store.path).map_err(|error| error.to_string())?;
        let watcher = PhaseCancel { phase: phase.clone(), hit: Cell::new(false) };
        let cancelled =
            store.append(&vectors.inputs[1..], &watcher) == Err(events::StoreError::Cancelled);
        let after = fs::read(&store.path).map_err(|error| error.to_string())?;
        let mut stage = store.path.clone().into_os_string();
        stage.push(".stage");
        let replayed = store.replay(&events::Uninterrupted);
        phases.push(Json::Object(vec![
            ("phase".to_string(), Json::String(phase.clone())),
            ("cancelled".to_string(), Json::Bool(cancelled)),
            ("unchanged".to_string(), Json::Bool(before == after)),
            ("stageGone".to_string(), Json::Bool(!std::path::PathBuf::from(stage).exists())),
            (
                "replayCount".to_string(),
                Json::Number(replayed.as_ref().map(|events| events.len()).unwrap_or_default() as f64),
            ),
            ("replayOk".to_string(), Json::Bool(replayed.is_ok())),
        ]));
    }
    Ok(Outcome::projection(Json::Object(vec![("phases".to_string(), Json::Array(phases))])))
}

fn record_checksum_is_sha256(ctx: &Context) -> Result<Outcome, String> {
    let vectors = store_vectors(ctx)?;
    let dir = fresh_scenario_dir(ctx, "record-checksum-is-sha256")?;
    let store = events::Store::new(dir.join("checksum.jsonl"));
    let appended =
        store.append(&vectors.inputs, &events::Uninterrupted).map_err(|error| error.to_string())?;
    let records = appended
        .iter()
        .map(|event| {
            Json::Object(vec![
                ("id".to_string(), Json::String(event.id.clone())),
                ("kind".to_string(), Json::String(event.kind.clone())),
                ("sequence".to_string(), Json::Number(event.sequence as f64)),
                (
                    "data".to_string(),
                    Json::String(serde_json::to_string(&event.data).unwrap_or_default()),
                ),
                ("checksum".to_string(), Json::String(event.checksum.clone())),
            ])
        })
        .collect();
    Ok(Outcome::projection(Json::Object(vec![("records".to_string(), Json::Array(records))])))
}
//#endregion 🔖️Scenarios

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> Adapter {
    Adapter::new("rust")
        .subject("append-then-replay-sequences", append_then_replay_sequences)
        .subject("duplicate-and-corrupt-are-refused", duplicate_and_corrupt_are_refused)
        .subject("interruption-preserves-committed-log", interruption_preserves_committed_log)
        .subject("record-checksum-is-sha256", record_checksum_is_sha256)
}
//#endregion 🔖️Registration
