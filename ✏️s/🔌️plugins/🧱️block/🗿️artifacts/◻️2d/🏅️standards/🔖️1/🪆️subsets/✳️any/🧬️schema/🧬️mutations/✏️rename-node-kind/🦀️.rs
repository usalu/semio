//! ✏️ Block2d mutation — `RenameNodeKind`: the node kind's identity `name` field.

use crate::Block2dSnapshot;
use crate::standards::v1::subsets::any::schema::diff::Block2dDiff;
use crate::standards::v1::subsets::any::schema::mutations::Block2dMutation;

//#region 🔖️Mutation
/// ✏️ `rename-node-kind` payload.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "rename-node-kind")]
pub struct RenameNodeKind {
    pub new_name: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn rename_node_kind(new_name: String) -> Block2dMutation {
    Block2dMutation::RenameNodeKind(RenameNodeKind { new_name })
}

impl protocol::MutationKind<Block2dSnapshot, Block2dMutation> for RenameNodeKind {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "rename", entity: "node-kind", kind: "rename-node-kind", record: "RenamedNodeKind" };

    fn diff(&self, base: &Block2dSnapshot) -> protocol::MutationOutcome<Block2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Block2dSnapshot) -> Result<Vec<Block2dMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Rename node kind to \"{}\"", self.new_name), &format!("Knotenart in \"{}\" umbenennen", self.new_name))
    }
}
//#endregion 🔖️Mutation
