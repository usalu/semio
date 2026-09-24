//! 🗂️ The `semio@v1/cad` `add-layer` mutation, expressed over this subset's own JSON carrier and
//! read back through `json` (json-rust) — a third-party JSON implementation and nothing of ours.
//!
//! `add-layer`'s production diff (`📐️cad/🧬️schema/🧬️mutations/🗂️add-layer/🦀️.rs` delegating into
//! `agg_diff`) is `NamedTripleDiff{added: vec![layer.clone()], removed: [], modified: []}` on
//! `SemioCadSnapshot::layers` — a plain unique-named append, validated only for a non-duplicate
//! `name` (`validate_named_triple`, `✉️base/🧬️schema/🧰️triples/🦀️.rs`). No computed field, no
//! cross-reference. A genuinely fresh layer name makes a domain-blind JSON array push structurally
//! identical to production's own outcome.

use json::JsonValue;

pub const KINDS: &[&str] = &["add-layer"];

/// 🗂️ The reviewed fixture directory each kind's pair is committed under.
pub const FIXTURE_DIRECTORY_BY_KIND: &[(&str, &str)] = &[("add-layer", "🗂️add-layer-applied")];

fn literal(text: &str) -> JsonValue {
    json::parse(text).expect("a carrier literal is valid JSON")
}

/// 🌱️ A deterministic seed carrying one named layer, so `add-layer`'s own uniqueness constraint has
/// something real to be checked against.
pub fn build_seed() -> JsonValue {
    literal(r#"{"schema": "stdio.semio.cad", "layers": [{"name": "walls", "colorIndex": 7, "lineType": "CONTINUOUS", "visible": true}], "blocks": [], "entities": []}"#)
}

/// ✍️ The forward mutation, as an edit to the JSON carrier: append one fresh, unique-named layer.
pub fn apply(kind: &str, doc: &JsonValue) -> Result<JsonValue, String> {
    let mut doc = doc.clone();
    match kind {
        "add-layer" => match &mut doc["layers"] {
            JsonValue::Array(items) => items.push(literal(r#"{"name": "dimensions", "colorIndex": 3, "lineType": "DASHED", "visible": true}"#)),
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

/// 📄️ The projection: the ordered layer list, so an append is visible as a genuine length/content
/// difference rather than a reordering artifact.
pub fn project(bytes: &[u8]) -> Result<JsonValue, String> {
    let text = std::str::from_utf8(bytes).map_err(|error| error.to_string())?;
    let parsed = json::parse(text).map_err(|error| error.to_string())?;
    let mut out = json::object::Object::new();
    for key in ["schema", "layers", "blocks", "entities"] {
        out.insert(key, canonical(&parsed[key]));
    }
    Ok(JsonValue::Object(out))
}
