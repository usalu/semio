//! 🏗️ NoteBuilder (1 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::note::{NoteDiff, NoteMutation, NoteSnapshot};
use crate::artifacts::note::standards::v1::subsets::any::builder::NoteBuilder as NoteAnyBuilder;

#[derive(Clone, Debug, Default)]
pub struct NoteBuilder(NoteAnyBuilder);

impl ArtifactBuilder for NoteBuilder {
    type Snapshot = NoteSnapshot;
    type Mutation = NoteMutation;
    type Diff = NoteDiff;
    fn empty() -> Self { Self(NoteAnyBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(NoteAnyBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(NoteAnyBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(NoteAnyBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> (Self, Self::Diff) { let (inner, diff) = self.0.mutate(mutation); (Self(inner), diff) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
