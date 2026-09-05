//! 🏗️ The `semio@v1/brep` `create-vertex` mutation, expressed over this subset's own JSON carrier
//! and read back through `serde_json` — a third-party JSON implementation and nothing of ours.
//!
//! `create-vertex`'s production diff appends a `{id, point}` to `SemioBrepSnapshot::vertices`
//! (an id-keyed `NamedTripleDiff`-style append); a duplicate `id` already present in `base` is a
//! no-op per that leaf's own doc comment, never a duplicate. No computed field, no cross-reference.
//! A genuinely fresh `id` makes a domain-blind JSON array push structurally identical to
//! production's own outcome.

use serde_json::{json, Value};

pub const KINDS: &[&str] = &["create-vertex"];

/// 🌱️ A deterministic seed carrying one named vertex, so `create-vertex`'s own uniqueness
/// constraint has something real to be checked against.
pub fn build_seed() -> Value {
    json!({
        "schema": "stdio.semio.brep",
        "vertices": [{"id": "v1", "point": {"x": 0.0, "y": 0.0, "z": 0.0}}],
        "edges": [],
        "loops": [],
        "faces": [],
        "shells": [],
        "solids": []
    })
}

fn vertices(doc: &mut Value) -> Result<&mut Vec<Value>, String> {
    doc.get_mut("vertices").and_then(Value::as_array_mut).ok_or_else(|| "the seed declares a vertices array".to_string())
}

/// ✍️ The forward mutation, as an edit to the JSON carrier: append one fresh, unique-id vertex.
pub fn apply(kind: &str, doc: &Value) -> Result<Value, String> {
    let mut doc = doc.clone();
    match kind {
        "create-vertex" => vertices(&mut doc)?.push(json!({"id": "v2", "point": {"x": 1.0, "y": 1.0, "z": 1.0}})),
        other => return Err(format!("unknown kind {other}")),
    }
    Ok(doc)
}

/// 📄️ The projection: the ordered vertex list, so an append is visible as a genuine
/// length/content difference rather than a reordering artifact.
pub fn project(bytes: &[u8]) -> Result<Value, String> {
    let parsed: Value = serde_json::from_slice(bytes).map_err(|error| error.to_string())?;
    let mut out = serde_json::Map::new();
    for key in ["schema", "vertices", "edges", "loops", "faces", "shells", "solids"] {
        out.insert(key.to_string(), parsed.get(key).cloned().unwrap_or(Value::Null));
    }
    Ok(Value::Object(out))
}
