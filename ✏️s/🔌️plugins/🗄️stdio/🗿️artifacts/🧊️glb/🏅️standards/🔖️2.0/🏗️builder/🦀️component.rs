//! 🏗️ GlbBuilder (2.0 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::glb::{GlbDiff, GlbMutation, GlbSnapshot};
use crate::artifacts::glb::standards::v2_0::subsets::any::builder::GlbBuilder as GlbRawAnyBuilder;

#[derive(Clone, Debug, Default)]
pub struct GlbBuilder(GlbRawAnyBuilder);

impl ArtifactBuilder for GlbBuilder {
    type Snapshot = GlbSnapshot;
    type Mutation = GlbMutation;
    type Diff = GlbDiff;
    fn empty() -> Self { Self(GlbRawAnyBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(GlbRawAnyBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(GlbRawAnyBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(GlbRawAnyBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> Self { Self(self.0.mutate(mutation)) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
