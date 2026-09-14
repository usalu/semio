//! 🦀️ Rust side of the export content-hash case. Builds a snapshot through the ExportSource port and
//! projects its identity, its namespaced input ids and the refusal of an unchanged re-export.

use semio_framework_repo_events as events;
use semio_framework_repo_events::serde_json::{self, Value as SerdeJson};
use semio_repo_test_host::{Adapter, Context, Json, Outcome};
use std::fs;

//#region 🔖️Vectors
fn entities(ctx: &Context) -> Result<Vec<events::ExportEntity>, String> {
    let bytes = ctx.fixture_bytes("shared://📤️export-vectors.json")?;
    let parsed: SerdeJson = serde_json::from_slice(&bytes).map_err(|error| error.to_string())?;
    parsed
        .get("entities")
        .and_then(SerdeJson::as_array)
        .ok_or("export vectors have no entities array")?
        .iter()
        .map(|entity| {
            Ok(events::ExportEntity {
                kind: entity.get("kind").and_then(SerdeJson::as_str).ok_or("export entity has no kind")?.to_string(),
                id: entity.get("id").and_then(SerdeJson::as_str).ok_or("export entity has no id")?.to_string(),
                value: events::Payload::of(&entity.get("value").cloned().ok_or("export entity has no value")?).map_err(|error| error.to_string())?,
            })
        })
        .collect()
}

fn snapshot(ctx: &Context) -> Result<events::ExportSnapshot, String> {
    let source = entities(ctx)?;
    events::build_export_snapshot(&source, &events::Uninterrupted)
        .map_err(|error| error.to_string())
}
//#endregion 🔖️Vectors

//#region 🔖️Scenarios
fn snapshot_identity(ctx: &Context) -> Result<Outcome, String> {
    let snapshot = snapshot(ctx)?;
    Ok(Outcome::projection(Json::Object(vec![
        ("snapshot".to_string(), Json::String(snapshot.snapshot.clone())),
        (
            "counts".to_string(),
            Json::Object(
                snapshot
                    .counts
                    .iter()
                    .map(|(kind, count)| (kind.clone(), Json::Number(*count as f64)))
                    .collect(),
            ),
        ),
    ])))
}

fn input_ids_are_namespaced(ctx: &Context) -> Result<Outcome, String> {
    let snapshot = snapshot(ctx)?;
    Ok(Outcome::projection(Json::Object(vec![(
        "inputIds".to_string(),
        Json::Array(snapshot.inputs.iter().map(|input| Json::String(input.id.clone())).collect()),
    )])))
}

fn unchanged_export_is_refused(ctx: &Context) -> Result<Outcome, String> {
    let snapshot = snapshot(ctx)?;
    // 🧹️The harness reuses one work directory per (case, role, implementation) and never empties it,
    // so a log left by an earlier run would make the FIRST append the duplicate. The scenario owns a
    // directory it clears first.
    let dir = ctx.work_dir.join("unchanged-export-is-refused");
    if dir.exists() {
        fs::remove_dir_all(&dir).map_err(|error| error.to_string())?;
    }
    fs::create_dir_all(&dir).map_err(|error| error.to_string())?;
    let store = events::Store::new(dir.join("export.events.jsonl"));
    store.append(&snapshot.inputs, &events::Uninterrupted).map_err(|error| error.to_string())?;
    let before = fs::read(&store.path).map_err(|error| error.to_string())?;
    let refused = matches!(
        store.append(&snapshot.inputs, &events::Uninterrupted),
        Err(events::StoreError::Duplicate(_))
    );
    let after = fs::read(&store.path).map_err(|error| error.to_string())?;
    Ok(Outcome::projection(Json::Object(vec![
        ("duplicateRefused".to_string(), Json::Bool(refused)),
        ("logUnchanged".to_string(), Json::Bool(before == after)),
        ("eventCount".to_string(), Json::Number(snapshot.inputs.len() as f64)),
    ])))
}
//#endregion 🔖️Scenarios

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> Adapter {
    Adapter::new("rust")
        .subject("snapshot-identity", snapshot_identity)
        .subject("input-ids-are-namespaced", input_ids_are_namespaced)
        .subject("unchanged-export-is-refused", unchanged_export_is_refused)
}
//#endregion 🔖️Registration
