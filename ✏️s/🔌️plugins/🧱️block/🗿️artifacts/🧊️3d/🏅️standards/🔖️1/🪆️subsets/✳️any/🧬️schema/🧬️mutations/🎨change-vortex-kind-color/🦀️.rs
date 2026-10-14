//! 🎨 Block3d mutation — `ChangeVortexKindColor`: a vortex-kind catalog row's `color`.

use semio_s_artifact_block_3d_model::Block3dSnapshot;
use semio_s_artifact_block_3d_diff::Block3dDiff;

//#region 🔖️Mutation
/// 🎨 `change-vortex-kind-color` payload.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "change-vortex-kind-color")]
pub struct ChangeVortexKindColor {
    pub id: String,
    pub new_color: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_vortex_kind_color<Op: From<crate::change_vortex_kind_color::ChangeVortexKindColor>>(id: String, new_color: String) -> Op {
    Op::from(ChangeVortexKindColor { id, new_color })
}

impl<Op> protocol::MutationKind<Block3dSnapshot, Op> for ChangeVortexKindColor where Op: protocol::Mutation<Block3dSnapshot, Diff = semio_s_artifact_block_3d_diff::Block3dDiff> + From<crate::change_vortex_kind_color::ChangeVortexKindColor> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "vortex-kind", kind: "change-vortex-kind-color", record: "ChangedVortexKindColor" };

    fn diff(&self, base: &Block3dSnapshot) -> protocol::MutationOutcome<Block3dDiff> {
        crate::change_vortex_kind_color::diff::diff(self, base)
    }
    fn inverse(&self, base: &Block3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::change_vortex_kind_color::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change vortex kind \"{}\" color to \"{}\"", self.id, self.new_color), &format!("Farbe von Wirbelart \"{}\" auf \"{}\" ändern", self.id, self.new_color))
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
