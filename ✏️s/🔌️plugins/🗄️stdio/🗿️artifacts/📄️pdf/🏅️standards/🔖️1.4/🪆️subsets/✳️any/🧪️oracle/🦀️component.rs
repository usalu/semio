//! 🔮️ Mutation oracle for this subset — every mutation kind the subset declares, performed by the
//! registered reference implementation so the subject's own mutation has an independent result to
//! be compared against instead of being checked against its own reading.
//!
//! The vocabulary is per SUBSET, not per artifact: two standards of the same format declare
//! different mutations, and a subset that shares an implementation with another reaches it through
//! the shared `document` module rather than by copying it.
//!
//! 🪞️ Wave 7 finding (see the ticket's `📓️` report): this subset's own `PdfSnapshot` is
//! `{schema, page: {width, height, text}}` — a single page, no object graph — and its `decode_pdf`
//! hardcodes `width`/`height` to `612.0`/`792.0` for EVERY input regardless of the real page size
//! (confirmed against the real `🎓️bachelor-thesis` fixture: true `MediaBox [0 0 595.276 841.89]`,
//! never read). `build_single_page_pdf` below deliberately mirrors that same `612×792` constant
//! rather than independently rediscovering each input's true page size, so `project_pdf_1_4`'s
//! `width`/`height` fields are honest about what they check: that the geometry this subset's OWN
//! documented contract promises (a fixed Letter-size default, never the input's real MediaBox)
//! round-trips consistently through apply/inverse — never a claim that `decode_pdf` reads real
//! page geometry, which it does not and is out of this wave's scope to fix. `text` is the one field
//! genuinely read from the input, independently, through `lopdf`'s own content-stream decoder —
//! never through this repository's byte-search `decode_pdf`.
//!
//! @see ../🧪️oracle/🔣️component.json — the mutation catalog this module is measured against.
//! @see ../🧬️schema/🧬️mutations/🦀️component.rs — the mutation vocabulary itself.

use semio_repo_test_host::Json;

//#region 🔖️Dispatch
/// 🦠️ Applies one declared mutation kind to a real artifact and returns the re-serialized bytes.
/// An unrecognised kind is an error, never a silent no-op: a mutation that is quietly skipped
/// reports as a passing test.
#[cfg(feature = "oracles")]
pub fn oracle_apply_mutation(input: &[u8], spec: &Json) -> Result<Vec<u8>, String> {
    match spec.str("kind").as_str() {
        "" => Err("mutation spec carries no `kind`".to_string()),
        "no-mutation" => build_single_page_pdf(&independent_first_text(input)?),
        "set-snapshot" => build_single_page_pdf(&target_text(spec)),
        kind => Err(format!("mutation kind {:?} has no oracle implementation ({} input byte(s))", kind, input.len())),
    }
}

/// 🚫️ Without the `oracles` feature the reference implementation is not linked at all.
#[cfg(not(feature = "oracles"))]
pub fn oracle_apply_mutation(_input: &[u8], _spec: &Json) -> Result<Vec<u8>, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}
//#endregion 🔖️Dispatch

//#region 🔖️Spec
/// 📄️ Reads `params.snapshot.page.text` out of a `set-snapshot` spec — the one field this
/// subset's mutations actually claim to carry end to end.
#[cfg(feature = "oracles")]
fn target_text(spec: &Json) -> String {
    spec.get("params").and_then(|params| params.get("snapshot")).and_then(|snapshot| snapshot.get("page")).map(|page| page.str("text")).unwrap_or_default()
}
//#endregion 🔖️Spec

//#region 🔖️IndependentReader
/// 👁️ Reads the FIRST text-showing operator (`Tj` or `TJ`) on real page 1, through `lopdf`'s own
/// content-stream decoder — never through this repository's byte-search `decode_pdf`. Mirrors what
/// `decode_pdf` claims to retain (the subset's whole `text` contract), read independently.
#[cfg(feature = "oracles")]
pub fn independent_first_text(input: &[u8]) -> Result<String, String> {
    use lopdf::{content::Content, Object};

    let document = lopdf::Document::load_mem(input).map_err(|error| format!("independent reader could not parse the document: {}", error))?;
    let pages = document.get_pages();
    let page_id = *pages.get(&1).ok_or("independent reader found no page 1")?;
    let content = document.get_page_content(page_id);
    let decoded = Content::decode(&content).map_err(|error| format!("independent reader could not decode page 1's content stream: {}", error))?;
    for operation in &decoded.operations {
        match operation.operator.as_str() {
            "Tj" => {
                if let Some(Object::String(bytes, _)) = operation.operands.first() {
                    return Ok(String::from_utf8_lossy(bytes).into_owned());
                }
            }
            "TJ" => {
                if let Some(Object::Array(items)) = operation.operands.first() {
                    if let Some(Object::String(bytes, _)) = items.iter().find(|item| matches!(item, Object::String(..))) {
                        return Ok(String::from_utf8_lossy(bytes).into_owned());
                    }
                }
            }
            _ => {}
        }
    }
    Err("independent reader found no text-showing operator on page 1".to_string())
}

