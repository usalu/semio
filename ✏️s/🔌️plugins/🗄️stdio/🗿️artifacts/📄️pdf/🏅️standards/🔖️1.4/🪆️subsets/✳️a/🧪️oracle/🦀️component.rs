//! 🔮️ Mutation oracle for this subset — every CONFORMANCE mutation the `pdf` `1.4`/`✳️a` subset
//! declares, performed by the registered `lopdf` 0.44 reference implementation on a real document,
//! so the subject's own mutation has an independent result to be compared against instead of being
//! checked against its own reading.
//!
//! **What this subset's vocabulary is, and why it is so small.** PDF 1.4's retained snapshot is the
//! document's page TREE — `PageDoc { width, height, text }` per page, with no object graph — and
//! this subset's own `check_pdf_a_conformance` (`../🧬️schema/🦀️component.rs`) says so in as many
//! words: it raises exactly TWO diagnostics, `stdio.pdf.a.text-empty` when the FIRST page's text is
//! blank, and `stdio.pdf.a.schema-gap-unverifiable`, which fires unconditionally on every document
//! and records that full ISO 19005-1 conformance cannot be checked from this schema at all. A
//! vocabulary derived honestly from that checker therefore has exactly ONE movable axis — page 1's
//! extractable text — and the schema-gap axis is not movable by anything, because no mutation can
//! give PDF 1.4's snapshot an object graph it does not have. The page count and every other page
//! come through untouched, which is what the projection's `pageCount` anchor is there to prove. Inventing PDF/A-1 kinds this subset cannot check (encryption,
//! JavaScript, output intents, font embedding — everything the 1.7 `✳️a` subset legitimately
//! declares) would be fabricating a vocabulary for a schema that cannot observe it.
//!
//! This is deliberately NOT the 1.7 `✳️a` subset's vocabulary: same conformance FAMILY, different
//! standard, and the two share not one kind. The distinction is the whole reason a mutation belongs
//! to one subset of one standard rather than to a format.
//!
//! **The reference reads AND writes.** `lopdf` decodes page 1's real content stream through its own
//! content-stream decoder and rewrites it from operators alone, so it is a genuine second producer
//! for the text axis and the scenarios are typed `@mode-differential`.
//!
//! @see ../🧪️oracle/🔣️component.json — the mutation catalog this module is measured against.
//! @see ../🧬️schema/🧬️mutations/🦀️component.rs — the mutation vocabulary itself.
//! @see ../🧬️schema/🦀️component.rs — `check_pdf_a_conformance`, the one axis list this derives from.

use semio_repo_test_host::Json;

//#region 🔖️Vocabulary
/// 🧾️ Kebab-case spelling of every variant this subset's `PdfA1Mutation` declares, in declaration
/// order. The catalog `pdf-1-4-a` is measured against this exact list.
pub const KINDS: &[&str] = &["no-mutation", "set-snapshot", "set-page-text", "clear-page-text"];
//#endregion 🔖️Vocabulary

//#region 🔖️Dispatch
/// 🦠️ Applies one declared mutation kind to a real document and returns the re-serialized bytes. An
/// unrecognised kind is an error, never a silent no-op: a mutation that is quietly skipped reports
/// as a passing test.
#[cfg(feature = "oracles")]
pub fn oracle_apply_mutation(input: &[u8], spec: &Json) -> Result<Vec<u8>, String> {
    let kind = spec.str("kind");
    if !KINDS.contains(&kind.as_str()) {
        return Err(format!("mutation kind {kind:?} is not declared by the pdf-1-4-a catalog"));
    }
    let params = spec.get("params").cloned().unwrap_or(Json::Null);
    match kind.as_str() {
        "no-mutation" => Ok(input.to_vec()),
        "set-snapshot" => match params.str("conformance").as_str() {
            "stamped" => oracles::write_page_text(input, CONFORMANT_TEXT),
            "stripped" => oracles::write_page_text(input, ""),
            other => Err(format!("set-snapshot: `conformance` must be \"stamped\" or \"stripped\", got {other:?}")),
        },
        "set-page-text" => {
            let text = params.str("text");
            if text.is_empty() {
                return Err("set-page-text: `text` must be non-empty — clearing the page is what clear-page-text is for, and a row whose parameters make the mutation indistinguishable from its sibling is not a test".to_string());
            }
            oracles::write_page_text(input, &text)
        }
        "clear-page-text" => {
            if oracles::read_page_text(input)?.is_empty() {
                return Err("clear-page-text: page 1 already shows no text — the mutation would be unobservable".to_string());
            }
            oracles::write_page_text(input, "")
        }
        other => Err(format!("mutation kind {other:?} has no oracle implementation")),
    }
}

/// 📝️ The text a class stamp writes. Real content, not a placeholder: `check_pdf_a_conformance`'s
/// only movable axis is "is there extractable text at all", and this is a real sentence a reader
/// would extract.
pub const CONFORMANT_TEXT: &str = "Reuse of load-bearing timber components in Swiss building stock";

