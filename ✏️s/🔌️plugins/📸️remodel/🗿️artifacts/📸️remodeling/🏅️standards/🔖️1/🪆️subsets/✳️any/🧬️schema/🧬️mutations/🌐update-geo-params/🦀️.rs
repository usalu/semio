//! ⚙️ Remodeling mutation — `UpdateGeoParams`: full-record replace of `ReconstructionParams.geo` (always
//! set wholesale from the palette form's flat field list — genuinely inseparable).

use crate::diff::RemodelingDiff;
use crate::mutations::RemodelingMutation;
use crate::{GeoParams, RemodelingSnapshot};
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// ⚙️ `update-geo-params` payload — full FINAL-state `GeoParams`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "update-geo-params")]
pub struct UpdateGeoParams {
    #[dsl(block)]
    pub params: GeoParams,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn update_geo_params(params: GeoParams) -> RemodelingMutation {
    RemodelingMutation::UpdateGeoParams(UpdateGeoParams { params })
}

impl protocol::MutationKind<RemodelingSnapshot, RemodelingMutation> for UpdateGeoParams {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "update", entity: "geo-params", kind: "update-geo-params", record: "UpdatedGeoParams" };

    fn diff(&self, base: &RemodelingSnapshot) -> protocol::MutationOutcome<RemodelingDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &RemodelingSnapshot) -> Vec<RemodelingMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        "Update geo params".to_string()
    }
}
//#endregion 🔖️Mutation
