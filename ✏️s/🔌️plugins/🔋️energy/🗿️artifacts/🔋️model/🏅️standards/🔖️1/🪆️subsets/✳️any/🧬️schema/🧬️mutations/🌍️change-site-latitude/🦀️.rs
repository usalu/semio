//! 🌍️ Energy model mutation — `ChangeSiteLatitude`: Sets the site's geographic latitude, north positive.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🌍️ `change-site-latitude` payload. Sets the site's geographic latitude, north positive.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-site-latitude")]
pub struct ChangeSiteLatitude {
    pub new_latitude_deg: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_site_latitude(new_latitude_deg: f64) -> EnergyModelMutation {
    EnergyModelMutation::ChangeSiteLatitude(ChangeSiteLatitude { new_latitude_deg })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeSiteLatitude {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "site", kind: "change-site-latitude", record: "ChangedSiteLatitude" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change site latitude to {}°", self.new_latitude_deg), &format!("Breitengrad des Standorts auf {}° ändern", self.new_latitude_deg))
    }
}
//#endregion 🔖️Mutation
