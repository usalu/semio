mod tests {
    use super::*;
    use crate::standards::v1::subsets::presentation::schema::snapshot::{Slide, SlideLayout, SlideMaster};

    #[semio_framework_async_macros::async_test]
    async fn empty_from_snapshot_and_build_round_trip() {
        let builder = SemioPresentationBuilderConstruction::empty();
        assert_eq!(builder.clone().build().unwrap(), SemioPresentationSnapshot::default());

        let populated = SemioPresentationSnapshot { masters: vec![SlideMaster { id: "m1".into(), shapes: Vec::new() }], ..Default::default() };
        let builder2 = SemioPresentationBuilderConstruction::from_snapshot(populated.clone());
        assert_eq!(builder2.build().unwrap(), populated);
    }

    #[semio_framework_async_macros::async_test]
    async fn from_text_and_from_binary_round_trip_through_a_mutated_snapshot() {
        let mut snap = SemioPresentationSnapshot::default();
        snap.masters.push(SlideMaster { id: "m1".into(), shapes: Vec::new() });
        snap.layouts.push(SlideLayout { id: "l1".into(), master_id: "m1".into(), shapes: Vec::new() });
        snap.slides.push(Slide { id: "s1".into(), layout_id: Some("l1".into()), shapes: Vec::new(), notes: Vec::new() });

        let text = <SemioPresentationSnapshot as store::ArtifactDsl>::print_dsl(&snap);
        let from_text = SemioPresentationBuilderConstruction::from_text(&text).unwrap().build().unwrap();
        assert_eq!(from_text, snap);

        let bytes = <SemioPresentationSnapshot as store::ArtifactPack>::encode_pack(&snap);
        let from_binary = SemioPresentationBuilderConstruction::from_binary(&bytes).unwrap().build().unwrap();
        assert_eq!(from_binary, snap);
    }

    #[semio_framework_async_macros::async_test]
    async fn mutate_then_absorb_matches_direct_apply() {
        let builder = SemioPresentationBuilderConstruction::empty();
        let mutation = SemioPresentationMutation::InsertMaster(insert_master::InsertMaster { master: SlideMaster { id: "m1".into(), shapes: Vec::new() } });
        let (builder, diff) = builder.mutate(mutation);
        let mutated_snapshot = builder.clone().build().unwrap();
        assert_eq!(mutated_snapshot.masters.len(), 1);

        let reabsorbed = SemioPresentationBuilderConstruction::empty().absorb(diff.diff().clone()).expect("absorb must succeed for a well-formed fixture");
        assert_eq!(reabsorbed.build().unwrap(), mutated_snapshot);
    }
}
