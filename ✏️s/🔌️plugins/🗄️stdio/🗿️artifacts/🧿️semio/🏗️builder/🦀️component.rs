//! 🏗️ SemioBuilder (final, artifact-level) — delegates to the only standard, v1.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::semio::{SemioDiff, SemioMutation, SemioSnapshot};
use crate::artifacts::semio::standards::v1::builder::SemioBuilder as SemioRawBuilder;

#[derive(Clone, Debug, Default)]
pub struct SemioBuilder(SemioRawBuilder);

impl ArtifactBuilder for SemioBuilder {
    type Snapshot = SemioSnapshot;
    type Mutation = SemioMutation;
    type Diff = SemioDiff;
    fn empty() -> Self { Self(SemioRawBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(SemioRawBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(SemioRawBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(SemioRawBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> (Self, Self::Diff) { let (inner, diff) = self.0.mutate(mutation); (Self(inner), diff) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
