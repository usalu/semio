//! 🔎️ Profile, container and scans of the PDF engine: the conformance profile, the lopdf loading, the object-graph scans and the object builders. See [`super`].

use super::*;

//#region 🔖️Profile
/// 🏅️ One subset's conformance coordinates: which marker its OutputIntent axis demands, whether
/// that intent must carry a `/DestOutputProfile`, and exactly which axes its own checker reads.
/// `axes` is the projection's field list, so a subset never projects an axis its checker does not read.
pub struct PdfConformanceProfile {
    pub subset: &'static str,
    pub output_intent_subtype: &'static str,
    pub output_intent_dest_profile: bool,
    pub axes: &'static [&'static str],
}
//#endregion 🔖️Profile

//#region 🔖️Spec
pub(super) fn params_of(spec: &Json) -> Json {
    spec.get("params").cloned().unwrap_or(Json::Null)
}

pub(super) fn number(params: &Json, key: &str) -> Option<f64> {
    match params.get(key) {
        Some(Json::Number(value)) => Some(*value),
        _ => None,
    }
}

pub(super) fn ordinal(params: &Json, key: &str) -> Result<usize, String> {
    number(params, key).map(|value| value.max(0.0) as usize).ok_or_else(|| format!("parameter `{key}` is required and must be a number"))
}

pub(super) fn required(params: &Json, key: &str) -> Result<String, String> {
    match params.get(key) {
        Some(Json::String(value)) if !value.is_empty() => Ok(value.clone()),
        _ => Err(format!("parameter `{key}` is required and must be a non-empty string")),
    }
}

pub(super) fn json_object(pairs: Vec<(&str, Json)>) -> Json {
    Json::Object(pairs.into_iter().map(|(key, value)| (key.to_string(), value)).collect())
}

pub(super) fn kind_spec(kind: &str, pairs: Vec<(&str, Json)>) -> Json {
    json_object(vec![("kind", Json::String(kind.to_string())), ("params", json_object(pairs))])
}

pub(super) fn optional_string(value: Option<String>) -> Json {
    value.map(Json::String).unwrap_or(Json::Null)
}

pub(super) fn number_array(values: &[f32]) -> Json {
    Json::Array(values.iter().map(|value| Json::Number(*value as f64)).collect())
}
//#endregion 🔖️Spec

//#region 🔖️Container
pub(super) fn load(input: &[u8]) -> Result<Document, String> {
    Document::load_mem(input).map_err(|error| format!("independent PDF reader could not parse the document: {error}"))
}

pub(super) fn save(document: &mut Document) -> Result<Vec<u8>, String> {
    let mut out = Vec::new();
    document.save_to(&mut out).map_err(|error| format!("independent PDF writer could not save: {error}"))?;
    Ok(out)
}

/// 🔁️ The reference implementation's own decode/re-encode: `lopdf` parses the whole object
/// graph and writes a fresh file from that graph alone.
pub fn round_trip(input: &[u8]) -> Result<Vec<u8>, String> {
    save(&mut load(input)?)
}

fn catalog_id(document: &Document) -> Result<ObjectId, String> {
    document.trailer.get(b"Root").and_then(Object::as_reference).map_err(|error| format!("the trailer carries no /Root reference: {error}"))
}

pub(super) fn catalog_dict(document: &Document) -> Result<&Dictionary, String> {
    let id = catalog_id(document)?;
    document.get_dictionary(id).map_err(|error| format!("/Root does not resolve to a dictionary: {error}"))
}

pub(super) fn catalog_dict_mut(document: &mut Document) -> Result<&mut Dictionary, String> {
    let id = catalog_id(document)?;
    document.get_dictionary_mut(id).map_err(|error| format!("/Root does not resolve to a dictionary: {error}"))
}

fn name_at(dict: &Dictionary, key: &str) -> Option<String> {
    dict.get(key.as_bytes()).ok().and_then(|value| value.as_name().ok()).map(|bytes| String::from_utf8_lossy(bytes).into_owned())
}

pub(super) fn text_at(dict: &Dictionary, key: &str) -> Option<String> {
    dict.get(key.as_bytes()).ok().and_then(|value| value.as_str().ok()).map(|bytes| String::from_utf8_lossy(bytes).into_owned())
}

fn deref<'d>(document: &'d Document, value: &'d Object) -> Option<&'d Object> {
    match value {
        Object::Reference(id) => document.get_object(*id).ok(),
        other => Some(other),
    }
}

