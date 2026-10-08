//! ➖️ `remove-node` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct RemoveNode {
    pub(crate) parent: NodePath,
    pub(crate) index: usize,
}

impl protocol::MutationKind<HtmlSnapshot, HtmlMutation> for RemoveNode {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "node", kind: "remove-node", record: "RemoveNode" };

    fn diff(&self, base: &HtmlSnapshot) -> protocol::MutationOutcome<<HtmlMutation as Mutation<HtmlSnapshot>>::Diff> {
        let Self { parent, index } = self;
        protocol::MutationOutcome::new({ diff_at_path(parent, HtmlNodeDiff::Element(HtmlElementDiff { name: None, attributes: None, children: Some(HtmlChildrenDiff { removed: vec![*index], modified: Vec::new(), added: Vec::new() }) })) })
    }
    fn inverse(&self, base: &HtmlSnapshot) -> Result<Vec<HtmlMutation>, semio_framework_value::ValueError> {
        let Self { parent, index } = self;
        Ok(match node_at(base, parent) {
            Ok(HtmlNode::Element { children, .. }) => match children.get(*index) {
                Some(node) => vec![HtmlMutation::InsertNode(insert_node::InsertNode { parent: parent.clone(), index: *index, node: node.clone() })],
                None => Vec::new(),
            },
            _ => Vec::new(),
        })
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Remove node", "Knoten entfernen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
