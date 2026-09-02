//! 🔁 Remodeling mutation — `ReplaceGeoProducts`: whole-value swap of `ReconstructionResults.geo`, a large
//! structured sub-payload swapped wholesale by the reconstruction engine or a clear/reset command.

use crate::artifacts::remodeling::{GeoProducts, RemodelingSnapshot};
use crate::artifacts::remodeling::diff::RemodelingDiff;
use crate::artifacts::remodeling::mutations::RemodelingMutation;
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Mutation
/// 🔁 `replace-geo-products` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "replace-geo-products")]
pub struct ReplaceGeoProducts {
    #[value(default)]
    #[serde(default)]
    #[dsl(block)]
    pub geo: Option<GeoProducts>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn replace_geo_products(geo: Option<GeoProducts>) -> RemodelingMutation {
    RemodelingMutation::ReplaceGeoProducts(ReplaceGeoProducts { geo })
}

impl protocol::MutationKind<RemodelingSnapshot, RemodelingMutation> for ReplaceGeoProducts {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "geo-products", kind: "replace-geo-products", record: "ReplacedGeoProducts" };

    async fn diff(&self, base: &RemodelingSnapshot) -> protocol::MutationOutcome<RemodelingDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &RemodelingSnapshot) -> Vec<RemodelingMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        "Replace geo-products".to_string()
    }
}
//#endregion 🔖️Mutation
