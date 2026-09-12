//! 🖍️ The `drawing@1/any` LAYER-METADATA mutation vocabulary, expressed over this subset's own JSON
//! carrier and read back through `serde_json` — a third-party JSON implementation and nothing of ours.
//!
//! Why these three and no others: this subset already carries `quick-xml-drawing-1-mutate`, which judges
//! the SVG export. SVG has no representation for a layer's `locked` flag, its `blendMode`, or its
//! authoring `name` — they are editor metadata that never reaches the rendered document — so those
//! three kinds were recorded `-uncarried` against it. That was honest about the SVG carrier.
//!
//! Unlike `mathematical` and `sequence`, whose mutated state lives in COMPOSED CHILD artifacts behind
//! `ArtifactChild` handles whose `local_owner` is `#[serde(skip)]` (so it never reaches their JSON at
//! all), `DrawingSnapshot::layers` is an INLINE `Vec<DrawingLayerNode>`. The three fields are therefore
//! genuine carrier-level facts here, and a JSON reader witnesses every one.

use serde_json::{json, Map, Value};

pub const KINDS: &[&str] = &["set-layer-locked", "set-layer-blend-mode", "rename-layer"];

/// 🌱️ A deterministic two-layer seed. Two layers, so a mutation targeting the first is distinguishable
/// from one targeting the second, and neither is the only thing in the document. Field spelling follows
/// the snapshot's own serde contract: `#[serde(rename_all = "camelCase")]`, `#[serde(tag = "kind")]` on
/// `DrawingLayerNode` with `#[serde(rename = "shape")]` on the variant, and `DrawingLayerBase` FLATTENED into
/// the body (`#[serde(flatten)]`), so its fields sit beside `shapeKind` rather than under a `base` key.
pub fn build_seed() -> Value {
    let layer = |id: &str, name: &str, locked: bool, blend: &str, x: f64| {
        json!({
            "kind": "shape",
            "id": id,
            "name": name,
            "visible": true,
            "locked": locked,
            "opacity": 1.0,
            "blendMode": blend,
            "transform": {"x": x, "y": 0.0, "scaleX": 1.0, "scaleY": 1.0, "rotation": 0.0},
            "attributes": {},
            "shapeKind": "rect",
            "rect": {"x": 0.0, "y": 0.0, "width": 120.0, "height": 80.0}
        })
    };
    json!({
        "schema": "drawing.drawing/1",
        "id": "drawing-carrier-seed",
        "title": "carrier seed",
        "layers": [layer("l1", "background", false, "normal", 0.0), layer("l2", "annotation", true, "multiply", 160.0)],
        "artboard": {"width": 800.0, "height": 600.0}
    })
}

pub fn arrange(_kind: &str, doc: &Value) -> Value {
    doc.clone()
}

fn first_layer(doc: &mut Value) -> Result<&mut Map<String, Value>, String> {
    doc.get_mut("layers")
        .and_then(Value::as_array_mut)
        .and_then(|layers| layers.first_mut())
        .and_then(Value::as_object_mut)
        .ok_or_else(|| "the seed declares at least one layer".to_string())
}

/// ✍️ The forward mutation, as an edit to the JSON carrier.
pub fn apply(kind: &str, doc: &Value) -> Result<Value, String> {
    let mut doc = doc.clone();
    match kind {
        "set-layer-locked" => {
            first_layer(&mut doc)?.insert("locked".to_string(), Value::Bool(true));
        }
        "set-layer-blend-mode" => {
            first_layer(&mut doc)?.insert("blendMode".to_string(), Value::String("screen".to_string()));
        }
        "rename-layer" => {
            first_layer(&mut doc)?.insert("name".to_string(), Value::String("backdrop".to_string()));
        }
        other => return Err(format!("unknown kind {other}")),
    }
    Ok(doc)
}

fn canonical(value: &Value) -> Value {
    match value {
        Value::Object(map) => {
            let mut sorted = Map::new();
            let mut keys: Vec<&String> = map.keys().collect();
            keys.sort();
            for key in keys {
                sorted.insert(key.clone(), canonical(&map[key]));
            }
            Value::Object(sorted)
        }
        Value::Array(items) => Value::Array(items.iter().map(canonical).collect()),
        other => other.clone(),
    }
}

/// 📄️ The projection: the document identity plus the ORDERED layer list with every field the three
/// kinds touch. Order is preserved, so a reordering would be a difference rather than a tie.
pub fn project(bytes: &[u8]) -> Result<Value, String> {
    let parsed: Value = serde_json::from_slice(bytes).map_err(|error| error.to_string())?;
    let mut out = Map::new();
    for key in ["schema", "id", "title", "artboard"] {
        out.insert(key.to_string(), canonical(parsed.get(key).unwrap_or(&Value::Null)));
    }
    out.insert("layers".to_string(), canonical(parsed.get("layers").unwrap_or(&Value::Null)));
    Ok(Value::Object(out))
}
