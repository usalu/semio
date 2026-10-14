//! 🪢️ Energy model mutation — `ConnectReferencedModel`: Creates the relationship between this energy model and the geometry model it was derived from, addressed by the target's `ArtifactRef` identity.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🪢️ `connect-referenced-model` payload. Creates the relationship between this energy model and the geometry model it was derived from, addressed by the target's `ArtifactRef` identity.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "connect-referenced-model")]
pub struct ConnectReferencedModel {
    pub target: semio_framework_artifact_reference::ArtifactRef,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn connect_referenced_model(target: semio_framework_artifact_reference::ArtifactRef) -> EnergyModelMutation {
    EnergyModelMutation::ConnectReferencedModel(ConnectReferencedModel { target })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ConnectReferencedModel {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "connect", entity: "referenced-model", kind: "connect-referenced-model", record: "ConnectedReferencedModel" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Result<Vec<EnergyModelMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Connect referenced model {}", self.target.artifact_id), &format!("Referenziertes Modell {} verbinden", self.target.artifact_id))
    }

    fn target(&self) -> Vec<String> {
        vec![self.target.artifact_id.clone()]
    }
}
//#endregion 🔖️Mutation
