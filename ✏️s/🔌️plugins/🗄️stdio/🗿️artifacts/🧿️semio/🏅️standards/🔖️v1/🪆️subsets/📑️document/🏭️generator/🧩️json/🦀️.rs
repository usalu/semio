//! 📄️ The `semio@v1/document` IMAGE mutation vocabulary, expressed over this subset's own JSON carrier
//! and read back through `json` (json-rust) — a third-party JSON implementation and nothing of ours.
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

use json::JsonValue;

pub const KINDS: &[&str] = &["insert-image", "remove-image", "set-image-bytes"];

/// 🗂️ The reviewed fixture directory each kind's pair is committed under.
pub const FIXTURE_DIRECTORY_BY_KIND: &[(&str, &str)] = &[("insert-image", "🖼️insert-image"), ("remove-image", "🪦️remove-image"), ("set-image-bytes", "📀️set-image-bytes")];

fn literal(text: &str) -> JsonValue {
    json::parse(text).expect("a carrier literal is valid JSON")
}

fn images(doc: &mut JsonValue) -> Result<&mut Vec<JsonValue>, String> {
    match &mut doc["images"] {
        JsonValue::Array(items) => Ok(items),
        _ => Err("the seed declares an images array".to_string()),
    }
}

/// 🌱️ A deterministic seed carrying two images and a block tree that references both. Two, so
/// `remove-image` leaves something behind and a mutation on one is distinguishable from the other.
/// `DocImage::bytes` is `Vec<u8>`, which the carrier renders as a JSON array of numbers.
pub fn build_seed() -> JsonValue {
    literal(
        r#"{
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
        }"#,
    )
}

/// ✍️ The forward mutation, as an edit to the JSON carrier. `remove-image` drops the block that
/// references the image too: a document keeping an `imageId` that resolves to nothing is not what the
/// mutation produces, and committing one would record a bug rather than the mutation.
pub fn apply(kind: &str, doc: &JsonValue) -> Result<JsonValue, String> {
    let mut doc = doc.clone();
    match kind {
        "insert-image" => images(&mut doc)?.push(literal(r#"{"id": "img3", "mime": "image/png", "bytes": [137, 80, 78, 71, 99, 98, 97, 96]}"#)),
        "remove-image" => {
            images(&mut doc)?.retain(|image| image["id"] != "img2");
            if let JsonValue::Array(blocks) = &mut doc["blocks"] {
                blocks.retain(|block| block["imageId"] != "img2");
            }
        }
        "set-image-bytes" => images(&mut doc)?[0]["bytes"] = literal("[137, 80, 78, 71, 13, 10, 26, 10, 44, 55, 66, 77, 88]"),
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

/// 📄️ The projection: the ORDERED image list with its raw bytes, plus the block tree that references
/// it — so a removal that orphans a reference is visible as a difference in both places.
pub fn project(bytes: &[u8]) -> Result<JsonValue, String> {
    let text = std::str::from_utf8(bytes).map_err(|error| error.to_string())?;
    let parsed = json::parse(text).map_err(|error| error.to_string())?;
    let mut out = json::object::Object::new();
    for key in ["schema", "styles", "images", "blocks"] {
        out.insert(key, canonical(&parsed[key]));
    }
    Ok(JsonValue::Object(out))
}
