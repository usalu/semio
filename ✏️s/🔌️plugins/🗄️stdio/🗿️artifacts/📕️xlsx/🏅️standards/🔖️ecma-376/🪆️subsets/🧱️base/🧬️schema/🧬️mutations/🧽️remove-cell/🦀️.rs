//! 🔻️ `remove-cell` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct RemoveCell {
    pub(crate) address: cell_address::XlsxCellAddress,
}

impl protocol::MutationKind<XlsxSnapshot, XlsxMutation> for RemoveCell {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "cell", kind: "remove-cell", record: "RemoveCell" };

    fn diff(&self, base: &XlsxSnapshot) -> protocol::MutationOutcome<XlsxDiff> {
        plan_outcome(canonical_edit::remove_cell_plan(base, &self.address))
    }

    fn inverse(&self, base: &XlsxSnapshot) -> Result<Vec<XlsxMutation>, semio_framework_value::ValueError> {
        Ok(plan_inverse(canonical_edit::remove_cell_plan(base, &self.address)))
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Remove cell", "Zelle entfernen")
    }
    fn target(&self) -> Vec<String> {
        let mut target = vec!["xmlParts".into(), self.address.part_path.clone(), "document".into(), "root".into()];
        target.extend(self.address.node_path.iter().map(usize::to_string));
        target
    }
}
//#endregion 🔖️Payload
