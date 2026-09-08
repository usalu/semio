mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn fluent_builder_round_trips_through_text_and_binary() {
        let built = SemioDocumentBuilderConstruction::empty()
            .with_style(DocStyle { id: "Normal".into(), name: "Normal".into(), based_on: None })
            .with_image(DocImage { id: "img1".into(), mime: "image/png".into(), bytes: vec![1, 2, 3] })
            .with_block(DocBlock::paragraph("hello"))
            .build()
            .expect("build");
        assert_eq!(built.styles.len(), 1);
        assert_eq!(built.images.len(), 1);
        assert_eq!(built.blocks.len(), 1);

        let text = <SemioDocumentSnapshot as store::ArtifactDsl>::print_dsl(&built);
        let from_text = SemioDocumentBuilderConstruction::from_text(&text).expect("from_text").build().expect("build");
        assert_eq!(from_text, built);

        let bytes = <SemioDocumentSnapshot as store::ArtifactPack>::encode_pack(&built);
        let from_binary = SemioDocumentBuilderConstruction::from_binary(&bytes).expect("from_binary").build().expect("build");
        assert_eq!(from_binary, built);
    }

    #[semio_framework_async_macros::async_test]
    async fn mutate_then_absorb_round_trips() {
        let (builder, diff) = SemioDocumentBuilderConstruction::empty().mutate(SemioDocumentMutation::InsertStyle(insert_style::InsertStyle { style: DocStyle { id: "s".into(), name: "S".into(), based_on: None } }));
        let rebuilt = SemioDocumentBuilderConstruction::empty().absorb(diff.diff().clone()).expect("absorb must succeed for a well-formed fixture");
        assert_eq!(builder.build().unwrap(), rebuilt.build().unwrap());
    }
}
