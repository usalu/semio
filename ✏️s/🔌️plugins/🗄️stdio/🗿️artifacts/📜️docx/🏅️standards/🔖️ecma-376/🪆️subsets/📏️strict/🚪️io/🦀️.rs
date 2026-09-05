//! 🚪️ IO stdio.docx (ecma-376/📏️strict) — doc-leaf only, referencing the owning ✳️any subset's
//! import/export tree (`🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🚪️io/`) rather than duplicating
//! it. Registration flows through `🎹️composer::register` (the `SubsetValidator` directly, the
//! `ComposerEntry` via the standard-level aggregator), not per-leaf `register()`.
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::artifacts::docx::standards::v_ecma_376::subsets::base::schema::DocxComposer as DocxAnyComposer;
    use crate::artifacts::docx::standards::v_ecma_376::subsets::strict::schema::check_strict_conformance;
    use crate::artifacts::docx::DocxSnapshot;
    use dsl::{Diagnostic, FaultCode, Severity, TextSpan};
    use semio_framework_plugin::{register_subset_validator, subset_validator_entry_of, ArtifactComposition, ComposeError, ComposeSource, Composition, Dialect, IoPayload, StandardId, SubsetId, SubsetValidator, SubsetValidatorEntry};
    use std::sync::OnceLock;

    const DIALECT_STRICT: Dialect = Dialect { artifact_kind: "s.stdio.docx", standard: StandardId("ecma-376"), subset: SubsetId("strict") };
    const DIALECT_ANY: Dialect = Dialect { artifact_kind: "s.stdio.docx", standard: StandardId("ecma-376"), subset: SubsetId("*") };
    const DEP_ZIP: Dialect = Dialect { artifact_kind: "s.stdio.zip", standard: StandardId("2.0"), subset: SubsetId("*") };
    const DEP_XML: Dialect = Dialect { artifact_kind: "s.stdio.xml", standard: StandardId("1.0"), subset: SubsetId("*") };

    //#region 🔖️Composer
    pub struct DocxStrictComposerComposition;

    impl ArtifactComposition for DocxStrictComposerComposition {
        type Snapshot = DocxSnapshot;
        const WRITES: Dialect = DIALECT_STRICT;

        fn reads() -> &'static [Dialect] {
            &[DIALECT_ANY, DIALECT_STRICT, DEP_ZIP, DEP_XML]
        }

        fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            let inner = DocxAnyComposer::compose(sources)?;
            let checks = check_strict_conformance(&inner.snapshot);
            let (hard, soft): (Vec<Diagnostic>, Vec<Diagnostic>) = checks.into_iter().partition(|d| matches!(d.severity, Severity::Error | Severity::Fatal));
            if !hard.is_empty() {
                let mut all = hard.clone();
                all.extend(soft);
                return Err(ComposeError { message: format!("ISO/IEC 29500-1 Strict conformance violated: {} hard issue(s) -- not stamping the strict dialect", hard.len()), diagnostics: all });
            }
            let mut diagnostics = inner.diagnostics;
            diagnostics.extend(soft);
            Ok(Composition { snapshot: inner.snapshot, confidence: inner.confidence, diagnostics })
        }
    }
    //#endregion 🔖️Composer

    //#region 🔖️SubsetValidator
    /// 🛡️ The registered `SubsetValidator` for `ecma-376/strict` -- see the module doc comment for
    /// how this relates to the composer's own pre-serialization hard gate above.
    pub struct DocxStrictValidator;

    impl SubsetValidator for DocxStrictValidator {
        const DIALECT: Dialect = DIALECT_STRICT;

        async fn validate(payload: &IoPayload) -> Vec<Diagnostic> {
            let decoded = match payload {
                IoPayload::Binary(bytes) => <DocxSnapshot as store::ArtifactPack>::decode_pack(bytes).ok(),
                IoPayload::Text(text) => <DocxSnapshot as store::ArtifactDsl>::parse_dsl(text).ok(),
            };
            match decoded {
                Some(snapshot) => check_strict_conformance(&snapshot),
                None => vec![Diagnostic {
                    code: FaultCode::new("stdio.docx.strict.validate-decode-failed"),
                    severity: Severity::Warning,
                    span: TextSpan::at(1, 1),
                    message: "docx strict SubsetValidator: payload did not decode as a DocxSnapshot -- skipped".into(),
                    expected: None,
                    scope: dsl::FaultScope::default(),
                }],
            }
        }
    }

    static VALIDATOR_ENTRY: OnceLock<SubsetValidatorEntry> = OnceLock::new();

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn validator_entry() -> &'static SubsetValidatorEntry {
        VALIDATOR_ENTRY.get_or_init(subset_validator_entry_of::<DocxStrictValidator>)
    }

    /// 📌️ Registers this subset's `SubsetValidator` with the generic io registry (D5's
    /// validate-on-build hook). Called from the ecma-376 standard's own `⚙️engine::register()`. The
    /// `ComposerEntry` itself is aggregated separately by the standard-level composer
    /// (`crate::artifacts::docx::standards::v_ecma_376::engine::io_registry::entries()`).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register() {
        let _ = register_subset_validator(validator_entry());
    }
    //#endregion 🔖️SubsetValidator

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::artifacts::docx::standards::v_ecma_376::subsets::strict::schema::CODE_REL_BASE;
        use crate::artifacts::zip::opc::{OpcPackage, RELS_CONTENT_TYPE, REL_TYPE_OFFICE_DOCUMENT};
        use semio_framework_plugin::AnalyzeSource;

        const STRICT_MAIN_NS: &str = "http://purl.oclc.org/ooxml/wordprocessingml/main";
        const STRICT_REL_BASE: &str = "http://purl.oclc.org/ooxml/officeDocument/relationships";

        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        fn strict_snapshot() -> DocxSnapshot {
            let mut opc = OpcPackage::empty();
            opc.content_types.set_default("rels", RELS_CONTENT_TYPE);
            opc.content_types.set_default("xml", "application/xml");
            opc.set_part("word/document.xml", "application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml", format!(r#"<w:document xmlns:w="{STRICT_MAIN_NS}" conformance="strict"><w:body/></w:document>"#).into_bytes());
            opc.add_relationship("", "rId1", &format!("{STRICT_REL_BASE}/officeDocument"), "word/document.xml");
            DocxSnapshot::from_parts(opc, Default::default())
        }

        /// 🩹 `encode_docx` (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/⚙️engine/🦀️.rs`)
        /// deliberately OVERWRITES `word/document.xml` from `snapshot.document` (the typed
        /// paragraphs/runs model) on every encode -- see `DocxSnapshot.opc`'s own doc comment ("kept
        /// in sync with `document` on encode"). Since `strict_snapshot()` sets `document` to
        /// `Default::default()`, going through `encode_pack` would silently discard the hand-set
        /// `xmlns:w`/`conformance="strict"` XML and replace it with the default empty document's
        /// regenerated (non-strict) XML. Encoding the OPC package directly (bypassing the docx typed
        /// model entirely, matching what `encode_pack_with` does minus that one overwrite step) is how
        /// this test genuinely exercises a document whose main-part XML matches what was set on `opc`.
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        fn conforming_pack_bytes(snapshot: &DocxSnapshot) -> Vec<u8> {
            let raw = crate::artifacts::zip::opc::encode_opc(&snapshot.opc).expect("valid opc package encodes");
            let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<DocxSnapshot as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).expect("valid envelope_id");
            store::semio_format::wrap_binary(&envelope, &raw)
        }

        #[semio_framework_async_macros::async_test]
        async fn conforming_snapshot_composes_and_stamps_strict() {
            let bytes = conforming_pack_bytes(&strict_snapshot());
            let sources = vec![ComposeSource { dialect: DIALECT_ANY, payload: AnalyzeSource::Binary(&bytes) }];
            let composed = DocxStrictComposerComposition::compose(&sources).expect("clean strict document must compose");
            assert!(composed.diagnostics.iter().all(|d| d.severity != Severity::Error), "got {:?}", composed.diagnostics);
        }

        #[semio_framework_async_macros::async_test]
        async fn transitional_relationship_base_fails_compose_with_real_diagnostic() {
            let mut opc = OpcPackage::empty();
            opc.content_types.set_default("rels", RELS_CONTENT_TYPE);
            opc.content_types.set_default("xml", "application/xml");
            opc.set_part("word/document.xml", "application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml", format!(r#"<w:document xmlns:w="{STRICT_MAIN_NS}"><w:body/></w:document>"#).into_bytes());
            opc.add_relationship("", "rId1", REL_TYPE_OFFICE_DOCUMENT, "word/document.xml");
            let snapshot = DocxSnapshot::from_parts(opc, Default::default());
            let bytes = <DocxSnapshot as store::ArtifactPack>::encode_pack(&snapshot);
            let sources = vec![ComposeSource { dialect: DIALECT_ANY, payload: AnalyzeSource::Binary(&bytes) }];
            let err = DocxStrictComposerComposition::compose(&sources).expect_err("transitional relationship base must not stamp strict");
            assert!(err.diagnostics.iter().any(|d| d.code.0 == CODE_REL_BASE && d.severity == Severity::Error), "got {:?}", err.diagnostics);
        }

        #[semio_framework_async_macros::async_test]
        async fn subset_validator_rechecks_wire_payload() {
            let bytes = conforming_pack_bytes(&strict_snapshot());
            let diagnostics = DocxStrictValidator::validate(&IoPayload::Binary(bytes)).await;
            assert!(diagnostics.iter().all(|d| d.severity != Severity::Error), "got {diagnostics:?}");
        }
    }
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition
