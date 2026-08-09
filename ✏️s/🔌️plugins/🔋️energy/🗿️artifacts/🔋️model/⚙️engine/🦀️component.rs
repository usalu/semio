//! ⚙️ EnergyModel artifact — headless BEM compute engine owning a real `EnergyModelArtifact`.

use crate::artifacts::model::{
    EnergyModelArtifact, EnergyModelDiff, EnergyModelMutation, EnergyModelSnapshot,
    ENERGY_MODEL_DOCUMENT_SCHEMA,
};
use crate::model::Model;
use crate::results::Results;

//#region 🔖️DocumentHelpers
/// 🌱 Empty persisted snapshot.
pub fn empty_energy_model_snapshot() -> EnergyModelSnapshot {
    EnergyModelSnapshot::default()
}

/// 🏢️ Decode the typed `Model` from a snapshot's opaque JSON body.
pub fn model_from_snapshot(snapshot: &EnergyModelSnapshot) -> Result<Model, String> {
    serde_json::from_str(&snapshot.model_json).map_err(|e| e.to_string())
}

/// 📕️ Encode a typed `Model` into snapshot form.
pub fn snapshot_from_model(model: &Model) -> Result<EnergyModelSnapshot, String> {
    Ok(EnergyModelSnapshot {
        schema: ENERGY_MODEL_DOCUMENT_SCHEMA.into(),
        model_json: serde_json::to_string(model).map_err(|e| e.to_string())?,
    })
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️Register
/// 🗂️ Registers codecs and the artifact schema descriptor.
pub fn register() {
    register_artifact_schema();
    register_pilot_languages();
    store::register_document_codec(store::DocumentCodec::of::<
        EnergyModelSnapshot,
        EnergyModelMutation,
    >(ENERGY_MODEL_DOCUMENT_SCHEMA));
}

/// 📌️ Registers handcrafted facet grammars (text) and protocols (binary).
pub fn register_pilot_languages() {
    dsl::register_language(dsl::LanguageSpec {
        id: "energy.model",
        extension: Some("energy"),
        role: dsl::LanguageRole::Document,
        grammar: Some(crate::artifacts::model::dsl::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::model::dsl::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::model::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::model::snapshot::pack::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("energy.model"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "energy.model.op",
        extension: None,
        role: dsl::LanguageRole::Ops,
        grammar: Some(crate::artifacts::model::op::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::model::op::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::model::spr::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::model::spr::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("energy.model.op"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "energy.model.diff",
        extension: None,
        role: dsl::LanguageRole::Diff,
        grammar: Some(crate::artifacts::model::diff::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::model::diff::COMPONENT_GRAMMAR_PATH),
        protocol: None,
        protocol_path: None,
        hooks: dsl::passthrough_hooks("energy.model.diff"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "energy.model.pack",
        extension: None,
        role: dsl::LanguageRole::Pack,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::model::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::model::snapshot::pack::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("energy.model.pack"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "energy.model.spr",
        extension: None,
        role: dsl::LanguageRole::Spr,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::model::spr::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::model::spr::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("energy.model.spr"),
    });
}
//#endregion 🔖️Register

//#region 🔖️SchemaRegistry
/// 📌️ Registers the fifteen handcrafted schema leaves for `s.energy.model`.
pub fn register_artifact_schema() {
    ::schema::register_artifact_schema_descriptor(crate::artifacts::model::schema::energy_model_artifact_schema_descriptor());
}
//#endregion 🔖️SchemaRegistry

//#region 🔖️ArtifactEngine
/// ⚡️ Headless energy-model artifact engine — owns a real `EnergyModelArtifact`.
pub struct EnergyModelEngine {
    artifact_state: EnergyModelArtifact,
    snapshot_state: EnergyModelSnapshot,
}

impl EnergyModelEngine {
    /// 🏗️ Builds an engine from a persisted snapshot (preview results start empty).
    pub fn new(snapshot: EnergyModelSnapshot) -> Self {
        let artifact_state = EnergyModelArtifact::from_snapshot(snapshot.clone());
        Self {
            artifact_state,
            snapshot_state: snapshot,
        }
    }

    /// 🚀️ Runs the BEM simulation, updates preview `results_json`, returns owned results.
    pub fn run_simulation(
        &mut self,
        config: &crate::kernel::SimulationConfig,
    ) -> Result<Results, crate::error::Error> {
        let model = model_from_snapshot(&self.snapshot_state).map_err(crate::error::Error::severe)?;
        let results = crate::sim::Engine::run(&model, config)?;
        self.artifact_state.results_json =
            serde_json::to_string(&results).map_err(|e| crate::error::Error::severe(e.to_string()))?;
        Ok(results)
    }
}

impl protocol::ArtifactEngine for EnergyModelEngine {
    type Artifact = EnergyModelArtifact;
    type Snapshot = EnergyModelSnapshot;
    type Mutation = EnergyModelMutation;
    type Diff = EnergyModelDiff;

    fn artifact(&self) -> &Self::Artifact {
        &self.artifact_state
    }

    fn snapshot(&self) -> &Self::Snapshot {
        &self.snapshot_state
    }

    fn apply(&mut self, mutation: &Self::Mutation) -> Result<Self::Diff, protocol::EngineFault> {
        let diff = <Self::Mutation as protocol::Mutation<Self::Snapshot>>::diff(mutation, &self.snapshot_state);
        self.snapshot_state = <Self::Diff as protocol::MutationDiff<Self::Snapshot>>::apply(&diff, &self.snapshot_state);
        self.artifact_state.set_snapshot(self.snapshot_state.clone());
        Ok(diff)
    }

    fn inverse(&self, mutation: &Self::Mutation) -> Vec<Self::Mutation> {
        <Self::Mutation as protocol::Mutation<Self::Snapshot>>::inverse(mutation, &self.snapshot_state)
    }
}
//#endregion 🔖️ArtifactEngine

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_snapshot_matches_schema() {
        let snapshot = empty_energy_model_snapshot();
        assert_eq!(snapshot.schema, ENERGY_MODEL_DOCUMENT_SCHEMA);
    }

    #[test]
    fn engine_owns_artifact_not_snapshot_alias() {
        use protocol::ArtifactEngine;
        let engine = EnergyModelEngine::new(empty_energy_model_snapshot());
        assert_eq!(engine.artifact().schema, ENERGY_MODEL_DOCUMENT_SCHEMA);
        assert!(engine.artifact().results_json.is_empty());
        assert_eq!(engine.snapshot().model_json, "{}");
    }

    #[test]
    fn example_fixture_parses() {
        let document = crate::artifacts::model::dsl::parse_dsl(
            crate::artifacts::model::dsl::SEMIO_ENERGY_MODEL_EXAMPLE_TEXT,
        )
        .expect("parse");
        assert_eq!(document.schema, ENERGY_MODEL_DOCUMENT_SCHEMA);
        assert!(!document.model_json.is_empty());
    }
}
//#endregion 🧪️Tests
