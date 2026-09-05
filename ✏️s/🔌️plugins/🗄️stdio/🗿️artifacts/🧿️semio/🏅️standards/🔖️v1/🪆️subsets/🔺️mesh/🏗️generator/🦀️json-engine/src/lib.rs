//! 🎨 The `semio@v1/mesh` `create-material` mutation, expressed over this subset's own JSON carrier
//! and read back through `serde_json` — a third-party JSON implementation and nothing of ours.
//!
//! `create-material`'s production diff appends a caller-supplied `SemioMaterial` to
//! `SemioMeshSnapshot::materials` (an id-keyed append); a duplicate `id` already present in `base`
//! is a no-op per that leaf's own doc comment. No computed field, no cross-reference. A genuinely
//! fresh `id` makes a domain-blind JSON array push structurally identical to production's own
//! outcome.

use serde_json::{json, Value};

pub const KINDS: &[&str] = &["create-material"];

/// 🌱️ A deterministic seed carrying one named material, so `create-material`'s own uniqueness
/// constraint has something real to be checked against.
pub fn build_seed() -> Value {
    json!({
        "schema": "stdio.semio.mesh",
        "meshes": [],
        "materials": [{"id": "mat1", "baseColor": {"r": 0.8, "g": 0.8, "b": 0.8, "a": 1.0}, "metallic": 0.0, "roughness": 0.5}],
        "textures": []
    })
}

fn materials(doc: &mut Value) -> Result<&mut Vec<Value>, String> {
    doc.get_mut("materials").and_then(Value::as_array_mut).ok_or_else(|| "the seed declares a materials array".to_string())
}

/// ✍️ The forward mutation, as an edit to the JSON carrier: append one fresh, unique-id material.
pub fn apply(kind: &str, doc: &Value) -> Result<Value, String> {
    let mut doc = doc.clone();
    match kind {
        "create-material" => materials(&mut doc)?.push(json!({"id": "mat2", "baseColor": {"r": 0.2, "g": 0.4, "b": 0.9, "a": 1.0}, "metallic": 0.2, "roughness": 0.8})),
        other => return Err(format!("unknown kind {other}")),
    }
    Ok(doc)
}

/// 📄️ The projection: the ordered material list, so an append is visible as a genuine
/// length/content difference rather than a reordering artifact.
pub fn project(bytes: &[u8]) -> Result<Value, String> {
    let parsed: Value = serde_json::from_slice(bytes).map_err(|error| error.to_string())?;
    let mut out = serde_json::Map::new();
    for key in ["schema", "meshes", "materials", "textures"] {
        out.insert(key.to_string(), parsed.get(key).cloned().unwrap_or(Value::Null));
    }
    Ok(Value::Object(out))
}
