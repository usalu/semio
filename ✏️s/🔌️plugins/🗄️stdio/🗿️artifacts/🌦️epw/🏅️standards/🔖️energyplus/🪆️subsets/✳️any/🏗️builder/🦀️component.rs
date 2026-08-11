//! 🏗️ EpwBuilder — 🚧 scaffolded by W1b: local `ArtifactBuilder` round-tripping the
//! minimal snapshot. W2 adds typed constructors + the real mutation vocabulary.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::epw::standards::energyplus::subsets::any::schema::diff::EpwDiff;
use crate::artifacts::epw::standards::energyplus::subsets::any::schema::mutations::{EpwMutation, apply_epw_mutation};
use crate::artifacts::epw::standards::energyplus::subsets::any::schema::snapshot::EpwSnapshot;

#[derive(Clone, Debug, Default)]
pub struct EpwBuilder { snapshot: EpwSnapshot }

impl ArtifactBuilder for EpwBuilder {
    type Snapshot = EpwSnapshot;
    type Mutation = EpwMutation;
    type Diff = EpwDiff;
    fn empty() -> Self { Self { snapshot: EpwSnapshot::default() } }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot } }
    fn from_text(text: &str) -> Result<Self, store::TextError> {
        Ok(Self::from_snapshot(<EpwSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
    }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
        Ok(Self::from_snapshot(<EpwSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
    }
    fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
        let diff = apply_epw_mutation(&mut self.snapshot, &mutation);
        (self, diff)
    }
    fn absorb(mut self, diff: Self::Diff) -> Self {
        self.snapshot = <EpwDiff as protocol::MutationDiff<EpwSnapshot>>::apply(&diff, &self.snapshot);
        self
    }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { Ok(self.snapshot) }
}
