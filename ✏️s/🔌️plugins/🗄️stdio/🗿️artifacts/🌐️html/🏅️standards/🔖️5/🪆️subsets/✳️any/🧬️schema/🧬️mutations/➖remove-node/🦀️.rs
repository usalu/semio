//! ➖️ `remove-node` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;
use semio_s_artifact_stdio_html_snapshot::standards::v5::subsets::any::schema::snapshot::{NodePath, HtmlSnapshot, node_at, HtmlNode};
use semio_s_artifact_stdio_html_inferences::standards::v5::subsets::any::schema::diff::{diff_at_path, HtmlNodeDiff, HtmlElementDiff, HtmlChildrenDiff};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct RemoveNode {
    pub(crate) parent: NodePath,
    pub(crate) index: usize,
}

impl protocol::MutationKind<HtmlSnapshot, HtmlMutation> for RemoveNode {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "node", kind: "remove-node", record: "RemoveNode" };

    fn diff(&self, _base: &HtmlSnapshot) -> protocol::MutationOutcome<<HtmlMutation as Mutation<HtmlSnapshot>>::Diff> {
        let Self { parent, index } = self;
        protocol::MutationOutcome::new(diff_at_path(parent, HtmlNodeDiff::Element(HtmlElementDiff { name: None, attributes: None, children: Some(HtmlChildrenDiff { removed: vec![*index], modified: Vec::new(), added: Vec::new() }) })))
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
