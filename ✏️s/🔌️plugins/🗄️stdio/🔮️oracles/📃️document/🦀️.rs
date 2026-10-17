//! 🔮️ Document-family oracles: the PDF reference implementations, and the OOXML CONFORMANCE-CLASS
//! engine the six `✳️strict`/`✳️transitional` subsets of `📕️xlsx`, `📜️docx` and `🎞️pptx` share.
//! Every registered reference library is wrapped behind an owned interface — no external type
//! appears in this module's public API, so nothing downstream can accidentally depend on
//! `pdf-writer`, `lopdf`, `zip` or `quick-xml`. Compiled only with the `oracles` feature, which no
//! production target enables.
//!
//! @see 🔮️oracles/🔣️.json — the approved oracle registry these functions implement.

use semio_repo_test_host::Json;

#[path = "📕️pdf-spec-and-oracles/🦀️.rs"]
mod pdf_spec_and_oracles;
pub use pdf_spec_and_oracles::*;

//#region 🔖️Unavailable

//#endregion 🔖️Unavailable

//#region 🔖️OoxmlConformanceClass
/// 🏅️ The OOXML CONFORMANCE-CLASS engine — one independent OPC + XML implementation (`zip` 6 for
/// the container, `quick-xml` 0.42 for every part), shared by the six subsets that police a
/// conformance class rather than document content: `📕️xlsx`, `📜️docx` and `🎞️pptx`, each in its
/// `✳️strict` (ISO/IEC 29500-1) and `✳️transitional` (ISO/IEC 29500-4) flavour.
///
/// It lives in the shared family module rather than in any one subset because all six genuinely
/// share it: the axes their conformance checkers read — the main-namespace declaration, the
/// DrawingML namespace, the `officeDocument` relationship base, the root `conformance` attribute,
/// the legacy VML and `mc:AlternateContent` markup — are package-level facts of the OPC container,
/// identical in mechanism across the three formats and differing only in which part carries them.
///
/// Nothing here interprets a workbook, a document body or a slide: that is the `✳️any` subsets'
/// vocabulary, and their own reference pairings answer for it. This engine reads and writes the
/// package, which is exactly the surface a conformance class is defined on.
#[path = "🗂️ooxml/🦀️.rs"]
pub mod ooxml;
//#endregion 🔖️OoxmlConformanceClass

//#region 🔖️PdfConformanceClass
/// 🏅️ The PDF CONFORMANCE-CLASS engine — one independent `lopdf` 0.44 implementation shared by the
/// six PDF 1.7 subsets that police a conformance CLASS rather than document content: `✳️a`
/// (ISO 19005-2/-3), `✳️e` (ISO 24517-1), `✳️h` (the PDF Healthcare Best Practices Guide), `✳️ua`
/// (ISO 14289-1), `✳️vt` (ISO 16612-2) and `✳️x` (ISO 15930-7).
///
/// It lives in the shared family module rather than in any one subset for the same reason the OOXML
/// engine above does: all six genuinely share the MECHANISM — every axis their conformance checkers
/// read is a fact of the COS object graph (a `/Root` key, an action dictionary's `/S` subtype, a
/// `/FontDescriptor`'s embedded font program, an `/OutputIntent`'s `/S` marker) — and differ only in
/// WHICH axes they police and with which marker. Each subset owns that selection in its own
/// [`PdfConformanceProfile`] and its own `KINDS`, and refuses a kind it does not declare even when
/// this engine could perform it.
///
/// Nothing here interprets page content: pages, media boxes, content streams and `/Info`-as-document
/// metadata are the `✳️any` subset's vocabulary and its own `lopdf` pairing answers for them. This
/// engine reads and writes exactly the object-graph surface a PDF conformance class is defined on.
///
/// 🔓️ ONE deliberate scope note, recorded rather than glossed: `insert-encryption-dictionary` adds a
/// free-standing Standard Security Handler dictionary OBJECT and does not link it from the trailer's
/// `/Encrypt`. That is faithful to what the subsets actually check — every `check_*_conformance`
/// scans `snapshot.objects` for the `/Filter /Standard` + `/V` + `/R` + `/O` + `/U` shape and never
/// reads the trailer — and it is the only form the mutation can take and still leave a document both
/// producers can re-read, since a genuinely `/Encrypt`-linked trailer makes every string and stream
/// in the file ciphertext.
#[path = "🧷️pdf-conformance/🦀️.rs"]
pub mod pdf_conformance;
//#endregion 🔖️PdfConformanceClass
