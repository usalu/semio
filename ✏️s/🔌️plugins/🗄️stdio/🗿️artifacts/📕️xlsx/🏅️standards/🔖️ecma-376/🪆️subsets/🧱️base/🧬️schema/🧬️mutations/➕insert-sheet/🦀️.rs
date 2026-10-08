//! ➕️ `insert-sheet` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`; every identifier the new
//! sheet needs (`sheetId`, relationship id, part path) is minted by the gesture ([`InsertSheet::minted`]) and travels in the payload's `slot`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct InsertSheet {
    pub(crate) sheet: XlsxSheet,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub(crate) index: Option<usize>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub(crate) slot: Option<XlsxSheetSlot>,
}

/// 🧩️ Everything the workbook holds for one sheet beyond its typed cells: the `sheet` entry's attributes, the workbook relationship to the worksheet part, the part itself and
/// the relationships that part owns. A slot is written verbatim, so a removed sheet returns exactly as it stood.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct XlsxSheetSlot {
    pub attrs: Vec<XmlAttr>,
    pub relationship: semio_s_artifact_stdio_zip::opc::OpcRelationship,
    pub part_path: String,
    pub content_type: String,
    pub document: semio_s_artifact_stdio_xml::schema::snapshot::XmlDocument,
    pub part_relationships: Vec<semio_s_artifact_stdio_zip::opc::OpcRelationship>,
}

impl InsertSheet {
    /// 🪪️ The insert a gesture builds for the typed `sheet`: it mints the sheet's slot (next free `sheetId`, relationship id and part path) against `base` ONCE and
    /// carries it, so applying the insert never derives anything.
    pub fn minted(base: &XlsxSnapshot, sheet: XlsxSheet, index: Option<usize>) -> Result<Self, String> {
        let slot = canonical_edit::mint_sheet_slot(base, &sheet)?;
        Ok(Self { sheet, index, slot: Some(slot) })
    }
}

impl protocol::MutationKind<XlsxSnapshot, XlsxMutation> for InsertSheet {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "sheet", kind: "insert-sheet", record: "InsertSheet" };

    fn diff(&self, base: &XlsxSnapshot) -> protocol::MutationOutcome<XlsxDiff> {
        plan_outcome(canonical_edit::insert_sheet_plan(base, &self.sheet, self.index, self.slot.as_ref()))
    }

    fn inverse(&self, base: &XlsxSnapshot) -> Result<Vec<XlsxMutation>, semio_framework_value::ValueError> {
        Ok(plan_inverse(canonical_edit::insert_sheet_plan(base, &self.sheet, self.index, self.slot.as_ref())))
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Insert sheet", "Arbeitsblatt einfügen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
