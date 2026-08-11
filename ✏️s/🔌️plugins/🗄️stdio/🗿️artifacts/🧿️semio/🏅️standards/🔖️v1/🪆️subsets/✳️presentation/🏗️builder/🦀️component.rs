//! 🏗️ SemioPresentationBuilder — real `ArtifactBuilder` for `s.stdio.semio.presentation`:
//! empty/from_snapshot/from_text/from_binary decode, `mutate` threading the real mutation
//! vocabulary through `apply_semio_presentation_mutation`, structural `absorb`, and `build`.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::semio::standards::v1::subsets::presentation::schema::diff::SemioPresentationDiff;
use crate::artifacts::semio::standards::v1::subsets::presentation::schema::mutations::{SemioPresentationMutation, apply_semio_presentation_mutation};
use crate::artifacts::semio::standards::v1::subsets::presentation::schema::snapshot::SemioPresentationSnapshot;

#[derive(Clone, Debug, Default)]
pub struct SemioPresentationBuilder { snapshot: SemioPresentationSnapshot }

impl ArtifactBuilder for SemioPresentationBuilder {
    type Snapshot = SemioPresentationSnapshot;
    type Mutation = SemioPresentationMutation;
    type Diff = SemioPresentationDiff;
    fn empty() -> Self { Self { snapshot: SemioPresentationSnapshot::default() } }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot } }
    fn from_text(text: &str) -> Result<Self, store::TextError> {
        Ok(Self::from_snapshot(<SemioPresentationSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
    }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
        Ok(Self::from_snapshot(<SemioPresentationSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
    }
    fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
        let diff = apply_semio_presentation_mutation(&mut self.snapshot, &mutation);
        (self, diff)
    }
    fn absorb(mut self, diff: Self::Diff) -> Self {
        self.snapshot = <SemioPresentationDiff as protocol::MutationDiff<SemioPresentationSnapshot>>::apply(&diff, &self.snapshot);
        self
    }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { Ok(self.snapshot) }
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::semio::standards::v1::subsets::presentation::schema::snapshot::{Slide, SlideLayout, SlideMaster};

    #[test]
    fn empty_from_snapshot_and_build_round_trip() {
        let builder = SemioPresentationBuilder::empty();
        assert_eq!(builder.clone().build().unwrap(), SemioPresentationSnapshot::default());

        let populated = SemioPresentationSnapshot { masters: vec![SlideMaster { id: "m1".into(), shapes: Vec::new() }], ..Default::default() };
        let builder2 = SemioPresentationBuilder::from_snapshot(populated.clone());
        assert_eq!(builder2.build().unwrap(), populated);
    }

    #[test]
    fn from_text_and_from_binary_round_trip_through_a_mutated_snapshot() {
        let mut snap = SemioPresentationSnapshot::default();
        snap.masters.push(SlideMaster { id: "m1".into(), shapes: Vec::new() });
        snap.layouts.push(SlideLayout { id: "l1".into(), master_id: "m1".into(), shapes: Vec::new() });
        snap.slides.push(Slide { id: "s1".into(), layout_id: Some("l1".into()), shapes: Vec::new(), notes: Vec::new() });

        let text = <SemioPresentationSnapshot as store::ArtifactDsl>::print_dsl(&snap);
        let from_text = SemioPresentationBuilder::from_text(&text).unwrap().build().unwrap();
        assert_eq!(from_text, snap);

        let bytes = <SemioPresentationSnapshot as store::ArtifactPack>::encode_pack(&snap);
        let from_binary = SemioPresentationBuilder::from_binary(&bytes).unwrap().build().unwrap();
        assert_eq!(from_binary, snap);
    }

    #[test]
    fn mutate_then_absorb_matches_direct_apply() {
        let builder = SemioPresentationBuilder::empty();
        let mutation = SemioPresentationMutation::InsertMaster { master: SlideMaster { id: "m1".into(), shapes: Vec::new() } };
        let (builder, diff) = builder.mutate(mutation);
        let mutated_snapshot = builder.clone().build().unwrap();
        assert_eq!(mutated_snapshot.masters.len(), 1);

        let reabsorbed = SemioPresentationBuilder::empty().absorb(diff);
        assert_eq!(reabsorbed.build().unwrap(), mutated_snapshot);
    }
}
//#endregion 🧪️Tests
