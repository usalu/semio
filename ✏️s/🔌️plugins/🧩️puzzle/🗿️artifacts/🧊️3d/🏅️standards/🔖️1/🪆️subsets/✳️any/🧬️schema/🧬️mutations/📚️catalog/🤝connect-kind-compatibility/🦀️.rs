//! 🤝 Puzzle3d mutation — `ConnectKindCompatibility`: allows one vortex-kind-id pair to attract.
use crate::disconnect_kind_compatibility::mutation::DisconnectKindCompatibility;
use semio_s_artifact_puzzle_3d_diff::Puzzle3dDiff;
use {semio_s_artifact_puzzle_3d_model::Puzzle3dCompatSpecificity, semio_s_artifact_puzzle_3d_model::snapshot::Puzzle3dSnapshot};

//#region 🔖️Mutation
/// 🤝 `connect-kind-compatibility` payload. A duplicate `(source, target)` pair is a no-op.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "testing"), derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(any(test, feature = "testing"), serde(rename_all = "camelCase"))]
#[dsl(keyword = "connect-kind-compatibility")]
pub struct ConnectKindCompatibility {
    pub source: String,
    pub target: String,
    pub bidirectional: bool,
    pub important: bool,
    pub specificity: Puzzle3dCompatSpecificity,
    pub index: Option<usize>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn connect_kind_compatibility<Op: From<ConnectKindCompatibility>>(source: String, target: String, bidirectional: bool, important: bool, specificity: Puzzle3dCompatSpecificity, index: Option<usize>) -> Op {
    Op::from(ConnectKindCompatibility { source, target, bidirectional, important, specificity, index })
}

impl<Op> protocol::MutationKind<Puzzle3dSnapshot, Op> for ConnectKindCompatibility
where
    Op: protocol::Mutation<Puzzle3dSnapshot, Diff = Puzzle3dDiff> + From<ConnectKindCompatibility> + From<DisconnectKindCompatibility>,
 {

    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "connect", entity: "kind-compatibility", kind: "connect-kind-compatibility", record: "ConnectedKindCompatibility" };



    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Connect kind compatibility \"{}\" -> \"{}\"", self.source, self.target), &format!("Artkompatibilität \"{}\" -> \"{}\" herstellen", self.source, self.target))
    }
    fn target(&self) -> Vec<String> {
        vec![self.source.clone(), self.target.clone()]
    }

}
impl<Op> protocol::ColdMutationKind<Puzzle3dSnapshot, Op> for ConnectKindCompatibility
where
    Op: protocol::ColdMutation<Puzzle3dSnapshot, ReferenceDiff = Puzzle3dDiff> + From<ConnectKindCompatibility> + From<DisconnectKindCompatibility>,
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
