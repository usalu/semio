//! 🏗️ The `semio@v1/brep` `create-vertex` mutation, expressed over this subset's own JSON carrier
//! and read back through `json` (json-rust) — a third-party JSON implementation and nothing of ours.
//!
//! `create-vertex`'s production diff appends a `{id, point}` to `SemioBrepSnapshot::vertices`
//! (an id-keyed `NamedTripleDiff`-style append); a duplicate `id` already present in `base` is a
//! no-op per that leaf's own doc comment, never a duplicate. No computed field, no cross-reference.
//! A genuinely fresh `id` makes a domain-blind JSON array push structurally identical to
//! production's own outcome.

use json::JsonValue;

pub const KINDS: &[&str] = &["create-vertex"];

/// 🗂️ The reviewed fixture directory each kind's pair is committed under.
pub const FIXTURE_DIRECTORY_BY_KIND: &[(&str, &str)] = &[("create-vertex", "➕️create-vertex-applied")];

fn literal(text: &str) -> JsonValue {
    json::parse(text).expect("a carrier literal is valid JSON")
}

/// 🌱️ A deterministic seed carrying one named vertex, so `create-vertex`'s own uniqueness
/// constraint has something real to be checked against.
pub fn build_seed() -> JsonValue {
    literal(r#"{"schema": "stdio.semio.brep", "vertices": [{"id": "v1", "point": {"x": 0.0, "y": 0.0, "z": 0.0}}], "edges": [], "loops": [], "faces": [], "shells": [], "solids": []}"#)
}

/// ✍️ The forward mutation, as an edit to the JSON carrier: append one fresh, unique-id vertex.
pub fn apply(kind: &str, doc: &JsonValue) -> Result<JsonValue, String> {
    let mut doc = doc.clone();
    match kind {
        "create-vertex" => match &mut doc["vertices"] {
            JsonValue::Array(items) => items.push(literal(r#"{"id": "v2", "point": {"x": 1.0, "y": 1.0, "z": 1.0}}"#)),
            _ => return Err("the seed declares a vertices array".to_string()),
        },
        other => return Err(format!("unknown kind {other}")),
    }
    Ok(doc)
}

/// 🔤️ Orders every object's keys, the committed carrier spelling; arrays keep their order.
pub fn canonical(value: &JsonValue) -> JsonValue {
    match value {
        JsonValue::Object(object) => {
            let mut entries: Vec<(&str, &JsonValue)> = object.iter().collect();
            entries.sort_by(|left, right| left.0.cmp(right.0));
            let mut sorted = json::object::Object::with_capacity(entries.len());
            for (key, member) in entries {
                sorted.insert(key, canonical(member));
            }
            JsonValue::Object(sorted)
        }
        JsonValue::Array(items) => JsonValue::Array(items.iter().map(canonical).collect()),
        other => other.clone(),
    }
}

/// 🖨️ The committed file bytes of one carrier.
pub fn render(value: &JsonValue) -> String {
    format!("{}\n", canonical(value).pretty(2))
}

/// 📄️ The projection: the ordered vertex list, so an append is visible as a genuine
/// length/content difference rather than a reordering artifact.
pub fn project(bytes: &[u8]) -> Result<JsonValue, String> {
    let text = std::str::from_utf8(bytes).map_err(|error| error.to_string())?;
    let parsed = json::parse(text).map_err(|error| error.to_string())?;
    let mut out = json::object::Object::new();
    for key in ["schema", "vertices", "edges", "loops", "faces", "shells", "solids"] {
        out.insert(key, canonical(&parsed[key]));
    }
    Ok(JsonValue::Object(out))
}
