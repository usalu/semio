//! 🏗️ StlBuilder (ascii standard) — delegates to its ✳️any subset.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::stl::{StlDiff, StlMutation, StlSnapshot};
use crate::artifacts::stl::standards::v_ascii::subsets::any::builder::StlBuilder as StlRawAnyBuilder;

#[derive(Clone, Debug, Default)]
pub struct StlBuilder(StlRawAnyBuilder);

impl ArtifactBuilder for StlBuilder {
    type Snapshot = StlSnapshot;
    type Mutation = StlMutation;
    type Diff = StlDiff;
    fn empty() -> Self { Self(StlRawAnyBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(StlRawAnyBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(StlRawAnyBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(StlRawAnyBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> (Self, Self::Diff) { let (inner, diff) = self.0.mutate(mutation); (Self(inner), diff) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
