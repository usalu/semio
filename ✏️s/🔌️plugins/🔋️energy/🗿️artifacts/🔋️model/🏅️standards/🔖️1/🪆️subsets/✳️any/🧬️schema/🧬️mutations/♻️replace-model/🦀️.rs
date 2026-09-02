//! ♻️ Direct `replace-model` payload and behavior owner.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_os_kernel::ToValue;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// ♻️ Replaces the composed model structure and zones from one typed model document.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
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
    let decode_snapshot = |text: &str| -> Result<EnergyModelSnapshot, String> { pack::json::from_json_str::<EnergyModelSnapshot>(text).map_err(|error| error.to_string()) };
    let base = decode_snapshot(base_json)?;
    let expected = decode_snapshot(after_json)?;
    let mutation: EnergyModelMutation = pack::json::from_json_str(mutation_json).map_err(|error| error.to_string())?;
    let mut applied = base.clone();
    let forward = <EnergyModelMutation as protocol::Mutation<EnergyModelSnapshot>>::diff(&mutation, &base).apply_to(&mut applied);
    let inverse = <EnergyModelMutation as protocol::Mutation<EnergyModelSnapshot>>::inverse(&mutation, &base);
    let mut undone = applied.clone();
    let mut inverse_messages = Vec::new();
    for step in &inverse {
        let outcome = <EnergyModelMutation as protocol::Mutation<EnergyModelSnapshot>>::diff(step, &undone).apply_to(&mut undone);
        inverse_messages.extend(outcome.messages().iter().cloned());
    }
    // 🌉️ `MutationMessage` (`🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️.rs`) is a
    // framework-owned type that has not itself gained `ToValue`/`FromValue` — its two call sites
    // here go through the PRE-EXISTING `protocol::to_dsl_value` serde bridge (framework-internal,
    // exempt) and land in `pack::json::Value` via `pack::json::from_dsl_value`, same as
    // `➗️mathematical`'s identical bridge function.
    let messages_json = protocol::to_dsl_value(forward.messages()).map(|value| pack::json::from_dsl_value(&value)).map_err(|error| error.to_string())?;
    let inverse_messages_json = protocol::to_dsl_value(&inverse_messages).map(|value| pack::json::from_dsl_value(&value)).map_err(|error| error.to_string())?;
    let report = pack::json::object([
        ("base".to_string(), pack::json::from_dsl_value(&base.to_value())),
        ("expectedSnapshot".to_string(), pack::json::from_dsl_value(&expected.to_value())),
        ("snapshot".to_string(), pack::json::from_dsl_value(&applied.to_value())),
        ("diff".to_string(), pack::json::from_dsl_value(&forward.diff().to_value())),
        ("messages".to_string(), messages_json),
        ("inverseSteps".to_string(), pack::json::from_dsl_value(&inverse.to_value())),
        ("inverseSnapshot".to_string(), pack::json::from_dsl_value(&undone.to_value())),
        ("inverseMessages".to_string(), inverse_messages_json),
    ]);
    Ok(pack::json::to_string(&report))
}
//#endregion 🌉️TestBridge

//#region 🧪️Behavior
#[cfg(test)]
mod tests {
    use super::*;
    use protocol::{Mutation, MutationDiff, SemanticMutation};

    fn demo_model_json(name: &str) -> String {
        pack::json::to_json_string(&crate::model::Model { name: name.into(), ..crate::model::Model::default() })
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
