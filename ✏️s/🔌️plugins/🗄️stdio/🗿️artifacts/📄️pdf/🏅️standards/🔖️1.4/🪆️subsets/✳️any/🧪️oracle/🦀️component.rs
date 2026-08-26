//! 🔮️ Mutation oracle for this subset — every mutation kind the subset declares, performed by the
//! registered reference implementation so the subject's own mutation has an independent result to
//! be compared against instead of being checked against its own reading.
//!
//! The vocabulary is per SUBSET, not per artifact: two standards of the same format declare
//! different mutations, and a subset that shares an implementation with another reaches it through
//! the shared `document` module rather than by copying it.
//!
//! 📚️ **What this module measures, and what changed.** PDF 1.4 has a real page TREE (ISO 32000-1
//! §7.7.3: a catalog pointing at `/Pages`, whose `/Kids` recursively resolve to `/Page` leaves,
//! each with its own inheritable `/MediaBox` and content stream), and this subset's `PdfSnapshot`
//! now carries it — `pages: Vec<PageDoc>`, one entry per leaf, in reading order. Until the first
//! full differential run of ticket 26/08/23/END-TO-END-TESTING-REFACTOR it carried a single
//! `page: PageDoc` instead, and this module was written to MIRROR that: it rebuilt every document
//! as one synthetic page pinned to `MediaBox [0 0 612 792]`, because the subject's own decoder
//! hardcoded the same constant and never read a real page's geometry. Both halves are gone. The
//! reference now round-trips the real document through `lopdf`'s own object graph, builds a
//! `set-snapshot` target page for page from the snapshot the spec carries, and projects EVERY
//! page's real box and shown text — so the comparison covers the 65-page thesis rather than one
//! page of it.
//!
//! 👁️ **Both directions are independent.** The reader is `lopdf`'s own object graph and
//! content-stream decoder, never this repository's byte-search `decode_pdf`; the writer is a fresh
//! `lopdf::Document` assembled object by object, never a delegation to `encode_pdf`. That is what
//! makes every scenario of this case `@mode-differential` rather than a self-check.
//!
//! @see ../🧪️oracle/🔣️component.json — the mutation catalog this module is measured against.
//! @see ../🧬️schema/🧬️mutations/🦀️component.rs — the mutation vocabulary itself.
//! @see ../🧬️schema/📸️snapshot/🦀️component.rs — `PdfSnapshot`, the page tree this projects.

use semio_repo_test_host::Json;

//#region 🔖️Vocabulary
/// 🧾️ Kebab-case spelling of every variant this subset's `PdfMutation` declares, in declaration
/// order. Two, and the thinness is the vocabulary's, not the case's: `../🧬️schema/🧬️mutations/
/// 🦀️component.rs` really has exactly `NoMutation` and `SetSnapshot`, because this standard's
/// document vocabulary is "replace the page tree" and nothing finer. Declared here rather than in
/// the case adapter so the adapter, this module's own tests and the manifest all read ONE list.
pub const KINDS: &[&str] = &["no-mutation", "set-snapshot"];
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
/// 📄️ Reads `params.snapshot.pages` out of a `set-snapshot` spec — the whole page tree this
/// subset's one non-baseline mutation carries end to end.
///
/// An absent or empty list is an ERROR, not an empty document: ISO 32000-1 §7.7.3.2 gives `/Count`
/// a lower bound of one, so "a PDF with no pages" is not a state either producer can write, and a
/// row that asked for it would be asking both halves to agree on nonsense.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn target_pages(spec: &Json) -> Result<Vec<OraclePage>, String> {
    let number = |value: Option<&Json>, fallback: f64| match value {
        Some(Json::Number(value)) => *value,
        _ => fallback,
    };
    let pages = match spec.get("params").and_then(|params| params.get("snapshot")).and_then(|snapshot| snapshot.get("pages")) {
        Some(Json::Array(items)) => items,
        _ => return Err("set-snapshot: `params.snapshot.pages` must be a list of pages — this standard's snapshot is a page TREE, not a single page".to_string()),
    };
    if pages.is_empty() {
        return Err("set-snapshot: `params.snapshot.pages` is empty, and ISO 32000-1 §7.7.3.2 gives a page tree a lower bound of one page".to_string());
    }
    Ok(pages.iter().map(|page| OraclePage { width: number(page.get("width"), 612.0), height: number(page.get("height"), 792.0), text: page.str("text") }).collect())
}
//#endregion 🔖️Spec

