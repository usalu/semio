//! 🕰️ Energy model mutation — `ChangeSiteTimeZone`: Sets the site's standard time zone as an offset from UTC.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🕰️ `change-site-time-zone` payload. Sets the site's standard time zone as an offset from UTC.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-site-time-zone")]
pub struct ChangeSiteTimeZone {
    pub new_time_zone_hours: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_site_time_zone<Op: From<ChangeSiteTimeZone>>(new_time_zone_hours: f64) -> Op {
    ChangeSiteTimeZone { new_time_zone_hours }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for ChangeSiteTimeZone
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<ChangeSiteTimeZone>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "site", kind: "change-site-time-zone", record: "ChangedSiteTimeZone" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change site time zone to UTC{:+}", self.new_time_zone_hours), &format!("Zeitzone des Standorts auf UTC{:+} ändern", self.new_time_zone_hours))
    }
}
//#endregion 🔖️Mutation
