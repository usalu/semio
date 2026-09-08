mod tests {
    use super::*;
    use crate::standards::v1_7::subsets::a::schema::PdfABuilderConstruction as PdfABuilder;
    use crate::standards::v1_7::subsets::a::schema::{CODE_JAVASCRIPT, CODE_LAUNCH};
    use semio_framework_plugin::AnalyzeSource;
    use semio_framework_plugin::ArtifactBuilder as _;

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn hex_encode(bytes: &[u8]) -> String {
        bytes.iter().map(|b| format!("{b:02x}")).collect()
    }

    #[semio_framework_async_macros::async_test]
    async fn conforming_builder_snapshot_composes_and_stamps_a() {
        let snapshot = PdfABuilder::new("sRGB IEC61966-2.1").add_page(crate::standards::v1_7::subsets::base::schema::snapshot::PdfPage::new(100.0, 100.0)).build().unwrap();
        let bytes = <PdfSnapshot as store::ArtifactPack>::encode_pack(&snapshot);
        let sources = vec![ComposeSource { dialect: DIALECT_ANY, payload: AnalyzeSource::Binary(&bytes) }];
        let composed = PdfAComposerComposition::compose(&sources).expect("clean document must compose to a");
        assert!(composed.diagnostics.iter().all(|d| d.severity != Severity::Error), "no hard diagnostics expected: {:?}", composed.diagnostics);
    }

    #[semio_framework_async_macros::async_test]
    async fn javascript_action_reachable_from_open_action_fails_compose_with_real_diagnostic() {
        let bytes = minimal_pdf_with_extra_object(b"<< /S /JavaScript /JS (app.alert(1)) >>");
        let hex = hex_encode(&bytes);
        let sources = vec![ComposeSource { dialect: DIALECT_ANY, payload: AnalyzeSource::Text(&hex) }];
        let err = PdfAComposerComposition::compose(&sources).expect_err("a document with a JS action must not stamp a");
        assert!(err.diagnostics.iter().any(|d| d.code.0 == CODE_JAVASCRIPT && d.severity == Severity::Error), "got {:?}", err.diagnostics);
    }

    #[semio_framework_async_macros::async_test]
    async fn launch_action_reachable_from_open_action_fails_compose_with_real_diagnostic() {
        let bytes = minimal_pdf_with_extra_object(b"<< /S /Launch /F (calc.exe) >>");
        let hex = hex_encode(&bytes);
        let sources = vec![ComposeSource { dialect: DIALECT_ANY, payload: AnalyzeSource::Text(&hex) }];
        let err = PdfAComposerComposition::compose(&sources).expect_err("a document with a Launch action must not stamp a");
        assert!(err.diagnostics.iter().any(|d| d.code.0 == CODE_LAUNCH && d.severity == Severity::Error), "got {:?}", err.diagnostics);
    }

    #[semio_framework_async_macros::async_test]
    async fn encrypted_trailer_document_is_rejected_upstream_by_the_shared_engine() {
        // 🔒 `⚙️engine::decode_pdf` already refuses any file whose trailer declares /Encrypt
        // (`PdfEngineError::Unsupported`) -- composing through the 🧱️base delegate surfaces that as
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
        assert!(err.diagnostics.iter().any(|d| d.message.contains("Encrypt")), "must be the real engine-level /Encrypt rejection, not a spurious decode error: {err:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn subset_validator_recheck_flags_soft_diagnostics_on_the_wire_payload() {
        let snapshot = PdfABuilder::new("sRGB IEC61966-2.1").add_page(crate::standards::v1_7::subsets::base::schema::snapshot::PdfPage::new(50.0, 50.0)).build().unwrap();
        let bytes = <PdfSnapshot as store::ArtifactPack>::encode_pack(&snapshot);
        // The registered validator, called directly (same fn the generic io hook calls): today's
        // writer drops `objects` on encode, so the OutputIntent genuinely isn't in these bytes --
        // a real, honest soft diagnostic, not a false positive (see module doc comment).
        let diagnostics = PdfAValidator::validate(&IoPayload::Binary(bytes)).await;
        assert!(diagnostics.iter().all(|d| d.severity != Severity::Error), "wire recheck must never report a hard violation for a builder-clean document: {diagnostics:?}");
    }
}
