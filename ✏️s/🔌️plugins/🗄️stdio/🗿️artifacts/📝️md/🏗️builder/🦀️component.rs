//! 🏗️ MdBuilder (final, artifact-level) — delegates to the commonmark standard.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::md::{MdDiff, MdMutation, MdSnapshot};
use crate::artifacts::md::standards::v_commonmark::builder::MdBuilder as MdRawBuilder;

#[derive(Clone, Debug, Default)]
pub struct MdBuilder(MdRawBuilder);

impl ArtifactBuilder for MdBuilder {
    type Snapshot = MdSnapshot;
    type Mutation = MdMutation;
    type Diff = MdDiff;
    fn empty() -> Self { Self(MdRawBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(MdRawBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(MdRawBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(MdRawBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> (Self, Self::Diff) { let (inner, diff) = self.0.mutate(mutation); (Self(inner), diff) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
