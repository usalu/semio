//! 🏗️ AviBuilder — 🚧 scaffolded by W1b: local `ArtifactBuilder` round-tripping the
//! minimal snapshot. W2 adds typed constructors + the real mutation vocabulary.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::avi::standards::v1_0::subsets::any::schema::diff::AviDiff;
use crate::artifacts::avi::standards::v1_0::subsets::any::schema::mutations::{AviMutation, apply_avi_mutation};
use crate::artifacts::avi::standards::v1_0::subsets::any::schema::snapshot::AviSnapshot;

#[derive(Clone, Debug, Default)]
pub struct AviBuilder { snapshot: AviSnapshot }

impl ArtifactBuilder for AviBuilder {
    type Snapshot = AviSnapshot;
    type Mutation = AviMutation;
    type Diff = AviDiff;
    fn empty() -> Self { Self { snapshot: AviSnapshot::default() } }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot } }
    fn from_text(text: &str) -> Result<Self, store::TextError> {
        Ok(Self::from_snapshot(<AviSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
    }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
        Ok(Self::from_snapshot(<AviSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
    }
    fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
        let diff = apply_avi_mutation(&mut self.snapshot, &mutation);
        (self, diff)
    }
    fn absorb(mut self, diff: Self::Diff) -> Self {
        self.snapshot = <AviDiff as protocol::MutationDiff<AviSnapshot>>::apply(&diff, &self.snapshot);
        self
    }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { Ok(self.snapshot) }
}
