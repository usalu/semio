//! 🏗️ SemioAnimationBuilder — 🚧 scaffolded by W1b: local `ArtifactBuilder` round-tripping the
//! minimal snapshot. W2 adds typed constructors + the real mutation vocabulary.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::semio::standards::v1::subsets::animation::schema::diff::SemioAnimationDiff;
use crate::artifacts::semio::standards::v1::subsets::animation::schema::mutations::{SemioAnimationMutation, apply_semio_animation_mutation};
use crate::artifacts::semio::standards::v1::subsets::animation::schema::snapshot::SemioAnimationSnapshot;

#[derive(Clone, Debug, Default)]
pub struct SemioAnimationBuilder { snapshot: SemioAnimationSnapshot }

impl ArtifactBuilder for SemioAnimationBuilder {
    type Snapshot = SemioAnimationSnapshot;
    type Mutation = SemioAnimationMutation;
    type Diff = SemioAnimationDiff;
    fn empty() -> Self { Self { snapshot: SemioAnimationSnapshot::default() } }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot } }
    fn from_text(text: &str) -> Result<Self, store::TextError> {
        Ok(Self::from_snapshot(<SemioAnimationSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
    }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
        Ok(Self::from_snapshot(<SemioAnimationSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
    }
    fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
        let diff = apply_semio_animation_mutation(&mut self.snapshot, &mutation);
        (self, diff)
    }
    fn absorb(mut self, diff: Self::Diff) -> Self {
        self.snapshot = <SemioAnimationDiff as protocol::MutationDiff<SemioAnimationSnapshot>>::apply(&diff, &self.snapshot);
        self
    }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { Ok(self.snapshot) }
}
