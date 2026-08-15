//! 🚪️ IO stdio.docx (ecma-376/✳️transitional) — doc-leaf only, referencing the owning ✳️any
//! subset's import/export tree (`🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🚪️io/`) rather than
//! duplicating it. Registration flows through `🎹️composer::register` (the `SubsetValidator`
//! directly, the `ComposerEntry` via the standard-level aggregator), not per-leaf `register()`.
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::artifacts::docx::standards::v_ecma_376::subsets::any::schema::DocxComposer as DocxAnyComposer;
    use crate::artifacts::docx::standards::v_ecma_376::subsets::transitional::schema::check_transitional_conformance;
    use crate::artifacts::docx::DocxSnapshot;
    use dsl::{Diagnostic, FaultCode, Severity, TextSpan};
    use semio_framework_plugin::{register_subset_validator, subset_validator_entry_of, ArtifactComposition, ComposeError, ComposeSource, Composition, Dialect, IoPayload, StandardId, SubsetId, SubsetValidator, SubsetValidatorEntry};
    use std::sync::OnceLock;

    const DIALECT_TRANSITIONAL: Dialect = Dialect { artifact_kind: "s.stdio.docx", standard: StandardId("ecma-376"), subset: SubsetId("transitional") };
    const DIALECT_ANY: Dialect = Dialect { artifact_kind: "s.stdio.docx", standard: StandardId("ecma-376"), subset: SubsetId("*") };
    const DEP_ZIP: Dialect = Dialect { artifact_kind: "s.stdio.zip", standard: StandardId("2.0"), subset: SubsetId("*") };
    const DEP_XML: Dialect = Dialect { artifact_kind: "s.stdio.xml", standard: StandardId("1.0"), subset: SubsetId("*") };

    //#region 🔖️Composer
    pub struct DocxTransitionalComposerComposition;

    impl ArtifactComposition for DocxTransitionalComposerComposition {
        type Snapshot = DocxSnapshot;
        const WRITES: Dialect = DIALECT_TRANSITIONAL;

        fn reads() -> &'static [Dialect] {
            &[DIALECT_ANY, DIALECT_TRANSITIONAL, DEP_ZIP, DEP_XML]
        }

        fn compose(sources: &[ComposeSource]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            let inner = DocxAnyComposer::compose(sources)?;
            let checks = check_transitional_conformance(&inner.snapshot);
            let (hard, soft): (Vec<Diagnostic>, Vec<Diagnostic>) = checks.into_iter().partition(|d| matches!(d.severity, Severity::Error | Severity::Fatal));
            if !hard.is_empty() {
                let mut all = hard.clone();
                all.extend(soft);
                return Err(ComposeError { message: format!("ISO/IEC 29500-4 Transitional conformance violated: {} hard issue(s) -- not stamping the transitional dialect", hard.len()), diagnostics: all });
            }
            let mut diagnostics = inner.diagnostics;
            diagnostics.extend(soft);
            Ok(Composition { snapshot: inner.snapshot, confidence: inner.confidence, diagnostics })
        }
    }
    //#endregion 🔖️Composer

    //#region 🔖️SubsetValidator
    /// 🛡️ The registered `SubsetValidator` for `ecma-376/transitional` -- see the module doc comment
    /// for how this relates to the composer's own pre-serialization hard gate above.
    pub struct DocxTransitionalValidator;

    impl SubsetValidator for DocxTransitionalValidator {
        const DIALECT: Dialect = DIALECT_TRANSITIONAL;

        fn validate(payload: &IoPayload) -> Vec<Diagnostic> {
            let decoded = match payload {
                IoPayload::Binary(bytes) => <DocxSnapshot as store::ArtifactPack>::decode_pack(bytes).ok(),
                IoPayload::Text(text) => <DocxSnapshot as store::ArtifactDsl>::parse_dsl(text).ok(),
            };
            match decoded {
                Some(snapshot) => check_transitional_conformance(&snapshot),
                None => vec![Diagnostic {
                    code: FaultCode::new("stdio.docx.transitional.validate-decode-failed"),
                    severity: Severity::Warning,
                    span: TextSpan::at(1, 1),
                    message: "docx transitional SubsetValidator: payload did not decode as a DocxSnapshot -- skipped".into(),
                    expected: None,
                    scope: dsl::FaultScope::default(),
                }],
            }
        }
    }

    static VALIDATOR_ENTRY: OnceLock<SubsetValidatorEntry> = OnceLock::new();

    fn validator_entry() -> &'static SubsetValidatorEntry {
        VALIDATOR_ENTRY.get_or_init(subset_validator_entry_of::<DocxTransitionalValidator>)
    }

    /// 📌️ Registers this subset's `SubsetValidator` with the generic io registry (D5's
    /// validate-on-build hook). Called from the ecma-376 standard's own `⚙️engine::register()`. The
    /// `ComposerEntry` itself is aggregated separately by the standard-level composer
    /// (`crate::artifacts::docx::standards::v_ecma_376::engine::io_registry::entries()`).
    pub fn register() {
        register_subset_validator(validator_entry());
    }
    //#endregion 🔖️SubsetValidator

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::artifacts::docx::standards::v_ecma_376::subsets::transitional::schema::CODE_STRICT_NS_PRESENT;
        use crate::artifacts::zip::opc::{OpcPackage, RELS_CONTENT_TYPE, REL_TYPE_OFFICE_DOCUMENT};
        use semio_framework_plugin::AnalyzeSource;

        const TRANSITIONAL_MAIN_NS: &str = "http://schemas.openxmlformats.org/wordprocessingml/2006/main";

        fn transitional_snapshot() -> DocxSnapshot {
            let mut opc = OpcPackage::empty();
            opc.content_types.set_default("rels", RELS_CONTENT_TYPE);
            opc.content_types.set_default("xml", "application/xml");
            opc.set_part("word/document.xml", "application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml", format!(r#"<w:document xmlns:w="{TRANSITIONAL_MAIN_NS}"><w:body/></w:document>"#).into_bytes());
            opc.add_relationship("", "rId1", REL_TYPE_OFFICE_DOCUMENT, "word/document.xml");
            DocxSnapshot::from_parts(opc, Default::default())
        }

        #[test]
        fn conforming_snapshot_composes_and_stamps_transitional() {
            let bytes = <DocxSnapshot as store::ArtifactPack>::encode_pack(&transitional_snapshot());
            let sources = vec![ComposeSource { dialect: DIALECT_ANY, payload: AnalyzeSource::Binary(&bytes) }];
            let composed = DocxTransitionalComposerComposition::compose(&sources).expect("clean transitional document must compose");
            assert!(composed.diagnostics.iter().all(|d| d.severity != Severity::Error), "got {:?}", composed.diagnostics);
        }

        #[test]
        fn strict_namespace_present_fails_compose_with_real_diagnostic() {
            let mut opc = OpcPackage::empty();
            opc.content_types.set_default("rels", RELS_CONTENT_TYPE);
            opc.content_types.set_default("xml", "application/xml");
            opc.set_part("word/document.xml", "application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml", format!(r#"<w:document xmlns:w="{TRANSITIONAL_MAIN_NS}"><w:body/></w:document>"#).into_bytes());
            opc.set_part("word/styles.xml", "application/xml", b"<w:styles xmlns:w=\"http://purl.oclc.org/ooxml/wordprocessingml/main\"/>".to_vec());
            opc.add_relationship("", "rId1", REL_TYPE_OFFICE_DOCUMENT, "word/document.xml");
            let snapshot = DocxSnapshot::from_parts(opc, Default::default());
            let bytes = <DocxSnapshot as store::ArtifactPack>::encode_pack(&snapshot);
            let sources = vec![ComposeSource { dialect: DIALECT_ANY, payload: AnalyzeSource::Binary(&bytes) }];
            let err = DocxTransitionalComposerComposition::compose(&sources).expect_err("mixed-in strict namespace must not stamp transitional");
            assert!(err.diagnostics.iter().any(|d| d.code.0 == CODE_STRICT_NS_PRESENT && d.severity == Severity::Error), "got {:?}", err.diagnostics);
        }

        #[test]
        fn subset_validator_rechecks_wire_payload() {
            let bytes = <DocxSnapshot as store::ArtifactPack>::encode_pack(&transitional_snapshot());
            let diagnostics = DocxTransitionalValidator::validate(&IoPayload::Binary(bytes));
            assert!(diagnostics.iter().all(|d| d.severity != Severity::Error), "got {diagnostics:?}");
        }
    }
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition
