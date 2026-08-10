//! 🏗️ BmpBuilder (final, artifact-level) — delegates to the v3 standard.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::bmp::{BmpDiff, BmpMutation, BmpSnapshot};
use crate::artifacts::bmp::standards::v_v3::builder::BmpBuilder as BmpRawBuilder;

#[derive(Clone, Debug, Default)]
pub struct BmpBuilder(BmpRawBuilder);

impl ArtifactBuilder for BmpBuilder {
    type Snapshot = BmpSnapshot;
    type Mutation = BmpMutation;
    type Diff = BmpDiff;
    fn empty() -> Self { Self(BmpRawBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(BmpRawBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(BmpRawBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(BmpRawBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> Self { Self(self.0.mutate(mutation)) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
