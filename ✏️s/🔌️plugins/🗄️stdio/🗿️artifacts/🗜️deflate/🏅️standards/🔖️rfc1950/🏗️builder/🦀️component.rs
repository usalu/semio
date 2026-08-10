//! 🏗️ DeflateBuilder (rfc1950 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::deflate::{DeflateDiff, DeflateMutation, DeflateSnapshot};
use crate::artifacts::deflate::standards::v_rfc1950::subsets::any::builder::DeflateBuilder as DeflateRawAnyBuilder;

#[derive(Clone, Debug, Default)]
pub struct DeflateBuilder(DeflateRawAnyBuilder);

impl ArtifactBuilder for DeflateBuilder {
    type Snapshot = DeflateSnapshot;
    type Mutation = DeflateMutation;
    type Diff = DeflateDiff;
    fn empty() -> Self { Self(DeflateRawAnyBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(DeflateRawAnyBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(DeflateRawAnyBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(DeflateRawAnyBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> Self { Self(self.0.mutate(mutation)) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
