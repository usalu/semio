mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn typed_constructors_build_a_populated_snapshot() {
        let snapshot = SemioImageBuilderConstruction::new(2, 2)
            .set_colorspace(SemioColorspace::Rgba)
            .set_bit_depth(8)
            .add_frame(SemioImageFrame { delay_ms: 0, rgba8: vec![255; 16] })
            .set_icc(Some(vec![1, 2, 3]))
            .add_metadata("Title", "test")
            .build()
            .expect("build");
        assert_eq!(snapshot.width, 2);
        assert_eq!(snapshot.height, 2);
        assert_eq!(snapshot.colorspace, SemioColorspace::Rgba);
        assert_eq!(snapshot.frames.len(), 1);
        assert_eq!(snapshot.icc, Some(vec![1, 2, 3]));
        assert_eq!(snapshot.metadata.len(), 1);
    }
}
