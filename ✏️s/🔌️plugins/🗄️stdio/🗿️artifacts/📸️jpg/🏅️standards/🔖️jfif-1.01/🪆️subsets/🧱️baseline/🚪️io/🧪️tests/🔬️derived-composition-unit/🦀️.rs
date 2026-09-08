mod tests {
    use super::*;
    use semio_framework_plugin::AnalyzeSource;

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn gradient_image(w: u32, h: u32) -> Vec<u8> {
        let mut out = vec![0u8; (w * h * 4) as usize];
        for y in 0..h {
            for x in 0..w {
                let idx = ((y * w + x) * 4) as usize;
                out[idx] = ((x * 255) / w.max(1)) as u8;
                out[idx + 1] = ((y * 255) / h.max(1)) as u8;
                out[idx + 2] = 128;
                out[idx + 3] = 255;
            }
        }
        out
    }

    #[semio_framework_async_macros::async_test]
    async fn engine_encoded_jpeg_composes_and_stamps_baseline() {
        let (w, h) = (32u32, 32u32);
        let snap = JpgSnapshot { width: w, height: h, pixels: gradient_image(w, h), ..JpgSnapshot::default() };
        // 🩹 `AnalyzeSource::Binary` for DIALECT_ANY expects an ALREADY pack-encoded (semio
        // envelope-wrapped) snapshot -- `store::ArtifactPack::encode_pack`, not raw
        // `engine::encode_jpg` bytes (which lack the envelope header the decode step expects).
        let bytes = <JpgSnapshot as store::ArtifactPack>::encode_pack(&snap);
        let sources = vec![ComposeSource { dialect: DIALECT_ANY, payload: AnalyzeSource::Binary(&bytes) }];
        // 🌱 Route through the 🧾️document composer first to get a real, engine-decoded snapshot (with
        // frame/sof_marker/huffman-table-count populated) the way `JpgBaselineComposerComposition::compose`
        // itself would internally.
        let composed = JpgBaselineComposerComposition::compose(&sources).expect("real baseline JPEG must compose and stamp baseline");
        assert!(composed.diagnostics.iter().all(|d| d.severity != Severity::Error), "no hard diagnostics expected: {:?}", composed.diagnostics);
        assert!(composed.snapshot.frame.is_some());
        assert_eq!(composed.snapshot.sof_marker, crate::standards::v_jfif_1_01::subsets::baseline::schema::SOF0);
    }

    #[semio_framework_async_macros::async_test]
    async fn subset_validator_recheck_flags_no_hard_diagnostics_for_a_real_encode() {
        let (w, h) = (16u32, 16u32);
        let snap = JpgSnapshot { width: w, height: h, pixels: gradient_image(w, h), ..JpgSnapshot::default() };
        let bytes = crate::standards::v_jfif_1_01::engine::encode_jpg(&snap).expect("encode");
        let decoded = crate::standards::v_jfif_1_01::engine::decode_jpg(&bytes).expect("decode");
        let packed = <JpgSnapshot as store::ArtifactPack>::encode_pack(&decoded);
        let diagnostics = JpgBaselineValidator::validate(&IoPayload::Binary(packed)).await;
        assert!(diagnostics.iter().all(|d| d.severity != Severity::Error), "wire recheck must never report a hard violation for a real baseline encode: {diagnostics:?}");
    }
}
