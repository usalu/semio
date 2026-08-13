//! ♻️ `replace-model` payload — replaces the EnergyModel document's `structure`/`zones` composed
//! children together, decoded from `new_model_json` (a serialized `crate::model::Model`; wire shape
//! unchanged by ticket 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM — only the snapshot's OWN
//! persisted representation composes `s.stdio.semio.value`/`table`, not this mutation payload).
//! `schema`/`referencedModel` are never targeted by a mutation, so this is the only meaningfully
//! mutable root content per `📓️derivation-rules.md` rule 6.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ReplaceModel
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceModel {
    pub new_model_json: String,
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ReplaceModel {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "model", kind: "replace-model", record: "ReplacedModel" };

    fn diff(&self, base: &EnergyModelSnapshot) -> EnergyModelDiff {
        crate::artifacts::model::mutations::replace_model::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        crate::artifacts::model::mutations::replace_model::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        "Replace energy model".to_string()
    }
}
//#endregion 🔖️ReplaceModel
