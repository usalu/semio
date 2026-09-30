//! 🔮️ Mutation oracle for this subset — every mutation kind the subset declares, performed by the
//! registered reference implementation so the subject's own mutation has an independent result to
//! be compared against instead of being checked against its own reading.
//!
//! The vocabulary is per SUBSET, not per artifact: two standards of the same format declare
//! different mutations, and a subset that shares an implementation with another reaches it through
//! the shared `document` module rather than by copying it.
//!
//! Two entry points: [`oracle_apply_mutation`] performs the FORWARD mutation (the `mutate-<kind>`
//! scenarios), [`oracle_apply_mutation_inverse`] performs the forward mutation and then its
//! computed inverse in sequence (the `inverse-<kind>` scenarios) — the same "apply, then apply the
//! inverse, land back on the start" law `PdfMutation::inverse` proves at the Rust-model level,
//! proven here independently against the registered reference library.
//!
//! @see ../🔣️oracle.json — the mutation catalog this module is measured against.
//! @see ../🧬️schema/🧬️mutations/🦀️.rs — the mutation vocabulary itself (`PdfMutation::KINDS`).

use semio_repo_test_host::Json;

//#region 🔖️Vocabulary
/// 🧾️ Kebab-case spelling of every variant this subset's `PdfMutation` declares, in declaration
/// order. The `pdf-1-7-base` catalog is measured against this exact list, and the production-side
/// `KINDS` carries `kinds_const_matches_enum_variants_in_declaration_order`, which proves enum,
/// constant and manifest never drift apart. Declared here rather than in the case adapter so the
/// adapter, this module's own tests and any future host all read ONE list.
pub const KINDS: &[&str] = &["insert-page", "remove-page", "set-page-media-box", "set-page-crop-box", "append-page-content", "set-info", "insert-object", "remove-object", "set-object-value", "set-dict-entry", "remove-dict-entry", "set-trailer-entry", "remove-trailer-entry", "move-page", "set-page-content", "set-page-rotation"];

/// 👁️ The ONE declared kind whose forward effect no semantic projection of a PDF can carry, with
/// the reason and the fix.
///
/// `InsertObject { id, value }` adds an indirect object and links it to nothing. ISO 32000-1 §7.5.4
/// makes a conforming reader reach objects only by following references from the trailer's `/Root`
/// and `/Info`, so an object nothing references is unreachable and changes nothing readable: page
/// count, page geometry, page content, metadata and the whole resolved object graph all stay where
/// they were. This is not a thin projection — it was measured on the real thesis, which carries
/// 3,173 objects, 3,173 references and ZERO orphans and ZERO dangling references, so there is no
/// id in the file at which an insertion could land somewhere already pointed at. The vocabulary is
/// what cannot express it: `InsertObject` carries no reference site, and only `SetDictEntry` can
/// create one. Widening it to carry the linking site (or requiring the pair) is the fix, and it
/// belongs to whoever owns `../🧬️schema/🧬️mutations/🦀️.rs`. Its INVERSE is still under the
/// full law, and so is every other kind — this exempts one kind from one law, not from the case.
pub const UNOBSERVABLE: &[&str] = &["insert-object"];
//#endregion 🔖️Vocabulary

//#region 🔖️PageContentLaw
/// 🧱️ The three kinds whose REFERENCE undo has to rebuild a page's content stream, and therefore
/// cannot restore `contentOperators`. The vocabulary itself carries the typed operator list
/// (`PdfPage.content`, `InsertPage`/`SetPageContent`/`AppendPageContent` all speak `PdfOp`), so the
/// subject restores a page's original stream exactly; the loss is this reference's own undo, which
/// captures a page's prior text through `Tj` alone ([`page_text`]) and rebuilds a minimal
/// `BT /F1 12 Tf 72 720 Td (…) Tj ET` from it. Page 8 of the real thesis carries 294 operators —
/// glyph positioning, graphics state, the lot — set with `TJ`, and no `Tj` capture can bring them
/// back.
///
/// ⚖️ Exactly ONE axis is exempted, and only for these three kinds. `version`, `pageCount`, every
/// page's `mediaBox`, `cropBox`, `rotate` and — critically — the shown `text` all stay under the
/// full law, as does the whole `objectGraph` surface. Lives here
/// rather than in the case adapter because the adapter's `inverse-<kind>` handler and this module's
/// own `every_declared_kind_is_observable_and_its_inverse_restores_the_document` must exempt the
/// same axis for the same three kinds or one of them is measuring a different law.
pub fn regenerates_page_content(kind: &str) -> bool {
    matches!(kind, "remove-page" | "append-page-content" | "set-page-content")
}

/// ✂️ The same projection with every page's `contentOperators` dropped — nothing else is touched,
/// so a divergence anywhere else still fails. @see [`regenerates_page_content`].
pub fn without_content_operators(projection: &Json) -> Json {
    let Json::Object(fields) = projection else { return projection.clone() };
    Json::Object(
        fields
            .iter()
            .map(|(key, value)| {
                if key != "pages" {
                    return (key.clone(), value.clone());
                }
                let Json::Array(pages) = value else { return (key.clone(), value.clone()) };
                let stripped = pages
                    .iter()
                    .map(|page| match page {
                        Json::Object(entries) => Json::Object(entries.iter().filter(|(name, _)| name != "contentOperators").cloned().collect()),
                        other => other.clone(),
                    })
                    .collect();
                (key.clone(), Json::Array(stripped))
            })
            .collect(),
    )
}
//#endregion 🔖️PageContentLaw

#[cfg(feature = "oracles")]
//#region 🔖️Oracles
mod oracles {
    use crate::document::{self, oracle_delete_page, oracle_replace_metadata};
    use lopdf::content::{Content, Operation};
    use lopdf::{Dictionary, Document, Object, ObjectId, Stream, StringFormat};
    use semio_repo_test_host::Json;

