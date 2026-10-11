//! 🧱️ `add-element` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct AddElement {
    pub(crate) index: usize,
    pub(crate) element: PlyElement,
}

impl protocol::MutationKind<PlySnapshot, PlyMutation> for AddElement {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "add", entity: "element", kind: "add-element", record: "AddElement" };

    fn diff(&self, base: &PlySnapshot) -> protocol::MutationOutcome<<PlyMutation as Mutation<PlySnapshot>>::Diff> {
        let Self { index, element } = self;
        protocol::MutationOutcome::new(diff_add_element(*index, element.clone()))
    }
    fn inverse(&self, base: &PlySnapshot) -> Result<Vec<PlyMutation>, semio_framework_value::ValueError> {
        let Self { element, .. } = self;
        Ok({ vec![PlyMutation::RemoveElement(remove_element::RemoveElement { name: element.name.clone() })] })
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Add element", "Element hinzufügen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
