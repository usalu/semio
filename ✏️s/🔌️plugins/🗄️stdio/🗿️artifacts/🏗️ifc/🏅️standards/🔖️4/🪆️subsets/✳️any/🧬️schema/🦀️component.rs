//! 🧬️ IfcArtifact schema — full artifact state. Ticket
//! 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION: this used to duplicate
//! `IfcSnapshot`'s prior worst-offender defect (`document: step::engine::part21::Part21Document`
//! verbatim) — now mirrors `IfcSnapshot`'s own typed `header`/`entities` fields.

use crate::artifacts::ifc::schema::snapshot::{IfcHeader, IfcEntity};
use crate::artifacts::ifc::IfcSnapshot;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Artifact
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.ifc")]
pub struct IfcArtifact {
    #[state(persistent)]
    pub schema: String,
    /// 📦️ The full, lossless IFC4 graph in IFC's own typed model — the actual persisted state.
    #[state(persistent)]
    #[serde(default)]
    pub header: IfcHeader,
    #[state(persistent)]
    #[serde(default)]
    pub entities: Vec<IfcEntity>,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for IfcArtifact {
    fn default() -> Self {
        Self::from_snapshot(IfcSnapshot::default())
    }
}

impl IfcArtifact {
    pub fn to_snapshot(&self) -> IfcSnapshot {
        IfcSnapshot {
            schema: self.schema.clone(),
            header: self.header.clone(),
            entities: self.entities.clone(),
        }
    }

    pub fn from_snapshot(snapshot: IfcSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            header: snapshot.header,
            entities: snapshot.entities,
        }
    }

    pub fn set_snapshot(&mut self, snapshot: IfcSnapshot) {
        self.schema = snapshot.schema;
        self.header = snapshot.header;
        self.entities = snapshot.entities;
    }

    /// 🏛️ Derived spatial-structure/placement/pset analyzer view — computed on demand, never
    /// stored; builds the shared generic Part-21 graph on the fly via `to_part21_document`
    /// (the analyzer's own relationship-graph traversal still walks that generic shape).
    pub fn spatial(&self) -> crate::artifacts::ifc::engine::spatial::SpatialAnalysis {
        let document = crate::artifacts::ifc::schema::snapshot::to_part21_document(&self.to_snapshot());
        crate::artifacts::ifc::engine::spatial::analyze_spatial(&document)
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
pub fn ifc_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.stdio.ifc",
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
    use crate::artifacts::ifc::{IfcDiff, IfcMutation, IfcSnapshot};

    //#region 🔖️Builder
    /// 🏗️ Builds a `stdio.ifc` snapshot.
    #[derive(Clone, Debug, Default)]
    pub struct IfcBuilderConstruction {
        snapshot: IfcSnapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for IfcBuilderConstruction {
        type Snapshot = IfcSnapshot;
        type Mutation = IfcMutation;
        type Diff = IfcDiff;
        fn empty() -> Self {
            Self { snapshot: IfcSnapshot::default(), diagnostics: Vec::new() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<IfcSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<IfcSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
            let diff = crate::artifacts::ifc::schema::mutations::apply_ifc_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }
        fn absorb(mut self, diff: Self::Diff) -> Self {
            self.snapshot = <IfcDiff as protocol::MutationDiff<IfcSnapshot>>::apply(&diff, &self.snapshot);
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
    use crate::artifacts::ifc::IfcSnapshot;

    //#region 🔖️Parts
    /// 🧩 Analyzed `stdio.ifc` parts.
    #[derive(Clone, Debug, Default)]
    pub struct IfcParts {
        pub snapshot: Option<IfcSnapshot>,
    }
    //#endregion 🔖️Parts

    //#region 🔖️Analyzer
    /// 🧐️ Analyzes `stdio.ifc` (4/✳️any) sources.
    pub struct IfcAnalyzerAnalysis;

    impl ArtifactAnalysis for IfcAnalyzerAnalysis {
        type Parts = IfcParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.ifc", standard: StandardId("4"), subset: SubsetId("*") };

        fn sniff(_source: &AnalyzeSource<'_>) -> IoConfidence {
            IoConfidence::Medium
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = IfcParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <IfcSnapshot as store::ArtifactDsl>::parse_dsl(text) {
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
                    AnalyzeSource::Binary(bytes) => match <IfcSnapshot as store::ArtifactPack>::decode_pack(bytes) {
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
    pub spec IfcBuilderFacets {
        construction: derived_construction::IfcBuilderConstruction,
        analysis: derived_analysis::IfcAnalyzerAnalysis,
        composition: super::super::io::derived_composition::IfcComposerComposition,
    }
    builder: IfcBuilder,
    analyzer: IfcAnalyzer,
    composer: IfcComposer,
);
//#endregion 🧬️DerivedArtifactFacets