    //#region 🔖️JsonValue
    /// 🔎️ The leaf wire's `PdfObject` — `{"kind": "null"|"bool"|"int"|"real"|"str"|"name"|"array"|"dict"|"ref"|"stream", …}`
    /// with newtype payloads under `value` and the `real`/`ref` records flattened — read into `lopdf`'s own object. It is
    /// the very JSON the subject decodes (this crate never depends on the production crate that owns `PdfObject`), so a
    /// row means the same thing on both sides. A stream is only accepted with its logical filter chain empty.
    fn json_to_object(value: &Json) -> Result<Object, String> {
        Ok(match value.str("kind").as_str() {
            "null" => Object::Null,
            "bool" => Object::Boolean(matches!(value.get("value"), Some(Json::Bool(true)))),
            "int" => Object::Integer(number_field(value, "value") as i64),
            "real" => Object::Real(decimal_to_f32(value)?),
            "str" => Object::String(byte_array(value.get("value"))?, StringFormat::Literal),
            "name" => Object::Name(value.str("value").into_bytes()),
            "array" => Object::Array(value.array("value").iter().map(json_to_object).collect::<Result<_, _>>()?),
            "dict" => Object::Dictionary(json_to_dictionary(&value.array("value"))?),
            "ref" => Object::Reference(json_object_id(value)),
            "stream" if value.array("filters").is_empty() => Object::Stream(Stream::new(json_to_dictionary(&value.array("dict"))?, byte_array(value.get("data"))?)),
            "stream" => return Err("a stream with a logical filter chain is outside this oracle's object grammar".to_string()),
            other => return Err(format!("{other:?} is not a PdfObject kind")),
        })
    }

    fn json_to_dictionary(entries: &[Json]) -> Result<Dictionary, String> {
        let mut dict = Dictionary::new();
        for entry in entries {
            dict.set(entry.str("key"), json_to_object(entry.get("value").unwrap_or(&Json::Null))?);
        }
        Ok(dict)
    }

    /// 🔢️ A `PdfDecimal` wire record (`negative`, decimal `coefficient` digits, `scale`) as the `f32` `lopdf` stores.
    fn decimal_to_f32(value: &Json) -> Result<f32, String> {
        let digits = value.str("coefficient");
        let scale = number_field(value, "scale") as usize;
        let padded = format!("{digits:0>width$}", width = scale + 1);
        let (integer, fraction) = padded.split_at(padded.len() - scale);
        let sign = if matches!(value.get("negative"), Some(Json::Bool(true))) { "-" } else { "" };
        format!("{sign}{integer}.{fraction}").parse().map_err(|error| format!("PdfDecimal {digits:?}/{scale} is not a number: {error}"))
    }

    /// 🔢️ The `PdfDecimal` wire record of a stored `f32`, the inverse of [`decimal_to_f32`].
    fn f32_to_decimal(value: f32) -> Vec<(String, Json)> {
        let text = format!("{}", value.abs());
        let (integer, fraction) = text.split_once('.').unwrap_or((&text, ""));
        vec![("negative".to_string(), Json::Bool(value.is_sign_negative() && value != 0.0)), ("coefficient".to_string(), Json::String(format!("{integer}{fraction}"))), ("scale".to_string(), Json::Number(fraction.len() as f64))]
    }

    /// 🧮️ A wire byte array (`[0..=255, …]`).
    fn byte_array(value: Option<&Json>) -> Result<Vec<u8>, String> {
        match value {
            Some(Json::Array(items)) => items.iter().map(|item| match item {
                Json::Number(byte) if (0.0..=255.0).contains(byte) && byte.fract() == 0.0 => Ok(*byte as u8),
                other => Err(format!("{other:?} is not a byte")),
            }).collect(),
            other => Err(format!("{other:?} is not a byte array")),
        }
    }

    fn bytes_to_json(bytes: &[u8]) -> Json {
        Json::Array(bytes.iter().map(|byte| Json::Number(*byte as f64)).collect())
    }

    fn tagged(kind: &str, members: Vec<(String, Json)>) -> Json {
        Json::Object([("kind".to_string(), Json::String(kind.to_string()))].into_iter().chain(members).collect())
    }

    /// 🔁️ The reverse of [`json_to_object`] — captures an object's CURRENT value in the same wire before a mutation
    /// touches it, so the computed undo hands that exact value back through [`json_to_object`].
    fn object_to_json(object: &Object) -> Json {
        let entries = |dict: &Dictionary| Json::Array(dict.iter().map(|(key, value)| Json::Object(vec![("key".to_string(), Json::String(String::from_utf8_lossy(key).to_string())), ("value".to_string(), object_to_json(value))])).collect());
        match object {
            Object::Null => tagged("null", vec![]),
            Object::Boolean(value) => tagged("bool", vec![("value".to_string(), Json::Bool(*value))]),
            Object::Integer(value) => tagged("int", vec![("value".to_string(), Json::Number(*value as f64))]),
            Object::Real(value) => tagged("real", f32_to_decimal(*value)),
            Object::String(bytes, _) => tagged("str", vec![("value".to_string(), bytes_to_json(bytes))]),
            Object::Name(bytes) => tagged("name", vec![("value".to_string(), Json::String(String::from_utf8_lossy(bytes).to_string()))]),
            Object::Array(items) => tagged("array", vec![("value".to_string(), Json::Array(items.iter().map(object_to_json).collect()))]),
            Object::Dictionary(dict) => tagged("dict", vec![("value".to_string(), entries(dict))]),
            Object::Stream(stream) => tagged("stream", vec![("dict".to_string(), entries(&stream.dict)), ("data".to_string(), bytes_to_json(&stream.content)), ("filters".to_string(), Json::Array(vec![]))]),
            Object::Reference(id) => tagged("ref", vec![("num".to_string(), Json::Number(id.0 as f64)), ("gen".to_string(), Json::Number(id.1 as f64))]),
        }
    }

    fn number_field(value: &Json, key: &str) -> f64 {
        match value.get(key) {
            Some(Json::Number(number)) => *number,
            _ => 0.0,
        }
    }

    fn usize_field(value: &Json, key: &str) -> usize {
        number_field(value, key).max(0.0) as usize
    }

    fn json_object_id(value: &Json) -> ObjectId {
        let id = value.get("id").cloned().unwrap_or(value.clone());
        (number_field(&id, "num") as u32, number_field(&id, "gen") as u16)
    }

