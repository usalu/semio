//! 🏗️ GifBuilder (final, artifact-level) — delegates to the 87a standard.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::gif::{GifDiff, GifMutation, GifSnapshot};
use crate::artifacts::gif::standards::v87a::builder::GifBuilder as GifRawBuilder;

#[derive(Clone, Debug, Default)]
pub struct GifBuilder(GifRawBuilder);

impl ArtifactBuilder for GifBuilder {
    type Snapshot = GifSnapshot;
    type Mutation = GifMutation;
    type Diff = GifDiff;
    fn empty() -> Self { Self(GifRawBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(GifRawBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(GifRawBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(GifRawBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> Self { Self(self.0.mutate(mutation)) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
