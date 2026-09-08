mod tests {
    use super::*;
    use semio_framework_plugin::AnalyzeSource;

    /// 🩹 The 1.7 writer (`encode_pdf`) deliberately does NOT re-emit `PdfSnapshot.objects` (see
    /// its own doc comment — asserted structurally, not byte-for-byte), so a builder-seeded
    /// OutputIntent/TrimBox/DPartRoot can never round-trip through `encode_pack`/`decode_pack`.
    /// Hand-craft bytes and route through `AnalyzeSource::Text` instead (`decode_pdf` parses the
    /// FULL real object graph) — same pattern `🗄️a`'s/`🖨️x`'s/`♿️ua`'s own composer tests use.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn minimal_conforming_vt_pdf() -> Vec<u8> {
        let mut body = Vec::new();
        body.extend_from_slice(b"%PDF-1.7\n");
        let o1 = body.len();
        body.extend_from_slice(b"1 0 obj\n<< /Type /Catalog /Pages 2 0 R /OutputIntents [4 0 R] /DPartRoot 6 0 R >>\nendobj\n");
        let o2 = body.len();
        body.extend_from_slice(b"2 0 obj\n<< /Type /Pages /Kids [3 0 R] /Count 1 >>\nendobj\n");
        let o3 = body.len();
        body.extend_from_slice(b"3 0 obj\n<< /Type /Page /Parent 2 0 R /MediaBox [0 0 200 200] /TrimBox [0 0 200 200] /Resources << >> >>\nendobj\n");
        let o4 = body.len();
        body.extend_from_slice(b"4 0 obj\n<< /Type /OutputIntent /S /GTS_PDFX /OutputConditionIdentifier (sRGB IEC61966-2.1) /DestOutputProfile 5 0 R >>\nendobj\n");
        let o5 = body.len();
        body.extend_from_slice(b"5 0 obj\n<< /N 3 >>\nendobj\n");
        let o6 = body.len();
        body.extend_from_slice(b"6 0 obj\n<< /Type /DPartRoot /DParts [7 0 R] >>\nendobj\n");
        let o7 = body.len();
        body.extend_from_slice(b"7 0 obj\n<< /Type /DPart /DPM 8 0 R >>\nendobj\n");
        let o8 = body.len();
        body.extend_from_slice(b"8 0 obj\n<< >>\nendobj\n");
        let xref = body.len();
        body.extend_from_slice(b"xref\n0 9\n0000000000 65535 f \n");
        for off in [o1, o2, o3, o4, o5, o6, o7, o8] {
            body.extend_from_slice(format!("{off:010} 00000 n \n").as_bytes());
        }
        body.extend_from_slice(format!("trailer\n<< /Size 9 /Root 1 0 R >>\nstartxref\n{xref}\n%%EOF\n").as_bytes());
        body
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn hex_encode(bytes: &[u8]) -> String {
        bytes.iter().map(|b| format!("{b:02x}")).collect()
    }

    #[semio_framework_async_macros::async_test]
    async fn conforming_builder_snapshot_composes_and_stamps_vt() {
        let bytes = minimal_conforming_vt_pdf();
        let hex = hex_encode(&bytes);
        let sources = vec![ComposeSource { dialect: DIALECT_ANY, payload: AnalyzeSource::Text(&hex) }];
        let composed = PdfVtComposerComposition::compose(&sources).expect("clean document must compose to vt");
        assert!(composed.diagnostics.iter().all(|d| d.severity != Severity::Error), "no hard diagnostics expected: {:?}", composed.diagnostics);
    }

    #[semio_framework_async_macros::async_test]
    async fn missing_dpartroot_fails_compose() {
        let snapshot = PdfSnapshot::default();
        let bytes = <PdfSnapshot as store::ArtifactPack>::encode_pack(&snapshot);
        let sources = vec![ComposeSource { dialect: DIALECT_ANY, payload: AnalyzeSource::Binary(&bytes) }];
        let err = PdfVtComposerComposition::compose(&sources).expect_err("a document with no DPartRoot must not stamp vt");
        assert!(err.diagnostics.iter().any(|d| d.severity == Severity::Error), "got {:?}", err.diagnostics);
    }
}
