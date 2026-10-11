//! 🤝 Puzzle3d mutation — `ConnectKindCompatibility`: allows one vortex-kind-id pair to attract.
use crate::standards::v1::subsets::any::schema::diff::Puzzle3dDiff;
use crate::standards::v1::subsets::any::schema::mutations::Puzzle3dMutation;
use crate::{Puzzle3dCompatSpecificity, Puzzle3dSnapshot};

//#region 🔖️Mutation
/// 🤝 `connect-kind-compatibility` payload. A duplicate `(source, target)` pair is a no-op.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
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
pub fn connect_kind_compatibility(source: String, target: String, bidirectional: bool, important: bool, specificity: Puzzle3dCompatSpecificity, index: Option<usize>) -> Puzzle3dMutation {
    Puzzle3dMutation::ConnectKindCompatibility(ConnectKindCompatibility { source, target, bidirectional, important, specificity, index })
}

impl protocol::MutationKind<Puzzle3dSnapshot, Puzzle3dMutation> for ConnectKindCompatibility {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "connect", entity: "kind-compatibility", kind: "connect-kind-compatibility", record: "ConnectedKindCompatibility" };

    fn diff(&self, base: &Puzzle3dSnapshot) -> protocol::MutationOutcome<Puzzle3dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Puzzle3dSnapshot) -> Result<Vec<Puzzle3dMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Connect kind compatibility \"{}\" -> \"{}\"", self.source, self.target), &format!("Artkompatibilität \"{}\" -> \"{}\" herstellen", self.source, self.target))
    }
    fn target(&self) -> Vec<String> {
        vec![self.source.clone(), self.target.clone()]
    }
}
//#endregion 🔖️Mutation
