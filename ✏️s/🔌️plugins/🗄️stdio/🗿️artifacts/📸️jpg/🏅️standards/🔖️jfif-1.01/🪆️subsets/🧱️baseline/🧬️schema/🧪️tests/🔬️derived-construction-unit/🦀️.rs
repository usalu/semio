mod tests {
    use super::*;

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
    async fn real_encoded_jpeg_builds_clean_via_from_binary() {
        let (w, h) = (24u32, 24u32);
        let snap = JpgSnapshot { width: w, height: h, pixels: gradient_image(w, h), ..JpgSnapshot::default() };
        let bytes = crate::standards::v_jfif_1_01::engine::encode_jpg(&snap).expect("encode");
        let decoded = crate::standards::v_jfif_1_01::engine::decode_jpg(&bytes).expect("decode");
        let packed = <JpgSnapshot as store::ArtifactPack>::encode_pack(&decoded);
        let built = JpgBaselineBuilderConstruction::from_binary(&packed).expect("from_binary").build().expect("real baseline JPEG must build clean");
        assert!(built.frame.is_some());
    }

    #[semio_framework_async_macros::async_test]
    async fn empty_snapshot_fails_build_with_no_frame() {
        let err = JpgBaselineBuilderConstruction::empty().build().expect_err("an empty snapshot has no SOF0 frame -- must fail build()");
        assert!(err.iter().any(|d| d.code.0 == crate::standards::v_jfif_1_01::subsets::baseline::schema::CODE_NO_FRAME));
    }
}
