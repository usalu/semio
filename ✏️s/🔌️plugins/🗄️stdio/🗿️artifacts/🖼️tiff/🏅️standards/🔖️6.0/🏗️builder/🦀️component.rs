//! 🏗️ TiffBuilder (6.0 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::tiff::{TiffDiff, TiffMutation, TiffSnapshot};
use crate::artifacts::tiff::standards::v6_0::subsets::any::builder::TiffBuilder as TiffRawAnyBuilder;

#[derive(Clone, Debug, Default)]
pub struct TiffBuilder(TiffRawAnyBuilder);

impl ArtifactBuilder for TiffBuilder {
    type Snapshot = TiffSnapshot;
    type Mutation = TiffMutation;
    type Diff = TiffDiff;
    fn empty() -> Self { Self(TiffRawAnyBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(TiffRawAnyBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(TiffRawAnyBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(TiffRawAnyBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> (Self, Self::Diff) { let (inner, diff) = self.0.mutate(mutation); (Self(inner), diff) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
