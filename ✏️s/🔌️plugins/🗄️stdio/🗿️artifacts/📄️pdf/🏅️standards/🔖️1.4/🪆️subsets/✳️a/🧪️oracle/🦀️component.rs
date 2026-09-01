//! 🔮️ Independent lopdf oracle for PDF 1.4/A's concrete first-page operations.

use semio_repo_test_host::Json;

//#region 🔖️Vocabulary
/// 🧾️ Kebab-case spelling of every variant this subset's `PdfA1Mutation` declares, in declaration
/// order. The catalog `pdf-1-4-a` is measured against this exact list.
pub const KINDS: &[&str] = &["set-page-text", "clear-page-text"];
//#endregion 🔖️Vocabulary

//#region 🔖️Dispatch
/// 🦠️ Applies one declared mutation kind to a real document and returns the re-serialized bytes. An
/// unrecognised kind is an error, never a silent no-op: a mutation that is quietly skipped reports
/// as a passing test.
#[cfg(feature = "oracles")]
pub fn oracle_apply_mutation(input: &[u8], spec: &Json) -> Result<Vec<u8>, String> {
    let params = spec.get("params").ok_or("Missing mutation parameters")?;
    match spec.str("kind").as_str() {
        "set-page-text" => match params.get("text") {
            Some(Json::String(text)) => oracles::write_page_text(input, text),
            _ => Err("Page text must be a string".into()),
        },
        "clear-page-text" => oracles::write_page_text(input, ""),
        other => Err(format!("Unknown PDF/A-1 operation {other:?}")),
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
    if !KINDS.contains(&forward.str("kind").as_str()) {
        return Err("Unknown inverse kind".into());
    }
    let previous = oracles::read_page_text(base)?;
    Ok(Json::Object(vec![("kind".into(), Json::String("set-page-text".into())), ("params".into(), Json::Object(vec![("text".into(), Json::String(previous))]))]))
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

    #[test]
    fn every_real_document_feature_row_is_observable_and_invertible() {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️base/📚️examples/🎓️bachelor-thesis/🖼️assets/📄️bachelor-thesis.pdf");
        let base = std::fs::read(path).unwrap();
        let feature = include_str!("../../../../../🧪️tests/mutate-pdf-1-4-a/🥒️.feature");
        let rows = crate::law::feature_rows(feature);
        assert_eq!(rows.len(), KINDS.len());
        for (kind, params) in rows {
            assert!(KINDS.contains(&kind.as_str()));
            let forward = Json::Object(vec![("kind".into(), Json::String(kind.clone())), ("params".into(), params)]);
            let mutated = oracle_apply_mutation(&base, &forward).unwrap();
            crate::law::mutation_is_observable_within(&kind, &project_conformance(&mutated).unwrap(), &project_conformance(&base).unwrap(), &[], &[], 0.001).unwrap();
            let restored = oracle_apply_mutation(&mutated, &oracle_inverse_spec(&base, &forward).unwrap()).unwrap();
            crate::law::inverse_restores_within(&kind, &project_conformance(&restored).unwrap(), &project_conformance(&base).unwrap(), &[], 0.001).unwrap();
        }
        let rewritten = oracle_round_trip(&base).unwrap();
        assert_ne!(rewritten, base);
        assert_eq!(project_conformance(&rewritten).unwrap(), project_conformance(&base).unwrap());
    }

    #[test]
    fn language_neutral_direct_vectors_match_independent_lopdf() {
        use crate::artifacts::pdf::standards::v1_4::subsets::base::{build_document, independent_pages, OraclePage};
        use semio_repo_test_host::parse_json;
        let vectors =
            [include_str!("../🧬️schema/🧬️mutations/📝️set-page-text/🧪️tests/round-trips-the-concrete-inverse/🔣️component.json"), include_str!("../🧬️schema/🧬️mutations/🧹️clear-page-text/🧪️tests/round-trips-the-concrete-inverse/🔣️component.json")];
        for text in vectors {
            let fixture = parse_json(text).unwrap();
            let to_page = |p: &Json| OraclePage {
                width: match p.get("width").unwrap() {
                    Json::Number(v) => *v,
                    _ => panic!("width"),
                },
                height: match p.get("height").unwrap() {
                    Json::Number(v) => *v,
                    _ => panic!("height"),
                },
                text: p.str("text"),
            };
            let pages: Vec<_> = fixture.get("base").unwrap().array("pages").iter().map(to_page).collect();
            let base = build_document(&pages).unwrap();
            let wire = fixture.get("mutation").unwrap();
            let forward = Json::Object(vec![("kind".into(), Json::String(wire.str("mutation"))), ("params".into(), wire.get("payload").unwrap().clone())]);
            let mutated = oracle_apply_mutation(&base, &forward).unwrap();
            let expected: Vec<_> = fixture.get("expected").unwrap().array("pages").iter().map(to_page).collect();
            let actual = independent_pages(&mutated).unwrap();
            assert_eq!(actual.len(), expected.len());
            for (a, b) in actual.iter().zip(&expected) {
                assert!((a.width - b.width).abs() < 0.001 && (a.height - b.height).abs() < 0.001);
                assert_eq!(a.text, b.text);
            }
            let restored = oracle_apply_mutation(&mutated, &oracle_inverse_spec(&base, &forward).unwrap()).unwrap();
            assert_eq!(independent_pages(&restored).unwrap(), independent_pages(&base).unwrap());
        }
    }
}
//#endregion 🧪️Tests
