//! 🔮️ Mutation oracle for this subset — every CONFORMANCE mutation the `pdf` `1.4`/`✳️x` subset
//! declares, performed by the registered `lopdf` 0.44 reference implementation on a real document,
//! so the subject's own mutation has an independent result to be compared against instead of being
//! checked against its own reading.
//!
//! **What this subset's vocabulary is, and why it shares no kind with its sibling.** PDF 1.4's
//! retained snapshot is the document's page TREE — `PageDoc { width, height, text }` per page, with
//! no object graph — and this subset's own `check_pdf_x_conformance` (`../🧬️schema/🦀️component.rs`)
//! raises exactly TWO diagnostics: `stdio.pdf.x.degenerate-page-size` when the FIRST page's width
//! or height is not strictly positive, and `stdio.pdf.x.schema-gap-unverifiable`, which fires
//! unconditionally and records that full ISO 15930 conformance cannot be checked from this schema
//! at all. The one movable axis is therefore page 1's GEOMETRY, and this vocabulary is that axis
//! and nothing else: the page count and every other page's box come through untouched, which is
//! what the projection's `pageCount` anchor is there to prove.
//!
//! That is the whole distance between this subset and `1.4/✳️a`, whose checker reads page 1's text
//! and never looks at the geometry, and whose vocabulary is the text axis and nothing else. Two subsets
//! of one standard over one snapshot type, sharing not a single kind, because their checkers read
//! different fields of it. Neither shares anything with `1.7/✳️x`, whose object graph lets it police
//! `/TrimBox` per page, output intents, encryption and font embedding — none of which PDF 1.4's
//! snapshot can observe, and inventing them here would be fabricating a vocabulary for a schema that
//! cannot check it.
//!
//! **The reference reads AND writes.** `lopdf` reads real page 1's `/MediaBox` and rewrites it, so
//! it is a genuine second producer for the geometry axis and the scenarios are `@mode-differential`.
//!
//! @see ../🧪️oracle/🔣️component.json — the mutation catalog this module is measured against.
//! @see ../🧬️schema/🧬️mutations/🦀️component.rs — the mutation vocabulary itself.
//! @see ../🧬️schema/🦀️component.rs — `check_pdf_x_conformance`, the one axis list this derives from.

use semio_repo_test_host::Json;

//#region 🔖️Vocabulary
/// 🧾️ Kebab-case spelling of every variant this subset's `PdfX1Mutation` declares, in declaration
/// order. The catalog `pdf-1-4-x` is measured against this exact list.
pub const KINDS: &[&str] = &["no-mutation", "set-snapshot", "set-page-size", "collapse-page-size"];
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
    let kind = spec.str("kind");
    if !KINDS.contains(&kind.as_str()) {
        return Err(format!("mutation kind {kind:?} is not declared by the pdf-1-4-x catalog"));
    }
    let params = spec.get("params").cloned().unwrap_or(Json::Null);
    let number = |key: &str| match params.get(key) {
        Some(Json::Number(value)) => Some(*value),
        _ => None,
    };
    match kind.as_str() {
        "no-mutation" => Ok(input.to_vec()),
        "set-snapshot" => match params.str("conformance").as_str() {
            "stamped" => oracles::write_media_box(input, CONFORMANT_WIDTH, CONFORMANT_HEIGHT),
            "stripped" => oracles::write_media_box(input, 0.0, CONFORMANT_HEIGHT),
            other => Err(format!("set-snapshot: `conformance` must be \"stamped\" or \"stripped\", got {other:?}")),
        },
        "set-page-size" => {
            let width = number("width").ok_or("set-page-size: `width` must be a number")?;
            let height = number("height").ok_or("set-page-size: `height` must be a number")?;
            if !(width > 0.0 && height > 0.0) {
                return Err("set-page-size: both dimensions must be strictly positive — collapsing the page is what collapse-page-size is for, and a row whose parameters make the mutation indistinguishable from its sibling is not a test".to_string());
            }
            let (current_width, current_height) = oracles::read_media_box(input)?;
            if (current_width - width).abs() < DIMENSION_TOLERANCE && (current_height - height).abs() < DIMENSION_TOLERANCE {
                return Err(format!("set-page-size: page 1 already measures {width}×{height} — the mutation would be unobservable"));
            }
            oracles::write_media_box(input, width, height)
        }
        "collapse-page-size" => {
            let (current_width, _) = oracles::read_media_box(input)?;
            if current_width <= 0.0 {
                return Err("collapse-page-size: page 1's width is already not strictly positive — the mutation would be unobservable".to_string());
            }
            let (_, height) = oracles::read_media_box(input)?;
            oracles::write_media_box(input, 0.0, height)
        }
        other => Err(format!("mutation kind {other:?} has no oracle implementation")),
    }
}

