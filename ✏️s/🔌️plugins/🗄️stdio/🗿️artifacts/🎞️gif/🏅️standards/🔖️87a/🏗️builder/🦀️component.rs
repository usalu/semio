//! 🏗️ GifBuilder (87a standard) — delegates to its ✳️any subset.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::gif::standards::v87a::subsets::any::schema::{diff::GifDiff, mutations::GifMutation, snapshot::GifSnapshot};
use crate::artifacts::gif::standards::v87a::subsets::any::builder::GifBuilder as GifRawAnyBuilder;

#[derive(Clone, Debug, Default)]
pub struct GifBuilder(GifRawAnyBuilder);

impl ArtifactBuilder for GifBuilder {
    type Snapshot = GifSnapshot;
    type Mutation = GifMutation;
    type Diff = GifDiff;
    fn empty() -> Self { Self(GifRawAnyBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(GifRawAnyBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(GifRawAnyBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(GifRawAnyBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> (Self, Self::Diff) { let (inner, diff) = self.0.mutate(mutation); (Self(inner), diff) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
