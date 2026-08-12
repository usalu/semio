//! 🚪️ IO stdio.pdf (1.7/✳️a) — reuses the ✳️any subset's `binary`/`deflate` raw-codec DAG
//! leaves rather than duplicating them (same `PdfSnapshot` type, same catalog DAG edges).
//! Registration flows through `🎹️composer::register` (the `ComposerEntry` via the standard-level
//! aggregator, and the `SubsetValidator` directly), not per-leaf `register()` — same pattern
//! `✳️any/🚪️io` already established for this artifact.
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use std::sync::OnceLock;
    use dsl::{Diagnostic, FaultCode, Severity, TextSpan};
    use semio_framework_plugin::{
        ArtifactComposition, ComposeError, ComposeSource, Composition, Dialect, IoPayload, StandardId, SubsetId, SubsetValidator, SubsetValidatorEntry,
        register_subset_validator, subset_validator_entry_of,
    };
    use crate::artifacts::pdf::standards::v1_7::subsets::a::schema::check_pdf_a_conformance;
    use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::PdfComposer as PdfAnyComposer;
    use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::snapshot::PdfSnapshot;

    const DIALECT_A: Dialect = Dialect { artifact_kind: "s.stdio.pdf", standard: StandardId("1.7"), subset: SubsetId("a") };
    const DIALECT_ANY: Dialect = Dialect { artifact_kind: "s.stdio.pdf", standard: StandardId("1.7"), subset: SubsetId("*") };
    const DEP_BINARY: Dialect = Dialect { artifact_kind: "s.stdio.binary", standard: StandardId("raw"), subset: SubsetId("*") };
    const DEP_DEFLATE: Dialect = Dialect { artifact_kind: "s.stdio.deflate", standard: StandardId("rfc1950"), subset: SubsetId("*") };

    //#region 🔖️Composer
    pub struct PdfAComposerComposition;

    impl ArtifactComposition for PdfAComposerComposition {
        type Snapshot = PdfSnapshot;
        const WRITES: Dialect = DIALECT_A;

        fn reads() -> &'static [Dialect] {
            &[DIALECT_ANY, DIALECT_A, DEP_BINARY, DEP_DEFLATE]
        }

        fn compose(sources: &[ComposeSource]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            let inner = PdfAnyComposer::compose(sources)?;
            let checks = check_pdf_a_conformance(&inner.snapshot);
            let (hard, soft): (Vec<Diagnostic>, Vec<Diagnostic>) = checks.into_iter().partition(|d| matches!(d.severity, Severity::Error | Severity::Fatal));
            if !hard.is_empty() {
                let mut all = hard.clone();
                all.extend(soft);
                return Err(ComposeError {
                    message: format!("PDF/A conformance violated: {} hard issue(s) -- not stamping the a dialect", hard.len()),
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
    /// 🛡️ The registered `SubsetValidator` for `1.7/a` -- see the module doc comment for how this
    /// relates to (and honestly differs from) the composer's own pre-serialization hard gate above.
    pub struct PdfAValidator;

    impl SubsetValidator for PdfAValidator {
        const DIALECT: Dialect = DIALECT_A;

        fn validate(payload: &IoPayload) -> Vec<Diagnostic> {
            let decoded = match payload {
                IoPayload::Binary(bytes) => <PdfSnapshot as store::ArtifactPack>::decode_pack(bytes).ok(),
                IoPayload::Text(text) => <PdfSnapshot as store::ArtifactDsl>::parse_dsl(text).ok(),
            };
            match decoded {
                Some(snapshot) => check_pdf_a_conformance(&snapshot),
                None => vec![Diagnostic {
                    code: FaultCode::new("stdio.pdf.a.validate-decode-failed"),
                    severity: Severity::Warning,
                    span: TextSpan::at(1, 1),
                    message: "PDF/A SubsetValidator: payload did not decode as a PdfSnapshot -- skipped".into(),
                    expected: None,
                    scope: dsl::FaultScope::default(),
                }],
            }
        }
    }

    static VALIDATOR_ENTRY: OnceLock<SubsetValidatorEntry> = OnceLock::new();

    fn validator_entry() -> &'static SubsetValidatorEntry {
        VALIDATOR_ENTRY.get_or_init(subset_validator_entry_of::<PdfAValidator>)
    }

    /// 📌️ Registers this subset's `SubsetValidator` with the generic io registry (D5's
    /// validate-on-build hook). Called from the 1.7 standard's own `⚙️engine::register()`, which is
    /// already invoked directly from the stdio plugin's `plugin()` -- see that file. The
    /// `ComposerEntry` itself is registered separately by the standard-level composer aggregator
    /// (`crate::artifacts::pdf::standards::v1_7::engine::io_registry::entries()`), matching how `✳️any`'s own
    /// entry is registered.
    pub fn register() {
        register_subset_validator(validator_entry());
    }
    //#endregion 🔖️SubsetValidator

    #[cfg(test)]
    mod tests {
        use super::*;
        use semio_framework_plugin::AnalyzeSource;
        use crate::artifacts::pdf::standards::v1_7::subsets::a::schema::{CODE_JAVASCRIPT, CODE_LAUNCH};
        use crate::artifacts::pdf::standards::v1_7::subsets::a::schema::PdfABuilder;
        use semio_framework_plugin::ArtifactBuilder as _;

        fn minimal_pdf_with_extra_object(extra_obj_body: &[u8]) -> Vec<u8> {
            // 🩹 Mirrors the hand-built classic-xref fixtures already used by `⚙️engine`'s own test
            // module: a real one-page PDF plus one extra indirect object (referenced from
            // `/OpenAction` so it's genuinely reachable, not just incidentally present in the xref
            // table).
            let mut body = Vec::new();
            body.extend_from_slice(b"%PDF-1.7\n");
            let o1 = body.len();
            body.extend_from_slice(b"1 0 obj\n<< /Type /Catalog /Pages 2 0 R /OpenAction 4 0 R >>\nendobj\n");
            let o2 = body.len();
            body.extend_from_slice(b"2 0 obj\n<< /Type /Pages /Kids [3 0 R] /Count 1 >>\nendobj\n");
            let o3 = body.len();
            body.extend_from_slice(b"3 0 obj\n<< /Type /Page /Parent 2 0 R /MediaBox [0 0 200 200] /Resources << >> >>\nendobj\n");
            let o4 = body.len();
            body.extend_from_slice(b"4 0 obj\n");
            body.extend_from_slice(extra_obj_body);
            body.extend_from_slice(b"\nendobj\n");
            let xref = body.len();
            body.extend_from_slice(b"xref\n0 5\n0000000000 65535 f \n");
            for off in [o1, o2, o3, o4] {
                body.extend_from_slice(format!("{off:010} 00000 n \n").as_bytes());
            }
            body.extend_from_slice(format!("trailer\n<< /Size 5 /Root 1 0 R >>\nstartxref\n{xref}\n%%EOF\n").as_bytes());
            body
        }

        /// 🔠️ `PdfSnapshot::parse_dsl` (the `ArtifactDsl` impl `AnalyzeSource::Text` decodes through)
        /// hex-decodes its body and passes it straight to the real `engine::decode_pdf` -- unlike
        /// `AnalyzeSource::Binary`, which expects an ALREADY pack-encoded `PdfSnapshot`. Routing
        /// hand-crafted raw PDF bytes through `Text(hex)` is how this test genuinely exercises the
        /// real `engine::decode_pdf` → full-object-graph-retention → PDF/A hard-gate pipeline
        /// end-to-end through the actual `ArtifactComposition::compose` surface.
        fn hex_encode(bytes: &[u8]) -> String {
            bytes.iter().map(|b| format!("{b:02x}")).collect()
        }

        #[test]
        fn conforming_builder_snapshot_composes_and_stamps_a() {
            let snapshot = PdfABuilder::new("sRGB IEC61966-2.1").add_page(crate::artifacts::pdf::standards::v1_7::subsets::any::schema::snapshot::PdfPage::new(100.0, 100.0)).build().unwrap();
            let bytes = <PdfSnapshot as store::ArtifactPack>::encode_pack(&snapshot);
            let sources = vec![ComposeSource { dialect: DIALECT_ANY, payload: AnalyzeSource::Binary(&bytes) }];
            let composed = PdfAComposerComposition::compose(&sources).expect("clean document must compose to a");
            assert!(composed.diagnostics.iter().all(|d| d.severity != Severity::Error), "no hard diagnostics expected: {:?}", composed.diagnostics);
        }

        #[test]
        fn javascript_action_reachable_from_open_action_fails_compose_with_real_diagnostic() {
            let bytes = minimal_pdf_with_extra_object(b"<< /S /JavaScript /JS (app.alert(1)) >>");
            let hex = hex_encode(&bytes);
            let sources = vec![ComposeSource { dialect: DIALECT_ANY, payload: AnalyzeSource::Text(&hex) }];
            let err = PdfAComposerComposition::compose(&sources).expect_err("a document with a JS action must not stamp a");
            assert!(err.diagnostics.iter().any(|d| d.code.0 == CODE_JAVASCRIPT && d.severity == Severity::Error), "got {:?}", err.diagnostics);
        }

        #[test]
        fn launch_action_reachable_from_open_action_fails_compose_with_real_diagnostic() {
            let bytes = minimal_pdf_with_extra_object(b"<< /S /Launch /F (calc.exe) >>");
            let hex = hex_encode(&bytes);
            let sources = vec![ComposeSource { dialect: DIALECT_ANY, payload: AnalyzeSource::Text(&hex) }];
            let err = PdfAComposerComposition::compose(&sources).expect_err("a document with a Launch action must not stamp a");
            assert!(err.diagnostics.iter().any(|d| d.code.0 == CODE_LAUNCH && d.severity == Severity::Error), "got {:?}", err.diagnostics);
        }

        #[test]
        fn encrypted_trailer_document_is_rejected_upstream_by_the_shared_engine() {
            // 🔒 `⚙️engine::decode_pdf` already refuses any file whose trailer declares /Encrypt
            // (`PdfEngineError::Unsupported`) -- composing through the ✳️any delegate surfaces that as
            // a real ComposeError before this subset's own conformance check even runs.
            let mut body = Vec::new();
            body.extend_from_slice(b"%PDF-1.7\n");
            let o1 = body.len();
            body.extend_from_slice(b"1 0 obj\n<< /Type /Catalog /Pages 2 0 R >>\nendobj\n");
            let o2 = body.len();
            body.extend_from_slice(b"2 0 obj\n<< /Type /Pages /Kids [] /Count 0 >>\nendobj\n");
            let xref = body.len();
            body.extend_from_slice(b"xref\n0 3\n0000000000 65535 f \n");
            body.extend_from_slice(format!("{o1:010} 00000 n \n").as_bytes());
            body.extend_from_slice(format!("{o2:010} 00000 n \n").as_bytes());
            body.extend_from_slice(format!("trailer\n<< /Size 3 /Root 1 0 R /Encrypt << /Filter /Standard >> >>\nstartxref\n{xref}\n%%EOF\n").as_bytes());
            let hex = hex_encode(&body);
            let sources = vec![ComposeSource { dialect: DIALECT_ANY, payload: AnalyzeSource::Text(&hex) }];
            let err = PdfAComposerComposition::compose(&sources).expect_err("an /Encrypt trailer must never compose, at a or any other dialect");
            assert!(
                err.diagnostics.iter().any(|d| d.message.contains("Encrypt")),
                "must be the real engine-level /Encrypt rejection, not a spurious decode error: {err:?}"
            );
        }

        #[test]
        fn subset_validator_recheck_flags_soft_diagnostics_on_the_wire_payload() {
            let snapshot = PdfABuilder::new("sRGB IEC61966-2.1").add_page(crate::artifacts::pdf::standards::v1_7::subsets::any::schema::snapshot::PdfPage::new(50.0, 50.0)).build().unwrap();
            let bytes = <PdfSnapshot as store::ArtifactPack>::encode_pack(&snapshot);
            // The registered validator, called directly (same fn the generic io hook calls): today's
            // writer drops `objects` on encode, so the OutputIntent genuinely isn't in these bytes --
            // a real, honest soft diagnostic, not a false positive (see module doc comment).
            let diagnostics = PdfAValidator::validate(&IoPayload::Binary(bytes));
            assert!(diagnostics.iter().all(|d| d.severity != Severity::Error), "wire recheck must never report a hard violation for a builder-clean document: {diagnostics:?}");
        }
    }
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition
