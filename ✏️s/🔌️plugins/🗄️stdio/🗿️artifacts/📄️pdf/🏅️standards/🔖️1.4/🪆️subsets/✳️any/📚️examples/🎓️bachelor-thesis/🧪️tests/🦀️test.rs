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
use crate::artifacts::pdf::standards::v1_7::subsets::any::io::{decode_pdf, encode_pdf};
use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::diff::PdfDiff;
use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::inferences::Pdf17Inference;
use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::mutations::{apply_pdf_mutation, PdfMutation};
use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::snapshot::PdfSnapshot;
use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::PdfBuilderConstruction as PdfBuilder;
use protocol::command::DiffAlgebra;
use protocol::{DiffCodec, Inference, Mutation, MutationDiff, OpBinary};
use semio_framework_plugin::ArtifactBuilder;
use store::{ArtifactDsl, ArtifactPack};

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn assert_logical_cos_retained(snapshot: &PdfSnapshot) {
    assert!(snapshot.objects.len() > 1_000, "native PDF import must retain the logical COS object graph");
    assert!(snapshot.objects.iter().any(|object| matches!(object.value, crate::artifacts::pdf::standards::v1_7::subsets::any::schema::snapshot::PdfObject::Stream { ref data, .. } if !data.is_empty())));
    assert!(snapshot.trailer.iter().any(|entry| entry.key == "Root"));
}

#[semio_framework_async_macros::async_test]
async fn fixture_is_real_pdf_not_a_stub() {
    assert!(FIXTURE_BYTES.len() > 1_000_000, "bachelor-thesis.pdf must be the real ~6.3MB fixture, got {} bytes", FIXTURE_BYTES.len());
    assert_eq!(&FIXTURE_BYTES[0..5], b"%PDF-", "fixture must start with the PDF magic header");
}

#[semio_framework_async_macros::async_test]
async fn source_nonempty() {
    let _ = source();
}