pub(super) fn deref_dict<'d>(document: &'d Document, value: &'d Object) -> Option<&'d Dictionary> {
    deref(document, value).and_then(|resolved| resolved.as_dict().ok())
}
//#endregion 🔖️Container

//#region 🔖️Scans
/// 🔒️ Every object with the Standard Security Handler shape the conformance checkers scan for.
pub(super) fn encryption_dictionaries(document: &Document) -> Vec<(ObjectId, i64, i64)> {
    document
        .objects
        .iter()
        .filter_map(|(id, object)| {
            let dict = object.as_dict().ok()?;
            if name_at(dict, "Filter").as_deref() != Some("Standard") {
                return None;
            }
            let version = dict.get(b"V").ok()?.as_i64().ok()?;
            let revision = dict.get(b"R").ok()?.as_i64().ok()?;
            if !dict.has(b"O") || !dict.has(b"U") {
                return None;
            }
            Some((*id, version, revision))
        })
        .collect()
}

/// 📜️ Every action dictionary with the given `/S` subtype, paired with the payload the subset's
/// own diagnostics quote — `/JS` for JavaScript, `/F` for Launch.
pub(super) fn actions(document: &Document, subtype: &str, payload_key: &str) -> Vec<(ObjectId, String)> {
    document
        .objects
        .iter()
        .filter_map(|(id, object)| {
            let dict = object.as_dict().ok()?;
            if name_at(dict, "S").as_deref() != Some(subtype) {
                return None;
            }
            Some((*id, text_at(dict, payload_key).unwrap_or_default()))
        })
        .collect()
}

/// 🎬️ Every `/Subtype /Movie` or `/Subtype /Sound` annotation. `/Subtype /3D` is a distinct,
/// explicitly allowed name and never matches — the same distinction `check_e_conformance` draws.
pub(super) fn media_annotations(document: &Document) -> Vec<(ObjectId, String, String)> {
    document
        .objects
        .iter()
        .filter_map(|(id, object)| {
            let dict = object.as_dict().ok()?;
            let subtype = name_at(dict, "Subtype")?;
            if subtype != "Movie" && subtype != "Sound" {
                return None;
            }
            Some((*id, subtype, text_at(dict, "T").unwrap_or_default()))
        })
        .collect()
}

/// 📎️ Every `/Type /Filespec` object, with the file name, whether it carries an `/EF` attached
/// stream, and its `/AFRelationship` if any — the exact triple PDF/A's embedded-file axis reads.
pub(super) fn file_specs(document: &Document) -> Vec<(ObjectId, String, bool, Option<String>)> {
    document
        .objects
        .iter()
        .filter_map(|(id, object)| {
            let dict = object.as_dict().ok()?;
            if name_at(dict, "Type").as_deref() != Some("Filespec") {
                return None;
            }
            let file_name = text_at(dict, "F").or_else(|| text_at(dict, "UF")).unwrap_or_default();
            Some((*id, file_name, dict.has(b"EF"), name_at(dict, "AFRelationship")))
        })
        .collect()
}

/// 🏳️ Every intent reachable from `/Root/OutputIntents`, with its `/S` marker, its output
/// condition identifier and whether it carries a `/DestOutputProfile`.
pub(super) fn output_intents(document: &Document) -> Vec<(String, String, bool)> {
    let Ok(catalog) = catalog_dict(document) else { return Vec::new() };
    let Ok(intents) = catalog.get(b"OutputIntents") else { return Vec::new() };
    let Some(items) = deref(document, intents).and_then(|value| value.as_array().ok()) else { return Vec::new() };
    items
        .iter()
        .filter_map(|item| {
            let dict = deref_dict(document, item)?;
            Some((name_at(dict, "S").unwrap_or_default(), text_at(dict, "OutputConditionIdentifier").unwrap_or_default(), dict.has(b"DestOutputProfile")))
        })
        .collect()
}

