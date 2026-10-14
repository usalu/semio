//! ⛰️ Energy model mutation — `ChangeSiteElevation`: Sets the site's elevation above sea level (EnergyPlus `Site:Location`: -300 m to below 8900 m).

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// ⛰️ `change-site-elevation` payload. Sets the site's elevation above sea level (EnergyPlus `Site:Location`: -300 m to below 8900 m).
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-site-elevation")]
pub struct ChangeSiteElevation {
    pub new_elevation_m: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_site_elevation<Op: From<ChangeSiteElevation>>(new_elevation_m: f64) -> Op {
    ChangeSiteElevation { new_elevation_m }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for ChangeSiteElevation
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<ChangeSiteElevation>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "site", kind: "change-site-elevation", record: "ChangedSiteElevation" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change site elevation to {} m", self.new_elevation_m), &format!("Höhe des Standorts auf {} m ändern", self.new_elevation_m))
    }
}
//#endregion 🔖️Mutation
