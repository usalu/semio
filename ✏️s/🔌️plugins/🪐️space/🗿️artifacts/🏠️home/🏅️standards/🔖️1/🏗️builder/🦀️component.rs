//! 🏗️ SHomeBuilder (1 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::home::{SHomeDiff, SHomeMutation, SHomeSnapshot};
use crate::artifacts::home::standards::v1::subsets::any::builder::HomeBuilder as SHomeAnyBuilder;

#[derive(Clone, Debug)]
pub struct SHomeBuilder(SHomeAnyBuilder);

impl ArtifactBuilder for SHomeBuilder {
    type Snapshot = SHomeSnapshot;
    type Mutation = SHomeMutation;
    type Diff = SHomeDiff;
    fn empty() -> Self { Self(SHomeAnyBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(SHomeAnyBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(SHomeAnyBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(SHomeAnyBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> (Self, Self::Diff) { let (inner, diff) = self.0.mutate(mutation); (Self(inner), diff) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
