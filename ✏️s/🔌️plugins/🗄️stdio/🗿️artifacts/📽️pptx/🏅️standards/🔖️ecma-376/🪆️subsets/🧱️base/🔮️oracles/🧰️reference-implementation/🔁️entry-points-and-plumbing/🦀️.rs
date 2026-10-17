//! 🔁️ Entry points and plumbing of the PPTX oracle: the apply, inverse, round-trip and projection entry points and the content-type and relationship plumbing. See [`super`].

use super::*;

//#region 🔖️Routing
pub fn apply_mutation(input: &[u8], kind: &str, params: &Json) -> Result<Vec<u8>, String> {
    write_zip(&apply(read_zip(input)?, kind, params)?)
}

/// ↩️ Applies `{kind, params}`, re-reads the forward result, and restores the pre-mutation package — every kind's
/// own inverse restores the base package — so the caller compares that projection
/// against the ORIGINAL input's own. A forward result the reference cannot re-read fails here.
pub fn apply_mutation_inverse(input: &[u8], kind: &str, params: &Json) -> Result<Vec<u8>, String> {
    let forward = apply_mutation(input, kind, params)?;
    read_presentation(&read_zip(&forward)?)?;
    match plumbing_inverse(&read_zip(input)?, kind, params)? {
        Some((undo_kind, undo_params)) => apply_mutation(&forward, &undo_kind, &undo_params),
        None => write_zip(&read_zip(input)?),
    }
}

/// ↩️ The wire spec that undoes a plumbing kind, read from the pre-mutation package: the previous row at its position, or the removal of the row the forward wrote.
fn plumbing_inverse(base: &Package, kind: &str, params: &Json) -> Result<Option<(String, Json)>, String> {
    let object = |entries: Vec<(&str, Json)>| Json::Object(entries.into_iter().map(|(key, value)| (key.to_string(), value)).collect());
    let bool_of = |key: &str| matches!(params.get(key), Some(Json::Bool(true)));
    Ok(match kind {
        "set-relationship" | "remove-relationship" => {
            let owner = params.str("owner");
            let id = params.str("id");
            let previous = plumbing_rows(base, &rels_path_for(&owner), "Relationship")?.into_iter().enumerate().find(|(_, row)| row.attr("Id") == Some(id.as_str()));
            match previous {
                Some((at, row)) => Some((
                    "set-relationship".to_string(),
                    object(vec![
                        ("owner", Json::String(owner)),
                        ("id", Json::String(id)),
                        ("relType", Json::String(row.attr("Type").unwrap_or_default().to_string())),
                        ("target", Json::String(row.attr("Target").unwrap_or_default().to_string())),
                        ("external", Json::Bool(row.attr("TargetMode") == Some("External"))),
                        ("index", Json::Number(at as f64)),
                    ]),
                )),
                None if kind == "set-relationship" => Some(("remove-relationship".to_string(), object(vec![("owner", Json::String(owner)), ("id", Json::String(id))]))),
                None => None,
            }
        }
        "set-content-type" | "remove-content-type" => {
            let is_override = bool_of("isOverride");
            let (tag, key) = if is_override { ("Override", "PartName") } else { ("Default", "Extension") };
            let name = params.str("name");
            let previous = plumbing_rows(base, CONTENT_TYPES_PART, tag)?.into_iter().enumerate().find(|(_, row)| row.attr(key) == Some(name.as_str()));
            match previous {
                Some((at, row)) => Some((
                    "set-content-type".to_string(),
                    object(vec![("isOverride", Json::Bool(is_override)), ("name", Json::String(name)), ("contentType", Json::String(row.attr("ContentType").unwrap_or_default().to_string())), ("index", Json::Number(at as f64))]),
                )),
                None if kind == "set-content-type" => Some(("remove-content-type".to_string(), object(vec![("isOverride", Json::Bool(is_override)), ("name", Json::String(name))]))),
                None => None,
            }
        }
        _ => None,
    })
}

