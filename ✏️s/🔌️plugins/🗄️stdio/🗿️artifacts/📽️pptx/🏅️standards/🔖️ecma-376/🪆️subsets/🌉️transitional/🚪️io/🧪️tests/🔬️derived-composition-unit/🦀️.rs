mod tests {
    use super::*;
    use semio_framework_plugin::AnalyzeSource;
    use semio_s_artifact_stdio_zip::opc::{self, OpcPackage, REL_TYPE_OFFICE_DOCUMENT, RELS_CONTENT_TYPE};

    const TRANSITIONAL_PRESENTATION_XML: &str = concat!(
        r#"<p:presentation xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">"#,
        r#"<p:sldMasterIdLst><p:sldMasterId id="2147483648" r:id="rId1"/></p:sldMasterIdLst>"#,
        r#"<p:sldIdLst/>"#,
        "</p:presentation>",
    );

    /// 🧾️ The real OPC input becomes the complete canonical owned native Text snapshot.
    fn transitional_package_text() -> String {
        let mut opc = OpcPackage::empty();
        opc.content_types.set_default("rels", RELS_CONTENT_TYPE);
        opc.content_types.set_default("xml", "application/xml");
        opc.set_part("ppt/presentation.xml", "application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml", TRANSITIONAL_PRESENTATION_XML.as_bytes().to_vec());
        opc.set_part("ppt/slideMasters/slideMaster1.xml", "application/vnd.openxmlformats-officedocument.presentationml.slideMaster+xml", b"<p:sldMaster/>".to_vec());
        opc.add_relationship("", "rId1", REL_TYPE_OFFICE_DOCUMENT, "ppt/presentation.xml");
        opc.add_relationship("ppt/presentation.xml", "rId1", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideMaster", "slideMasters/slideMaster1.xml");
        let bytes = opc::encode_opc(&opc).expect("encode hand-built Transitional OPC package");
        store::ArtifactDsl::print_dsl(&crate::standards::v_ecma_376::subsets::base::io::import::deserializers::decode_pptx(&bytes).expect("decode actual profile package"))
    }

    #[semio_framework_async_macros::async_test]
    async fn conforming_transitional_package_composes_and_stamps_transitional() {
        let text = transitional_package_text();
        let sources = vec![ComposeSource { dialect: DIALECT_ANY, payload: AnalyzeSource::Text(&text) }];
        let composed = PptxTransitionalComposerComposition::compose(&sources).expect("clean Transitional document must compose to transitional");
        assert!(composed.diagnostics.iter().all(|d| d.severity != Severity::Error), "no hard diagnostics expected: {:?}", composed.diagnostics);
    }

    #[semio_framework_async_macros::async_test]
    async fn strict_document_fails_compose_with_real_diagnostic() {
        let mut opc = OpcPackage::empty();
        opc.content_types.set_default("rels", RELS_CONTENT_TYPE);
        opc.content_types.set_default("xml", "application/xml");
        let strict_xml = concat!(
            r#"<p:presentation xmlns:a="http://purl.oclc.org/ooxml/drawingml/main" xmlns:p="http://purl.oclc.org/ooxml/presentationml/main" xmlns:r="http://purl.oclc.org/ooxml/officeDocument/relationships" conformance="strict">"#,
            r#"<p:sldMasterIdLst><p:sldMasterId id="2147483648" r:id="rId1"/></p:sldMasterIdLst>"#,
            r#"<p:sldIdLst/>"#,
            "</p:presentation>",
        );
        opc.set_part("ppt/presentation.xml", "application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml", strict_xml.as_bytes().to_vec());
        opc.set_part("ppt/slideMasters/slideMaster1.xml", "application/vnd.openxmlformats-officedocument.presentationml.slideMaster+xml", b"<p:sldMaster/>".to_vec());
        opc.add_relationship("", "rId1", "http://purl.oclc.org/ooxml/officeDocument/relationships/officeDocument", "ppt/presentation.xml");
        opc.add_relationship("ppt/presentation.xml", "rId1", "http://purl.oclc.org/ooxml/officeDocument/relationships/slideMaster", "slideMasters/slideMaster1.xml");
        let bytes = opc::encode_opc(&opc).expect("encode hand-built Strict OPC package");
        let text = store::ArtifactDsl::print_dsl(&crate::standards::v_ecma_376::subsets::base::io::import::deserializers::decode_pptx(&bytes).expect("decode actual profile package"));
        let sources = vec![ComposeSource { dialect: DIALECT_ANY, payload: AnalyzeSource::Text(&text) }];
        let err = PptxTransitionalComposerComposition::compose(&sources).expect_err("a Strict document must not stamp transitional");
        assert!(err.diagnostics.iter().any(|d| d.severity == Severity::Error && d.code.0.starts_with("stdio.pptx.")), "got {:?}", err.diagnostics);
    }

    #[semio_framework_async_macros::async_test]
    async fn subset_validator_recheck_flags_clean_document_as_clean() {
        let text = transitional_package_text();
        let diagnostics = PptxTransitionalValidator::validate(&IoPayload::Text(text)).await;
        assert!(diagnostics.iter().all(|d| d.severity != Severity::Error && !d.code.0.ends_with("validate-decode-failed")), "wire recheck must never report a hard violation for a clean Transitional document: {diagnostics:?}");
    }
}
