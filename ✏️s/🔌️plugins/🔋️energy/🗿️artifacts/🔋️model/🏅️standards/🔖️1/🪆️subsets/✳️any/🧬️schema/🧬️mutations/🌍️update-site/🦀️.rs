//! 🌍️ Energy model mutation — `UpdateSite`: Sets the whole inseparable site facet — latitude, longitude, elevation, time zone and north axis are validated and consumed together by the solar geometry, so none of them is meaningfully set on its own.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🌍️ `update-site` payload. Sets the whole inseparable site facet — latitude, longitude, elevation, time zone and north axis are validated and consumed together by the solar geometry, so none of them is meaningfully set on its own.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "update-site")]
pub struct UpdateSite {
    pub latitude_deg: f64,
    pub longitude_deg: f64,
    pub elevation_m: f64,
    pub time_zone_hours: f64,
    pub north_axis_deg: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn update_site(latitude_deg: f64, longitude_deg: f64, elevation_m: f64, time_zone_hours: f64, north_axis_deg: f64) -> EnergyModelMutation {
    EnergyModelMutation::UpdateSite(UpdateSite { latitude_deg, longitude_deg, elevation_m, time_zone_hours, north_axis_deg })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for UpdateSite {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "update", entity: "site", kind: "update-site", record: "UpdatedSite" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Update site to {:.4}, {:.4}", self.latitude_deg, self.longitude_deg)
    }
}
//#endregion 🔖️Mutation