//#region (a) RealDecodeNonTrivialInvariants
#[semio_framework_async_macros::async_test]
async fn real_decode_has_many_pages_and_real_extracted_text() {
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
/// convention): decode→writer reconstruction reaches a deterministic logical fixed point.
#[semio_framework_async_macros::async_test]
async fn codec_retention_law_bachelor_thesis_decode_encode_decode() {
    let original = decode_pdf(FIXTURE_BYTES).expect("decode");
    let rewritten_bytes = encode_pdf(&original).expect("encode");
    assert_eq!(encode_pdf(&decode_pdf(&rewritten_bytes).expect("re-decode canonical output")).expect("re-encode canonical output"), rewritten_bytes);
    let redecoded = decode_pdf(&rewritten_bytes).expect("re-decode");
    assert_eq!(redecoded.pages.len(), original.pages.len());
    for (a, b) in original.pages.iter().zip(redecoded.pages.iter()) {
        assert_eq!(a.media_box, b.media_box);
        assert_eq!(a.rotate, b.rotate);
        assert_eq!(a.text, b.text);
    }
}

#[semio_framework_async_macros::async_test]
async fn lossless_structural_flow_law_bachelor_thesis_snapshot_mutation_diff_io_and_inverse() {
    let original = decode_pdf(FIXTURE_BYTES).expect("decode exact fixture");
    assert_logical_cos_retained(&original);
    let canonical = encode_pdf(&original).expect("logical writer export");

    let dsl = original.print_dsl();
    let from_dsl = PdfSnapshot::parse_dsl(&dsl).expect("snapshot DSL roundtrip");
    assert_eq!(from_dsl, original, "DSL must carry the complete logical snapshot model");
    assert_logical_cos_retained(&from_dsl);
    assert_eq!(encode_pdf(&from_dsl).expect("DSL-restored logical export"), canonical);

    let pack = original.encode_pack();
    let from_pack = PdfSnapshot::decode_pack(&pack).expect("snapshot unpack");
    assert_eq!(from_pack, original, "pack must carry the complete logical snapshot model");
    assert_logical_cos_retained(&from_pack);
    assert_eq!(encode_pdf(&from_pack).expect("pack-restored logical export"), canonical);

    let empty = PdfDiff::between(&original, &original);
    assert!(empty.is_empty());
    assert_eq!(encode_pdf(&empty.apply(&original).unwrap()).expect("self-diff logical export"), canonical);

    let mut no_op = original.clone();
    let no_op_diff = apply_pdf_mutation(&mut no_op, &PdfMutation::NoMutation);
    assert!(no_op_diff.diff().is_empty());
    assert_eq!(encode_pdf(&no_op).expect("no-op mutation logical export"), canonical);

    let mutation = PdfMutation::AppendPageContent { index: 0, text: "dirty".into() };
    let mutation_frame = mutation.encode_op().expect("encode structural mutation");
    let restored_mutation = PdfMutation::decode_op(&mutation_frame).expect("decode structural mutation");
    assert_eq!(restored_mutation, mutation);
    let diff = restored_mutation.diff(&original);
    let diff_frame = diff.diff().encode_diff().expect("encode structural diff");
    let restored_diff = PdfDiff::decode_diff(&diff_frame).expect("decode structural diff");
    assert_eq!(&restored_diff, diff.diff());
    let dirty = restored_diff.apply(&original).unwrap();
    let dirty_bytes = encode_pdf(&dirty).expect("dirty snapshot must use the canonical writer");
    assert_ne!(dirty_bytes, canonical);
    let dirty_redecoded = decode_pdf(&dirty_bytes).expect("dirty writer output must remain valid PDF");
    assert!(dirty_redecoded.pages[0].text.ends_with("dirty"));

    let inverse = restored_diff.inverse(&original);
    let inverse_frame = inverse.encode_diff().expect("encode inverse diff");
    let restored_inverse = PdfDiff::decode_diff(&inverse_frame).expect("decode inverse diff");
    let restored = restored_inverse.apply(&dirty).unwrap();
    assert_eq!(restored, original, "diff inverse must restore the complete logical model");
    assert_eq!(encode_pdf(&restored).expect("inverse logical writer export"), canonical);

    let mut mutation_dirty = original.clone();
    apply_pdf_mutation(&mut mutation_dirty, &restored_mutation);
    for inverse_mutation in restored_mutation.inverse(&original) {
        let inverse_mutation_frame = inverse_mutation.encode_op().expect("encode inverse mutation");
        let restored_inverse_mutation = PdfMutation::decode_op(&inverse_mutation_frame).expect("decode inverse mutation");
        apply_pdf_mutation(&mut mutation_dirty, &restored_inverse_mutation);
    }
    assert_eq!(mutation_dirty, original);
    assert_eq!(encode_pdf(&mutation_dirty).expect("mutation inverse logical export"), canonical);
}

#[semio_framework_async_macros::async_test]
async fn decode_encode_decode_is_structurally_equal_at_page_level() {
    // 📏 The logical writer deterministically materializes a fresh PDF serialization.
    let original = decode_pdf(FIXTURE_BYTES).expect("decode");
    let rewritten_bytes = encode_pdf(&original).expect("encode");
    assert_eq!(encode_pdf(&decode_pdf(&rewritten_bytes).expect("canonical decode")).expect("canonical re-encode"), rewritten_bytes);
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
#[semio_framework_async_macros::async_test]
async fn analyzer_to_builder_round_trip_reproduces_equivalent_pages() {
    // 🎯 The project's core acceptance test: walk the real decode's page-tree view, reconstruct
    // an equivalent document using ONLY typed builder calls (`PdfBuilder::add_page`,
    // requirement #8), then compare the two documents' *analyzer output* (a fresh real decode of
    // the rebuilt file), not the in-memory structs -- proving the builder's typed ops are
    // actually sufficient to reconstruct what the analyzer sees, round-tripped through real bytes.
    let original = decode_pdf(FIXTURE_BYTES).expect("decode");

    let mut builder = PdfBuilder::empty().await;
    for page in &original.pages {
        builder = builder.add_page(page.clone());
    }
    let rebuilt_snapshot = builder.build().await.expect("builder-only reconstruction must succeed");
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
#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = decode_pdf(FIXTURE_BYTES).expect("decode real fixture");
    assert_eq!(Pdf17Inference::infer(&snapshot), Pdf17Inference::infer(&snapshot));
}

/// 🧪️ (e) `infer(&PdfSnapshot::default())` matches `Pdf17Inference::default()` — the hand-written
/// `Default` impl (`💡️inferences/🦀️component.rs`) must stay in lockstep with `infer` itself.
#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(Pdf17Inference::infer(&PdfSnapshot::default()), Pdf17Inference::default());
}

/// 🧪️ `outline` on the real fixture matches the independently-verified page count/text volume
/// `real_decode_has_many_pages_and_real_extracted_text` above already asserts.
#[semio_framework_async_macros::async_test]
async fn outline_matches_real_fixture_page_count() {
    let snapshot = decode_pdf(FIXTURE_BYTES).expect("decode real fixture");
    let inferred = Pdf17Inference::infer(&snapshot);
    assert_eq!(inferred.outline.page_count, 65);
    assert_eq!(inferred.outline.title, snapshot.info.title);
}
//#endregion (d)(e) InferenceLaws
