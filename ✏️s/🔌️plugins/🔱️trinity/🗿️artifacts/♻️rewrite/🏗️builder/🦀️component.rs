//! 🏗️ RewriteBuilder (final, artifact-level) — delegates to the 1 standard.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::rewrite::{RewriteDiff, RewriteRuleMutation, RewriteSnapshot};
use crate::artifacts::rewrite::standards::v1::builder::RewriteBuilder as RewriteRawBuilder;

#[derive(Clone, Debug)]
pub struct RewriteBuilder(RewriteRawBuilder);

impl ArtifactBuilder for RewriteBuilder {
    type Snapshot = RewriteSnapshot;
    type Mutation = RewriteRuleMutation;
    type Diff = RewriteDiff;
    fn empty() -> Self { Self(RewriteRawBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(RewriteRawBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(RewriteRawBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(RewriteRawBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> (Self, Self::Diff) { let (inner, diff) = self.0.mutate(mutation); (Self(inner), diff) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
