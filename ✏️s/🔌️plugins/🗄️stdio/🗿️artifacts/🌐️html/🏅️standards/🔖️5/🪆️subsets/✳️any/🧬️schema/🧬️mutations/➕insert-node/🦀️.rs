//! ➕️ `insert-node` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct InsertNode {
    pub(crate) parent: NodePath,
    pub(crate) index: usize,
    pub(crate) node: HtmlNode,
}

impl protocol::MutationKind<HtmlSnapshot, HtmlMutation> for InsertNode {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "node", kind: "insert-node", record: "InsertNode" };

    fn diff(&self, base: &HtmlSnapshot) -> protocol::MutationOutcome<<HtmlMutation as Mutation<HtmlSnapshot>>::Diff> {
        let Self { parent, index, node } = self;
        protocol::MutationOutcome::new(diff_at_path(
            parent,
            HtmlNodeDiff::Element(HtmlElementDiff { name: None, attributes: None, children: Some(HtmlChildrenDiff { removed: Vec::new(), modified: Vec::new(), added: vec![HtmlChildAdded { index: *index, item: node.clone() }] }) }),
        ))
    }
    fn inverse(&self, base: &HtmlSnapshot) -> Result<Vec<HtmlMutation>, semio_framework_value::ValueError> {
        let Self { parent, index, .. } = self;
        Ok(vec![HtmlMutation::RemoveNode(remove_node::RemoveNode { parent: parent.clone(), index: *index })])
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Insert node", "Knoten einfügen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
