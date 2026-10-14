//! 📍 Puzzle2d mutation — `MoveNode`: absolute reposition of a node's anchor point.

use semio_framework_value::paged::PagedUtf8;

use crate::standards::v1::subsets::any::schema::diff::Puzzle2dDiff;
use crate::Puzzle2dSnapshot;

#[path = "🎮️prepare/🦀️.rs"]
pub mod preparation;

//#region 🔖️Mutation
/// 📍 `move-node` payload — absolute FINAL-state position.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "move-node")]
pub struct MoveNode {
    pub id: PagedUtf8<{ usize::MAX }>,
    pub new_x: f64,
    pub new_y: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn move_node<Op: From<MoveNode>>(id: PagedUtf8<{ usize::MAX }>, new_x: f64, new_y: f64) -> Op {
    Op::from(MoveNode { id, new_x, new_y })
}

impl<Op> protocol::MutationKind<Puzzle2dSnapshot, Op> for MoveNode
where
    Op: protocol::Mutation<Puzzle2dSnapshot, Diff = Puzzle2dDiff> + From<MoveNode>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "move", entity: "node", kind: "move-node", record: "MovedNode" };

    fn diff(&self, base: &Puzzle2dSnapshot) -> protocol::MutationOutcome<Puzzle2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Puzzle2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Move node \"{}\"", self.id), &format!("Knoten \"{}\" verschieben", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.to_string_owner()]
    }
}
//#endregion 🔖️Mutation
