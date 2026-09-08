mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn typed_constructors_build_the_expected_snapshot() {
        let snapshot =
            SemioAudioBuilderConstruction::new(44_100, SemioAudioFormat::Pcm16).add_channel(SemioAudioChannel { samples: vec![0.0, 0.5] }).add_channel(SemioAudioChannel { samples: vec![0.0, -0.5] }).add_tag("title", "test").build().expect("build");
        assert_eq!(snapshot.sample_rate, 44_100);
        assert_eq!(snapshot.channels.len(), 2);
        assert_eq!(snapshot.tags[0].key, "title");
    }

    #[semio_framework_async_macros::async_test]
    async fn mutate_then_absorb_round_trips_through_the_builder() {
        let builder = SemioAudioBuilderConstruction::new(48_000, SemioAudioFormat::Float32);
        let (builder, diff) = builder.mutate(SemioAudioMutation::InsertChannel(insert_channel::InsertChannel { index: 0, channel: SemioAudioChannel { samples: vec![1.0, 2.0] } }));
        let snapshot_after_mutate = builder.clone().build().expect("build");
        let rebuilt = SemioAudioBuilderConstruction::empty()
            .absorb(SemioAudioDiff::default())
            .expect("absorb must succeed for a well-formed fixture")
            .mutate(SemioAudioMutation::SetSampleRate(set_sample_rate::SetSampleRate { sample_rate: 48_000 }))
            .0
            .mutate(SemioAudioMutation::SetFormat(set_format::SetFormat { format: SemioAudioFormat::Float32 }))
            .0;
        let rebuilt = rebuilt.absorb(diff.diff().clone()).expect("absorb must succeed for a well-formed fixture");
        assert_eq!(rebuilt.build().expect("build"), snapshot_after_mutate);
    }

    #[semio_framework_async_macros::async_test]
    async fn from_binary_and_from_text_round_trip_through_the_builder() {
        let snapshot = SemioAudioBuilderConstruction::new(22_050, SemioAudioFormat::Pcm24).add_channel(SemioAudioChannel { samples: vec![0.1] }).build().expect("build");
        let bytes = <SemioAudioSnapshot as store::ArtifactPack>::encode_pack(&snapshot);
        let via_binary = SemioAudioBuilderConstruction::from_binary(&bytes).expect("from_binary").build().expect("build");
        assert_eq!(via_binary, snapshot);

        let text = <SemioAudioSnapshot as store::ArtifactDsl>::print_dsl(&snapshot);
        let via_text = SemioAudioBuilderConstruction::from_text(&text).expect("from_text").build().expect("build");
        assert_eq!(via_text, snapshot);
    }
}
