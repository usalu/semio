//! 🏗️ DrawBuilder (1 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::draw::{DrawDiff, DrawMutation, DrawSnapshot};
use crate::artifacts::draw::standards::v1::subsets::any::builder::DrawBuilder as DrawAnyBuilder;

#[derive(Clone, Debug, Default)]
pub struct DrawBuilder(DrawAnyBuilder);

impl ArtifactBuilder for DrawBuilder {
    type Snapshot = DrawSnapshot;
    type Mutation = DrawMutation;
    type Diff = DrawDiff;
    fn empty() -> Self { Self(DrawAnyBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(DrawAnyBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(DrawAnyBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(DrawAnyBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> (Self, Self::Diff) { let (inner, diff) = self.0.mutate(mutation); (Self(inner), diff) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