//#region 🔖️Dispatch
/// 🦠️ Applies one declared mutation kind to a real artifact and returns the re-serialized bytes.
/// An unrecognised kind is an error, never a silent no-op: a mutation that is quietly skipped
/// reports as a passing test.
#[cfg(feature = "oracles")]
pub fn oracle_apply_mutation(input: &[u8], spec: &Json) -> Result<Vec<u8>, String> {
    match spec.str("kind").as_str() {
        "" => Err("mutation spec carries no `kind`".to_string()),
        "no-mutation" => oracles::round_trip(input),
        "set-snapshot" => oracles::build_document(&target_pages(spec)?),
        kind => Err(format!("mutation kind {:?} has no oracle implementation ({} input byte(s))", kind, input.len())),
    }
}

/// 🚫️ Without the `oracles` feature the reference implementation is not linked at all.
#[cfg(not(feature = "oracles"))]
pub fn oracle_apply_mutation(_input: &[u8], _spec: &Json) -> Result<Vec<u8>, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}

/// ↩️ The undo of `forward`, read out of `base` by the independent implementation alone.
/// `NoMutation` inverts to itself; `SetSnapshot` inverts to a `SetSnapshot` carrying the base
/// document's OWN page tree, read back through `lopdf` — the same closed form
/// `../🧬️schema/🧬️mutations/🦀️component.rs`'s `impl Mutation<PdfSnapshot>` declares.
#[cfg(feature = "oracles")]
pub fn oracle_inverse_spec(base: &[u8], forward: &Json) -> Result<Json, String> {
    let object = |pairs: Vec<(&str, Json)>| Json::Object(pairs.into_iter().map(|(key, value)| (key.to_string(), value)).collect());
    Ok(match forward.str("kind").as_str() {
        "no-mutation" => object(vec![("kind", Json::String("no-mutation".to_string())), ("params", object(vec![]))]),
        "set-snapshot" => object(vec![
            ("kind", Json::String("set-snapshot".to_string())),
            ("params", object(vec![("snapshot", object(vec![("pages", Json::Array(independent_pages(base)?.iter().map(OraclePage::to_json).collect()))]))])),
        ]),
        other => return Err(format!("no inverse rule for kind {other:?}")),
    })
}

