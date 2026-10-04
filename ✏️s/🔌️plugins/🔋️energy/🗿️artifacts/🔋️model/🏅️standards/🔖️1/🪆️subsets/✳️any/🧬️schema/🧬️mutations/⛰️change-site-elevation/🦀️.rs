//! ⛰️ Energy model mutation — `ChangeSiteElevation`: Sets the site's elevation above sea level (EnergyPlus `Site:Location`: -300 m to below 8900 m).

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// ⛰️ `change-site-elevation` payload. Sets the site's elevation above sea level (EnergyPlus `Site:Location`: -300 m to below 8900 m).
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-site-elevation")]
pub struct ChangeSiteElevation {
    pub new_elevation_m: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_site_elevation(new_elevation_m: f64) -> EnergyModelMutation {
    EnergyModelMutation::ChangeSiteElevation(ChangeSiteElevation { new_elevation_m })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeSiteElevation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "site", kind: "change-site-elevation", record: "ChangedSiteElevation" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Result<Vec<EnergyModelMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change site elevation to {} m", self.new_elevation_m), &format!("Höhe des Standorts auf {} m ändern", self.new_elevation_m))
    }
}
//#endregion 🔖️Mutation
