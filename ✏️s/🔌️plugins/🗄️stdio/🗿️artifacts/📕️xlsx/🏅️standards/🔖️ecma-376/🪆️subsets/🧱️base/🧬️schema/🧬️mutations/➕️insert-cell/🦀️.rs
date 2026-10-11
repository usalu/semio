//! ➕️ Inserts one revision-bound cell into a canonical SpreadsheetML vacancy.

use super::*;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct InsertCell {
    pub(crate) address: cell_address::XlsxCellVacancyAddress,
    pub(crate) value: XlsxCellValue,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub(crate) node: Option<XmlNode>,
}

impl protocol::MutationKind<XlsxSnapshot, XlsxMutation> for InsertCell {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "cell", kind: "insert-cell", record: "InsertCell" };

    fn diff(&self, base: &XlsxSnapshot) -> protocol::MutationOutcome<XlsxDiff> {
        plan_outcome(canonical_edit::insert_cell_plan(base, &self.address, &self.value, self.node.as_ref()))
    }

    fn inverse(&self, base: &XlsxSnapshot) -> Result<Vec<XlsxMutation>, semio_framework_value::ValueError> {
        Ok(plan_inverse(canonical_edit::insert_cell_plan(base, &self.address, &self.value, self.node.as_ref())))
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Insert cell", "Zelle einfügen")
    }
    fn target(&self) -> Vec<String> {
        let mut target = vec!["xmlParts".into(), self.address.worksheet.part_path.clone(), "document".into(), "root".into()];
        target.extend(self.address.worksheet.node_path.iter().map(usize::to_string));
        target.push(self.address.row.to_string());
        target.push(self.address.column.to_string());
        target
    }
}
