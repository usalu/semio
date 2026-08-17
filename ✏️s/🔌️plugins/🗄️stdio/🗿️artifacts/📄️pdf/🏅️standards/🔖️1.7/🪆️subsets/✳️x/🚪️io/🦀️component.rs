//! 🚪️ IO stdio.pdf (1.7/✳️x) — reuses the ✳️any subset's `binary`/`deflate` raw-codec DAG
//! leaves rather than duplicating them (same `PdfSnapshot` type, same catalog DAG edges).
//! Registration flows through `🎹️composer::register` (the `ComposerEntry` via the standard-level
//! aggregator, and the `SubsetValidator` directly), not per-leaf `register()` — same pattern
//! established by `✳️a/🚪️io` and `✳️any/🚪️io` for this artifact. ISO 15930-7:2010 (PDF/X-4).
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::snapshot::PdfSnapshot;
    use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::PdfComposer as PdfAnyComposer;
    use crate::artifacts::pdf::standards::v1_7::subsets::x::schema::check_x_conformance;
    use dsl::{Diagnostic, FaultCode, Severity, TextSpan};
    use semio_framework_plugin::{register_subset_validator, subset_validator_entry_of, ArtifactComposition, ComposeError, ComposeSource, Composition, Dialect, IoPayload, StandardId, SubsetId, SubsetValidator, SubsetValidatorEntry};
    use std::sync::OnceLock;

    pub(crate) const DIALECT_X: Dialect = Dialect { artifact_kind: "s.stdio.pdf", standard: StandardId("1.7"), subset: SubsetId("x") };
    const DIALECT_ANY: Dialect = Dialect { artifact_kind: "s.stdio.pdf", standard: StandardId("1.7"), subset: SubsetId("*") };
    const DEP_BINARY: Dialect = Dialect { artifact_kind: "s.stdio.binary", standard: StandardId("raw"), subset: SubsetId("*") };
    const DEP_DEFLATE: Dialect = Dialect { artifact_kind: "s.stdio.deflate", standard: StandardId("rfc1950"), subset: SubsetId("*") };

    //#region 🔖️Composer
    pub struct PdfXComposerComposition;

    impl ArtifactComposition for PdfXComposerComposition {
        type Snapshot = PdfSnapshot;
        const WRITES: Dialect = DIALECT_X;

        fn reads() -> &'static [Dialect] {
            &[DIALECT_ANY, DIALECT_X, DEP_BINARY, DEP_DEFLATE]
        }

        fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            let inner = PdfAnyComposer::compose(sources)?;
            let checks = check_x_conformance(&inner.snapshot);
            let (hard, soft): (Vec<Diagnostic>, Vec<Diagnostic>) = checks.into_iter().partition(|d| matches!(d.severity, Severity::Error | Severity::Fatal));
            if !hard.is_empty() {
                let mut all = hard.clone();
                all.extend(soft);
                return Err(ComposeError { message: format!("PDF/X-4 conformance violated: {} hard issue(s) -- not stamping the x dialect", hard.len()), diagnostics: all });
            }
            let mut diagnostics = inner.diagnostics;
            diagnostics.extend(soft);
            Ok(Composition { snapshot: inner.snapshot, confidence: inner.confidence, diagnostics })
        }
    }
    //#endregion 🔖️Composer

    //#region 🔖️SubsetValidator
    pub struct PdfXValidator;

    impl SubsetValidator for PdfXValidator {
        const DIALECT: Dialect = DIALECT_X;

        fn validate(payload: &IoPayload) -> Vec<Diagnostic> {
            let decoded = match payload {
                IoPayload::Binary(bytes) => <PdfSnapshot as store::ArtifactPack>::decode_pack(bytes).ok(),
                IoPayload::Text(text) => <PdfSnapshot as store::ArtifactDsl>::parse_dsl(text).ok(),
            };
            match decoded {
                Some(snapshot) => check_x_conformance(&snapshot),
                None => vec![Diagnostic {
                    code: FaultCode::new("stdio.pdf.x.validate-decode-failed"),
                    severity: Severity::Warning,
                    span: TextSpan::at(1, 1),
                    message: "PDF/X SubsetValidator: payload did not decode as a PdfSnapshot -- skipped".into(),
                    expected: None,
                    scope: dsl::FaultScope::default(),
                }],
            }
        }
    }

    static VALIDATOR_ENTRY: OnceLock<SubsetValidatorEntry> = OnceLock::new();

    fn validator_entry() -> &'static SubsetValidatorEntry {
        VALIDATOR_ENTRY.get_or_init(subset_validator_entry_of::<PdfXValidator>)
    }

    /// 📌️ Registers this subset's `SubsetValidator` with the generic io registry. Called from the
    /// 1.7 standard's own `⚙️engine::register()`. The `ComposerEntry` itself is registered separately
    /// by the standard-level composer aggregator (`composer::entries()`).
    pub fn register() {
        let _ = register_subset_validator(validator_entry());
    }
    //#endregion 🔖️SubsetValidator

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::artifacts::pdf::standards::v1_7::subsets::x::schema::PdfXBuilderConstruction as PdfXBuilder;
        use semio_framework_plugin::AnalyzeSource;
        use semio_framework_plugin::ArtifactBuilder as _;

        /// 🩹 A genuinely PDF/X-4-conforming raw fixture: the 1.7 writer (`encode_pdf`) deliberately
        /// does NOT re-emit `PdfSnapshot.objects` (see its own doc comment — asserted structurally,
        /// not byte-for-byte), so `PdfXBuilder::new(...).build()` -> `encode_pack` -> `decode_pack`
        /// can never round-trip the OutputIntent/TrimBox it seeds (same documented gap
        /// `subset_validator_recheck_runs_the_same_check` below already accounts for). Hand-crafting
        /// bytes and routing through `AnalyzeSource::Text` (which `decode_pdf`s the FULL real object
        /// graph, unlike `encode_pdf`) is the same pattern `✳️a`'s own composer tests already use for
        /// `minimal_pdf_with_extra_object` — this is that same pattern, positive-path.
        fn minimal_conforming_x_pdf() -> Vec<u8> {
            let mut body = Vec::new();
            body.extend_from_slice(b"%PDF-1.7\n");
            let o1 = body.len();
            body.extend_from_slice(b"1 0 obj\n<< /Type /Catalog /Pages 2 0 R /OutputIntents [4 0 R] >>\nendobj\n");
            let o2 = body.len();
            body.extend_from_slice(b"2 0 obj\n<< /Type /Pages /Kids [3 0 R] /Count 1 >>\nendobj\n");
            let o3 = body.len();
            body.extend_from_slice(b"3 0 obj\n<< /Type /Page /Parent 2 0 R /MediaBox [0 0 200 200] /TrimBox [0 0 200 200] /Resources << >> >>\nendobj\n");
            let o4 = body.len();
            body.extend_from_slice(b"4 0 obj\n<< /Type /OutputIntent /S /GTS_PDFX /OutputConditionIdentifier (sRGB IEC61966-2.1) /DestOutputProfile 5 0 R >>\nendobj\n");
            let o5 = body.len();
            body.extend_from_slice(b"5 0 obj\n<< /N 3 >>\nendobj\n");
            let xref = body.len();
            body.extend_from_slice(b"xref\n0 6\n0000000000 65535 f \n");
            for off in [o1, o2, o3, o4, o5] {
                body.extend_from_slice(format!("{off:010} 00000 n \n").as_bytes());
            }
            body.extend_from_slice(format!("trailer\n<< /Size 6 /Root 1 0 R >>\nstartxref\n{xref}\n%%EOF\n").as_bytes());
            body
        }

        fn hex_encode(bytes: &[u8]) -> String {
            bytes.iter().map(|b| format!("{b:02x}")).collect()
        }

        #[test]
        fn conforming_builder_snapshot_composes_and_stamps_x() {
            let bytes = minimal_conforming_x_pdf();
            let hex = hex_encode(&bytes);
            let sources = vec![ComposeSource { dialect: DIALECT_ANY, payload: AnalyzeSource::Text(&hex) }];
            let composed = PdfXComposerComposition::compose(&sources).expect("clean document must compose to x");
            assert!(composed.diagnostics.iter().all(|d| d.severity != Severity::Error), "no hard diagnostics expected: {:?}", composed.diagnostics);
        }

        #[test]
        fn missing_output_intent_fails_compose() {
            let snapshot = PdfSnapshot::default();
            let bytes = <PdfSnapshot as store::ArtifactPack>::encode_pack(&snapshot);
            let sources = vec![ComposeSource { dialect: DIALECT_ANY, payload: AnalyzeSource::Binary(&bytes) }];
            let err = PdfXComposerComposition::compose(&sources).expect_err("a document with no OutputIntent must not stamp x");
            assert!(err.diagnostics.iter().any(|d| d.code.0 == crate::artifacts::pdf::standards::v1_7::subsets::x::schema::CODE_OUTPUT_INTENT), "got {:?}", err.diagnostics);
        }

        #[test]
        fn subset_validator_recheck_runs_the_same_check() {
            let snapshot = PdfXBuilder::new("sRGB IEC61966-2.1").add_page(crate::artifacts::pdf::standards::v1_7::subsets::any::schema::snapshot::PdfPage::new(50.0, 50.0)).build().unwrap();
            let bytes = <PdfSnapshot as store::ArtifactPack>::encode_pack(&snapshot);
            let diagnostics = PdfXValidator::validate(&IoPayload::Binary(bytes));
            // The 1.7 writer doesn't re-serialize `objects`, so the wire recheck honestly re-reports
            // the OutputIntent/TrimBox as missing (same documented gap as ✳️a's own validator test).
            assert!(diagnostics.iter().any(|d| d.code.0 == crate::artifacts::pdf::standards::v1_7::subsets::x::schema::CODE_OUTPUT_INTENT), "got {diagnostics:?}");
        }
    }
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition
