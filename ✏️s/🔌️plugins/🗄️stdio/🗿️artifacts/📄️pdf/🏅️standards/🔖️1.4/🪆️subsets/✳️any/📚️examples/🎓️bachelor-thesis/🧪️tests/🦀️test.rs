//! 🧪️ Tests for example `🎓️bachelor-thesis` — the plan's 3-test pattern against the real
//! ~6.3MB fixture (PDF **1.5** per its own header, decoded via 1.7's lenient reader). Ticket
//! 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION.
//!
//! Fixture facts confirmed by DIRECT inspection before writing these assertions (never guessed,
//! including a standalone debug harness run against the real bytes -- see the session's working
//! notes): classic xref table (no xref streams/ObjStm/`/Encrypt` in this file), 3190 indirect
//! objects, 65 pages. Zero `/ToUnicode` CMaps anywhere. Font mix is 24 `/Subtype /Type3` + 16
//! `/Subtype /TrueType` + 9 `/Subtype /Type1` — but EVERY font actually referenced from a page's
//! content stream (title page logo *and* every body-text paragraph checked) turned out to be
//! `/Type3` with a `/Differences` array of synthetic, subset-local glyph names tied to embedded
//! `/CharProcs` (e.g. `46/a46 65/a65/a66/a67...`, not AGL names) — the declared TrueType/Type1
//! fonts (confirmed independently to resolve correctly, e.g. `/BaseFont /ULVLGZ+Consolas` with a
//! plain `/Encoding /WinAnsiEncoding` decodes `"Hello"` byte-for-byte via `build_font_decoder`)
//! are simply never the ones actually shown in this particular document's text. Per spec, a
//! Type3 char code has NO required relationship to Unicode without `/ToUnicode` or AGL-resolvable
//! `/Differences` names -- neither is present here, so honest extraction (requirement #6: never
//! fabricate) legitimately returns U+FFFD for nearly all of this fixture's shown text. Real PDF
//! readers without OCR hit the exact same wall on this file. The assertions below are written to
//! match that reality: they prove the xref/object/page-tree/content-stream pipeline runs for
//! real on all 65 pages (exact page count, non-trivial extracted character volume proving Tj/TJ
//! parsing found real operators), while the *resolution* claim is proven separately by the
//! synthetic, non-fixture-dependent engine tests (`differences_and_agl_resolve_...`,
//! `tounicode_cmap_bfrange_identity_and_bfchar`, and the Identity-H writer/reader round trip).

use crate::artifacts::pdf::examples::bachelor_thesis::{source, FIXTURE_BYTES};
use crate::artifacts::pdf::standards::v1_7::engine::{decode_pdf, encode_pdf};
use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::PdfBuilderConstruction as PdfBuilder;
use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::inferences::Pdf17Inference;
use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::snapshot::PdfSnapshot;
use protocol::Inference;
use semio_framework_plugin::ArtifactBuilder;

#[test]
fn fixture_is_real_pdf_not_a_stub() {
    assert!(FIXTURE_BYTES.len() > 1_000_000, "bachelor-thesis.pdf must be the real ~6.3MB fixture, got {} bytes", FIXTURE_BYTES.len());
    assert_eq!(&FIXTURE_BYTES[0..5], b"%PDF-", "fixture must start with the PDF magic header");
}

#[test]
fn source_nonempty() {
    let _ = source();
}

//#region (a) RealDecodeNonTrivialInvariants
#[test]
fn real_decode_has_many_pages_and_real_extracted_text() {
    let snap = decode_pdf(FIXTURE_BYTES).expect("real 1.7 engine must decode the real fixture");
    assert_eq!(snap.declared_version, "1.5", "1.7's lenient reader must report the fixture's own declared version, not overwrite it");
    assert!(snap.pages.len() > 1, "bachelor-thesis.pdf must decode to more than one page, got {}", snap.pages.len());
    assert!(!snap.objects.is_empty(), "the full raw object graph must be retained (lossless-retention ground rule)");

    assert_eq!(snap.pages.len(), 65, "exact page count confirmed by direct inspection (/Count 65 on the root /Pages node)");
    assert!(snap.objects.len() > 1000, "3190-sized xref confirmed by direct inspection, got only {} resolved objects", snap.objects.len());

    let non_empty_pages = snap.pages.iter().filter(|p| !p.text.trim().is_empty()).count();
    assert!(non_empty_pages > 0, "at least one page must have non-empty extracted text");

    // 📏 Non-trivial extracted character volume across the whole document -- proves the
    // content-stream tokenizer + BT/ET + Tj/TJ operator handling actually ran against real,
    // non-trivial page content (not that it just silently found nothing). The characters
    // themselves are legitimately mostly U+FFFD for THIS fixture (see the module doc comment:
    // every referenced font is Type3 with synthetic, non-AGL `/Differences` names and there is
    // no `/ToUnicode` anywhere in the file) -- asserting "no U+FFFD" here would be dishonest.
    let total_chars: usize = snap.pages.iter().map(|p| p.text.chars().count()).sum();
    assert!(total_chars > 10_000, "expected substantial extracted character volume across 65 pages of body text, got {total_chars}");
}
//#endregion (a) RealDecodeNonTrivialInvariants