    fn media_box_field(value: &Json, key: &str) -> Option<[f32; 4]> {
        match value.get(key) {
            Some(Json::Array(items)) if items.len() == 4 => {
                let n: Vec<f32> = items
                    .iter()
                    .map(|item| match item {
                        Json::Number(number) => *number as f32,
                        _ => 0.0,
                    })
                    .collect();
                Some([n[0], n[1], n[2], n[3]])
            }
            _ => None,
        }
    }

    fn number_array(values: &[f32]) -> Json {
        Json::Array(values.iter().map(|value| Json::Number(*value as f64)).collect())
    }

    fn object(entries: Vec<(&str, Json)>) -> Json {
        Json::Object(entries.into_iter().map(|(key, value)| (key.to_string(), value)).collect())
    }

    /// 🚧️ Refuses a wire record member this oracle does not reproduce, so a row can never pass on a field nobody applied.
    fn only_members(value: &Json, allowed: &[&str], what: &str) -> Result<(), String> {
        match value {
            Json::Object(entries) => entries.iter().find(|(key, _)| !allowed.contains(&key.as_str())).map_or(Ok(()), |(key, _)| Err(format!("{what} member {key:?} is outside this oracle's reference implementation"))),
            other => Err(format!("{what} must be an object, got {other:?}")),
        }
    }
    //#endregion 🔖️JsonValue

    //#region 🔖️ContentStream
    /// ✏️️ The leaf wire's `PdfOp` list (`{"op": "beginText"|"setFont"|"moveText"|"showText"|"endText", …}`) written as a
    /// content stream by `lopdf`'s own encoder — the text-object operators the case's rows and this module's undo capture
    /// speak. Any other operator is refused rather than skipped.
    fn content_stream(ops: &[Json]) -> Result<Vec<u8>, String> {
        let operations = ops
            .iter()
            .map(|op| {
                Ok(match op.str("op").as_str() {
                    "beginText" => Operation::new("BT", vec![]),
                    "endText" => Operation::new("ET", vec![]),
                    "setFont" => Operation::new("Tf", vec![Object::Name(op.str("name").into_bytes()), Object::Real(number_field(op, "size") as f32)]),
                    "moveText" => Operation::new("Td", vec![Object::Real(number_field(op, "tx") as f32), Object::Real(number_field(op, "ty") as f32)]),
                    "showText" => Operation::new("Tj", vec![text_operand(op.get("text").unwrap_or(&Json::Null))?]),
                    other => return Err(format!("content operator {other:?} is outside this oracle's text-object vocabulary")),
                })
            })
            .collect::<Result<Vec<_>, String>>()?;
        Content { operations }.encode().map_err(|error| format!("lopdf could not encode the content stream: {error}"))
    }

    /// 🔤️ A `PdfTextString` wire operand (`{"kind": "text", "text"}` or `{"kind": "codes", "bytes"}`) as a literal string.
    fn text_operand(text: &Json) -> Result<Object, String> {
        match text.str("kind").as_str() {
            "text" => Ok(Object::string_literal(text.str("text"))),
            "codes" => Ok(Object::String(byte_array(text.get("bytes"))?, StringFormat::Literal)),
            other => Err(format!("{other:?} is not a PdfTextString kind")),
        }
    }

    /// ✏️️ The minimal `BT /F1 12 Tf 72 720 Td (…) Tj ET` op list carrying `text` as one `Tj` — the reference's own lossy
    /// rebuild of a page from its captured `Tj` text. A page with NO extractable text gets a bare `BT ET`: the real
    /// thesis sets its type with `TJ`, so [`page_text`] extracts nothing from it, and a `() Tj` would turn a page the
    /// independent reader projects as `text: []` into one it projects as `text: [""]`.
    fn text_ops(text: &str) -> Json {
        let op = |name: &str, members: Vec<(&str, Json)>| object([("op", Json::String(name.to_string()))].into_iter().chain(members).collect());
        if text.is_empty() {
            return Json::Array(vec![op("beginText", vec![]), op("endText", vec![])]);
        }
        Json::Array(vec![
            op("beginText", vec![]),
            op("setFont", vec![("name", Json::String("F1".to_string())), ("size", Json::Number(12.0))]),
            op("moveText", vec![("tx", Json::Number(72.0)), ("ty", Json::Number(720.0))]),
            op("showText", vec![("text", object(vec![("kind", Json::String("text".to_string())), ("text", Json::String(text.to_string()))]))]),
            op("endText", vec![]),
        ])
    }

    /// 🔎️ Concatenated `Tj` operand text of one page's content -- the independent-reader counterpart
    /// of what [`text_ops`] writes, used to capture a page's prior text before mutating it.
    fn page_text(document: &Document, page_id: ObjectId) -> String {
        let content = document.get_page_content(page_id);
        let tj_operand_separator = "
";
        Content::decode(&content)
            .map(|decoded| {
                decoded
                    .operations
                    .iter()
                    .filter(|operation| operation.operator == "Tj")
                    .flat_map(|operation| operation.operands.iter())
                    .filter_map(|operand| operand.as_str().ok().map(|bytes| String::from_utf8_lossy(bytes).to_string()))
                    .collect::<Vec<_>>()
                    .join(tj_operand_separator)
            })
            .unwrap_or_default()
    }
    //#endregion 🔖️ContentStream

    //#region 🔖️PageTree
    fn page_id_at(document: &Document, index: usize) -> Option<ObjectId> {
        document.get_pages().get(&(index as u32 + 1)).copied()
    }

    fn pages_tree_id(document: &Document) -> Result<ObjectId, String> {
        document.catalog().map_err(|error| error.to_string())?.get(b"Pages").and_then(Object::as_reference).map_err(|error| error.to_string())
    }

