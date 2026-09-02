//! 🚪️ drawing -> pdf — foreign `Serializer<DrawingSnapshot>` (design.md §3). Honest not-yet-implemented
//! stub: the pre-migration free function this replaces printed the artifact's OWN `.drawing` DSL text
//! and mislabeled it as PDF bytes — a real correctness bug (any consumer trusting the `s.stdio.
//! pdf@1.4/*` dialect would receive bytes that are not a PDF at all). Fixed here by refusing
//! honestly instead of perpetuating the mislabeled payload; real PDF export is out of scope for
//! this cutover. `IoFidelity::Lossy` (would-be, once implemented) is moot while the hop always errors.

use crate::artifacts::drawing::DrawingSnapshot;
use semio_framework::io::io_mechanism::Serializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};

pub const PDF_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.pdf", standard: StandardId("1.4"), subset: SubsetId::ANY };

pub struct DrawingIntoPdf;

impl Serializer<DrawingSnapshot> for DrawingIntoPdf {
    const INTO: Dialect = PDF_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    fn serialize(_from: &DrawingSnapshot) -> IoResult<IoPayload> {
        Err(IoError { message: "DrawingIntoPdf: PDF export is not yet implemented".to_string(), diagnostics: Vec::new() })
    }
}
