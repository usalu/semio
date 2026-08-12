//! 🧬️ Imperative artifact schema — every field with its state class.

use crate::artifacts::imperative::Path;
use neural_engine::Value;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

//#region 🔖️Artifact
/// 🧬️ Full imperative artifact state across persistent, shared-ui, local-ui and effect classes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.imperative.imperative")]
pub struct ImperativeArtifact {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    pub path: Path,
    #[state(persistent)]
    #[serde(default)]
    pub seed: BTreeMap<String, Value>,
    #[state(shared_ui)]
    #[serde(default)]
    pub selected_step_ids: Vec<String>,
    #[state(local_ui)]
    pub locale: String,
    #[state(local_ui)]
    #[serde(default = "default_contributions_json")]
    pub contributions_json: String,
    #[state(effect)]
    #[serde(default)]
    pub run_output_json: String,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
fn default_contributions_json() -> String {
    "[]".into()
}

impl Default for ImperativeArtifact {
    fn default() -> Self {
        Self {
            schema: "imperative.document".into(),
            path: Path::new(),
            seed: BTreeMap::new(),
            selected_step_ids: Vec::new(),
            locale: "en-US".into(),
            contributions_json: default_contributions_json(),
            run_output_json: String::new(),
        }
    }
}

impl ImperativeArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> crate::artifacts::imperative::ImperativeSnapshot {
        crate::artifacts::imperative::ImperativeSnapshot {
            schema: self.schema.clone(),
            path: self.path.clone(),
            seed: self.seed.clone(),
        }
    }

    /// 🧬️ Builds a full artifact from a snapshot, leaving UI fields at defaults.
    pub fn from_snapshot(snapshot: crate::artifacts::imperative::ImperativeSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            path: snapshot.path,
            seed: snapshot.seed,
            ..Self::default()
        }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: crate::artifacts::imperative::ImperativeSnapshot) {
        self.schema = snapshot.schema;
        self.path = snapshot.path;
        self.seed = snapshot.seed;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.imperative.imperative` — twenty handcrafted schema leaves.
pub fn imperative_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.imperative.imperative",
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
    use crate::artifacts::imperative::schema::diff::ImperativeDiff;
    use crate::artifacts::imperative::schema::mutations::ImperativeMutation;
    use crate::artifacts::imperative::schema::snapshot::ImperativeSnapshot;

    #[derive(Clone, Debug, Default)]
    pub struct ImperativeBuilderConstruction {
        snapshot: ImperativeSnapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for ImperativeBuilderConstruction {
        type Snapshot = ImperativeSnapshot;
        type Mutation = ImperativeMutation;
        type Diff = ImperativeDiff;
        fn empty() -> Self { Self { snapshot: ImperativeSnapshot::default(), diagnostics: Vec::new() } }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot, diagnostics: Vec::new() } }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<ImperativeSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<ImperativeSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
            let d = <ImperativeMutation as protocol::Mutation<ImperativeSnapshot>>::diff(&mutation, &self.snapshot);
            self.snapshot = protocol::MutationDiff::apply(&d, &self.snapshot);
            (self, d)
        }
        fn absorb(mut self, diff: Self::Diff) -> Self {
            self.snapshot = <ImperativeDiff as protocol::MutationDiff<ImperativeSnapshot>>::apply(&diff, &self.snapshot);
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
    use crate::artifacts::imperative::ImperativeSnapshot;

    #[derive(Clone, Debug, Default)]
    pub struct ImperativeParts {
        pub snapshot: Option<ImperativeSnapshot>,
    }

    pub struct ImperativeAnalyzerAnalysis;

    impl ArtifactAnalysis for ImperativeAnalyzerAnalysis {
        type Parts = ImperativeParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.imperative", standard: StandardId("1"), subset: SubsetId("*") };

        fn sniff(_source: &AnalyzeSource<'_>) -> IoConfidence {
            IoConfidence::Medium
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = ImperativeParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <ImperativeSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <ImperativeSnapshot as store::ArtifactPack>::decode_pack(bytes) {
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
    pub spec ImperativeBuilderFacets {
        construction: derived_construction::ImperativeBuilderConstruction,
        analysis: derived_analysis::ImperativeAnalyzerAnalysis,
        composition: super::super::io::derived_composition::ImperativeComposerComposition,
    }
    builder: ImperativeBuilder,
    analyzer: ImperativeAnalyzer,
    composer: ImperativeComposer,
);
//#endregion 🧬️DerivedArtifactFacets
