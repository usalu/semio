//! 🚪️ IO stdio.pdf (1.7/✳️ua) — reuses the ✳️any subset's `binary`/`deflate` raw-codec DAG
//! leaves rather than duplicating them (same `PdfSnapshot` type, same catalog DAG edges).
//! Registration flows through `🎹️composer::register` (the `ComposerEntry` via the standard-level
//! aggregator, and the `SubsetValidator` directly), not per-leaf `register()` — same pattern
//! established by `✳️a/🚪️io` and `✳️any/🚪️io` for this artifact. ISO 14289-1:2014 (PDF/UA-1).
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::snapshot::PdfSnapshot;
    use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::PdfComposer as PdfAnyComposer;
    use crate::artifacts::pdf::standards::v1_7::subsets::ua::schema::check_ua_conformance;
    use dsl::{Diagnostic, FaultCode, Severity, TextSpan};
    use semio_framework_plugin::{register_subset_validator, subset_validator_entry_of, ArtifactComposition, ComposeError, ComposeSource, Composition, Dialect, IoPayload, StandardId, SubsetId, SubsetValidator, SubsetValidatorEntry};
    use std::sync::OnceLock;

    const DIALECT_UA: Dialect = Dialect { artifact_kind: "s.stdio.pdf", standard: StandardId("1.7"), subset: SubsetId("ua") };
    const DIALECT_ANY: Dialect = Dialect { artifact_kind: "s.stdio.pdf", standard: StandardId("1.7"), subset: SubsetId("*") };
    const DEP_BINARY: Dialect = Dialect { artifact_kind: "s.stdio.binary", standard: StandardId("raw"), subset: SubsetId("*") };
    const DEP_DEFLATE: Dialect = Dialect { artifact_kind: "s.stdio.deflate", standard: StandardId("rfc1950"), subset: SubsetId("*") };

    //#region 🔖️Composer
    pub struct PdfUaComposerComposition;

    impl ArtifactComposition for PdfUaComposerComposition {
        type Snapshot = PdfSnapshot;
        const WRITES: Dialect = DIALECT_UA;

        async fn reads() -> &'static [Dialect] {
            &[DIALECT_ANY, DIALECT_UA, DEP_BINARY, DEP_DEFLATE]
        }

        async fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            let inner = semio_framework_plugin::resolve_ready(PdfAnyComposer::compose(sources))?;
            let checks = check_ua_conformance(&inner.snapshot);
            let (hard, soft): (Vec<Diagnostic>, Vec<Diagnostic>) = checks.into_iter().partition(|d| matches!(d.severity, Severity::Error | Severity::Fatal));
            if !hard.is_empty() {
                let mut all = hard.clone();
                all.extend(soft);
                return Err(ComposeError { message: format!("PDF/UA-1 conformance violated: {} hard issue(s) -- not stamping the ua dialect", hard.len()), diagnostics: all });
            }
            let mut diagnostics = inner.diagnostics;
            diagnostics.extend(soft);
            Ok(Composition { snapshot: inner.snapshot, confidence: inner.confidence, diagnostics })
        }
    }
    //#endregion 🔖️Composer

    //#region 🔖️SubsetValidator
    pub struct PdfUaValidator;

    impl SubsetValidator for PdfUaValidator {
        const DIALECT: Dialect = DIALECT_UA;

        async fn validate(payload: &IoPayload) -> Vec<Diagnostic> {
            let decoded = match payload {
                IoPayload::Binary(bytes) => <PdfSnapshot as store::ArtifactPack>::decode_pack(bytes).ok(),
                IoPayload::Text(text) => <PdfSnapshot as store::ArtifactDsl>::parse_dsl(text).ok(),
            };
            match decoded {
                Some(snapshot) => check_ua_conformance(&snapshot),
                None => vec![Diagnostic {
                    code: FaultCode::new("stdio.pdf.ua.validate-decode-failed"),
                    severity: Severity::Warning,
                    span: TextSpan::at(1, 1),
                    message: "PDF/UA SubsetValidator: payload did not decode as a PdfSnapshot -- skipped".into(),
                    expected: None,
                    scope: dsl::FaultScope::default(),
                }],
            }
        }
    }

    static VALIDATOR_ENTRY: OnceLock<SubsetValidatorEntry> = OnceLock::new();

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn validator_entry() -> &'static SubsetValidatorEntry {
        VALIDATOR_ENTRY.get_or_init(subset_validator_entry_of::<PdfUaValidator>)
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register() {
        let _ = register_subset_validator(validator_entry());
    }
    //#endregion 🔖️SubsetValidator

    #[cfg(test)]
    mod tests {
        use super::*;
        use semio_framework_plugin::AnalyzeSource;

        /// 🩹 The 1.7 writer (`encode_pdf`) deliberately does NOT re-emit `PdfSnapshot.objects` (see
        /// its own doc comment — asserted structurally, not byte-for-byte), so a builder-seeded
        /// MarkInfo/StructTreeRoot can never round-trip through `encode_pack`/`decode_pack`. Hand-craft
        /// bytes and route through `AnalyzeSource::Text` instead (`decode_pdf` parses the FULL real
        /// object graph) — same pattern `✳️a`'s and `✳️x`'s own composer tests already use.
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        fn minimal_conforming_ua_pdf() -> Vec<u8> {
            let mut body = Vec::new();
            body.extend_from_slice(b"%PDF-1.7\n");
            let o1 = body.len();
            body.extend_from_slice(b"1 0 obj\n<< /Type /Catalog /Pages 2 0 R /MarkInfo 4 0 R /StructTreeRoot 5 0 R /Lang (en-US) /ViewerPreferences 6 0 R >>\nendobj\n");
            let o2 = body.len();
            body.extend_from_slice(b"2 0 obj\n<< /Type /Pages /Kids [3 0 R] /Count 1 >>\nendobj\n");
            let o3 = body.len();
            body.extend_from_slice(b"3 0 obj\n<< /Type /Page /Parent 2 0 R /MediaBox [0 0 200 200] /Resources << >> >>\nendobj\n");
            let o4 = body.len();
            body.extend_from_slice(b"4 0 obj\n<< /Marked true >>\nendobj\n");
            let o5 = body.len();
            body.extend_from_slice(b"5 0 obj\n<< /Type /StructTreeRoot >>\nendobj\n");
            let o6 = body.len();
            body.extend_from_slice(b"6 0 obj\n<< /DisplayDocTitle true >>\nendobj\n");
            let xref = body.len();
            body.extend_from_slice(b"xref\n0 7\n0000000000 65535 f \n");
            for off in [o1, o2, o3, o4, o5, o6] {
                body.extend_from_slice(format!("{off:010} 00000 n \n").as_bytes());
            }
            body.extend_from_slice(format!("trailer\n<< /Size 7 /Root 1 0 R >>\nstartxref\n{xref}\n%%EOF\n").as_bytes());
            body
        }

        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        fn hex_encode(bytes: &[u8]) -> String {
            bytes.iter().map(|b| format!("{b:02x}")).collect()
        }

        #[semio_framework_async_macros::async_test]
        async fn conforming_builder_snapshot_composes_and_stamps_ua() {
            let bytes = minimal_conforming_ua_pdf();
            let hex = hex_encode(&bytes);
            let sources = vec![ComposeSource { dialect: DIALECT_ANY, payload: AnalyzeSource::Text(&hex) }];
            let composed = PdfUaComposerComposition::compose(&sources).expect("clean document must compose to ua");
            assert!(composed.diagnostics.iter().all(|d| d.severity != Severity::Error), "no hard diagnostics expected: {:?}", composed.diagnostics);
        }

        #[semio_framework_async_macros::async_test]
        async fn missing_markinfo_fails_compose() {
            let snapshot = PdfSnapshot::default();
            let bytes = <PdfSnapshot as store::ArtifactPack>::encode_pack(&snapshot);
            let sources = vec![ComposeSource { dialect: DIALECT_ANY, payload: AnalyzeSource::Binary(&bytes) }];
            let err = PdfUaComposerComposition::compose(&sources).expect_err("an untagged document must not stamp ua");
            assert!(err.diagnostics.iter().any(|d| d.code.0 == crate::artifacts::pdf::standards::v1_7::subsets::ua::schema::CODE_MARKINFO), "got {:?}", err.diagnostics);
        }
    }
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition
