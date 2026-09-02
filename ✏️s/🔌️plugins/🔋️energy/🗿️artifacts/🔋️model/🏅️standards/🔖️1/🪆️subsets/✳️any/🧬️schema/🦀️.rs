//! 🧬️ EnergyModel artifact schema — every field with its state class.

use crate::artifacts::model::{EnergyModelSnapshot, EnergyStructureChild, EnergyZonesChild, ENERGY_MODEL_ARTIFACT_SCHEMA_ID, ENERGY_MODEL_DOCUMENT_SCHEMA};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};
use semio_framework_os_kernel::{from_dsl_value, to_dsl_value, DslValue, FromValue, ToValue, ValueError};

//#region 🔖️DocumentHelpers
/// 🌱 Empty persisted snapshot. Relocated from `⚙️engine/🦀️.rs` (ticket
/// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) — a pure codec helper over the document
/// type, not engine behaviour.
pub fn empty_energy_model_snapshot() -> EnergyModelSnapshot {
    EnergyModelSnapshot::default()
}

/// 🏢️ The typed `Model` behind a snapshot's composed `structure`/`zones` children — reads through
/// the working-scene cache (ticket 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM). Replaces the old
/// `serde_json::from_str(&snapshot.model_json)` decode-on-demand now that `model_json` is gone.
pub fn model_from_snapshot(snapshot: &EnergyModelSnapshot) -> Result<crate::model::Model, String> {
    Ok(crate::artifacts::model::energy_model(snapshot))
}

/// 📕️ Encode a typed `Model` into snapshot form — mints+caches its composed `structure`/`zones`
/// children in one call via [`crate::artifacts::model::energy_snapshot_with_state`].
pub fn snapshot_from_model(model: &crate::model::Model) -> Result<EnergyModelSnapshot, String> {
    Ok(crate::artifacts::model::energy_snapshot_with_state(ENERGY_MODEL_DOCUMENT_SCHEMA, model, None))
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️Artifact
/// 🧬️ Full energy-model artifact across the artifact lane (no UI app). Ticket
/// 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM: mirrors `EnergyModelSnapshot`'s `structure`/`zones`/
/// `referenced_model` field swap identically (same shape every composed exemplar's full-artifact
/// struct mirrors its snapshot). `results_json` is UNCHANGED — a preview-only field (recomputed by
/// the BEM engine, never persisted, never part of the snapshot), no composition applies to it.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.energy.model")]
pub struct EnergyModelArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub model: crate::model::Model,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio.value")]
    pub structure: EnergyStructureChild,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio.table")]
    pub zones: EnergyZonesChild,
    #[state(artifact)]
    #[link_slot(roles("model"))]
    #[serde(rename = "referencedModel", default, skip_serializing_if = "Option::is_none")]
    pub referenced_model: Option<store::ArtifactLink>,
    /// 📋️ Opaque JSON of `crate::Results` — recomputed by the BEM engine; never persisted.
    #[state(artifact)]
    pub results_json: String,
}

