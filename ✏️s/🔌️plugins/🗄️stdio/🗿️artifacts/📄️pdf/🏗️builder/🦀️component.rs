//! 🏗️ PdfBuilder (final, artifact-level) — delegates to the 1.7 standard (canonical per the S-6
//! twin fix: `.claude/plans/the-current-schemas-are-scalable-journal.md`; 1.4 stays reachable at
//! `standards::v1_4::builder`).

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::pdf::{PdfDiff, PdfMutation, PdfSnapshot};
use crate::artifacts::pdf::standards::v1_7::builder::PdfBuilder as PdfRawBuilder;

#[derive(Clone, Debug, Default)]
pub struct PdfBuilder(PdfRawBuilder);

impl ArtifactBuilder for PdfBuilder {
    type Snapshot = PdfSnapshot;
    type Mutation = PdfMutation;
    type Diff = PdfDiff;
    fn empty() -> Self { Self(PdfRawBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(PdfRawBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(PdfRawBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(PdfRawBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> (Self, Self::Diff) { let (inner, diff) = self.0.mutate(mutation); (Self(inner), diff) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
