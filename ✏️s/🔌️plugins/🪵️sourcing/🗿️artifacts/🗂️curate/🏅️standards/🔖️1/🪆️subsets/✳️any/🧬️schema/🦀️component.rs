//! 🧬️ Curate artifact schema — every field of the artifact with its state class.

use crate::artifacts::curate::{CuratedItem, Filters, ObjectKind};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Artifact
/// 🧬️ Full curate artifact state across persistent, shared-ui and local-ui classes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.sourcing.curate")]
pub struct CurateArtifact {
    #[state(persistent)]
    pub stock: Vec<ObjectKind>,
    #[state(persistent)]
    pub curated: Vec<CuratedItem>,
    #[state(local_ui)]
    pub filters: Filters,
    #[state(shared_ui)]
    pub selected_object_id: Option<String>,
    #[state(local_ui)]
    pub locale: String,
    #[state(local_ui)]
    pub contributions_json: String,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
fn default_contributions_json() -> String {
    "[]".into()
}

impl Default for CurateArtifact {
    fn default() -> Self {
        Self {
            stock: Vec::new(),
            curated: Vec::new(),
            filters: Filters::default(),
            selected_object_id: None,
            locale: "en-US".into(),
            contributions_json: default_contributions_json(),
        }
    }
}

impl CurateArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> crate::artifacts::curate::CurateSnapshot {
        crate::artifacts::curate::CurateSnapshot {
            stock: self.stock.clone(),
            curated: self.curated.clone(),
        }
    }

    /// 🧬️ Builds a full artifact from a snapshot, leaving UI fields at defaults.
    pub fn from_snapshot(snapshot: crate::artifacts::curate::CurateSnapshot) -> Self {
        Self {
            stock: snapshot.stock,
            curated: snapshot.curated,
            ..Self::default()
        }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: crate::artifacts::curate::CurateSnapshot) {
        self.stock = snapshot.stock;
        self.curated = snapshot.curated;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.sourcing.curate` — twenty handcrafted schema leaves.
pub fn curate_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.sourcing.curate",
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
    use crate::artifacts::curate::schema::diff::CurateDiff;
    use crate::artifacts::curate::schema::mutations::SourcingMutation;
    use crate::artifacts::curate::schema::snapshot::CurateSnapshot;

    #[derive(Clone, Debug, Default)]
    pub struct CurateBuilderConstruction {
        snapshot: CurateSnapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for CurateBuilderConstruction {
        type Snapshot = CurateSnapshot;
        type Mutation = SourcingMutation;
        type Diff = CurateDiff;
        fn empty() -> Self { Self { snapshot: CurateSnapshot::default(), diagnostics: Vec::new() } }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot, diagnostics: Vec::new() } }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<CurateSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<CurateSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
            let d = <SourcingMutation as protocol::Mutation<CurateSnapshot>>::diff(&mutation, &self.snapshot);
            self.snapshot = protocol::MutationDiff::apply(&d, &self.snapshot);
            (self, d)
        }
        fn absorb(mut self, diff: Self::Diff) -> Self {
            self.snapshot = <CurateDiff as protocol::MutationDiff<CurateSnapshot>>::apply(&diff, &self.snapshot);
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
    use crate::artifacts::curate::CurateSnapshot;

    #[derive(Clone, Debug, Default)]
    pub struct CurateParts {
        pub snapshot: Option<CurateSnapshot>,
    }

    pub struct CurateAnalyzerAnalysis;

    impl ArtifactAnalysis for CurateAnalyzerAnalysis {
        type Parts = CurateParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.curate", standard: StandardId("1"), subset: SubsetId("*") };

        fn sniff(_source: &AnalyzeSource<'_>) -> IoConfidence {
            IoConfidence::Medium
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = CurateParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <CurateSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <CurateSnapshot as store::ArtifactPack>::decode_pack(bytes) {
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
    pub spec CurateBuilderFacets {
        construction: derived_construction::CurateBuilderConstruction,
        analysis: derived_analysis::CurateAnalyzerAnalysis,
        composition: super::super::io::derived_composition::CurateComposerComposition,
    }
    builder: CurateBuilder,
    analyzer: CurateAnalyzer,
    composer: CurateComposer,
);
//#endregion 🧬️DerivedArtifactFacets
