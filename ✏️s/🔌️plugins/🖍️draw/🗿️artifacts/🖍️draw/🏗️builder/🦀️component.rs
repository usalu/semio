//! 🏗️ DrawBuilder (final, artifact-level) — delegates to the 1 standard.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::draw::{DrawDiff, DrawMutation, DrawSnapshot};
use crate::artifacts::draw::standards::v1::builder::DrawBuilder as DrawRawBuilder;

#[derive(Clone, Debug, Default)]
pub struct DrawBuilder(DrawRawBuilder);

impl ArtifactBuilder for DrawBuilder {
    type Snapshot = DrawSnapshot;
    type Mutation = DrawMutation;
    type Diff = DrawDiff;
    fn empty() -> Self { Self(DrawRawBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(DrawRawBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(DrawRawBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(DrawRawBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> (Self, Self::Diff) { let (inner, diff) = self.0.mutate(mutation); (Self(inner), diff) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
