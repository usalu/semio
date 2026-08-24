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

//#region 🔖️Vocabulary
/// 🧾️ Kebab-case spelling of every variant this subset's `PdfMutation` declares, in declaration
/// order. Two, and the thinness is the vocabulary's, not the case's: `../🧬️schema/🧬️mutations/
/// 🦀️component.rs` really has exactly `NoMutation` and `SetSnapshot`, because this standard's whole
/// snapshot is one page with a width, a height and a text. Declared here rather than in the case
/// adapter so the adapter, this module's own tests and the manifest all read ONE list.
pub const KINDS: &[&str] = &["no-mutation", "set-snapshot"];
//#endregion 🔖️Vocabulary

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

//#region 🧪️Tests
#[cfg(all(test, feature = "oracles"))]
mod tests {
    use super::*;

    /// 🧫️ The real committed document `mutate-pdf-1-4` runs on — the 6.3 MB, 65-page LaTeX
    /// bachelor thesis this standard's own examples directory carries, true `MediaBox
    /// [0 0 595.276 841.89]`.
    const FIXTURE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/📚️examples/🎓️bachelor-thesis/🖼️assets/📄️bachelor-thesis.pdf");

    /// 📄️ The `set-snapshot` Examples row `../../../../../🧪️tests/mutate-pdf-1-4/component.feature`
    /// carries, so a failure here and a failure there have the same cause.
    const REPLACEMENT_TEXT: &str = "Wave seven replaced this page.";

    fn json_object(pairs: Vec<(&str, Json)>) -> Json {
        Json::Object(pairs.into_iter().map(|(key, value)| (key.to_string(), value)).collect())
    }

    fn spec(kind: &str) -> Json {
        let params = match kind {
            "no-mutation" => json_object(vec![]),
            "set-snapshot" => json_object(vec![(
                "snapshot",
                json_object(vec![("schema", Json::String("s.stdio.pdf".to_string())), ("page", json_object(vec![("width", Json::Number(612.0)), ("height", Json::Number(792.0)), ("text", Json::String(REPLACEMENT_TEXT.to_string()))]))]),
            )]),
            other => panic!("no test parameters for kind {other:?}"),
        };
        json_object(vec![("kind", Json::String(kind.to_string())), ("params", params)])
    }

    fn fixture() -> Vec<u8> {
        std::fs::read(FIXTURE).expect("the committed bachelor-thesis document")
    }

    /// ⚖️ The two laws `mutate-pdf-1-4`'s adapter asserts in role, proven here against the real
    /// document without the runner.
    ///
    /// The base is the reference's OWN `no-mutation` output, never the committed input's
    /// projection. That is the whole honesty of this subset's case: the oracle is a
    /// rebuild-from-text writer pinning `MediaBox [0 0 612 792]`, mirroring `decode_pdf`, which
    /// hardcodes the same constant and never reads a real page's geometry. Measuring against the
    /// real input would credit `set-snapshot` with a `595.276 → 612` move the REBUILD made and the
    /// mutation did not, which is a green for something never observed. Against the rebuild, the
    /// only thing that can move is `text` — the one field this subset genuinely reads out of a
    /// document — and it must.
    #[test]
    fn every_declared_kind_is_observable_and_its_inverse_restores_the_document() {
        let original = fixture();
        let base_text = independent_first_text(&original).expect("the independent reader finds page 1's first shown text");
        let base = project_pdf_1_4(&oracle_apply_mutation(&original, &spec("no-mutation")).expect("the reference rebuilds the document")).expect("the independent reader projects the rebuild");
        for kind in KINDS {
            let mutated = oracle_apply_mutation(&original, &spec(kind)).unwrap_or_else(|error| panic!("{kind}: {error}"));
            let moved = project_pdf_1_4(&mutated).unwrap_or_else(|error| panic!("{kind}: projecting the result failed: {error}"));
            if *kind != "no-mutation" {
                assert_ne!(moved, base, "{kind} left the compared projection untouched, so its scenario would pass whether or not the mutation ran");
            }
            let undo = match *kind {
                "no-mutation" => spec("no-mutation"),
                _ => json_object(vec![("kind", Json::String(kind.to_string())), ("params", json_object(vec![("snapshot", json_object(vec![("page", json_object(vec![("text", Json::String(base_text.clone()))]))]))]))]),
            };
            let restored = oracle_apply_mutation(&mutated, &undo).unwrap_or_else(|error| panic!("{kind}: inverse: {error}"));
            assert_eq!(project_pdf_1_4(&restored).unwrap(), base, "{kind}: applying the mutation and then its algebraic inverse must restore the rebuilt document's projection");
        }
    }

    /// 🔒️ Both halves of the identity law, on the real document. The `text` half is the load-bearing
    /// one: it is read out of the REAL 6.3 MB input by `lopdf`'s content-stream decoder and must
    /// survive into a document this module wrote object by object.
    #[test]
    fn the_round_trip_recovers_the_real_documents_own_text_and_is_not_a_byte_passthrough() {
        let original = fixture();
        let base_text = independent_first_text(&original).expect("the independent reader finds page 1's first shown text");
        assert!(!base_text.is_empty(), "the real thesis shows text on page 1");
        let rebuilt = oracle_apply_mutation(&original, &spec("no-mutation")).expect("the reference rebuilds the document");
        assert_ne!(rebuilt, original, "a from-scratch single-page writer cannot reproduce a 65-page thesis; identical bytes would mean the input was smuggled");
        assert_eq!(project_pdf_1_4(&rebuilt).unwrap().str("text"), base_text);
    }

    #[test]
    fn unknown_kind_is_an_error_never_a_silent_no_op() {
        let unknown = json_object(vec![("kind", Json::String("not-a-real-kind".to_string())), ("params", json_object(vec![]))]);
        assert!(oracle_apply_mutation(&fixture(), &unknown).is_err());
        assert!(oracle_apply_mutation(&fixture(), &json_object(vec![("params", json_object(vec![]))])).is_err(), "a spec with no kind at all is an error too");
    }

    /// 📇️ The three declarations that must never drift: this module's [`KINDS`], the catalog in
    /// `🔣️component.json`, and the `Examples` rows of the case that claims it.
    #[test]
    fn kinds_matches_the_catalog_and_every_feature_row() {
        let manifest = include_str!("🔣️component.json");
        let feature = include_str!("../../../../../🧪️tests/mutate-pdf-1-4/component.feature");
        for kind in KINDS {
            assert!(manifest.contains(&format!("\"{kind}\"")), "the pdf-1-4-any catalog is missing {kind:?}");
            assert!(feature.contains(&format!("| {kind} ")), "the feature declares no Examples row for {kind:?}");
        }
        assert_eq!(KINDS.len(), 2, "PdfMutation declares exactly NoMutation and SetSnapshot in this standard");
    }
}
//#endregion 🧪️Tests
