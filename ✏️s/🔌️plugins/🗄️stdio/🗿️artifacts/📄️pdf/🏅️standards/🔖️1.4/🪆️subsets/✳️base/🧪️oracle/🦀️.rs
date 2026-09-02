//! 🔮️ Independent lopdf oracle for the five concrete PDF 1.4 page operations.

use semio_repo_test_host::Json;

//#region 🔖️Vocabulary
pub const KINDS: &[&str] = &["insert-page", "remove-page", "move-page", "resize-page", "replace-page-text"];
//#endregion 🔖️Vocabulary

//#region 🔖️Page
/// 📄️ One page of a target document, in this subset's own vocabulary: a `/MediaBox` extent and the
/// text the page shows. The oracle's `Json` mirror of `PageDoc`.
#[derive(Clone, Debug, PartialEq)]
pub struct OraclePage {
    pub width: f64,
    pub height: f64,
    pub text: String,
}

impl OraclePage {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn to_json(&self) -> Json {
        Json::Object(vec![("width".to_string(), Json::Number(self.width)), ("height".to_string(), Json::Number(self.height)), ("text".to_string(), Json::String(self.text.clone()))])
    }
}
//#endregion 🔖️Page

//#region 🔖️Spec
fn number(params: &Json, key: &str) -> Result<f64, String> {
    match params.get(key) {
        Some(Json::Number(value)) if value.is_finite() => Ok(*value),
        _ => Err(format!("{key} must be a finite number")),
    }
}

fn index(params: &Json, key: &str) -> Result<usize, String> {
    let value = number(params, key)?;
    if value < 0.0 || value.fract() != 0.0 || value >= usize::MAX as f64 {
        return Err(format!("{key} must be an addressable page index"));
    }
    Ok(value as usize)
}

fn page(value: &Json) -> Result<OraclePage, String> {
    let text = match value.get("text") {
        Some(Json::String(text)) => text.clone(),
        _ => return Err("Page text must be a string".into()),
    };
    Ok(OraclePage { width: number(value, "width")?, height: number(value, "height")?, text })
}

fn object(pairs: Vec<(&str, Json)>) -> Json {
    Json::Object(pairs.into_iter().map(|(key, value)| (key.into(), value)).collect())
}
fn spec(kind: &str, params: Json) -> Json {
    object(vec![("kind", Json::String(kind.into())), ("params", params)])
}

fn mutate_pages(pages: &mut Vec<OraclePage>, mutation: &Json) -> Result<(), String> {
    if pages.is_empty() {
        return Err("A PDF page tree must not be empty".into());
    }
    let params = mutation.get("params").ok_or("Missing mutation parameters")?;
    match mutation.str("kind").as_str() {
        "insert-page" => {
            let at = index(params, "index")?;
            if at > pages.len() {
                return Err("Page insertion index is out of bounds".into());
            }
            pages.insert(at, page(params.get("page").ok_or("Missing inserted page")?)?);
        }
        "remove-page" => {
            let at = index(params, "index")?;
            if at >= pages.len() || pages.len() == 1 {
                return Err("Page removal would leave the PDF domain".into());
            }
            pages.remove(at);
        }
        "move-page" => {
            let from = index(params, "from")?;
            let to = index(params, "to")?;
            if from >= pages.len() || to >= pages.len() {
                return Err("Page move is out of bounds".into());
            }
            let moved = pages.remove(from);
            pages.insert(to, moved);
        }
        "resize-page" => {
            let at = index(params, "index")?;
            let target = pages.get_mut(at).ok_or("Page resize is out of bounds")?;
            target.width = number(params, "width")?;
            target.height = number(params, "height")?;
        }
        "replace-page-text" => {
            let at = index(params, "index")?;
            let target = pages.get_mut(at).ok_or("Page text target is out of bounds")?;
            target.text = match params.get("text") {
                Some(Json::String(text)) => text.clone(),
                _ => return Err("Replacement text must be a string".into()),
            };
        }
        other => return Err(format!("Unknown PDF 1.4 mutation {other:?}")),
    }
    Ok(())
}
//#endregion 🔖️Spec

