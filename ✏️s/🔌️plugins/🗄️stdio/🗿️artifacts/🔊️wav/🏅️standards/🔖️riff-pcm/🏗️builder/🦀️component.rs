//! 🏗️ WavBuilder (riff-pcm standard) — delegates to its ✳️any subset.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::wav::standards::riff_pcm::subsets::any::schema::diff::WavDiff;
use crate::artifacts::wav::standards::riff_pcm::subsets::any::schema::mutations::WavMutation;
use crate::artifacts::wav::standards::riff_pcm::subsets::any::schema::snapshot::WavSnapshot;
use crate::artifacts::wav::standards::riff_pcm::subsets::any::builder::WavBuilder as WavRawAnyBuilder;

#[derive(Clone, Debug, Default)]
pub struct WavBuilder(WavRawAnyBuilder);

impl ArtifactBuilder for WavBuilder {
    type Snapshot = WavSnapshot;
    type Mutation = WavMutation;
    type Diff = WavDiff;
    fn empty() -> Self { Self(WavRawAnyBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(WavRawAnyBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(WavRawAnyBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(WavRawAnyBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> (Self, Self::Diff) { let (inner, diff) = self.0.mutate(mutation); (Self(inner), diff) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
