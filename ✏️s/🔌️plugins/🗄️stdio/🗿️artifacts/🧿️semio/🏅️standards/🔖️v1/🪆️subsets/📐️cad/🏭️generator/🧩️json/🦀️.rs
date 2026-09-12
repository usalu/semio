//! 🗂️ The `semio@v1/cad` `add-layer` mutation, expressed over this subset's own JSON carrier and
//! read back through `serde_json` — a third-party JSON implementation and nothing of ours.
//!
//! `add-layer`'s production diff (`📐️cad/🧬️schema/🧬️mutations/🗂️add-layer/🦀️.rs` delegating into
//! `agg_diff`) is `NamedTripleDiff{added: vec![layer.clone()], removed: [], modified: []}` on
//! `SemioCadSnapshot::layers` — a plain unique-named append, validated only for a non-duplicate
//! `name` (`validate_named_triple`, `✉️base/🧬️schema/🧰️triples/🦀️.rs`). No computed field, no
//! cross-reference. A genuinely fresh layer name makes a domain-blind JSON array push structurally
//! identical to production's own outcome.

use serde_json::{json, Value};

pub const KINDS: &[&str] = &["add-layer"];

/// 🌱️ A deterministic seed carrying one named layer, so `add-layer`'s own uniqueness constraint has
/// something real to be checked against.
pub fn build_seed() -> Value {
    json!({
        "schema": "stdio.semio.cad",
        "layers": [{"name": "walls", "colorIndex": 7, "lineType": "CONTINUOUS", "visible": true}],
        "blocks": [],
        "entities": []
    })
}

fn layers(doc: &mut Value) -> Result<&mut Vec<Value>, String> {
    doc.get_mut("layers").and_then(Value::as_array_mut).ok_or_else(|| "the seed declares a layers array".to_string())
}

/// ✍️ The forward mutation, as an edit to the JSON carrier: append one fresh, unique-named layer.
pub fn apply(kind: &str, doc: &Value) -> Result<Value, String> {
    let mut doc = doc.clone();
    match kind {
        "add-layer" => layers(&mut doc)?.push(json!({"name": "dimensions", "colorIndex": 3, "lineType": "DASHED", "visible": true})),
        other => return Err(format!("unknown kind {other}")),
    }
    Ok(doc)
}

/// 📄️ The projection: the ordered layer list, so an append is visible as a genuine length/content
/// difference rather than a reordering artifact.
pub fn project(bytes: &[u8]) -> Result<Value, String> {
    let parsed: Value = serde_json::from_slice(bytes).map_err(|error| error.to_string())?;
    let mut out = serde_json::Map::new();
    for key in ["schema", "layers", "blocks", "entities"] {
        out.insert(key.to_string(), parsed.get(key).cloned().unwrap_or(Value::Null));
    }
    Ok(Value::Object(out))
}
