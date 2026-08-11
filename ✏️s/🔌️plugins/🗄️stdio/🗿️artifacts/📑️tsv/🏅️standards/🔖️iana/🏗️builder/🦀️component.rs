//! 🏗️ TsvBuilder (iana standard) — delegates to its ✳️any subset.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::tsv::standards::iana::subsets::any::schema::diff::TsvDiff;
use crate::artifacts::tsv::standards::iana::subsets::any::schema::mutations::TsvMutation;
use crate::artifacts::tsv::standards::iana::subsets::any::schema::snapshot::TsvSnapshot;
use crate::artifacts::tsv::standards::iana::subsets::any::builder::TsvBuilder as TsvRawAnyBuilder;

#[derive(Clone, Debug, Default)]
pub struct TsvBuilder(TsvRawAnyBuilder);

impl ArtifactBuilder for TsvBuilder {
    type Snapshot = TsvSnapshot;
    type Mutation = TsvMutation;
    type Diff = TsvDiff;
    fn empty() -> Self { Self(TsvRawAnyBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(TsvRawAnyBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(TsvRawAnyBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(TsvRawAnyBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> (Self, Self::Diff) { let (inner, diff) = self.0.mutate(mutation); (Self(inner), diff) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
