//! 🏗️ BinaryBuilder (raw standard) — delegates to the single ✳️any subset today; the standard
//! level exists so a future second subset of the raw standard aggregates here without touching
//! the artifact-level facade.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::binary::{BinaryDiff, BinaryMutation, BinarySnapshot};
use crate::artifacts::binary::standards::v_raw::subsets::any::builder::BinaryBuilder as BinaryRawAnyBuilder;

#[derive(Clone, Debug, Default)]
pub struct BinaryBuilder(BinaryRawAnyBuilder);

impl ArtifactBuilder for BinaryBuilder {
    type Snapshot = BinarySnapshot;
    type Mutation = BinaryMutation;
    type Diff = BinaryDiff;
    fn empty() -> Self {
        Self(BinaryRawAnyBuilder::empty())
    }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self {
        Self(BinaryRawAnyBuilder::from_snapshot(snapshot))
    }
    fn from_text(text: &str) -> Result<Self, store::TextError> {
        Ok(Self(BinaryRawAnyBuilder::from_text(text)?))
    }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
        Ok(Self(BinaryRawAnyBuilder::from_binary(bytes)?))
    }
    fn mutate(self, mutation: Self::Mutation) -> Self {
        Self(self.0.mutate(mutation))
    }
    fn absorb(self, diff: Self::Diff) -> Self {
        Self(self.0.absorb(diff))
    }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
        self.0.build()
    }
}
