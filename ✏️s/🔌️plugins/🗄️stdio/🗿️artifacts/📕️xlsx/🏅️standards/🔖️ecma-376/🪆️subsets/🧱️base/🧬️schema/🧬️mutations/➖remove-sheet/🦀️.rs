//! ➖️ `remove-sheet` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct RemoveSheet {
    pub(crate) name: String,
}

impl protocol::MutationKind<XlsxSnapshot, XlsxMutation> for RemoveSheet {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "sheet", kind: "remove-sheet", record: "RemoveSheet" };

    fn diff(&self, base: &XlsxSnapshot) -> protocol::MutationOutcome<XlsxDiff> {
        plan_outcome(canonical_edit::remove_sheet_plan(base, &self.name))
    }

    fn inverse(&self, base: &XlsxSnapshot) -> Result<Vec<XlsxMutation>, semio_framework_value::ValueError> {
        Ok(plan_inverse(canonical_edit::remove_sheet_plan(base, &self.name)))
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Remove sheet", "Arbeitsblatt entfernen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
