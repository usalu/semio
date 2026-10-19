//! cascade — any attraction whose `attracting`/`attracted` referenced this vortex is severed too).
//! ➖ Puzzle3d mutation — `RemoveObjectVortex`: detaches a rim vortex from an object (captures
use {crate::add_object_vortex::mutation::AddObjectVortex, crate::connect_vortices::mutation::ConnectVortices};
use semio_s_artifact_puzzle_3d_diff::Puzzle3dDiff;
use semio_s_artifact_puzzle_3d_model::snapshot::Puzzle3dSnapshot;

//#region 🔖️Mutation
/// ➖ `remove-object-vortex` payload.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "testing"), derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(any(test, feature = "testing"), serde(rename_all = "camelCase"))]
#[dsl(keyword = "remove-object-vortex")]
pub struct RemoveObjectVortex {
    pub object_id: String,
    pub vortex_id: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn remove_object_vortex<Op: From<RemoveObjectVortex>>(object_id: String, vortex_id: String) -> Op {
    Op::from(RemoveObjectVortex { object_id, vortex_id })
}

impl<Op> protocol::MutationKind<Puzzle3dSnapshot, Op> for RemoveObjectVortex
where
    Op: protocol::Mutation<Puzzle3dSnapshot, Diff = Puzzle3dDiff> + From<RemoveObjectVortex> + From<AddObjectVortex> + From<ConnectVortices>,
 {

    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "object-vortex", kind: "remove-object-vortex", record: "RemovedObjectVortex" };



    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Remove vortex \"{}\" from object \"{}\"", self.vortex_id, self.object_id), &format!("Wirbel \"{}\" aus Objekt \"{}\" entfernen", self.vortex_id, self.object_id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.object_id.clone(), self.vortex_id.clone()]
    }

}
impl<Op> protocol::ColdMutationKind<Puzzle3dSnapshot, Op> for RemoveObjectVortex
where
    Op: protocol::ColdMutation<Puzzle3dSnapshot, ReferenceDiff = Puzzle3dDiff> + From<RemoveObjectVortex> + From<AddObjectVortex> + From<ConnectVortices>,
 {
fn diff(&self, base: &Puzzle3dSnapshot) -> protocol::MutationOutcome<Puzzle3dDiff> {
        super::diff::diff(self, base)
    }
fn inverse(&self, base: &Puzzle3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Mutation
