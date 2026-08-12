//! ⚙️ Remodel mutation — `UpdateGeoParams`: full-record replace of `ReconstructionParams.geo` (always
//! set wholesale from the palette form's flat field list — genuinely inseparable).
use crate::artifacts::remodel::diff::RemodelDiff;
use crate::artifacts::remodel::mutations::RemodelMutation;
use crate::artifacts::remodel::{GeoParams, RemodelSnapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// ⚙️ `update-geo-params` payload — full FINAL-state `GeoParams`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "update-geo-params")]
pub struct UpdateGeoParams {
    #[dsl(block)]
    pub params: GeoParams,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn update_geo_params(params: GeoParams) -> RemodelMutation {
    RemodelMutation::UpdateGeoParams(UpdateGeoParams { params })
}

impl protocol::MutationKind<RemodelSnapshot, RemodelMutation> for UpdateGeoParams {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "update", entity: "geo-params", kind: "update-geo-params", record: "UpdatedGeoParams" };

    fn diff(&self, base: &RemodelSnapshot) -> RemodelDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &RemodelSnapshot) -> Vec<RemodelMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        "Update geo params".to_string()
    }
}
//#endregion 🔖️Mutation
