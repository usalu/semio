//! 🏗️ Mp4Builder (final, artifact-level) — delegates to the only standard, isobmff.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::mp4::{Mp4Diff, Mp4Mutation, Mp4Snapshot};
use crate::artifacts::mp4::standards::isobmff::builder::Mp4Builder as Mp4RawBuilder;

#[derive(Clone, Debug, Default)]
pub struct Mp4Builder(Mp4RawBuilder);

impl ArtifactBuilder for Mp4Builder {
    type Snapshot = Mp4Snapshot;
    type Mutation = Mp4Mutation;
    type Diff = Mp4Diff;
    fn empty() -> Self { Self(Mp4RawBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(Mp4RawBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(Mp4RawBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(Mp4RawBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> (Self, Self::Diff) { let (inner, diff) = self.0.mutate(mutation); (Self(inner), diff) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