#[cfg(not(feature = "oracles"))]
pub fn oracle_apply_mutation(_input: &[u8], _spec: &Json) -> Result<Vec<u8>, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}

/// ↩️ The undo of `forward`, read out of `base` by the independent implementation alone.
#[cfg(feature = "oracles")]
pub fn oracle_inverse_spec(base: &[u8], forward: &Json) -> Result<Json, String> {
    let (width, height) = oracles::read_media_box(base)?;
    let object = |pairs: Vec<(&str, Json)>| Json::Object(pairs.into_iter().map(|(key, value)| (key.to_string(), value)).collect());
    let spec = |kind: &str, pairs: Vec<(&str, Json)>| object(vec![("kind", Json::String(kind.to_string())), ("params", object(pairs))]);
    Ok(match forward.str("kind").as_str() {
        "no-mutation" => spec("no-mutation", vec![]),
        "set-snapshot" => {
            let back = if forward.get("params").map(|params| params.str("conformance")).unwrap_or_default() == "stamped" { "stripped" } else { "stamped" };
            spec("set-snapshot", vec![("conformance", Json::String(back.to_string()))])
        }
        "set-page-size" | "collapse-page-size" => {
            if width > 0.0 && height > 0.0 {
                spec("set-page-size", vec![("width", Json::Number(width)), ("height", Json::Number(height))])
            } else {
                spec("collapse-page-size", vec![])
            }
        }
        other => return Err(format!("no inverse rule for kind {other:?}")),
    })
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

    const FIXTURE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/📚️examples/🎓️bachelor-thesis/🖼️assets/📄️bachelor-thesis.pdf");

    fn json_object(pairs: Vec<(&str, Json)>) -> Json {
        Json::Object(pairs.into_iter().map(|(key, value)| (key.to_string(), value)).collect())
    }

    fn params_for(kind: &str) -> Json {
        match kind {
            "no-mutation" => json_object(vec![]),
            "set-snapshot" => json_object(vec![("conformance", Json::String("stripped".to_string()))]),
            "set-page-size" => json_object(vec![("width", Json::Number(419.528)), ("height", Json::Number(595.276))]),
            "collapse-page-size" => json_object(vec![]),
            other => panic!("no test parameters for kind {other:?}"),
        }
    }

    fn spec(kind: &str) -> Json {
        json_object(vec![("kind", Json::String(kind.to_string())), ("params", params_for(kind))])
    }

    fn fixture() -> Vec<u8> {
        std::fs::read(FIXTURE).expect("the committed bachelor-thesis document")
    }

    /// 📐️ The committed thesis really is typeset at A4, which is what makes the A5 row of the
    /// feature's Examples table an observable change rather than a coincidence.
    #[test]
    fn the_real_fixture_is_a4_and_the_projection_reads_its_true_box() {
        let projection = project_conformance(&fixture()).expect("the fixture projects");
        let value = |key: &str| match projection.get(key) {
            Some(Json::Number(number)) => *number,
            other => panic!("{key} is {other:?}"),
        };
        assert!((value("width") - 595.276).abs() < DIMENSION_TOLERANCE, "width was {}", value("width"));
        assert!((value("height") - 841.89).abs() < DIMENSION_TOLERANCE, "height was {}", value("height"));
        assert_ne!(value("width"), 612.0, "A4 is not US Letter — a projection reporting 612 would be reading a default rather than this document's own /MediaBox");
        assert_eq!(value("pageCount"), 65.0, "the committed thesis is 65 pages, and this axis anchors every scenario against a producer that silently drops pages");
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
    fn the_sibling_subsets_text_vocabulary_is_refused_here() {
        let spec = json_object(vec![("kind", Json::String("set-page-text".to_string())), ("params", json_object(vec![]))]);
        assert!(oracle_apply_mutation(&fixture(), &spec).is_err(), "the text axis belongs to 1.4/✳️a, whose checker reads it; this subset's checker never does");
    }

    #[test]
    fn a_parameter_that_would_make_the_mutation_a_no_op_is_refused() {
        let spec = json_object(vec![("kind", Json::String("set-page-size".to_string())), ("params", json_object(vec![("width", Json::Number(595.276)), ("height", Json::Number(841.89))]))]);
        assert!(oracle_apply_mutation(&fixture(), &spec).is_err(), "setting the size the page already has is not a test");
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
