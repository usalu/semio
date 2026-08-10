//! 🏗️ StlBuilder (final, artifact-level) — delegates to the ascii standard.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::stl::{StlDiff, StlMutation, StlSnapshot};
use crate::artifacts::stl::standards::v_ascii::builder::StlBuilder as StlRawBuilder;

#[derive(Clone, Debug, Default)]
pub struct StlBuilder(StlRawBuilder);

impl ArtifactBuilder for StlBuilder {
    type Snapshot = StlSnapshot;
    type Mutation = StlMutation;
    type Diff = StlDiff;
    fn empty() -> Self { Self(StlRawBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(StlRawBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(StlRawBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(StlRawBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> (Self, Self::Diff) { let (inner, diff) = self.0.mutate(mutation); (Self(inner), diff) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
