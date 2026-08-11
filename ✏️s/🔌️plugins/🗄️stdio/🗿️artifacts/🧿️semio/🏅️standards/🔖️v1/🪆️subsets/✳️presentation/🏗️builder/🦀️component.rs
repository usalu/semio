//! 🏗️ SemioPresentationBuilder — 🚧 scaffolded by W1b: local `ArtifactBuilder` round-tripping the
//! minimal snapshot. W2 adds typed constructors + the real mutation vocabulary.

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
