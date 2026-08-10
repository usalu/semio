//! 🏗️ XlsxBuilder — local ArtifactBuilder until SDK Wave 3.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::xlsx::schema::snapshot::{XlsxCell, XlsxCellValue, XlsxRow, XlsxSheet};
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

//#region 🔖️TypedConstructors
/// 🧱️ Typed content constructors — build a workbook from sheets and rows of cell values,
/// auto-assigning A1-style references (`crate::artifacts::xlsx::engine::column_letter`).
impl XlsxBuilder {
    /// ➕️ Appends a new (initially empty) sheet and makes it the active sheet for `add_row`.
    pub fn add_sheet(mut self, name: impl Into<String>) -> Self {
        self.snapshot.workbook.sheets.push(XlsxSheet { name: name.into(), rows: Vec::new() });
        self.rebuild()
    }

    /// ➕️ Appends a row of values to the active sheet (the most recently added one), assigning
    /// A1-style references left-to-right for the given `index`.
    pub fn add_row(mut self, index: u32, values: Vec<XlsxCellValue>) -> Self {
        if let Some(sheet) = self.snapshot.workbook.sheets.last_mut() {
            let cells = values
                .into_iter()
                .enumerate()
                .map(|(col, value)| XlsxCell { reference: format!("{}{index}", crate::artifacts::xlsx::engine::column_letter(col as u32)), value })
                .collect();
            sheet.rows.push(XlsxRow { index, cells });
        }
        self.rebuild()
    }

    fn rebuild(mut self) -> Self {
        self.snapshot = crate::artifacts::xlsx::engine::build_minimal_xlsx(self.snapshot.workbook);
        self
    }
}
//#endregion 🔖️TypedConstructors
