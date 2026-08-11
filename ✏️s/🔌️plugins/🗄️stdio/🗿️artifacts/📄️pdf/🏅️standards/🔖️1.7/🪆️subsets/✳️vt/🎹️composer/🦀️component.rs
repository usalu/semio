//! 🎹️ PdfVtComposer (1.7/✳️vt) — reads the same sources ✳️any does, delegates parsing to it,
//! then HARD-GATES the `vt` dialect stamp on real ISO 16612-2:2010 (PDF/VT-1/-2) conformance
//! (`check_vt_conformance`, which itself layers on `✳️x`'s ISO 15930-7 checks). A hard violation
//! (missing `/DPartRoot`, or any inherited X-4 hard violation) fails composition; a soft one
//! (DPart missing `/DPM`, or any inherited X-4 soft violation) passes through as an advisory
//! diagnostic. Also registers this dialect's `SubsetValidator`. Ticket
//! 26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES W3.

use std::sync::OnceLock;
use dsl::{Diagnostic, FaultCode, Severity, TextSpan};
use semio_framework_plugin::{
    ArtifactComposer, ComposeError, ComposeSource, Composition, Dialect, IoPayload, StandardId, SubsetId, SubsetValidator, SubsetValidatorEntry,
    register_subset_validator, subset_validator_entry_of,
};
use crate::artifacts::pdf::standards::v1_7::subsets::vt::analyzer::check_vt_conformance;
use crate::artifacts::pdf::standards::v1_7::subsets::any::composer::PdfComposer as PdfAnyComposer;
use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::snapshot::PdfSnapshot;

const DIALECT_VT: Dialect = Dialect { artifact_kind: "s.stdio.pdf", standard: StandardId("1.7"), subset: SubsetId("vt") };
const DIALECT_ANY: Dialect = Dialect { artifact_kind: "s.stdio.pdf", standard: StandardId("1.7"), subset: SubsetId("*") };
const DIALECT_X: Dialect = Dialect { artifact_kind: "s.stdio.pdf", standard: StandardId("1.7"), subset: SubsetId("x") };
const DEP_BINARY: Dialect = Dialect { artifact_kind: "s.stdio.binary", standard: StandardId("raw"), subset: SubsetId("*") };
const DEP_DEFLATE: Dialect = Dialect { artifact_kind: "s.stdio.deflate", standard: StandardId("rfc1950"), subset: SubsetId("*") };

//#region 🔖️Composer
pub struct PdfVtComposer;

impl ArtifactComposer for PdfVtComposer {
    type Snapshot = PdfSnapshot;
    const WRITES: Dialect = DIALECT_VT;

    /// 📚️ Reads `✳️x` alongside `✳️any`/self/deps -- VT is layered on X-4 (ISO 16612-2 is based
    /// on ISO 15930-7), matching the catalog DAG relationship the roster describes.
    fn reads() -> &'static [Dialect] {
        &[DIALECT_ANY, DIALECT_X, DIALECT_VT, DEP_BINARY, DEP_DEFLATE]
    }

    fn compose(sources: &[ComposeSource]) -> Result<Composition<Self::Snapshot>, ComposeError> {
        let inner = PdfAnyComposer::compose(sources)?;
        let checks = check_vt_conformance(&inner.snapshot);
        let (hard, soft): (Vec<Diagnostic>, Vec<Diagnostic>) = checks.into_iter().partition(|d| matches!(d.severity, Severity::Error | Severity::Fatal));
        if !hard.is_empty() {
            let mut all = hard.clone();
            all.extend(soft);
            return Err(ComposeError {
                message: format!("PDF/VT-1/-2 conformance violated: {} hard issue(s) -- not stamping the vt dialect", hard.len()),
                diagnostics: all,
            });
        }
        let mut diagnostics = inner.diagnostics;
        diagnostics.extend(soft);
        Ok(Composition { snapshot: inner.snapshot, confidence: inner.confidence, diagnostics })
    }
}
//#endregion 🔖️Composer

//#region 🔖️SubsetValidator
pub struct PdfVtValidator;

impl SubsetValidator for PdfVtValidator {
    const DIALECT: Dialect = DIALECT_VT;

    fn validate(payload: &IoPayload) -> Vec<Diagnostic> {
        let decoded = match payload {
            IoPayload::Binary(bytes) => <PdfSnapshot as store::ArtifactPack>::decode_pack(bytes).ok(),
            IoPayload::Text(text) => <PdfSnapshot as store::ArtifactDsl>::parse_dsl(text).ok(),
        };
        match decoded {
            Some(snapshot) => check_vt_conformance(&snapshot),
            None => vec![Diagnostic {
                code: FaultCode::new("stdio.pdf.vt.validate-decode-failed"),
                severity: Severity::Warning,
                span: TextSpan::at(1, 1),
                message: "PDF/VT SubsetValidator: payload did not decode as a PdfSnapshot -- skipped".into(),
                expected: None,
                scope: dsl::FaultScope::default(),
            }],
        }
    }
}

