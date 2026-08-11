//! 🏗️ SemioVideoBuilder — 🚧 scaffolded by W1b: local `ArtifactBuilder` round-tripping the
//! minimal snapshot. W2 adds typed constructors + the real mutation vocabulary.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::semio::standards::v1::subsets::video::schema::diff::SemioVideoDiff;
use crate::artifacts::semio::standards::v1::subsets::video::schema::mutations::{SemioVideoMutation, apply_semio_video_mutation};
use crate::artifacts::semio::standards::v1::subsets::video::schema::snapshot::SemioVideoSnapshot;

#[derive(Clone, Debug, Default)]
pub struct SemioVideoBuilder { snapshot: SemioVideoSnapshot }

impl ArtifactBuilder for SemioVideoBuilder {
    type Snapshot = SemioVideoSnapshot;
    type Mutation = SemioVideoMutation;
    type Diff = SemioVideoDiff;
    fn empty() -> Self { Self { snapshot: SemioVideoSnapshot::default() } }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot } }
    fn from_text(text: &str) -> Result<Self, store::TextError> {
        Ok(Self::from_snapshot(<SemioVideoSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
    }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
        Ok(Self::from_snapshot(<SemioVideoSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
    }
    fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
        let diff = apply_semio_video_mutation(&mut self.snapshot, &mutation);
        (self, diff)
    }
    fn absorb(mut self, diff: Self::Diff) -> Self {
        self.snapshot = <SemioVideoDiff as protocol::MutationDiff<SemioVideoSnapshot>>::apply(&diff, &self.snapshot);
        self
    }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { Ok(self.snapshot) }
}
