//! 🏗️ SemioBuilder — 🚧 scaffolded by W1b: local `ArtifactBuilder` round-tripping the
//! minimal snapshot. W2 adds typed constructors + the real mutation vocabulary.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::semio::standards::v1::subsets::any::schema::diff::SemioDiff;
use crate::artifacts::semio::standards::v1::subsets::any::schema::mutations::{SemioMutation, apply_semio_mutation};
use crate::artifacts::semio::standards::v1::subsets::any::schema::snapshot::SemioSnapshot;

#[derive(Clone, Debug, Default)]
pub struct SemioBuilder { snapshot: SemioSnapshot }

impl ArtifactBuilder for SemioBuilder {
    type Snapshot = SemioSnapshot;
    type Mutation = SemioMutation;
    type Diff = SemioDiff;
    fn empty() -> Self { Self { snapshot: SemioSnapshot::default() } }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot } }
    fn from_text(text: &str) -> Result<Self, store::TextError> {
        Ok(Self::from_snapshot(<SemioSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
    }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
        Ok(Self::from_snapshot(<SemioSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
    }
    fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
        let diff = apply_semio_mutation(&mut self.snapshot, &mutation);
        (self, diff)
    }
    fn absorb(mut self, diff: Self::Diff) -> Self {
        self.snapshot = <SemioDiff as protocol::MutationDiff<SemioSnapshot>>::apply(&diff, &self.snapshot);
        self
    }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { Ok(self.snapshot) }
}
