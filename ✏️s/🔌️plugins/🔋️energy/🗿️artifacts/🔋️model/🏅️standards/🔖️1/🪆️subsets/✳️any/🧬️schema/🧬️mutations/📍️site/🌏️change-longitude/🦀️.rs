//! 🌏️ Energy model mutation — `ChangeSiteLongitude`: Sets the site's geographic longitude, east positive.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🌏️ `change-site-longitude` payload. Sets the site's geographic longitude, east positive.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-site-longitude")]
pub struct ChangeSiteLongitude {
    pub new_longitude_deg: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_site_longitude<Op: From<ChangeSiteLongitude>>(new_longitude_deg: f64) -> Op {
    ChangeSiteLongitude { new_longitude_deg }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for ChangeSiteLongitude
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<ChangeSiteLongitude>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "site", kind: "change-site-longitude", record: "ChangedSiteLongitude" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change site longitude to {}°", self.new_longitude_deg), &format!("Längengrad des Standorts auf {}° ändern", self.new_longitude_deg))
    }
}
//#endregion 🔖️Mutation
