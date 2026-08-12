//! 🧬️ S Home artifact schema — every field of the artifact with its state class.

use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Artifact
/// 🧬️ Full S Home launcher artifact state across persistent and local-ui classes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.space.home")]
pub struct SHomeArtifact {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    pub catalog_generation: u64,
    #[state(local_ui)]
    pub active_panel_tab: String,
    #[state(local_ui)]
    pub locale: String,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for SHomeArtifact {
    fn default() -> Self {
        Self {
            schema: crate::artifacts::home::S_HOME_DOCUMENT_SCHEMA.into(),
            catalog_generation: 0,
            active_panel_tab: String::new(),
            locale: "en-US".into(),
        }
    }
}

impl SHomeArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> crate::artifacts::home::SHomeSnapshot {
        crate::artifacts::home::SHomeSnapshot {
            schema: self.schema.clone(),
            catalog_generation: self.catalog_generation,
        }
    }

    /// 🧬️ Builds a full artifact from a snapshot, leaving UI fields at defaults.
    pub fn from_snapshot(snapshot: crate::artifacts::home::SHomeSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            catalog_generation: snapshot.catalog_generation,
            ..Self::default()
        }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: crate::artifacts::home::SHomeSnapshot) {
        self.schema = snapshot.schema;
        self.catalog_generation = snapshot.catalog_generation;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.space.home` — twenty handcrafted schema leaves.
pub fn home_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.space.home",
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
    use crate::artifacts::home::schema::diff::SHomeDiff;
    use crate::artifacts::home::schema::mutations::SHomeMutation;
    use crate::artifacts::home::schema::snapshot::SHomeSnapshot;

    #[derive(Clone, Debug, Default)]
    pub struct HomeBuilderConstruction {
        snapshot: SHomeSnapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for HomeBuilderConstruction {
        type Snapshot = SHomeSnapshot;
        type Mutation = SHomeMutation;
        type Diff = SHomeDiff;
        fn empty() -> Self { Self { snapshot: SHomeSnapshot::default(), diagnostics: Vec::new() } }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot, diagnostics: Vec::new() } }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<SHomeSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<SHomeSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
            let d = <SHomeMutation as protocol::Mutation<SHomeSnapshot>>::diff(&mutation, &self.snapshot);
            self.snapshot = protocol::MutationDiff::apply(&d, &self.snapshot);
            (self, d)
        }
        fn absorb(mut self, diff: Self::Diff) -> Self {
            self.snapshot = <SHomeDiff as protocol::MutationDiff<SHomeSnapshot>>::apply(&diff, &self.snapshot);
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
    use crate::artifacts::home::SHomeSnapshot;

    #[derive(Clone, Debug, Default)]
    pub struct SHomeParts {
        pub snapshot: Option<SHomeSnapshot>,
    }

    pub struct SHomeAnalyzerAnalysis;

    impl ArtifactAnalysis for SHomeAnalyzerAnalysis {
        type Parts = SHomeParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.home", standard: StandardId("1"), subset: SubsetId("*") };

        fn sniff(_source: &AnalyzeSource<'_>) -> IoConfidence {
            IoConfidence::Medium
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = SHomeParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <SHomeSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <SHomeSnapshot as store::ArtifactPack>::decode_pack(bytes) {
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
    pub spec HomeBuilderFacets {
        construction: derived_construction::HomeBuilderConstruction,
        analysis: derived_analysis::SHomeAnalyzerAnalysis,
        composition: super::super::io::derived_composition::SHomeComposerComposition,
    }
    builder: HomeBuilder,
    analyzer: SHomeAnalyzer,
    composer: SHomeComposer,
);
//#endregion 🧬️DerivedArtifactFacets
