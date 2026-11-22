//! 💼️ The spawned-job ledger against the language-neutral `💼️job-ledger` vectors the TypeScript ledger also drives.

use semio_renderer_wgpu_host_base::{JobLedger, SpawnedJob};
use serde_json::Value;
use std::sync::{atomic::{AtomicUsize, Ordering}, Arc};

fn fixture() -> Value {
    serde_json::from_str(include_str!("../../../../../🧪️tests/🧫️fixtures/💼️job-ledger/🔣️.json")).expect("job ledger vectors")
}

fn job(value: &Value) -> SpawnedJob {
    let text = |name: &str| value[name].as_str().unwrap().to_string();
    SpawnedJob { key: text("key"), plugin_id: text("pluginId"), actor_id: text("actorId"), instance_id: value["instanceId"].as_u64().unwrap() as u32, job: value["job"].as_u64().unwrap(), kind: text("kind"), started_at_ms: value["startedAtMs"].as_u64().unwrap() }
}

#[test]
fn every_vector_case_leaves_the_declared_rows() {
    for case in fixture()["cases"].as_array().unwrap() {
        let id = case["id"].as_str().unwrap();
        let mut ledger = JobLedger::new();
        let mut cancels: std::collections::HashMap<String, Arc<AtomicUsize>> = Default::default();
        for op in case["ops"].as_array().unwrap() {
            let key = op["key"].as_str().unwrap_or_default();
            match op["op"].as_str().unwrap() {
                "begin" => {
                    let spawned = job(&op["job"]);
                    let counter = cancels.entry(spawned.key.clone()).or_default().clone();
                    assert_eq!(ledger.begin(spawned, Arc::new(move || { counter.fetch_add(1, Ordering::Relaxed); })), op["opened"].as_bool().unwrap(), "{id}");
                }
                "step" => ledger.step(key, op["steps"].as_u64().unwrap(), op["progressed"].as_bool().unwrap(), op["atMs"].as_u64().unwrap()),
                "end" => ledger.end(key),
                "cancel" => {
                    let cancel = ledger.request_cancel(key);
                    assert_eq!(cancel.is_some(), op["accepted"].as_bool().unwrap(), "{id}");
                    if let Some(cancel) = cancel { cancel(); }
                }
                "cancelCalls" => assert_eq!(cancels.get(key).map_or(0, |count| count.load(Ordering::Relaxed)), op["count"].as_u64().unwrap() as usize, "{id}"),
                "snapshot" => {
                    let rows: Vec<Value> = ledger.snapshot().iter().map(|row| serde_json::json!({ "key": row.job.key, "steps": row.steps, "reportsProgress": row.reports_progress, "progressAtMs": row.progress_at_ms, "cancelling": row.cancelling })).collect();
                    assert_eq!(Value::Array(rows), op["rows"], "{id}");
                }
                other => panic!("{id}: unknown op {other}"),
            }
        }
    }
}
