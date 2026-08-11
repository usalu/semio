//! 🏗️ WavBuilder — 🚧 scaffolded by W1b: local `ArtifactBuilder` round-tripping the
//! minimal snapshot. W2 adds typed constructors + the real mutation vocabulary.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::wav::standards::riff_pcm::subsets::any::schema::diff::WavDiff;
use crate::artifacts::wav::standards::riff_pcm::subsets::any::schema::mutations::{WavMutation, apply_wav_mutation};
use crate::artifacts::wav::standards::riff_pcm::subsets::any::schema::snapshot::WavSnapshot;

#[derive(Clone, Debug, Default)]
pub struct WavBuilder { snapshot: WavSnapshot }

impl ArtifactBuilder for WavBuilder {
    type Snapshot = WavSnapshot;
    type Mutation = WavMutation;
    type Diff = WavDiff;
    fn empty() -> Self { Self { snapshot: WavSnapshot::default() } }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot } }
    fn from_text(text: &str) -> Result<Self, store::TextError> {
        Ok(Self::from_snapshot(<WavSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
    }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
        Ok(Self::from_snapshot(<WavSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
    }
    fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
        let diff = apply_wav_mutation(&mut self.snapshot, &mutation);
        (self, diff)
    }
    fn absorb(mut self, diff: Self::Diff) -> Self {
        self.snapshot = <WavDiff as protocol::MutationDiff<WavSnapshot>>::apply(&diff, &self.snapshot);
        self
    }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { Ok(self.snapshot) }
}
