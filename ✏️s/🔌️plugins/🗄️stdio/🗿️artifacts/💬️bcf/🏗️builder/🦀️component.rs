//! 🏗️ BcfBuilder (final, artifact-level) — delegates to the 2.1 standard.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::bcf::{BcfDiff, BcfMutation, BcfSnapshot};
use crate::artifacts::bcf::standards::v2_1::builder::BcfBuilder as BcfRawBuilder;

#[derive(Clone, Debug, Default)]
pub struct BcfBuilder(BcfRawBuilder);

impl ArtifactBuilder for BcfBuilder {
    type Snapshot = BcfSnapshot;
    type Mutation = BcfMutation;
    type Diff = BcfDiff;
    fn empty() -> Self { Self(BcfRawBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(BcfRawBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(BcfRawBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(BcfRawBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> (Self, Self::Diff) { let (inner, diff) = self.0.mutate(mutation); (Self(inner), diff) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
