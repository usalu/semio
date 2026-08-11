//! 🏗️ SemioCadBuilder — 🚧 scaffolded by W1b: local `ArtifactBuilder` round-tripping the
//! minimal snapshot. W2 adds typed constructors + the real mutation vocabulary.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::semio::standards::v1::subsets::cad::schema::diff::SemioCadDiff;
use crate::artifacts::semio::standards::v1::subsets::cad::schema::mutations::{SemioCadMutation, apply_semio_cad_mutation};
use crate::artifacts::semio::standards::v1::subsets::cad::schema::snapshot::SemioCadSnapshot;

#[derive(Clone, Debug, Default)]
pub struct SemioCadBuilder { snapshot: SemioCadSnapshot }

impl ArtifactBuilder for SemioCadBuilder {
    type Snapshot = SemioCadSnapshot;
    type Mutation = SemioCadMutation;
    type Diff = SemioCadDiff;
    fn empty() -> Self { Self { snapshot: SemioCadSnapshot::default() } }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot } }
    fn from_text(text: &str) -> Result<Self, store::TextError> {
        Ok(Self::from_snapshot(<SemioCadSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
    }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
        Ok(Self::from_snapshot(<SemioCadSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
    }
    fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
        let diff = apply_semio_cad_mutation(&mut self.snapshot, &mutation);
        (self, diff)
    }
    fn absorb(mut self, diff: Self::Diff) -> Self {
        self.snapshot = <SemioCadDiff as protocol::MutationDiff<SemioCadSnapshot>>::apply(&diff, &self.snapshot);
        self
    }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { Ok(self.snapshot) }
}
