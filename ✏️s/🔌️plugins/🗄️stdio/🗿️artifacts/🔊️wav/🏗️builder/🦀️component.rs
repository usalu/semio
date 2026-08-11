//! 🏗️ WavBuilder (final, artifact-level) — delegates to the only standard, riff-pcm.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::wav::{WavDiff, WavMutation, WavSnapshot};
use crate::artifacts::wav::standards::riff_pcm::builder::WavBuilder as WavRawBuilder;

#[derive(Clone, Debug, Default)]
pub struct WavBuilder(WavRawBuilder);

impl ArtifactBuilder for WavBuilder {
    type Snapshot = WavSnapshot;
    type Mutation = WavMutation;
    type Diff = WavDiff;
    fn empty() -> Self { Self(WavRawBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(WavRawBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(WavRawBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(WavRawBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> (Self, Self::Diff) { let (inner, diff) = self.0.mutate(mutation); (Self(inner), diff) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
