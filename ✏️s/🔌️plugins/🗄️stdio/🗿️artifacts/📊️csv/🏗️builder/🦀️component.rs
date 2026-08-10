//! 🏗️ CsvBuilder (final, artifact-level) — delegates to the rfc4180 standard.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::csv::{CsvDiff, CsvMutation, CsvSnapshot};
use crate::artifacts::csv::standards::v_rfc4180::builder::CsvBuilder as CsvRawBuilder;

#[derive(Clone, Debug, Default)]
pub struct CsvBuilder(CsvRawBuilder);

impl ArtifactBuilder for CsvBuilder {
    type Snapshot = CsvSnapshot;
    type Mutation = CsvMutation;
    type Diff = CsvDiff;
    fn empty() -> Self { Self(CsvRawBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(CsvRawBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(CsvRawBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(CsvRawBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> (Self, Self::Diff) { let (inner, diff) = self.0.mutate(mutation); (Self(inner), diff) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
