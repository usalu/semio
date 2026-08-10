//! 🏗️ MdBuilder (commonmark standard) — delegates to its ✳️any subset.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::md::{MdDiff, MdMutation, MdSnapshot};
use crate::artifacts::md::standards::v_commonmark::subsets::any::builder::MdBuilder as MdRawAnyBuilder;

#[derive(Clone, Debug, Default)]
pub struct MdBuilder(MdRawAnyBuilder);

impl ArtifactBuilder for MdBuilder {
    type Snapshot = MdSnapshot;
    type Mutation = MdMutation;
    type Diff = MdDiff;
    fn empty() -> Self { Self(MdRawAnyBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(MdRawAnyBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(MdRawAnyBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(MdRawAnyBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> Self { Self(self.0.mutate(mutation)) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