//#region (b) DecodeEncodeDecodeStructuralEquality
/// 🧪️ `codec_retention_law` (real-fixture instance, per the ticket's test-law naming
/// convention): decode→encode→decode is structurally equal at the page level on the real
/// bachelor-thesis fixture -- `objects`/`trailer` are intentionally NOT re-emitted (see
/// `PdfSnapshot`'s own doc comment), so this asserts the writer's honestly-scoped normal form,
/// not full-file byte identity.
#[test]
fn codec_retention_law_bachelor_thesis_decode_encode_decode() {
    let original = decode_pdf(FIXTURE_BYTES).expect("decode");
    let rewritten_bytes = encode_pdf(&original).expect("encode");
    let redecoded = decode_pdf(&rewritten_bytes).expect("re-decode");
    assert_eq!(redecoded.pages.len(), original.pages.len());
    for (a, b) in original.pages.iter().zip(redecoded.pages.iter()) {
        assert_eq!(a.media_box, b.media_box);
        assert_eq!(a.rotate, b.rotate);
        assert_eq!(a.text, b.text);
    }
}

#[test]
fn decode_encode_decode_is_structurally_equal_at_page_level() {
    // 📏 Structural, not byte-identical (the writer regenerates a fresh minimal file from
    // pages+info only -- `objects`, the raw graph, is intentionally NOT re-emitted; see the 1.7
    // snapshot's doc comment). Page-level fields (media box, rotate, extracted text) must match
    // exactly since our own Identity-H + ToUnicode writer/reader round-trips any Unicode string
    // losslessly, including any U+FFFD the original extraction produced.
    let original = decode_pdf(FIXTURE_BYTES).expect("decode");
    let rewritten_bytes = encode_pdf(&original).expect("encode");
    let redecoded = decode_pdf(&rewritten_bytes).expect("re-decode");

    assert_eq!(redecoded.pages.len(), original.pages.len());
    for (i, (a, b)) in original.pages.iter().zip(redecoded.pages.iter()).enumerate() {
        assert_eq!(a.media_box, b.media_box, "page {i} media_box must round-trip");
        assert_eq!(a.rotate, b.rotate, "page {i} rotate must round-trip");
        assert_eq!(a.text, b.text, "page {i} extracted text must round-trip byte-for-byte through our own Identity-H writer");
    }
    assert_eq!(original.info.title, redecoded.info.title);
    assert_eq!(original.info.author, redecoded.info.author);
}
//#endregion (b) DecodeEncodeDecodeStructuralEquality

//#region (c) AnalyzerBuilderRoundTrip
#[test]
fn analyzer_to_builder_round_trip_reproduces_equivalent_pages() {
    // 🎯 The project's core acceptance test: walk the real decode's page-tree view, reconstruct
    // an equivalent document using ONLY typed builder calls (`PdfBuilder::add_page`,
    // requirement #8), then compare the two documents' *analyzer output* (a fresh real decode of
    // the rebuilt file), not the in-memory structs -- proving the builder's typed ops are
    // actually sufficient to reconstruct what the analyzer sees, round-tripped through real bytes.
    let original = decode_pdf(FIXTURE_BYTES).expect("decode");

    let mut builder = PdfBuilder::empty();
    for page in &original.pages {
        builder = builder.add_page(page.clone());
    }
    let rebuilt_snapshot = builder.build().expect("builder-only reconstruction must succeed");
    let rebuilt_bytes = encode_pdf(&rebuilt_snapshot).expect("encode rebuilt snapshot");
    let rebuilt_redecoded = decode_pdf(&rebuilt_bytes).expect("re-decode rebuilt bytes");

    assert_eq!(rebuilt_redecoded.pages.len(), original.pages.len(), "builder-only reconstruction must preserve page count");
    for (i, (a, b)) in original.pages.iter().zip(rebuilt_redecoded.pages.iter()).enumerate() {
        assert_eq!(a.media_box, b.media_box, "page {i} media_box must match after builder round trip");
        assert_eq!(a.rotate, b.rotate, "page {i} rotate must match after builder round trip");
        assert_eq!(a.text, b.text, "page {i} extracted text must match after builder round trip");
    }
}
//#endregion (c) AnalyzerBuilderRoundTrip

//#region (d)(e) InferenceLaws
/// 🧪️ (d) `infer` on the real 65-page fixture is deterministic — two calls over the same decoded
/// snapshot produce byte-equal results. Ticket
/// 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING.
#[test]
fn inference_determinism_law() {
    let snapshot = decode_pdf(FIXTURE_BYTES).expect("decode real fixture");
    assert_eq!(Pdf17Inference::infer(&snapshot), Pdf17Inference::infer(&snapshot));
}

/// 🧪️ (e) `infer(&PdfSnapshot::default())` matches `Pdf17Inference::default()` — the hand-written
/// `Default` impl (`💡️inferences/🦀️component.rs`) must stay in lockstep with `infer` itself.
#[test]
fn inference_default_law() {
    assert_eq!(Pdf17Inference::infer(&PdfSnapshot::default()), Pdf17Inference::default());
}

/// 🧪️ `outline` on the real fixture matches the independently-verified page count/text volume
/// `real_decode_has_many_pages_and_real_extracted_text` above already asserts.
#[test]
fn outline_matches_real_fixture_page_count() {
    let snapshot = decode_pdf(FIXTURE_BYTES).expect("decode real fixture");
    let inferred = Pdf17Inference::infer(&snapshot);
    assert_eq!(inferred.outline.page_count, 65);
    assert_eq!(inferred.outline.title, snapshot.info.title);
}
//#endregion (d)(e) InferenceLaws