//#region 🔖️Dispatch
/// ▶️ Mutates independently decoded pages and rebuilds them using lopdf.
#[cfg(feature = "oracles")]
pub fn oracle_apply_mutation(input: &[u8], mutation: &Json) -> Result<Vec<u8>, String> {
    let mut pages = independent_pages(input)?;
    mutate_pages(&mut pages, mutation)?;
    build_document(&pages)
}

#[cfg(not(feature = "oracles"))]
pub fn oracle_apply_mutation(_input: &[u8], _mutation: &Json) -> Result<Vec<u8>, String> {
    Err("The oracles feature is disabled".into())
}

/// ↩️ Captures only the fields required by the concrete opposite operation.
#[cfg(feature = "oracles")]
pub fn oracle_inverse_spec(base: &[u8], forward: &Json) -> Result<Json, String> {
    let pages = independent_pages(base)?;
    mutate_pages(&mut pages.clone(), forward)?;
    let params = forward.get("params").ok_or("Missing mutation parameters")?;
    let at = |key| index(params, key);
    Ok(match forward.str("kind").as_str() {
        "insert-page" => spec("remove-page", object(vec![("index", Json::Number(at("index")? as f64))])),
        "remove-page" => {
            let i = at("index")?;
            spec("insert-page", object(vec![("index", Json::Number(i as f64)), ("page", pages[i].to_json())]))
        }
        "move-page" => spec("move-page", object(vec![("from", Json::Number(at("to")? as f64)), ("to", Json::Number(at("from")? as f64))])),
        "resize-page" => {
            let i = at("index")?;
            spec("resize-page", object(vec![("index", Json::Number(i as f64)), ("width", Json::Number(pages[i].width)), ("height", Json::Number(pages[i].height))]))
        }
        "replace-page-text" => {
            let i = at("index")?;
            spec("replace-page-text", object(vec![("index", Json::Number(i as f64)), ("text", Json::String(pages[i].text.clone()))]))
        }
        other => return Err(format!("Unknown inverse kind {other:?}")),
    })
}

#[cfg(not(feature = "oracles"))]
pub fn oracle_inverse_spec(_base: &[u8], _forward: &Json) -> Result<Json, String> {
    Err("The oracles feature is disabled".into())
}

/// 🔁️ Re-serializes the original lopdf object graph without an identity mutation.
#[cfg(feature = "oracles")]
pub fn oracle_round_trip(input: &[u8]) -> Result<Vec<u8>, String> {
    oracles::round_trip(input)
}

#[cfg(not(feature = "oracles"))]
pub fn oracle_round_trip(_input: &[u8]) -> Result<Vec<u8>, String> {
    Err("The oracles feature is disabled".into())
}
//#endregion 🔖️Dispatch

//#region 🔖️IndependentReader
/// 👁️ Every page of the document, in reading order, read through `lopdf`'s own page-tree walk,
/// `/MediaBox` inheritance chain and content-stream decoder — never through this repository's
/// byte-search `decode_pdf`.
#[cfg(feature = "oracles")]
pub fn independent_pages(input: &[u8]) -> Result<Vec<OraclePage>, String> {
    oracles::pages(input)
}

/// 👁️ Projects PDF bytes onto this subset's own semantic shape using `lopdf` as an INDEPENDENT
/// reader, so a producer (oracle or subject) is never checked against its own writing. The shape
/// is exactly what `PdfSnapshot` carries — the page count and, per page, the `/MediaBox` extent
/// and the shown text — and nothing more. The document VERSION is deliberately absent: this
/// snapshot does not retain it, and `%PDF-1.5` (which is what the committed thesis actually
/// declares) surviving a `lopdf` round trip while our writer emits its own `%PDF-1.4` would report
/// a divergence about a field neither producer was asked to carry.
#[cfg(feature = "oracles")]
pub fn project_pdf_1_4(input: &[u8]) -> Result<Json, String> {
    let pages = independent_pages(input)?;
    Ok(Json::Object(vec![("pageCount".to_string(), Json::Number(pages.len() as f64)), ("pages".to_string(), Json::Array(pages.iter().map(OraclePage::to_json).collect()))]))
}

