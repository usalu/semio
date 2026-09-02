//! 🔮️ Independent lopdf oracle for PDF 1.4/X's concrete first-page operations.

use semio_repo_test_host::Json;

//#region 🔖️Vocabulary
/// 🧾️ Kebab-case spelling of every variant this subset's `PdfX1Mutation` declares, in declaration
/// order. The catalog `pdf-1-4-x` is measured against this exact list.
pub const KINDS: &[&str] = &["set-page-size", "collapse-page-size"];
//#endregion 🔖️Vocabulary

//#region 🔖️Dispatch
/// 📐️ The geometry a class stamp writes: ISO 216 A4 in PDF user-space units, the real trim size the
/// committed thesis is typeset at, rather than a made-up number.
pub const CONFORMANT_WIDTH: f64 = 595.276;
pub const CONFORMANT_HEIGHT: f64 = 841.89;

/// 📏️ How close two page dimensions have to be to count as the same page. PDF stores reals as
/// single-precision decimals and `lopdf` hands them back as `f32`, so a box written as `595.276`
/// reads back as `595.2760009765625` — an exact comparison would call that a different page size and
/// let a genuinely unobservable row through the observability guard. A thousandth of a user-space
/// unit is well under a typesetter's tolerance and well over the representation error.
pub const DIMENSION_TOLERANCE: f64 = 1e-3;

/// 🦠️ Applies one declared mutation kind to a real document and returns the re-serialized bytes. An
/// unrecognised kind is an error, never a silent no-op.
#[cfg(feature = "oracles")]
pub fn oracle_apply_mutation(input: &[u8], spec: &Json) -> Result<Vec<u8>, String> {
    let params = spec.get("params").ok_or("Missing mutation parameters")?;
    let number = |key| match params.get(key) {
        Some(Json::Number(value)) if value.is_finite() => Ok(*value),
        _ => Err(format!("{key} must be a finite number")),
    };
    match spec.str("kind").as_str() {
        "set-page-size" => oracles::write_media_box(input, number("width")?, number("height")?),
        "collapse-page-size" => {
            let (_, height) = oracles::read_media_box(input)?;
            oracles::write_media_box(input, 0.0, height)
        }
        other => Err(format!("Unknown PDF/X-1 operation {other:?}")),
    }
}

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
    let (width, height) = oracles::read_media_box(base)?;
    Ok(Json::Object(vec![("kind".into(), Json::String("set-page-size".into())), ("params".into(), Json::Object(vec![("width".into(), Json::Number(width)), ("height".into(), Json::Number(height))]))]))
}

#[cfg(not(feature = "oracles"))]
pub fn oracle_inverse_spec(_base: &[u8], _forward: &Json) -> Result<Json, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}

/// 🔁️ The reference implementation's own decode/re-encode.
#[cfg(feature = "oracles")]
pub fn oracle_round_trip(input: &[u8]) -> Result<Vec<u8>, String> {
    oracles::round_trip(input)
}

#[cfg(not(feature = "oracles"))]
pub fn oracle_round_trip(_input: &[u8]) -> Result<Vec<u8>, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}

/// 👁️ Projects document bytes onto exactly the axis `check_pdf_x_conformance` reads: page 1's real
/// `/MediaBox` extents and the degeneracy verdict the checker itself computes from them.
/// `schemaGapUnverifiable` is reported as the constant `true` it genuinely is — the checker raises it
/// unconditionally on every PDF 1.4 document, so recording it is honest bookkeeping of an axis no
/// mutation can move, not a field pretending to carry evidence.
///
/// 📐️ The width and height here are the document's TRUE `/MediaBox` extents, read from the bytes.
/// That is what this axis measured against until the first full differential run of ticket
/// 26/08/23/END-TO-END-TESTING-REFACTOR: the `✳️any` subset's `decode_pdf` hardcoded `612×792` for
/// every input and never read a real page's box, so this case scored 0 of 9 on a document typeset
/// at A4. It reads the real page tree now, and `pageCount` is here for the other half of the same
/// defect — the old codec returned a ONE-page snapshot for the committed 65-page thesis, so a
/// producer that silently drops pages fails this projection on its first field.
#[cfg(feature = "oracles")]
pub fn project_conformance(input: &[u8]) -> Result<Json, String> {
    let (width, height) = oracles::read_media_box(input)?;
    Ok(Json::Object(vec![
        ("subset".to_string(), Json::String("x".to_string())),
        ("pageCount".to_string(), Json::Number(oracles::page_count(input)? as f64)),
        ("width".to_string(), Json::Number(width)),
        ("height".to_string(), Json::Number(height)),
        ("degeneratePageSize".to_string(), Json::Bool(!(width > 0.0 && height > 0.0))),
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
    use lopdf::{Document, Object};

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

    /// 📐️ Page 1's real `/MediaBox` extents, resolved through the page-tree inheritance `lopdf`'s own
    /// `get_dictionary_deref` walks, so a document that declares the box on `/Pages` rather than on
    /// the leaf still measures correctly.
    pub fn read_media_box(input: &[u8]) -> Result<(f64, f64), String> {
        let document = load(input)?;
        let page = *document.get_pages().get(&1).ok_or("independent reader found no page 1")?;
        let value = document.get_dictionary(page).map_err(|error| format!("page 1 dictionary unreadable: {error}"))?.get(b"MediaBox").map_err(|_| "page 1 carries no MediaBox".to_string())?;
        let items = match value {
            Object::Reference(id) => document.get_object(*id).and_then(Object::as_array).map_err(|error| format!("page 1 MediaBox unreadable: {error}"))?,
            other => other.as_array().map_err(|error| format!("page 1 MediaBox is not an array: {error}"))?,
        };
        if items.len() != 4 {
            return Err(format!("page 1 MediaBox has {} entries, expected 4", items.len()));
        }
        let numbers: Vec<f64> = items.iter().map(|item| item.as_float().unwrap_or(0.0) as f64).collect();
        Ok((numbers[2] - numbers[0], numbers[3] - numbers[1]))
    }

    /// ✍️ Rewrites real page 1's `/MediaBox` to `width`×`height` anchored at the origin. Every other
    /// page is left exactly as it was, so the document stays the real 65-page thesis.
    pub fn write_media_box(input: &[u8], width: f64, height: f64) -> Result<Vec<u8>, String> {
        let mut document = load(input)?;
        let page = *document.get_pages().get(&1).ok_or("independent reader found no page 1")?;
        let media_box = Object::Array(vec![Object::Real(0.0), Object::Real(0.0), Object::Real(width as f32), Object::Real(height as f32)]);
        document.get_dictionary_mut(page).map_err(|error| format!("page 1 dictionary unreadable: {error}"))?.set("MediaBox", media_box);
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
        let feature = include_str!("../../../../../🧪️tests/mutate-pdf-1-4-x/🥒️.feature");
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
            [include_str!("../🧬️schema/🧬️mutations/📐️set-page-size/🧪️tests/round-trips-the-concrete-inverse/🔣️.json"), include_str!("../🧬️schema/🧬️mutations/📉️collapse-page-size/🧪️tests/round-trips-the-concrete-inverse/🔣️.json")];
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
