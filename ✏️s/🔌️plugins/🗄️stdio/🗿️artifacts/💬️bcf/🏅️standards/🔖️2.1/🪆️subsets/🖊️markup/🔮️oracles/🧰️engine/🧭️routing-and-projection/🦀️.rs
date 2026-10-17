//! 🧭️ Routing and projection of the BCF oracle: the mutation entry points, the round trip and the semantic projection. See [`super`].

use super::*;

//#region 🔖️Routing
pub fn apply_mutation(input: &[u8], kind: &str, params: &Json) -> Result<Vec<u8>, String> {
    let mut doc = decode(input)?;
    apply_kind(&mut doc, kind, params)?;
    encode(&doc)
}

/// ↩️ Applies `{kind, params}` and then its computed inverse, in sequence — the caller compares
/// its projection against the ORIGINAL input's own.
pub fn apply_mutation_inverse(input: &[u8], kind: &str, params: &Json) -> Result<Vec<u8>, String> {
    let base = decode(input)?;
    let mutated = apply_mutation(input, kind, params)?;
    match inverse_spec(&base, kind, params)? {
        Some(inverse) => apply_mutation(&mutated, &inverse.str("kind"), &inverse.get("params").cloned().unwrap_or(Json::Null)),
        None => Ok(mutated),
    }
}

/// 🔁️ The identity round trip's own producer: the `zip`+`quick-xml` composition decodes the archive and re-encodes it
/// from its own model alone.
pub fn round_trip(input: &[u8]) -> Result<Vec<u8>, String> {
    encode(&decode(input)?)
}

//#region 🔖️Projection
fn topic_projection(topic: &OTopic) -> Json {
    obj(vec![
        ("title", Json::String(topic.title.clone())),
        ("description", Json::String(topic.description.clone())),
        ("status", Json::String(topic.status.clone())),
        ("priority", Json::String(topic.priority.clone())),
        ("labels", strings_json(&topic.labels)),
        ("creationDate", Json::String(topic.creation_date.clone())),
        ("creationAuthor", Json::String(topic.creation_author.clone())),
        ("comments", Json::Object(topic.comments.iter().map(|comment| (comment.guid.clone(), comment_to_json(comment))).collect())),
        ("viewpoints", Json::Object(topic.viewpoints.iter().map(|viewpoint| (viewpoint.guid.clone(), viewpoint_projection(viewpoint))).collect())),
    ])
}

/// 👁️ A viewpoint's snapshot projects as a size+digest pair, not raw bytes — the same
/// content-addressed treatment the `🎒️zip`/`🎨️svg` precedents already give opaque binary
/// payloads, so the comparison mechanism's per-number tolerance never has to look inside a PNG.
fn viewpoint_projection(viewpoint: &OViewpoint) -> Json {
    obj(vec![
        (
            "camera",
            match &viewpoint.camera {
                Some(camera) => camera_to_json(camera),
                None => Json::Null,
            },
        ),
        (
            "components",
            match &viewpoint.components {
                Some(components) => components_to_json(components),
                None => Json::Null,
            },
        ),
        (
            "snapshotSize",
            match &viewpoint.snapshot {
                Some(bytes) => Json::Number(bytes.len() as f64),
                None => Json::Null,
            },
        ),
        (
            "snapshotDigest",
            match &viewpoint.snapshot {
                Some(bytes) => Json::String(digest(bytes)),
                None => Json::Null,
            },
        ),
    ])
}

/// 👁️ This subset's own semantic projection — version, every topic keyed by guid (comments and
/// viewpoints keyed by guid in turn), and every raw retained part keyed by name — independently
/// re-derived by re-decoding `bytes` through this module's own `zip`+`quick-xml` composition.
pub fn project(bytes: &[u8]) -> Result<Json, String> {
    let doc = decode(bytes)?;
    Ok(obj(vec![
        ("version", Json::String(doc.version)),
        ("topics", Json::Object(doc.topics.iter().map(|topic| (topic.guid.clone(), topic_projection(topic))).collect())),
        ("parts", Json::Object(doc.parts.iter().map(|part| (part.name.clone(), obj(vec![("size", Json::Number(part.data.len() as f64)), ("digest", Json::String(digest(&part.data)))]))).collect())),
    ]))
}
//#endregion 🔖️Projection
//#endregion 🔖️Routing