#[cfg(not(feature = "oracles"))]
pub fn oracle_inverse_spec(_base: &[u8], _forward: &Json) -> Result<Json, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
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
    Ok(Json::Object(vec![
        ("pageCount".to_string(), Json::Number(pages.len() as f64)),
        ("pages".to_string(), Json::Array(pages.iter().map(OraclePage::to_json).collect())),
    ]))
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

    /// 🧫️ The real committed document `mutate-pdf-1-4` runs on — the 6.3 MB, 65-page LaTeX
    /// bachelor thesis this standard's own examples directory carries, every page typeset at A4
    /// (`/MediaBox [0 0 595.276 841.89]`).
    const FIXTURE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/📚️examples/🎓️bachelor-thesis/🖼️assets/📄️bachelor-thesis.pdf");

    fn json_object(pairs: Vec<(&str, Json)>) -> Json {
        Json::Object(pairs.into_iter().map(|(key, value)| (key.to_string(), value)).collect())
    }

    fn fixture() -> Vec<u8> {
        std::fs::read(FIXTURE).expect("the committed bachelor-thesis document")
    }

    /// 📄️ The `set-snapshot` row every scenario of the case carries, read out of the feature file
    /// itself so a row that drifts fails here rather than silently measuring something else.
    fn spec(kind: &str) -> Json {
        let feature = include_str!("../../../../../🧪️tests/mutate-pdf-1-4/component.feature");
        let (_, params) = crate::law::feature_rows(feature).into_iter().find(|(id, _)| id == kind).unwrap_or_else(|| panic!("the feature declares no Examples row for {kind:?}"));
        json_object(vec![("kind", Json::String(kind.to_string())), ("params", params)])
    }

    /// ⚖️ The two laws `mutate-pdf-1-4`'s adapter asserts in role, proven here against the real
    /// document without the runner, and measured against the COMMITTED input's own projection —
    /// not against a rebuild. That is what the real page-tree codec bought: the reference no longer
    /// pins a synthetic geometry the subject could not read, so `no-mutation` genuinely lands on
    /// the input's own 65-page projection and `set-snapshot` genuinely has to move it.
    #[test]
    fn every_declared_kind_is_observable_and_its_inverse_restores_the_document() {
        let original = fixture();
        let base = project_pdf_1_4(&original).expect("the independent reader projects the real document");
        for kind in KINDS {
            let forward = spec(kind);
            let mutated = oracle_apply_mutation(&original, &forward).unwrap_or_else(|error| panic!("{kind}: {error}"));
            let moved = project_pdf_1_4(&mutated).unwrap_or_else(|error| panic!("{kind}: projecting the result failed: {error}"));
            if *kind != "no-mutation" {
                assert_ne!(moved, base, "{kind} left the compared projection untouched, so its scenario would pass whether or not the mutation ran");
            } else {
                assert_eq!(moved, base, "no-mutation must leave the real document's whole page tree exactly where it was");
            }
            let undo = oracle_inverse_spec(&original, &forward).unwrap_or_else(|error| panic!("{kind}: inverse spec: {error}"));
            let restored = oracle_apply_mutation(&mutated, &undo).unwrap_or_else(|error| panic!("{kind}: inverse: {error}"));
            assert_eq!(project_pdf_1_4(&restored).unwrap(), base, "{kind}: applying the mutation and then its algebraic inverse must restore the document's projection");
        }
    }

    /// 📚️ The page tree really is read as a tree: 65 leaves, every one of them A4, page 1 carrying
    /// the thesis title. A regression to the one-page stub fails on the very first assertion.
    #[test]
    fn the_real_document_reads_as_sixty_five_a4_pages() {
        let pages = independent_pages(&fixture()).expect("the independent reader walks the page tree");
        assert_eq!(pages.len(), 65);
        assert!(pages.iter().all(|page| (page.width - 595.276).abs() < 0.01 && (page.height - 841.89).abs() < 0.01), "every page of this thesis is typeset at A4");
        assert!(pages[0].text.starts_with("SemIO"), "page 1 shows the thesis title, got {:?}", &pages[0].text);
        assert!(pages.iter().filter(|page| !page.text.is_empty()).count() > 60, "a 65-page thesis shows text on nearly every page");
    }

    /// 🔒️ Both halves of the identity law, on the real document.
    #[test]
    fn the_round_trip_recovers_the_whole_page_tree_and_is_not_a_byte_passthrough() {
        let original = fixture();
        let rebuilt = oracle_apply_mutation(&original, &spec("no-mutation")).expect("the reference re-serializes the document");
        assert_ne!(rebuilt, original, "the reference writes a fresh file from its own object graph; identical bytes would mean the input was smuggled");
        assert_eq!(project_pdf_1_4(&rebuilt).unwrap(), project_pdf_1_4(&original).unwrap());
    }

    /// ✍️ The independent WRITER is a real page-tree writer, not a one-page one: what it is handed
    /// is what a reader gets back.
    #[test]
    fn the_independent_writer_round_trips_a_multi_page_target() {
        let target = vec![
            OraclePage { width: 595.276, height: 841.89, text: "first".to_string() },
            OraclePage { width: 419.528, height: 595.276, text: String::new() },
            OraclePage { width: 612.0, height: 792.0, text: "third".to_string() },
        ];
        let bytes = build_document(&target).expect("the independent writer builds a three-page document");
        let read_back = independent_pages(&bytes).expect("the independent reader reads it back");
        assert_eq!(read_back.len(), 3);
        assert_eq!(read_back.iter().map(|page| page.text.clone()).collect::<Vec<_>>(), vec!["first".to_string(), String::new(), "third".to_string()]);
        for (written, read) in target.iter().zip(&read_back) {
            assert!((written.width - read.width).abs() < 0.01 && (written.height - read.height).abs() < 0.01, "page geometry must survive the writer");
        }
    }

    #[test]
    fn unknown_kind_is_an_error_never_a_silent_no_op() {
        let unknown = json_object(vec![("kind", Json::String("not-a-real-kind".to_string())), ("params", json_object(vec![]))]);
        assert!(oracle_apply_mutation(&fixture(), &unknown).is_err());
        assert!(oracle_apply_mutation(&fixture(), &json_object(vec![("params", json_object(vec![]))])).is_err(), "a spec with no kind at all is an error too");
    }

    /// 🚫️ A `set-snapshot` row that carries the OLD single-page shape, or no page at all, is an
    /// error rather than a document silently rebuilt as one blank page — which is exactly the
    /// failure mode this whole rewrite exists to remove.
    #[test]
    fn a_set_snapshot_spec_without_a_page_list_is_refused() {
        let single = json_object(vec![("kind", Json::String("set-snapshot".to_string())), ("params", json_object(vec![("snapshot", json_object(vec![("page", json_object(vec![("text", Json::String("one".to_string()))]))]))]))]);
        assert!(oracle_apply_mutation(&fixture(), &single).is_err(), "the one-page shape this standard no longer has must be refused, never silently accepted");
        let empty = json_object(vec![("kind", Json::String("set-snapshot".to_string())), ("params", json_object(vec![("snapshot", json_object(vec![("pages", Json::Array(vec![]))]))]))]);
        assert!(oracle_apply_mutation(&fixture(), &empty).is_err(), "a page tree with no page is not a document");
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
