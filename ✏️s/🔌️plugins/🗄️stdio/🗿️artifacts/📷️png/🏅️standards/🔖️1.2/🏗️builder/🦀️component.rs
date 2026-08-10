//! 🏗️ PngBuilder (1.2 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::png::{PngDiff, PngMutation, PngSnapshot};
use crate::artifacts::png::standards::v1_2::subsets::any::builder::PngBuilder as PngRawAnyBuilder;

#[derive(Clone, Debug, Default)]
pub struct PngBuilder(PngRawAnyBuilder);

impl ArtifactBuilder for PngBuilder {
    type Snapshot = PngSnapshot;
    type Mutation = PngMutation;
    type Diff = PngDiff;
    fn empty() -> Self { Self(PngRawAnyBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(PngRawAnyBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(PngRawAnyBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(PngRawAnyBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> Self { Self(self.0.mutate(mutation)) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
