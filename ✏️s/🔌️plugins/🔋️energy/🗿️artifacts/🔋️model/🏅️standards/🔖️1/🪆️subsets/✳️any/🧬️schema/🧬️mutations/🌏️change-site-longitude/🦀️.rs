//! 🌏️ Energy model mutation — `ChangeSiteLongitude`: Sets the site's geographic longitude, east positive.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🌏️ `change-site-longitude` payload. Sets the site's geographic longitude, east positive.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-site-longitude")]
pub struct ChangeSiteLongitude {
    pub new_longitude_deg: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_site_longitude(new_longitude_deg: f64) -> EnergyModelMutation {
    EnergyModelMutation::ChangeSiteLongitude(ChangeSiteLongitude { new_longitude_deg })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeSiteLongitude {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "site", kind: "change-site-longitude", record: "ChangedSiteLongitude" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change site longitude to {}°", self.new_longitude_deg), &format!("Längengrad des Standorts auf {}° ändern", self.new_longitude_deg))
    }
}
//#endregion 🔖️Mutation
