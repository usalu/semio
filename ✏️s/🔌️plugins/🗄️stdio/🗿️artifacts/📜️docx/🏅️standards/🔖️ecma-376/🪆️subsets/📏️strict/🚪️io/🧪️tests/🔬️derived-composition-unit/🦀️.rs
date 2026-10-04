mod tests {
    use super::*;
    use crate::schema::snapshot::DocxXmlPart;
    use crate::standards::v_ecma_376::subsets::strict::schema::CODE_REL_BASE;
    use semio_framework_plugin::AnalyzeSource;
    use semio_s_artifact_stdio_zip::opc::{OpcPackage, RELS_CONTENT_TYPE, REL_TYPE_OFFICE_DOCUMENT};

    const STRICT_MAIN_NS: &str = "http://purl.oclc.org/ooxml/wordprocessingml/main";
    const STRICT_REL_BASE: &str = "http://purl.oclc.org/ooxml/officeDocument/relationships";

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn strict_snapshot() -> DocxSnapshot {
        let mut opc = OpcPackage::empty();
        opc.content_types.set_default("rels", RELS_CONTENT_TYPE);
        opc.content_types.set_default("xml", "application/xml");
        let content_type = "application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml";
        opc.content_types.set_override("word/document.xml", content_type);
        opc.add_relationship("", "rId1", &format!("{STRICT_REL_BASE}/officeDocument"), "word/document.xml");
        DocxSnapshot::from_parts(
            opc,
            vec![DocxXmlPart {
                path: "word/document.xml".into(),
                content_type: content_type.into(),
                document: semio_s_artifact_stdio_xml::schema::snapshot::retained::RetainedXmlDocument::try_from_document(&semio_s_artifact_stdio_xml::schema::snapshot::xml_document_from_text(&format!(r#"<w:document xmlns:w="{STRICT_MAIN_NS}" conformance="strict"><w:body/></w:document>"#)).unwrap()).unwrap(),
            }],
        ).expect("bounded test OPC converts to retained ownership")
    }

    /// 📦️ The artifact pack serializes the authoritative strict XML part without regeneration.
    fn conforming_pack_bytes(snapshot: &DocxSnapshot) -> Vec<u8> {
        <DocxSnapshot as store::ArtifactPack>::encode_pack(snapshot)
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
        let content_type = "application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml";
        opc.content_types.set_override("word/document.xml", content_type);
        opc.add_relationship("", "rId1", REL_TYPE_OFFICE_DOCUMENT, "word/document.xml");
        let snapshot = DocxSnapshot::from_parts(
            opc,
            vec![DocxXmlPart {
                path: "word/document.xml".into(),
                content_type: content_type.into(),
                document: semio_s_artifact_stdio_xml::schema::snapshot::retained::RetainedXmlDocument::try_from_document(&semio_s_artifact_stdio_xml::schema::snapshot::xml_document_from_text(&format!(r#"<w:document xmlns:w="{STRICT_MAIN_NS}"><w:body/></w:document>"#)).unwrap()).unwrap(),
            }],
        ).expect("bounded test OPC converts to retained ownership");
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
