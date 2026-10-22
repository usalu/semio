//! ➖️ `remove-element` — authored as its own mutation leaf. It builds its own sparse diff and concrete
//! inverse from its payload and reads of `base`.

#[allow(unused_imports)]
use crate::schema::snapshot::NodePath;
#[allow(unused_imports)]
use crate::schema::diff::SvgChildrenDiff;
#[allow(unused_imports)]
use crate::schema::diff::SvgDiff;
#[allow(unused_imports)]
use crate::schema::diff::SvgElementDiff;
#[allow(unused_imports)]
use crate::schema::snapshot::SvgNode;
#[allow(unused_imports)]
use crate::schema::diff::SvgNodeDiff;
#[allow(unused_imports)]
use crate::SvgSnapshot;
#[allow(unused_imports)]
use crate::schema::diff::diff_at_path;
#[allow(unused_imports)]
use crate::standards::v1_1::subsets::tiny::schema::mutations::insert_tiny_element;
#[allow(unused_imports)]
use crate::schema::snapshot::node_at;
#[allow(unused_imports)]
use crate::standards::v1_1::subsets::tiny::schema::mutations::restore_non_tiny;
#[allow(unused_imports)]
use crate::standards::v1_1::subsets::tiny::schema::mutations::support::subtree_profile_violation;

//#region 🔖️Payload
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct RemoveElement {
    pub(crate) parent: NodePath,
    pub(crate) index: usize,
}

impl<Op> protocol::MutationKind<SvgSnapshot, Op> for RemoveElement
where
    Op: protocol::Mutation<SvgSnapshot, Diff = SvgDiff> + From<RemoveElement> + From<crate::standards::v1_1::subsets::tiny::schema::mutations::insert_tiny_element::InsertTinyElement> + From<crate::standards::v1_1::subsets::tiny::schema::mutations::restore_non_tiny::RestoreNonTiny>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "element", kind: "remove-element", record: "RemoveElement" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Remove element", "Element entfernen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}

impl<Op> protocol::ColdMutationKind<SvgSnapshot, Op> for RemoveElement
where
    Op: protocol::ColdMutation<SvgSnapshot, ReferenceDiff = SvgDiff> + From<RemoveElement> + From<crate::standards::v1_1::subsets::tiny::schema::mutations::insert_tiny_element::InsertTinyElement> + From<crate::standards::v1_1::subsets::tiny::schema::mutations::restore_non_tiny::RestoreNonTiny>,
{
    fn diff(&self, _base: &SvgSnapshot) -> protocol::MutationOutcome<SvgDiff> {
        let Self { parent, index } = self;
        {
            protocol::MutationOutcome::new(diff_at_path(parent, SvgNodeDiff::Element(SvgElementDiff { name: None, attributes: None, children: Some(SvgChildrenDiff { removed: vec![*index], modified: Vec::new(), added: Vec::new() }) })))
        }
    }
    fn inverse(&self, base: &SvgSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        let Self { parent, index } = self;
        Ok(match node_at(&base.doc, parent) {
            Ok(SvgNode::Element { children, .. }) => match children.get(*index) {
                Some(node) if subtree_profile_violation(node).is_some() => {
                    vec![Op::from(restore_non_tiny::RestoreNonTiny { elements: vec![restore_non_tiny::RestoredElement { parent: parent.clone(), index: *index, node: node.clone() }], attributes: Vec::new() })]
                }
                Some(node) => vec![Op::from(insert_tiny_element::InsertTinyElement { parent: parent.clone(), index: *index, node: node.clone() })],
                None => Vec::new(),
            },
            _ => Vec::new(),
        })
    }
}
//#endregion 🔖️Payload
