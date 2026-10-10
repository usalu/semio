//! ➕️ `insert-tiny-element` — authored as its own mutation leaf. It builds its own sparse diff and concrete
//! inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct InsertTinyElement {
    pub(crate) parent: NodePath,
    pub(crate) index: usize,
    pub(crate) node: SvgNode,
}

impl protocol::MutationKind<SvgSnapshot, SvgTinyMutation> for InsertTinyElement {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "tiny-element", kind: "insert-tiny-element", record: "InsertTinyElement" };

    fn diff(&self, base: &SvgSnapshot) -> protocol::MutationOutcome<SvgDiff> {
        let Self { parent, index, node } = self;
        match subtree_profile_violation(node) {
            Some(message) => protocol::MutationOutcome::error(CODE_REJECTED, message, Vec::<String>::new()),
            None => protocol::MutationOutcome::new(diff_at_path(
                parent,
                SvgNodeDiff::Element(SvgElementDiff { name: None, attributes: None, children: Some(SvgChildrenDiff { removed: Vec::new(), modified: Vec::new(), added: vec![SvgChildAdded { index: *index, item: node.clone() }] }) }),
            )),
        }
    }
    fn inverse(&self, base: &SvgSnapshot) -> Result<Vec<SvgTinyMutation>, semio_framework_value::ValueError> {
        let Self { parent, index, .. } = self;
        Ok(vec![SvgTinyMutation::RemoveElement(remove_element::RemoveElement { parent: parent.clone(), index: *index })])
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Insert tiny element", "Tiny-Element einfügen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