// 🌱️ Hand-written, not derived — same composed-child/link shape as
// `📸️snapshot/🦀️.rs`'s `EnergyModelSnapshot` impl.
impl ToValue for EnergyModelArtifact {
    fn to_value(&self) -> DslValue {
        DslValue::object([
            ("schema".to_string(), self.schema.to_value()),
            ("model".to_string(), self.model.to_value()),
            ("structure".to_string(), to_dsl_value(&self.structure).unwrap_or(DslValue::Null)),
            ("zones".to_string(), to_dsl_value(&self.zones).unwrap_or(DslValue::Null)),
            ("referencedModel".to_string(), to_dsl_value(&self.referenced_model).unwrap_or(DslValue::Null)),
            ("resultsJson".to_string(), self.results_json.to_value()),
        ])
    }
}
impl FromValue for EnergyModelArtifact {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let entries = DslValue::into_object(value)?;
        let field = |key: &str| entries.iter().find(|(k, _)| k == key).map(|(_, v)| v.clone()).unwrap_or(DslValue::Null);
        Ok(Self {
            schema: String::from_value(field("schema"))?,
            model: crate::model::Model::from_value(field("model"))?,
            structure: from_dsl_value(field("structure")).map_err(ValueError::new)?,
            zones: from_dsl_value(field("zones")).map_err(ValueError::new)?,
            referenced_model: from_dsl_value(field("referencedModel")).map_err(ValueError::new)?,
            results_json: String::from_value(field("resultsJson"))?,
        })
    }
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
        EnergyModelSnapshot { schema: self.schema.clone(), model: self.model.clone(), structure: self.structure.clone(), zones: self.zones.clone(), referenced_model: self.referenced_model.clone() }
    }

    /// 🧬️ Builds a full artifact from a snapshot, leaving preview empty.
    pub fn from_snapshot(snapshot: EnergyModelSnapshot) -> Self {
        Self { schema: snapshot.schema, model: snapshot.model, structure: snapshot.structure, zones: snapshot.zones, referenced_model: snapshot.referenced_model, results_json: String::new() }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: EnergyModelSnapshot) {
        self.schema = snapshot.schema;
        self.model = snapshot.model;
        self.structure = snapshot.structure;
        self.zones = snapshot.zones;
        self.referenced_model = snapshot.referenced_model;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.energy.model` — twenty handcrafted schema leaves.
pub fn energy_model_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: ENERGY_MODEL_ARTIFACT_SCHEMA_ID,
        artifact: schema::FacetLeaves {
            rust: include_str!("🦀️.rs"),
            typescript: include_str!("🟦️.ts"),
            graphql: include_str!("🔗️.graphql"),
            json_schema: include_str!("🔣️.json"),
            proto: include_str!("🛰️.proto"),
        },
        snapshot: schema::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️.rs"),
            typescript: include_str!("📸️snapshot/🟦️.ts"),
            graphql: include_str!("📸️snapshot/🔗️.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️.json"),
            proto: include_str!("📸️snapshot/🛰️.proto"),
        },
        diff: schema::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️.rs"),
            typescript: include_str!("🔺️diff/🟦️.ts"),
            graphql: include_str!("🔺️diff/🔗️.graphql"),
            json_schema: include_str!("🔺️diff/🔣️.json"),
            proto: include_str!("🔺️diff/🛰️.proto"),
        },
        mutations: schema::FacetLeaves {
            rust: include_str!("🧬️mutations/🦀️.rs"),
            typescript: include_str!("🧬️mutations/🟦️.ts"),
            graphql: include_str!("🧬️mutations/🔗️.graphql"),
            json_schema: include_str!("🧬️mutations/🔣️.json"),
            proto: include_str!("🧬️mutations/🛰️.proto"),
        },
    }
}
//#endregion 🔖️Descriptor
//#region 🏗️DerivedConstruction
pub mod derived_construction {
    use crate::artifacts::model::{EnergyModelDiff, EnergyModelMutation, EnergyModelSnapshot};
    use semio_framework_plugin::ArtifactBuilder;

    #[derive(Clone, Debug, Default)]
    pub struct ModelBuilderConstruction {
        snapshot: EnergyModelSnapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for ModelBuilderConstruction {
        type Snapshot = EnergyModelSnapshot;
        type Mutation = EnergyModelMutation;
        type Diff = EnergyModelDiff;
        async fn empty() -> Self {
            Self { snapshot: EnergyModelSnapshot::default(), diagnostics: Vec::new() }
        }
        async fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        async fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<EnergyModelSnapshot as store::ArtifactDsl>::parse_dsl(text)?).await)
        }
        async fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<EnergyModelSnapshot as store::ArtifactPack>::decode_pack(bytes)?).await)
        }
        async fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let outcome = <Self::Mutation as protocol::Mutation<Self::Snapshot>>::diff(&mutation, &self.snapshot);
            match <Self::Diff as protocol::MutationDiff<Self::Snapshot>>::apply(outcome.diff(), &self.snapshot) {
                Ok(snapshot) => self.snapshot = snapshot,
                Err(error) => self.diagnostics.push(dsl::Diagnostic::error("mutation.apply", dsl::TextSpan::at(1, 1), error.to_string())),
            }
            (self, outcome)
        }
        async fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            let snapshot = <EnergyModelDiff as protocol::MutationDiff<EnergyModelSnapshot>>::apply(&diff, &self.snapshot)?;
            self.snapshot = snapshot;
            Ok(self)
        }
        async fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
            if self.diagnostics.is_empty() {
                Ok(self.snapshot)
            } else {
                Err(self.diagnostics)
            }
        }
    }
}
pub use derived_construction::*;
//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis
pub mod derived_analysis {
    use crate::artifacts::model::EnergyModelSnapshot;
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

    #[derive(Clone, Debug, Default)]
    pub struct EnergyModelParts {
        pub snapshot: Option<EnergyModelSnapshot>,
    }

    pub struct EnergyModelAnalyzerAnalysis;

    impl ArtifactAnalysis for EnergyModelAnalyzerAnalysis {
        type Parts = EnergyModelParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.model", standard: StandardId("1"), subset: SubsetId("*") };

        async fn sniff(_source: &AnalyzeSource<'_>) -> IoConfidence {
            IoConfidence::Medium
        }

        async fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
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
        construction: ModelBuilderConstruction,
        analysis: EnergyModelAnalyzerAnalysis,
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

    #[semio_framework_async_macros::async_test]
    async fn empty_snapshot_matches_schema() {
        let snapshot = empty_energy_model_snapshot();
        assert_eq!(snapshot.schema, ENERGY_MODEL_DOCUMENT_SCHEMA);
    }

    /// 🌱 Relocated from `⚙️engine/🦀️.rs` — DSL-parse sanity check with zero
    /// `EnergyModelEngine` dependency, so it survives that struct's deletion. Ticket
    /// 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM: `model_json` is gone — asserts the composed
    /// `structure`/`zones` child handles are real (non-empty ids) instead.
    #[semio_framework_async_macros::async_test]
    async fn example_fixture_parses() {
        let document = crate::artifacts::model::dsl::parse_dsl(crate::artifacts::model::dsl::SEMIO_ENERGY_MODEL_EXAMPLE_TEXT).expect("parse");
        assert_eq!(document.schema, ENERGY_MODEL_DOCUMENT_SCHEMA);
        assert!(!document.structure.child_id.is_empty());
        assert!(!document.zones.child_id.is_empty());
    }
}
//#endregion 🧪️Tests