    /// 🔀️ Flattens the WHOLE page order to `order`, one level directly under the top `/Pages` node
    /// -- our own snapshot model has no tree structure to preserve either (`PdfSnapshot::pages` is
    /// flat), so a real reorder/insert is expressed the same way here: every leaf's `/Parent` is
    /// repointed at the top node and its `/Kids`/`/Count` rewritten. Real, whole-document surgery,
    /// not a token edit — matches `PptxMutation::MoveSlide`'s own precedent of composing a reorder
    /// from `removed`+`added` rather than a dedicated "move" primitive.
    fn reorder_pages(document: &mut Document, order: &[ObjectId]) -> Result<(), String> {
        let tree_id = pages_tree_id(document)?;
        for &kid in order {
            if let Ok(dict) = document.get_object_mut(kid).and_then(Object::as_dict_mut) {
                dict.set("Parent", Object::Reference(tree_id));
            }
        }
        let tree = document.get_object_mut(tree_id).and_then(Object::as_dict_mut).map_err(|error| error.to_string())?;
        tree.set("Kids", Object::Array(order.iter().map(|id| Object::Reference(*id)).collect()));
        tree.set("Count", Object::Integer(order.len() as i64));
        Ok(())
    }
    //#endregion 🔖️PageTree

    //#region 🔖️PathAddressing
    /// 🔎️ Immutable walk of `path` (the wire's `PdfPathSegment` steps `{"kind":"arrayIndex","index":N}` /
    /// `{"kind":"dictKey","key":"K"}`)
    /// from object `id`'s own value down to the dict/stream-dict the leaf `key` lives in.
    fn navigate<'d>(document: &'d Document, id: ObjectId, path: &[Json]) -> Option<&'d Dictionary> {
        let mut current = document.get_object(id).ok()?;
        for segment in path {
            current = match (segment.str("kind").as_str(), current) {
                ("arrayIndex", Object::Array(items)) => items.get(usize_field(segment, "index"))?,
                ("dictKey", Object::Dictionary(dict)) => dict.get(segment.str("key").as_bytes()).ok()?,
                ("dictKey", Object::Stream(stream)) => stream.dict.get(segment.str("key").as_bytes()).ok()?,
                _ => return None,
            };
        }
        match current {
            Object::Dictionary(dict) => Some(dict),
            Object::Stream(stream) => Some(&stream.dict),
            _ => None,
        }
    }

    /// 🔧️ Mutable counterpart of [`navigate`].
    fn navigate_mut<'d>(document: &'d mut Document, id: ObjectId, path: &[Json]) -> Option<&'d mut Dictionary> {
        let mut current = document.get_object_mut(id).ok()?;
        for segment in path {
            current = match (segment.str("kind").as_str(), current) {
                ("arrayIndex", Object::Array(items)) => items.get_mut(usize_field(segment, "index"))?,
                ("dictKey", Object::Dictionary(dict)) => dict.get_mut(segment.str("key").as_bytes()).ok()?,
                ("dictKey", Object::Stream(stream)) => stream.dict.get_mut(segment.str("key").as_bytes()).ok()?,
                _ => return None,
            };
        }
        match current {
            Object::Dictionary(dict) => Some(dict),
            Object::Stream(stream) => Some(&mut stream.dict),
            _ => None,
        }
    }
    //#endregion 🔖️PathAddressing

    //#region 🔖️Forward
    /// ▶️ The 15 kinds NOT already covered by the shared `document` module's own
    /// `oracle_delete_page`/`oracle_replace_metadata` (see [`super::oracle_apply_mutation`]'s own
    /// routing) -- mutates `document` in place. Out-of-range indices / missing ids / unresolvable
    /// paths are no-ops, mirroring `apply_pdf_mutation`'s own "never panic on a stale reference"
    /// contract at the Rust-model level.
    fn apply_kind(document: &mut Document, kind: &str, params: &Json) -> Result<(), String> {
        match kind {
            "insert-page" => {
                let page = params.get("page").cloned().unwrap_or(Json::Null);
                only_members(&page, &["mediaBox", "cropBox", "rotate", "content"], "PdfPage")?;
                let media_box = media_box_field(&page, "mediaBox").ok_or("insert-page: `page.mediaBox` must be four numbers")?;
                let rotate = number_field(&page, "rotate") as i64;
                let mut order: Vec<ObjectId> = document.get_pages().into_values().collect();
                let content_id = document.add_object(Object::Stream(Stream::new(Dictionary::new(), content_stream(&page.array("content"))?)));
                let mut dict = Dictionary::new();
                dict.set("Type", Object::Name(b"Page".to_vec()));
                dict.set("MediaBox", Object::Array(media_box.iter().map(|value| Object::Real(*value)).collect()));
                if let Some(crop_box) = media_box_field(&page, "cropBox") {
                    dict.set("CropBox", Object::Array(crop_box.iter().map(|value| Object::Real(*value)).collect()));
                }
                dict.set("Rotate", Object::Integer(rotate));
                dict.set("Resources", Object::Dictionary(Dictionary::new()));
                dict.set("Contents", Object::Reference(content_id));
                let page_id = document.add_object(Object::Dictionary(dict));
                let clamped = usize_field(params, "index").min(order.len());
                order.insert(clamped, page_id);
                reorder_pages(document, &order)?;
            }
            "set-page-media-box" => {
                if let Some(page_id) = page_id_at(document, usize_field(params, "index")) {
                    if let Some(media_box) = media_box_field(params, "mediaBox") {
                        if let Ok(dict) = document.get_object_mut(page_id).and_then(Object::as_dict_mut) {
                            dict.set("MediaBox", Object::Array(media_box.iter().map(|value| Object::Real(*value)).collect()));
                        }
                    }
                }
            }
            "set-page-crop-box" => {
                if let Some(page_id) = page_id_at(document, usize_field(params, "index")) {
                    if let Ok(dict) = document.get_object_mut(page_id).and_then(Object::as_dict_mut) {
                        match media_box_field(params, "cropBox") {
                            Some(crop_box) => dict.set("CropBox", Object::Array(crop_box.iter().map(|value| Object::Real(*value)).collect())),
                            None => {
                                dict.remove(b"CropBox");
                            }
                        }
                    }
                }
            }
            "append-page-content" => {
                if let Some(page_id) = page_id_at(document, usize_field(params, "index")) {
                    document.add_page_contents(page_id, content_stream(&params.array("content"))?).map_err(|error| format!("append-page-content: {error}"))?;
                }
            }
            "insert-object" => {
                let id = json_object_id(params);
                let value = json_to_object(params.get("value").unwrap_or(&Json::Null))?;
                document.objects.entry(id).or_insert(value);
            }
            "remove-object" => {
                document.objects.remove(&json_object_id(params));
            }
            "set-object-value" => {
                let id = json_object_id(params);
                document.objects.insert(id, json_to_object(params.get("value").unwrap_or(&Json::Null))?);
            }
            "set-dict-entry" => {
                let id = json_object_id(params);
                let path = params.array("path");
                let key = params.str("key");
                let value = json_to_object(params.get("value").unwrap_or(&Json::Null))?;
                if let Some(dict) = navigate_mut(document, id, &path) {
                    dict.set(key, value);
                }
            }
            "remove-dict-entry" => {
                let id = json_object_id(params);
                let path = params.array("path");
                let key = params.str("key");
                if let Some(dict) = navigate_mut(document, id, &path) {
                    dict.remove(key.as_bytes());
                }
            }
            "set-trailer-entry" => {
                let key = params.str("key");
                let value = json_to_object(params.get("value").unwrap_or(&Json::Null))?;
                document.trailer.set(key, value);
            }
            "remove-trailer-entry" => {
                document.trailer.remove(params.str("key").as_bytes());
            }
            "move-page" => {
                let from = usize_field(params, "from");
                let mut order: Vec<ObjectId> = document.get_pages().into_values().collect();
                if from < order.len() {
                    let page_id = order.remove(from);
                    let clamped_to = usize_field(params, "to").min(order.len());
                    order.insert(clamped_to, page_id);
                    reorder_pages(document, &order)?;
                }
            }
            "set-page-content" => {
                if let Some(page_id) = page_id_at(document, usize_field(params, "index")) {
                    document.change_page_content(page_id, content_stream(&params.array("content"))?).map_err(|error| format!("set-page-content: {error}"))?;
                }
            }
            "set-page-rotation" => {
                if let Some(page_id) = page_id_at(document, usize_field(params, "index")) {
                    if let Ok(dict) = document.get_object_mut(page_id).and_then(Object::as_dict_mut) {
                        dict.set("Rotate", Object::Integer(number_field(params, "rotation") as i64));
                    }
                }
            }
            other => return Err(format!("mutation kind {other:?} has no oracle implementation")),
        }
        Ok(())
    }
    //#endregion 🔖️Forward

    //#region 🔖️Inverse
    /// ↩️ Reads `document`'s CURRENT (pre-mutation) state to build the spec that undoes `{kind,
    /// params}` -- same law `PdfMutation::inverse` proves at the Rust-model level
    /// (`apply(inverse(m, base), apply(m, base)) == base`), computed here against the reference
    /// library instead.
    ///
    /// ⚠️ An unrecognised kind is an ERROR, never "the same mutation again": a fallback that hands
    /// back `{kind, params}` unchanged is not an inverse.
    fn inverse_spec(document: &Document, kind: &str, params: &Json) -> Result<Json, String> {
        let spec = |inverse_kind: &str, inverse_params: Json| Json::Object(vec![("kind".to_string(), Json::String(inverse_kind.to_string())), ("params".to_string(), inverse_params)]);
        Ok(match kind {
            "set-info" => {
                let mut entries = Vec::new();
                if let Some(title) = info_entry(document, b"Title") {
                    entries.push(("title", Json::String(title)));
                }
                if let Some(author) = info_entry(document, b"Author") {
                    entries.push(("author", Json::String(author)));
                }
                spec("set-info", object(vec![("info", object(entries))]))
            }
            "insert-page" => {
                let clamped = usize_field(params, "index").min(document.get_pages().len());
                spec("remove-page", object(vec![("index", Json::Number(clamped as f64))]))
            }
            "remove-page" => {
                let index = usize_field(params, "index");
                match page_id_at(document, index) {
                    Some(page_id) => {
                        let media_box = document
                            .get_dictionary(page_id)
                            .ok()
                            .and_then(|dict| dict.get(b"MediaBox").ok())
                            .and_then(|value| value.as_array().ok())
                            .map(|items| items.iter().map(|item| item.as_float().unwrap_or(0.0)).collect::<Vec<f32>>())
                            .unwrap_or_else(|| vec![0.0, 0.0, 612.0, 792.0]);
                        let rotate = document.get_dictionary(page_id).ok().and_then(|dict| dict.get(b"Rotate").ok()).and_then(|value| value.as_i64().ok()).unwrap_or(0);
                        let page = object(vec![("mediaBox", number_array(&media_box)), ("rotate", Json::Number(rotate as f64)), ("content", text_ops(&page_text(document, page_id)))]);
                        spec("insert-page", object(vec![("index", Json::Number(index as f64)), ("page", page)]))
                    }
                    None => return Err(format!("remove-page index {index} has no inverse target")),
                }
            }
            "set-page-media-box" => {
                let index = usize_field(params, "index");
                let prior = page_id_at(document, index)
                    .and_then(|page_id| document.get_dictionary(page_id).ok())
                    .and_then(|dict| dict.get(b"MediaBox").ok())
                    .and_then(|value| value.as_array().ok())
                    .map(|items| items.iter().map(|item| item.as_float().unwrap_or(0.0)).collect::<Vec<f32>>())
                    .unwrap_or_else(|| vec![0.0, 0.0, 612.0, 792.0]);
                spec("set-page-media-box", object(vec![("index", Json::Number(index as f64)), ("mediaBox", number_array(&prior))]))
            }
            "set-page-crop-box" => {
                let index = usize_field(params, "index");
                match page_id_at(document, index).and_then(|page_id| document.get_dictionary(page_id).ok()).and_then(|dict| dict.get(b"CropBox").ok()).and_then(|value| value.as_array().ok()) {
                    Some(items) => {
                        let prior: Vec<f32> = items.iter().map(|item| item.as_float().unwrap_or(0.0)).collect();
                        spec("set-page-crop-box", object(vec![("index", Json::Number(index as f64)), ("cropBox", number_array(&prior))]))
                    }
                    None => spec("set-page-crop-box", object(vec![("index", Json::Number(index as f64)), ("cropBox", Json::Null)])),
                }
            }
            "append-page-content" => {
                let index = usize_field(params, "index");
                let prior = page_id_at(document, index).map(|page_id| page_text(document, page_id)).unwrap_or_default();
                spec("set-page-content", object(vec![("index", Json::Number(index as f64)), ("content", text_ops(&prior))]))
            }
            "insert-object" => spec("remove-object", object(vec![("id", params.get("id").cloned().unwrap_or(Json::Null))])),
            "remove-object" => {
                let id = json_object_id(params);
                match document.objects.get(&id) {
                    Some(value) => spec("insert-object", object(vec![("id", params.get("id").cloned().unwrap_or(Json::Null)), ("value", object_to_json(value))])),
                    None => return Err(format!("remove-object id {id:?} has no inverse target")),
                }
            }
            "set-object-value" => {
                let id = json_object_id(params);
                match document.objects.get(&id) {
                    Some(value) => spec("set-object-value", object(vec![("id", params.get("id").cloned().unwrap_or(Json::Null)), ("value", object_to_json(value))])),
                    None => spec("remove-object", object(vec![("id", params.get("id").cloned().unwrap_or(Json::Null))])),
                }
            }
            "set-dict-entry" => {
                let id = json_object_id(params);
                let path = params.array("path");
                let key = params.str("key");
                match navigate(document, id, &path).and_then(|dict| dict.get(key.as_bytes()).ok()) {
                    Some(prior) => {
                        spec("set-dict-entry", object(vec![("id", params.get("id").cloned().unwrap_or(Json::Null)), ("path", params.get("path").cloned().unwrap_or(Json::Array(vec![]))), ("key", Json::String(key)), ("value", object_to_json(prior))]))
                    }
                    None => spec("remove-dict-entry", object(vec![("id", params.get("id").cloned().unwrap_or(Json::Null)), ("path", params.get("path").cloned().unwrap_or(Json::Array(vec![]))), ("key", Json::String(key))])),
                }
            }
            "remove-dict-entry" => {
                let id = json_object_id(params);
                let path = params.array("path");
                let key = params.str("key");
                match navigate(document, id, &path).and_then(|dict| dict.get(key.as_bytes()).ok()) {
                    Some(prior) => {
                        spec("set-dict-entry", object(vec![("id", params.get("id").cloned().unwrap_or(Json::Null)), ("path", params.get("path").cloned().unwrap_or(Json::Array(vec![]))), ("key", Json::String(key)), ("value", object_to_json(prior))]))
                    }
                    None => return Err(format!("remove-dict-entry key {key:?} has no inverse target")),
                }
            }
            "set-trailer-entry" => {
                let key = params.str("key");
                match document.trailer.get(key.as_bytes()).ok() {
                    Some(prior) => spec("set-trailer-entry", object(vec![("key", Json::String(key)), ("value", object_to_json(prior))])),
                    None => spec("remove-trailer-entry", object(vec![("key", Json::String(key))])),
                }
            }
            "remove-trailer-entry" => {
                let key = params.str("key");
                match document.trailer.get(key.as_bytes()).ok() {
                    Some(prior) => spec("set-trailer-entry", object(vec![("key", Json::String(key)), ("value", object_to_json(prior))])),
                    None => return Err(format!("remove-trailer-entry key {key:?} has no inverse target")),
                }
            }
            "move-page" => {
                let from = usize_field(params, "from");
                let len = document.get_pages().len();
                if from >= len {
                    return Err(format!("move-page index {from} has no inverse target"));
                }
                let clamped_to = usize_field(params, "to").min(len.saturating_sub(1));
                spec("move-page", object(vec![("from", Json::Number(clamped_to as f64)), ("to", Json::Number(from as f64))]))
            }
            "set-page-content" => {
                let index = usize_field(params, "index");
                let prior = page_id_at(document, index).map(|page_id| page_text(document, page_id)).unwrap_or_default();
                spec("set-page-content", object(vec![("index", Json::Number(index as f64)), ("content", text_ops(&prior))]))
            }
            "set-page-rotation" => {
                let index = usize_field(params, "index");
                let prior = page_id_at(document, index).and_then(|page_id| document.get_dictionary(page_id).ok()).and_then(|dict| dict.get(b"Rotate").ok()).and_then(|value| value.as_i64().ok()).unwrap_or(0);
                spec("set-page-rotation", object(vec![("index", Json::Number(index as f64)), ("rotation", Json::Number(prior as f64))]))
            }
            other => return Err(format!("mutation kind {other:?} has no oracle inverse implementation")),
        })
    }
    //#endregion 🔖️Inverse

    //#region 🔖️Routing
    /// 🧭️ `remove-page`/`set-info` route to the shared `document` module's own reference
    /// implementation (per the fleet brief: reuse/extend it rather than duplicating).
    /// Every other kind mutates a freshly loaded [`Document`] directly via [`apply_kind`].
    pub fn apply_mutation(input: &[u8], kind: &str, params: &Json) -> Result<Vec<u8>, String> {
        match kind {
            "" => Err("mutation spec carries no `kind`".to_string()),
            "remove-page" => oracle_delete_page(input, usize_field(params, "index") as u32 + 1),
            "set-info" => {
                let info = params.get("info").cloned().unwrap_or(Json::Null);
                only_members(&info, &["title", "author"], "PdfInfo")?;
                oracle_replace_metadata(input, present_string(&info, "title").as_deref(), present_string(&info, "author").as_deref())
            }
            _ => {
                let mut document = Document::load_mem(input).map_err(|error| format!("lopdf could not parse the input: {error}"))?;
                apply_kind(&mut document, kind, params)?;
                let mut out = Vec::new();
                document.save_to(&mut out).map_err(|error| format!("lopdf could not save: {error}"))?;
                Ok(out)
            }
        }
    }

    /// 🔄️ Parses and reserializes a document without routing through a synthetic mutation.
    pub fn round_trip(input: &[u8]) -> Result<Vec<u8>, String> {
        let mut document = Document::load_mem(input).map_err(|error| format!("lopdf could not parse the input: {error}"))?;
        let mut out = Vec::new();
        document.save_to(&mut out).map_err(|error| format!("lopdf could not save: {error}"))?;
        Ok(out)
    }

    /// ↩️ Applies `{kind, params}` and then its computed inverse, in sequence, and returns the
    /// re-serialized result -- the caller compares its projection against the ORIGINAL input's own.
    pub fn apply_mutation_inverse(input: &[u8], kind: &str, params: &Json) -> Result<Vec<u8>, String> {
        let reader = Document::load_mem(input).map_err(|error| format!("lopdf could not parse the input: {error}"))?;
        let inverse = inverse_spec(&reader, kind, params)?;
        let mutated = apply_mutation(input, kind, params)?;
        apply_mutation(&mutated, &inverse.str("kind"), inverse.get("params").unwrap_or(&Json::Null))
    }

    /// 🔤️ A spec field that is PRESENT as a string, empty or not. `/Title ()` and an absent
    /// `/Title` are different documents -- this fixture's own `/Info` carries both `/Title ()` and
    /// `/Author ()` -- so an inverse that has to restore an empty metadata value must be able to
    /// ask for one.
    fn present_string(value: &Json, key: &str) -> Option<String> {
        match value.get(key) {
            Some(Json::String(text)) => Some(text.clone()),
            _ => None,
        }
    }

    /// 🔎️ One `/Info` entry of the CURRENT document, resolving the trailer's indirect reference.
    /// Present-and-empty is distinguished from absent, for the reason [`present_string`] gives.
    fn info_entry(document: &Document, key: &[u8]) -> Option<String> {
        let dictionary = match document.trailer.get(b"Info").ok()? {
            Object::Reference(id) => document.get_dictionary(*id).ok()?,
            Object::Dictionary(dictionary) => dictionary,
            _ => return None,
        };
        dictionary.get(key).ok()?.as_str().ok().map(|bytes| String::from_utf8_lossy(bytes).into_owned())
    }

    /// 🪟️ How far the catalog rendering resolves indirect references before it stops. Three is what
    /// the real thesis needs and no more: `/OpenAction → #145 → {/S /GoTo, /D [ref, /Fit]}` and
    /// `/Outlines → #3015 → {/Type /Outlines, /First ref, /Last ref, /Count 6}` both land inside it,
    /// while everything below renders as an opaque marker — so an edit far away in the graph can
    /// never register here as a change to the catalog.
    const CATALOG_DEPTH: u8 = 3;

    /// 🔎️ One object rendered for the [`object_graph`] surface, with indirect references RESOLVED
    /// inline while `depth` lasts and rendered as `"<indirect object>"` once it runs out. Object
    /// NUMBERS never appear — `semantic-pdf-v1` declares them writer freedom, and the resolved value
    /// is what a conforming reader sees anyway (ISO 32000-1 §7.3.10). A reference that resolves to
    /// nothing renders as `null`, which is precisely what makes `remove-object` observable. `seen`
    /// is the cycle guard: `/Parent` back-pointers make a PDF object graph cyclic by construction.
    fn render_object(document: &Document, object: &Object, depth: u8, seen: &mut Vec<ObjectId>) -> Json {
        match object {
            Object::Null => Json::Null,
            Object::Boolean(value) => Json::Bool(*value),
            Object::Integer(value) => Json::Number(*value as f64),
            Object::Real(value) => Json::Number(*value as f64),
            Object::Name(name) => Json::String(format!("/{}", String::from_utf8_lossy(name))),
            Object::String(bytes, _) => Json::String(String::from_utf8_lossy(bytes).into_owned()),
            Object::Array(items) => Json::Array(items.iter().map(|item| render_object(document, item, depth.saturating_sub(1), seen)).collect()),
            Object::Dictionary(dictionary) => render_dictionary(document, dictionary, depth, seen),
            Object::Stream(stream) => Json::Object(vec![("streamDictionary".to_string(), render_dictionary(document, &stream.dict, depth, seen))]),
            Object::Reference(id) => {
                if depth == 0 || seen.contains(id) {
                    return Json::String("<indirect object>".to_string());
                }
                seen.push(*id);
                let resolved = match document.get_object(*id) {
                    Ok(target) => render_object(document, target, depth - 1, seen),
                    Err(_) => Json::Null,
                };
                seen.pop();
                resolved
            }
        }
    }

    /// 🔤️ A dictionary rendered key-sorted, so dictionary ORDER — writer freedom under
    /// `semantic-pdf-v1` — never reads as a difference.
    fn render_dictionary(document: &Document, dictionary: &Dictionary, depth: u8, seen: &mut Vec<ObjectId>) -> Json {
        let mut entries: Vec<(String, Json)> = dictionary.iter().map(|(key, value)| (String::from_utf8_lossy(key).into_owned(), render_object(document, value, depth.saturating_sub(1), seen))).collect();
        entries.sort_by(|one, other| one.0.cmp(&other.0));
        Json::Object(entries)
    }

    /// 🕸️ The surface the object-graph half of this vocabulary lives on. `insert-object`,
    /// `remove-object`, `set-object-value`, `set-dict-entry`, `remove-dict-entry`,
    /// `set-trailer-entry` and `remove-trailer-entry` — seven of the eighteen declared kinds — never
    /// touch a page, so `document::project_pdf`'s page-and-metadata shape cannot see them at all,
    /// and six of the seven would pass whether or not the mutation ran. This is that gap closed, not
    /// excused.
    ///
    /// Two members, both read out of the bytes by `lopdf` alone:
    ///
    /// * `trailer` — every trailer entry except `Size`, `Prev` and `XRefStm`, which are
    ///   cross-reference bookkeeping the writer recomputes on every save and which
    ///   `semantic-pdf-v1` already calls non-normative. Values render at depth 0, so `/Root` and
    ///   `/Info` show as opaque markers rather than pulling their whole subtree in twice.
    /// * `catalog` — the document catalog, resolved to [`CATALOG_DEPTH`], WITHOUT `/Pages`: the page
    ///   tree is already projected in full by `pageCount` and `pages`, and re-projecting it here
    ///   would make every page edit register twice and make a page reorder churn a surface it has no
    ///   business churning.
    fn object_graph(document: &Document) -> Json {
        let mut trailer: Vec<(String, Json)> = document
            .trailer
            .iter()
            .filter(|(key, _)| !matches!(key.as_slice(), b"Size" | b"Prev" | b"XRefStm"))
            .map(|(key, value)| (String::from_utf8_lossy(key).into_owned(), render_object(document, value, 0, &mut Vec::new())))
            .collect();
        trailer.sort_by(|one, other| one.0.cmp(&other.0));
        let catalog = match document.catalog() {
            Ok(dictionary) => {
                let mut seen: Vec<ObjectId> = Vec::new();
                let mut entries: Vec<(String, Json)> = dictionary.iter().filter(|(key, _)| key.as_slice() != b"Pages").map(|(key, value)| (String::from_utf8_lossy(key).into_owned(), render_object(document, value, CATALOG_DEPTH - 1, &mut seen))).collect();
                entries.sort_by(|one, other| one.0.cmp(&other.0));
                Json::Object(entries)
            }
            Err(error) => Json::String(format!("<no catalog: {error}>")),
        };
        Json::Object(vec![("trailer".to_string(), Json::Object(trailer)), ("catalog".to_string(), catalog)])
    }

    /// 👁️ This subset's own projection: the shared `document::project_pdf` independent-reader
    /// projection, augmented with the two surfaces the `pdf-1-7-base` vocabulary needs and no other
    /// subset does — each page's resolved `/CropBox` and `/Rotate` (normative for
    /// `set-page-crop-box` and `set-page-rotation`), and the [`object_graph`] surface the seven
    /// object/dict/trailer kinds live on. The fleet brief's own "do not edit the shared family
    /// module's existing functions" rule is what makes all three an addition here rather than a
    /// change there.
    pub fn project_pdf_1_7(bytes: &[u8]) -> Result<Json, String> {
        let base = document::project_pdf(bytes)?;
        let reader = Document::load_mem(bytes).map_err(|error| format!("independent reader could not parse the document: {error}"))?;
        let boxes: Vec<(Json, i64)> = reader
            .get_pages()
            .into_values()
            .map(|page_id| {
                let dictionary = reader.get_dictionary(page_id).ok();
                let crop_box = dictionary
                    .and_then(|dict| dict.get(b"CropBox").ok())
                    .and_then(|value| value.as_array().ok())
                    .map(|items| Json::Array(items.iter().map(|item| Json::Number(item.as_float().unwrap_or(0.0) as f64)).collect()))
                    .unwrap_or(Json::Null);
                let rotate = dictionary.and_then(|dict| dict.get(b"Rotate").ok()).and_then(|value| value.as_i64().ok()).unwrap_or(0);
                (crop_box, rotate)
            })
            .collect();
        let Json::Object(entries) = base else { return Ok(base) };
        let mut augmented: Vec<(String, Json)> = entries
            .into_iter()
            .map(|(key, value)| {
                if key != "pages" {
                    return (key, value);
                }
                let Json::Array(pages) = value else { return (key, value) };
                let merged = pages
                    .into_iter()
                    .enumerate()
                    .map(|(index, page)| match page {
                        Json::Object(mut fields) => {
                            let (crop_box, rotate) = boxes.get(index).cloned().unwrap_or((Json::Null, 0));
                            fields.push(("cropBox".to_string(), crop_box));
                            fields.push(("rotate".to_string(), Json::Number(rotate as f64)));
                            Json::Object(fields)
                        }
                        other => other,
                    })
                    .collect();
                (key, Json::Array(merged))
            })
            .collect();
        augmented.push(("objectGraph".to_string(), object_graph(&reader)));
        Ok(Json::Object(augmented))
    }
}
//#endregion 🔖️Oracles

