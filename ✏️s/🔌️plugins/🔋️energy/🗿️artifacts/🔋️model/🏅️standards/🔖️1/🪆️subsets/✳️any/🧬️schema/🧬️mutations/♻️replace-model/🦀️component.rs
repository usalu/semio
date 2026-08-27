//! ♻️ Direct `replace-model` payload and behavior owner.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// ♻️ Replaces the composed model structure and zones from one typed model document.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceModel {
    pub new_model_json: String,
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ReplaceModel {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "model", kind: "replace-model", record: "ReplacedModel" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        "Replace energy model".to_string()
    }
}

/// 🏷️ Direct semantic roster exported for the language-neutral test adapter.
pub const KINDS: &[&str] = &["replace-model"];
//#endregion 🔖️Mutation

//#region 🌉️TestBridge
/// 🔮️ Reports the forward and inverse behavior of one committed language-neutral vector.
pub fn energy_model_mutation_report_json(base_json: &str, mutation_json: &str, after_json: &str) -> Result<String, String> {
    let decode_snapshot = |text: &str| -> Result<EnergyModelSnapshot, String> { serde_json::from_str(text).map_err(|error| error.to_string()) };
    let base = decode_snapshot(base_json)?;
    let expected = decode_snapshot(after_json)?;
    let mutation: EnergyModelMutation = serde_json::from_str(mutation_json).map_err(|error| error.to_string())?;
    let mut applied = base.clone();
    let forward = <EnergyModelMutation as protocol::Mutation<EnergyModelSnapshot>>::diff(&mutation, &base).apply_to(&mut applied);
    let inverse = <EnergyModelMutation as protocol::Mutation<EnergyModelSnapshot>>::inverse(&mutation, &base);
    let mut undone = applied.clone();
    let mut inverse_messages = Vec::new();
    for step in &inverse {
        let outcome = <EnergyModelMutation as protocol::Mutation<EnergyModelSnapshot>>::diff(step, &undone).apply_to(&mut undone);
        inverse_messages.extend(outcome.messages().iter().cloned());
    }
    let report = serde_json::json!({
        "base": serde_json::to_value(&base).map_err(|error| error.to_string())?,
        "expectedSnapshot": serde_json::to_value(&expected).map_err(|error| error.to_string())?,
        "snapshot": serde_json::to_value(&applied).map_err(|error| error.to_string())?,
        "diff": serde_json::to_value(forward.diff()).map_err(|error| error.to_string())?,
        "messages": serde_json::to_value(forward.messages()).map_err(|error| error.to_string())?,
        "inverseSteps": serde_json::to_value(&inverse).map_err(|error| error.to_string())?,
        "inverseSnapshot": serde_json::to_value(&undone).map_err(|error| error.to_string())?,
        "inverseMessages": serde_json::to_value(&inverse_messages).map_err(|error| error.to_string())?,
    });
    Ok(report.to_string())
}
//#endregion 🌉️TestBridge

//#region 🧪️Behavior
#[cfg(test)]
mod tests {
    use super::*;
    use protocol::{Mutation, MutationDiff, SemanticMutation};

    fn demo_model_json(name: &str) -> String {
        serde_json::to_string(&crate::model::Model { name: name.into(), ..crate::model::Model::default() }).expect("Model serializes")
    }

    fn mutation(name: &str) -> EnergyModelMutation {
        EnergyModelMutation::ReplaceModel(ReplaceModel { new_model_json: demo_model_json(name) })
    }

    #[semio_framework_async_macros::async_test]
    async fn semantic_descriptor_and_inverse_are_complete() {
        let base = EnergyModelSnapshot::default();
        let operation = mutation("demo");
        let descriptor = protocol::SemanticMutation::semantics(&operation);
        assert!(protocol::is_approved_verb(descriptor.verb));
        assert_eq!(<EnergyModelMutation as SemanticMutation<EnergyModelSnapshot>>::kinds().len(), KINDS.len());
        let forward = operation.diff(&base).diff().apply(&base).expect("valid mutation diff");
        let mut restored = forward;
        for back in operation.inverse(&base) {
            restored = back.diff(&restored).diff().apply(&restored).expect("valid inverse diff");
        }
        assert_eq!(restored, base);
    }

    #[semio_framework_async_macros::async_test]
    async fn inverse_and_absorb_laws_hold() {
        let base = EnergyModelSnapshot::default();
        let operation = mutation("a");
        protocol::os_spr::testkit::assert_mutation_inverse_law(&base, &operation).await;
        let first = operation.diff(&base).diff().clone();
        let second = mutation("b").diff(&base).diff().clone();
        protocol::os_spr::testkit::assert_mutation_diff_absorb_law(&base, first, second).await;
    }
}
//#endregion 🧪️Behavior
