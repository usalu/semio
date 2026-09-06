//! 🪢️ Energy model mutation — `ConnectReferencedModel`: Creates the relationship between this energy model and the geometry model it was derived from, addressed by the target's `ArtifactRef` URI.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🪢️ `connect-referenced-model` payload. Creates the relationship between this energy model and the geometry model it was derived from, addressed by the target's `ArtifactRef` URI.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "connect-referenced-model")]
pub struct ConnectReferencedModel {
    pub target_uri: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn connect_referenced_model(target_uri: String) -> EnergyModelMutation {
    EnergyModelMutation::ConnectReferencedModel(ConnectReferencedModel { target_uri })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ConnectReferencedModel {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "connect", entity: "referenced-model", kind: "connect-referenced-model", record: "ConnectedReferencedModel" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Connect referenced model {}", self.target_uri)
    }

    fn target(&self) -> Vec<String> {
        vec![self.target_uri.clone()]
    }
}
//#endregion 🔖️Mutation
