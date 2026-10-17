//! 📍 Note mutation — `MoveBlock`: absolute reposition of a block.

use crate::{NoteDiff, NoteSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 📍 `move-block` payload — absolute reposition of a block.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "move-block")]
pub struct MoveBlock {
    pub id: String,
    pub new_x: f64,
    pub new_y: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn move_block<Op>(id: String, new_x: f64, new_y: f64) -> Op
where
    Op: From<crate::standards::v1::subsets::block::schema::mutations::move_block::MoveBlock>,
{
    Op::from(MoveBlock { id, new_x, new_y })
}

impl<Op> MutationKind<NoteSnapshot, Op> for MoveBlock
where
    Op: protocol::Mutation<NoteSnapshot, Diff = NoteDiff> + From<MoveBlock>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "move", entity: "block", kind: "move-block", record: "MovedBlock" };

    fn diff(&self, base: &NoteSnapshot) -> protocol::MutationOutcome<NoteDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &NoteSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Move block \"{}\"", self.id), &format!("Block \"{}\" verschieben", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
