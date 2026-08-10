//! 🏗️ RewriteBuilder (1 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::rewrite::{RewriteDiff, RewriteRuleMutation, RewriteSnapshot};
use crate::artifacts::rewrite::standards::v1::subsets::any::builder::RewriteBuilder as RewriteAnyBuilder;

#[derive(Clone, Debug)]
pub struct RewriteBuilder(RewriteAnyBuilder);

impl ArtifactBuilder for RewriteBuilder {
    type Snapshot = RewriteSnapshot;
    type Mutation = RewriteRuleMutation;
    type Diff = RewriteDiff;
    fn empty() -> Self { Self(RewriteAnyBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(RewriteAnyBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(RewriteAnyBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(RewriteAnyBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> Self { Self(self.0.mutate(mutation)) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
