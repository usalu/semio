//! 🏗️ TsvBuilder — 🚧 scaffolded by W1b: local `ArtifactBuilder` round-tripping the
//! minimal snapshot. W2 adds typed constructors + the real mutation vocabulary.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::tsv::standards::iana::subsets::any::schema::diff::TsvDiff;
use crate::artifacts::tsv::standards::iana::subsets::any::schema::mutations::{TsvMutation, apply_tsv_mutation};
use crate::artifacts::tsv::standards::iana::subsets::any::schema::snapshot::TsvSnapshot;

#[derive(Clone, Debug, Default)]
pub struct TsvBuilder { snapshot: TsvSnapshot }

impl ArtifactBuilder for TsvBuilder {
    type Snapshot = TsvSnapshot;
    type Mutation = TsvMutation;
    type Diff = TsvDiff;
    fn empty() -> Self { Self { snapshot: TsvSnapshot::default() } }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot } }
    fn from_text(text: &str) -> Result<Self, store::TextError> {
        Ok(Self::from_snapshot(<TsvSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
    }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
        Ok(Self::from_snapshot(<TsvSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
    }
    fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
        let diff = apply_tsv_mutation(&mut self.snapshot, &mutation);
        (self, diff)
    }
    fn absorb(mut self, diff: Self::Diff) -> Self {
        self.snapshot = <TsvDiff as protocol::MutationDiff<TsvSnapshot>>::apply(&diff, &self.snapshot);
        self
    }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { Ok(self.snapshot) }
}
