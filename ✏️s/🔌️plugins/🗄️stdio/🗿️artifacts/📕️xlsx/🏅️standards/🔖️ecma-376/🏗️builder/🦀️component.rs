//! 🏗️ XlsxBuilder (ecma-376 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::xlsx::schema::snapshot::XlsxCellValue;
use crate::artifacts::xlsx::{XlsxDiff, XlsxMutation, XlsxSnapshot};
use crate::artifacts::xlsx::standards::v_ecma_376::subsets::any::builder::XlsxBuilder as XlsxRawAnyBuilder;

#[derive(Clone, Debug, Default)]
pub struct XlsxBuilder(XlsxRawAnyBuilder);

impl ArtifactBuilder for XlsxBuilder {
    type Snapshot = XlsxSnapshot;
    type Mutation = XlsxMutation;
    type Diff = XlsxDiff;
    fn empty() -> Self { Self(XlsxRawAnyBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(XlsxRawAnyBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(XlsxRawAnyBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(XlsxRawAnyBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> Self { Self(self.0.mutate(mutation)) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}

impl XlsxBuilder {
    pub fn add_sheet(self, name: impl Into<String>) -> Self { Self(self.0.add_sheet(name)) }
    pub fn add_row(self, index: u32, values: Vec<XlsxCellValue>) -> Self { Self(self.0.add_row(index, values)) }
}
