//! 🌱️ The `semio@v1/drawing` `create-layer` mutation, expressed over this subset's own JSON carrier
//! and read back through `serde_json` — a third-party JSON implementation and nothing of ours.
//!
//! `create-layer`'s production diff (`create-layer/🔺️diff/🦀️.rs`) inserts a caller-supplied
//! `DrawLayer` at `index.min(base.layers.len())`; an `index` at or past the end is a plain append,
//! and a duplicate `id` already present in `base` is rejected outright (fatal, never reached here
//! since the seed's own id is not reused). No computed field, no cross-reference. A domain-blind
//! JSON array push at the end is structurally identical to production's own outcome for an
//! end-of-list index.

use serde_json::{json, Value};

pub const KINDS: &[&str] = &["create-layer"];

/// 🌱️ A deterministic seed carrying one named layer with a minimal, valid `DrawNode` root (an
/// empty `Path`), so `create-layer`'s own uniqueness constraint has something real to be checked
/// against.
pub fn build_seed() -> Value {
    json!({
        "schema": "stdio.semio.drawing",
        "canvas": {"width": 100.0, "height": 100.0},
        "styles": [],
        "layers": [{"id": "layer1", "name": "Background", "visible": true, "root": {"kind": "path", "segments": []}}]
    })
}

fn layers(doc: &mut Value) -> Result<&mut Vec<Value>, String> {
    doc.get_mut("layers").and_then(Value::as_array_mut).ok_or_else(|| "the seed declares a layers array".to_string())
}

/// ✍️ The forward mutation, as an edit to the JSON carrier: append one fresh, unique-id layer at
/// the end (`index` == `base.layers.len()`).
pub fn apply(kind: &str, doc: &Value) -> Result<Value, String> {
    let mut doc = doc.clone();
    match kind {
        "create-layer" => layers(&mut doc)?.push(json!({"id": "layer2", "name": "Foreground", "visible": true, "root": {"kind": "path", "segments": []}})),
        other => return Err(format!("unknown kind {other}")),
    }
    Ok(doc)
}

/// 📄️ The projection: the ordered layer list, so an append is visible as a genuine
/// length/content difference rather than a reordering artifact.
pub fn project(bytes: &[u8]) -> Result<Value, String> {
    let parsed: Value = serde_json::from_slice(bytes).map_err(|error| error.to_string())?;
    let mut out = serde_json::Map::new();
    for key in ["schema", "canvas", "styles", "layers"] {
        out.insert(key.to_string(), parsed.get(key).cloned().unwrap_or(Value::Null));
    }
    Ok(Value::Object(out))
}
