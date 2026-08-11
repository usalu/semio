//! 🏗️ AviBuilder (final, artifact-level) — delegates to the only standard, 1.0.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::avi::{AviDiff, AviMutation, AviSnapshot};
use crate::artifacts::avi::standards::v1_0::builder::AviBuilder as AviRawBuilder;

#[derive(Clone, Debug, Default)]
pub struct AviBuilder(AviRawBuilder);

impl ArtifactBuilder for AviBuilder {
    type Snapshot = AviSnapshot;
    type Mutation = AviMutation;
    type Diff = AviDiff;
    fn empty() -> Self { Self(AviRawBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(AviRawBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(AviRawBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(AviRawBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> (Self, Self::Diff) { let (inner, diff) = self.0.mutate(mutation); (Self(inner), diff) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
