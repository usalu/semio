//! 🏗️ ZipBuilder (2.0 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::zip::{ZipDiff, ZipMutation, ZipSnapshot};
use crate::artifacts::zip::standards::v2_0::subsets::any::builder::ZipBuilder as ZipRawAnyBuilder;

#[derive(Clone, Debug, Default)]
pub struct ZipBuilder(ZipRawAnyBuilder);

impl ArtifactBuilder for ZipBuilder {
    type Snapshot = ZipSnapshot;
    type Mutation = ZipMutation;
    type Diff = ZipDiff;
    fn empty() -> Self { Self(ZipRawAnyBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(ZipRawAnyBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(ZipRawAnyBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(ZipRawAnyBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> Self { Self(self.0.mutate(mutation)) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
