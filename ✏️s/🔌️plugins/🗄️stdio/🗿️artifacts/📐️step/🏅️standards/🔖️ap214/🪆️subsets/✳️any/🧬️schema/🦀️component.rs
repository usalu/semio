//! 🧬️ StepArtifact schema — full artifact state, mirrors `StepSnapshot`'s own typed HEADER +
//! id-keyed entity graph (never a raw `Part21Document` — same specific-code mandate as the
//! snapshot itself).

use crate::artifacts::step::schema::snapshot::{StepEntity, StepHeader};
use crate::artifacts::step::StepSnapshot;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Artifact
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.step")]
pub struct StepArtifact {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    #[serde(default)]
    pub header: StepHeader,
    #[state(persistent)]
    #[serde(default)]
    pub entities: Vec<StepEntity>,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for StepArtifact {
    fn default() -> Self {
        Self::from_snapshot(StepSnapshot::default())
    }
}

impl StepArtifact {
    pub fn to_snapshot(&self) -> StepSnapshot {
        StepSnapshot {
            schema: self.schema.clone(),
            header: self.header.clone(),
            entities: self.entities.clone(),
        }
    }

    pub fn from_snapshot(snapshot: StepSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            header: snapshot.header,
            entities: snapshot.entities,
        }
    }

    pub fn set_snapshot(&mut self, snapshot: StepSnapshot) {
        self.schema = snapshot.schema;
        self.header = snapshot.header;
        self.entities = snapshot.entities;
    }

    /// 🧐️ Derived BrepMesh analyzer view — computed on demand from the typed entity graph via
    /// `StepSnapshot::to_part21_document`, never stored.
    pub fn brep_mesh(&self) -> crate::artifacts::step::engine::brep::BrepMeshView {
        crate::artifacts::step::engine::brep::analyze_brep_mesh(&self.to_snapshot().to_part21_document())
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
pub fn step_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.stdio.step",
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
    use crate::artifacts::step::{StepDiff, StepMutation, StepSnapshot};

    //#region 🔖️Builder
    /// 🏗️ Builds a `stdio.step` snapshot.
    #[derive(Clone, Debug, Default)]
    pub struct StepBuilderConstruction {
        snapshot: StepSnapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for StepBuilderConstruction {
        type Snapshot = StepSnapshot;
        type Mutation = StepMutation;
        type Diff = StepDiff;
        fn empty() -> Self {
            Self { snapshot: StepSnapshot::default(), diagnostics: Vec::new() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<StepSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<StepSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
            let diff = crate::artifacts::step::schema::mutations::apply_step_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }
        fn absorb(mut self, diff: Self::Diff) -> Self {
            self.snapshot = <StepDiff as protocol::MutationDiff<StepSnapshot>>::apply(&diff, &self.snapshot);
            self
        }
        fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
            if self.diagnostics.is_empty() { Ok(self.snapshot) } else { Err(self.diagnostics) }
        }
    }
    //#endregion 🔖️Builder
}
pub use derived_construction::*;
//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis
pub mod derived_analysis {
    use semio_framework_plugin::{ArtifactAnalysis, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
    use crate::artifacts::step::StepSnapshot;

    //#region 🔖️Parts
    /// 🧩 Analyzed `stdio.step` parts.
    #[derive(Clone, Debug, Default)]
    pub struct StepParts {
        pub snapshot: Option<StepSnapshot>,
    }
    //#endregion 🔖️Parts

    //#region 🔖️Analyzer
    /// 🧐️ Analyzes `stdio.step` (ap214/✳️any) sources.
    pub struct StepAnalyzerAnalysis;

    impl ArtifactAnalysis for StepAnalyzerAnalysis {
        type Parts = StepParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.step", standard: StandardId("ap214"), subset: SubsetId("*") };

        fn sniff(_source: &AnalyzeSource<'_>) -> IoConfidence {
            IoConfidence::Medium
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = StepParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <StepSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error(
                                "stdio.analyze.text",
                                dsl::TextSpan::at(1, 1),
                                err.to_string(),
                            ));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <StepSnapshot as store::ArtifactPack>::decode_pack(bytes) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error(
                                "stdio.analyze.binary",
                                dsl::TextSpan::at(1, 1),
                                err.to_string(),
                            ));
                        }
                    },
                }
            }
            Analysis { parts, dialect: Self::DIALECT, confidence, diagnostics }
        }
    }
    //#endregion 🔖️Analyzer
}
pub use derived_analysis::*;
//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets
semio_framework_plugin::derive_artifact_facets!(
    pub spec StepBuilderFacets {
        construction: derived_construction::StepBuilderConstruction,
        analysis: derived_analysis::StepAnalyzerAnalysis,
        composition: super::super::io::derived_composition::StepComposerComposition,
    }
    builder: StepBuilder,
    analyzer: StepAnalyzer,
    composer: StepComposer,
);
//#endregion 🧬️DerivedArtifactFacets
