//! ➕️ `insert-basic-element` — authored as its own mutation leaf. It builds its own sparse diff and concrete
//! inverse from its payload and reads of `base`.

#[allow(unused_imports)]
use crate::standards::v1_1::subsets::basic::schema::mutations::support::CODE_REJECTED;
#[allow(unused_imports)]
use crate::schema::snapshot::NodePath;
#[allow(unused_imports)]
use crate::schema::diff::SvgDiff;
#[allow(unused_imports)]
use crate::schema::snapshot::SvgNode;
#[allow(unused_imports)]
use crate::SvgSnapshot;
#[allow(unused_imports)]
use crate::standards::v1_1::subsets::basic::schema::mutations::support::insert_child_diff;
#[allow(unused_imports)]
use crate::standards::v1_1::subsets::basic::schema::mutations::remove_element;
#[allow(unused_imports)]
use crate::standards::v1_1::subsets::basic::schema::mutations::support::subtree_profile_violation;

//#region 🔖️Payload
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct InsertBasicElement {
    pub(crate) parent: NodePath,
    pub(crate) index: usize,
    pub(crate) node: SvgNode,
}

impl<Op> protocol::MutationKind<SvgSnapshot, Op> for InsertBasicElement
where
    Op: protocol::Mutation<SvgSnapshot, Diff = SvgDiff> + From<InsertBasicElement> + From<crate::standards::v1_1::subsets::basic::schema::mutations::remove_element::RemoveElement>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "basic-element", kind: "insert-basic-element", record: "InsertBasicElement" };

    fn diff(&self, _base: &SvgSnapshot) -> protocol::MutationOutcome<SvgDiff> {
        let Self { parent, index, node } = self;
        match subtree_profile_violation(node) {
            Some(message) => protocol::MutationOutcome::error(CODE_REJECTED, message, Vec::<String>::new()),
            None => protocol::MutationOutcome::new(insert_child_diff(parent, *index, node)),
        }
    }
    fn inverse(&self, _base: &SvgSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        let Self { parent, index, .. } = self;
        Ok(vec![Op::from(remove_element::RemoveElement { parent: parent.clone(), index: *index })])
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Insert basic element", "Basic-Element einfügen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
