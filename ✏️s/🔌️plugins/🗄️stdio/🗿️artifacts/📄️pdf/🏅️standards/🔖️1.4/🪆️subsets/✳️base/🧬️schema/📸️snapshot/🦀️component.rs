//! 🧬️ PdfSnapshot schema (1.4) — persistent fields + real codecs.
//!
//! 📄️ **This standard's own page model.** PDF 1.4 (Adobe PDF Reference 1.4, the version ISO
//! 19005-1/PDF-A-1 and ISO 15930-1/PDF-X-1a are written against) has a real page TREE: a catalog
//! pointing at a `/Pages` node whose `/Kids` recursively resolve to `/Page` leaves, each with its
//! own (possibly inherited) `/MediaBox` and content stream. A snapshot that can hold exactly one
//! page cannot hold a real document — the differential run of ticket
//! 26/08/23/END-TO-END-TESTING-REFACTOR measured that directly: the committed 65-page bachelor
//! thesis came back out of the old `{schema, page: PageDoc}` snapshot as a 607-byte one-pager, 64
//! pages destroyed on write, and `mutate-pdf-1-4-a`/`mutate-pdf-1-4-x` scored 0/9 apiece on
//! `pageCount: 65` vs `1`. `pages` is therefore a `Vec`, and this standard's codec walks the real
//! page tree (`../../🚪️io/🦀️component.rs`).
//!
//! 🔀️ **What is 1.4's and what is 1.7's.** The COS OBJECT GRAMMAR (ISO 32000-1 §7.3 — the same
//! lexical grammar in every PDF version) is reused from the 1.7 subtree rather than re-typed here,
//! the same way `ifc` reuses `step`'s Part-21 tokenizer and `gif` 89a reuses 87a's LZW/sub-block
//! codec: one syntax layer, one implementation. What is 1.4's OWN is everything above it — this
//! `PageDoc { width, height, text }` page vocabulary (1.7 models a page as `PdfPage { media_box,
//! crop_box, rotate, text }`, a different and deliberately richer view), the classic
//! cross-reference TABLE (`xref`/`trailer`; cross-reference STREAMS and object streams are PDF
//! 1.5 features and this standard's reader/writer has neither), and a writer that emits `%PDF-1.4`
//! with a simple `/Type1` font so the shown text is recoverable by any reader, not only one that
//! consults a `/ToUnicode` CMap.
//!
//! 🚫️ **What this snapshot deliberately does not carry.** The retained indirect-object graph is
//! 1.7's (`PdfSnapshot.objects: Vec<PdfIndirectObject>`). 1.4 keeps the resolved PAGE view only,
//! which is why `../✳️a`/`../✳️x`'s conformance checkers still report
//! `stdio.pdf.{a,x}.schema-gap-unverifiable` — an honest statement about this schema, unchanged by
//! this wave and not weakened by it.

use crate::artifacts::pdf::STDIO_PDF_DOCUMENT_SCHEMA;
use schema::ArtifactSchema;

//#region 🔖️Page
/// 📄️ One resolved page of a PDF 1.4 document — this standard's own page vocabulary.
///
/// `width`/`height` are the page's `/MediaBox` EXTENT (ISO 32000-1 §7.7.3.3's `[x0 y0 x1 y1]`
/// reduced to `x1-x0`/`y1-y0`), which is what a 1.4 consumer of this snapshot actually asks for;
/// the box's origin offset is not modelled, so a page whose MediaBox does not start at the origin
/// is re-emitted origin-anchored — a documented normal form of this standard's page view, not a
/// silent loss of the extent.
///
/// `text` is the page's SHOWN TEXT: the operand bytes of the text-showing operators (`Tj`, `TJ`,
/// `'`, `"` — ISO 32000-1 §9.4.3) concatenated in content-stream order, decoded lossily to UTF-8.
/// It is deliberately NOT font-decoded through a `/ToUnicode` CMap: this standard's writer shows
/// the field back through a simple single-byte font, so what is read is exactly what any reader
/// recovers, and decode→encode→decode is stable on it.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct PageDoc {
    pub width: f64,
    pub height: f64,
    #[value(default)]
    pub text: String,
}

impl PageDoc {
    /// 📐️ US Letter, the default `/MediaBox` a PDF consumer assumes when none is declared
    /// anywhere on the page's inheritance chain (ISO 32000-1 §7.7.3.3).
    pub const DEFAULT_WIDTH: f64 = 612.0;
    pub const DEFAULT_HEIGHT: f64 = 792.0;

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn new(width: f64, height: f64) -> Self {
        Self { width, height, text: String::new() }
    }
}

impl Default for PageDoc {
    fn default() -> Self {
        Self { width: Self::DEFAULT_WIDTH, height: Self::DEFAULT_HEIGHT, text: String::new() }
    }
}
//#endregion 🔖️Page

