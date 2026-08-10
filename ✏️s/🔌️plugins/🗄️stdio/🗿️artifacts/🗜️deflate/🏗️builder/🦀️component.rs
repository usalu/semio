//! 🏗️ DeflateBuilder (final, artifact-level) — delegates to the rfc1950 standard.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::deflate::{DeflateDiff, DeflateMutation, DeflateSnapshot};
use crate::artifacts::deflate::standards::v_rfc1950::builder::DeflateBuilder as DeflateRawBuilder;

#[derive(Clone, Debug, Default)]
pub struct DeflateBuilder(DeflateRawBuilder);

impl ArtifactBuilder for DeflateBuilder {
    type Snapshot = DeflateSnapshot;
    type Mutation = DeflateMutation;
    type Diff = DeflateDiff;
    fn empty() -> Self { Self(DeflateRawBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(DeflateRawBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(DeflateRawBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(DeflateRawBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> (Self, Self::Diff) { let (inner, diff) = self.0.mutate(mutation); (Self(inner), diff) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