/// 🌈️ Reads actual destination profile component counts and decompressed bytes through the independent PDF graph.
pub fn output_profiles(input: &[u8]) -> Result<Vec<(i64, Vec<u8>)>, String> {
    let document = load(input)?;
    let catalog = catalog_dict(&document)?;
    let intents = catalog.get(b"OutputIntents").map_err(|error| format!("no output intents: {error}"))?;
    let intents = deref(&document, intents).and_then(|value| value.as_array().ok()).ok_or_else(|| "output intents do not resolve to an array".to_string())?;
    let mut profiles = Vec::new();
    for intent in intents {
        let intent = deref_dict(&document, intent).ok_or_else(|| "output intent does not resolve to a dictionary".to_string())?;
        let profile = intent.get(b"DestOutputProfile").map_err(|error| format!("no destination profile: {error}"))?;
        let stream = deref(&document, profile).and_then(|value| value.as_stream().ok()).ok_or_else(|| "destination profile does not resolve to a stream".to_string())?;
        let components = stream.dict.get(b"N").and_then(Object::as_i64).map_err(|error| format!("profile component count is absent: {error}"))?;
        let bytes = stream.get_plain_content().map_err(|error| format!("profile stream decoding failed: {error}"))?;
        profiles.push((components, bytes));
    }
    Ok(profiles)
}

pub(super) const FONT_PROGRAM_KEYS: [&str; 3] = ["FontFile", "FontFile2", "FontFile3"];

/// 🔤️ Every `/Type /FontDescriptor` object, in object-number order — a stable ordinal space no
/// mutation in this vocabulary adds to or removes from, which is what lets a scenario address
/// one by ordinal instead of by an object number nobody can read off a feature file.
pub(super) fn font_descriptors(document: &Document) -> Vec<ObjectId> {
    document.objects.iter().filter(|(_, object)| object.as_dict().map(|dict| name_at(dict, "Type").as_deref() == Some("FontDescriptor")).unwrap_or(false)).map(|(id, _)| *id).collect()
}

/// 🔤️ The embedded font program a descriptor points at: which of the three keys carries it, the
/// object it references, and the DECODED program — its length and its digest.
///
/// 🔓️ Why decoded and not `stream.content.len()`. The normative object here is the font program,
/// not the stream that carries it. `/FontFile*` streams are almost always `/FlateDecode`, and
/// deflate output length is ENCODER FREEDOM: two conformant writers may compress a
/// byte-identical program to byte counts that differ by a byte or two, which is exactly what the
/// first full differential run of the six PDF 1.7 conformance classes observed — all 23 embedded
/// programs of the real thesis decoded byte-identically while five of their compressed streams
/// differed by 1–2 bytes, failing 188 comparisons including the identity round trip. The `✳️any` subset's
/// `semantic-pdf-v1` profile already committed to that reading by listing `streamLength` as
/// writer freedom. Measuring the decoded program restores what the profile description always
/// claimed to measure — "how many bytes that program is" — and the digest makes the axis
/// STRICTER than the compressed length ever was: a program whose bytes change is now caught even
/// when its length does not. A stream whose filter chain will not decode falls back to its
/// STORED bytes rather than to nothing: an undecodable program is still evidence, and erasing it
/// to an empty digest would make two unreadable programs compare equal.
pub(super) fn font_program(document: &Document, descriptor: ObjectId) -> Option<(String, ObjectId, usize, String)> {
    let dict = document.get_dictionary(descriptor).ok()?;
    for key in FONT_PROGRAM_KEYS {
        if let Ok(value) = dict.get(key.as_bytes()) {
            let id = value.as_reference().ok()?;
            let program = document.get_object(id).ok().and_then(|object| object.as_stream().ok()).map(|stream| stream.get_plain_content().unwrap_or_else(|_| stream.content.clone())).unwrap_or_default();
            return Some((key.to_string(), id, program.len(), digest(&program)));
        }
    }
    None
}

pub(super) fn page_at(document: &Document, index: usize) -> Result<ObjectId, String> {
    document.get_pages().get(&(index as u32 + 1)).copied().ok_or_else(|| format!("page index {index} is out of range"))
}

pub(super) fn box_of(document: &Document, page: ObjectId, key: &str) -> Option<Vec<f32>> {
    let dict = document.get_dictionary(page).ok()?;
    let value = dict.get(key.as_bytes()).ok()?;
    let items = deref(document, value)?.as_array().ok()?;
    Some(items.iter().map(|item| item.as_float().unwrap_or(0.0)).collect())
}

