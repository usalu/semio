//! 🏗️ ImperativeBuilder (final, artifact-level) — delegates to the 1 standard.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::imperative::{ImperativeDiff, ImperativeMutation, ImperativeSnapshot};
use crate::artifacts::imperative::standards::v1::builder::ImperativeBuilder as ImperativeRawBuilder;

#[derive(Clone, Debug)]
pub struct ImperativeBuilder(ImperativeRawBuilder);

impl ArtifactBuilder for ImperativeBuilder {
    type Snapshot = ImperativeSnapshot;
    type Mutation = ImperativeMutation;
    type Diff = ImperativeDiff;
    fn empty() -> Self { Self(ImperativeRawBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(ImperativeRawBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(ImperativeRawBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(ImperativeRawBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> (Self, Self::Diff) { let (inner, diff) = self.0.mutate(mutation); (Self(inner), diff) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
