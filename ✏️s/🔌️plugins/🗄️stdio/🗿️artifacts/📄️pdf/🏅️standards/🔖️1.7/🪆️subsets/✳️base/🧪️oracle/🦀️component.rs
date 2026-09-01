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
//! @see ../🧪️oracle/🔣️.json — the mutation catalog this module is measured against.
//! @see ../🧬️schema/🧬️mutations/🦀️component.rs — the mutation vocabulary itself (`PdfMutation::KINDS`).

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
/// belongs to whoever owns `../🧬️schema/🧬️mutations/🦀️component.rs`. Its INVERSE is still under the
/// full law, and so is every other kind — this exempts one kind from one law, not from the case.
pub const UNOBSERVABLE: &[&str] = &["insert-object"];
//#endregion 🔖️Vocabulary

//#region 🔖️PageContentLaw
/// 🧱️ The three kinds whose undo has to REBUILD a page's content stream, and therefore cannot
/// restore `contentOperators`. This is a property of the `pdf-1-7-base` VOCABULARY, not of the
/// reference implementation, and it was found by asserting the law rather than by reasoning about
/// it: `PdfPage`'s only content field is `text` (`../🧬️schema/📸️snapshot/🦀️component.rs`), so
/// `InsertPage`/`SetPageContent` carry extracted text and nothing else, and both producers
/// regenerate a five-operator `BT /F1 12 Tf 72 720 Td (…) Tj ET` stream from it. Page 8 of the real
/// thesis carries 294 operators — glyph positioning, graphics state, the lot — and no round trip
/// through a single `text` field can bring them back. `AppendPageContent` was documented from the
/// start as having no minimal inverse in this vocabulary; this is the same gap, measured.
///
/// ⚖️ Exactly ONE axis is exempted, and only for these three kinds. `version`, `pageCount`, every
/// page's `mediaBox`, `cropBox`, `rotate` and — critically — the shown `text` the vocabulary DOES
/// carry all stay under the full law, as does the whole `objectGraph` surface. Widening `PdfPage`
/// to retain a real content stream is the fix; it belongs to whoever owns that snapshot. Lives here
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
    use lopdf::{Dictionary, Document, Object, ObjectId, Stream};
    use semio_repo_test_host::Json;

    //#region 🔖️JsonValue
    /// 🔎️ Owned PDF-object JSON grammar this module's mutation params speak, independent of the
    /// wire `PdfObject` enum (this crate never depends on `semio-s-plugin-stdio`, the production
    /// crate `PdfObject` lives in — see this file's own header) but shaped identically field for
    /// field, so a spec written for the oracle reads the same as one written for the subject.
    /// `{"kind":"null"|"bool"|"int"|"real"|"str"|"name"|"array"|"dict"|"ref", ...}`.
    fn json_to_object(value: &Json) -> Object {
        match value.str("kind").as_str() {
            "bool" => Object::Boolean(matches!(value.get("value"), Some(Json::Bool(true)))),
            "int" => Object::Integer(number_field(value, "value") as i64),
            "real" => Object::Real(number_field(value, "value") as f32),
            "str" => Object::string_literal(value.str("value")),
            "name" => Object::Name(value.str("value").into_bytes()),
            "array" => Object::Array(value.array("items").iter().map(json_to_object).collect()),
            "dict" => Object::Dictionary(json_to_dictionary(value)),
            "ref" => Object::Reference(json_object_id(value)),
            _ => Object::Null,
        }
    }

    fn json_to_dictionary(value: &Json) -> Dictionary {
        let mut dict = Dictionary::new();
        for entry in value.array("entries") {
            dict.set(entry.str("key"), json_to_object(entry.get("value").unwrap_or(&Json::Null)));
        }
        dict
    }

    /// 🔁️ The reverse of [`json_to_object`] — used to capture an object's CURRENT value before a
    /// mutation touches it, so `oracle_apply_mutation_inverse` can hand that exact value back to
    /// [`json_to_object`] as the undo's own params.
    fn object_to_json(object: &Object) -> Json {
        match object {
            Object::Null => Json::Object(vec![("kind".to_string(), Json::String("null".to_string()))]),
            Object::Boolean(value) => Json::Object(vec![("kind".to_string(), Json::String("bool".to_string())), ("value".to_string(), Json::Bool(*value))]),
            Object::Integer(value) => Json::Object(vec![("kind".to_string(), Json::String("int".to_string())), ("value".to_string(), Json::Number(*value as f64))]),
            Object::Real(value) => Json::Object(vec![("kind".to_string(), Json::String("real".to_string())), ("value".to_string(), Json::Number(*value as f64))]),
            Object::String(bytes, _) => Json::Object(vec![("kind".to_string(), Json::String("str".to_string())), ("value".to_string(), Json::String(String::from_utf8_lossy(bytes).to_string()))]),
            Object::Name(bytes) => Json::Object(vec![("kind".to_string(), Json::String("name".to_string())), ("value".to_string(), Json::String(String::from_utf8_lossy(bytes).to_string()))]),
            Object::Array(items) => Json::Object(vec![("kind".to_string(), Json::String("array".to_string())), ("items".to_string(), Json::Array(items.iter().map(object_to_json).collect()))]),
            Object::Dictionary(dict) => Json::Object(vec![
                ("kind".to_string(), Json::String("dict".to_string())),
                ("entries".to_string(), Json::Array(dict.iter().map(|(key, value)| Json::Object(vec![("key".to_string(), Json::String(String::from_utf8_lossy(key).to_string())), ("value".to_string(), object_to_json(value))])).collect())),
            ]),
            Object::Stream(stream) => Json::Object(vec![
                ("kind".to_string(), Json::String("dict".to_string())),
                ("entries".to_string(), Json::Array(stream.dict.iter().map(|(key, value)| Json::Object(vec![("key".to_string(), Json::String(String::from_utf8_lossy(key).to_string())), ("value".to_string(), object_to_json(value))])).collect())),
            ]),
            Object::Reference(id) => Json::Object(vec![("kind".to_string(), Json::String("ref".to_string())), ("num".to_string(), Json::Number(id.0 as f64)), ("gen".to_string(), Json::Number(id.1 as f64))]),
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
    //#endregion 🔖️JsonValue

    //#region 🔖️ContentStream
    /// ✏️️ A minimal `BT ... Tj ET` content stream carrying `text` as one `Tj`-shown string — the
    /// same operator `document::project_pdf`'s independent reader already scans for.
    fn text_content_stream(text: &str) -> Vec<u8> {
        // 🕳️ A page with NO extractable text gets a text object with no text-showing operator, not a
        // `() Tj` showing the empty string. Found by asserting the inverse law: the real thesis sets
        // its type with `TJ` (the positioned-array form), so `page_text` — which reads `Tj`, the only
        // form this writer emits — extracts nothing from it, and re-encoding an empty `text` as
        // `() Tj` turned a page the independent reader projects as `text: []` into one it projects
        // as `text: [""]`. Writing no operator is the faithful reconstruction of "no text".
        if text.is_empty() {
            return b"BT ET".to_vec();
        }
        let mut out = Vec::new();
        out.extend_from_slice(b"BT /F1 12 Tf 72 720 Td (");
        for byte in text.as_bytes() {
            match byte {
                b'(' | b')' | b'\\' => {
                    out.push(b'\\');
                    out.push(*byte);
                }
                b'\n' => out.extend_from_slice(b"\\n"),
                other => out.push(*other),
            }
        }
        out.extend_from_slice(b") Tj ET");
        out
    }

    /// 🔎️ Concatenated `Tj` operand text of one page's content -- the independent-reader counterpart
    /// of what `text_content_stream` writes, used to capture a page's prior text before mutating it.
    fn page_text(document: &Document, page_id: ObjectId) -> String {
        let content = document.get_page_content(page_id);
        let tj_operand_separator = "
";
        lopdf::content::Content::decode(&content)
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
    /// 🔎️ Immutable walk of `path` (`{"kind":"index","index":N}` / `{"kind":"key","key":"K"}` steps)
    /// from object `id`'s own value down to the dict/stream-dict the leaf `key` lives in.
    fn navigate<'d>(document: &'d Document, id: ObjectId, path: &[Json]) -> Option<&'d Dictionary> {
        let mut current = document.get_object(id).ok()?;
        for segment in path {
            current = match (segment.str("kind").as_str(), current) {
                ("index", Object::Array(items)) => items.get(usize_field(segment, "index"))?,
                ("key", Object::Dictionary(dict)) => dict.get(segment.str("key").as_bytes()).ok()?,
                ("key", Object::Stream(stream)) => stream.dict.get(segment.str("key").as_bytes()).ok()?,
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
                ("index", Object::Array(items)) => items.get_mut(usize_field(segment, "index"))?,
                ("key", Object::Dictionary(dict)) => dict.get_mut(segment.str("key").as_bytes()).ok()?,
                ("key", Object::Stream(stream)) => stream.dict.get_mut(segment.str("key").as_bytes()).ok()?,
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
                let media_box = media_box_field(&page, "mediaBox").unwrap_or([0.0, 0.0, 612.0, 792.0]);
                let rotate = number_field(&page, "rotate") as i64;
                let text = page.str("text");
                let mut order: Vec<ObjectId> = document.get_pages().into_values().collect();
                let content_id = document.add_object(Object::Stream(Stream::new(Dictionary::new(), text_content_stream(&text))));
                let mut dict = Dictionary::new();
                dict.set("Type", Object::Name(b"Page".to_vec()));
                dict.set("MediaBox", Object::Array(media_box.iter().map(|value| Object::Real(*value)).collect()));
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
                    let _ = document.add_page_contents(page_id, text_content_stream(&params.str("text")));
                }
            }
            "insert-object" => {
                let id = json_object_id(params);
                document.objects.entry(id).or_insert_with(|| json_to_object(&params.get("value").cloned().unwrap_or(Json::Null)));
            }
            "remove-object" => {
                document.objects.remove(&json_object_id(params));
            }
            "set-object-value" => {
                let id = json_object_id(params);
                document.objects.insert(id, json_to_object(&params.get("value").cloned().unwrap_or(Json::Null)));
            }
            "set-dict-entry" => {
                let id = json_object_id(params);
                let path = params.array("path");
                let key = params.str("key");
                let value = json_to_object(&params.get("value").cloned().unwrap_or(Json::Null));
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
                let value = json_to_object(&params.get("value").cloned().unwrap_or(Json::Null));
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
                    let _ = document.change_page_content(page_id, text_content_stream(&params.str("text")));
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
        let obj = |entries: Vec<(&str, Json)>| Json::Object(entries.into_iter().map(|(key, value)| (key.to_string(), value)).collect());
        Ok(match kind {
            "set-info" => {
                let mut entries = Vec::new();
                if let Some(title) = info_entry(document, b"Title") {
                    entries.push(("title", Json::String(title)));
                }
                if let Some(author) = info_entry(document, b"Author") {
                    entries.push(("author", Json::String(author)));
                }
                spec("set-info", obj(entries))
            }
            "insert-page" => {
                let clamped = usize_field(params, "index").min(document.get_pages().len());
                spec("remove-page", obj(vec![("index", Json::Number(clamped as f64))]))
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
                        let text = page_text(document, page_id);
                        let page = obj(vec![("mediaBox", Json::Array(media_box.into_iter().map(|value| Json::Number(value as f64)).collect())), ("rotate", Json::Number(rotate as f64)), ("text", Json::String(text))]);
                        spec("insert-page", obj(vec![("index", Json::Number(index as f64)), ("page", page)]))
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
                spec("set-page-media-box", obj(vec![("index", Json::Number(index as f64)), ("mediaBox", Json::Array(prior.into_iter().map(|value| Json::Number(value as f64)).collect()))]))
            }
            "set-page-crop-box" => {
                let index = usize_field(params, "index");
                match page_id_at(document, index).and_then(|page_id| document.get_dictionary(page_id).ok()).and_then(|dict| dict.get(b"CropBox").ok()).and_then(|value| value.as_array().ok()) {
                    Some(items) => {
                        let prior: Vec<f32> = items.iter().map(|item| item.as_float().unwrap_or(0.0)).collect();
                        spec("set-page-crop-box", obj(vec![("index", Json::Number(index as f64)), ("cropBox", Json::Array(prior.into_iter().map(|value| Json::Number(value as f64)).collect()))]))
                    }
                    None => spec("set-page-crop-box", obj(vec![("index", Json::Number(index as f64))])),
                }
            }
            "append-page-content" => {
                let index = usize_field(params, "index");
                let prior = page_id_at(document, index).map(|page_id| page_text(document, page_id)).unwrap_or_default();
                spec("set-page-content", obj(vec![("index", Json::Number(index as f64)), ("text", Json::String(prior))]))
            }
            "insert-object" => spec("remove-object", obj(vec![("id", params.get("id").cloned().unwrap_or(Json::Null))])),
            "remove-object" => {
                let id = json_object_id(params);
                match document.objects.get(&id) {
                    Some(value) => spec("insert-object", obj(vec![("id", params.get("id").cloned().unwrap_or(Json::Null)), ("value", object_to_json(value))])),
                    None => return Err(format!("remove-object id {id:?} has no inverse target")),
                }
            }
            "set-object-value" => {
                let id = json_object_id(params);
                match document.objects.get(&id) {
                    Some(value) => spec("set-object-value", obj(vec![("id", params.get("id").cloned().unwrap_or(Json::Null)), ("value", object_to_json(value))])),
                    None => spec("remove-object", obj(vec![("id", params.get("id").cloned().unwrap_or(Json::Null))])),
                }
            }
            "set-dict-entry" => {
                let id = json_object_id(params);
                let path = params.array("path");
                let key = params.str("key");
                match navigate(document, id, &path).and_then(|dict| dict.get(key.as_bytes()).ok()) {
                    Some(prior) => {
                        spec("set-dict-entry", obj(vec![("id", params.get("id").cloned().unwrap_or(Json::Null)), ("path", params.get("path").cloned().unwrap_or(Json::Array(vec![]))), ("key", Json::String(key)), ("value", object_to_json(prior))]))
                    }
                    None => spec("remove-dict-entry", obj(vec![("id", params.get("id").cloned().unwrap_or(Json::Null)), ("path", params.get("path").cloned().unwrap_or(Json::Array(vec![]))), ("key", Json::String(key))])),
                }
            }
            "remove-dict-entry" => {
                let id = json_object_id(params);
                let path = params.array("path");
                let key = params.str("key");
                match navigate(document, id, &path).and_then(|dict| dict.get(key.as_bytes()).ok()) {
                    Some(prior) => {
                        spec("set-dict-entry", obj(vec![("id", params.get("id").cloned().unwrap_or(Json::Null)), ("path", params.get("path").cloned().unwrap_or(Json::Array(vec![]))), ("key", Json::String(key)), ("value", object_to_json(prior))]))
                    }
                    None => return Err(format!("remove-dict-entry key {key:?} has no inverse target")),
                }
            }
            "set-trailer-entry" => {
                let key = params.str("key");
                match document.trailer.get(key.as_bytes()).ok() {
                    Some(prior) => spec("set-trailer-entry", obj(vec![("key", Json::String(key)), ("value", object_to_json(prior))])),
                    None => spec("remove-trailer-entry", obj(vec![("key", Json::String(key))])),
                }
            }
            "remove-trailer-entry" => {
                let key = params.str("key");
                match document.trailer.get(key.as_bytes()).ok() {
                    Some(prior) => spec("set-trailer-entry", obj(vec![("key", Json::String(key)), ("value", object_to_json(prior))])),
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
                spec("move-page", obj(vec![("from", Json::Number(clamped_to as f64)), ("to", Json::Number(from as f64))]))
            }
            "set-page-content" => {
                let index = usize_field(params, "index");
                let prior = page_id_at(document, index).map(|page_id| page_text(document, page_id)).unwrap_or_default();
                spec("set-page-content", obj(vec![("index", Json::Number(index as f64)), ("text", Json::String(prior))]))
            }
            "set-page-rotation" => {
                let index = usize_field(params, "index");
                let prior = page_id_at(document, index).and_then(|page_id| document.get_dictionary(page_id).ok()).and_then(|dict| dict.get(b"Rotate").ok()).and_then(|value| value.as_i64().ok()).unwrap_or(0);
                spec("set-page-rotation", obj(vec![("index", Json::Number(index as f64)), ("rotation", Json::Number(prior as f64))]))
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
            "set-info" => oracle_replace_metadata(input, present_string(params, "title").as_deref(), present_string(params, "author").as_deref()),
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
mod tests {
    use super::*;

    /// 🧫️ The real committed document `mutate-pdf-1-7` runs on, read where the artifact already
    /// keeps it — a 6.3 MB, 65-page LaTeX bachelor thesis carrying 3,173 indirect objects, a
    /// six-entry outline tree and an `/OpenAction` `/GoTo` destination.
    const FIXTURE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️base/📚️examples/🎓️bachelor-thesis/🖼️assets/📄️bachelor-thesis.pdf");

    fn json_object(pairs: Vec<(&str, Json)>) -> Json {
        Json::Object(pairs.into_iter().map(|(key, value)| (key.to_string(), value)).collect())
    }

    fn number(value: f64) -> Json {
        Json::Number(value)
    }

    fn text(value: &str) -> Json {
        Json::String(value.to_string())
    }

    fn object_id(num: f64) -> Json {
        json_object(vec![("num", number(num)), ("gen", number(0.0))])
    }

    fn pdf_dict(entries: Vec<(&str, Json)>) -> Json {
        json_object(vec![("kind", text("dict")), ("entries", Json::Array(entries.into_iter().map(|(key, value)| json_object(vec![("key", text(key)), ("value", value)])).collect()))])
    }

    fn pdf_name(value: &str) -> Json {
        json_object(vec![("kind", text("name")), ("value", text(value))])
    }

    fn pdf_str(value: &str) -> Json {
        json_object(vec![("kind", text("str")), ("value", text(value))])
    }

    /// 🧾️ The Examples rows `../../../../🧪️tests/mutate-pdf-1-7/🥒️.feature` carries, one per
    /// declared kind — the same targets against the same real document, so a failure here and a
    /// failure there have the same cause and the same fix.
    fn params_for(kind: &str) -> Json {
        match kind {
            "insert-page" => json_object(vec![("index", number(30.0)), ("page", json_object(vec![("mediaBox", Json::Array(vec![number(0.0), number(0.0), number(612.0), number(792.0)])), ("rotate", number(0.0)), ("text", text("Inserted page for wave 7 mutation testing"))]))]),
            "remove-page" => json_object(vec![("index", number(7.0))]),
            "set-page-media-box" => json_object(vec![("index", number(15.0)), ("mediaBox", Json::Array(vec![number(0.0), number(0.0), number(595.0), number(842.0)]))]),
            "set-page-crop-box" => json_object(vec![("index", number(16.0)), ("cropBox", Json::Array(vec![number(10.0), number(10.0), number(580.0), number(820.0)]))]),
            "append-page-content" => json_object(vec![("index", number(17.0)), ("text", text("Appended content line for wave 7 testing"))]),
            "set-info" => json_object(vec![("title", text("Wave 7 Replaced Title")), ("author", text("Wave 7 Test Author"))]),
            "insert-object" => json_object(vec![("id", object_id(900001.0)), ("value", pdf_dict(vec![("Type", pdf_name("SemioWave7Marker")), ("Note", pdf_str("inserted by wave 7"))]))]),
            "remove-object" => json_object(vec![("id", object_id(3015.0))]),
            "set-object-value" => json_object(vec![("id", object_id(145.0)), ("value", pdf_dict(vec![("S", pdf_name("GoToR")), ("Note", pdf_str("replaced by wave 7"))]))]),
            "set-dict-entry" => json_object(vec![("id", object_id(3188.0)), ("path", Json::Array(vec![])), ("key", text("PageMode")), ("value", pdf_name("UseNone"))]),
            "remove-dict-entry" => json_object(vec![("id", object_id(3188.0)), ("path", Json::Array(vec![])), ("key", text("Outlines"))]),
            "set-trailer-entry" => json_object(vec![("key", text("SemioWave7Marker")), ("value", json_object(vec![("kind", text("int")), ("value", number(42.0))]))]),
            "remove-trailer-entry" => json_object(vec![("key", text("ID"))]),
            "move-page" => json_object(vec![("from", number(10.0)), ("to", number(40.0))]),
            "set-page-content" => json_object(vec![("index", number(20.0)), ("text", text("Replaced page content for wave 7 mutation testing"))]),
            "set-page-rotation" => json_object(vec![("index", number(5.0)), ("rotation", number(90.0))]),
            other => panic!("no test parameters for kind {other:?}"),
        }
    }

    fn spec(kind: &str) -> Json {
        json_object(vec![("kind", text(kind)), ("params", params_for(kind))])
    }

    fn fixture() -> Vec<u8> {
        std::fs::read(FIXTURE).expect("the committed bachelor-thesis document")
    }

    /// ⚖️ The two laws `mutate-pdf-1-7`'s adapter asserts in role, proven here against the real
    /// document without the runner: every declared kind moves the projection it is compared through
    /// (except the one [`UNOBSERVABLE`] names, with its reason), and every declared kind's own
    /// computed inverse lands back on the untouched document's projection (with
    /// [`regenerates_page_content`]'s single documented axis dropped for its three kinds).
    #[test]
    fn every_declared_kind_is_observable_and_its_inverse_restores_the_document() {
        let original = fixture();
        let base = project_pdf_1_7(&original).expect("the independent reader projects the real document");
        for kind in KINDS {
            let forward = spec(kind);
            let mutated = oracle_apply_mutation(&original, &forward).unwrap_or_else(|error| panic!("{kind}: {error}"));
            let moved = project_pdf_1_7(&mutated).unwrap_or_else(|error| panic!("{kind}: projecting the result failed: {error}"));
            if !UNOBSERVABLE.contains(kind) {
                assert_ne!(moved, base, "{kind} left the compared projection untouched, so its scenario would pass whether or not the mutation ran");
            }
            let restored = oracle_apply_mutation_inverse(&original, &forward).unwrap_or_else(|error| panic!("{kind}: inverse: {error}"));
            let recovered = project_pdf_1_7(&restored).unwrap_or_else(|error| panic!("{kind}: projecting the restored document failed: {error}"));
            let (expected, actual) = if regenerates_page_content(kind) { (without_content_operators(&base), without_content_operators(&recovered)) } else { (base.clone(), recovered) };
            assert_eq!(actual, expected, "{kind}: applying the mutation and then its own inverse must restore the document's projection");
        }
    }

    /// 🚫️ The one exemption, pinned rather than merely asserted: `insert-object` is unobservable
    /// BECAUSE the real document has nowhere for an unreferenced object to be seen from, not
    /// because the projection is thin. The moment the vocabulary grows a linking site — or the
    /// fixture grows a dangling reference — this flips red and the exemption has to be re-argued.
    #[test]
    fn insert_object_is_unobservable_only_because_nothing_can_reference_the_new_object() {
        let original = fixture();
        let base = project_pdf_1_7(&original).expect("the independent reader projects the real document");
        let mutated = oracle_apply_mutation(&original, &spec("insert-object")).expect("the reference inserts the object");
        assert_ne!(mutated, original, "the reference really did rewrite the file");
        assert_eq!(project_pdf_1_7(&mutated).unwrap(), base, "an object nothing references is unreachable (ISO 32000-1 §7.5.4) and must project identically");
    }

    /// 🕸️ The object-graph surface, checked against the values the real document actually carries,
    /// so a future refactor that quietly renders it empty cannot keep the observability test green.
    #[test]
    fn the_object_graph_surface_reads_the_real_catalog_and_trailer() {
        let projection = project_pdf_1_7(&fixture()).expect("the independent reader projects the real document");
        let graph = projection.get("objectGraph").expect("the projection carries the object-graph surface").clone();
        let trailer = graph.get("trailer").expect("a trailer surface").clone();
        assert_eq!(trailer.get("Root"), Some(&Json::String("<indirect object>".to_string())), "trailer references render opaquely; the catalog has its own member");
        assert!(trailer.get("ID").is_some(), "the real trailer carries the /ID pair remove-trailer-entry targets");
        assert!(trailer.get("Size").is_none(), "/Size is cross-reference bookkeeping the writer recomputes on every save");
        let catalog = graph.get("catalog").expect("a catalog surface").clone();
        assert!(catalog.get("Pages").is_none(), "the page tree is projected by pageCount/pages, never twice");
        assert_eq!(catalog.get("PageMode"), Some(&Json::String("/UseOutlines".to_string())), "set-dict-entry's target axis");
        assert_eq!(catalog.get("Outlines").and_then(|outlines| outlines.get("Count")).cloned(), Some(Json::Number(6.0)), "remove-object #3015 is the outline root the catalog resolves to");
        assert_eq!(catalog.get("OpenAction").and_then(|action| action.get("S")).cloned(), Some(Json::String("/GoTo".to_string())), "set-object-value #145 is the OpenAction the catalog resolves to");
    }

    /// 🔒️ Both halves of the identity law, on the real document.
    #[test]
    fn the_round_trip_is_projection_stable_and_not_a_byte_passthrough() {
        let original = fixture();
        let rebuilt = oracle_round_trip(&original).expect("the reference re-serializes the document");
        assert_ne!(rebuilt, original, "the reference rebuilds the file from its own object graph; identical bytes would mean the input was smuggled");
        assert_eq!(project_pdf_1_7(&rebuilt).unwrap(), project_pdf_1_7(&original).unwrap());
    }

    #[test]
    fn unknown_kind_is_an_error_never_a_silent_no_op() {
        let unknown = json_object(vec![("kind", text("not-a-real-kind")), ("params", json_object(vec![]))]);
        assert!(oracle_apply_mutation(&fixture(), &unknown).is_err());
        assert!(oracle_apply_mutation_inverse(&fixture(), &unknown).is_err());
    }

    /// 📇️ The three declarations that must never drift: this module's [`KINDS`], the catalog in
    /// `🔣️component.json`, and the `Examples` rows of the case that claims it.
    #[test]
    fn kinds_matches_the_catalog_and_every_feature_row() {
        let manifest = include_str!("🔣️.json");
        let feature = include_str!("../../../../../🧪️tests/mutate-pdf-1-7/🥒️.feature");
        for kind in KINDS {
            assert!(manifest.contains(&format!("\"{kind}\"")), "the pdf-1-7-base catalog is missing {kind:?}");
            assert!(feature.contains(&format!("| {kind} ")) || feature.contains(&format!("| {kind}\n")), "the feature declares no Examples row for {kind:?}");
        }
        assert_eq!(KINDS.len(), 16, "the pdf-1-7-base vocabulary declares sixteen direct kinds");
    }
}
//#endregion 🧪️Tests
