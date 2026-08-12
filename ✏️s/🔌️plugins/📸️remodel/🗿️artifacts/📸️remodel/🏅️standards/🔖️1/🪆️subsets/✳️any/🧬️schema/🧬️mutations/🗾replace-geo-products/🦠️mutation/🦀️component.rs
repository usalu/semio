//! 🔁 Remodel mutation — `ReplaceGeoProducts`: whole-value swap of `ReconstructionResults.geo`, a large
//! structured sub-payload swapped wholesale by the reconstruction engine or a clear/reset command.
use crate::artifacts::remodel::diff::RemodelDiff;
use crate::artifacts::remodel::mutations::RemodelMutation;
use crate::artifacts::remodel::{RemodelSnapshot, GeoProducts};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🔁 `replace-geo-products` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "replace-geo-products")]
pub struct ReplaceGeoProducts {
    #[serde(default)]
    #[dsl(block)]
    pub geo: Option<GeoProducts>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn replace_geo_products(geo: Option<GeoProducts>) -> RemodelMutation {
    RemodelMutation::ReplaceGeoProducts(ReplaceGeoProducts { geo })
}

impl protocol::MutationKind<RemodelSnapshot, RemodelMutation> for ReplaceGeoProducts {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "geo-products", kind: "replace-geo-products", record: "ReplacedGeoProducts" };

    fn diff(&self, base: &RemodelSnapshot) -> RemodelDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &RemodelSnapshot) -> Vec<RemodelMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        "Replace geo-products".to_string()
    }
}
//#endregion 🔖️Mutation
