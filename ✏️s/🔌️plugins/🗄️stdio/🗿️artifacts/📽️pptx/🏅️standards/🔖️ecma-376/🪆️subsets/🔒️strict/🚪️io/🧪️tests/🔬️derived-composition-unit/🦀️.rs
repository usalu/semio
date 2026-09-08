mod tests {
    use super::*;
    use semio_framework_plugin::AnalyzeSource;
    use semio_s_artifact_stdio_zip::opc::{self, OpcPackage, RELS_CONTENT_TYPE};

    const STRICT_PRESENTATION_XML: &str = concat!(
        r#"<p:presentation xmlns:a="http://purl.oclc.org/ooxml/drawingml/main" xmlns:p="http://purl.oclc.org/ooxml/presentationml/main" xmlns:r="http://purl.oclc.org/ooxml/officeDocument/relationships" conformance="strict">"#,
        r#"<p:sldMasterIdLst><p:sldMasterId id="2147483648" r:id="rId1"/></p:sldMasterIdLst>"#,
        r#"<p:sldIdLst/>"#,
        "</p:presentation>",
    );

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn hex_encode(bytes: &[u8]) -> String {
        bytes.iter().map(|b| format!("{b:02x}")).collect()
    }

    /// 🩹 Builds real OPC zip bytes directly via `opc::encode_opc` (never `PptxSnapshot`'s
    /// `ArtifactPack::encode_pack`, which round-trips through `⚙️engine::encode_pptx` --
    /// `regenerate_presentation_parts` unconditionally rewrites `ppt/presentation.xml` with
    /// hardcoded Transitional markup, which would silently clobber this hand-crafted Strict
    /// content). Routing through `AnalyzeSource::Text(hex)` (`ArtifactDsl::parse_dsl`) is how the
    /// `📄️pdf` 1.7 `✳️a` pilot's own composer tests exercise the real decode path -- same
    /// technique here.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn strict_package_hex() -> String {
        let mut opc = OpcPackage::empty();
        opc.content_types.set_default("rels", RELS_CONTENT_TYPE);
        opc.content_types.set_default("xml", "application/xml");
        opc.set_part("ppt/presentation.xml", "application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml", STRICT_PRESENTATION_XML.as_bytes().to_vec());
        opc.set_part("ppt/slideMasters/slideMaster1.xml", "application/vnd.openxmlformats-officedocument.presentationml.slideMaster+xml", b"<p:sldMaster/>".to_vec());
        opc.add_relationship("", "rId1", "http://purl.oclc.org/ooxml/officeDocument/relationships/officeDocument", "ppt/presentation.xml");
        opc.add_relationship("ppt/presentation.xml", "rId1", "http://purl.oclc.org/ooxml/officeDocument/relationships/slideMaster", "slideMasters/slideMaster1.xml");
        let bytes = opc::encode_opc(&opc).expect("encode hand-built Strict OPC package");
        hex_encode(&bytes)
    }

    #[semio_framework_async_macros::async_test]
    async fn conforming_strict_package_composes_and_stamps_strict() {
        let hex = strict_package_hex();
        let sources = vec![ComposeSource { dialect: DIALECT_ANY, payload: AnalyzeSource::Text(&hex) }];
        let composed = PptxStrictComposerComposition::compose(&sources).expect("clean Strict document must compose to strict");
        assert!(composed.diagnostics.iter().all(|d| d.severity != Severity::Error), "no hard diagnostics expected: {:?}", composed.diagnostics);
    }

    #[semio_framework_async_macros::async_test]
    async fn transitional_document_fails_compose_with_real_diagnostic() {
        let mut opc = OpcPackage::empty();
        opc.content_types.set_default("rels", RELS_CONTENT_TYPE);
        opc.content_types.set_default("xml", "application/xml");
        let transitional_xml = concat!(
            r#"<p:presentation xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">"#,
            r#"<p:sldMasterIdLst><p:sldMasterId id="2147483648" r:id="rId1"/></p:sldMasterIdLst>"#,
            r#"<p:sldIdLst/>"#,
            "</p:presentation>",
        );
        opc.set_part("ppt/presentation.xml", "application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml", transitional_xml.as_bytes().to_vec());
        opc.set_part("ppt/slideMasters/slideMaster1.xml", "application/vnd.openxmlformats-officedocument.presentationml.slideMaster+xml", b"<p:sldMaster/>".to_vec());
        opc.add_relationship("", "rId1", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument", "ppt/presentation.xml");
        opc.add_relationship("ppt/presentation.xml", "rId1", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideMaster", "slideMasters/slideMaster1.xml");
        let bytes = opc::encode_opc(&opc).expect("encode hand-built Transitional OPC package");
        let hex = hex_encode(&bytes);
        let sources = vec![ComposeSource { dialect: DIALECT_ANY, payload: AnalyzeSource::Text(&hex) }];
        let err = PptxStrictComposerComposition::compose(&sources).expect_err("a Transitional document must not stamp strict");
        assert!(err.diagnostics.iter().any(|d| d.severity == Severity::Error), "got {:?}", err.diagnostics);
    }

    #[semio_framework_async_macros::async_test]
    async fn subset_validator_recheck_flags_clean_document_as_clean() {
        let hex = strict_package_hex();
        let diagnostics = PptxStrictValidator::validate(&IoPayload::Text(hex)).await;
        assert!(diagnostics.iter().all(|d| d.severity != Severity::Error), "wire recheck must never report a hard violation for a clean Strict document: {diagnostics:?}");
    }
}
