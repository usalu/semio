//! 📎️ `insert-clip-path-shape` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

#[allow(unused_imports)]
use crate::standards::v1_1::subsets::basic::schema::mutations::support::CODE_REJECTED;
#[allow(unused_imports)]
use crate::schema::diff::SvgDiff;
#[allow(unused_imports)]
use crate::schema::snapshot::SvgNode;
#[allow(unused_imports)]
use crate::SvgSnapshot;
#[allow(unused_imports)]
use crate::standards::v1_1::subsets::basic::schema::mutations::support::carries_text;
#[allow(unused_imports)]
use crate::standards::v1_1::subsets::basic::schema::mutations::support::insert_child_diff;
#[allow(unused_imports)]
use crate::standards::v1_1::subsets::basic::schema::mutations::support::path_of_id;
#[allow(unused_imports)]
use crate::standards::v1_1::subsets::basic::schema::mutations::remove_element;
#[allow(unused_imports)]
use crate::standards::v1_1::subsets::basic::schema::mutations::support::resolve_clip_path;
#[allow(unused_imports)]
use crate::standards::v1_1::subsets::basic::schema::mutations::support::subtree_profile_violation;

//#region 🔖️Payload
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct InsertClipPathShape {
    pub clip_path_id: String,
    pub index: usize,
    pub node: SvgNode,
}

impl<Op> protocol::MutationKind<SvgSnapshot, Op> for InsertClipPathShape
where
    Op: protocol::Mutation<SvgSnapshot, Diff = SvgDiff> + From<InsertClipPathShape> + From<crate::standards::v1_1::subsets::basic::schema::mutations::remove_element::RemoveElement>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "clip-path-shape", kind: "insert-clip-path-shape", record: "InsertClipPathShape" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Insert clip path shape", "Form des Beschneidungspfads einfügen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}

impl<Op> protocol::ColdMutationKind<SvgSnapshot, Op> for InsertClipPathShape
where
    Op: protocol::ColdMutation<SvgSnapshot, ReferenceDiff = SvgDiff> + From<InsertClipPathShape> + From<crate::standards::v1_1::subsets::basic::schema::mutations::remove_element::RemoveElement>,
{
    fn diff(&self, base: &SvgSnapshot) -> protocol::MutationOutcome<SvgDiff> {
        let Self { clip_path_id, index, node } = self;
        {
            if carries_text(node) {
                return protocol::MutationOutcome::error(CODE_REJECTED, "the inserted shape carries a text element -- SVG Basic 1.1 forbids clipping to text".to_string(), Vec::<String>::new());
            }
            if let Some(message) = subtree_profile_violation(node) {
                return protocol::MutationOutcome::error(CODE_REJECTED, message, Vec::<String>::new());
            }
            match resolve_clip_path(base, clip_path_id) {
                Ok(target) => protocol::MutationOutcome::new(insert_child_diff(&target, *index, node)),
                Err(message) => protocol::MutationOutcome::error(CODE_REJECTED, message, Vec::<String>::new()),
            }
        }
    }
    fn inverse(&self, base: &SvgSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        let Self { clip_path_id, index, .. } = self;
        Ok(match path_of_id(base, clip_path_id) {
            Some(target) => vec![Op::from(remove_element::RemoveElement { parent: target, index: *index })],
            None => Vec::new(),
        })
    }
}
//#endregion 🔖️Payload
