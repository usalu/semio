//! 🏗️ Mp4Builder — 🚧 scaffolded by W1b: local `ArtifactBuilder` round-tripping the
//! minimal snapshot. W2 adds typed constructors + the real mutation vocabulary.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::mp4::standards::isobmff::subsets::any::schema::diff::Mp4Diff;
use crate::artifacts::mp4::standards::isobmff::subsets::any::schema::mutations::{Mp4Mutation, apply_mp4_mutation};
use crate::artifacts::mp4::standards::isobmff::subsets::any::schema::snapshot::Mp4Snapshot;

#[derive(Clone, Debug, Default)]
pub struct Mp4Builder { snapshot: Mp4Snapshot }

impl ArtifactBuilder for Mp4Builder {
    type Snapshot = Mp4Snapshot;
    type Mutation = Mp4Mutation;
    type Diff = Mp4Diff;
    fn empty() -> Self { Self { snapshot: Mp4Snapshot::default() } }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot } }
    fn from_text(text: &str) -> Result<Self, store::TextError> {
        Ok(Self::from_snapshot(<Mp4Snapshot as store::ArtifactDsl>::parse_dsl(text)?))
    }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
        Ok(Self::from_snapshot(<Mp4Snapshot as store::ArtifactPack>::decode_pack(bytes)?))
    }
    fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
        let diff = apply_mp4_mutation(&mut self.snapshot, &mutation);
        (self, diff)
    }
    fn absorb(mut self, diff: Self::Diff) -> Self {
        self.snapshot = <Mp4Diff as protocol::MutationDiff<Mp4Snapshot>>::apply(&diff, &self.snapshot);
        self
    }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { Ok(self.snapshot) }
}
