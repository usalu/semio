//! 🏗️ BcfBuilder (2.1 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::bcf::{BcfDiff, BcfMutation, BcfSnapshot};
use crate::artifacts::bcf::standards::v2_1::subsets::any::builder::BcfBuilder as BcfRawAnyBuilder;

#[derive(Clone, Debug, Default)]
pub struct BcfBuilder(BcfRawAnyBuilder);

impl ArtifactBuilder for BcfBuilder {
    type Snapshot = BcfSnapshot;
    type Mutation = BcfMutation;
    type Diff = BcfDiff;
    fn empty() -> Self { Self(BcfRawAnyBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(BcfRawAnyBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(BcfRawAnyBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(BcfRawAnyBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> Self { Self(self.0.mutate(mutation)) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
