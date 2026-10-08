//! 🚮️ `remove-element` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct RemoveElement {
    pub(crate) name: String,
}

impl protocol::MutationKind<PlySnapshot, PlyMutation> for RemoveElement {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "element", kind: "remove-element", record: "RemoveElement" };

    fn diff(&self, base: &PlySnapshot) -> protocol::MutationOutcome<<PlyMutation as Mutation<PlySnapshot>>::Diff> {
        let Self { name } = self;
        protocol::MutationOutcome::new(diff_remove_element(name))
    }
    fn inverse(&self, base: &PlySnapshot) -> Result<Vec<PlyMutation>, semio_framework_value::ValueError> {
        let Self { name } = self;
        Ok({
            match base.elements.iter().position(|e| &e.name == name) {
                Some(idx) => vec![PlyMutation::AddElement(add_element::AddElement { index: idx, element: base.elements[idx].clone() })],
                None => Vec::new(),
            }
        })
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Remove element", "Element entfernen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
