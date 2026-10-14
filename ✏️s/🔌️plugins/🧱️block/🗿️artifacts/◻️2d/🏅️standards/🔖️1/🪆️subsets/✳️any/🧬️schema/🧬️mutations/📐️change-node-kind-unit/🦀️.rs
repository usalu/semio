//! 📐️ Block2d mutation — `ChangeNodeKindUnit`: the node kind's optional `unit`.

use semio_s_artifact_block_2d_model::Block2dSnapshot;
use semio_s_artifact_block_2d_diff::Block2dDiff;

//#region 🔖️Mutation
/// 📐️ `change-node-kind-unit` payload.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "change-node-kind-unit")]
pub struct ChangeNodeKindUnit {
    pub new_unit: Option<String>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_node_kind_unit<Op: From<crate::change_node_kind_unit::ChangeNodeKindUnit>>(new_unit: Option<String>) -> Op {
    Op::from(ChangeNodeKindUnit { new_unit })
}

impl<Op> protocol::MutationKind<Block2dSnapshot, Op> for ChangeNodeKindUnit where Op: protocol::Mutation<Block2dSnapshot, Diff = semio_s_artifact_block_2d_diff::Block2dDiff> + From<crate::change_node_kind_unit::ChangeNodeKindUnit> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "node-kind", kind: "change-node-kind-unit", record: "ChangedNodeKindUnit" };

    fn diff(&self, base: &Block2dSnapshot) -> protocol::MutationOutcome<Block2dDiff> {
        crate::change_node_kind_unit::diff::diff(self, base)
    }
    fn inverse(&self, base: &Block2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::change_node_kind_unit::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change node kind unit to {:?}", self.new_unit), &format!("Knotenarteinheit auf {:?} ändern", self.new_unit))
    }
}
//#endregion 🔖️Mutation

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
