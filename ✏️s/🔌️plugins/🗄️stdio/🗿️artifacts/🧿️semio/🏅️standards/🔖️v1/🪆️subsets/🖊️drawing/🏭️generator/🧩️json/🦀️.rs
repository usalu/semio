//! 🌱️ The `semio@v1/drawing` `create-layer` mutation, expressed over this subset's own JSON carrier
//! and read back through `json` (json-rust) — a third-party JSON implementation and nothing of ours.
//!
//! `create-layer`'s production diff (`create-layer/🔺️diff/🦀️.rs`) inserts a caller-supplied
//! `DrawLayer` at `index.min(base.layers.len())`; an `index` at or past the end is a plain append,
//! and a duplicate `id` already present in `base` is rejected outright (fatal, never reached here
//! since the seed's own id is not reused). No computed field, no cross-reference. A domain-blind
//! JSON array push at the end is structurally identical to production's own outcome for an
//! end-of-list index.

use json::JsonValue;

pub const KINDS: &[&str] = &["create-layer"];

/// 🗂️ The reviewed fixture directory each kind's pair is committed under.
pub const FIXTURE_DIRECTORY_BY_KIND: &[(&str, &str)] = &[("create-layer", "🗂️create-layer-applied")];

fn literal(text: &str) -> JsonValue {
    json::parse(text).expect("a carrier literal is valid JSON")
}

/// 🌱️ A deterministic seed carrying one named layer with a minimal, valid `DrawNode` root (an
/// empty `Path`), so `create-layer`'s own uniqueness constraint has something real to be checked
/// against.
pub fn build_seed() -> JsonValue {
    literal(r#"{"schema": "stdio.semio.drawing", "canvas": {"width": 100.0, "height": 100.0}, "styles": [], "layers": [{"id": "layer1", "name": "Background", "visible": true, "root": {"kind": "path", "segments": []}}]}"#)
}

/// ✍️ The forward mutation, as an edit to the JSON carrier: append one fresh, unique-id layer at
/// the end (`index` == `base.layers.len()`).
pub fn apply(kind: &str, doc: &JsonValue) -> Result<JsonValue, String> {
    let mut doc = doc.clone();
    match kind {
        "create-layer" => match &mut doc["layers"] {
            JsonValue::Array(items) => items.push(literal(r#"{"id": "layer2", "name": "Foreground", "visible": true, "root": {"kind": "path", "segments": []}}"#)),
            _ => return Err("the seed declares a layers array".to_string()),
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

/// 📄️ The projection: the ordered layer list, so an append is visible as a genuine
/// length/content difference rather than a reordering artifact.
pub fn project(bytes: &[u8]) -> Result<JsonValue, String> {
    let text = std::str::from_utf8(bytes).map_err(|error| error.to_string())?;
    let parsed = json::parse(text).map_err(|error| error.to_string())?;
    let mut out = json::object::Object::new();
    for key in ["schema", "canvas", "styles", "layers"] {
        out.insert(key, canonical(&parsed[key]));
    }
    Ok(JsonValue::Object(out))
}
