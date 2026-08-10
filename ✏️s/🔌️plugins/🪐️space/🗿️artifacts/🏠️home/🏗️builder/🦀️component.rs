//! 🏗️ SHomeBuilder (final, artifact-level) — delegates to the 1 standard.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::home::{SHomeDiff, SHomeMutation, SHomeSnapshot};
use crate::artifacts::home::standards::v1::builder::SHomeBuilder as SHomeRawBuilder;

#[derive(Clone, Debug)]
pub struct SHomeBuilder(SHomeRawBuilder);

impl ArtifactBuilder for SHomeBuilder {
    type Snapshot = SHomeSnapshot;
    type Mutation = SHomeMutation;
    type Diff = SHomeDiff;
    fn empty() -> Self { Self(SHomeRawBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(SHomeRawBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(SHomeRawBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(SHomeRawBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> Self { Self(self.0.mutate(mutation)) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
