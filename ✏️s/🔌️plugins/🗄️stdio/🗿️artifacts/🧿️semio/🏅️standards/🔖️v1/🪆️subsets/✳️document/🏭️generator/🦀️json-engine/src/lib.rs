//! 📄️ The `semio@v1/document` IMAGE mutation vocabulary, expressed over this subset's own JSON carrier
//! and read back through `serde_json` — a third-party JSON implementation and nothing of ours.
//!
//! Why these three: this subset's two registered readers judge the docx and markdown exports. An
//! embedded image's raw BYTES survive neither faithfully — docx stores them as separate zip media
//! parts and markdown references them by path — so `insert-image`, `remove-image` and `set-image-bytes`
//! were recorded `-uncarried` against both.
//!
//! `SemioDocumentSnapshot::images` is an INLINE `Vec<DocImage>` carrying `{id, mime, bytes}` directly,
//! so all three are carrier-level facts here. Contrast `mathematical` and `sequence`, which look
//! similar and are not: their mutated state lives in composed CHILD artifacts behind `ArtifactChild`
//! handles whose `local_owner` is `#[serde(skip)]`, so it never reaches their own JSON at all.

use serde_json::{json, Map, Value};

pub const KINDS: &[&str] = &["insert-image", "remove-image", "set-image-bytes"];

/// 🌱️ A deterministic seed carrying two images and a block tree that references both. Two, so
/// `remove-image` leaves something behind and a mutation on one is distinguishable from the other.
/// `DocImage::bytes` is `Vec<u8>`, which serde renders as a JSON array of numbers.
pub fn build_seed() -> Value {
    json!({
        "schema": "s.stdio.semio.document/v1",
        "styles": [{"id": "body", "name": "Body", "basedOn": null}],
        "images": [
            {"id": "img1", "mime": "image/png", "bytes": [137, 80, 78, 71, 13, 10, 26, 10, 1, 2, 3, 4]},
            {"id": "img2", "mime": "image/jpeg", "bytes": [255, 216, 255, 224, 9, 8, 7, 6]}
        ],
        "blocks": [
            {"kind": "paragraph", "styleId": "body", "runs": [{"text": "figure one", "style": {"bold": false, "italic": false, "underline": false}}]},
            {"kind": "image", "imageId": "img1", "alt": "first figure"},
            {"kind": "image", "imageId": "img2", "alt": "second figure"}
        ]
    })
}

pub fn arrange(_kind: &str, doc: &Value) -> Value {
    doc.clone()
}

fn images(doc: &mut Value) -> Result<&mut Vec<Value>, String> {
    doc.get_mut("images").and_then(Value::as_array_mut).ok_or_else(|| "the seed declares an images array".to_string())
}

/// ✍️ The forward mutation, as an edit to the JSON carrier.
pub fn apply(kind: &str, doc: &Value) -> Result<Value, String> {
    let mut doc = doc.clone();
    match kind {
        "insert-image" => images(&mut doc)?.push(json!({"id": "img3", "mime": "image/png", "bytes": [137, 80, 78, 71, 99, 98, 97, 96]})),
        "remove-image" => {
            images(&mut doc)?.retain(|i| i.get("id").and_then(Value::as_str) != Some("img2"));
            // 🔗️The block referencing it goes too: a document keeping an `imageId` that resolves to
            // nothing is not what this mutation produces, and committing one would make the fixture a
            // record of a bug rather than of the mutation.
            if let Some(blocks) = doc.get_mut("blocks").and_then(Value::as_array_mut) {
                blocks.retain(|b| b.get("imageId").and_then(Value::as_str) != Some("img2"));
            }
        }
        "set-image-bytes" => {
            let list = images(&mut doc)?;
            list[0].as_object_mut().ok_or("img1 is not an object")?.insert("bytes".to_string(), json!([137, 80, 78, 71, 13, 10, 26, 10, 44, 55, 66, 77, 88]));
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

/// 📄️ The projection: the ORDERED image list with its raw bytes, plus the block tree that references
/// it — so a removal that orphans a reference is visible as a difference in both places.
pub fn project(bytes: &[u8]) -> Result<Value, String> {
    let parsed: Value = serde_json::from_slice(bytes).map_err(|error| error.to_string())?;
    let mut out = Map::new();
    for key in ["schema", "styles", "images", "blocks"] {
        out.insert(key.to_string(), canonical(parsed.get(key).unwrap_or(&Value::Null)));
    }
    Ok(Value::Object(out))
}
