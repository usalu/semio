//! 🌍️ Energy model mutation — `ChangeSiteLatitude`: Sets the site's geographic latitude, north positive.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🌍️ `change-site-latitude` payload. Sets the site's geographic latitude, north positive.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-site-latitude")]
pub struct ChangeSiteLatitude {
    pub new_latitude_deg: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_site_latitude<Op: From<ChangeSiteLatitude>>(new_latitude_deg: f64) -> Op {
    ChangeSiteLatitude { new_latitude_deg }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for ChangeSiteLatitude
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<ChangeSiteLatitude>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "site", kind: "change-site-latitude", record: "ChangedSiteLatitude" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change site latitude to {}°", self.new_latitude_deg), &format!("Breitengrad des Standorts auf {}° ändern", self.new_latitude_deg))
    }
}
//#endregion 🔖️Mutation