/// 🚫️ Without the `oracles` feature the reference implementation is not linked at all.
#[cfg(not(feature = "oracles"))]
pub fn oracle_apply_mutation(_input: &[u8], _spec: &Json) -> Result<Vec<u8>, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}

/// ↩️ The undo of `forward`, read out of `base` by the independent implementation alone.
#[cfg(feature = "oracles")]
pub fn oracle_inverse_spec(base: &[u8], forward: &Json) -> Result<Json, String> {
    let previous = oracles::read_page_text(base)?;
    let object = |pairs: Vec<(&str, Json)>| Json::Object(pairs.into_iter().map(|(key, value)| (key.to_string(), value)).collect());
    let spec = |kind: &str, pairs: Vec<(&str, Json)>| object(vec![("kind", Json::String(kind.to_string())), ("params", object(pairs))]);
    Ok(match forward.str("kind").as_str() {
        "no-mutation" => spec("no-mutation", vec![]),
        // 🔁️ `set-snapshot`'s undo is NOT the opposite stamp. The committed thesis already carries
        // real extractable text on page 1, so `stripped` would clear it rather than put it back —
        // the class stamp is bijective only on a document that carries none of what it installs, and
        // this one does. The honest inverse restores the base's OWN text, read out of the base by
        // this same independent implementation, which is exact on any input.
        "set-snapshot" | "set-page-text" | "clear-page-text" => {
            if previous.is_empty() {
                spec("clear-page-text", vec![])
            } else {
                spec("set-page-text", vec![("text", Json::String(previous))])
            }
        }
        other => return Err(format!("no inverse rule for kind {other:?}")),
    })
}

#[cfg(not(feature = "oracles"))]
pub fn oracle_inverse_spec(_base: &[u8], _forward: &Json) -> Result<Json, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}

/// 🔁️ The reference implementation's own decode/re-encode: `lopdf` parses the whole file and writes
/// a fresh one from its own object graph alone.
#[cfg(feature = "oracles")]
pub fn oracle_round_trip(input: &[u8]) -> Result<Vec<u8>, String> {
    oracles::round_trip(input)
}

#[cfg(not(feature = "oracles"))]
pub fn oracle_round_trip(_input: &[u8]) -> Result<Vec<u8>, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}

/// 👁️ Projects document bytes onto exactly the axis `check_pdf_a_conformance` reads: the extractable
/// page text, and the emptiness verdict the checker itself computes from it. `schemaGapUnverifiable`
/// is reported as the constant `true` it genuinely is — the checker raises it unconditionally on
/// every PDF 1.4 document, so recording it is honest bookkeeping of an axis no mutation can move,
/// not a field pretending to carry evidence. Page COUNT is the stability anchor.
#[cfg(feature = "oracles")]
pub fn project_conformance(input: &[u8]) -> Result<Json, String> {
    let text = oracles::read_page_text(input)?;
    Ok(Json::Object(vec![
        ("subset".to_string(), Json::String("a".to_string())),
        ("pageCount".to_string(), Json::Number(oracles::page_count(input)? as f64)),
        ("pageText".to_string(), Json::String(text.clone())),
        ("textEmpty".to_string(), Json::Bool(text.trim().is_empty())),
        ("schemaGapUnverifiable".to_string(), Json::Bool(true)),
    ]))
}

#[cfg(not(feature = "oracles"))]
pub fn project_conformance(_input: &[u8]) -> Result<Json, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}
//#endregion 🔖️Dispatch

//#region 🔖️Reference
#[cfg(feature = "oracles")]
mod oracles {
    use lopdf::{content::Content, Document, Object};

    fn load(input: &[u8]) -> Result<Document, String> {
        Document::load_mem(input).map_err(|error| format!("independent PDF reader could not parse the document: {error}"))
    }

    fn save(document: &mut Document) -> Result<Vec<u8>, String> {
        let mut out = Vec::new();
        document.save_to(&mut out).map_err(|error| format!("independent PDF writer could not save: {error}"))?;
        Ok(out)
    }

    pub fn round_trip(input: &[u8]) -> Result<Vec<u8>, String> {
        save(&mut load(input)?)
    }

    pub fn page_count(input: &[u8]) -> Result<usize, String> {
        Ok(load(input)?.get_pages().len())
    }

