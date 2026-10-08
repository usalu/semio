//! ✍️ `set-cell` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetCell {
    pub(crate) address: cell_address::XlsxCellAddress,
    pub(crate) value: XlsxCellValue,
}

impl protocol::MutationKind<XlsxSnapshot, XlsxMutation> for SetCell {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "cell", kind: "set-cell", record: "SetCell" };

    fn diff(&self, base: &XlsxSnapshot) -> protocol::MutationOutcome<XlsxDiff> {
        plan_outcome(canonical_edit::set_cell_plan(base, &self.address, &self.value))
    }

    fn inverse(&self, base: &XlsxSnapshot) -> Result<Vec<XlsxMutation>, semio_framework_value::ValueError> {
        Ok(plan_inverse(canonical_edit::set_cell_plan(base, &self.address, &self.value)))
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set cell", "Zelle setzen")
    }
    fn target(&self) -> Vec<String> {
        let mut target = vec!["xmlParts".into(), self.address.part_path.clone(), "document".into(), "root".into()];
        target.extend(self.address.node_path.iter().map(usize::to_string));
        target
    }
}
//#endregion 🔖️Payload
