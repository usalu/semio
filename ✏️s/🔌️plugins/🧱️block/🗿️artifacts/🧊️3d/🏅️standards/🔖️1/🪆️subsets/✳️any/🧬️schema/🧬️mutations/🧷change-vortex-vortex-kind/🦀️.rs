//! 🧷 Block3d mutation — `ChangeVortexVortexKind`: a vortex's `vortexKind` catalog reference (rebind).

use crate::Block3dSnapshot;
use crate::standards::v1::subsets::any::schema::diff::Block3dDiff;
use crate::standards::v1::subsets::any::schema::mutations::Block3dMutation;

//#region 🔖️Mutation
/// 🧷 `change-vortex-vortex-kind` payload.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "change-vortex-vortex-kind")]
pub struct ChangeVortexVortexKind {
    pub id: String,
    pub new_vortex_kind: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_vortex_vortex_kind(id: String, new_vortex_kind: String) -> Block3dMutation {
    Block3dMutation::ChangeVortexVortexKind(ChangeVortexVortexKind { id, new_vortex_kind })
}

impl protocol::MutationKind<Block3dSnapshot, Block3dMutation> for ChangeVortexVortexKind {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "vortex", kind: "change-vortex-vortex-kind", record: "ChangedVortexVortexKind" };

    fn diff(&self, base: &Block3dSnapshot) -> protocol::MutationOutcome<Block3dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Block3dSnapshot) -> Result<Vec<Block3dMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change vortex \"{}\" vortex kind to \"{}\"", self.id, self.new_vortex_kind), &format!("Wirbelart von Wirbel \"{}\" auf \"{}\" ändern", self.id, self.new_vortex_kind))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
