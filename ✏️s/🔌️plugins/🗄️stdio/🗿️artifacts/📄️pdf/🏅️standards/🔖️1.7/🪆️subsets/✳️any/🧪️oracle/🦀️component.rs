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
//! @see ../🧪️oracle/🔣️component.json — the mutation catalog this module is measured against.
//! @see ../🧬️schema/🧬️mutations/🦀️component.rs — the mutation vocabulary itself (`PdfMutation::KINDS`).

use semio_repo_test_host::Json;

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
        lopdf::content::Content::decode(&content)
            .map(|decoded| {
                decoded
                    .operations
                    .iter()
                    .filter(|operation| operation.operator == "Tj")
                    .flat_map(|operation| operation.operands.iter())
                    .filter_map(|operand| operand.as_str().ok().map(|bytes| String::from_utf8_lossy(bytes).to_string()))
                    .collect::<Vec<_>>()
                    .join("\n")
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
            "no-mutation" => {}
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
    /// back `{kind, params}` unchanged is not an inverse, and while the case adapter merely
    /// returned the projection it hid two declared kinds -- `set-info` and `set-snapshot`, both
    /// routed away from [`apply_kind`] -- behind it.
    fn inverse_spec(document: &Document, kind: &str, params: &Json) -> Result<Json, String> {
        let spec = |inverse_kind: &str, inverse_params: Json| Json::Object(vec![("kind".to_string(), Json::String(inverse_kind.to_string())), ("params".to_string(), inverse_params)]);
        let obj = |entries: Vec<(&str, Json)>| Json::Object(entries.into_iter().map(|(key, value)| (key.to_string(), value)).collect());
        Ok(match kind {
            "no-mutation" => spec("no-mutation", obj(vec![])),
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
            "set-snapshot" => {
                let mut entries = vec![("declaredVersion", Json::String(document.version.clone()))];
                if let Some(title) = info_entry(document, b"Title") {
                    entries.push(("title", Json::String(title)));
                }
                spec("set-snapshot", obj(entries))
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
                    None => spec("no-mutation", obj(vec![])),
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
                    None => spec("no-mutation", obj(vec![])),
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
                    None => spec("no-mutation", obj(vec![])),
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
                    None => spec("no-mutation", obj(vec![])),
                }
            }
            "move-page" => {
                let from = usize_field(params, "from");
                let len = document.get_pages().len();
                if from >= len {
                    spec("no-mutation", obj(vec![]))
                } else {
                    let clamped_to = usize_field(params, "to").min(len.saturating_sub(1));
                    spec("move-page", obj(vec![("from", Json::Number(clamped_to as f64)), ("to", Json::Number(from as f64))]))
                }
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
    /// `set-snapshot` does NOT: `PdfMutation::SetSnapshot` clones the base snapshot and overrides
    /// `declared_version`/`info.title`, leaving every other field alone, so the oracle must edit
    /// the `/Info` dictionary IN PLACE. `oracle_replace_metadata` replaces the whole dictionary --
    /// correct for `set-info`, whose `PdfInfo { title, author, ..Default::default() }` really does
    /// discard the rest, and wrong for `set-snapshot`, where it silently dropped `/Author`.
    /// Every other kind mutates a freshly loaded [`Document`] directly via [`apply_kind`].
    pub fn apply_mutation(input: &[u8], kind: &str, params: &Json) -> Result<Vec<u8>, String> {
        match kind {
            "" => Err("mutation spec carries no `kind`".to_string()),
            "remove-page" => oracle_delete_page(input, usize_field(params, "index") as u32 + 1),
            "set-info" => oracle_replace_metadata(input, present_string(params, "title").as_deref(), present_string(params, "author").as_deref()),
            "set-snapshot" => {
                let mut document = Document::load_mem(input).map_err(|error| format!("lopdf could not parse the input: {error}"))?;
                if let Some(title) = present_string(params, "title") {
                    set_info_entry(&mut document, "Title", &title);
                }
                if let Some(version) = non_empty(params, "declaredVersion") {
                    document.version = version;
                }
                let mut out = Vec::new();
                document.save_to(&mut out).map_err(|error| format!("lopdf could not save: {error}"))?;
                Ok(out)
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

    /// ↩️ Applies `{kind, params}` and then its computed inverse, in sequence, and returns the
    /// re-serialized result -- the caller compares its projection against the ORIGINAL input's own.
    pub fn apply_mutation_inverse(input: &[u8], kind: &str, params: &Json) -> Result<Vec<u8>, String> {
        let reader = Document::load_mem(input).map_err(|error| format!("lopdf could not parse the input: {error}"))?;
        let inverse = inverse_spec(&reader, kind, params)?;
        let mutated = apply_mutation(input, kind, params)?;
        apply_mutation(&mutated, &inverse.str("kind"), inverse.get("params").unwrap_or(&Json::Null))
    }

    fn non_empty(value: &Json, key: &str) -> Option<String> {
        match value.get(key) {
            Some(Json::String(text)) if !text.is_empty() => Some(text.clone()),
            _ => None,
        }
    }

    /// 🔤️ A spec field that is PRESENT as a string, empty or not. `/Title ()` and an absent
    /// `/Title` are different documents -- this fixture's own `/Info` carries both `/Title ()` and
    /// `/Author ()` -- so an inverse that has to restore an empty metadata value must be able to
    /// ask for one, which [`non_empty`] cannot express.
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

    /// ✍️ Sets ONE `/Info` entry in place, leaving every other metadata field intact -- what
    /// `PdfMutation::SetSnapshot` models, unlike `set-info`, which replaces the whole dictionary.
    fn set_info_entry(document: &mut Document, key: &str, value: &str) {
        match document.trailer.get(b"Info").ok().cloned() {
            Some(Object::Reference(id)) => {
                if let Ok(dictionary) = document.get_object_mut(id).and_then(Object::as_dict_mut) {
                    dictionary.set(key, Object::string_literal(value));
                }
            }
            Some(Object::Dictionary(mut dictionary)) => {
                dictionary.set(key, Object::string_literal(value));
                document.trailer.set("Info", Object::Dictionary(dictionary));
            }
            _ => {
                let mut dictionary = Dictionary::new();
                dictionary.set(key, Object::string_literal(value));
                let id = document.add_object(Object::Dictionary(dictionary));
                document.trailer.set("Info", Object::Reference(id));
            }
        }
    }

    /// 👁️ This subset's own projection: the shared `document::project_pdf` independent-reader
    /// projection, augmented with each page's resolved `/Rotate` (normative for `set-page-rotation`
    /// but outside the shared projection's own scope, since no other subset needs it -- the fleet
    /// brief's own "do not edit the shared family module's existing functions" rule is what makes
    /// this an addition here rather than a change there).
    pub fn project_pdf_1_7(bytes: &[u8]) -> Result<Json, String> {
        let base = document::project_pdf(bytes)?;
        let reader = Document::load_mem(bytes).map_err(|error| format!("independent reader could not parse the document: {error}"))?;
        let rotations: Vec<i64> = reader.get_pages().into_values().map(|page_id| reader.get_dictionary(page_id).ok().and_then(|dict| dict.get(b"Rotate").ok()).and_then(|value| value.as_i64().ok()).unwrap_or(0)).collect();
        let Json::Object(entries) = base else { return Ok(base) };
        let augmented: Vec<(String, Json)> = entries
            .into_iter()
            .map(|(key, value)| {
                if key != "pages" {
                    return (key, value);
                }
                let Json::Array(pages) = value else { return (key, value) };
                let merged = pages
                    .into_iter()
                    .zip(rotations.iter().chain(std::iter::repeat(&0)))
                    .map(|(page, rotate)| match page {
                        Json::Object(mut fields) => {
                            fields.push(("rotate".to_string(), Json::Number(*rotate as f64)));
                            Json::Object(fields)
                        }
                        other => other,
                    })
                    .collect();
                (key, Json::Array(merged))
            })
            .collect();
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
pub fn project_pdf_1_7(_bytes: &[u8]) -> Result<Json, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}
//#endregion 🔖️Dispatch
