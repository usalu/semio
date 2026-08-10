//! 🏗️ BmpBuilder (v3 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::bmp::{BmpDiff, BmpMutation, BmpSnapshot};
use crate::artifacts::bmp::standards::v_v3::subsets::any::builder::BmpBuilder as BmpRawAnyBuilder;

#[derive(Clone, Debug, Default)]
pub struct BmpBuilder(BmpRawAnyBuilder);

impl ArtifactBuilder for BmpBuilder {
    type Snapshot = BmpSnapshot;
    type Mutation = BmpMutation;
    type Diff = BmpDiff;
    fn empty() -> Self { Self(BmpRawAnyBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(BmpRawAnyBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(BmpRawAnyBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(BmpRawAnyBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> (Self, Self::Diff) { let (inner, diff) = self.0.mutate(mutation); (Self(inner), diff) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
