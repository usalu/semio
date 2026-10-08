//! ➕️ `insert-basic-element` — authored as its own mutation leaf. It builds its own sparse diff and concrete
//! inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct InsertBasicElement {
    pub(crate) parent: NodePath,
    pub(crate) index: usize,
    pub(crate) node: SvgNode,
}

impl protocol::MutationKind<SvgSnapshot, SvgBasicMutation> for InsertBasicElement {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "basic-element", kind: "insert-basic-element", record: "InsertBasicElement" };

    fn diff(&self, base: &SvgSnapshot) -> protocol::MutationOutcome<SvgDiff> {
        let Self { parent, index, node } = self;
        match subtree_profile_violation(node) {
            Some(message) => protocol::MutationOutcome::error(CODE_REJECTED, message, Vec::<String>::new()),
            None => protocol::MutationOutcome::new(insert_child_diff(parent, *index, node)),
        }
    }
    fn inverse(&self, base: &SvgSnapshot) -> Result<Vec<SvgBasicMutation>, semio_framework_value::ValueError> {
        let Self { parent, index, .. } = self;
        Ok(vec![SvgBasicMutation::RemoveElement(remove_element::RemoveElement { parent: parent.clone(), index: *index })])
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Insert basic element", "Basic-Element einfügen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