/// 🧱️ The rows named `tag` of an OPC plumbing part, in order; none when the part is absent.
fn plumbing_rows(pkg: &Package, part: &str, tag: &str) -> Result<Vec<XNode>, String> {
    if !pkg.parts.contains_key(part) {
        return Ok(Vec::new());
    }
    Ok(pkg.xml(part)?.children_named(tag).cloned().collect())
}

/// 🔁️ Decodes with the independent reader and re-encodes with the reference writer, no
/// mutation applied — the identity round trip this subset's `identity-round-trip` scenario
/// checks (real re-serialization every time, per this module's own header note — never a
/// pass-through of the original bytes).
pub fn round_trip(input: &[u8]) -> Result<Vec<u8>, String> {
    let pkg = read_zip(input)?;
    let slides = read_presentation(&pkg)?;
    write_zip(&write_presentation(&pkg, &slides)?)
}

/// 👁️ This subset's own semantic projection — ordered slide list, each slide's ordered shape
/// list (kind/text/position), independently re-derived by re-parsing `bytes` through the
/// registered `zip` + `quick-xml` reference implementations rather than trusting whatever
/// produced them.
pub fn project(bytes: &[u8]) -> Result<Json, String> {
    let pkg = read_zip(bytes)?;
    let slides = read_presentation(&pkg)?;
    Ok(Json::Object(vec![("slideCount".into(), Json::Number(slides.len() as f64)), ("slides".into(), Json::Array(slides.iter().map(slide_to_json).collect()))]))
}
//#endregion 🔖️Routing

//#region 🔖️Plumbing
/// 🪢️ The package plumbing of `bytes` as the independent reader states it. @see [`plumbing_json`].
pub fn project_plumbing(bytes: &[u8]) -> Result<Json, String> {
    plumbing_json(&read_zip(bytes)?)
}

/// 🪢️ The owner part a `*.rels` part belongs to (`""` for the package root).
fn owner_of_rels(path: &str) -> Option<String> {
    let file = path.rsplit('/').next()?.strip_suffix(".rels")?;
    let directory = path[..path.len() - path.rsplit('/').next()?.len()].strip_suffix("_rels/")?;
    Some(format!("{directory}{file}"))
}

/// 🪢️ The package plumbing in the order the package states it: the `[Content_Types].xml` defaults and overrides as ordered pairs, and every owner's relationships as
/// ordered `{id, type, target, external}` rows (owners sorted by part name).
fn plumbing_json(pkg: &Package) -> Result<Json, String> {
    let pairs = |tag: &str, key: &str| -> Result<Json, String> {
        Ok(Json::Array(plumbing_rows(pkg, CONTENT_TYPES_PART, tag)?.iter().map(|row| Json::Array(vec![Json::String(row.attr(key).unwrap_or_default().to_string()), Json::String(row.attr("ContentType").unwrap_or_default().to_string())])).collect()))
    };
    let mut owners: Vec<(String, String)> = pkg.parts.keys().filter_map(|path| owner_of_rels(path).map(|owner| (owner, path.clone()))).collect();
    owners.sort();
    let mut relationships = Vec::with_capacity(owners.len());
    for (owner, path) in owners {
        let rows = plumbing_rows(pkg, &path, "Relationship")?
            .iter()
            .map(|rel| {
                Json::Object(vec![
                    ("id".to_string(), Json::String(rel.attr("Id").unwrap_or_default().to_string())),
                    ("type".to_string(), Json::String(rel.attr("Type").unwrap_or_default().to_string())),
                    ("target".to_string(), Json::String(rel.attr("Target").unwrap_or_default().to_string())),
                    ("external".to_string(), Json::Bool(rel.attr("TargetMode") == Some("External"))),
                ])
            })
            .collect();
        relationships.push((owner, Json::Array(rows)));
    }
    Ok(Json::Object(vec![("defaults".to_string(), pairs("Default", "Extension")?), ("overrides".to_string(), pairs("Override", "PartName")?), ("relationships".to_string(), Json::Object(relationships))]))
}
//#endregion 🔖️Plumbing
