//! 🏗️ XlsxBuilder — local ArtifactBuilder until SDK Wave 3.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::xlsx::{XlsxDiff, XlsxMutation, XlsxSnapshot};

//#region 🔖️Builder
/// 🏗️ Builds a `stdio.xlsx` snapshot.
#[derive(Clone, Debug, Default)]
pub struct XlsxBuilder {
    snapshot: XlsxSnapshot,
    diagnostics: Vec<dsl::Diagnostic>,
}

impl ArtifactBuilder for XlsxBuilder {
    type Snapshot = XlsxSnapshot;
    type Mutation = XlsxMutation;
    type Diff = XlsxDiff;
    fn empty() -> Self {
        Self { snapshot: XlsxSnapshot::default(), diagnostics: Vec::new() }
    }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self {
        Self { snapshot, diagnostics: Vec::new() }
    }
    fn from_text(text: &str) -> Result<Self, store::TextError> {
        Ok(Self::from_snapshot(<XlsxSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
    }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
        Ok(Self::from_snapshot(<XlsxSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
    }
    fn mutate(mut self, mutation: Self::Mutation) -> Self {
        crate::artifacts::xlsx::schema::mutations::apply_xlsx_mutation(&mut self.snapshot, &mutation);
        self
    }
    fn absorb(mut self, diff: Self::Diff) -> Self {
        self.snapshot = <XlsxDiff as protocol::MutationDiff<XlsxSnapshot>>::apply(&diff, &self.snapshot);
        self
    }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
        if self.diagnostics.is_empty() { Ok(self.snapshot) } else { Err(self.diagnostics) }
    }
}
//#endregion 🔖️Builder
