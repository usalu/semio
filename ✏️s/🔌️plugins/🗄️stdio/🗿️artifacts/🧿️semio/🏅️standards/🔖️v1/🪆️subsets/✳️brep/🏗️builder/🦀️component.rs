//! 🏗️ SemioBrepBuilder — 🚧 scaffolded by W1b: local `ArtifactBuilder` round-tripping the
//! minimal snapshot. W2 adds typed constructors + the real mutation vocabulary.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::diff::SemioBrepDiff;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::mutations::{SemioBrepMutation, apply_semio_brep_mutation};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot;

#[derive(Clone, Debug, Default)]
pub struct SemioBrepBuilder { snapshot: SemioBrepSnapshot }

impl ArtifactBuilder for SemioBrepBuilder {
    type Snapshot = SemioBrepSnapshot;
    type Mutation = SemioBrepMutation;
    type Diff = SemioBrepDiff;
    fn empty() -> Self { Self { snapshot: SemioBrepSnapshot::default() } }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot } }
    fn from_text(text: &str) -> Result<Self, store::TextError> {
        Ok(Self::from_snapshot(<SemioBrepSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
    }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
        Ok(Self::from_snapshot(<SemioBrepSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
    }
    fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
        let diff = apply_semio_brep_mutation(&mut self.snapshot, &mutation);
        (self, diff)
    }
    fn absorb(mut self, diff: Self::Diff) -> Self {
        self.snapshot = <SemioBrepDiff as protocol::MutationDiff<SemioBrepSnapshot>>::apply(&diff, &self.snapshot);
        self
    }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { Ok(self.snapshot) }
}
