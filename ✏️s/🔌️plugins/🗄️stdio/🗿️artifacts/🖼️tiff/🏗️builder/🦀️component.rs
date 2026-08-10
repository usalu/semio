//! 🏗️ TiffBuilder (final, artifact-level) — delegates to the 6.0 standard.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::tiff::{TiffDiff, TiffMutation, TiffSnapshot};
use crate::artifacts::tiff::standards::v6_0::builder::TiffBuilder as TiffRawBuilder;

#[derive(Clone, Debug, Default)]
pub struct TiffBuilder(TiffRawBuilder);

impl ArtifactBuilder for TiffBuilder {
    type Snapshot = TiffSnapshot;
    type Mutation = TiffMutation;
    type Diff = TiffDiff;
    fn empty() -> Self { Self(TiffRawBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(TiffRawBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(TiffRawBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(TiffRawBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> Self { Self(self.0.mutate(mutation)) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
