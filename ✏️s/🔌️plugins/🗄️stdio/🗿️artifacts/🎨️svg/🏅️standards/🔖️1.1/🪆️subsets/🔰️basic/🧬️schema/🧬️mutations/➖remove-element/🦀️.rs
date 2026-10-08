//! ➖️ `remove-element` — authored as its own mutation leaf. It builds its own sparse diff and concrete
//! inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct RemoveElement {
    pub(crate) parent: NodePath,
    pub(crate) index: usize,
}

impl protocol::MutationKind<SvgSnapshot, SvgBasicMutation> for RemoveElement {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "element", kind: "remove-element", record: "RemoveElement" };

    fn diff(&self, base: &SvgSnapshot) -> protocol::MutationOutcome<SvgDiff> {
        let Self { parent, index } = self;
        protocol::MutationOutcome::new(remove_child_diff(parent, *index))
    }
    fn inverse(&self, base: &SvgSnapshot) -> Result<Vec<SvgBasicMutation>, semio_framework_value::ValueError> {
        let Self { parent, index } = self;
        Ok(match node_at(&base.doc, parent) {
            Ok(SvgNode::Element { children, .. }) => match children.get(*index) {
                Some(node) => vec![SvgBasicMutation::InsertBasicElement(insert_basic_element::InsertBasicElement { parent: parent.clone(), index: *index, node: node.clone() })],
                None => Vec::new(),
            },
            _ => Vec::new(),
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
