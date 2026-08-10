//! 🏗️ DwgBuilder (final, artifact-level) — delegates to the ac1018 standard.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::dwg::{DwgDiff, DwgMutation, DwgSnapshot};
use crate::artifacts::dwg::standards::v_ac1018::builder::DwgBuilder as DwgRawBuilder;

#[derive(Clone, Debug, Default)]
pub struct DwgBuilder(DwgRawBuilder);

impl ArtifactBuilder for DwgBuilder {
    type Snapshot = DwgSnapshot;
    type Mutation = DwgMutation;
    type Diff = DwgDiff;
    fn empty() -> Self { Self(DwgRawBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(DwgRawBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(DwgRawBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(DwgRawBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> Self { Self(self.0.mutate(mutation)) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