//#region 🔖️Dispatch
/// 🦠️ Applies one declared mutation kind to a real artifact and returns the re-serialized bytes.
/// An unrecognised kind is an error, never a silent no-op: a mutation that is quietly skipped
/// reports as a passing test.
#[cfg(feature = "oracles")]
pub fn oracle_apply_mutation(input: &[u8], spec: &Json) -> Result<Vec<u8>, String> {
    let kind = spec.str("kind");
    let params = spec.get("params").cloned().unwrap_or(Json::Null);
    oracles::apply_mutation(input, &kind, &params)
}

/// ↩️ Applies one declared mutation kind and then its own computed inverse, in sequence, proving
/// the same `apply(inverse(m, base), apply(m, base)) == base` law `PdfMutation::inverse` proves at
/// the Rust-model level, here against the registered reference library instead.
#[cfg(feature = "oracles")]
pub fn oracle_apply_mutation_inverse(input: &[u8], spec: &Json) -> Result<Vec<u8>, String> {
    let kind = spec.str("kind");
    let params = spec.get("params").cloned().unwrap_or(Json::Null);
    oracles::apply_mutation_inverse(input, &kind, &params)
}

/// 🔄️ Parses and reserializes through the independent implementation without a fake mutation.
#[cfg(feature = "oracles")]
pub fn oracle_round_trip(input: &[u8]) -> Result<Vec<u8>, String> {
    oracles::round_trip(input)
}

/// 👁️ This subset's own semantic projection -- `document::project_pdf` augmented with per-page
/// `/Rotate`. @see [`oracles::project_pdf_1_7`].
#[cfg(feature = "oracles")]
pub fn project_pdf_1_7(bytes: &[u8]) -> Result<Json, String> {
    oracles::project_pdf_1_7(bytes)
}

/// 🚫️ Without the `oracles` feature the reference implementation is not linked at all.
#[cfg(not(feature = "oracles"))]
pub fn oracle_apply_mutation(_input: &[u8], _spec: &Json) -> Result<Vec<u8>, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}

#[cfg(not(feature = "oracles"))]
pub fn oracle_apply_mutation_inverse(_input: &[u8], _spec: &Json) -> Result<Vec<u8>, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}

#[cfg(not(feature = "oracles"))]
pub fn oracle_round_trip(_input: &[u8]) -> Result<Vec<u8>, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}

#[cfg(not(feature = "oracles"))]
pub fn project_pdf_1_7(_bytes: &[u8]) -> Result<Json, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}
//#endregion 🔖️Dispatch

//#region 🧪️Tests
#[cfg(all(test, feature = "oracles"))]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
