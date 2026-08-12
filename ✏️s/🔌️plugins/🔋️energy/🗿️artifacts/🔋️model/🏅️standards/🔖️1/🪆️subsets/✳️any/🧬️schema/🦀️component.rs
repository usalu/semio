//! 🧬️ EnergyModel artifact schema — every field with its state class.

use crate::artifacts::model::{EnergyModelSnapshot, ENERGY_MODEL_ARTIFACT_SCHEMA_ID, ENERGY_MODEL_DOCUMENT_SCHEMA};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️DocumentHelpers
/// 🌱 Empty persisted snapshot. Relocated from `⚙️engine/🦀️component.rs` (ticket
/// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) — a pure codec helper over the document
/// type, not engine behaviour.
pub fn empty_energy_model_snapshot() -> EnergyModelSnapshot {
    EnergyModelSnapshot::default()
}

/// 🏢️ Decode the typed `Model` from a snapshot's opaque JSON body.
pub fn model_from_snapshot(snapshot: &EnergyModelSnapshot) -> Result<crate::model::Model, String> {
    serde_json::from_str(&snapshot.model_json).map_err(|e| e.to_string())
}

/// 📕️ Encode a typed `Model` into snapshot form.
pub fn snapshot_from_model(model: &crate::model::Model) -> Result<EnergyModelSnapshot, String> {
    Ok(EnergyModelSnapshot {
        schema: ENERGY_MODEL_DOCUMENT_SCHEMA.into(),
        model_json: serde_json::to_string(model).map_err(|e| e.to_string())?,
    })
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️Artifact
/// 🧬️ Full energy-model artifact across persistent and preview classes (no UI app).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.energy.model")]
pub struct EnergyModelArtifact {
    #[state(persistent)]
    pub schema: String,
    /// 🏢️ Opaque JSON of `crate::Model` — building inputs that persist.
    #[state(persistent)]
    pub model_json: String,
    /// 📋️ Opaque JSON of `crate::Results` — recomputed by the BEM engine; never persisted.
    #[state(preview)]
    pub results_json: String,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for EnergyModelArtifact {
    fn default() -> Self {
        Self::from_snapshot(EnergyModelSnapshot::default())
    }
}

impl EnergyModelArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> EnergyModelSnapshot {
        EnergyModelSnapshot {
            schema: self.schema.clone(),
            model_json: self.model_json.clone(),
        }
    }

    /// 🧬️ Builds a full artifact from a snapshot, leaving preview empty.
    pub fn from_snapshot(snapshot: EnergyModelSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            model_json: snapshot.model_json,
            results_json: String::new(),
        }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: EnergyModelSnapshot) {
        self.schema = snapshot.schema;
        self.model_json = snapshot.model_json;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.energy.model` — twenty handcrafted schema leaves.
pub fn energy_model_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: ENERGY_MODEL_ARTIFACT_SCHEMA_ID,
        artifact: schema::FacetLeaves {
            rust: include_str!("🦀️component.rs"),
            typescript: include_str!("🟦️component.ts"),
            graphql: include_str!("🔗️component.graphql"),
            json_schema: include_str!("🔣️component.json"),
            proto: include_str!("🛰️component.proto"),
        },
        snapshot: schema::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️component.rs"),
            typescript: include_str!("📸️snapshot/🟦️component.ts"),
            graphql: include_str!("📸️snapshot/🔗️component.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️component.json"),
            proto: include_str!("📸️snapshot/🛰️component.proto"),
        },
        diff: schema::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️component.rs"),
            typescript: include_str!("🔺️diff/🟦️component.ts"),
            graphql: include_str!("🔺️diff/🔗️component.graphql"),
            json_schema: include_str!("🔺️diff/🔣️component.json"),
            proto: include_str!("🔺️diff/🛰️component.proto"),
        },
        mutations: schema::FacetLeaves {
            rust: include_str!("🧬️mutations/🦀️component.rs"),
            typescript: include_str!("🧬️mutations/🟦️component.ts"),
            graphql: include_str!("🧬️mutations/🔗️component.graphql"),
            json_schema: include_str!("🧬️mutations/🔣️component.json"),
            proto: include_str!("🧬️mutations/🛰️component.proto"),
        },
    }
}
//#endregion 🔖️Descriptor
//#region 🏗️DerivedConstruction
pub mod derived_construction {
    use semio_framework_plugin::ArtifactBuilder;
    use crate::artifacts::model::{EnergyModelDiff, EnergyModelMutation, EnergyModelSnapshot};

    #[derive(Clone, Debug, Default)]
    pub struct ModelBuilderConstruction {
        snapshot: EnergyModelSnapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for ModelBuilderConstruction {
        type Snapshot = EnergyModelSnapshot;
        type Mutation = EnergyModelMutation;
        type Diff = EnergyModelDiff;
        fn empty() -> Self { Self { snapshot: EnergyModelSnapshot::default(), diagnostics: Vec::new() } }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot, diagnostics: Vec::new() } }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<EnergyModelSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<EnergyModelSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
            let diff = <Self::Mutation as protocol::Mutation<Self::Snapshot>>::diff(&mutation, &self.snapshot);
            self.snapshot = <Self::Diff as protocol::MutationDiff<Self::Snapshot>>::apply(&diff, &self.snapshot);
            (self, diff)
        }
        fn absorb(mut self, diff: Self::Diff) -> Self {
            self.snapshot = <EnergyModelDiff as protocol::MutationDiff<EnergyModelSnapshot>>::apply(&diff, &self.snapshot);
            self
        }
        fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
            if self.diagnostics.is_empty() { Ok(self.snapshot) } else { Err(self.diagnostics) }
        }
    }
}
pub use derived_construction::*;
//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis
pub mod derived_analysis {
    use semio_framework_plugin::{ArtifactAnalysis, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
    use crate::artifacts::model::EnergyModelSnapshot;

    #[derive(Clone, Debug, Default)]
    pub struct EnergyModelParts {
        pub snapshot: Option<EnergyModelSnapshot>,
    }

    pub struct EnergyModelAnalyzerAnalysis;

    impl ArtifactAnalysis for EnergyModelAnalyzerAnalysis {
        type Parts = EnergyModelParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.model", standard: StandardId("1"), subset: SubsetId("*") };

        fn sniff(_source: &AnalyzeSource<'_>) -> IoConfidence {
            IoConfidence::Medium
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = EnergyModelParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <EnergyModelSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <EnergyModelSnapshot as store::ArtifactPack>::decode_pack(bytes) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("analyze.binary", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                }
            }
            Analysis { parts, dialect: Self::DIALECT, confidence, diagnostics }
        }
    }
}
pub use derived_analysis::*;
//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets
semio_framework_plugin::derive_artifact_facets!(
    pub spec ModelBuilderFacets {
        construction: derived_construction::ModelBuilderConstruction,
        analysis: derived_analysis::EnergyModelAnalyzerAnalysis,
        composition: super::super::io::derived_composition::EnergyModelComposerComposition,
    }
    builder: ModelBuilder,
    analyzer: EnergyModelAnalyzer,
    composer: EnergyModelComposer,
);
//#endregion 🧬️DerivedArtifactFacets

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_snapshot_matches_schema() {
        let snapshot = empty_energy_model_snapshot();
        assert_eq!(snapshot.schema, ENERGY_MODEL_DOCUMENT_SCHEMA);
    }

    /// 🌱 Relocated from `⚙️engine/🦀️component.rs` — DSL-parse sanity check with zero
    /// `EnergyModelEngine` dependency, so it survives that struct's deletion.
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
