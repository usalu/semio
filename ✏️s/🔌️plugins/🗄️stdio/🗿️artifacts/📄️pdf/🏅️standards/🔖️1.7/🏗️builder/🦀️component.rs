//! 🏗️ PdfBuilder (1.7 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::diff::PdfDiff;
use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::mutations::PdfMutation;
use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::snapshot::{PdfInfo, PdfPage, PdfSnapshot};
use crate::artifacts::pdf::standards::v1_7::subsets::any::builder::PdfBuilder as PdfRawAnyBuilder;

#[derive(Clone, Debug, Default)]
pub struct PdfBuilder(PdfRawAnyBuilder);

impl PdfBuilder {
    /// ➕ Passthrough to the ✳️any subset's typed constructors (requirement #8).
    pub fn add_page(self, page: PdfPage) -> Self { Self(self.0.add_page(page)) }
    pub fn set_info(self, info: PdfInfo) -> Self { Self(self.0.set_info(info)) }
}

impl ArtifactBuilder for PdfBuilder {
    type Snapshot = PdfSnapshot;
    type Mutation = PdfMutation;
    type Diff = PdfDiff;
    fn empty() -> Self { Self(PdfRawAnyBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(PdfRawAnyBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(PdfRawAnyBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(PdfRawAnyBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> Self { Self(self.0.mutate(mutation)) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