#[cfg(not(feature = "oracles"))]
pub fn project_pdf_1_4(_input: &[u8]) -> Result<Json, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}

#[cfg(not(feature = "oracles"))]
pub fn independent_pages(_input: &[u8]) -> Result<Vec<OraclePage>, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}
//#endregion 🔖️IndependentReader

//#region 🔖️IndependentWriter
/// ✍️ Builds a fresh PDF carrying exactly `pages`, object by object through `lopdf`'s own writer —
/// never by delegating to this repository's own `encode_pdf`. One `/Page` per entry under a single
/// `/Pages` node, each with its own `/MediaBox [0 0 width height]` and a content stream showing
/// that page's text through a simple `/Type1` font, so what a reader recovers is what the snapshot
/// said. A page whose text is empty gets an empty text object rather than a `Tj` of `""`.
#[cfg(feature = "oracles")]
pub fn build_document(pages: &[OraclePage]) -> Result<Vec<u8>, String> {
    oracles::build_document(pages)
}

#[cfg(not(feature = "oracles"))]
pub fn build_document(_pages: &[OraclePage]) -> Result<Vec<u8>, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}
//#endregion 🔖️IndependentWriter

//#region 🔖️Reference
#[cfg(feature = "oracles")]
mod oracles {
    use super::OraclePage;
    use lopdf::{
        content::{Content, Operation},
        dictionary, Document, Object, ObjectId, Stream,
    };

    fn load(input: &[u8]) -> Result<Document, String> {
        Document::load_mem(input).map_err(|error| format!("independent PDF reader could not parse the document: {error}"))
    }

    fn save(document: &mut Document) -> Result<Vec<u8>, String> {
        let mut out = Vec::new();
        document.save_to(&mut out).map_err(|error| format!("independent PDF writer could not save: {error}"))?;
        Ok(out)
    }

    /// 🔁️ The reference's own decode/re-encode: `lopdf` parses the whole file and writes a fresh
    /// one from its own object graph alone.
    pub fn round_trip(input: &[u8]) -> Result<Vec<u8>, String> {
        save(&mut load(input)?)
    }

    /// 📐️ `/MediaBox` resolved along ISO 32000-1 §7.7.3.4's inheritance chain: the leaf's own box
    /// if it has one, else the nearest ancestor's, else US Letter — the default a consumer assumes
    /// when the chain declares none.
    fn media_box(document: &Document, page: ObjectId) -> (f64, f64) {
        let number = |object: &Object| match object {
            Object::Integer(value) => *value as f64,
            Object::Real(value) => *value as f64,
            _ => 0.0,
        };
        let mut node = Some(page);
        let mut seen = 0;
        while let Some(id) = node {
            seen += 1;
            if seen > 64 {
                break;
            }
            let Ok(dictionary) = document.get_dictionary(id) else { break };
            if let Some(value) = dictionary.get(b"MediaBox").ok() {
                let resolved = match value {
                    Object::Reference(target) => document.get_object(*target).ok().and_then(|object| object.as_array().ok()),
                    other => other.as_array().ok(),
                };
                if let Some(items) = resolved {
                    if items.len() == 4 {
                        let values: Vec<f64> = items.iter().map(number).collect();
                        return (values[2] - values[0], values[3] - values[1]);
                    }
                }
            }
            node = dictionary.get(b"Parent").ok().and_then(|value| value.as_reference().ok());
        }
        (612.0, 792.0)
    }