/// 👁️ Projects PDF bytes onto this subset's own thin semantic shape using `lopdf` as an
/// INDEPENDENT reader, so a producer (oracle or subject) is never checked against its own writing.
/// Scoped to `width`/`height`/`text` — everything `PdfSnapshot` itself carries, nothing more.
#[cfg(feature = "oracles")]
pub fn project_pdf_1_4(input: &[u8]) -> Result<Json, String> {
    use lopdf::Object;

    let document = lopdf::Document::load_mem(input).map_err(|error| format!("independent reader could not parse the document: {}", error))?;
    let pages = document.get_pages();
    let page_id = *pages.get(&1).ok_or("independent reader found no page 1")?;
    let dictionary = document.get_dictionary(page_id).map_err(|error| format!("page 1 dictionary unreadable: {}", error))?;
    let number = |object: &Object| -> f64 {
        match object {
            Object::Integer(value) => *value as f64,
            Object::Real(value) => *value as f64,
            _ => 0.0,
        }
    };
    let media_box: Vec<f64> = match dictionary.get(b"MediaBox").ok().and_then(|value| value.as_array().ok()) {
        Some(items) => items.iter().map(number).collect(),
        None => return Err("page 1 carries no MediaBox".to_string()),
    };
    if media_box.len() != 4 {
        return Err(format!("page 1 MediaBox has {} entries, expected 4", media_box.len()));
    }
    let text = independent_first_text(input)?;
    Ok(Json::Object(vec![("width".to_string(), Json::Number(media_box[2] - media_box[0])), ("height".to_string(), Json::Number(media_box[3] - media_box[1])), ("text".to_string(), Json::String(text))]))
}
//#endregion 🔖️IndependentReader

//#region 🔖️IndependentWriter
/// ✍️ Builds a fresh single-page PDF containing exactly `text`, object by object through `lopdf`'s
/// own writer — never by delegating to this repository's own `encode_pdf`. `MediaBox [0 0 612
/// 792]` matches both `decode_pdf`'s hardcoded default (the `no-mutation` case) and this case's own
/// `set-snapshot` Examples row (chosen deliberately so every scenario shares one geometry and the
/// comparison never has to paper over the width/height gap `decode_pdf` cannot close).
#[cfg(feature = "oracles")]
pub fn build_single_page_pdf(text: &str) -> Result<Vec<u8>, String> {
    use lopdf::{
        content::{Content, Operation},
        dictionary, Document, Object, Stream,
    };

    let mut document = Document::with_version("1.4");
    let pages_id = document.new_object_id();
    let font_id = document.add_object(dictionary! { "Type" => "Font", "Subtype" => "Type1", "BaseFont" => "Helvetica" });
    let resources_id = document.add_object(dictionary! { "Font" => dictionary! { "F1" => font_id } });
    let content = Content {
        operations: vec![Operation::new("BT", vec![]), Operation::new("Tf", vec!["F1".into(), 12.into()]), Operation::new("Td", vec![72.into(), 720.into()]), Operation::new("Tj", vec![Object::string_literal(text)]), Operation::new("ET", vec![])],
    };
    let content_id = document.add_object(Stream::new(dictionary! {}, content.encode().map_err(|error| format!("independent writer could not encode page content: {}", error))?));
    let page_id = document.add_object(dictionary! { "Type" => "Page", "Parent" => pages_id, "Contents" => content_id, "MediaBox" => vec![0.into(), 0.into(), 612.into(), 792.into()], "Resources" => resources_id });
    document.objects.insert(pages_id, Object::Dictionary(dictionary! { "Type" => "Pages", "Kids" => vec![page_id.into()], "Count" => 1 }));
    let catalog_id = document.add_object(dictionary! { "Type" => "Catalog", "Pages" => pages_id });
    document.trailer.set("Root", catalog_id);
    let mut out = Vec::new();
    document.save_to(&mut out).map_err(|error| format!("independent writer could not save: {}", error))?;
    Ok(out)
}
//#endregion 🔖️IndependentWriter
