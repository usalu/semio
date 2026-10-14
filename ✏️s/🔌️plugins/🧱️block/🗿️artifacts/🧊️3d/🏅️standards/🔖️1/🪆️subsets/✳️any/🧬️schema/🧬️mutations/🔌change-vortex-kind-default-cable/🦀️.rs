//! 🔌 Block3d mutation — `ChangeVortexKindDefaultCableKind`: a vortex-kind catalog row's `defaultCableKind`.

use semio_s_artifact_block_3d_model::Block3dSnapshot;
use semio_s_artifact_block_3d_diff::Block3dDiff;

//#region 🔖️Mutation
/// 🔌 `change-vortex-kind-default-cable-kind` payload.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "change-vortex-kind-default-cable-kind")]
pub struct ChangeVortexKindDefaultCableKind {
    pub id: String,
    pub new_default_cable_kind: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_vortex_kind_default_cable_kind<Op: From<crate::change_vortex_kind_default_cable_kind::ChangeVortexKindDefaultCableKind>>(id: String, new_default_cable_kind: String) -> Op {
    Op::from(ChangeVortexKindDefaultCableKind { id, new_default_cable_kind })
}

impl<Op> protocol::MutationKind<Block3dSnapshot, Op> for ChangeVortexKindDefaultCableKind where Op: protocol::Mutation<Block3dSnapshot, Diff = semio_s_artifact_block_3d_diff::Block3dDiff> + From<crate::change_vortex_kind_default_cable_kind::ChangeVortexKindDefaultCableKind> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "vortex-kind", kind: "change-vortex-kind-default-cable-kind", record: "ChangedVortexKindDefaultCableKind" };

    fn diff(&self, base: &Block3dSnapshot) -> protocol::MutationOutcome<Block3dDiff> {
        crate::change_vortex_kind_default_cable_kind::diff::diff(self, base)
    }
    fn inverse(&self, base: &Block3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::change_vortex_kind_default_cable_kind::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change vortex kind \"{}\" default cable kind to \"{}\"", self.id, self.new_default_cable_kind), &format!("Standardkabelart von Wirbelart \"{}\" auf \"{}\" ändern", self.id, self.new_default_cable_kind))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
