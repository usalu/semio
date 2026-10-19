//! 🔝️ Energy model mutation — `ChangeSiteNorthAxis`: Sets the angle from true north to the building's north axis, clockwise.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🔝️ `change-site-north-axis` payload. Sets the angle from true north to the building's north axis, clockwise.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-site-north-axis")]
pub struct ChangeSiteNorthAxis {
    pub new_north_axis_deg: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_site_north_axis<Op: From<ChangeSiteNorthAxis>>(new_north_axis_deg: f64) -> Op {
    ChangeSiteNorthAxis { new_north_axis_deg }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for ChangeSiteNorthAxis
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<ChangeSiteNorthAxis>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "site", kind: "change-site-north-axis", record: "ChangedSiteNorthAxis" };





    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change building north axis to {}°", self.new_north_axis_deg), &format!("Nordachse des Gebäudes auf {}° ändern", self.new_north_axis_deg))
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for ChangeSiteNorthAxis
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<ChangeSiteNorthAxis>,
{
    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Mutation
