//! 🏅️ Conformance mutations and projection of the OOXML engine: the profile, the namespace and relationship rewrites, the arrange and inverse specs and the projection. See [`super`].

use super::*;

//#region 🔖️ConformanceMutations
/// 🏅️ One artifact's conformance-class coordinates. Each pair is `[transitional, strict]` — the
/// ISO/IEC 29500-4 value first, the ISO/IEC 29500-1 value second — which is what makes the class
/// stamp bijective and therefore exactly invertible.
#[derive(Clone, Copy, Debug)]
pub struct OoxmlProfile {
    pub format: &'static str,
    pub main_namespaces: [&'static str; 2],
    pub drawing_namespaces: Option<[&'static str; 2]>,
    pub relationship_namespaces: [&'static str; 2],
    pub relationship_bases: [&'static str; 2],
    pub vml_content_type: &'static str,
}

fn params_of(spec: &Json) -> Json {
    spec.get("params").cloned().unwrap_or(Json::Null)
}

pub(super) fn text(params: &Json, key: &str) -> String {
    match params.get(key) {
        Some(Json::String(value)) => value.clone(),
        _ => String::new(),
    }
}

/// 🧩️ The canonical legacy-VML part body the engine inserts when a mutation does not carry its
/// own `markup`. Real VML: the `urn:schemas-microsoft-com:vml` namespace on a `v:shape`, which is
/// exactly what the ✳️strict conformance checkers scan a part's bytes for.
pub const VML_MARKUP: &str = "<xml xmlns:v=\"urn:schemas-microsoft-com:vml\"><v:shape id=\"legacyShape\" type=\"#_x0000_t202\"/></xml>";

/// 🧩️ The canonical markup-compatibility fragment the engine appends when a mutation does not
/// carry its own `markup`.
pub const ALTERNATE_CONTENT_MARKUP: &str = "<mc:AlternateContent xmlns:mc=\"http://schemas.openxmlformats.org/markup-compatibility/2006\"><mc:Fallback/></mc:AlternateContent>";

fn markup_or_default(params: &Json, fallback: &str) -> String {
    let carried = text(params, "markup");
    if carried.is_empty() {
        fallback.to_string()
    } else {
        carried
    }
}

fn rewrite_namespaces(parts: &mut Vec<(String, Vec<u8>)>, pair: &[&str; 2], to: &str) -> Result<(), String> {
    if to.is_empty() {
        return Err("namespace mutation carries no target namespace".to_string());
    }
    let targets: Vec<(String, Vec<u8>)> = parts.iter().filter(|(path, _)| is_xml_part(path)).cloned().collect();
    for (path, bytes) in targets {
        let rewritten = replace_attribute_values(&bytes, &pair[..], to).map_err(|error| format!("{path}: {error}"))?;
        set_part(parts, &path, rewritten);
    }
    Ok(())
}

fn rewrite_relationship_bases(parts: &mut Vec<(String, Vec<u8>)>, pair: &[&str; 2], to: &str) -> Result<(), String> {
    if to.is_empty() {
        return Err("relationship-base mutation carries no target base".to_string());
    }
    let targets: Vec<(String, Vec<u8>)> = parts.iter().filter(|(path, _)| path.ends_with(".rels")).cloned().collect();
    for (path, bytes) in targets {
        let rewritten = replace_attribute_value_prefixes(&bytes, &pair[..], to).map_err(|error| format!("{path}: {error}"))?;
        set_part(parts, &path, rewritten);
    }
    Ok(())
}

/// 🏅️ Stamps the WHOLE package into one conformance class: both namespace families, the
/// `officeDocument` relationship base, and the main part's own `conformance` attribute. Bijective
/// by construction, so stamping back is an exact inverse.
pub fn stamp_conformance_class(parts: &mut Vec<(String, Vec<u8>)>, profile: &OoxmlProfile, strict: bool) -> Result<(), String> {
    let index = usize::from(strict);
    rewrite_namespaces(parts, &profile.main_namespaces, profile.main_namespaces[index])?;
    if let Some(pair) = profile.drawing_namespaces {
        rewrite_namespaces(parts, &pair, pair[index])?;
    }
    rewrite_namespaces(parts, &profile.relationship_namespaces, profile.relationship_namespaces[index])?;
    rewrite_relationship_bases(parts, &profile.relationship_bases, profile.relationship_bases[index])?;
    let main = main_part(parts)?;
    let bytes = part_bytes(parts, &main).ok_or_else(|| format!("main part {main} is declared but absent"))?.to_vec();
    let stamped = set_root_attribute(&bytes, "conformance", if strict { Some("strict") } else { None })?;
    set_part(parts, &main, stamped);
    Ok(())
}

/// 🦠️ Performs one declared conformance-class mutation with the independent `zip` + `quick-xml`
/// implementation and returns the re-serialized package. An unrecognised kind is an error, never
/// a silent no-op: a mutation that is quietly skipped reports as a passing test.
pub fn apply_conformance_mutation(input: &[u8], spec: &Json, profile: &OoxmlProfile) -> Result<Vec<u8>, String> {
    let params = params_of(spec);
    let kind = spec.str("kind");
    if kind.is_empty() {
        return Err("mutation spec carries no `kind`".to_string());
    }
    let mut parts = read_parts(input)?;
    match kind.as_str() {
        "set-main-namespace" => rewrite_namespaces(&mut parts, &profile.main_namespaces, &text(&params, "namespace"))?,
        "set-drawing-namespace" => {
            let pair = profile.drawing_namespaces.ok_or("set-drawing-namespace: this artifact declares no DrawingML namespace pair")?;
            rewrite_namespaces(&mut parts, &pair, &text(&params, "namespace"))?;
        }
        "set-relationships-namespace" => rewrite_namespaces(&mut parts, &profile.relationship_namespaces, &text(&params, "namespace"))?,
        "set-relationship-base" => rewrite_relationship_bases(&mut parts, &profile.relationship_bases, &text(&params, "base"))?,
        "set-conformance-attribute" | "remove-conformance-attribute" => {
            let main = main_part(&parts)?;
            let bytes = part_bytes(&parts, &main).ok_or_else(|| format!("main part {main} is declared but absent"))?.to_vec();
            let value = text(&params, "value");
            let stamped = set_root_attribute(&bytes, "conformance", if kind == "set-conformance-attribute" { Some(value.as_str()) } else { None })?;
            if kind == "remove-conformance-attribute" && stamped == bytes {
                return Err(format!("remove-conformance-attribute: {main} declares no conformance attribute to remove"));
            }
            set_part(&mut parts, &main, stamped);
        }
        "insert-vml-part" => {
            let path = text(&params, "path");
            if part_bytes(&parts, &path).is_some() {
                return Err(format!("insert-vml-part: {path} already exists"));
            }
            let document = params.get("document").ok_or_else(|| "insert-vml-part carries no owned XML document".to_string())?;
            set_part(&mut parts, &path, xml_document_text(document)?.into_bytes());
            set_content_type_override(&mut parts, &path, Some(profile.vml_content_type))?;
        }
        "remove-vml-part" => {
            let path = text(&params, "path");
            if !remove_part(&mut parts, &path) {
                return Err(format!("remove-vml-part: {path} is not in the package"));
            }
            set_content_type_override(&mut parts, &path, None)?;
        }
        "insert-alternate-content" => {
            let path = text(&params, "path");
            let bytes = part_bytes(&parts, &path).ok_or_else(|| format!("insert-alternate-content: {path} is not in the package"))?.to_vec();
            let appended = append_root_child(&bytes, &markup_or_default(&params, ALTERNATE_CONTENT_MARKUP))?;
            set_part(&mut parts, &path, appended);
        }
        "remove-alternate-content" => {
            let path = text(&params, "path");
            let bytes = part_bytes(&parts, &path).ok_or_else(|| format!("remove-alternate-content: {path} is not in the package"))?.to_vec();
            let stripped = remove_root_children(&bytes, "mc:AlternateContent")?;
            set_part(&mut parts, &path, stripped);
        }
        "set-worksheet-content-type" => {
            let path = text(&params, "path");
            if part_bytes(&parts, &path).is_none() {
                return Err(format!("set-worksheet-content-type: {path} is not in the package"));
            }
            set_content_type_override(&mut parts, &path, Some(&text(&params, "content_type")))?;
        }
        other => return Err(format!("mutation kind {other:?} has no oracle implementation ({} input byte(s))", input.len())),
    }
    write_parts(&parts)
}

/// 🔁️ The reference implementation's own decode/re-encode: read every entry with `zip`, rebuild
/// the container from those entries alone. Proves the independent container codec is stable on
/// the real package before the subject's own codec is asked to be.
pub fn round_trip(input: &[u8]) -> Result<Vec<u8>, String> {
    write_parts(&read_parts(input)?)
}

fn json_object(pairs: Vec<(&str, Json)>) -> Json {
    Json::Object(pairs.into_iter().map(|(key, value)| (key.to_string(), value)).collect())
}

fn kind_spec(kind: &str, pairs: Vec<(&str, Json)>) -> Json {
    json_object(vec![("kind", Json::String(kind.to_string())), ("params", json_object(pairs))])
}

/// 🔎️ Which member of a `[transitional, strict]` namespace pair the package actually declares.
fn declared_pair_member(parts: &[(String, Vec<u8>)], pair: &[&str; 2]) -> Result<String, String> {
    let declared = declared_namespaces(parts)?;
    pair.iter().find(|candidate| declared.iter().any(|value| value == *candidate)).map(|found| found.to_string()).ok_or_else(|| format!("package declares neither {} nor {}", pair[0], pair[1]))
}

/// 🔎️ Which member of a `[transitional, strict]` relationship-base pair the package's own
/// relationship types are built on.
fn declared_pair_base(parts: &[(String, Vec<u8>)], pair: &[&str; 2]) -> Result<String, String> {
    let types = relationship_types(parts)?;
    pair.iter().find(|candidate| types.iter().any(|value| value.starts_with(*candidate))).map(|found| found.to_string()).ok_or_else(|| format!("package declares no relationship type under {} or {}", pair[0], pair[1]))
}

/// 🎬️ Prepares the input a REMOVAL kind needs a target in. Not one of this repository's real
/// ECMA-376 packages carries VML, `mc:AlternateContent` or a `conformance` attribute — a fact
/// about the corpus, recorded here rather than papered over — so the three removal kinds are
/// exercised on the real package after the SAME independent implementation has inserted their
/// target. Every other kind reads the committed bytes untouched.
pub fn conformance_arrange(input: &[u8], forward: &Json, profile: &OoxmlProfile) -> Result<Vec<u8>, String> {
    let params = params_of(forward);
    let path = || Json::String(text(&params, "path"));
    match forward.str("kind").as_str() {
        "remove-conformance-attribute" => apply_conformance_mutation(input, &kind_spec("set-conformance-attribute", vec![("value", Json::String("strict".to_string()))]), profile),
        "remove-vml-part" => apply_conformance_mutation(input, &kind_spec("insert-vml-part", vec![("path", path()), ("document", vml_document())]), profile),
        "remove-alternate-content" => apply_conformance_mutation(input, &kind_spec("insert-alternate-content", vec![("path", path())]), profile),
        _ => Ok(input.to_vec()),
    }
}

/// ↩️ The undo of `forward`, computed by reading whatever pre-mutation state it needs straight
/// out of `base` through the SAME independent implementation the mutation runs on — never by
/// calling this repository's own `Mutation::inverse`, which would defeat the point of an
/// independently computed oracle. `base` is the mutation's real pre-state, i.e. the output of
/// [`conformance_arrange`].
pub fn conformance_inverse_spec(base: &[u8], forward: &Json, profile: &OoxmlProfile) -> Result<Json, String> {
    let params = params_of(forward);
    let parts = read_parts(base)?;
    let path = || Json::String(text(&params, "path"));
    let conformance = || -> Result<Option<String>, String> {
        let main = main_part(&parts)?;
        let bytes = part_bytes(&parts, &main).ok_or_else(|| format!("main part {main} is declared but absent"))?;
        root_attribute(bytes, "conformance")
    };
    Ok(match forward.str("kind").as_str() {
        "set-main-namespace" => kind_spec("set-main-namespace", vec![("namespace", Json::String(declared_pair_member(&parts, &profile.main_namespaces)?))]),
        "set-drawing-namespace" => {
            let pair = profile.drawing_namespaces.ok_or("set-drawing-namespace: this artifact declares no DrawingML namespace pair")?;
            kind_spec("set-drawing-namespace", vec![("namespace", Json::String(declared_pair_member(&parts, &pair)?))])
        }
        "set-relationships-namespace" => kind_spec("set-relationships-namespace", vec![("namespace", Json::String(declared_pair_member(&parts, &profile.relationship_namespaces)?))]),
        "set-relationship-base" => kind_spec("set-relationship-base", vec![("base", Json::String(declared_pair_base(&parts, &profile.relationship_bases)?))]),
        "set-conformance-attribute" => match conformance()? {
            Some(value) => kind_spec("set-conformance-attribute", vec![("value", Json::String(value))]),
            None => kind_spec("remove-conformance-attribute", vec![]),
        },
        "remove-conformance-attribute" => match conformance()? {
            Some(value) => kind_spec("set-conformance-attribute", vec![("value", Json::String(value))]),
            None => return Err("remove-conformance-attribute has no inverse: the base declares no conformance attribute".to_string()),
        },
        "insert-vml-part" => kind_spec("remove-vml-part", vec![("path", path())]),
        "remove-vml-part" => {
            let target = text(&params, "path");
            let bytes = part_bytes(&parts, &target).ok_or_else(|| format!("remove-vml-part has no inverse: {target} is not in the base"))?;
            kind_spec("insert-vml-part", vec![("path", path()), ("document", wire_xml_document(bytes)?)])
        }
        "insert-alternate-content" => kind_spec("remove-alternate-content", vec![("path", path())]),
        "remove-alternate-content" => kind_spec("insert-alternate-content", vec![("path", path())]),
        "set-worksheet-content-type" => {
            let target = text(&params, "path");
            let (defaults, overrides) = content_types(&parts)?;
            kind_spec("set-worksheet-content-type", vec![("path", Json::String(target.clone())), ("content_type", Json::String(resolve_content_type(&defaults, &overrides, &target)))])
        }
        other => return Err(format!("no inverse rule for kind {other:?}")),
    })
}
//#endregion 🔖️ConformanceMutations

//#region 🔖️Projection
/// 👁️ The conformance-class projection every `✳️strict`/`✳️transitional` OOXML subset is
/// compared through: the package's part inventory with resolved content types, the main part's
/// own root element and attributes, every declared OOXML namespace, every relationship type, and
/// the legacy-markup inventory. Everything is read back out of the BYTES by this independent
/// implementation — nothing is carried by the caller.
pub fn project(input: &[u8], format: &str) -> Result<Json, String> {
    let parts = read_parts(input)?;
    let (defaults, overrides) = content_types(&parts)?;
    let main = main_part(&parts)?;
    let main_bytes = part_bytes(&parts, &main).ok_or_else(|| format!("main part {main} is declared by _rels/.rels but absent from the package"))?;
    let (root_name, mut root_attrs) = root_element(main_bytes)?;
    root_attrs.sort();
    let mut inventory: Vec<(String, String)> = parts.iter().map(|(path, _)| (path.clone(), resolve_content_type(&defaults, &overrides, path))).collect();
    inventory.sort();
    let namespaces = declared_namespaces(&parts)?;
    let relationship_types = relationship_types(&parts)?;
    let alternate_content = alternate_content_parts(&parts);
    let strings = |values: Vec<String>| Json::Array(values.into_iter().map(Json::String).collect());
    Ok(Json::Object(vec![
        ("format".to_string(), Json::String(format.to_string())),
        ("mainPart".to_string(), Json::String(main)),
        ("mainRootName".to_string(), Json::String(root_name)),
        (
            "mainRootAttributes".to_string(),
            Json::Array(root_attrs.into_iter().map(|(name, value)| Json::Object(vec![("name".to_string(), Json::String(name)), ("value".to_string(), Json::String(value))])).collect()),
        ),
        (
            "parts".to_string(),
            Json::Array(inventory.into_iter().map(|(path, content_type)| Json::Object(vec![("path".to_string(), Json::String(path)), ("contentType".to_string(), Json::String(content_type))])).collect()),
        ),
        ("namespaces".to_string(), strings(namespaces)),
        ("relationshipTypes".to_string(), strings(relationship_types)),
        ("alternateContentParts".to_string(), strings(alternate_content)),
    ]))
}
//#endregion 🔖️Projection
