//! 🔢 SHome mutation — `ChangeCatalogGeneration`: pins the catalog-generation counter that forces
//! a re-materialize of the studio list after a create/import/delete side-effect on the catalog
//! port. Single root-scalar setter — no `id` addressing (whole-artifact scope).
use crate::artifacts::home::diff::SHomeDiff;
use crate::artifacts::home::mutations::SHomeMutation;
use crate::artifacts::home::SHomeSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🔢 `change-catalog-generation` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "change-catalog-generation")]
pub struct ChangeCatalogGeneration {
    pub new_catalog_generation: u64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_catalog_generation(new_catalog_generation: u64) -> SHomeMutation {
    SHomeMutation::ChangeCatalogGeneration(ChangeCatalogGeneration { new_catalog_generation })
}

impl protocol::MutationKind<SHomeSnapshot, SHomeMutation> for ChangeCatalogGeneration {
    const SEMANTICS: protocol::SemanticDescriptor =
        protocol::SemanticDescriptor { verb: "change", entity: "catalog-generation", kind: "change-catalog-generation", record: "ChangedCatalogGeneration" };

    fn diff(&self, base: &SHomeSnapshot) -> protocol::MutationOutcome<SHomeDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SHomeSnapshot) -> Vec<SHomeMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change catalog generation to {}", self.new_catalog_generation)
    }
}
//#endregion 🔖️Mutation
