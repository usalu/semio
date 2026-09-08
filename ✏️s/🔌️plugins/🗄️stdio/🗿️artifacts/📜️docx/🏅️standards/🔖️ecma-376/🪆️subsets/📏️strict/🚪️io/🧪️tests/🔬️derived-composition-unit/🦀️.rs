mod tests {
    use super::*;
    use crate::standards::v_ecma_376::subsets::strict::schema::CODE_REL_BASE;
    use semio_framework_plugin::AnalyzeSource;
    use semio_s_artifact_stdio_zip::opc::{OpcPackage, REL_TYPE_OFFICE_DOCUMENT, RELS_CONTENT_TYPE};

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
        let raw = semio_s_artifact_stdio_zip::opc::encode_opc(&snapshot.opc).expect("valid opc package encodes");
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
