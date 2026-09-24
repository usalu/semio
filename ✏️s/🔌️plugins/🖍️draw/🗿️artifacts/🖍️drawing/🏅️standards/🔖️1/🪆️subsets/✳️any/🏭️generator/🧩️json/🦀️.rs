//! 🖍️ The `drawing@1/any` LAYER-METADATA mutation vocabulary, expressed over this subset's own JSON
//! carrier and read back through `json` (json-rust) — a third-party JSON implementation and nothing
//! of ours.
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

use json::JsonValue;

pub const KINDS: &[&str] = &["set-layer-locked", "set-layer-blend-mode", "rename-layer"];

/// 🗂️ The reviewed fixture directory, relative to the `🪆️subsets` root, each kind's pair is committed under.
pub const FIXTURE_DIRECTORY_BY_KIND: &[(&str, &str)] = &[
    ("set-layer-locked", "🏷️metadata/🧫️fixtures/🔒️set-layer-locked"),
    ("set-layer-blend-mode", "🎨️style/🧫️fixtures/🌓️set-layer-blend-mode"),
    ("rename-layer", "🏷️metadata/🧫️fixtures/🏷️rename-layer"),
];

fn literal(text: &str) -> JsonValue {
    json::parse(text).expect("a carrier literal is valid JSON")
}

fn layer(id: &str, name: &str, locked: bool, blend: &str, x: &str) -> String {
    format!(
        r#"{{"kind": "shape", "id": "{id}", "name": "{name}", "visible": true, "locked": {locked}, "opacity": 1.0, "blendMode": "{blend}",
            "transform": {{"x": {x}, "y": 0.0, "scaleX": 1.0, "scaleY": 1.0, "rotation": 0.0}},
            "attributes": {{}}, "shapeKind": "rect", "rect": {{"x": 0.0, "y": 0.0, "width": 120.0, "height": 80.0}}}}"#
    )
}

/// 🌱️ A deterministic two-layer seed. Two layers, so a mutation targeting the first is distinguishable
/// from one targeting the second, and neither is the only thing in the document. Field spelling follows
/// the snapshot's own contract: camelCase fields, `kind: "shape"` on the tagged `DrawingLayerNode`, and
/// `DrawingLayerBase` FLATTENED into the body, so its fields sit beside `shapeKind` rather than under a
/// `base` key.
pub fn build_seed() -> JsonValue {
    literal(&format!(
        r#"{{"schema": "drawing.drawing/1", "id": "drawing-carrier-seed", "title": "carrier seed", "layers": [{}, {}], "artboard": {{"width": 800.0, "height": 600.0}}}}"#,
        layer("l1", "background", false, "normal", "0.0"),
        layer("l2", "annotation", true, "multiply", "160.0")
    ))
}

/// ✍️ The forward mutation, as an edit to the JSON carrier.
pub fn apply(kind: &str, doc: &JsonValue) -> Result<JsonValue, String> {
    let mut doc = doc.clone();
    let first = &mut doc["layers"][0];
    if !first.is_object() {
        return Err("the seed declares at least one layer".to_string());
    }
    match kind {
        "set-layer-locked" => first["locked"] = true.into(),
        "set-layer-blend-mode" => first["blendMode"] = "screen".into(),
        "rename-layer" => first["name"] = "backdrop".into(),
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

/// 📄️ The projection: the document identity plus the ORDERED layer list with every field the three
/// kinds touch. Order is preserved, so a reordering would be a difference rather than a tie.
pub fn project(bytes: &[u8]) -> Result<JsonValue, String> {
    let text = std::str::from_utf8(bytes).map_err(|error| error.to_string())?;
    let parsed = json::parse(text).map_err(|error| error.to_string())?;
    let mut out = json::object::Object::new();
    for key in ["schema", "id", "title", "artboard", "layers"] {
        out.insert(key, canonical(&parsed[key]));
    }
    Ok(JsonValue::Object(out))
}
