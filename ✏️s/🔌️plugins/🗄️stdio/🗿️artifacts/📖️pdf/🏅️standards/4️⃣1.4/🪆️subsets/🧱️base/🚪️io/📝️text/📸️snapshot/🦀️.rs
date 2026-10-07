//! 📝️ Text representation codec surface for `stdio.pdf` (snapshot).

/// 📖️ Grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type PdfSnapshotText = String;
//#endregion 🚚️Carrier

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v1_4::subsets::base::schema::snapshot::*;
use crate::STDIO_PDF_DOCUMENT_SCHEMA;
use framework_schema::ArtifactSchema;

impl store::ArtifactDsl for PdfSnapshot {
    const EXTENSION: &'static str = "pdf";
    fn envelope_id() -> &'static str {
        "stdio.pdf"
    }
    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((envelope, rest)) => {
                if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1) { return Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "PDF snapshot text envelope mismatch", semio_framework_diagnostic::TextSpan::at(1, 1))); }
                rest
            }
            Err(_) => text,
        };
        let record = semio_framework_dsl_record::parse(body, &Self::__dsl_spec(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits { max_bytes: 272 * 1024 * 1024, ..semio_framework_diagnostic::Limits::default() }, mode: semio_framework_dsl_record::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = semio_framework_dsl_record::print(&self.__dsl_to_record(), &Self::__dsl_spec(), semio_framework_dsl_record::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}
}
pub use snapshot_codec::*;

mod source_fixtures {
use crate::STDIO_PDF_DOCUMENT_SCHEMA;
use crate::standards::v1_4::subsets::base::schema::snapshot::*;
/// 📄️ The demo `stdio.pdf` document — the single source of truth for `📚️examples/🎬️demo/🖼️assets/
/// 🗣️.dsl.semio`/`🎒️.pack.semio` (both are literally this snapshot's `print_dsl`/
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
    let bytes = crate::standards::v1_4::subsets::base::io::encode_pdf(&seed).expect("encode_pdf(seed) must succeed");
    crate::standards::v1_4::subsets::base::io::decode_pdf(&bytes).expect("decode_pdf(encode_pdf(seed)) must succeed")
}
}
pub use source_fixtures::*;
