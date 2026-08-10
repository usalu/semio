//! 🏗️ BinaryBuilder (final, artifact-level) — delegates to the raw standard, which delegates to
//! its ✳️any subset. Real materialization logic lives at the subset level; this facade is what
//! every other artifact's io leaves and the OS reach for when it doesn't care which standard.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::binary::{BinaryDiff, BinaryMutation, BinarySnapshot};
use crate::artifacts::binary::standards::v_raw::builder::BinaryBuilder as BinaryRawBuilder;

#[derive(Clone, Debug, Default)]
pub struct BinaryBuilder(BinaryRawBuilder);

impl ArtifactBuilder for BinaryBuilder {
    type Snapshot = BinarySnapshot;
    type Mutation = BinaryMutation;
    type Diff = BinaryDiff;
    fn empty() -> Self {
        Self(BinaryRawBuilder::empty())
    }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self {
        Self(BinaryRawBuilder::from_snapshot(snapshot))
    }
    fn from_text(text: &str) -> Result<Self, store::TextError> {
        Ok(Self(BinaryRawBuilder::from_text(text)?))
    }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
        Ok(Self(BinaryRawBuilder::from_binary(bytes)?))
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