    /// 👁️ Every `Tj`/`TJ` operand on real page 1, concatenated — read through `lopdf`'s own
    /// content-stream decoder, never through this repository's byte-search `decode_pdf`. An empty
    /// string is a real answer here (a page that shows no text), not an error, because "no
    /// extractable text" is precisely the state `stdio.pdf.a.text-empty` reports.
    pub fn read_page_text(input: &[u8]) -> Result<String, String> {
        let document = load(input)?;
        let page = *document.get_pages().get(&1).ok_or("independent reader found no page 1")?;
        let content = document.get_page_content(page);
        let decoded = Content::decode(&content).map_err(|error| format!("independent reader could not decode page 1's content stream: {error}"))?;
        let mut out = String::new();
        for operation in &decoded.operations {
            match operation.operator.as_str() {
                "Tj" => {
                    if let Some(Object::String(bytes, _)) = operation.operands.first() {
                        out.push_str(&String::from_utf8_lossy(bytes));
                    }
                }
                "TJ" => {
                    if let Some(Object::Array(items)) = operation.operands.first() {
                        for item in items {
                            if let Object::String(bytes, _) = item {
                                out.push_str(&String::from_utf8_lossy(bytes));
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        Ok(out)
    }

    /// ✍️ Replaces real page 1's content stream with one built from operators alone — `BT … Tj … ET`
    /// for text, a bare `BT ET` for the empty case. Every other page is left exactly as it was, so
    /// the document stays the real 65-page thesis rather than becoming a synthetic one-pager.
    pub fn write_page_text(input: &[u8], text: &str) -> Result<Vec<u8>, String> {
        use lopdf::content::Operation;
        let mut document = load(input)?;
        let page = *document.get_pages().get(&1).ok_or("independent reader found no page 1")?;
        let mut operations = vec![Operation::new("BT", vec![])];
        if !text.is_empty() {
            operations.push(Operation::new("Tf", vec!["F1".into(), 12.into()]));
            operations.push(Operation::new("Td", vec![72.into(), 720.into()]));
            operations.push(Operation::new("Tj", vec![Object::string_literal(text)]));
        }
        operations.push(Operation::new("ET", vec![]));
        let encoded = Content { operations }.encode().map_err(|error| format!("independent writer could not encode page content: {error}"))?;
        document.change_page_content(page, encoded).map_err(|error| format!("independent writer could not replace page 1's content: {error}"))?;
        save(&mut document)
    }
}
//#endregion 🔖️Reference

//#region 🧪️Tests
#[cfg(all(test, feature = "oracles"))]
mod tests {
    use super::*;

    const FIXTURE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/📚️examples/🎓️bachelor-thesis/🖼️assets/📄️bachelor-thesis.pdf");

    fn json_object(pairs: Vec<(&str, Json)>) -> Json {
        Json::Object(pairs.into_iter().map(|(key, value)| (key.to_string(), value)).collect())
    }

    fn params_for(kind: &str) -> Json {
        match kind {
            "no-mutation" => json_object(vec![]),
            "set-snapshot" => json_object(vec![("conformance", Json::String("stamped".to_string()))]),
            "set-page-text" => json_object(vec![("text", Json::String("An abstract a reader can actually extract".to_string()))]),
            "clear-page-text" => json_object(vec![]),
            other => panic!("no test parameters for kind {other:?}"),
        }
    }

    fn spec(kind: &str) -> Json {
        json_object(vec![("kind", Json::String(kind.to_string())), ("params", params_for(kind))])
    }

    fn fixture() -> Vec<u8> {
        std::fs::read(FIXTURE).expect("the committed bachelor-thesis document")
    }

    #[test]
    fn every_declared_kind_is_observable_and_its_inverse_restores_the_document() {
        let base = fixture();
        let base_projection = project_conformance(&base).expect("the base projects");
        for kind in KINDS {
            let forward = spec(kind);
            let mutated = oracle_apply_mutation(&base, &forward).unwrap_or_else(|error| panic!("{kind}: {error}"));
            let mutated_projection = project_conformance(&mutated).unwrap_or_else(|error| panic!("{kind}: {error}"));
            if *kind != "no-mutation" {
                assert_ne!(mutated_projection, base_projection, "{kind} must be observable in the conformance projection");
            }
            let undo = oracle_inverse_spec(&base, &forward).unwrap_or_else(|error| panic!("{kind}: inverse spec: {error}"));
            let restored = oracle_apply_mutation(&mutated, &undo).unwrap_or_else(|error| panic!("{kind}: inverse: {error}"));
            assert_eq!(project_conformance(&restored).unwrap(), base_projection, "{kind}: undoing the mutation must restore the conformance projection");
        }
    }

    #[test]
    fn a_kind_the_1_7_a_subset_declares_is_refused_here() {
        let spec = json_object(vec![("kind", Json::String("set-output-intent".to_string())), ("params", json_object(vec![]))]);
        assert!(oracle_apply_mutation(&fixture(), &spec).is_err(), "PDF 1.4's snapshot has no object graph; an output-intent kind belongs to the 1.7 ✳️a subset and must be refused here");
    }

    #[test]
    fn a_parameter_that_would_make_the_mutation_a_no_op_is_refused() {
        let spec = json_object(vec![("kind", Json::String("set-page-text".to_string())), ("params", json_object(vec![("text", Json::String(String::new()))]))]);
        assert!(oracle_apply_mutation(&fixture(), &spec).is_err());
    }

    #[test]
    fn the_round_trip_is_projection_stable_and_not_a_byte_passthrough() {
        let input = fixture();
        let rebuilt = oracle_round_trip(&input).expect("the reference re-serializes the document");
        assert_ne!(rebuilt, input);
        assert_eq!(project_conformance(&rebuilt).unwrap(), project_conformance(&input).unwrap());
    }
}
//#endregion 🧪️Tests