//#region 🔖️Snapshot
/// 📸️ Persisted `stdio.pdf` (1.4) snapshot — the document's resolved page tree, in order.
///
/// A PDF document always has at least one page (ISO 32000-1 §7.7.3.2: `/Count` is at least 1 for a
/// readable document), so `Default` is one blank US-Letter page rather than an empty list.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.pdf")]
pub struct PdfSnapshot {
    #[state(artifact)]
    pub schema: String,
    /// 📚️ The document's pages in reading order — the page tree walked flat. Index-keyed for
    /// diffing (`../🔺️diff/🦀️component.rs`'s `PdfPagesDiff`).
    #[state(artifact)]
    #[value(default)]
    #[dsl(block)]
    pub pages: Vec<PageDoc>,
}

impl Default for PdfSnapshot {
    fn default() -> Self {
        Self { schema: STDIO_PDF_DOCUMENT_SCHEMA.into(), pages: vec![PageDoc::default()] }
    }
}

impl PdfSnapshot {
    /// 📄️ The first page, which is the one every `1.4/✳️a` and `1.4/✳️x` conformance axis is read
    /// from. `None` only for a snapshot carrying no pages at all — a state the codec never
    /// produces but serde can deserialize.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn first_page(&self) -> Option<&PageDoc> {
        self.pages.first()
    }

    /// 📄️ Mutable first page, created as a blank US-Letter page if the snapshot carries none.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn first_page_mut(&mut self) -> &mut PageDoc {
        if self.pages.is_empty() {
            self.pages.push(PageDoc::default());
        }
        &mut self.pages[0]
    }

    /// 📝️ The first page's shown text, or the empty string when the snapshot carries no pages.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn first_page_text(&self) -> &str {
        self.pages.first().map(|page| page.text.as_str()).unwrap_or("")
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️Codecs
impl store::ArtifactDsl for PdfSnapshot {
    const EXTENSION: &'static str = "pdf";
    fn envelope_id() -> &'static str {
        "stdio.pdf"
    }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let hex: String = body.chars().filter(|c| !c.is_whitespace()).collect();
        if hex.len() % 2 != 0 {
            return Err(store::TextError::new("odd hex length", dsl::TextSpan::at(1, 1)));
        }
        let mut bytes = Vec::with_capacity(hex.len() / 2);
        for i in (0..hex.len()).step_by(2) {
            bytes.push(u8::from_str_radix(&hex[i..i + 2], 16).map_err(|e| store::TextError::new(format!("invalid hex: {e}"), dsl::TextSpan::at(1, 1)))?);
        }
        crate::artifacts::pdf::standards::v1_4::subsets::base::io::decode_pdf(&bytes).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
    fn print_dsl(&self) -> String {
        let bytes = crate::artifacts::pdf::standards::v1_4::subsets::base::io::encode_pdf(self).unwrap_or_default();
        let body: String = bytes.iter().map(|b| format!("{b:02x}")).collect();
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for PdfSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = crate::artifacts::pdf::standards::v1_4::subsets::base::io::encode_pdf(self).map_err(store::PackError::Schema)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema("pack envelope mismatch".into()));
        }
        let _ = options;
        crate::artifacts::pdf::standards::v1_4::subsets::base::io::decode_pdf(&inner).map_err(store::PackError::Schema)
    }
}
//#endregion 🔖️Codecs

//#region 🔖️SnapshotFixtures
/// 🦑 Dissolved out of the former `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-
/// STATE-MACHINES) — pure snapshot constructors, no codec/IO concern.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn empty_pdf_snapshot() -> PdfSnapshot {
    PdfSnapshot::default()
}

/// 📄️ The demo `stdio.pdf` document — the single source of truth for `📚️examples/🎬️demo/🖼️assets/
/// 🗣️example.dsl.semio`/`🎒️example.pack.semio` (both are literally this snapshot's `print_dsl`/
/// `encode_pack` output, asserted equal by `fixture_honesty_law`).
///
/// Deliberately the real `decode_pdf(encode_pdf(seed))` FIXED POINT rather than a hand-written
/// struct: `encode_pdf` writes each page's `/MediaBox` and content stream from the model, and
/// `decode_pdf` reads them back through the real page-tree walk, so the fixed point is what
/// `parse_dsl(print_dsl(demo)) == demo` genuinely requires. Same construction 1.7's own
/// `demo_pdf17_snapshot` uses, for the same reason.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn demo_pdf_snapshot() -> PdfSnapshot {
    let seed = PdfSnapshot { schema: STDIO_PDF_DOCUMENT_SCHEMA.into(), pages: vec![PageDoc { width: 612.0, height: 792.0, text: "Semio Demo".into() }] };
    let bytes = crate::artifacts::pdf::standards::v1_4::subsets::base::io::encode_pdf(&seed).expect("encode_pdf(seed) must succeed");
    crate::artifacts::pdf::standards::v1_4::subsets::base::io::decode_pdf(&bytes).expect("decode_pdf(encode_pdf(seed)) must succeed")
}
//#endregion 🔖️SnapshotFixtures
