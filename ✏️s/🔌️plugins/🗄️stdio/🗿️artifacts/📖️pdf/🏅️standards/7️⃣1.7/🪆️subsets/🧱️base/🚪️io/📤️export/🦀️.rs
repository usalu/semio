//! 📤️ PDF byte writer of `stdio.pdf` (1.7/🧱️base): typed lanes reconciled onto the retained graph when one is carried
//! (re-read through the lift so only moved lanes are re-stated), lowered afresh otherwise; the streaming
//! [`DocumentStream`] a guest drives one page per step; and the typed builders ([`text_document`], [`PdfTextLayout`],
//! embedded TrueType fonts). Depends on the model, the COS engine and the lift.

use crate::standards::v1_7::subsets::base::modules::content::{content_references, ContentReferences};
use crate::standards::v1_7::subsets::base::modules::fonts::{standard_font, FontCodec};
use crate::standards::v1_7::subsets::base::modules::lexer::dict_get;
use crate::standards::v1_7::subsets::base::io::foreign_artifacts;
use crate::standards::v1_7::subsets::base::io::error::{PResult, PdfEngineError};
use crate::standards::v1_7::subsets::base::modules::lift::{lift_document_with, Category};
use crate::standards::v1_7::subsets::base::modules::lower::{lower_acro_form_standalone, lower_catalog_standalone, lower_document, lower_document_headless, lower_info, lower_page_standalone, LowerOptions, LoweredDocument};
use crate::standards::v1_7::subsets::base::modules::writer::{serialize_document, DocumentTrailer, PdfWriter, WriteOptions};
use crate::standards::v1_7::subsets::base::schema::graph_source::{GraphSource,ObjectSource};
use crate::standards::v1_7::subsets::base::schema::snapshot::*;
#[cfg(test)]
use crate::standards::v1_7::subsets::base::io::import::{decode_pdf, decode_pdf_with_password, sniff_pdf};
use std::borrow::Cow;
use std::collections::{BTreeMap, HashMap, HashSet};

#[path = "🧬️encode-and-reconcile/🦀️.rs"]
mod encode_and_reconcile;
pub use encode_and_reconcile::*;

#[path = "🪡️graft/🦀️.rs"]
pub mod graft;
use graft::*;

#[path = "🌊️streaming-and-builders/🦀️.rs"]
mod streaming_and_builders;
pub use streaming_and_builders::*;

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🆕️Blank
/// 📄️ The demo `stdio.pdf.1.7` document -- the single source of truth for `🏅️standards/7️⃣1.7/
/// 📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio`/`🎒️.pack.semio` (both are literally this
/// snapshot's `print_dsl`/`encode_pack` output, asserted equal by `fixture_honesty_law`).
///
/// Deliberately the real `decode_pdf(encode_pdf(seed))` FIXED POINT: the typed lanes survive the
/// round trip by the `lift(lower(t)) == t` law, and `objects`/`trailer` are whatever the fresh
/// write produced, read back — a hand-built snapshot with empty `objects` would not equal its
/// own decoded print.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn demo_pdf17_snapshot() -> PdfSnapshot {
    let seed = text_document(&[(200.0, 300.0, "Semio")]);
    let bytes = encode_pdf(&seed).expect("encode_pdf(seed) must succeed");
    crate::standards::v1_7::subsets::base::io::import::decode_pdf(&bytes).expect("decode_pdf(encode_pdf(seed)) must succeed")
}

/// 🆕️ A new pdf document: the empty 1.7 document as the real codec round-trips it — its retained object graph
/// (`objects`/`trailer`) is what a fresh write produced, read back, exactly like [`demo_pdf17_snapshot`]; the empty
/// `Default` carries no graph, so it saved as a document that reopened as a different one.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn blank_pdf_snapshot() -> PdfSnapshot {
    use crate::standards::v1_7::subsets::base::io::import::decode_pdf;
    encode_pdf(&PdfSnapshot::default()).and_then(|bytes| decode_pdf(&bytes)).expect("blank_pdf_snapshot: the empty document round-trips through the real codec")
}
//#endregion 🆕️Blank
