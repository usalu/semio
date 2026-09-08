mod tests {
    use super::*;
    use crate::standards::v1_7::subsets::x::schema::PdfXBuilderConstruction as PdfXBuilder;
    use semio_framework_plugin::AnalyzeSource;
    use semio_framework_plugin::ArtifactBuilder as _;

    /// 🩹 A genuinely PDF/X-4-conforming raw fixture: the 1.7 writer (`encode_pdf`) deliberately
    /// does NOT re-emit `PdfSnapshot.objects` (see its own doc comment — asserted structurally,
    /// not byte-for-byte), so `PdfXBuilder::new(...).build()` -> `encode_pack` -> `decode_pack`
    /// can never round-trip the OutputIntent/TrimBox it seeds (same documented gap
    /// `subset_validator_recheck_runs_the_same_check` below already accounts for). Hand-crafting
    /// bytes and routing through `AnalyzeSource::Text` (which `decode_pdf`s the FULL real object
    /// graph, unlike `encode_pdf`) is the same pattern `🗄️a`'s own composer tests already use for
    /// `minimal_pdf_with_extra_object` — this is that same pattern, positive-path.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn hex_encode(bytes: &[u8]) -> String {
        bytes.iter().map(|b| format!("{b:02x}")).collect()
    }

    #[semio_framework_async_macros::async_test]
    async fn conforming_builder_snapshot_composes_and_stamps_x() {
        let bytes = minimal_conforming_x_pdf();
        let hex = hex_encode(&bytes);
        let sources = vec![ComposeSource { dialect: DIALECT_ANY, payload: AnalyzeSource::Text(&hex) }];
        let composed = PdfXComposerComposition::compose(&sources).expect("clean document must compose to x");
        assert!(composed.diagnostics.iter().all(|d| d.severity != Severity::Error), "no hard diagnostics expected: {:?}", composed.diagnostics);
    }

    #[semio_framework_async_macros::async_test]
    async fn missing_output_intent_fails_compose() {
        let snapshot = PdfSnapshot::default();
        let bytes = <PdfSnapshot as store::ArtifactPack>::encode_pack(&snapshot);
        let sources = vec![ComposeSource { dialect: DIALECT_ANY, payload: AnalyzeSource::Binary(&bytes) }];
        let err = PdfXComposerComposition::compose(&sources).expect_err("a document with no OutputIntent must not stamp x");
        assert!(err.diagnostics.iter().any(|d| d.code.0 == crate::standards::v1_7::subsets::x::schema::CODE_OUTPUT_INTENT), "got {:?}", err.diagnostics);
    }

    #[semio_framework_async_macros::async_test]
    async fn subset_validator_recheck_runs_the_same_check() {
        let snapshot = PdfXBuilder::new("sRGB IEC61966-2.1").add_page(crate::standards::v1_7::subsets::base::schema::snapshot::PdfPage::new(50.0, 50.0)).build().unwrap();
        let bytes = <PdfSnapshot as store::ArtifactPack>::encode_pack(&snapshot);
        let diagnostics = PdfXValidator::validate(&IoPayload::Binary(bytes)).await;
        // The 1.7 writer doesn't re-serialize `objects`, so the wire recheck honestly re-reports
        // the OutputIntent/TrimBox as missing (same documented gap as 🗄️a's own validator test).
        assert!(diagnostics.iter().any(|d| d.code.0 == crate::standards::v1_7::subsets::x::schema::CODE_OUTPUT_INTENT), "got {diagnostics:?}");
    }
}