static VALIDATOR_ENTRY: OnceLock<SubsetValidatorEntry> = OnceLock::new();

fn validator_entry() -> &'static SubsetValidatorEntry {
    VALIDATOR_ENTRY.get_or_init(subset_validator_entry_of::<PdfVtValidator>)
}

pub fn register() {
    register_subset_validator(validator_entry());
}
//#endregion 🔖️SubsetValidator

#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_plugin::AnalyzeSource;

    /// 🩹 The 1.7 writer (`encode_pdf`) deliberately does NOT re-emit `PdfSnapshot.objects` (see
    /// its own doc comment — asserted structurally, not byte-for-byte), so a builder-seeded
    /// OutputIntent/TrimBox/DPartRoot can never round-trip through `encode_pack`/`decode_pack`.
    /// Hand-craft bytes and route through `AnalyzeSource::Text` instead (`decode_pdf` parses the
    /// FULL real object graph) — same pattern `✳️a`'s/`✳️x`'s/`✳️ua`'s own composer tests use.
    fn minimal_conforming_vt_pdf() -> Vec<u8> {
        let mut body = Vec::new();
        body.extend_from_slice(b"%PDF-1.7\n");
        let o1 = body.len();
        body.extend_from_slice(b"1 0 obj\n<< /Type /Catalog /Pages 2 0 R /OutputIntents [4 0 R] /DPartRoot 6 0 R >>\nendobj\n");
        let o2 = body.len();
        body.extend_from_slice(b"2 0 obj\n<< /Type /Pages /Kids [3 0 R] /Count 1 >>\nendobj\n");
        let o3 = body.len();
        body.extend_from_slice(b"3 0 obj\n<< /Type /Page /Parent 2 0 R /MediaBox [0 0 200 200] /TrimBox [0 0 200 200] /Resources << >> >>\nendobj\n");
        let o4 = body.len();
        body.extend_from_slice(b"4 0 obj\n<< /Type /OutputIntent /S /GTS_PDFX /OutputConditionIdentifier (sRGB IEC61966-2.1) /DestOutputProfile 5 0 R >>\nendobj\n");
        let o5 = body.len();
        body.extend_from_slice(b"5 0 obj\n<< /N 3 >>\nendobj\n");
        let o6 = body.len();
        body.extend_from_slice(b"6 0 obj\n<< /Type /DPartRoot /DParts [7 0 R] >>\nendobj\n");
        let o7 = body.len();
        body.extend_from_slice(b"7 0 obj\n<< /Type /DPart /DPM 8 0 R >>\nendobj\n");
        let o8 = body.len();
        body.extend_from_slice(b"8 0 obj\n<< >>\nendobj\n");
        let xref = body.len();
        body.extend_from_slice(b"xref\n0 9\n0000000000 65535 f \n");
        for off in [o1, o2, o3, o4, o5, o6, o7, o8] {
            body.extend_from_slice(format!("{off:010} 00000 n \n").as_bytes());
        }
        body.extend_from_slice(format!("trailer\n<< /Size 9 /Root 1 0 R >>\nstartxref\n{xref}\n%%EOF\n").as_bytes());
        body
    }

    fn hex_encode(bytes: &[u8]) -> String {
        bytes.iter().map(|b| format!("{b:02x}")).collect()
    }

    #[test]
    fn conforming_builder_snapshot_composes_and_stamps_vt() {
        let bytes = minimal_conforming_vt_pdf();
        let hex = hex_encode(&bytes);
        let sources = vec![ComposeSource { dialect: DIALECT_ANY, payload: AnalyzeSource::Text(&hex) }];
        let composed = PdfVtComposer::compose(&sources).expect("clean document must compose to vt");
        assert!(composed.diagnostics.iter().all(|d| d.severity != Severity::Error), "no hard diagnostics expected: {:?}", composed.diagnostics);
    }

    #[test]
    fn missing_dpartroot_fails_compose() {
        let snapshot = PdfSnapshot::default();
        let bytes = <PdfSnapshot as store::ArtifactPack>::encode_pack(&snapshot);
        let sources = vec![ComposeSource { dialect: DIALECT_ANY, payload: AnalyzeSource::Binary(&bytes) }];
        let err = PdfVtComposer::compose(&sources).expect_err("a document with no DPartRoot must not stamp vt");
        assert!(err.diagnostics.iter().any(|d| d.severity == Severity::Error), "got {:?}", err.diagnostics);
    }
}
