//! attraction allowance.
//! 💔 Puzzle3d mutation — `DisconnectKindCompatibility`: revokes one vortex-kind-id pair's
use crate::connect_kind_compatibility::mutation::ConnectKindCompatibility;
use semio_s_artifact_puzzle_3d_diff::Puzzle3dDiff;
use semio_s_artifact_puzzle_3d_model::snapshot::Puzzle3dSnapshot;

//#region 🔖️Mutation
/// 💔 `disconnect-kind-compatibility` payload.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "testing"), derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(any(test, feature = "testing"), serde(rename_all = "camelCase"))]
#[dsl(keyword = "disconnect-kind-compatibility")]
pub struct DisconnectKindCompatibility {
    pub source: String,
    pub target: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn disconnect_kind_compatibility<Op: From<DisconnectKindCompatibility>>(source: String, target: String) -> Op {
    Op::from(DisconnectKindCompatibility { source, target })
}

impl<Op> protocol::MutationKind<Puzzle3dSnapshot, Op> for DisconnectKindCompatibility
where
    Op: protocol::Mutation<Puzzle3dSnapshot, Diff = Puzzle3dDiff> + From<DisconnectKindCompatibility> + From<ConnectKindCompatibility>,
 {

    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "disconnect", entity: "kind-compatibility", kind: "disconnect-kind-compatibility", record: "DisconnectedKindCompatibility" };



    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Disconnect kind compatibility \"{}\" -> \"{}\"", self.source, self.target), &format!("Artkompatibilität \"{}\" -> \"{}\" aufheben", self.source, self.target))
    }
    fn target(&self) -> Vec<String> {
        vec![self.source.clone(), self.target.clone()]
    }

}
impl<Op> protocol::ColdMutationKind<Puzzle3dSnapshot, Op> for DisconnectKindCompatibility
where
    Op: protocol::ColdMutation<Puzzle3dSnapshot, ReferenceDiff = Puzzle3dDiff> + From<DisconnectKindCompatibility> + From<ConnectKindCompatibility>,
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
