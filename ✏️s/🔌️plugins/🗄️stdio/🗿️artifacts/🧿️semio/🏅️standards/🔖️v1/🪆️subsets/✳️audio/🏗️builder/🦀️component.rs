//! 🏗️ SemioAudioBuilder — 🚧 scaffolded by W1b: local `ArtifactBuilder` round-tripping the
//! minimal snapshot. W2 adds typed constructors + the real mutation vocabulary.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::semio::standards::v1::subsets::audio::schema::diff::SemioAudioDiff;
use crate::artifacts::semio::standards::v1::subsets::audio::schema::mutations::{SemioAudioMutation, apply_semio_audio_mutation};
use crate::artifacts::semio::standards::v1::subsets::audio::schema::snapshot::SemioAudioSnapshot;

#[derive(Clone, Debug, Default)]
pub struct SemioAudioBuilder { snapshot: SemioAudioSnapshot }

impl ArtifactBuilder for SemioAudioBuilder {
    type Snapshot = SemioAudioSnapshot;
    type Mutation = SemioAudioMutation;
    type Diff = SemioAudioDiff;
    fn empty() -> Self { Self { snapshot: SemioAudioSnapshot::default() } }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot } }
    fn from_text(text: &str) -> Result<Self, store::TextError> {
        Ok(Self::from_snapshot(<SemioAudioSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
    }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
        Ok(Self::from_snapshot(<SemioAudioSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
    }
    fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
        let diff = apply_semio_audio_mutation(&mut self.snapshot, &mutation);
        (self, diff)
    }
    fn absorb(mut self, diff: Self::Diff) -> Self {
        self.snapshot = <SemioAudioDiff as protocol::MutationDiff<SemioAudioSnapshot>>::apply(&diff, &self.snapshot);
        self
    }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { Ok(self.snapshot) }
}