    /// 📝️ One page's shown text: every `Tj`/`'`/`"` operand and every string element of every `TJ`
    /// array, in content-stream order, lossily decoded to UTF-8. ISO 32000-1 §9.4.3's four
    /// text-showing operators, all of them — `'` and `"` take the shown string as their LAST
    /// operand, which is why the scan reads the operand list from the back rather than the front.
    fn shown_text(document: &Document, page: ObjectId) -> Result<String, String> {
        let content = document.get_page_content(page);
        let decoded = Content::decode(&content).map_err(|error| format!("independent reader could not decode a page's content stream: {error}"))?;
        let mut out: Vec<u8> = Vec::new();
        for operation in &decoded.operations {
            match operation.operator.as_str() {
                "Tj" | "'" | "\"" => {
                    if let Some(Object::String(bytes, _)) = operation.operands.iter().rev().find(|operand| matches!(operand, Object::String(..))) {
                        out.extend_from_slice(bytes);
                    }
                }
                "TJ" => {
                    if let Some(Object::Array(items)) = operation.operands.iter().rev().find(|operand| matches!(operand, Object::Array(_))) {
                        for item in items {
                            if let Object::String(bytes, _) = item {
                                out.extend_from_slice(bytes);
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        Ok(String::from_utf8_lossy(&out).into_owned())
    }

    pub fn pages(input: &[u8]) -> Result<Vec<OraclePage>, String> {
        let document = load(input)?;
        let mut out = Vec::new();
        for (_, page) in document.get_pages() {
            let (width, height) = media_box(&document, page);
            out.push(OraclePage { width, height, text: shown_text(&document, page)? });
        }
        if out.is_empty() {
            return Err("independent reader found no page at all".to_string());
        }
        Ok(out)
    }

    pub fn build_document(pages: &[OraclePage]) -> Result<Vec<u8>, String> {
        if pages.is_empty() {
            return Err("independent writer refuses a page tree with no page — ISO 32000-1 §7.7.3.2 gives /Count a lower bound of one".to_string());
        }
        let mut document = Document::with_version("1.4");
        let pages_id = document.new_object_id();
        let font_id = document.add_object(dictionary! { "Type" => "Font", "Subtype" => "Type1", "BaseFont" => "Helvetica" });
        let resources_id = document.add_object(dictionary! { "Font" => dictionary! { "F1" => font_id } });
        let mut kids: Vec<Object> = Vec::new();
        for page in pages {
            let mut operations = vec![Operation::new("BT", vec![])];
            if !page.text.is_empty() {
                operations.push(Operation::new("Tf", vec!["F1".into(), 12.into()]));
                operations.push(Operation::new("Td", vec![72.into(), ((page.height - 72.0) as i64).into()]));
                operations.push(Operation::new("Tj", vec![Object::string_literal(page.text.as_str())]));
            }
            operations.push(Operation::new("ET", vec![]));
            let encoded = Content { operations }.encode().map_err(|error| format!("independent writer could not encode page content: {error}"))?;
            let content_id = document.add_object(Stream::new(dictionary! {}, encoded));
            let media_box: Vec<Object> = vec![0.into(), 0.into(), Object::Real(page.width as f32), Object::Real(page.height as f32)];
            let page_id = document.add_object(dictionary! { "Type" => "Page", "Parent" => pages_id, "Contents" => content_id, "MediaBox" => media_box, "Resources" => resources_id });
            kids.push(page_id.into());
        }
        let count = kids.len() as i64;
        document.objects.insert(pages_id, Object::Dictionary(dictionary! { "Type" => "Pages", "Kids" => kids, "Count" => count }));
        let catalog_id = document.add_object(dictionary! { "Type" => "Catalog", "Pages" => pages_id });
        document.trailer.set("Root", catalog_id);
        save(&mut document)
    }
}
//#endregion 🔖️Reference

//#region 🧪️Tests
#[cfg(all(test, feature = "oracles"))]
mod tests {
    use super::*;
    use semio_repo_test_host::parse_json;

    #[test]
    fn direct_language_neutral_vectors_match_lopdf_and_concrete_inverse() {
        macro_rules! vector {
            ($mutation:literal) => {
                (
                    include_str!(concat!("../🧬️schema/🧬️mutations/", $mutation, "/🧪️tests/round-trips-the-concrete-inverse/🎯️outcome/🔣️.json")),
                    include_str!(concat!("../🧬️schema/🧬️mutations/", $mutation, "/🧪️tests/round-trips-the-concrete-inverse/📸️snapshot/⬅️before/🔣️.json")),
                    include_str!(concat!("../🧬️schema/🧬️mutations/", $mutation, "/🧪️tests/round-trips-the-concrete-inverse/📸️snapshot/➡️after/🔣️.json")),
                    include_str!(concat!("../🧬️schema/🧬️mutations/", $mutation, "/🧪️tests/round-trips-the-concrete-inverse/🔺️diff/🔣️.json")),
                    include_str!(concat!("../🧬️schema/🧬️mutations/", $mutation, "/🧪️tests/round-trips-the-concrete-inverse/🦠️mutation/🔣️.json")),
                )
            };
        }
        let vectors = [vector!("📥️insert-page"), vector!("🗑️remove-page"), vector!("🔀️move-page"), vector!("📐️resize-page"), vector!("📝️replace-page-text")];
        for (outcome, before, after, diff, mutation) in vectors {
            assert_eq!(parse_json(outcome).unwrap().str("status"), "applied");
            let _ = parse_json(diff).unwrap();
            let base = parse_json(before).unwrap();
            let mutation = parse_json(mutation).unwrap();
            let expected = parse_json(after).unwrap();
            let pages = base.array("pages").iter().map(page).collect::<Result<Vec<_>, _>>().unwrap();
            let base = build_document(&pages).unwrap();
            let forward = spec(&mutation.str("mutation"), mutation.get("payload").unwrap().clone());
            let mutated = oracle_apply_mutation(&base, &forward).unwrap();
            let expected = expected.array("pages").iter().map(page).collect::<Result<Vec<_>, _>>().unwrap();
            let actual = independent_pages(&mutated).unwrap();
            assert_eq!(actual.len(), expected.len());
            for (left, right) in actual.iter().zip(&expected) {
                assert!((left.width - right.width).abs() < 0.001 && (left.height - right.height).abs() < 0.001);
                assert_eq!(left.text, right.text);
            }
            let inverse = oracle_inverse_spec(&base, &forward).unwrap();
            let restored = oracle_apply_mutation(&mutated, &inverse).unwrap();
            crate::law::inverse_restores_within(&forward.str("kind"), &project_pdf_1_4(&restored).unwrap(), &project_pdf_1_4(&base).unwrap(), &[], 0.001).unwrap();
        }
    }

    #[test]
    fn every_real_document_feature_row_is_observable_and_invertible() {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️base/📚️examples/🎓️bachelor-thesis/🖼️assets/📄️bachelor-thesis.pdf");
        let base = std::fs::read(path).unwrap();
        assert_eq!(independent_pages(&base).unwrap().len(), 65);
        let feature = include_str!("../../../../../🧪️tests/mutate-pdf-1-4/🥒️.feature");
        let rows = crate::law::feature_rows(feature);
        assert_eq!(rows.len(), KINDS.len());
        for (kind, params) in rows {
            assert!(KINDS.contains(&kind.as_str()));
            let forward = spec(&kind, params);
            let mutated = oracle_apply_mutation(&base, &forward).unwrap();
            crate::law::mutation_is_observable_within(&kind, &project_pdf_1_4(&mutated).unwrap(), &project_pdf_1_4(&base).unwrap(), &[], &[], 0.001).unwrap();
            let restored = oracle_apply_mutation(&mutated, &oracle_inverse_spec(&base, &forward).unwrap()).unwrap();
            crate::law::inverse_restores_within(&kind, &project_pdf_1_4(&restored).unwrap(), &project_pdf_1_4(&base).unwrap(), &[], 0.001).unwrap();
        }
        let rewritten = oracle_round_trip(&base).unwrap();
        assert_ne!(rewritten, base);
        assert_eq!(project_pdf_1_4(&rewritten).unwrap(), project_pdf_1_4(&base).unwrap());
    }
}
//#endregion 🧪️Tests
