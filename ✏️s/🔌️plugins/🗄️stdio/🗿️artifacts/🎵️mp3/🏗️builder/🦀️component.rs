//! 🏗️ Mp3Builder (final, artifact-level) — delegates to the only standard, mpeg1-layer3.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::mp3::{Mp3Diff, Mp3Mutation, Mp3Snapshot};
use crate::artifacts::mp3::standards::mpeg1_layer3::builder::Mp3Builder as Mp3RawBuilder;

#[derive(Clone, Debug, Default)]
pub struct Mp3Builder(Mp3RawBuilder);

impl ArtifactBuilder for Mp3Builder {
    type Snapshot = Mp3Snapshot;
    type Mutation = Mp3Mutation;
    type Diff = Mp3Diff;
    fn empty() -> Self { Self(Mp3RawBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(Mp3RawBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(Mp3RawBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(Mp3RawBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> (Self, Self::Diff) { let (inner, diff) = self.0.mutate(mutation); (Self(inner), diff) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
