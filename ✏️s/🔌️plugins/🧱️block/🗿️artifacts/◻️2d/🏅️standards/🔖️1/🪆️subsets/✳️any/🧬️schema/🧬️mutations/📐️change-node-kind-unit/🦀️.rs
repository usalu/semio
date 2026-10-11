//! 📐️ Block2d mutation — `ChangeNodeKindUnit`: the node kind's optional `unit`.

use crate::Block2dSnapshot;
use crate::standards::v1::subsets::any::schema::diff::Block2dDiff;
use crate::standards::v1::subsets::any::schema::mutations::Block2dMutation;

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
pub fn change_node_kind_unit(new_unit: Option<String>) -> Block2dMutation {
    Block2dMutation::ChangeNodeKindUnit(ChangeNodeKindUnit { new_unit })
}

impl protocol::MutationKind<Block2dSnapshot, Block2dMutation> for ChangeNodeKindUnit {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "node-kind", kind: "change-node-kind-unit", record: "ChangedNodeKindUnit" };

    fn diff(&self, base: &Block2dSnapshot) -> protocol::MutationOutcome<Block2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Block2dSnapshot) -> Result<Vec<Block2dMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change node kind unit to {:?}", self.new_unit), &format!("Knotenarteinheit auf {:?} ändern", self.new_unit))
    }
}
//#endregion 🔖️Mutation
