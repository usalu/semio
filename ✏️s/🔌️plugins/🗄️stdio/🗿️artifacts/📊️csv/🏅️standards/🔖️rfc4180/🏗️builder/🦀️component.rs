//! 🏗️ CsvBuilder (rfc4180 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::csv::{CsvDiff, CsvMutation, CsvSnapshot};
use crate::artifacts::csv::standards::v_rfc4180::subsets::any::builder::CsvBuilder as CsvRawAnyBuilder;

#[derive(Clone, Debug, Default)]
pub struct CsvBuilder(CsvRawAnyBuilder);

impl ArtifactBuilder for CsvBuilder {
    type Snapshot = CsvSnapshot;
    type Mutation = CsvMutation;
    type Diff = CsvDiff;
    fn empty() -> Self { Self(CsvRawAnyBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(CsvRawAnyBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(CsvRawAnyBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(CsvRawAnyBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> (Self, Self::Diff) { let (inner, diff) = self.0.mutate(mutation); (Self(inner), diff) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
