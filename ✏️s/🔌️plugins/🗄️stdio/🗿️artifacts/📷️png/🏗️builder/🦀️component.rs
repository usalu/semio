//! 🏗️ PngBuilder (final, artifact-level) — delegates to the 1.2 standard.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::png::{PngDiff, PngMutation, PngSnapshot};
use crate::artifacts::png::standards::v1_2::builder::PngBuilder as PngRawBuilder;

#[derive(Clone, Debug, Default)]
pub struct PngBuilder(PngRawBuilder);

impl ArtifactBuilder for PngBuilder {
    type Snapshot = PngSnapshot;
    type Mutation = PngMutation;
    type Diff = PngDiff;
    fn empty() -> Self { Self(PngRawBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(PngRawBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(PngRawBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(PngRawBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> (Self, Self::Diff) { let (inner, diff) = self.0.mutate(mutation); (Self(inner), diff) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