/// ✍️ Every `/AcroForm` field whose `/FT` is `/Sig`, by its `/T` title — the signature-flow axis
/// the PDF Healthcare guide's checker reads.
pub(super) fn signature_fields(document: &Document) -> Vec<(ObjectId, String)> {
    let Ok(catalog) = catalog_dict(document) else { return Vec::new() };
    let Ok(acro_form) = catalog.get(b"AcroForm") else { return Vec::new() };
    let Some(form) = deref_dict(document, acro_form) else { return Vec::new() };
    let Some(fields) = form.get(b"Fields").ok().and_then(|value| deref(document, value)).and_then(|value| value.as_array().ok()) else { return Vec::new() };
    fields
        .iter()
        .filter_map(|item| {
            let id = item.as_reference().ok()?;
            let dict = document.get_dictionary(id).ok()?;
            if name_at(dict, "FT").as_deref() != Some("Sig") {
                return None;
            }
            Some((id, text_at(dict, "T").unwrap_or_default()))
        })
        .collect()
}

/// 🗂️ The `/DPartRoot` node chain: its root node and, per node, whether it carries `/DPM` and
/// what that metadata's `/Job` entry says.
pub(super) fn dpart_nodes(document: &Document) -> Option<(ObjectId, Vec<(ObjectId, Option<String>)>)> {
    let catalog = catalog_dict(document).ok()?;
    let root_id = catalog.get(b"DPartRoot").ok()?.as_reference().ok()?;
    let root = document.get_dictionary(root_id).ok()?;
    let node_id = root.get(b"DPartRootNode").ok()?.as_reference().ok()?;
    let node = document.get_dictionary(node_id).ok()?;
    let metadata = node.get(b"DPM").ok().and_then(|value| deref_dict(document, value)).and_then(|dict| text_at(dict, "Job"));
    Some((root_id, vec![(node_id, metadata)]))
}

fn info_id(document: &Document) -> Option<ObjectId> {
    document.trailer.get(b"Info").ok().and_then(|value| value.as_reference().ok())
}

pub(super) fn info_entry(document: &Document, key: &str) -> Option<String> {
    let id = info_id(document)?;
    let dict = document.get_dictionary(id).ok()?;
    text_at(dict, key)
}

pub(super) fn set_info_entry(document: &mut Document, key: &str, value: &str) -> Result<(), String> {
    match info_id(document) {
        Some(id) => {
            let dict = document.get_dictionary_mut(id).map_err(|error| format!("/Info does not resolve to a dictionary: {error}"))?;
            dict.set(key, Object::string_literal(value));
        }
        None => {
            let mut dict = Dictionary::new();
            dict.set(key, Object::string_literal(value));
            let id = document.add_object(Object::Dictionary(dict));
            document.trailer.set("Info", Object::Reference(id));
        }
    }
    Ok(())
}
//#endregion 🔖️Scans

//#region 🔖️Builders
/// 🔒️ A real Standard Security Handler dictionary — the `/Filter`/`/V`/`/R`/`/O`/`/U` shape the
/// checkers match, with 32-byte owner and user strings of the length ISO 32000-1 §7.6.3.3 fixes.
pub(super) fn encryption_dictionary(version: i64, revision: i64) -> Object {
    let mut dict = Dictionary::new();
    dict.set("Filter", Object::Name(b"Standard".to_vec()));
    dict.set("V", Object::Integer(version));
    dict.set("R", Object::Integer(revision));
    dict.set("O", Object::string_literal(vec![0x4fu8; 32]));
    dict.set("U", Object::string_literal(vec![0x55u8; 32]));
    dict.set("P", Object::Integer(-1));
    dict.set("Length", Object::Integer(128));
    Object::Dictionary(dict)
}

pub(super) fn javascript_action(script: &str) -> Object {
    let mut dict = Dictionary::new();
    dict.set("Type", Object::Name(b"Action".to_vec()));
    dict.set("S", Object::Name(b"JavaScript".to_vec()));
    dict.set("JS", Object::string_literal(script));
    Object::Dictionary(dict)
}

pub(super) fn launch_action(target: &str) -> Object {
    let mut dict = Dictionary::new();
    dict.set("Type", Object::Name(b"Action".to_vec()));
    dict.set("S", Object::Name(b"Launch".to_vec()));
    dict.set("F", Object::string_literal(target));
    Object::Dictionary(dict)
}

pub(super) fn media_annotation(subtype: &str, title: &str) -> Object {
    let mut dict = Dictionary::new();
    dict.set("Type", Object::Name(b"Annot".to_vec()));
    dict.set("Subtype", Object::Name(subtype.as_bytes().to_vec()));
    dict.set("T", Object::string_literal(title));
    dict.set("Rect", Object::Array(vec![Object::Integer(0), Object::Integer(0), Object::Integer(144), Object::Integer(96)]));
    Object::Dictionary(dict)
}
//#endregion 🔖️Builders
