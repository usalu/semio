mod tests {
    use super::*;
    use semio_framework_plugin::AnalyzeSource;

    /// 🩹 The 1.7 writer (`encode_pdf`) deliberately does NOT re-emit `PdfSnapshot.objects` (see
    /// its own doc comment — asserted structurally, not byte-for-byte), so a builder-seeded
    /// MarkInfo/StructTreeRoot can never round-trip through `encode_pack`/`decode_pack`. Hand-craft
    /// bytes and route through `AnalyzeSource::Text` instead (`decode_pdf` parses the FULL real
    /// object graph) — same pattern `🗄️a`'s and `🖨️x`'s own composer tests already use.
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
        assert!(err.diagnostics.iter().any(|d| d.code.0 == crate::standards::v1_7::subsets::ua::schema::CODE_MARKINFO), "got {:?}", err.diagnostics);
    }
}
